# FocalPoint — Phased Plan

> Source: arch doc lines 592–625 + 1051–1189. This file is the indexed roadmap;
> detailed work packages WP-0001..WP-0503 live in source.

## Phases (DAG-ordered)

```
Phase 0: Discovery (scaffold done)
  └─▶ Phase 1: Core + iOS (CURRENT)
        └─▶ Phase 1.5: NFC unlock adapter
              └─▶ Phase 3: Connector ecosystem (iOS)
                    └─▶ Phase 4: Browser + extensions (Safari)
                          ├─▶ Phase 5: Optional backend services
                          └─▶ Phase 6+: Android revival (deferred)
```

**Scope narrowed 2026-04-22:** iOS-only for MVP → Phase 4. Android deferred
beyond Phase 5. See `docs/research/open_questions.md` Q2.

## Phase 0 — Discovery (now → v0.0.1 scaffold — **done**)

Goals:
- [x] Stack decisions locked (ADR-001..009)
- [x] Crate structure + stubs
- [x] PRD + FR + USER_JOURNEYS drafted
- [ ] Resolve open questions in `docs/research/open_questions.md`

## Phase 1 — Core + iOS (Aggressive: ~15–20 parallel agent batches, wall-clock ≈ 40–90 min each)

WP-0001..0050:
- Implement `focus-domain` invariants (`Result<T>`, ID newtypes, aggregates)
- Implement `focus-events` schema + dedupe factory
- Implement `focus-rules` engine (synchronous; cooldowns; explanation render)
- Implement `focus-storage::sqlite` (event store, rule store, wallet store, penalty store; migrations)
- Implement `focus-audit` chain + verify
- UniFFI UDL + Swift bindings generation for `focus-ffi::FocalPointCore`
- iOS shell: SwiftUI screens + FamilyControls entitlement flow + ManagedSettings driver
- Canvas connector (OAuth2 + course/assignment sync)
- Canvas fixture replay in `connector-testkit`

Exit: iOS app authenticates Canvas, ingests assignments, evaluates a sample rule, locks a test app, displays explanation, verifies audit chain.

## ~~Phase 2 — Android~~ DEFERRED beyond Phase 5

Scope narrowed 2026-04-22. Will revisit after iOS ships + ecosystem
gains traction. If revived:
- JNI bindings for `FocalPointCore`
- Compose shell
- UsageStats + AccessibilityService driver
- Android-side Canvas OAuth (Custom Tabs)
- Potential fork of `aload0/Reef` (MIT; requires rebrand per trademark reservation)

## Phase 3 — Connector ecosystem

- Connector SDK spec (`docs/connector-sdk/SPEC.md`)
- Connector manifest validator
- 3 more first-party connectors (Notion, Todoist, Apple Health)
- Rule template marketplace format (community-authorable)

## Phase 4 — Browser / extensions

- Safari Web Extension (macOS + iOS 17+)
- Chrome/Firefox extension for desktop distractor URLs

## Phase 5 — Optional backend

- `services/auth-broker` — OAuth callback server for connectors that don't support native redirect
- `services/webhook-ingest` — connector webhook fan-in
- `services/sync-api` — multi-device state sync

## Cross-cutting

- **Test strategy**: fixture-replay for every connector; property tests on rule engine; golden-file tests on explanation rendering
- **Security review** after Phase 1 (audit chain, token storage, FamilyControls entitlement scope)
- **Privacy review** before any external ingestion of connector data
