//! Templates Registry Service — marketplace server for FocalPoint template packs.
//!
//! Endpoints:
//! - GET /api/v1/search?q=<query> — search packs by name/author (rate: 60 req/min)
//! - GET /api/v1/packs/:id — fetch pack manifest + signature + README
//! - POST /api/v1/packs/:id/rate — submit rating (anonymous, 10 req/min)
//! - POST /api/v1/packs — upload new pack (requires bearer token, 10 req/min)
//!
//! Traces to: FR-TEMPLATE-MARKETPLACE-001.

mod db;
mod handlers;
mod models;
mod ratelimit;
mod auth;
mod error;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let bind_addr = std::env::var("TEMPLATES_REGISTRY_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8080".into());

    let db_path = std::env::var("TEMPLATES_REGISTRY_DB")
        .unwrap_or_else(|_| "/tmp/templates.db".into());

    let token = std::env::var("TEMPLATES_REGISTRY_TOKEN")
        .unwrap_or_else(|_| "dev-token-change-in-prod".into());

    let catalog_path = std::env::var("TEMPLATES_CATALOG_PATH")
        .unwrap_or_else(|_| "examples/templates".into());

    // Initialize database
    let db = Arc::new(db::TemplatesDb::new(&db_path)?);
    db.init_schema()?;

    // Load initial catalog (scan examples/templates/)
    if let Ok(count) = db.load_catalog_from_path(&catalog_path) {
        info!("Loaded {} template packs from catalog", count);
    }

    let shared_state = Arc::new(SharedState {
        db,
        token: token.clone(),
        ratelimit: ratelimit::RateLimiter::new(),
    });

    // Router
    let app = Router::new()
        .route("/api/v1/search", get(handlers::search_packs))
        .route("/api/v1/packs/:id", get(handlers::get_pack))
        .route("/api/v1/packs/:id/rate", post(handlers::rate_pack))
        .route("/api/v1/packs", post(handlers::upload_pack))
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Templates Registry listening on {}", bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

pub struct SharedState {
    pub db: Arc<db::TemplatesDb>,
    pub token: String,
    pub ratelimit: ratelimit::RateLimiter,
}
