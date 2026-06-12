# pheno-config — AGENTS.md

## Build & Test
- `cargo test` — runs all tests
- `cargo clippy --all-targets -- -D warnings` — lint check
- `cargo build --release` or equivalent — release build

## Code Style
- Rust 1.75+ edition 2021
- Prefer small, focused modules
- All public API must have doc comments
- No unwrap() in library code (use ? or anyhow::Result)

## PR Conventions
- Branch: `chore/<scope>-config-YYYY-MM-DD`
- One concern per PR
- Run tests + linter before push
- Update `WORKLOG.md` row with PR number

## Do Not Touch
- The public API surface (breaking changes go through deprecation cycle)
- `Cargo.toml` or `pyproject.toml` deps without justification

## Reference
- See `llms.txt` for the one-page API reference
- See `WORKLOG.md` for the current task ID
- See `/Users/kooshapari/CodeProjects/Phenotype/repos/FLEET_DAG_v3.md` for the L3 task that produced this crate
