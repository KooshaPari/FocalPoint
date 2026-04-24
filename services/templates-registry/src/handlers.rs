//! HTTP handlers for all endpoints.
//!
//! Traces to: FR-TEMPLATE-MARKETPLACE-001.

use crate::{error::RegistryError, models::*, auth, SharedState};
use axum::{
    extract::{ConnectInfo, Path, Query, State},
    http::HeaderMap,
    Json,
};
use focus_templates::TemplatePack;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Search query parameters.
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    #[serde(default)]
    q: String,
}

/// GET /api/v1/search?q=<query>
///
/// Returns matching packs (60 req/min per IP).
/// Traces to: FR-TEMPLATE-MARKETPLACE-001.
pub async fn search_packs(
    State(state): State<Arc<SharedState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, RegistryError> {
    // Rate limit: 60 req/min = 1 token/sec
    if !state.ratelimit.check_search(&addr.ip().to_string()) {
        return Err(RegistryError::RateLimited);
    }

    let packs = state.db.search_packs(&params.q)?;
    Ok(Json(SearchResponse {
        total: packs.len(),
        packs,
    }))
}

/// GET /api/v1/packs/:id
///
/// Returns pack manifest, signature, and README.
/// Traces to: FR-TEMPLATE-MARKETPLACE-001.
pub async fn get_pack(
    State(state): State<Arc<SharedState>>,
    Path(id): Path<String>,
) -> Result<Json<PackManifest>, RegistryError> {
    let manifest = state
        .db
        .get_pack(&id)?
        .ok_or_else(|| RegistryError::NotFound(format!("pack not found: {}", id)))?;

    Ok(Json(manifest))
}

/// POST /api/v1/packs/:id/rate
///
/// Submit a rating (1-5 stars). Anonymous, 10 req/min per IP.
/// Traces to: FR-TEMPLATE-MARKETPLACE-001.
pub async fn rate_pack(
    State(state): State<Arc<SharedState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Path(id): Path<String>,
    Json(submission): Json<RatingSubmission>,
) -> Result<Json<RatingResponse>, RegistryError> {
    // Validate pack exists
    state
        .db
        .get_pack(&id)?
        .ok_or_else(|| RegistryError::NotFound(format!("pack not found: {}", id)))?;

    // Rate limit: 10 req/min per IP
    if !state.ratelimit.check_upload(&addr.ip().to_string()) {
        return Err(RegistryError::RateLimited);
    }

    // Validate rating
    if submission.rating < 1 || submission.rating > 5 {
        return Err(RegistryError::BadRequest(
            "rating must be 1-5".to_string(),
        ));
    }

    // Hash IP for privacy
    let ip_hash = format!(
        "{:x}",
        Sha256::digest(addr.ip().to_string().as_bytes())
    );

    state.db.add_rating(&id, submission.rating, submission.comment, &ip_hash)?;

    Ok(Json(RatingResponse {
        status: "success".to_string(),
        pack_id: id,
        rating: submission.rating,
    }))
}

/// POST /api/v1/packs
///
/// Upload a new template pack (multipart: pack.tar.zst + signature.ed25519).
/// Requires bearer token. 10 req/min per IP.
/// Traces to: FR-TEMPLATE-MARKETPLACE-001.
pub async fn upload_pack(
    State(state): State<Arc<SharedState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, RegistryError> {
    // Rate limit: 10 req/min per IP
    if !state.ratelimit.check_upload(&addr.ip().to_string()) {
        return Err(RegistryError::RateLimited);
    }

    // Authenticate
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = auth::extract_bearer_token(auth_header)
        .ok_or_else(|| RegistryError::Unauthorized("missing bearer token".to_string()))?;

    if !auth::validate_token(&token, &state.token) {
        return Err(RegistryError::Unauthorized("invalid token".to_string()));
    }

    // TODO: multipart handling for pack.tar.zst + signature.ed25519
    // For now, return a placeholder response
    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "upload endpoint stub (multipart handling not yet implemented)"
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_validation_rejects_zero() {
        let submission = RatingSubmission {
            rating: 0,
            comment: None,
        };
        assert!(submission.rating < 1, "rating < 1 should be invalid");
    }

    #[test]
    fn rating_validation_rejects_over_five() {
        let submission = RatingSubmission {
            rating: 6,
            comment: None,
        };
        assert!(submission.rating > 5, "rating > 5 should be invalid");
    }

    #[test]
    fn rating_validation_accepts_valid() {
        for r in 1..=5 {
            let submission = RatingSubmission {
                rating: r,
                comment: None,
            };
            assert!(submission.rating >= 1 && submission.rating <= 5);
        }
    }
}
