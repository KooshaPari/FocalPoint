# AGENTS.md — pheno-tower (L3 #54 phenotype-track)

## Build & test

```bash
# workspace setup
cargo build                # builds the full repo workspace
cargo test -p pheno-tower  # 3/3 unit tests, 1/1 doctest
cargo test -p pheno-tokio-base
cargo test -p pheno-axum-stack

# standalone
cargo test --manifest-path pheno-tower/Cargo.toml
```

## Code style

- Rust 1.75+, edition 2021
- 100% safe code (no `unsafe`); if you must, justify in a comment
- Public API uses `Result<T, pheno_tower::Error>` for fallible operations
- All public types are `Send + Sync` unless documented otherwise
- Run `cargo clippy --all-targets -- -D warnings` before committing

## PR conventions

- Branch prefix: `chore/l3-54-pheno-tower-2026-06-11` or similar
- One crate per PR (pheno-tower, pheno-tokio-base, pheno-axum-stack are independent)
- Reference the FLEET_DAG_v3.md §201 task-id in commit messages
- Use Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`)
- Add a worklog entry to `worklogs/l3-54-pheno-tower-stack-2026-06-11.json`

## Do not touch

- `pheno-tower/src/runtime.rs` — frozen trait surface for the L3 #54 release
- `pheno-axum-stack/src/lib.rs` `Service` impl — see L3 #54 §201 for stable contract
- `pheno-tokio-base/src/main_loop.rs` — used by the L4 builder agent
- `Cargo.lock` at monorepo root — managed by `cargo update --workspace`
- `LICENSE-MIT` — release-year 2026, owner Koosha Pari

## Reference

- FLEET_DAG_v3.md §201 (task-201-01: pheno-tower-stack, AI-DD crutches)
- V3_EXECUTION_LOG_2026_06_10.md (L3 #54)
- pheno-agents-md (canonical AGENTS.md template generator)
- pheno-llms-txt (LLM-friendly reference generator)
- pheno-worklog-schema (V2 10-col WORKLOG.md validator)

## Architecture (one-line each)

- **pheno-tower** — builder for layered async apps: Tower-style `Service` + `Layer` traits with a pheno-flavored API; 3 unit tests cover service composition
- **pheno-tokio-base** — opinionated Tokio runtime: signal handling (SIGTERM/SIGINT), graceful shutdown via CancellationToken, structured panic logging
- **pheno-axum-stack** — Axum + pheno-tower + pheno-tokio-base: pre-wired Router with CORS, request-id, trace context, health check, 404 fallback

## Stable trait surface (L3 #54 contract)

```rust
// pheno-tower
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    fn call(&self, req: Request) -> Self::Future;
}
pub trait Layer<S> {
    type Service;
    fn layer(&self, inner: S) -> Self::Service;
}

// pheno-axum-stack
pub use pheno_tower::Service;
pub use pheno_tokio_base::CancellationToken;
```

Any breaking change to these traits requires a SemVer bump and a new DAG §201 task.
