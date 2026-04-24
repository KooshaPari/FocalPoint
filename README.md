# FocalPoint

**Status:** v0.0.5 — Phase 1 in progress (~85%).
**⚠️ Compilation broken (2026-04-23).** 5 crates have unmerged E-series errors (backup borrow-check, rituals f32, connectors). See [honest_coverage.md](docs/reference/honest_coverage.md).

Connector-first screen-time management platform. Native iOS enforcement built on a portable Rust core: rules engine, connector runtime, reward/penalty ledger, audit chain, mascot state machine.

**What's shipped:**
- ✅ Domain layer end-to-end: event sourcing, rules engine, wallet/penalty ledgers, audit chain (hash-chained, tamper-evident), task scheduling, rituals (Morning Brief + Evening Shutdown)
- ✅ iOS shell: SwiftUI views, FamilyControls integration (awaiting entitlement approval), rule authoring wizard, Canvas OAuth
- ✅ 17 crates + 80+ passing tests (when workspace compiles)
- ✅ Multi-platform FFI (UniFFI Rust → Swift; JNI stubs for Android future)

**Honest gaps blocking production:**
- ❌ **Workspace compilation:** backup (E0505 borrow-check), rituals (E0277 Eq on f32), 3× connectors (type errors)
- ❌ **Apple entitlement:** FamilyControls driver logic shipped but gated behind `#if FOCALPOINT_HAS_FAMILYCONTROLS` flag. Awaiting Apple review (submitted Phase 0, 1–4 week SLA).
- ❌ **Onboarding UX:** 0 screens shipped. Users cannot self-serve setup today.
- ❌ **Designer assets:** Coachy 3D animation (`.riv` Rive file); SwiftUI placeholder in use.
- ❌ **Real-device QA:** simulator only; entitlement approval required for real testing.
- ❌ **GCal/GitHub OAuth:** buttons exist, flows incomplete.

**See [roadmap_v2.md](docs/roadmap_v2.md) for phased plan (6 phases, honest effort estimates, dependencies, and known deviations from earlier claims).**

## Primary differentiators

- **Connector runtime** treats Canvas LMS, calendars, tasks, health apps as
  first-class behavioral inputs. Ecosystem is the compounding moat, not blocking.
- **Rules engine** with explainable decisions, cooldowns, and state snapshots.
- **Reward/penalty dual-ledger** with escalation tiers, streaks, bypass budgets.
- **Portable Rust core** exported to iOS (Swift via UniFFI) + Android (Kotlin via JNI).

## Repo structure

```
crates/          Rust workspace — 17 crate stubs (domain core + mascot)
apps/ios/        SwiftUI + FamilyControls/ManagedSettings + Spline mascot
apps/android/    Deferred beyond Phase 2 (placeholder)
services/        Optional backend — deferred to Phase 5 (placeholders)
docs/            Architecture, ADRs, connector SDK, ecosystem strategy
examples/        Sample rules + connector fixtures
```

## Spec docs

- [`PRD.md`](PRD.md) — product requirements
- [`ADR.md`](ADR.md) — architecture decisions index
- [`FUNCTIONAL_REQUIREMENTS.md`](FUNCTIONAL_REQUIREMENTS.md) — FR-CONN/EVT/RULE/STATE/ENF/DATA/UX
- [`PLAN.md`](PLAN.md) — phased roadmap
- [`USER_JOURNEYS.md`](USER_JOURNEYS.md) — primary flows
- [`00_START_HERE.md`](00_START_HERE.md) — onboarding

## Build

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

iOS / Android builds pending UniFFI scaffold (Phase 1).

## Stack

- **Rust** (1.82, edition 2021) — shared core (will bind to Android later)
- **Swift 5.9+ / SwiftUI** — iOS 16+; FamilyControls entitlement required
- **SQLite** — local-first persistence
- **UniFFI** — Rust↔Swift bindings
- **Spline** — iOS mascot animation runtime (`crates/focus-mascot` + `apps/ios/Mascot/`)

Android Kotlin/Compose support reserved in `apps/android/` but deferred
beyond Phase 2. Cross-native frameworks (Tauri / RN / Flutter) rejected
per ADR-001.

See [`ADR.md`](ADR.md) for the full decision log.

## Open questions

Tracked in [`docs/research/open_questions.md`](docs/research/open_questions.md).

## License

MIT OR Apache-2.0.
