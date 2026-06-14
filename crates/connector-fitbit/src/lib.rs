//! Fitbit connector — OAuth2 auth, REST client, event mapping, `Connector` impl.

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

use crate::api::FitbitClient;
use crate::auth::{FitbitOAuth2, KeychainTokenStore, TokenStore};
use crate::events::FitbitEventMapper;

/// Fitbit connector.
pub struct FitbitConnector {
    manifest: ConnectorManifest,
    account_id: Uuid,
    #[allow(dead_code)]
    token_store: Arc<dyn TokenStore>,
    #[allow(dead_code)]
    oauth: Option<Arc<FitbitOAuth2>>,
    client: Mutex<FitbitClient>,
}

pub struct FitbitConnectorBuilder {
    #[allow(dead_code)]
    client_id: String,
    #[allow(dead_code)]
    client_secret: String,
    account_id: Uuid,
    token_store: Option<Arc<dyn TokenStore>>,
    oauth: Option<Arc<FitbitOAuth2>>,
    http: Option<reqwest::Client>,
}

impl FitbitConnectorBuilder {
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

    pub fn oauth(mut self, o: Arc<FitbitOAuth2>) -> Self {
        self.oauth = Some(o);
        self
    }

    pub fn http(mut self, h: reqwest::Client) -> Self {
        self.http = Some(h);
        self
    }

    pub fn build(self) -> FitbitConnector {
        let http = self.http.unwrap_or_default();
        let store = self
            .token_store
            .unwrap_or_else(|| Arc::new(KeychainTokenStore::new()));
        let client = FitbitClient::new(http);
        FitbitConnector {
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
        id: "fitbit".to_string(),
        version: "0.1.0".to_string(),
        display_name: "Fitbit".to_string(),
        auth_strategy: AuthStrategy::OAuth2 {
            scopes: vec![
                "activity".to_string(),
                "sleep".to_string(),
                "heartrate".to_string(),
            ],
        },
        sync_mode: SyncMode::Polling {
            cadence_seconds: 300,
        },
        capabilities: vec![],
        entity_types: vec!["workout".to_string(), "sleep".to_string(), "heart_rate".to_string()],
        event_types: vec![
            "fitbit:workout_completed".to_string(),
            "fitbit:sleep_reported".to_string(),
            "fitbit:daily_steps_milestone".to_string(),
            "fitbit:heart_rate_resting".to_string(),
        ],
        tier: VerificationTier::Verified,
        health_indicators: vec!["last_sync_ok".to_string(), "auth_token_fresh".to_string()],
    }
}

#[async_trait]
impl Connector for FitbitConnector {
    fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }

    async fn health(&self) -> HealthState {
        let client = self.client.lock().await;
        match client.get_profile().await {
            Ok(_) => HealthState::Healthy,
            Err(ConnectorError::Authentication { .. }) => HealthState::Unauthenticated,
            Err(e) => HealthState::Failing(e.to_string()),
        }
    }

    async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
        let client = self.client.lock().await;
        let mapper = FitbitEventMapper::new(self.account_id);
        let mut events = Vec::new();

        // Fetch today's activities + sleep + heart rate
        debug!("Fitbit: fetching today's activities, sleep, heart rate");

        match client.get_activities_today().await {
            Ok(activities) => {
                events.extend(mapper.map_activities(activities));
            }
            Err(e) => warn!("Fitbit: activity sync failed: {}", e),
        }

        match client.get_sleep_today().await {
            Ok(sleep) => {
                events.extend(mapper.map_sleep(sleep));
            }
            Err(e) => warn!("Fitbit: sleep sync failed: {}", e),
        }

        match client.get_heart_rate_today().await {
            Ok(heart_rate) => {
                events.extend(mapper.map_heart_rate(heart_rate));
            }
            Err(e) => warn!("Fitbit: heart rate sync failed: {}", e),
        }

        info!("Fitbit: synced {} events", events.len());

        Ok(SyncOutcome {
            events,
            next_cursor: None,
            partial: false,
        })
    }
}
