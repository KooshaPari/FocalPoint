#![forbid(unsafe_code)]

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

connector_builder! {
    builder: FitbitConnectorBuilder,
    token_store: dyn TokenStore,
    oauth: FitbitOAuth2,
}

impl FitbitConnectorBuilder {
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
        id: "fitbit".into(),
        version: "0.1.0".into(),
        display_name: "Fitbit".into(),
        auth_strategy: AuthStrategy::OAuth2 {
            scopes: vec!["activity".into(), "sleep".into(), "heartrate".into()],
        },
        sync_mode: SyncMode::Polling {
            cadence_seconds: 300,
        },
        capabilities: vec![],
        entity_types: vec!["workout".into(), "sleep".into(), "heart_rate".into()],
        event_types: vec![
            "fitbit:workout_completed".into(),
            "fitbit:sleep_reported".into(),
            "fitbit:daily_steps_milestone".into(),
            "fitbit:heart_rate_resting".into(),
        ],
        tier: VerificationTier::Verified,
        health_indicators: vec!["last_sync_ok".into(), "auth_token_fresh".into()],
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
            Err(ConnectorError::Unauthorized(_)) => HealthState::Unauthenticated,
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
