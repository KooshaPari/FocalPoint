//! Strava connector — OAuth2 auth, REST client, event mapping, `Connector` impl.

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

use crate::api::StravaClient;
use crate::auth::{StravaOAuth2, KeychainTokenStore, TokenStore};
use crate::events::StravaEventMapper;

/// Strava connector.
pub struct StravaConnector {
    manifest: ConnectorManifest,
    account_id: Uuid,
    #[allow(dead_code)]
    token_store: Arc<dyn TokenStore>,
    #[allow(dead_code)]
    oauth: Option<Arc<StravaOAuth2>>,
    client: Mutex<StravaClient>,
}

pub struct StravaConnectorBuilder {
    #[allow(dead_code)]
    client_id: String,
    #[allow(dead_code)]
    client_secret: String,
    account_id: Uuid,
    token_store: Option<Arc<dyn TokenStore>>,
    oauth: Option<Arc<StravaOAuth2>>,
    http: Option<reqwest::Client>,
}

impl StravaConnectorBuilder {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            account_id: Uuid::nil(),
            token_store: None,
            oauth: None,
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

    pub fn oauth(mut self, o: Arc<StravaOAuth2>) -> Self {
        self.oauth = Some(o);
        self
    }

    pub fn http(mut self, h: reqwest::Client) -> Self {
        self.http = Some(h);
        self
    }

    pub fn build(self) -> StravaConnector {
        let http = self.http.unwrap_or_default();
        let store = self
            .token_store
            .unwrap_or_else(|| Arc::new(KeychainTokenStore::new()));
        let client = StravaClient::new(http);
        StravaConnector {
            manifest: default_manifest(),
            account_id: self.account_id,
            token_store: store,
            oauth: self.oauth,
            client: Mutex::new(client),
        }
    }
}

fn default_manifest() -> ConnectorManifest {
    ConnectorManifest {
        id: "strava".into(),
        version: "0.1.0".into(),
        display_name: "Strava".into(),
        auth_strategy: AuthStrategy::OAuth2 {
            scopes: vec!["read".into(), "activity:read".into()],
        },
        sync_mode: SyncMode::Polling {
            cadence_seconds: 300,
        },
        capabilities: vec![],
        entity_types: vec!["activity".into(), "workout".into()],
        event_types: vec![
            "strava:activity_completed".into(),
            "strava:pr_earned".into(),
        ],
        tier: VerificationTier::Verified,
        health_indicators: vec!["last_sync_ok".into(), "auth_token_fresh".into()],
    }
}

#[async_trait]
impl Connector for StravaConnector {
    fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }

    async fn health(&self) -> HealthState {
        let client = self.client.lock().await;
        match client.get_athlete().await {
            Ok(_) => HealthState::Healthy,
            Err(ConnectorError::Unauthorized(_)) => HealthState::Unauthenticated,
            Err(e) => HealthState::Failing(e.to_string()),
        }
    }

    async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
        let client = self.client.lock().await;
        let mapper = StravaEventMapper::new(self.account_id);
        let mut events = Vec::new();

        // Fetch recent activities (per-hour rate limit: 100 req/15min, 1000/day).
        debug!("Strava: fetching recent activities");

        match client.get_recent_activities(10).await {
            Ok(activities) => {
                events.extend(mapper.map_activities(activities));
            }
            Err(e) => warn!("Strava: activity sync failed: {}", e),
        }

        info!("Strava: synced {} events", events.len());

        Ok(SyncOutcome {
            events,
            next_cursor: None,
            partial: false,
        })
    }
}
