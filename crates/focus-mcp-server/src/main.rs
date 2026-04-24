//! MCP (Model Context Protocol) server exposing FocalPoint's rule/task/wallet/audit surface.
//!
//! Exposes 13 tools + resources for agent-driven planning, rule authoring, and diagnostics.
//! STDIO transport by default; optional SSE transport via `--sse` flag.
//!
//! Usage:
//!   focalpoint-mcp-server [--db `<path>`] [--sse]
//!
//! Env:
//!   FOCALPOINT_DB — path to core.db (overrides --db flag and platform default)

mod server;
mod tools;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "focalpoint-mcp-server")]
#[command(about = "MCP server for FocalPoint: tasks, rules, wallet, audit, templates, connectors")]
struct Cli {
    /// Transport mode: stdio (default) or socket.
    #[arg(long, value_name = "MODE", default_value = "stdio")]
    mode: String,

    /// Path to FocalPoint core.db. Defaults to FOCALPOINT_DB env var or platform default.
    #[arg(long)]
    db: Option<PathBuf>,

    /// Bind address for socket mode (Unix path or host:port).
    #[arg(long, value_name = "BIND")]
    bind: Option<String>,

    /// Enable HTTP+SSE transport instead of default STDIO.
    #[arg(long)]
    sse: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    let db_path = cli.db.or_else(|| {
        std::env::var("FOCALPOINT_DB")
            .ok()
            .map(PathBuf::from)
    }).or_else(|| {
        // Platform default: macOS Application Support
        #[cfg(target_os = "macos")]
        {
            let mut path = dirs::home_dir()?;
            path.push("Library/Application Support/focalpoint/core.db");
            Some(path)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let mut path = dirs::data_local_dir()?;
            path.push("focalpoint/core.db");
            Some(path)
        }
    });

    if let Some(path) = &db_path {
        tracing::info!("Using database: {}", path.display());
    } else {
        anyhow::bail!("No database path found. Set --db or FOCALPOINT_DB.");
    }

    let db_path = db_path.unwrap();

    // Route based on mode parameter
    match cli.mode.as_str() {
        "stdio" => {
            tracing::info!("Running MCP server in STDIO mode");
            server::run_stdio(db_path).await?;
        }
        "socket" => {
            let bind_addr = cli.bind.clone().unwrap_or_else(|| {
                // Platform default for Unix socket
                #[cfg(unix)]
                {
                    format!("{}/mcp.sock",
                        dirs::config_local_dir()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|| "/tmp".to_string()))
                }
                #[cfg(not(unix))]
                {
                    "127.0.0.1:9090".to_string()
                }
            });
            tracing::info!("Running MCP server in socket mode at {}", bind_addr);
            tracing::warn!("Socket mode is experimental; use STDIO mode for production");
            // Socket implementation stub
            anyhow::bail!("Socket mode implementation deferred; use --mode stdio for now");
        }
        _ => {
            anyhow::bail!("Invalid mode '{}'. Valid modes: stdio, socket", cli.mode);
        }
    }

    Ok(())
}
