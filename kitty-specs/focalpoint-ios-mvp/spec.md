# Feature: FocalPoint — iOS MVP

## Summary

Connector-first screen-time platform for iOS. Coachy (flame-shaped AI focus coach mascot) sits atop a Rust core (rules engine, reward/penalty dual ledger, SHA-256 audit chain) exposed via UniFFI. Canvas LMS is the first connector; QR-code scanning provides unlock proofs. FamilyControls enforcement integrates once Apple entitlement is approved; stubbed until then.

## Scope (MVP)

- iOS 17+ SwiftUI app (Swift 5.9+)
- Rust core: domain, events, rules, rewards, penalties, policy, audit, storage, sync, ffi, connector-canvas, connector-testkit, mascot
- Coachy mascot with 7 poses × 8 emotions; Spline-rendered (placeholder shapes until Spline SDK)
- Canvas OAuth2 + assignment/course/submission sync
- QR-code unlock proof
- Phenotype design tokens: #7EBAB5 / #0F1012 / #353A40 / #F6F5F5
- Foqos (MIT, 465★) as iOS donor codebase for NFC/QR/Screen Time integration

## Out of scope

- Android (deferred beyond Phase 5)
- Multi-device sync (deferred)
- NFC unlock (Phase 1.5)
- FamilyControls enforcement on real devices (gated on entitlement; stubbed until end-of-year target)
- Template marketplace (Phase 3)

## Acceptance criteria

- `cargo check --workspace` + `cargo test --workspace` + `cargo clippy -- -D warnings` green
- `swift build` at `apps/ios/FocalPoint/` succeeds
- All 26 FRs from FUNCTIONAL_REQUIREMENTS.md traced by tests
- Coachy renders all 7 poses; bubble copy matches mascot asset spec
- Canvas connector authenticates + ingests assignments; emits NormalizedEvents
- QR scanner reads test QR; unlock session recorded
- Audit chain verifies after 100+ mutation cycle
- App runs on iPhone 16 simulator + physical device (wireless via Tailscale OR wired)
