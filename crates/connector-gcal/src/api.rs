//! Google Calendar v3 REST client.

use focus_connectors::ConnectorError;
use reqwest::header::{HeaderMap, AUTHORIZATION, RETRY_AFTER};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use tracing::warn;

use crate::models::{CalendarList, CalendarListEntry, EventList, GCalEvent, GCalUser};

pub const GOOGLE_API_BASE: &str = "https://www.googleapis.com";

/// Minimal Google Calendar v3 REST client.
#[derive(Debug, Clone)]
pub struct GCalClient {
    pub base_url: String,
    pub access_token: String,
    http: reqwest::Client,
}

/// Result of a paginated listing. Google uses `pageToken` (an opaque string)
/// rather than `Link` headers — we expose it as `next_cursor`.
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

impl GCalClient {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self::with_http(GOOGLE_API_BASE, access_token, reqwest::Client::new())
    }

    pub fn with_http(
        base_url: impl Into<String>,
        access_token: impl Into<String>,
        http: reqwest::Client,
    ) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            access_token: access_token.into(),
            http,
        }
    }

    pub fn set_access_token(&mut self, token: impl Into<String>) {
        self.access_token = token.into();
    }

    fn auth_headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        if let Ok(v) = format!("Bearer {}", self.access_token).parse() {
            h.insert(AUTHORIZATION, v);
        }
        h
    }

    async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, ConnectorError> {
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
                resp.json::<T>().await.map_err(|e| ConnectorError::Schema(e.to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(ConnectorError::Auth("401 from Google".into())),
            StatusCode::FORBIDDEN => {
                // Google's 403 is either:
                //   * `rateLimitExceeded` / `userRateLimitExceeded` — throttle
                //   * `forbidden` / `insufficientPermissions` — real denial
                // The discriminator is in the JSON body: `error.errors[].reason`.
                let body_text = resp.text().await.unwrap_or_default();
                if looks_like_rate_limit(&body_text) {
                    let retry = parse_retry_after(&headers).unwrap_or(30);
                    warn!(
                        target: "gcal::api",
                        retry_after = retry,
                        "gcal 403 rate-limit"
                    );
                    Err(ConnectorError::RateLimited(retry))
                } else {
                    Err(ConnectorError::Auth(format!(
                        "403 from Google (permission denied): {}",
                        truncate(&body_text, 256)
                    )))
                }
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let retry = parse_retry_after(&headers).unwrap_or(30);
                warn!(target: "gcal::api", retry_after = retry, "gcal 429 rate-limit");
                Err(ConnectorError::RateLimited(retry))
            }
            other => Err(ConnectorError::Network(format!("HTTP {other}"))),
        }
    }

    /// List all calendars on the user's calendar list.
    ///
    /// `cursor` is Google's `pageToken` from a previous call.
    pub async fn list_calendar_list(
        &self,
        cursor: Option<String>,
    ) -> Result<Page<CalendarListEntry>, ConnectorError> {
        let mut url = format!("{}/calendar/v3/users/me/calendarList?maxResults=250", self.base_url);
        if let Some(tok) = cursor {
            url.push_str("&pageToken=");
            url.push_str(&urlencode(&tok));
        }
        let body: CalendarList = self.get_json(&url).await?;
        Ok(Page { items: body.items, next_cursor: body.next_page_token })
    }

    /// List events on a single calendar, expanded as single instances and
    /// ordered by start time within the `[time_min, time_max]` window.
    ///
    /// `time_min` / `time_max` are RFC3339 strings (callers build these from
    /// chrono). Both are required to be present by Google when `orderBy` is
    /// `startTime` with `singleEvents=true`.
    pub async fn list_events(
        &self,
        calendar_id: &str,
        time_min: &str,
        time_max: &str,
        cursor: Option<String>,
    ) -> Result<Page<GCalEvent>, ConnectorError> {
        let mut url = format!(
            "{}/calendar/v3/calendars/{cal}/events?singleEvents=true&orderBy=startTime&timeMin={tmin}&timeMax={tmax}&maxResults=250",
            self.base_url,
            cal = urlencode(calendar_id),
            tmin = urlencode(time_min),
            tmax = urlencode(time_max),
        );
        if let Some(tok) = cursor {
            url.push_str("&pageToken=");
            url.push_str(&urlencode(&tok));
        }
        let body: EventList = self.get_json(&url).await?;
        Ok(Page { items: body.items, next_cursor: body.next_page_token })
    }

    /// Fetch the user's identity for health-check purposes.
    pub async fn get_self(&self) -> Result<GCalUser, ConnectorError> {
        let url = format!("{}/oauth2/v2/userinfo", self.base_url);
        self.get_json::<GCalUser>(&url).await
    }
}

fn looks_like_rate_limit(body: &str) -> bool {
    let lower = body.to_lowercase();
    lower.contains("ratelimitexceeded")
        || lower.contains("userratelimitexceeded")
        || lower.contains("rate limit exceeded")
}

fn parse_retry_after(headers: &HeaderMap) -> Option<u64> {
    headers
        .get(RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
}

fn urlencode(s: &str) -> String {
    // Minimal percent-encoder — Google accepts the lenient subset.
    // Using `url::form_urlencoded` would also work but pulls a slightly
    // different contract for path segments.
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        let safe = b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~');
        if safe {
            out.push(b as char);
        } else {
            out.push('%');
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0x0f) as usize] as char);
        }
    }
    out
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
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn lists_calendar_list() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/calendar/v3/users/me/calendarList"))
            .and(header("authorization", "Bearer TOK"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [{"id":"primary","summary":"Me","primary":true}],
                "nextPageToken": "pg2"
            })))
            .mount(&server)
            .await;

        let client = GCalClient::with_http(server.uri(), "TOK", reqwest::Client::new());
        let page = client.list_calendar_list(None).await.unwrap();
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].id, "primary");
        assert_eq!(page.next_cursor.as_deref(), Some("pg2"));
    }

    #[tokio::test]
    async fn lists_events_with_single_events_and_order() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/calendar/v3/calendars/primary/events"))
            .and(query_param("singleEvents", "true"))
            .and(query_param("orderBy", "startTime"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [{"id":"e1","summary":"A","start":{"dateTime":"2026-05-01T09:00:00Z"},"end":{"dateTime":"2026-05-01T10:00:00Z"}}]
            })))
            .mount(&server)
            .await;

        let client = GCalClient::with_http(server.uri(), "TOK", reqwest::Client::new());
        let page = client
            .list_events("primary", "2026-05-01T00:00:00Z", "2026-05-08T00:00:00Z", None)
            .await
            .unwrap();
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].id, "e1");
        assert!(page.next_cursor.is_none());
    }

    #[tokio::test]
    async fn unauthorized_maps_to_auth_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/oauth2/v2/userinfo"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;
        let client = GCalClient::with_http(server.uri(), "bad", reqwest::Client::new());
        let err = client.get_self().await.unwrap_err();
        assert!(matches!(err, ConnectorError::Auth(_)));
    }

    #[tokio::test]
    async fn forbidden_with_rate_limit_body_maps_to_rate_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/oauth2/v2/userinfo"))
            .respond_with(
                ResponseTemplate::new(403).insert_header("Retry-After", "42").set_body_json(
                    serde_json::json!({
                        "error": {
                            "code": 403,
                            "errors": [{"reason": "rateLimitExceeded"}]
                        }
                    }),
                ),
            )
            .mount(&server)
            .await;
        let client = GCalClient::with_http(server.uri(), "t", reqwest::Client::new());
        let err = client.get_self().await.unwrap_err();
        match err {
            ConnectorError::RateLimited(secs) => assert_eq!(secs, 42),
            other => panic!("expected RateLimited, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn forbidden_permission_denied_maps_to_auth() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/oauth2/v2/userinfo"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "error": {"code": 403, "message":"insufficient permissions"}
            })))
            .mount(&server)
            .await;
        let client = GCalClient::with_http(server.uri(), "t", reqwest::Client::new());
        let err = client.get_self().await.unwrap_err();
        match err {
            ConnectorError::Auth(msg) => assert!(msg.contains("permission denied"), "got: {msg}"),
            other => panic!("expected Auth error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn too_many_requests_honors_retry_after() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/oauth2/v2/userinfo"))
            .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "17"))
            .mount(&server)
            .await;
        let client = GCalClient::with_http(server.uri(), "t", reqwest::Client::new());
        let err = client.get_self().await.unwrap_err();
        match err {
            ConnectorError::RateLimited(secs) => assert_eq!(secs, 17),
            other => panic!("expected RateLimited, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn too_many_requests_defaults_when_retry_after_missing() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/oauth2/v2/userinfo"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&server)
            .await;
        let client = GCalClient::with_http(server.uri(), "t", reqwest::Client::new());
        let err = client.get_self().await.unwrap_err();
        assert!(matches!(err, ConnectorError::RateLimited(30)));
    }

    #[test]
    fn urlencode_escapes_nonalpha() {
        assert_eq!(urlencode("a@b.com"), "a%40b.com");
        assert_eq!(urlencode("2026-05-01T00:00:00Z"), "2026-05-01T00%3A00%3A00Z");
        assert_eq!(urlencode("primary"), "primary");
    }
}
