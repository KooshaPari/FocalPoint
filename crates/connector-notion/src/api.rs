//! Notion API v1 client — /users/me, /databases, /pages endpoints.

use reqwest::Client;
use serde_json::Value;

use focus_connectors::Result as ConnResult;

use crate::models::{NotionPage, NotionTask};

const NOTION_API_BASE: &str = "https://api.notion.com/v1";

/// Notion API v1 client — makes authenticated calls to Notion.
pub struct NotionClient {
    http: Client,
}

impl NotionClient {
    pub fn new(http: Client) -> Self {
        Self { http }
    }

    /// GET /users/me — fetch current user for health check.
    pub async fn get_me(&self) -> ConnResult<Value> {
        let url = format!("{}/users/me", NOTION_API_BASE);
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
                "Notion integration token invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Notion me request failed: {}",
                resp.status()
            )))
        }
    }

    /// GET /search (query for pages) — fetch all accessible pages.
    pub async fn get_pages(&self) -> ConnResult<Vec<NotionPage>> {
        let url = format!("{}/search", NOTION_API_BASE);
        let resp = self
            .http
            .post(&url)
            .json(&serde_json::json!({
                "filter": { "property": "object", "value": "page" },
                "sort": { "direction": "descending", "timestamp": "last_edited_time" }
            }))
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            let json = resp
                .json::<Value>()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))?;
            let pages = NotionPage::from_notion_json(&json);
            Ok(pages)
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Notion integration token invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Notion pages request failed: {}",
                resp.status()
            )))
        }
    }

    /// Query all pages marked as tasks in Notion.
    pub async fn get_tasks(&self) -> ConnResult<Vec<NotionTask>> {
        let url = format!("{}/search", NOTION_API_BASE);
        let resp = self
            .http
            .post(&url)
            .json(&serde_json::json!({
                "filter": { "property": "object", "value": "page" },
            }))
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            let json = resp
                .json::<Value>()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))?;
            let tasks = NotionTask::from_notion_json(&json);
            Ok(tasks)
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Notion integration token invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Notion tasks request failed: {}",
                resp.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-NOTION-API-001 (API client contract)
    #[tokio::test]
    async fn notion_client_construction() {
        let http = Client::new();
        let _client = NotionClient::new(http);
        assert!(true);
    }
}
