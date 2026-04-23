//! Connector trait, manifest, auth, sync contracts.

pub mod mcp_bridge;

use async_trait::async_trait;
use focus_events::NormalizedEvent;
use serde::{Deserialize, Serialize};
use thiserror::Error;

fn default_verification_tier() -> VerificationTier {
    VerificationTier::Verified
}

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
    /// 401 Unauthorized — token invalid, revoked, or expired and not
    /// refreshable (e.g. GitHub PAT). Distinct from `Auth` so callers can
    /// surface a dedicated "reconnect" UI path.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// 403 Forbidden for reasons other than rate-limiting (scope/permission).
    #[error("forbidden: {0}")]
    Forbidden(String),
    /// Rate-limited with an absolute reset timestamp (e.g. GitHub's
    /// `X-RateLimit-Reset`). Prefer this over `RateLimited(u64)` when the
    /// upstream provides an absolute deadline.
    #[error("rate_limited_until: {0}")]
    RateLimitedUntil(chrono::DateTime<chrono::Utc>),
}

pub type Result<T> = std::result::Result<T, ConnectorError>;

/// Verification tier for a connector — how much we vouch for the implementation.
///
/// Traces to: FR-CONN-TIER-001.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationTier {
    /// First-party, shipped in-tree.
    Official,
    /// Community contribution we reviewed and signed.
    #[default]
    Verified,
    /// User pointed us at an arbitrary MCP server.
    MCPBridged,
    /// User-hosted, local to the user's machine.
    Private,
}

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
    /// Traces to: FR-CONN-TIER-001. Defaults to `Verified` so older manifests
    /// without this field deserialize to a safe middle ground.
    #[serde(default = "default_verification_tier")]
    pub tier: VerificationTier,
    /// Flagged in the arch audit — declared health-signal names the connector
    /// exposes (e.g. `["last_sync_ok", "auth_token_fresh"]`). Optional and
    /// informational; defaults to empty.
    #[serde(default)]
    pub health_indicators: Vec<String>,
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
