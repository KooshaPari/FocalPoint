//! GraphQL API gateway for FocalPoint.
//!
//! Exposes core read APIs (tasks, rules, wallet, audit, connectors, sessions)
//! and mutations (markTaskDone, enableRule, triggerSync) via GraphQL.
//! Subscriptions deliver live audit feed via WebSocket.
//!
//! Auth: Bearer token (env: `FOCALPOINT_GRAPHQL_TOKEN`).
//! Introspection: enabled in dev, gated by `FOCALPOINT_GRAPHQL_PROD` env flag.

pub mod auth;
pub mod model;
pub mod rate_limit;
pub mod subscription;

use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;

/// Shared application state for the GraphQL gateway.
#[derive(Clone)]
pub struct AppState {
    /// Bearer token for auth (from env).
    pub auth_token: Option<String>,
    /// Whether introspection is enabled (gated by FOCALPOINT_GRAPHQL_PROD).
    pub introspection_enabled: bool,
    /// Rate limiter: per-client tracking.
    pub rate_limiter: Arc<rate_limit::RateLimiter>,
}

impl AppState {
    /// Create a new AppState from environment variables.
    pub fn from_env() -> Result<Self> {
        let auth_token = std::env::var("FOCALPOINT_GRAPHQL_TOKEN").ok();
        let prod_mode = std::env::var("FOCALPOINT_GRAPHQL_PROD")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);

        Ok(Self {
            auth_token,
            introspection_enabled: !prod_mode,
            rate_limiter: Arc::new(rate_limit::RateLimiter::new()),
        })
    }
}

/// Build the GraphQL schema.
pub fn build_schema() -> Schema<model::Query, model::Mutation, EmptySubscription> {
    Schema::build(model::Query, model::Mutation, EmptySubscription).finish()
}

/// Health check endpoint.
async fn health() -> impl IntoResponse {
    StatusCode::OK
}

/// Create the main router with GraphQL and health check.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .with_state(state)
}
