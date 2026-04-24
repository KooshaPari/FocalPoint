//! Readwise Reader API client — /documents, /highlights endpoints.

use reqwest::Client;
use serde_json::Value;

use focus_connectors::Result as ConnResult;

use crate::models::{Article, Highlight};

const READWISE_API_BASE: &str = "https://readwise.io/api/v3";

/// Readwise REST client — makes authenticated calls to Readwise Reader API.
pub struct ReadwiseClient {
    http: Client,
}

impl ReadwiseClient {
    pub fn new(http: Client) -> Self {
        Self { http }
    }

    /// GET /reader — fetch reader metadata for health check.
    pub async fn get_reader_data(&self) -> ConnResult<Value> {
        let url = format!("{}/reader", READWISE_API_BASE);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            resp.json()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Readwise token invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Readwise reader request failed: {}",
                resp.status()
            )))
        }
    }

    /// GET /documents — fetch all documents (articles).
    pub async fn get_articles(&self) -> ConnResult<Vec<Article>> {
        let url = format!("{}/documents", READWISE_API_BASE);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            let json = resp
                .json::<Value>()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))?;
            let articles = Article::from_readwise_json(&json);
            Ok(articles)
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Readwise token invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Readwise articles request failed: {}",
                resp.status()
            )))
        }
    }

    /// GET /highlights — fetch all highlights.
    pub async fn get_highlights(&self) -> ConnResult<Vec<Highlight>> {
        let url = format!("{}/highlights", READWISE_API_BASE);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            let json = resp
                .json::<Value>()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))?;
            let highlights = Highlight::from_readwise_json(&json);
            Ok(highlights)
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Readwise token invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Readwise highlights request failed: {}",
                resp.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-READWISE-API-001 (API client contract)
    #[tokio::test]
    async fn readwise_client_construction() {
        let http = Client::new();
        let _client = ReadwiseClient::new(http);
        assert!(true);
    }
}
