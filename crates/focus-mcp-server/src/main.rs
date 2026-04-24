//! MCP (Model Context Protocol) server exposing FocalPoint's rule/task/wallet/audit surface.
//!
//! Exposes 13 tools + resources for agent-driven planning, rule authoring, and diagnostics.
//! STDIO transport by default; optional SSE transport via `--sse` flag.
//!
//! Usage:
//!   focalpoint-mcp-server [--db <path>] [--sse]
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
    /// Path to FocalPoint core.db. Defaults to FOCALPOINT_DB env var or platform default.
    #[arg(long)]
    db: Option<PathBuf>,

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

    if cli.sse {
        #[cfg(feature = "http-sse")]
        {
            server::run_sse(db_path).await?;
        }
        #[cfg(not(feature = "http-sse"))]
        {
            anyhow::bail!("SSE transport requires feature 'http-sse'. Rebuild with: cargo build --features http-sse");
        }
    } else {
        server::run_stdio(db_path).await?;
    }

    Ok(())
}
