# Phenotype-org standard justfile — FocalPoint

# Usage: `just <recipe>` — https://github.com/casey/just

# ---------------------------------------------------------------------------
# Default
# ---------------------------------------------------------------------------

# Show available recipes
[private]
default:
    @just --list

# ---------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------

# Build the entire Rust workspace
build:
    cargo build --workspace

# Build the iOS app (requires Xcode 15.2+, iOS 16+ simulator)
build-ios:
    cd apps/ios/FocalPoint && xcodebuild -project FocalPoint.xcodeproj -scheme FocalPoint -destination 'platform=iOS Simulator,name=iPhone 16' build

# Build the CLI binary
build-cli:
    cargo build -p focus-cli --release

# ---------------------------------------------------------------------------
# Test
# ---------------------------------------------------------------------------

# Run all Rust workspace tests
test:
    cargo test --workspace

# Run tests for a specific crate
test-crate crate:
    cargo test -p {{crate}}

# Run iOS unit tests (simulator)
test-ios:
    cd apps/ios/FocalPoint && xcodebuild -project FocalPoint.xcodeproj -scheme FocalPoint -destination 'platform=iOS Simulator,name=iPhone 16' test

# ---------------------------------------------------------------------------
# Demo / Exploration
# ---------------------------------------------------------------------------

# Run the full CLI demo walkthrough (seed, tasks, rules, wallet, audit)
demo:
    cargo run -p focus-cli -- demo seed --db=/tmp/focus-demo.db
    cargo run -p focus-cli -- tasks list --db=/tmp/focus-demo.db --json
    cargo run -p focus-cli -- rules list --db=/tmp/focus-demo.db
    cargo run -p focus-cli -- wallet balance --db=/tmp/focus-demo.db
    cargo run -p focus-cli -- audit verify --db=/tmp/focus-demo.db

# ---------------------------------------------------------------------------
# Quality
# ---------------------------------------------------------------------------

# Run clippy and format check
lint:
    cargo clippy --workspace -- -D warnings
    cargo fmt --check

# Auto-format code
fmt:
    cargo fmt

# Run cargo-deny and cargo-audit
audit:
    cargo deny check
    cargo audit

# Check for unused dependencies
unused:
    cargo machete

# Run the full quality gate (lint + test + audit + unused)
ci: lint test audit unused

# ---------------------------------------------------------------------------
# Documentation
# ---------------------------------------------------------------------------

# Build Rust docs
docs:
    cargo doc --no-deps --workspace

# Open local docs in browser
docs-open:
    cargo doc --no-deps --workspace --open

# ---------------------------------------------------------------------------
# FR Coverage
# ---------------------------------------------------------------------------

# Generate the functional-requirements coverage matrix
fr-coverage:
    cargo run -p fr-coverage

# Strict CI check (fails if missing FR coverage)
fr-coverage-strict:
    cargo run -p fr-coverage -- --strict

# ---------------------------------------------------------------------------
# Release
# ---------------------------------------------------------------------------

# Generate release notes
release-notes version:
    cargo run -p release-notes -- --version {{version}}

# Generate SBOM
sbom:
    cargo run -p sbom-gen

# ---------------------------------------------------------------------------
# Clean
# ---------------------------------------------------------------------------

clean:
    cargo clean
    rm -rf apps/ios/FocalPoint/build*

# ---------------------------------------------------------------------------
# Grading (wraps the legacy grade.sh fleet-wide grading engine)
# ---------------------------------------------------------------------------

# Run the full grading gate (build, test, lint, fmt, clippy, deny, coverage)
[private]
grade: ci

# Quick grading mode (skips heavy checks: coverage, fuzz, mutation, perf)
grade-fast:
    ./grade.sh --fast

# Grading with machine-readable JSON output
grade-json:
    ./grade.sh --json

# Grading with HTML report
grade-html:
    ./grade.sh --html
