# pheno-ssot-template — AGENTS.md

## Project Overview

`pheno-ssot-template` is the canonical single-source-of-truth (SSOT) starter template for any new project in the `pheno-*` fleet. It satisfies the 4 SSOT invariants out of the box:

1. Every error is a `pheno_errors::AppError`.
2. Every log line is structured (tracing fields).
3. Every config is loaded via `pheno_config::load::<MyConfig>()`.
4. Every schema lives in `pheno-zod-schemas` (TS) or `pheno-pydantic-models` (Py).

## Stack

- Language: Rust
- Edition: 2021
- MSRV: 1.75
- Platform: cross-platform (tokio-based)
- License: MIT OR Apache-2.0

## Key Commands

```bash
# Render the template
cargo check --manifest-path pheno-ssot-template/Cargo.toml

# Run tests
cargo test --manifest-path pheno-ssot-template/Cargo.toml

# Run SSOT invariant checks
./scripts/check-ssot-invariant-1-errors.sh
./scripts/check-ssot-invariant-2-logging.sh
```

## Notes

- This crate is a standalone package, not a member of the root FocalPoint/Phenotype workspace.
- The `Cargo.toml.template` and `src/lib.rs.template` are the canonical sources; `Cargo.toml` and `src/lib.rs` are rendered instantiations for CI/testing.
- Fleet layer: L3
- Fleet task ID: L3-055
