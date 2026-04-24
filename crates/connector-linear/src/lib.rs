//! Linear connector — OAuth2/PAT auth, GraphQL API client, event mapping, `Connector` impl.
//! Emits: `linear:issue_created`, `linear:issue_closed`.

pub mod api;
pub mod auth;
pub mod events;
pub mod models;

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};
use uuid::Uuid;

use focus_connectors::{
    AuthStrategy, Connector, ConnectorError, ConnectorManifest, HealthState, Result, SyncMode,
    SyncOutcome, VerificationTier,
};

use crate::api::LinearClient;
use crate::auth::TokenStore;
use crate::events::LinearEventMapper;

/// Linear connector.
pub struct LinearConnector {
    manifest: ConnectorManifest,
    account_id: Uuid,
    token_store: Arc<dyn TokenStore>,
    client: Mutex<LinearClient>,
}

pub struct LinearConnectorBuilder {
    account_id: Uuid,
    token_store: Option<Arc<dyn TokenStore>>,
    http: Option<reqwest::Client>,
}

impl LinearConnectorBuilder {
    pub fn new() -> Self {
        Self {
            account_id: Uuid::nil(),
            token_store: None,
            http: None,
        }
    }

    pub fn account_id(mut self, id: Uuid) -> Self {
        self.account_id = id;
        self
    }

    pub fn token_store(mut self, s: Arc<dyn TokenStore>) -> Self {
        self.token_store = Some(s);
        self
    }

    pub fn http(mut self, h: reqwest::Client) -> Self {
        self.http = Some(h);
        self
    }

    pub fn build(self) -> LinearConnector {
        let http = self.http.unwrap_or_default();
        let store = self
            .token_store
            .unwrap_or_else(|| Arc::new(auth::InMemoryTokenStore::new()));
        let client = LinearClient::new(http);
        LinearConnector {
            manifest: default_manifest(),
            account_id: self.account_id,
            token_store: store,
            client: Mutex::new(client),
        }
    }
}

fn default_manifest() -> ConnectorManifest {
    ConnectorManifest {
        id: "linear".into(),
        version: "0.1.0".into(),
        display_name: "Linear".into(),
        auth_strategy: AuthStrategy::ApiKey,
        sync_mode: SyncMode::Polling {
            cadence_seconds: 300,
        },
        capabilities: vec![],
        entity_types: vec!["issue".into()],
        event_types: vec![
            "linear:issue_created".into(),
            "linear:issue_closed".into(),
        ],
        tier: VerificationTier::Verified,
        health_indicators: vec!["last_sync_ok".into(), "api_key_valid".into()],
    }
}

#[async_trait]
impl Connector for LinearConnector {
    fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }

    async fn health(&self) -> HealthState {
        let client = self.client.lock().await;
        match client.get_viewer().await {
            Ok(_) => HealthState::Healthy,
            Err(ConnectorError::Unauthorized(_)) => HealthState::Unauthenticated,
            Err(e) => HealthState::Failing(e.to_string()),
        }
    }

    async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
        let client = self.client.lock().await;
        let mapper = LinearEventMapper::new(self.account_id);
        let mut events = Vec::new();

        debug!("Linear: fetching issues");

        match client.get_issues().await {
            Ok(issues) => {
                events.extend(mapper.map_issues(issues));
            }
            Err(e) => warn!("Linear: issue sync failed: {}", e),
        }

        info!("Linear: synced {} events", events.len());

        Ok(SyncOutcome {
            events,
            next_cursor: None,
            partial: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-LINEAR-001 (manifest and connector contract)
    #[test]
    fn linear_builder_constructs() {
        let account_id = Uuid::new_v4();
        let connector = LinearConnectorBuilder::new()
            .account_id(account_id)
            .build();
        assert_eq!(connector.manifest().id, "linear");
    }

    // Traces to: FR-LINEAR-001
    #[test]
    fn linear_manifest_has_events() {
        let manifest = default_manifest();
        assert_eq!(manifest.event_types.len(), 2);
        assert!(manifest.event_types.iter().any(|e| e.contains("issue_created")));
        assert!(manifest.event_types.iter().any(|e| e.contains("issue_closed")));
    }
}
