# Changelog — pheno-tower

All notable changes to pheno-tower are documented here. Format: [Keep a Changelog](https://keepachangelog.com/).
pheno-tower adheres to [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-06-11

### Added
- `TowerService` blanket impl for any `Fn(Request) -> Future<Output=Result<Response, Error>>` + Clone + Send + Sync + 'static
- `from_fn` constructor (re-exported as `service`)
- `from_fn_with_state` for stateful services
- 3 unit tests (response_ok, state_propagation, request_metadata) — all green
- L1 Stabilize: clean re-export of `tower::Service` trait; 1st pheno-* service-layer lib
- L16 AX: AGENTS.md, llms.txt, WORKLOG.md (this schema, V2 10-col)

### Known limitations
- No `poll_ready` integration (purely `call`-based; ok for the dispatch-mcp use case)
- No `tokio` runtime dep (uses `futures` only)

### Refs
- FLEET_100TASK_DAG_V4.md §83.6 (V15 - 74 tests target)
- V3_EXECUTION_LOG_2026_06_10.md (L3-54 pheno-tower-stack)
- V11 §70.2-§70.3 (DX/AX acceptance)
