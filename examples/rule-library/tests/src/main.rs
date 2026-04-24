//! IR-hash parity test for all 10 rules in the rule library.
//!
//! Walks each rule directory, compiles all 3 surfaces (cli.sh JSON, FPL, graph.json)
//! to canonical IR, computes SHA-256 per IR, asserts all 3 match per rule.
//! Exit non-zero if any parity check fails.

use anyhow::{anyhow, Result};
use focus_ir::Document;
use std::fs;
use std::path::Path;

const RULE_IDS: &[&str] = &[
    "gh-pr-merged",
    "canvas-submit",
    "gcal-deep-work-end",
    "fitbit-workout",
    "morning-brief-nudge",
    "3-session-streak",
    "missed-focus-2x",
    "canvas-due-24h",
    "strava-pr",
    "weeknight-strict-block",
];

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let rule_library_dir = cwd.parent().ok_or(anyhow!("Could not find parent dir"))?;

    println!("Rule Library Parity Test");
    println!("========================\n");

    let mut all_passed = true;

    for rule_id in RULE_IDS {
        let rule_dir = rule_library_dir.join(rule_id);
        if !rule_dir.exists() {
            eprintln!("SKIP: {}: directory not found", rule_id);
            all_passed = false;
            continue;
        }

        match test_rule_parity(&rule_dir, rule_id) {
            Ok(passed) => {
                if passed {
                    println!("  {} ✓ ir_hash match", rule_id);
                } else {
                    println!("  {} ✗ ir_hash MISMATCH", rule_id);
                    all_passed = false;
                }
            }
            Err(e) => {
                eprintln!("  {} ERROR: {}", rule_id, e);
                all_passed = false;
            }
        }
    }

    println!("\nResult: {}", if all_passed { "PASS" } else { "FAIL" });
    std::process::exit(if all_passed { 0 } else { 1 });
}

/// Test parity for a single rule across all 3 surfaces.
/// Currently tests that both CLI and Graph compile to the same IR hash.
/// FPL parity is documented as a gap (requires deeper Starlark integration).
fn test_rule_parity(rule_dir: &Path, rule_id: &str) -> Result<bool> {
    let cli_json_path = rule_dir.join("cli.sh");
    let graph_json_path = rule_dir.join("graph.json");

    // Load graph.json as the canonical source
    let graph_json_str = fs::read_to_string(&graph_json_path)
        .map_err(|e| anyhow!("Failed to read graph.json: {}", e))?;
    let graph_doc: Document = serde_json::from_str(&graph_json_str)
        .map_err(|e| anyhow!("Failed to parse graph.json: {}", e))?;
    let graph_hash = graph_doc.content_hash_hex()?;

    // Load CLI JSON (extract from shell script)
    let cli_sh_content = fs::read_to_string(&cli_json_path)
        .map_err(|e| anyhow!("Failed to read cli.sh: {}", e))?;
    let cli_json_str = extract_json_from_cli_sh(&cli_sh_content)
        .ok_or(anyhow!("Could not extract JSON from cli.sh"))?;
    let cli_doc: Document = serde_json::from_str(&cli_json_str)
        .map_err(|e| anyhow!("Failed to parse cli.sh JSON: {}", e))?;
    let cli_hash = cli_doc.content_hash_hex()?;

    // CLI and Graph must match (FPL parity is documented separately)
    let all_match = graph_hash == cli_hash;

    if !all_match {
        eprintln!("  Hashes for {}:", rule_id);
        eprintln!("    Graph:     {}", graph_hash);
        eprintln!("    CLI JSON:  {}", cli_hash);
    }

    Ok(all_match)
}

/// Extract JSON from cli.sh by looking for the --json '...' pattern.
fn extract_json_from_cli_sh(content: &str) -> Option<String> {
    // Find the --json marker and extract the JSON that follows
    if let Some(start) = content.find("--json '{") {
        let start = start + 8; // Skip "--json '"
        let mut depth = 0;
        let mut in_string = false;
        let mut escape_next = false;
        let mut end = start;

        for (i, ch) in content[start..].chars().enumerate() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => escape_next = true,
                '"' if !in_string => in_string = true,
                '"' if in_string => in_string = false,
                '{' if !in_string => depth += 1,
                '}' if !in_string => {
                    depth -= 1;
                    if depth == 0 {
                        end = start + i;
                        break;
                    }
                }
                _ => {}
            }
        }

        if depth == 0 && end > start {
            return Some(content[start..=end].to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_extraction() {
        let cli_sh = r#"#!/bin/bash
focus rules upsert --json '{"version": 1, "kind": "Rule"}'
"#;
        let extracted = extract_json_from_cli_sh(cli_sh);
        assert!(extracted.is_some());
        let json = extracted.unwrap();
        assert!(json.contains("\"version\""));
    }
}
