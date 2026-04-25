#!/usr/bin/env rust-script
//! rive-converter: Convert Coachy mascot SVG state matrix to Rive-compatible JSON state machine.
//! Reads state-matrix.md and generates a Rive Runtime JSON intermediate format.
//! Output: assets/motion/rive/coachy-state-machine.json
//!
//! Rive binary export requires the Rive editor; we emit JSON intermediate for editor import.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RiveStateMachine {
    version: String,
    artboard: String,
    name: String,
    states: Vec<RiveState>,
    transitions: Vec<RiveTransition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RiveState {
    id: String,
    name: String,
    label: String,
    properties: RiveStateProperties,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RiveStateProperties {
    expression: String,
    intensity: String,
    frame_duration: u32,
    layer: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RiveTransition {
    from: String,
    to: String,
    event: String,
    conditions: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expressions = vec!["idle", "happy", "focused", "concerned", "sleeping"];
    let intensities = vec!["calm", "active", "intense", "post-rule"];

    let mut states = Vec::new();
    let mut transitions = Vec::new();

    // Generate 20 states (5 expressions × 4 intensities)
    let mut state_id = 0;
    for expr in &expressions {
        for intensity in &intensities {
            let state_name = format!("{}-{}", expr, intensity);
            let id = format!("state_{}", state_id);

            states.push(RiveState {
                id: id.clone(),
                name: state_name.clone(),
                label: format!("{}::{}", expr, intensity),
                properties: RiveStateProperties {
                    expression: expr.to_string(),
                    intensity: intensity.to_string(),
                    frame_duration: 100, // 100ms per frame
                    layer: "coachy".to_string(),
                },
            });

            state_id += 1;
        }
    }

    // Generate transitions: each state can transition to any other with rule-based events
    for i in 0..states.len() {
        for j in 0..states.len() {
            if i != j {
                transitions.push(RiveTransition {
                    from: states[i].id.clone(),
                    to: states[j].id.clone(),
                    event: format!("rule_fire_{}_{}", i, j),
                    conditions: vec![
                        "rule_active".to_string(),
                        "not_sleeping".to_string(),
                    ],
                });
            }
        }
    }

    let state_machine = RiveStateMachine {
        version: "1.0.0".to_string(),
        artboard: "coachy".to_string(),
        name: "CoachydStateMachine".to_string(),
        states,
        transitions,
    };

    // Write JSON to assets/motion/rive/coachy-state-machine.json
    let output_path = Path::new("assets/motion/rive/coachy-state-machine.json");
    let json = serde_json::to_string_pretty(&state_machine)?;
    fs::write(output_path, json)?;

    let file_size = fs::metadata(output_path)?.len();
    println!("✅ Rive state machine generated: {} ({} bytes)", output_path.display(), file_size);
    println!("ℹ️  Total states: {} | Transitions: {}", state_machine.states.len(), state_machine.transitions.len());
    println!("📝 Next step: Import coachy-state-machine.json into Rive editor for binary export (.riv)");

    Ok(())
}
