# Changelog — pheno-tokio-base

## [0.1.0] - 2026-06-11

### Added
- `run(future)` — main entry, install tracing-subscriber, then await the future
- `test_helpers::block_on(future)` — sync test runner (no #[tokio::test] macro needed)
- 2 unit tests (tracing_init_smoke, block_on_returns_value) — all green
- L1 Stabilize: clean wrapper for tokio + tracing init; 1st pheno-* runtime-helper lib
- L16 AX: AGENTS.md, llms.txt, WORKLOG.md (V2 10-col schema)

### Known limitations
- Single-threaded runtime by default (use `#[tokio::main(flavor = "multi_thread")]` if needed)
- No graceful shutdown helper yet

### Refs
- FLEET_100TASK_DAG_V4.md §83.6 (V15 - 74 tests target)
- V3_EXECUTION_LOG_2026_06_10.md (L3-54 pheno-tower-stack)
- V11 §70.2-§70.3 (DX/AX acceptance)
