//! Enforcement policy generation from rule decisions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementPolicy {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub block_profile: BlockProfile,
    pub app_targets: Vec<AppTarget>,
    pub scheduled_windows: Vec<Window>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProfile {
    pub name: String,
    pub categories: Vec<String>,
    pub exceptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppTarget {
    Category(String),
    BundleId(String),
    PackageName(String),
    Domain(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub starts_at: chrono::DateTime<chrono::Utc>,
    pub ends_at: chrono::DateTime<chrono::Utc>,
}

pub struct PolicyBuilder {
    // Stub
}

impl PolicyBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
