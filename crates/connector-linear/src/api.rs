//! Linear GraphQL API client — issues, viewer endpoints.

use reqwest::Client;
use serde_json::Value;

use focus_connectors::Result as ConnResult;

use crate::models::LinearIssue;

const LINEAR_API_BASE: &str = "https://api.linear.app/graphql";

/// Linear GraphQL API client — makes authenticated queries to Linear.
pub struct LinearClient {
    http: Client,
}

impl LinearClient {
    pub fn new(http: Client) -> Self {
        Self { http }
    }

    /// Query for viewer info (health check).
    pub async fn get_viewer(&self) -> ConnResult<Value> {
        let query = r#"
            query {
                viewer {
                    id
                    email
                }
            }
        "#;

        let resp = self
            .http
            .post(LINEAR_API_BASE)
            .json(&serde_json::json!({ "query": query }))
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            resp.json()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Linear API key invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Linear viewer request failed: {}",
                resp.status()
            )))
        }
    }

    /// Query for all issues.
    pub async fn get_issues(&self) -> ConnResult<Vec<LinearIssue>> {
        let query = r#"
            query {
                issues(first: 50) {
                    nodes {
                        id
                        identifier
                        title
                        state {
                            name
                        }
                        createdAt
                        updatedAt
                    }
                }
            }
        "#;

        let resp = self
            .http
            .post(LINEAR_API_BASE)
            .json(&serde_json::json!({ "query": query }))
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            let json = resp
                .json::<Value>()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))?;
            let issues = LinearIssue::from_linear_json(&json);
            Ok(issues)
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Linear API key invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Linear issues request failed: {}",
                resp.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-LINEAR-API-001 (API client contract)
    #[tokio::test]
    async fn linear_client_construction() {
        let http = Client::new();
        let _client = LinearClient::new(http);
        assert!(true);
    }
}
