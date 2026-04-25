# Open Questions — block before meaningful impl

Source: scaffold-plan agent output (2026-04-22).

## Q1 — Final project name

Arch doc uses "Latch" placeholder; scaffolded as "FocalPoint" per user.
**Decide** final brand name before any published artifact (crate names,
bundle IDs, store listings). If "FocalPoint" sticks, `focus-*` crate prefix
is correct.

## Q2 — Single-device vs multi-device MVP

Arch doc flags this open (line 627). Affects:
- Whether `services/sync-api/` is Phase 1 or Phase 5
- Whether `focus-storage` needs conflict-resolution semantics now or later
- Whether iOS + Android share a user account or are independent

**Default assumed:** single-device v1, multi-device Phase 3+.

## Q3 — Unlock proof MVP scope

QR-only vs NFC-required (line 631). Affects:
- iOS entitlements (CoreNFC requires specific entitlement + device hardware)
- `FR-ENF-006` adapter scope
- User onboarding flow complexity

**Default assumed:** QR MVP; NFC optional in Phase 1.5.

## Q4 — Backend language for optional services

Rust vs Go (line 633). Affects:
- `services/auth-broker`, `services/webhook-ingest`, `services/sync-api` scaffold
- CI toolchain
- Ops story

**Default assumed:** Rust (consistent with core). Flag if Go preferred for auth-broker specifically.

## Q5 — Foqos + Reef reference repos

Arch doc names them stylistically ("Foqos-style", "Reef-style") without
GitHub URLs or license info. Before copying any code:
- [ ] Confirm exact upstream repos
- [ ] License compat check (MIT-compatible for our dual license)
- [ ] Fork private vs. reference-only decision
- [ ] Scope: donor for `apps/ios/`, `apps/android/`, or just patterns?

## Q6 — Mascot + LLM personality system

Prompt 3 in source doc (lines 1846–1862) truncated mid-thought. No crate/module exists.
Decide: reserve namespaces `crates/focus-mascot/` + `apps/ios/Mascot/` now, or
defer entirely?

**Default assumed:** defer; revisit after Phase 2.

## Q7 — Rules-pack / template marketplace format

Ecosystem strategy calls for template marketplace before community connectors,
but schema not defined. Needs:
- Template JSON/YAML schema
- Signing + verification
- Distribution channel (app-internal vs external)

**Default assumed:** defer to Phase 3.

## Q8 — FamilyControls entitlement application

**RESOLVED (2026-04-24):** MockFamilyControls adapter deployed.

Apple requires a specific entitlement for FamilyControls/DeviceActivity APIs.
Entitlement application timeline: 1–4 weeks.

**POC Unblock:** Deployed `crates/focus-connectors-mock-familycontrols/` with:
- Deterministic synthetic event generation (app launches, screen-time, interventions, emergency-exit)
- 20 unit tests covering all event types
- Feature flag wiring (`mock-familycontrols` in focus-ffi)
- UniFFI bridge stub for iOS compilation against mock data

Full enforcement loop (rules → eval → penalties → audit) now demo-able end-to-end
on TestFlight without the entitlement. Actual FamilyControls integration unblocks
when Apple grants entitlement.
