# FocalPoint

Connector-first screen-time management platform. Native device enforcement
(iOS + Android) built on a portable Rust core: rules engine, connector
runtime, reward/penalty ledger, audit chain.

**Status:** v0.0.1 scaffold — structure + stubs only, no business logic yet.

## Primary differentiators

- **Connector runtime** treats Canvas LMS, calendars, tasks, health apps as
  first-class behavioral inputs. Ecosystem is the compounding moat, not blocking.
- **Rules engine** with explainable decisions, cooldowns, and state snapshots.
- **Reward/penalty dual-ledger** with escalation tiers, streaks, bypass budgets.
- **Portable Rust core** exported to iOS (Swift via UniFFI) + Android (Kotlin via JNI).

## Repo structure

```
crates/          Rust workspace — domain core
apps/ios/        SwiftUI + FamilyControls/DeviceActivity/ManagedSettings/CoreNFC
apps/android/    Jetpack Compose + UsageStats/AccessibilityService
services/        Optional backend (auth-broker, webhook-ingest, sync-api)
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

- **Rust** (1.82, edition 2021) — shared core
- **Swift 5.9+ / SwiftUI** — iOS (iOS 16+; Screen Time APIs require parental-control entitlement)
- **Kotlin / Jetpack Compose** — Android (minSdk 24)
- **SQLite** — local-first persistence
- **UniFFI** — Rust↔Swift bindings
- **JNI** — Rust↔Kotlin bindings

See [`ADR.md`](ADR.md) for the full decision log.

## Open questions

Tracked in [`docs/research/open_questions.md`](docs/research/open_questions.md).

## License

MIT OR Apache-2.0.
