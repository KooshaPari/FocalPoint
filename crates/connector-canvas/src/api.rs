//! Canvas REST client.

use focus_connectors::ConnectorError;
use reqwest::header::{HeaderMap, AUTHORIZATION, LINK};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::models::{Assignment, CanvasUser, Course, Submission};

/// Minimal Canvas REST client.
#[derive(Debug, Clone)]
pub struct CanvasClient {
    pub base_url: String,
    pub access_token: String,
    http: reqwest::Client,
}

/// Result of a paginated listing.
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

impl CanvasClient {
    pub fn new(base_url: impl Into<String>, access_token: impl Into<String>) -> Self {
        Self::with_http(base_url, access_token, reqwest::Client::new())
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
            StatusCode::UNAUTHORIZED => Err(ConnectorError::Auth("401 from Canvas".into())),
            StatusCode::FORBIDDEN => {
                // Canvas throttles with 403 + X-Request-Cost; treat as rate limit.
                let retry = headers
                    .get("X-Rate-Limit-Remaining")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<f64>().ok())
                    .map(|_| 5u64)
                    .unwrap_or(10);
                Err(ConnectorError::RateLimited(retry))
            }
            StatusCode::TOO_MANY_REQUESTS => Err(ConnectorError::RateLimited(30)),
            other => Err(ConnectorError::Network(format!("HTTP {other}"))),
        }
    }

    async fn list_paginated<T: DeserializeOwned>(
        &self,
        initial_url: String,
        cursor: Option<String>,
    ) -> Result<Page<T>, ConnectorError> {
        let url = cursor.unwrap_or(initial_url);
        let (items, headers) = self.get_json::<Vec<T>>(&url).await?;
        let next_cursor = parse_next_link(headers.get(LINK).and_then(|v| v.to_str().ok()));
        Ok(Page { items, next_cursor })
    }

    /// List courses. If `user_id` is None uses `self`.
    pub async fn list_courses(
        &self,
        user_id: Option<u64>,
        cursor: Option<String>,
    ) -> Result<Page<Course>, ConnectorError> {
        let who = user_id.map(|i| i.to_string()).unwrap_or_else(|| "self".into());
        let url = format!(
            "{}/api/v1/users/{}/courses?per_page=50&enrollment_state=active",
            self.base_url, who
        );
        self.list_paginated(url, cursor).await
    }

    pub async fn list_assignments(
        &self,
        course_id: u64,
        cursor: Option<String>,
    ) -> Result<Page<Assignment>, ConnectorError> {
        let url = format!("{}/api/v1/courses/{}/assignments?per_page=50", self.base_url, course_id);
        self.list_paginated(url, cursor).await
    }

    pub async fn list_submissions(
        &self,
        assignment_id: u64,
        course_id: u64,
        cursor: Option<String>,
    ) -> Result<Page<Submission>, ConnectorError> {
        let url = format!(
            "{}/api/v1/courses/{}/assignments/{}/submissions?per_page=50",
            self.base_url, course_id, assignment_id
        );
        self.list_paginated(url, cursor).await
    }

    pub async fn get_self(&self) -> Result<CanvasUser, ConnectorError> {
        let url = format!("{}/api/v1/users/self", self.base_url);
        let (u, _) = self.get_json::<CanvasUser>(&url).await?;
        Ok(u)
    }
}

/// Parse a Canvas `Link` header and return the URL with `rel="next"`, if present.
pub fn parse_next_link(link: Option<&str>) -> Option<String> {
    let link = link?;
    for part in link.split(',') {
        let seg = part.trim();
        // Example: <https://canvas/.../?page=2>; rel="next"
        let (url_part, rel_part) = seg.split_once(';')?;
        let url = url_part.trim().trim_start_matches('<').trim_end_matches('>');
        if rel_part.contains("rel=\"next\"") {
            return Some(url.to_string());
        }
    }
    None
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn parse_next_link_finds_next() {
        let header = "<https://c.example/api/v1/x?page=1>; rel=\"current\", <https://c.example/api/v1/x?page=2>; rel=\"next\", <https://c.example/api/v1/x?page=5>; rel=\"last\"";
        let next = parse_next_link(Some(header));
        assert_eq!(next.as_deref(), Some("https://c.example/api/v1/x?page=2"));
    }

    #[test]
    fn parse_next_link_absent() {
        assert!(parse_next_link(None).is_none());
        assert!(parse_next_link(Some("<x>; rel=\"last\"")).is_none());
    }

    #[tokio::test]
    async fn lists_courses_and_follows_pagination() {
        let server = MockServer::start().await;
        let base = server.uri();
        let next_url = format!("{base}/api/v1/users/self/courses?page=2");
        let link_hdr = format!("<{next_url}>; rel=\"next\"");

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/self/courses$"))
            .and(header("authorization", "Bearer TOK"))
            .respond_with(
                ResponseTemplate::new(200).insert_header("Link", link_hdr.as_str()).set_body_json(
                    serde_json::json!([{"id":1,"name":"A","workflow_state":"available"}]),
                ),
            )
            .mount(&server)
            .await;

        let client = CanvasClient::new(&base, "TOK");
        let page = client.list_courses(None, None).await.unwrap();
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].id, 1);
        assert_eq!(page.next_cursor.as_deref(), Some(next_url.as_str()));
    }

    #[tokio::test]
    async fn unauthorized_maps_to_auth_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/self$"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;
        let client = CanvasClient::new(server.uri(), "bad");
        let err = client.get_self().await.unwrap_err();
        assert!(matches!(err, ConnectorError::Auth(_)));
    }

    #[tokio::test]
    async fn forbidden_maps_to_rate_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/self$"))
            .respond_with(ResponseTemplate::new(403).insert_header("X-Request-Cost", "100.0"))
            .mount(&server)
            .await;
        let client = CanvasClient::new(server.uri(), "t");
        let err = client.get_self().await.unwrap_err();
        assert!(matches!(err, ConnectorError::RateLimited(_)));
    }
}
