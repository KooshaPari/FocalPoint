use anyhow::Result;
use focus_graphql_gateway::{router, AppState};
use std::net::SocketAddr;
use tracing::info;

struct Args {
    /// Bind address (default: 127.0.0.1:8473)
    bind: String,
}

impl Args {
    fn parse() -> Self {
        let bind = std::env::var("BIND").unwrap_or_else(|_| "127.0.0.1:8473".to_string());
        Args { bind }
    }
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
    let addr: SocketAddr = args.bind.parse()?;

    let state = AppState::from_env()?;
    info!(
        bind = %addr,
        introspection = state.introspection_enabled,
        auth_configured = state.auth_token.is_some(),
        "starting focalpoint-graphql-gateway"
    );

    let app = router(state);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("listening on http://{}", addr);
    info!("GraphQL schema: http://{}/graphql", addr);
    info!("WebSocket subscriptions: ws://{}/graphql/ws", addr);
    info!("Health check: http://{}/health", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
