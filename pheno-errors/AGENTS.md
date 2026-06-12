# pheno-errors AGENTS.md

This is the **agent constitution** for the `pheno-errors` crate. Read this before editing.

## Build & Test

```bash
cargo test --manifest-path pheno-errors/Cargo.toml   # unit + integration tests
cargo build --manifest-path pheno-errors/Cargo.toml --release
cargo doc --manifest-path pheno-errors/Cargo.toml --no-deps
```

## Code Style

- **Edition:** 2021
- **Lints:** `#![deny(missing_docs)]` (enforced in lib root)
- **Naming:** `PascalCase` types, `snake_case` fns, `SCREAMING_SNAKE` consts
- **Docs:** Public API has `///` doc comment with example
- **Errors:** Use `thiserror::Error` for error enums; return `AppResult<T>`
- **No `unwrap()`** in library code; tests are fine
- **No `unsafe`** without a `// SAFETY:` block

## PR Conventions

- Title: `feat(errors):` / `fix(errors):` / `docs(errors):`
- Body: 1-3 bullets, link to task ID (e.g. `V20-errors.1`)
- Rebase onto `main`; no merge commits
- Run `cargo test --manifest-path pheno-errors/Cargo.toml` before pushing

## Do Not Touch

- `src/lib.rs` top-level `deny(missing_docs)` — too noisy to maintain
- `Cargo.toml` `version` field — bumped by release-drafter only
- `LICENSE-MIT` — fixed text
- The 5-variant `AppError` enum — adding a 6th variant is a breaking change

## Reference

- **Public API:** see `llms.txt` (one-page reference)
- **Worklog schema:** `WORKLOG.md` (V2 10-col)
- **Task DAG:** `V20_STRATEGIC_PLAN_2026_06_12.md` §96.1
- **Crates.io:** `pheno-errors` (when published)

## Layer

- L3 Consolidate: canonical `AppError` type adopted from `chore/l3-47-pheno-tracing-2026-06-11`
- L5 Consume: used by L5 #81-85 across the pheno-* fleet
- Design: 5-variant enum (`Domain`, `NotFound`, `Conflict`, `Validation`, `Storage`) on `thiserror` + `anyhow` interop
