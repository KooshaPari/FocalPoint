# pheno-errors AGENTS.md

This is the **agent constitution** for the `pheno-errors` crate. Read this before editing.

## Build & Test

```bash
cargo test --manifest-path pheno-errors/Cargo.toml   # 3 unit + 2 integration = 5/5
cargo build --manifest-path pheno-errors/Cargo.toml --release
cargo doc --manifest-path pheno-errors/Cargo.toml --no-deps
```

## Code Style

- **Edition:** 2021
- **Deps:** none. The crate is intentionally dependency-free.
- **Naming:** `PascalCase` types, `snake_case` fns, `SCREAMING_SNAKE` consts
- **Docs:** Public API has `///` doc comment with example
- **Errors:** Return `pheno_errors::Result<T>` (= `Result<T, pheno_errors::Error>`)
- **No `unwrap()`** in library code; tests are fine
- **No `unsafe`** without a `// SAFETY:` block

## PR Conventions

- Title: `feat(errors):` / `fix(errors):` / `docs(errors):`
- Body: 1-3 bullets, link to task ID (e.g. `V20-errors.2`)
- Rebase onto `main`; no merge commits
- Run `cargo test --manifest-path pheno-errors/Cargo.toml` before pushing

## Do Not Touch

- `Cargo.toml` `version` field — bumped by release-drafter only
- `LICENSE-MIT` — fixed text
- The `Error(pub String)` newtype shape — single-field struct is the public contract

## Reference

- **Public API:** see `llms.txt` (one-page reference)
- **Worklog schema:** `WORKLOG.md` (V2 10-col)
- **Task DAG:** `V20_STRATEGIC_PLAN_2026_06_12.md` §96.1
- **Crates.io:** `pheno-errors` (when published)

## Layer

- L3 Consolidate: minimal `Error(pub String)` newtype + `Result<T>` alias
- L5 Consume: used by L5 #81-85 across the pheno-* fleet
- Design: zero dependencies, single newtype, blanket `From<&str>` + `From<String>`
