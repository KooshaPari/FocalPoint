# Quality Gate

The FocalPoint quality gate is a single authoritative pipeline that runs locally and in CI, ensuring all code meets formatting, linting, testing, documentation, and compliance standards.

## Overview

The quality gate is a Rust binary (`tooling/quality-gate/`) that runs checks in strict order and aborts on first failure. This ensures fast feedback and prevents accumulation of quality debt.

## Checks

| Check | Purpose | Skip Flag |
|-------|---------|-----------|
| `cargo fmt --check` | Rust code formatting | Never |
| `cargo clippy --workspace -- -D warnings` | Static analysis + lints | Never |
| `cargo test --workspace --no-fail-fast` | Unit and integration tests | `--quick` |
| `cargo doc --workspace --no-deps` | Documentation build (0 warnings) | `--quick` |
| `cargo deny check --hide-inclusion-graph` | Supply chain policy (if deny.toml exists) | Never |
| `fr-coverage` | Functional requirement traceability | Never |
| `bun run build` in `apps/builder/` | TypeScript/JavaScript builder (if bun.lockb exists) | `--quick` |
| `doc-link-check` | Documentation link validation | `--quick` |

## Running Locally

### Full Quality Gate

Run all checks including tests and documentation:

```bash
task quality
```

Or directly:

```bash
cargo build --manifest-path tooling/quality-gate/Cargo.toml --release
./tooling/quality-gate/target/release/quality-gate
```

### Quick Quality Gate

Run only format and linting checks (fast feedback):

```bash
task quality:quick
```

Or directly:

```bash
./tooling/quality-gate/target/release/quality-gate --quick
```

### JSON Output

Get structured results for tooling integration:

```bash
./tooling/quality-gate/target/release/quality-gate --format=json
```

## Pre-Push Hook

The `.githooks/pre-push` hook runs the quick quality gate before allowing a push. This prevents non-compliant code from reaching the remote.

Install hooks (if using lefthook):

```bash
task hooks-install
```

Or manually:

```bash
git config core.hooksPath .githooks
```

## CI Integration

The `quality-gate.yml` workflow runs on all PRs and pushes to `main`. It runs the full quality gate (including tests and docs) and blocks merge if any check fails.

## Fixing Failures

### Format Issues

```bash
cargo fmt --all
```

### Linting Issues

Review clippy warnings:

```bash
cargo clippy --workspace -- -D warnings
```

Most warnings include suggestions. Apply them or adjust code to comply.

### Test Failures

Run full test suite locally:

```bash
cargo test --workspace --no-fail-fast
```

Add `RUST_BACKTRACE=1` for detailed failures:

```bash
RUST_BACKTRACE=1 cargo test --workspace --no-fail-fast
```

### Documentation Build

Build and check docs:

```bash
cargo doc --workspace --no-deps 2>&1 | grep -i warning
```

Fix broken links, missing docs, or invalid references in doc comments.

### Supply Chain Issues

Review deny violations:

```bash
cargo deny check
```

See `deny.toml` for policy rules. Common issues:

- **Duplicate/duplicate crates:** consolidate dependencies
- **Banned licenses:** replace with compliant dependency
- **Advisories:** update vulnerable crates

### FR Coverage

Ensure all tests reference Functional Requirements:

```bash
cargo build --manifest-path tooling/fr-coverage/Cargo.toml --release
./tooling/fr-coverage/target/release/fr-coverage
```

Add `// Traces to: FR-<PROJECT>-<ID>` to test functions.

### Builder Build

If `apps/builder/bun.lockb` exists:

```bash
cd apps/builder
bun install
bun run build
```

### Documentation Links

Check broken links in docs-site:

```bash
cargo build --manifest-path tooling/doc-link-check/Cargo.toml --release
./tooling/doc-link-check/target/release/doc-link-check
```

Fix or remove broken `[text](url)` references in `.md` files.

## Re-enabling Skipped Checks

If a check was intentionally disabled, re-enable it by:

1. Removing the skip flag from the quality-gate binary invocation
2. Fixing all failures locally
3. Running the full gate to verify

Example: To re-enable docs build in quick mode:

1. Edit `Taskfile.yml` to add `cargo doc` to the `quality:quick` target
2. Or edit `tooling/quality-gate/src/main.rs` to remove the `if !config.quick` condition

## Architecture

The quality-gate binary is standalone with its own `Cargo.toml` (empty `[workspace]`). It:

1. Parses command-line flags (`--quick`, `--format=json`)
2. Runs each check in sequence via `sh -c`
3. Captures exit codes, stdout, and stderr
4. Aborts on first failure with error context
5. Prints human-readable or JSON output

This design keeps the gate independent of the main workspace and allows easy addition of new checks.
