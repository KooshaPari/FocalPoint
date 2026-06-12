# Changelog

All notable changes to `pheno-go-ctxkit` are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-12

### Added
- Initial scaffold (L3-52 cherry-pick)
- 3 unit + 2 race + 1 example tests = 6/6 passing
- `Background(traceID)` — root context with attached trace ID
- `WithTimeout`, `WithDeadline`, `WithCancel` — wrappers around `context.*` that propagate trace IDs
- `TraceID(ctx)` — extracts the trace ID; returns `""` if missing
- `IsCanceled(ctx)` — `ctx.Err() != nil` shortcut
- AI-DD crutches: `AGENTS.md`, `llms.txt`, `WORKLOG.md` (V2 schema), `LICENSE-MIT`
- This `CHANGELOG.md`

### Notes
- Go module: `github.com/kooshapari/ctxkit`
- Go 1.22+ (uses `slices`, `maps`, `cmp` from stdlib)
- Zero external dependencies
- L4 hexagonal: trace IDs compose with `pheno-otel` via `TraceID()` lookup
