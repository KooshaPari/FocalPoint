//! MCP server transport layer: STDIO (default) and optional SSE.

use crate::tools::FocalPointTools;
use anyhow::Result;
use mcp_sdk::server::Server;
use mcp_sdk::transport::stdio::StdioTransport;
use std::path::PathBuf;
use tracing::info;

/// Run the MCP server over STDIO transport (default).
pub async fn run_stdio(db_path: PathBuf) -> Result<()> {
    info!("Starting FocalPoint MCP server (STDIO)");

    // Load the SQLite adapter (blocking, so run it in a task)
    let adapter = tokio::task::spawn_blocking(move || {
        focus_storage::SqliteAdapter::open(&db_path)
    })
    .await??;

    let tools = FocalPointTools::new(adapter);

    // Create the MCP server
    let mut server = Server::new();

    // Register all 13 tools
    tools.register_tools(&mut server);

    // Create STDIO transport
    let transport = StdioTransport::new();

    // Start the server
    server.run(transport).await?;

    Ok(())
}

/// Run the MCP server over HTTP+SSE transport (requires `http-sse` feature).
#[cfg(feature = "http-sse")]
pub async fn run_sse(db_path: PathBuf) -> Result<()> {
    info!("Starting FocalPoint MCP server (HTTP+SSE)");

    // Load the SQLite adapter (blocking, so run it in a task)
    let adapter = tokio::task::spawn_blocking(move || {
        focus_storage::SqliteAdapter::open(&db_path)
    })
    .await??;

    let tools = FocalPointTools::new(adapter);

    // Create the MCP server
    let mut server = Server::new();

    // Register all 13 tools
    tools.register_tools(&mut server);

    // HTTP+SSE transport would be implemented here
    // For now, just a placeholder
    anyhow::bail!("SSE transport not yet implemented");
}

#[cfg(not(feature = "http-sse"))]
pub async fn run_sse(_db_path: PathBuf) -> Result<()> {
    anyhow::bail!("SSE transport requires feature 'http-sse'");
}
