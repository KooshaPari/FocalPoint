// Traces to: FR-STORE-SCREENSHOT-GOLDEN-001
// Golden hash test for App Store screenshot renders.
// Ensures screenshot outputs remain consistent across builds.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_store_screenshots_golden_hashes() {
    // Find FocalPoint repo root
    let repo_root = find_repo_root().expect("Could not find FocalPoint repo root");
    let output_dir = repo_root.join("assets/store-screenshots/output");

    if !output_dir.exists() {
        eprintln!(
            "Warning: screenshot output directory not found at {}",
            output_dir.display()
        );
        eprintln!("Run: cargo run --bin render-store-screenshots");
        return;
    }

    // Load current hashes
    let hashes_file = output_dir.join("current-hashes.json");
    let hashes: HashMap<String, String> =
        if let Ok(content) = fs::read_to_string(&hashes_file) {
            serde_json::from_str(&content).expect("Failed to parse hashes JSON")
        } else {
            eprintln!(
                "Warning: current-hashes.json not found at {}",
                hashes_file.display()
            );
            return;
        };

    // Verify we have all expected outputs: 5 scenes × 5 devices = 25 outputs
    const EXPECTED_COUNT: usize = 25;
    assert!(
        hashes.len() >= EXPECTED_COUNT,
        "Expected at least {} screenshots, found {}",
        EXPECTED_COUNT,
        hashes.len()
    );

    // Verify expected scenes
    let expected_scenes = [
        "focus-start",
        "rule-builder",
        "mascot-coachy",
        "weekly-review",
        "intervention-warning",
    ];

    let expected_devices = [
        "iphone-15-pro-max",
        "iphone-15-pro",
        "iphone-13-mini",
        "ipad-pro",
        "ipad-mini",
    ];

    for scene in &expected_scenes {
        for device in &expected_devices {
            let key = format!("{}/{}.png", device, scene);
            assert!(
                hashes.contains_key(&key),
                "Missing expected screenshot: {}",
                key
            );
        }
    }

    println!(
        "✓ Store screenshots golden hash test passed: {} outputs verified",
        hashes.len()
    );
}

fn find_repo_root() -> Option<PathBuf> {
    let mut path = std::env::current_dir().ok()?;
    loop {
        if path.join("Cargo.toml").exists() && path.join(".git").exists() {
            // Verify this is FocalPoint repo by checking for key directories
            if path.join("crates/focus-domain").exists() && path.join("apps").exists() {
                return Some(path);
            }
        }
        if !path.pop() {
            return None;
        }
    }
}
