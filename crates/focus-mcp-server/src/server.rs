//! MCP server transport layer: STDIO (default) and optional SSE.

use crate::tools::FocalPointTools;
use anyhow::Result;
use mcp_sdk::server::ServerBuilder;
use mcp_sdk::transport::StdioTransport;
use mcp_sdk::tools::Tools;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

/// Run the MCP server over STDIO transport (default).
pub async fn run_stdio(db_path: PathBuf) -> Result<()> {
    info!("Starting FocalPoint MCP server (STDIO)");

    // Load the SQLite adapter (blocking, so run it in a task)
    let adapter = tokio::task::spawn_blocking(move || {
        focus_storage::SqliteAdapter::open(&db_path)
    })
    .await??;

    let tools = Arc::new(FocalPointTools::new(adapter));

    // Create STDIO transport
    let transport = StdioTransport::new();

    // Build the MCP server
    let mcp_tools = tools.build_mcp_tools();
    let server = ServerBuilder::new(transport)
        .name("focalpoint-mcp-server")
        .version(env!("CARGO_PKG_VERSION"))
        .tools(mcp_tools)
        .build();

    // Run the server
    server.run().await?;

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

    let tools = Arc::new(FocalPointTools::new(adapter));

    // HTTP+SSE transport would be implemented here
    // For now, just a placeholder
    anyhow::bail!("SSE transport not yet implemented");
}

#[cfg(not(feature = "http-sse"))]
pub async fn run_sse(_db_path: PathBuf) -> Result<()> {
    anyhow::bail!("SSE transport requires feature 'http-sse'");
}
