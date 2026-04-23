# Start Here — FocalPoint

**Status:** v0.0.1 scaffold (2026-04-22).

## What is it

Connector-first screen-time platform. Rust core + native iOS/Android.
See [`README.md`](README.md) for the pitch.

## Read in this order

1. [`README.md`](README.md) — pitch + build
2. [`PRD.md`](PRD.md) — product intent
3. [`ADR.md`](ADR.md) — stack decisions (→ `docs/adr/`)
4. [`FUNCTIONAL_REQUIREMENTS.md`](FUNCTIONAL_REQUIREMENTS.md) — what it must do
5. [`USER_JOURNEYS.md`](USER_JOURNEYS.md) — how it feels
6. [`PLAN.md`](PLAN.md) — phased roadmap
7. [`docs/research/open_questions.md`](docs/research/open_questions.md) — unresolved before impl

## Crate layout

See [`crates/`](crates/). Each crate has a 2-line top-level doc comment.
Start with `focus-domain` → `focus-events` → `focus-connectors` → `focus-rules`
for the domain spine.

## Apps

- [`apps/ios/`](apps/ios/) — SwiftUI + FamilyControls (stub)
- [`apps/android/`](apps/android/) — Compose + UsageStats (stub)

## Open questions that block meaningful impl

Top 3 (full list in `docs/research/open_questions.md`):
1. Project name final — "FocalPoint" vs "Latch" (arch doc placeholder).
2. Single-device vs multi-device MVP.
3. Foqos + Reef reference-repo GitHub URLs + license compat.

## How to contribute stubs → impl

Pick a crate with a `// Stub` comment, write impl + tests. Keep the trait
surface stable — ports/adapters separation (see `docs/reference/` — pending).
