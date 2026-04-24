use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use focus_connectors::WebhookRegistry;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

mod handler;

#[derive(Parser)]
#[command(name = "focalpoint-webhook-server")]
#[command(about = "FocalPoint webhook receiver for GitHub, Canvas, GCal")]
struct Args {
    /// Bind address (default: 127.0.0.1:8472)
    #[arg(long, default_value = "127.0.0.1:8472")]
    bind: String,

    /// Path to core.db (shared with CLI)
    #[arg(long, default_value = "")]
    db: String,
}

#[derive(Clone)]
struct AppState {
    registry: Arc<WebhookRegistry>,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Debug, Deserialize)]
struct WebhookPayload {
    // Generic webhook payload — structure varies per provider
    #[serde(default)]
    #[allow(dead_code)]
    raw: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = Args::parse();
    info!(bind = %args.bind, "starting focalpoint-webhook-server");

    if !args.db.is_empty() {
        debug!(db_path = %args.db, "using core.db");
    }

    // Initialize webhook registry and handlers
    let registry = Arc::new(WebhookRegistry::new());
    register_default_handlers(&registry).await;

    let state = AppState {
        registry: registry.clone(),
    };

    // Build router
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/webhooks/:connector_id", post(webhook_handler))
        .with_state(state);

    // Bind and serve
    let listener = tokio::net::TcpListener::bind(&args.bind).await?;
    info!(
        bind = %args.bind,
        "listening for webhook deliveries"
    );

    axum::serve(listener, app).await?;

    Ok(())
}

async fn healthz() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn webhook_handler(
    Path(connector_id): Path<String>,
    State(state): State<AppState>,
    body: bytes::Bytes,
) -> impl IntoResponse {
    debug!(connector_id = %connector_id, "received webhook");

    // Dispatch to handler
    match handler::handle_webhook(&state.registry, &connector_id, body.to_vec()).await {
        Ok(_events) => {
            debug!(connector_id = %connector_id, "webhook processed successfully");
            (StatusCode::ACCEPTED, "").into_response()
        }
        Err(handler::WebhookError::SignatureInvalid) => {
            warn!(connector_id = %connector_id, "invalid webhook signature");
            (StatusCode::UNAUTHORIZED, "invalid signature").into_response()
        }
        Err(handler::WebhookError::UnknownConnector) => {
            warn!(connector_id = %connector_id, "unknown connector");
            (StatusCode::NOT_FOUND, "unknown connector").into_response()
        }
        Err(e) => {
            error!(connector_id = %connector_id, error = %e, "webhook processing failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "processing error").into_response()
        }
    }
}

/// Register default webhook handlers (GitHub, Canvas stub, GCal stub).
/// Reads env vars to configure verifiers; missing env = handler not registered.
async fn register_default_handlers(registry: &WebhookRegistry) {
    use std::collections::HashMap;

    // GitHub handler
    if let Ok(secret) = std::env::var("FOCALPOINT_GITHUB_WEBHOOK_SECRET") {
        info!("registering github webhook handler");
        let verifier = Arc::new(
            focus_connectors::signature_verifiers::GitHubHmacVerifier {
                secret: secrecy::SecretString::new(secret),
            },
        );
        let handler = Arc::new(handler::GitHubHandlerImpl {
            account_id: uuid::Uuid::nil(), // TODO: extract from config
            verifier,
        });
        registry.register("github", handler);
    } else {
        warn!("FOCALPOINT_GITHUB_WEBHOOK_SECRET not set; github handler not registered");
    }

    // Canvas handler (stub)
    if let Ok(jwks_url) = std::env::var("FOCALPOINT_CANVAS_JWKS_URL") {
        info!("registering canvas webhook handler (stub)");
        let verifier = Arc::new(
            focus_connectors::signature_verifiers::CanvasLtiVerifier { jwks_url },
        );
        let handler = Arc::new(handler::CanvasHandlerImpl {
            account_id: uuid::Uuid::nil(),
            verifier,
        });
        registry.register("canvas", handler);
    } else {
        warn!("FOCALPOINT_CANVAS_JWKS_URL not set; canvas handler not registered");
    }

    // GCal handler (stub)
    if let Ok(channel_token) = std::env::var("FOCALPOINT_GCAL_CHANNEL_TOKEN") {
        info!("registering google calendar webhook handler (stub)");
        let verifier = Arc::new(
            focus_connectors::signature_verifiers::GCalChannelVerifier {
                channel_token: secrecy::SecretString::new(channel_token),
            },
        );
        let handler = Arc::new(handler::GCalHandlerImpl {
            account_id: uuid::Uuid::nil(),
            verifier,
        });
        registry.register("gcal", handler);
    } else {
        warn!("FOCALPOINT_GCAL_CHANNEL_TOKEN not set; gcal handler not registered");
    }
}
