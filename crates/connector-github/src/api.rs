//! GitHub REST client.
//!
//! Hits `https://api.github.com` (or an override for tests) with
//! `Authorization: Bearer <pat>` + a `User-Agent` header (required by GitHub
//! — requests without one are rejected with 403).

use chrono::{DateTime, TimeZone, Utc};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, LINK, USER_AGENT};
use reqwest::StatusCode;
use secrecy::ExposeSecret;
use serde::de::DeserializeOwned;
use tracing::{debug, warn};

use focus_connectors::ConnectorError;

use crate::auth::GitHubToken;
use crate::models::{GitHubEvent, GitHubUser};

/// GitHub REST API default base URL.
pub const DEFAULT_BASE_URL: &str = "https://api.github.com";
/// User-Agent header value — GitHub requires UA, and requires it to be
/// something "informative" (login or app name). We use the app slug.
pub const USER_AGENT_VALUE: &str = "FocalPoint/0.0.1";
/// Defensive pagination cap — prevents runaway loops if `Link: rel="next"`
/// ever points back at us. GitHub caps `per_page` at 100, so 10 pages ≈ the
/// last 1k events, well beyond what any reasonable user produces in a
/// polling window.
pub const MAX_PAGES: usize = 10;

/// Minimal GitHub REST client.
#[derive(Debug, Clone)]
pub struct GitHubClient {
    pub base_url: String,
    token: GitHubToken,
    http: reqwest::Client,
}

/// Paginated listing result.
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

impl GitHubClient {
    pub fn new(base_url: impl Into<String>, token: GitHubToken) -> Self {
        Self::with_http(base_url, token, reqwest::Client::new())
    }

    pub fn with_http(
        base_url: impl Into<String>,
        token: GitHubToken,
        http: reqwest::Client,
    ) -> Self {
        Self { base_url: base_url.into().trim_end_matches('/').to_string(), token, http }
    }

    fn auth_headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        if let Ok(v) = format!("Bearer {}", self.token.access_token.expose_secret()).parse() {
            h.insert(AUTHORIZATION, v);
        }
        h.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
        h.insert(ACCEPT, HeaderValue::from_static("application/vnd.github+json"));
        h.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));
        h
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<(T, HeaderMap), ConnectorError> {
        let resp = self
            .http
            .get(url)
            .headers(self.auth_headers())
            .send()
            .await
            .map_err(|e| ConnectorError::Network(e.to_string()))?;

        let status = resp.status();
        let headers = resp.headers().clone();

        match status {
            s if s.is_success() => {
                let body: T =
                    resp.json().await.map_err(|e| ConnectorError::Schema(e.to_string()))?;
                Ok((body, headers))
            }
            StatusCode::UNAUTHORIZED => Err(ConnectorError::Unauthorized("401 from GitHub".into())),
            StatusCode::FORBIDDEN => {
                // GitHub uses 403 + X-RateLimit-Remaining: 0 for primary
                // rate-limit exhaustion. Anything else is a genuine 403.
                if rate_limit_remaining(&headers) == Some(0) {
                    let reset = rate_limit_reset(&headers).unwrap_or_else(Utc::now);
                    warn!(
                        target: "connector_github::api",
                        reset_at = %reset,
                        "github 403 rate-limit exhausted"
                    );
                    Err(ConnectorError::RateLimitedUntil(reset))
                } else {
                    let body_text = resp.text().await.unwrap_or_default();
                    Err(ConnectorError::Forbidden(format!(
                        "403 from GitHub: {}",
                        truncate(&body_text, 256)
                    )))
                }
            }
            other => {
                let body_text = resp.text().await.unwrap_or_default();
                debug!(target: "connector_github::api", status = %other, body = %truncate(&body_text, 128), "github non-success");
                Err(ConnectorError::Network(format!("HTTP {other}")))
            }
        }
    }

    /// `GET /user` — validates the token and returns the authenticated user.
    pub async fn get_self(&self) -> Result<GitHubUser, ConnectorError> {
        let url = format!("{}/user", self.base_url);
        let (u, _) = self.get_json::<GitHubUser>(&url).await?;
        Ok(u)
    }

    /// `GET /users/{login}/events` — public events for the user.
    ///
    /// Walks `Link: rel="next"` up to [`MAX_PAGES`]. `cursor` is the URL of
    /// the next page when resuming a partial sync.
    pub async fn list_user_events(
        &self,
        login: &str,
        cursor: Option<String>,
    ) -> Result<Page<GitHubEvent>, ConnectorError> {
        let initial = format!("{}/users/{}/events?per_page=100", self.base_url, login);
        let mut url = cursor.unwrap_or(initial);
        let mut items: Vec<GitHubEvent> = Vec::new();
        let mut next_cursor: Option<String> = None;

        for page_ix in 0..MAX_PAGES {
            let (batch, headers) = self.get_json::<Vec<GitHubEvent>>(&url).await?;
            items.extend(batch);
            let link = headers.get(LINK).and_then(|v| v.to_str().ok());
            let next = parse_next_link(link);
            match next {
                Some(n) if page_ix + 1 < MAX_PAGES => {
                    url = n;
                }
                Some(n) => {
                    // Hit the cap — hand the cursor back so the driver can
                    // resume on the next tick rather than losing ground.
                    warn!(
                        target: "connector_github::api",
                        max_pages = MAX_PAGES,
                        "hit pagination cap on user events; returning next cursor"
                    );
                    next_cursor = Some(n);
                    break;
                }
                None => {
                    break;
                }
            }
        }

        Ok(Page { items, next_cursor })
    }
}

/// Parse a GitHub `Link` header and return the URL with `rel="next"`.
pub fn parse_next_link(link: Option<&str>) -> Option<String> {
    let link = link?;
    for part in link.split(',') {
        let seg = part.trim();
        // <https://api.github.com/...?page=2>; rel="next"
        let (url_part, rel_part) = seg.split_once(';')?;
        let url = url_part.trim().trim_start_matches('<').trim_end_matches('>');
        if rel_part.contains("rel=\"next\"") {
            return Some(url.to_string());
        }
    }
    None
}

/// Read `X-RateLimit-Remaining` as a `u64`.
pub fn rate_limit_remaining(headers: &HeaderMap) -> Option<u64> {
    headers
        .get("X-RateLimit-Remaining")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
}

/// Read `X-RateLimit-Reset` as a UTC timestamp. GitHub sends epoch seconds.
pub fn rate_limit_reset(headers: &HeaderMap) -> Option<DateTime<Utc>> {
    let secs = headers
        .get("X-RateLimit-Reset")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<i64>().ok())?;
    Utc.timestamp_opt(secs, 0).single()
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use reqwest::header::HeaderMap;

    #[test]
    fn parse_next_link_finds_next() {
        let h = r#"<https://api.github.com/users/x/events?page=1>; rel="prev", <https://api.github.com/users/x/events?page=3>; rel="next", <https://api.github.com/users/x/events?page=10>; rel="last""#;
        assert_eq!(
            parse_next_link(Some(h)).as_deref(),
            Some("https://api.github.com/users/x/events?page=3")
        );
    }

    #[test]
    fn parse_next_link_absent() {
        assert!(parse_next_link(None).is_none());
        assert!(parse_next_link(Some("<x>; rel=\"last\"")).is_none());
    }

    #[test]
    fn rate_limit_headers_parse() {
        let mut h = HeaderMap::new();
        h.insert("X-RateLimit-Remaining", HeaderValue::from_static("0"));
        h.insert("X-RateLimit-Reset", HeaderValue::from_static("1800000000"));
        assert_eq!(rate_limit_remaining(&h), Some(0));
        let reset = rate_limit_reset(&h).unwrap();
        assert_eq!(reset.timestamp(), 1_800_000_000);
    }
}
