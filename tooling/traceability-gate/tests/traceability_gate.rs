use std::{fs, path::Path, process::Command};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_traceability-gate")
}

#[test]
fn valid_fixture_passes_with_pending_capture_warning() {
    let repo = fixture("valid");
    let output = Command::new(bin())
        .arg("--repo")
        .arg(&repo)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("traceability gate passed"));
    assert!(String::from_utf8_lossy(&output.stderr).contains("capture pending"));
}

#[test]
fn missing_test_ref_fails() {
    let repo = fixture("invalid_missing_test");
    let output = Command::new(bin())
        .arg("--repo")
        .arg(&repo)
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing test path"));
}

#[test]
fn no_skip_allowed_fails_pending_capture() {
    let repo = fixture("valid");
    let output = Command::new(bin())
        .arg("--repo")
        .arg(&repo)
        .arg("--no-skip-allowed")
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("NEEDS_CAPTURE is forbidden"));
}

fn fixture(name: &str) -> String {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("test-fixtures")
        .join(name);

    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }

    fs::create_dir_all(root.join("docs/traceability")).unwrap();
    fs::create_dir_all(root.join("docs/journeys/manifests")).unwrap();
    fs::create_dir_all(root.join("docs/operations")).unwrap();
    fs::create_dir_all(root.join("crates/focus-rules/src")).unwrap();

    fs::write(root.join("SPEC.md"), "# Spec\n").unwrap();
    fs::write(
        root.join("docs/operations/journey-traceability.md"),
        "# Docs\n",
    )
    .unwrap();
    fs::write(
        root.join("crates/focus-rules/src/lib.rs"),
        "pub fn ok() {}\n",
    )
    .unwrap();

    if name == "valid" {
        fs::write(
            root.join("crates/focus-rules/src/test.rs"),
            "#[test] fn ok() {}\n",
        )
        .unwrap();
    }

    fs::write(
        root.join("docs/journeys/manifests/core.json"),
        r#"{
          "steps": [{
            "id":"step-1",
            "capture_status":"NEEDS_CAPTURE",
            "blind_eval":"skip",
            "media_stub_reason":"capture pending"
          }]
        }"#,
    )
    .unwrap();

    let test_refs = if name == "valid" {
        "[\"crates/focus-rules/src/test.rs\"]"
    } else {
        "[\"missing.rs\"]"
    };

    fs::write(
        root.join("docs/traceability/fr-nfr-traceability.json"),
        format!(
            r#"{{
              "requirements": [{{
                "id":"FR-RULES-001",
                "kind":"FR",
                "spec_refs":["SPEC.md"],
                "code_refs":["crates/focus-rules/src/lib.rs"],
                "test_refs":{},
                "doc_refs":["docs/operations/journey-traceability.md"],
                "journey_refs":["docs/journeys/manifests/core.json"],
                "gates":["cargo test -p focus-rules"]
              }}]
            }}"#,
            test_refs
        ),
    )
    .unwrap();

    root.to_string_lossy().to_string()
}
