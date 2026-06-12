# AGENTS.md — pheno-axum-stack (L3 #54 phenotype-track)

## Build & test

```bash
cargo test -p pheno-axum-stack   # 2/2 unit tests
cargo test --manifest-path pheno-axum-stack/Cargo.toml
```

## Code style

- Rust 1.75+, edition 2021, 100% safe
- Re-export `axum` types when possible; don't re-wrap
- Use `pheno-tower::Service` for service composition (not `tower::Service`)
- Use `pheno-tokio-base::shutdown_signal` for graceful shutdown
- Run `cargo clippy --all-targets -- -D warnings` before committing

## PR conventions

- Branch prefix: `chore/l3-54-pheno-tower-2026-06-11` (shared with pheno-tower)
- Reference FLEET_DAG_v3.md §201 task-id in commit messages
- Add a worklog entry to `worklogs/l3-54-pheno-tower-stack-2026-06-11.json`

## Do not touch

- `pheno-axum-stack/src/lib.rs` `app()` function — public API is stable per L3 #54
- `pheno-axum-stack/src/middleware.rs` — middleware ordering is part of the contract
- `Cargo.toml` deps (axum, tower, pheno-tower, pheno-tokio-base, pheno-tracing)
- `LICENSE-MIT`

## Reference

- FLEET_DAG_v3.md §201 (task-201-01)
- V3_EXECUTION_LOG_2026_06_10.md (L3 #54)
- pheno-agents-md, pheno-llms-txt, pheno-worklog-schema (AI-DD crutches)

## Architecture (one-liner)

- **pheno-axum-stack** — pre-wired Axum Router with CORS, request-id,
  trace-context (via pheno-tracing), health check (`/healthz`),
  404 fallback, JSON 500 error response. Default `app()` returns a
  fully-middleware-wired `Router` you can `.route("/", ...)` on.

## Stable API (L3 #54 contract)

```rust
pub fn app() -> axum::Router;          // the canonical pre-wired router
pub fn health_router() -> axum::Router; // just /healthz + 404
```

Any breaking change requires a SemVer bump and a new DAG §201 task.

## Middleware stack (order matters!)

1. TraceLayer (outermost, captures everything)
2. RequestIdLayer
3. CorsLayer (default permissive; production should override)
4. CatchPanicLayer (converts panics to 500 JSON)
5. (your routes)
