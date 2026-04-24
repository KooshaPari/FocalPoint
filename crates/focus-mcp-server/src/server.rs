//! MCP server transport layer: STDIO (default) and optional SSE.

use crate::tools::FocalPointToolsImpl;
use anyhow::Result;
use mcp_sdk::server::Server;
use mcp_sdk::transport::{Transport, ServerStdioTransport};
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

    // Create STDIO transport
    let transport = ServerStdioTransport;
    transport.open()?;

    // Create tool implementations
    let tools_impl = FocalPointToolsImpl::new(adapter);

    // Build the MCP server with tools
    let server = Server::builder(transport)
        .name("focalpoint-mcp-server")
        .version(env!("CARGO_PKG_VERSION"))
        .tools(tools_impl.build_mcp_tools())
        .build();

    // Run the server (listen for incoming requests)
    server.listen().await?;

    Ok(())
}

/// Run the MCP server over HTTP+SSE transport (requires `http-sse` feature).
#[cfg(feature = "http-sse")]
#[allow(dead_code)]
pub async fn run_sse(db_path: PathBuf) -> Result<()> {
    info!("Starting FocalPoint MCP server (HTTP+SSE)");

    // Load the SQLite adapter (blocking, so run it in a task)
    let adapter = tokio::task::spawn_blocking(move || {
        focus_storage::SqliteAdapter::open(&db_path)
    })
    .await??;

    // HTTP+SSE transport would be implemented here
    // For now, just a placeholder
    anyhow::bail!("SSE transport not yet implemented");
}

#[cfg(not(feature = "http-sse"))]
pub async fn run_sse(_db_path: PathBuf) -> Result<()> {
    anyhow::bail!("SSE transport requires feature 'http-sse'");
}
