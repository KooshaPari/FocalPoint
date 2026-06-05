//! FocalPoint xtask — day-1 dev ergonomics.
//!
//! Run with `cargo xtask <subcommand>` after the alias is added to
//! `.cargo/config.toml`.  The first subcommand is `info` which prints a
//! compact summary of the workspace so a new contributor can confirm the
//! toolchain + crate layout in one command instead of grepping the
//! `crates/` and `tooling/` directories by hand.

use std::path::Path;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "xtask",
    about = "FocalPoint day-1 dev ergonomics",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print a compact summary of the workspace (rustc + cargo + crate counts).
    Info,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Info => info()?,
    }
    Ok(())
}

fn info() -> Result<()> {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let rustc_out = std::process::Command::new(&rustc)
        .arg("--version")
        .output()
        .context("running rustc --version")?;
    print!(
        "{}",
        String::from_utf8_lossy(&rustc_out.stdout).trim_end()
    );

    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let cargo_out = std::process::Command::new(&cargo)
        .arg("--version")
        .output()
        .context("running cargo --version")?;
    print!(
        " / {}",
        String::from_utf8_lossy(&cargo_out.stdout).trim_end()
    );

    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string());
    println!(" / target={}", target_dir);

    // Count crates in the workspace by scanning the well-known dirs.
    let repo_root = std::env::current_dir().context("locating repo root")?;
    let crates_count = count_dirs(&repo_root.join("crates"));
    let tooling_count = count_dirs(&repo_root.join("tooling"));
    println!("crates  : {crates_count}");
    println!("tooling : {tooling_count}");

    Ok(())
}

fn count_dirs(path: &Path) -> usize {
    let entries = match std::fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return 0,
    };
    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .count()
}
