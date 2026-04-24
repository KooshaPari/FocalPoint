//! Data models for the templates registry.

use serde::{Deserialize, Serialize};

/// Search response for template packs.
///
/// Traces to: FR-TEMPLATE-MARKETPLACE-001.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub packs: Vec<PackSummary>,
    pub total: usize,
}

/// Summary of a template pack (for search/list results).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackSummary {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub sha256: String,
    pub signed_by: Option<String>,
    pub avg_rating: Option<f32>,
    pub rating_count: usize,
}

/// Detailed pack manifest (returned by GET /packs/:id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub sha256: String,
    /// base64-encoded ed25519 signature (detached).
    pub signature: Option<String>,
    pub signed_by: Option<String>,
    pub readme: Option<String>,
    pub avg_rating: Option<f32>,
    pub rating_count: usize,
}

/// Rating submission (POST body).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingSubmission {
    pub rating: u8, // 1-5
    pub comment: Option<String>,
}

/// Pack rating (stored in DB).
#[derive(Debug, Clone)]
pub struct Rating {
    pub id: String,
    pub pack_id: String,
    pub rating: u8,
    pub comment: Option<String>,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub ip_hash: String, // SHA-256 of IP for rating limit tracking
}

/// Error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub detail: Option<String>,
}

/// Success response for rating submission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingResponse {
    pub status: String,
    pub pack_id: String,
    pub rating: u8,
}

/// Success response for upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub status: String,
    pub id: String,
    pub sha256: String,
}
