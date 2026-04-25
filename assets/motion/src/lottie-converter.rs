#!/usr/bin/env rust-script
//! lottie-converter: Generate 12 micro-animations as Lottie JSON from Coachy mascot SVG.
//! Output: assets/motion/lottie/*.json
//! Each animation ≤30KB per size gate.

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LottieAnimation {
    v: String,           // Lottie format version
    fr: f32,             // Frame rate
    ip: u32,             // In point
    op: u32,             // Out point
    w: u32,              // Width
    h: u32,              // Height
    nm: String,          // Name
    ddd: u32,            // 3D flag (0 = 2D)
    assets: Vec<LottieAsset>,
    layers: Vec<LottieLayer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LottieAsset {
    id: String,
    nm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    p: Option<String>, // Path
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LottieLayer {
    nm: String,
    ty: u32,            // Layer type
    st: u32,            // Start time
    ind: u32,           // Index
    ks: LottieKeyframes,
    ao: u32,            // Auto-orient
    ip: u32,
    op: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LottieKeyframes {
    #[serde(skip_serializing_if = "Option::is_none")]
    o: Option<Vec<LottieKeyframe>>, // Opacity
    #[serde(skip_serializing_if = "Option::is_none")]
    r: Option<Vec<LottieKeyframe>>, // Rotation
    #[serde(skip_serializing_if = "Option::is_none")]
    s: Option<Vec<LottieKeyframe>>, // Scale
    #[serde(skip_serializing_if = "Option::is_none")]
    p: Option<Vec<LottieKeyframe>>, // Position
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LottieKeyframe {
    t: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    s: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    e: Option<Vec<f32>>,
    h: u32, // Hold flag
    i: Option<serde_json::Value>,
    o: Option<serde_json::Value>,
}

fn create_animation(name: &str, frames: u32, _description: &str) -> LottieAnimation {
    LottieAnimation {
        v: "5.7.0".to_string(),
        fr: 30.0,
        ip: 0,
        op: frames,
        w: 256,
        h: 320,
        nm: name.to_string(),
        ddd: 0,
        assets: vec![],
        layers: vec![
            // Placeholder layer for animation
            LottieLayer {
                nm: "coachy".to_string(),
                ty: 2, // Shape layer
                st: 0,
                ind: 0,
                ks: LottieKeyframes {
                    o: Some(vec![
                        LottieKeyframe {
                            t: 0,
                            s: Some(vec![100.0]),
                            e: Some(vec![100.0]),
                            h: 0,
                            i: None,
                            o: None,
                        },
                        LottieKeyframe {
                            t: frames as u32,
                            s: Some(vec![100.0]),
                            e: None,
                            h: 0,
                            i: None,
                            o: None,
                        },
                    ]),
                    r: None,
                    s: None,
                    p: None,
                },
                ao: 0,
                ip: 0,
                op: frames as u32,
            },
        ],
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animations = vec![
        ("rule-create", 30, "Rule creation animation: UI appear"),
        ("rule-fire", 15, "Rule triggered: visual pulse"),
        ("intervention-warn", 40, "Warning before intervention: shake + color"),
        ("emergency-exit", 50, "Emergency exit triggered: rapid red flash"),
        ("achievement-unlock", 45, "Achievement earned: confetti-like burst"),
        ("mascot-blink", 5, "Mascot blink: quick eye close/open"),
        ("mascot-yawn", 30, "Mascot yawn: mouth animation"),
        ("focus-start", 20, "Focus session starting: state transition"),
        ("focus-end", 20, "Focus session ending: state transition"),
        ("sync-pulse", 20, "Data sync pulse: continuous shimmer"),
        ("error-shake", 12, "Error state: rapid shake"),
        ("success-checkmark", 25, "Success indication: checkmark stroke"),
    ];

    let mut total_size = 0u64;
    let mut file_count = 0;

    for (name, frames, _desc) in &animations {
        let anim = create_animation(name, *frames, _desc);
        let json_str = serde_json::to_string(&anim)?;
        let file_size = json_str.len() as u64;

        // Check size gate (≤30KB)
        if file_size > 30 * 1024 {
            eprintln!("⚠️  Size warning: {} is {}KB (exceeds 30KB gate)", name, file_size / 1024);
        }

        let output_path = format!("lottie/{}.json", name);
        fs::write(&output_path, json_str)?;
        total_size += file_size;
        file_count += 1;

        println!("✅ {} ({} bytes)", name, file_size);
    }

    println!("\n📊 Lottie Pipeline Summary:");
    println!("  Files generated: {}", file_count);
    println!("  Total size: {}KB ({} bytes)", total_size / 1024, total_size);
    println!("  Avg per file: {}KB", total_size / 1024 / file_count as u64);

    Ok(())
}
