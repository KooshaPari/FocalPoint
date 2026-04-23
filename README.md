# FocalPoint

Connector-first screen-time management platform. Native iOS enforcement
built on a portable Rust core: rules engine, connector runtime,
reward/penalty ledger, audit chain, mascot state machine.

**Status:** v0.0.3 — end-to-end loop landed (2026-04-23). App runs
the full flow: add task → connect tool → sync pulls events → rules
evaluate → wallet/penalty/policy mutate → audit chain records → UI
surfaces it through Today / Tasks / Activity / Settings. 22/26 FRs
genuinely shipped. See [CHANGELOG.md](CHANGELOG.md).

Remaining gaps are external blockers: Apple FamilyControls entitlement,
visual connector builder, template-pack signing, Coachy 3D art.
Android deferred beyond Phase 2.

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
