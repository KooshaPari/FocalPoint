//! Connector trait, manifest, auth, sync contracts.

use async_trait::async_trait;
use focus_events::NormalizedEvent;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("auth: {0}")]
    Auth(String),
    #[error("network: {0}")]
    Network(String),
    #[error("schema: {0}")]
    Schema(String),
    #[error("rate_limited: retry after {0}s")]
    RateLimited(u64),
}

pub type Result<T> = std::result::Result<T, ConnectorError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorManifest {
    pub id: String,
    pub version: String,
    pub display_name: String,
    pub auth_strategy: AuthStrategy,
    pub sync_mode: SyncMode,
    pub capabilities: Vec<ConnectorCapability>,
    pub entity_types: Vec<String>,
    pub event_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthStrategy {
    OAuth2 { scopes: Vec<String> },
    ApiKey,
    DeviceBrokered,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncMode {
    Polling { cadence_seconds: u64 },
    Webhook,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorCapability {
    pub name: String,
    pub params_schema: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,
    Degraded(String),
    Unauthenticated,
    Failing(String),
}

#[async_trait]
pub trait Connector: Send + Sync {
    fn manifest(&self) -> &ConnectorManifest;

    async fn health(&self) -> HealthState;

    async fn sync(&self, cursor: Option<String>) -> Result<SyncOutcome>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOutcome {
    pub events: Vec<NormalizedEvent>,
    pub next_cursor: Option<String>,
    pub partial: bool,
}
