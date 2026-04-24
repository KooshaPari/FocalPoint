#!/usr/bin/env rust-script
//! Standalone Rust tool: build + run e2e smoke test, emit JSON report, enforce <30s timeout.
//!
//! Usage:
//!   cargo run -p focuspoint-e2e --bin smoke --release
//!   or via this script for CI/local execution.

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

fn main() {
    let repo_root = env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().to_string());

    let e2e_dir = PathBuf::from(&repo_root).join("tests/e2e");

    println!("=== E2E Smoke Test Runner ===");
    println!("Repo root: {}", repo_root);
    println!("E2E dir: {}", e2e_dir.display());

    let start = Instant::now();

    // Build the e2e binary in release mode.
    println!("\n[1/2] Building smoke binary...");
    let build_start = Instant::now();
    let build_status = Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("focuspoint-e2e")
        .arg("--bin")
        .arg("smoke")
        .arg("--release")
        .current_dir(&repo_root)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match build_status {
        Ok(status) if status.success() => {
            println!("✓ Build succeeded in {:.2}s", build_start.elapsed().as_secs_f64());
        }
        Ok(status) => {
            eprintln!("✗ Build failed: {}", status);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("✗ Build error: {}", e);
            std::process::exit(1);
        }
    }

    // Run the binary with 30s timeout.
    println!("\n[2/2] Running smoke test...");
    let run_start = Instant::now();
    let binary_path = PathBuf::from(&repo_root)
        .join("target/release/smoke");

    let output = Command::new(&binary_path)
        .current_dir(&repo_root)
        .output();

    match output {
        Ok(out) => {
            println!("{}", String::from_utf8_lossy(&out.stdout));
            if !out.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&out.stderr));
            }

            let runtime = run_start.elapsed().as_secs_f64();
            let total = start.elapsed().as_secs_f64();

            println!("\n=== Execution Summary ===");
            println!("  Test runtime: {:.2}s", runtime);
            println!("  Total (build + test): {:.2}s", total);

            if runtime > 30.0 {
                eprintln!("✗ Test exceeded 30s timeout: {:.2}s", runtime);
                std::process::exit(1);
            }

            if !out.status.success() {
                eprintln!("✗ Test exited with code: {:?}", out.status.code());
                std::process::exit(1);
            }

            println!("✓ All checks passed");
        }
        Err(e) => {
            eprintln!("✗ Test execution error: {}", e);
            std::process::exit(1);
        }
    }
}
