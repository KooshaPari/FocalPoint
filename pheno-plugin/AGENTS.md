# pheno-plugin AGENTS

This is the **agent constitution** for the `pheno-plugin` crate. Read this before editing.

## Build & Test

```bash
cargo test                    # unit + integration tests
cargo build --release         # release build
cargo doc --no-deps           # docs (publish target)
```

## Code Style

- **Edition:** 2021
- **Lints:** `#![deny(missing_docs, unsafe_op_in_unsafe_fn)]` (enforced in lib root)
- **Naming:** `PascalCase` types, `snake_case` fns, `SCREAMING_SNAKE` consts
- **Docs:** Public API has `///` doc comment with example
- **Errors:** Use `thiserror::Error` for error enums; return `Result<T, PluginError>`
- **No `unwrap()`** in library code; tests are fine
- **No `unsafe`** without a `// SAFETY:` block

## PR Conventions

- Title: `feat(plugin):` / `fix(plugin):` / `docs(plugin):`
- Body: 1-3 bullets, link to task ID (e.g. `V18-plugin.2`)
- Rebase onto `main`; no merge commits
- Run `cargo test && cargo clippy --all-targets -- -D warnings` before pushing

## Do Not Touch

- `src/lib.rs` top-level `deny(missing_docs)` — too noisy to maintain
- `Cargo.toml` `version` field — bumped by release-drafter only
- `LICENSE-MIT` — fixed text
- `src/registry.rs` — stable contract, breaking changes need major version

## Reference

- **Public API:** see `llms.txt` (one-page reference)
- **Worklog schema:** `WORKLOG.md` (V2 10-col)
- **Task DAG:** `FLEET_DAG_v3.md` §92 (V18 EXT)
- **Crates.io:** `pheno-plugin` (when published)

## Layer

- L1 Stabilize: trait + impl surface (`Plugin` + `PluginRegistry`)
- L4 Hexagonal: `Plugin<S>` wraps the `tower::Service` contract from `pheno-tower`
- L1: registrar uses `inventory::collect!` for global registration
