//! GitHub REST domain types — minimal subset of `/user` and `/users/{login}/events`.
//!
//! Field names match the GitHub REST v3 / 2022-11-28 API. We model only what
//! the connector maps into focus-events; everything else is either ignored
//! via serde's default behaviour for untagged extra fields or dropped into
//! the untyped `payload` passthrough.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// `GET /user` response (the authenticated user).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    #[serde(default)]
    pub name: Option<String>,
}

/// `GET /users/{login}/events` item.
///
/// GitHub wraps the polymorphic event body in `payload` whose shape depends
/// on `type`. We keep `payload` as a raw JSON value and pick it apart in the
/// event mapper — this keeps the model stable even as GitHub evolves event
/// shapes (they change more often than the `type` enum).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub actor: GitHubActor,
    pub repo: GitHubRepo,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub public: bool,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubActor {
    pub id: u64,
    pub login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepo {
    pub id: u64,
    pub name: String,
}
