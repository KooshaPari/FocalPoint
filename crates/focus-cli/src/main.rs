//! Dev CLI: sync runner, rule replay, audit inspect.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "focus", about = "FocalPoint dev CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run a connector sync once.
    Sync {
        #[arg(long)]
        connector: String,
    },
    /// Replay rule evaluation against a fixture event stream.
    Replay {
        #[arg(long)]
        rule_id: String,
        #[arg(long)]
        fixture: String,
    },
    /// Inspect the audit chain.
    Audit {
        #[arg(long)]
        verify: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Sync { connector } => {
            println!("sync {connector} — not implemented");
        }
        Cmd::Replay { rule_id, fixture } => {
            println!("replay rule={rule_id} fixture={fixture} — not implemented");
        }
        Cmd::Audit { verify } => {
            println!("audit verify={verify} — not implemented");
        }
    }
    Ok(())
}
