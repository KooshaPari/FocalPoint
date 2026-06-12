# AGENTS.md — pheno-tokio-base (L3 #54 phenotype-track)

## Build & test

```bash
cargo test -p pheno-tokio-base   # 2/2 unit tests
cargo test --manifest-path pheno-tokio-base/Cargo.toml
```

## Code style

- Rust 1.75+, edition 2021, 100% safe (no `unsafe`)
- All public types are `Send + Sync + 'static` (they own the runtime)
- Use `tokio::signal` (not `tokio_signal`); wrap in pheno-tokio-base for ergonomics
- Public API uses `Result<T, pheno_tokio_base::Error>` for fallible setup
- Run `cargo clippy --all-targets -- -D warnings` before committing

## PR conventions

- Branch prefix: `chore/l3-54-pheno-tower-2026-06-11` (shared with pheno-tower)
- Reference FLEET_DAG_v3.md §201 task-id in commit messages
- Add a worklog entry to `worklogs/l3-54-pheno-tower-stack-2026-06-11.json`
- Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`)

## Do not touch

- `pheno-tokio-base/src/main_loop.rs` — used by the L4 builder agent
- `pheno-tokio-base/src/signal.rs` — signal handling is stable per L3 #54
- `Cargo.toml` deps (tokio, async-trait, thiserror) — owned by L3 #54 release
- `LICENSE-MIT`

## Reference

- FLEET_DAG_v3.md §201 (task-201-01)
- V3_EXECUTION_LOG_2026_06_10.md (L3 #54)
- pheno-agents-md, pheno-llms-txt, pheno-worklog-schema (AI-DD crutches)

## Architecture (one-liner)

- **pheno-tokio-base** — opinionated Tokio runtime: SIGTERM/SIGINT handlers,
  graceful shutdown via CancellationToken, structured panic logging, runtime
  builder with worker-thread count + name prefix.

## Stable API (L3 #54 contract)

```rust
pub fn runtime() -> tokio::runtime::Runtime;
pub async fn shutdown_signal() -> std::io::Result<()>; // returns on SIGTERM/SIGINT
pub struct CancellationToken { ... }
impl CancellationToken {
    pub fn new() -> Self;
    pub fn cancel(&self);
    pub fn is_cancelled(&self) -> bool;
    pub async fn cancelled(&self);
}
```

Any breaking change requires a SemVer bump and a new DAG §201 task.
