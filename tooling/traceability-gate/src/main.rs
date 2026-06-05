use anyhow::{anyhow, Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = ".")]
    repo: PathBuf,
    #[arg(long, default_value = "docs/traceability/fr-nfr-traceability.json")]
    manifest: PathBuf,
    #[arg(long)]
    no_skip_allowed: bool,
}

#[derive(Deserialize)]
struct TraceabilityManifest {
    requirements: Vec<Requirement>,
}

#[derive(Deserialize)]
struct Requirement {
    id: String,
    kind: String,
    spec_refs: Vec<String>,
    code_refs: Vec<String>,
    test_refs: Vec<String>,
    doc_refs: Vec<String>,
    journey_refs: Vec<String>,
    gates: Vec<String>,
}

#[derive(Deserialize)]
struct JourneyManifest {
    steps: Vec<JourneyStep>,
}

#[derive(Deserialize)]
struct JourneyStep {
    id: String,
    capture_status: Option<String>,
    blind_eval: Option<String>,
    media_stub_reason: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let manifest_path = args.repo.join(&args.manifest);
    let manifest: TraceabilityManifest = read_json(&manifest_path)?;

    let mut failures = Vec::new();
    let mut warnings = Vec::new();

    for req in &manifest.requirements {
        validate_paths(&args.repo, &req.id, "spec", &req.spec_refs, &mut failures);
        validate_paths(&args.repo, &req.id, "code", &req.code_refs, &mut failures);
        validate_paths(&args.repo, &req.id, "test", &req.test_refs, &mut failures);
        validate_paths(&args.repo, &req.id, "doc", &req.doc_refs, &mut failures);
        validate_gates(req, &mut failures);

        if (req.kind == "FR" || req.kind == "NFR") && req.journey_refs.is_empty() {
            failures.push(format!("{}: missing journey_refs", req.id));
        }

        for journey_ref in &req.journey_refs {
            let path = args.repo.join(strip_anchor(journey_ref));
            if !path.exists() {
                failures.push(format!(
                    "{}: missing journey manifest {}",
                    req.id, journey_ref
                ));
                continue;
            }
            validate_journey(&path, args.no_skip_allowed, &mut failures, &mut warnings)?;
        }
    }

    for warning in &warnings {
        eprintln!("WARN: {warning}");
    }

    if !failures.is_empty() {
        for failure in failures {
            eprintln!("FAIL: {failure}");
        }
        return Err(anyhow!("traceability gate failed"));
    }

    println!("traceability gate passed");
    Ok(())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parse {}", path.display()))
}

fn strip_anchor(path: &str) -> &str {
    path.split('#').next().unwrap_or(path)
}

fn validate_paths(repo: &Path, id: &str, kind: &str, refs: &[String], failures: &mut Vec<String>) {
    if refs.is_empty() {
        failures.push(format!("{id}: missing {kind}_refs"));
        return;
    }

    for path in refs {
        let full = repo.join(strip_anchor(path));
        if !full.exists() {
            failures.push(format!("{id}: missing {kind} path {path}"));
        }
    }
}

fn validate_gates(req: &Requirement, failures: &mut Vec<String>) {
    if req.gates.is_empty() {
        failures.push(format!("{}: missing gates", req.id));
    }

    for gate in &req.gates {
        if gate.trim().is_empty() {
            failures.push(format!("{}: empty gate command", req.id));
        }
    }
}

fn validate_journey(
    path: &Path,
    no_skip_allowed: bool,
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<()> {
    let journey: JourneyManifest = read_json(path)?;

    for step in journey.steps {
        if step.capture_status.as_deref() == Some("NEEDS_CAPTURE") {
            let honest_skip = step.blind_eval.as_deref() == Some("skip")
                && step
                    .media_stub_reason
                    .as_deref()
                    .is_some_and(|reason| !reason.trim().is_empty());

            if !honest_skip {
                failures.push(format!(
                    "{}: NEEDS_CAPTURE must set blind_eval=skip and media_stub_reason",
                    step.id
                ));
            } else if no_skip_allowed {
                failures.push(format!(
                    "{}: NEEDS_CAPTURE is forbidden with --no-skip-allowed",
                    step.id
                ));
            } else {
                warnings.push(format!("{}: capture pending", step.id));
            }
        }
    }

    Ok(())
}
