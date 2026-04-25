// Motion pipeline tests: verify Rive + Lottie generation and validation
// Traces to: FR-MOTION-001 (animation generation), FR-MOTION-002 (size gates)

use std::fs;
use std::path::Path;
use serde_json::json;

/// FR-MOTION-001: Verify Rive state machine parses and has expected structure
#[test]
fn test_rive_state_machine_structure() {
    let rive_path = Path::new("rive/coachy-state-machine.json");

    // Generate if not exists
    if !rive_path.exists() {
        eprintln!("Rive state machine not generated; run build.sh first");
        return;
    }

    let rive_json = fs::read_to_string(rive_path)
        .expect("Failed to read Rive state machine JSON");
    let root: serde_json::Value = serde_json::from_str(&rive_json)
        .expect("Rive JSON is invalid");

    // Verify structure
    assert!(root.get("version").is_some(), "Missing version field");
    assert!(root.get("artboard").is_some(), "Missing artboard field");
    assert!(root.get("states").is_some(), "Missing states array");
    assert!(root.get("transitions").is_some(), "Missing transitions array");

    // Verify state count: 5 expressions × 4 intensities = 20 states
    let states = root.get("states").unwrap().as_array().expect("states not array");
    assert_eq!(states.len(), 20, "Expected 20 states (5×4), got {}", states.len());

    // Verify each state has expression + intensity properties
    for state in states {
        assert!(state.get("id").is_some(), "State missing id");
        assert!(state.get("name").is_some(), "State missing name");
        assert!(state.get("properties").is_some(), "State missing properties");

        let props = state.get("properties").unwrap().as_object().unwrap();
        assert!(props.contains_key("expression"), "State missing expression property");
        assert!(props.contains_key("intensity"), "State missing intensity property");
    }

    println!("✅ Rive state machine valid: {} states", states.len());
}

/// FR-MOTION-002: Verify all 12 Lottie animations exist and parse
#[test]
fn test_lottie_animations_exist_and_parse() {
    let expected_animations = vec![
        "rule-create",
        "rule-fire",
        "intervention-warn",
        "emergency-exit",
        "achievement-unlock",
        "mascot-blink",
        "mascot-yawn",
        "focus-start",
        "focus-end",
        "sync-pulse",
        "error-shake",
        "success-checkmark",
    ];

    let lottie_dir = Path::new("lottie");

    if !lottie_dir.exists() {
        eprintln!("Lottie directory not found; run build.sh first");
        return;
    }

    for name in expected_animations {
        let path = lottie_dir.join(format!("{}.json", name));
        assert!(path.exists(), "Animation {} not generated", name);

        let json_str = fs::read_to_string(&path)
            .expect(&format!("Failed to read {}", name));
        let anim: serde_json::Value = serde_json::from_str(&json_str)
            .expect(&format!("{} is not valid JSON", name));

        // Verify Lottie structure
        assert!(anim.get("v").is_some(), "{}: missing version field", name);
        assert!(anim.get("fr").is_some(), "{}: missing frame rate", name);
        assert!(anim.get("w").is_some(), "{}: missing width", name);
        assert!(anim.get("h").is_some(), "{}: missing height", name);
        assert!(anim.get("layers").is_some(), "{}: missing layers", name);

        // Verify dimensions (256×320)
        assert_eq!(anim.get("w").unwrap().as_u64(), Some(256), "{}: wrong width", name);
        assert_eq!(anim.get("h").unwrap().as_u64(), Some(320), "{}: wrong height", name);

        println!("  ✅ {} valid", name);
    }

    println!("✅ All 12 Lottie animations verified");
}

/// FR-MOTION-002: Size gate verification (Lottie ≤30KB, Rive ≤50KB)
#[test]
fn test_animation_size_gates() {
    const LOTTIE_MAX: u64 = 30 * 1024;  // 30KB
    const RIVE_MAX: u64 = 50 * 1024;    // 50KB

    let mut lottie_total = 0u64;
    let mut rive_total = 0u64;
    let mut violated = false;

    // Check Lottie animations
    let lottie_dir = Path::new("lottie");
    if lottie_dir.exists() {
        for entry in fs::read_dir(lottie_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().map_or(false, |ext| ext == "json") {
                let size = fs::metadata(&path).unwrap().len();
                lottie_total += size;
                if size > LOTTIE_MAX {
                    eprintln!("❌ {} exceeds 30KB limit: {} bytes", path.display(), size);
                    violated = true;
                }
            }
        }
    }

    // Check Rive animations
    let rive_dir = Path::new("rive");
    if rive_dir.exists() {
        for entry in fs::read_dir(rive_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().map_or(false, |ext| ext == "json") {
                let size = fs::metadata(&path).unwrap().len();
                rive_total += size;
                if size > RIVE_MAX {
                    eprintln!("❌ {} exceeds 50KB limit: {} bytes", path.display(), size);
                    violated = true;
                }
            }
        }
    }

    assert!(!violated, "Size gate check failed");
    println!("✅ Size gates passed: Lottie {}KB, Rive {}KB", lottie_total / 1024, rive_total / 1024);
}

/// FR-MOTION-001: Verify frame counts match animation intent
#[test]
fn test_lottie_frame_counts() {
    let animations = vec![
        ("rule-create", 30),
        ("rule-fire", 15),
        ("intervention-warn", 40),
        ("emergency-exit", 50),
        ("achievement-unlock", 45),
        ("mascot-blink", 5),
        ("mascot-yawn", 30),
        ("focus-start", 20),
        ("focus-end", 20),
        ("sync-pulse", 20),
        ("error-shake", 12),
        ("success-checkmark", 25),
    ];

    for (name, expected_frames) in animations {
        let path = format!("lottie/{}.json", name);
        if !Path::new(&path).exists() {
            eprintln!("Skipping frame count test for {} (not generated)", name);
            continue;
        }

        let json_str = fs::read_to_string(&path).unwrap();
        let anim: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        let out_point = anim.get("op").unwrap().as_u64().unwrap() as u32;

        assert_eq!(out_point, expected_frames,
            "{}: expected {} frames, got {}", name, expected_frames, out_point);
    }

    println!("✅ All frame counts verified");
}

/// Verify animation metadata (names, descriptions)
#[test]
fn test_animation_metadata() {
    let lottie_dir = Path::new("lottie");
    if !lottie_dir.exists() {
        eprintln!("Lottie directory not found; skipping metadata test");
        return;
    }

    for entry in fs::read_dir(lottie_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |ext| ext == "json") {
            let json_str = fs::read_to_string(&path).unwrap();
            let anim: serde_json::Value = serde_json::from_str(&json_str).unwrap();

            // Verify name field is set
            assert!(anim.get("nm").is_some(), "Animation missing name field");
            let nm = anim.get("nm").unwrap().as_str().unwrap();
            assert!(!nm.is_empty(), "Animation name is empty");
        }
    }

    println!("✅ All animation metadata valid");
}
