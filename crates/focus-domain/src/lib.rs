//! Canonical domain entities, IDs, aggregate roots, invariants.
//!
//! No persistence, no I/O. Pure types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub type Result<T, E = DomainError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invariant violation: {0}")]
    Invariant(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub created_at: DateTime<Utc>,
    pub display_name: String,
    pub primary_device_id: Option<DeviceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: DeviceId,
    pub user_id: UserId,
    pub platform: Platform,
    pub os_version: String,
    pub enrolled_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Ios,
    Android,
    Macos,
    Unknown,
}
