# Templates-Registry Modernization (W-70)

## Context
FocalPoint's templates-registry already uses `axum = "0.7"` on edition 2021. The single offending dep is `multipart = "0.18"` — root cause of all 19 cargo-deny advisories (5 CVE + 14 unmaintained: hyper x2, idna, protobuf, time, iron, nickel, hyper 0.10 transitives). Removing `multipart 0.18` retires all 19.

## Phased WBS

### Phase 1 — Discovery (~25 tool-calls)
- Enumerate `multipart::*` use sites in `services/templates-registry/src/{handlers.rs,main.rs,models.rs}`.
- Inventory consumers: grep `apps/ crates/ tests/` for `/templates/upload`. Capture wire contract from `openapi.yaml`.
- Confirm signing/integrity flow: is digest over raw bytes or decoded archive? Dictates streaming safety.
- Capture cargo-deny baseline.

### Phase 2 — Replacement Crate Selection (~10 calls)
- Default: `axum::extract::Multipart` (transitively present, zero new deps, tower limits).
- Fallback: `multer = "3"` if framework-agnostic parsing needed.
- Reject: `actix-multipart`, `mpart-async`.
- Record in ADR.

### Phase 3 — API Compat Layer (~40 calls)
- Reimplement upload handler against `axum::extract::Multipart`. Preserve field names, max body size (`DefaultBodyLimit` + per-field cap), content-type tolerances, error codes, signature ordering.
- Stream each field to `tempfile` for memory-bounded large bundles.
- Compute digest while streaming.
- Lock response JSON shape with openapi.yaml snapshot.

### Phase 4 — Cutover (~30 calls)
- Golden contract tests (reqwest in-process axum) before code change.
- Swap dep: drop `multipart`, verify no transitive pull.
- `cargo deny check advisories` → confirm 19/19 cleared.
- Perf smoke: 50 MB bundle, compare p50/p95.

### Phase 5 — Decom (~10 calls)
- Remove deny.toml W-70 exemption.
- Update CHANGELOG.md + sbom.cdx.json.
- Close W-70 with cargo-deny diff.

Total: ~115 tool-calls.

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| API break | Med | High | Golden contract tests captured Phase 1; openapi snapshot diff in CI |
| Perf regression on large bundles | Low | Med | Stream to tempfile; bench in Phase 4 |
| Signing/integrity drift | Med | Critical | Co-locate digest with reader; bit-identical signature regression test |
| Body-limit semantics (axum default 2MB) | High | Med | Explicit `DefaultBodyLimit::max(...)` + assertion test |
| Transitive hyper 0.10 re-intro | Low | High | `deny.toml` ban hyper<0.14, iron, nickel, multipart 0.18 post-cutover |
| Edition-2024 coupling | Low | Low | Out of W-70 scope |

## Critical Files
- services/templates-registry/Cargo.toml
- services/templates-registry/src/{handlers,main,models}.rs
- services/templates-registry/openapi.yaml
- deny.toml
