---
title: Status Dashboard
description: Live status of FocalPoint's major subsystems and features.
---

# Status Dashboard

Updated 2026-04-23. All times UTC.

## Core Runtime

| Subsystem | Status | Notes |
|-----------|--------|-------|
| Rust workspace build | ✅ Shipped | All 6 crates building cleanly |
| Event sourcing (focus-events) | ✅ Shipped | SHA-256 hash chain + verification pass |
| Rule engine (focus-rules) | ✅ Shipped | DSL parser complete; evaluation harness ready |
| Reward/penalty ledger | ✅ Shipped | Dual ledger skeleton; scoring pending rules |
| Audit chain (focus-audit) | ✅ Shipped | Append-only store; genesis verification working |
| Policy engine (focus-policy) | ✅ Shipped | Decision stubs; no enforcement yet |

## iOS Integration

| Feature | Status | Blocker |
|---------|--------|---------|
| UniFFI bindings (focus-ffi) | ✅ Shipped | — |
| SwiftUI app scaffold | ✅ Shipped | — |
| FamilyControls bridge | 🔄 Partial | Entitlement pending Apple review |
| ManagedSettings enforcement | 🔄 Partial | Entitlement pending Apple review |
| DeviceActivity integration | ⏸ Blocked | Requires approved entitlement |
| Coachy mascot (Spline) | ✅ Shipped | Runtime embedded; no animation yet |

## Connectors

| Connector | Status | Notes |
|-----------|--------|-------|
| Canvas LMS | 🔄 Partial | Manifest + auth spec; SDK testing pending |
| Google Calendar | ⏸ Design phase | Scope doc in progress |
| Apple Health | ⏸ Design phase | FFI layer spec complete |
| GitHub | ⏸ Design phase | Prospective for developer journey |
| Todoist | ⏸ Planned | Lower priority; template support first |
| YNAB | ⏸ Planned | Lower priority; health signals first |
| MacroFactor | ⏸ Aspirational | Nutrition scope not finalized |

## SDK & Ecosystem

| Component | Status | Notes |
|-----------|--------|-------|
| Connector SDK spec | ✅ Shipped | Manifest, event schema, auth flows documented |
| Rule template format | ✅ Shipped | TOML + FPL embedded in ecosystem docs |
| Verification tiers | ✅ Shipped | 3-tier criteria; audit process sketched |
| Marketplace | ⏸ Design phase | Registry & publishing TBD |
| Node builder (UI) | ⏸ Design phase | Figma mockup in progress |

## Documentation

| Artifact | Status | Completeness |
|----------|--------|--------------|
| Quick Start | ✅ Shipped | macOS dev + iOS entitlement flow |
| User Guides | 🔄 Partial | Rule writing + template install; feedback/rituals TBD |
| Concepts | 🔄 Partial | Core loop, audit chain, DSL; Coachy personality in progress |
| Reference | ✅ Shipped | Design tokens, dual surface matrix, CLI, coverage tracking |
| Architecture | ✅ Shipped | System diagram, FFI topology, ADRs, design system |
| Reports | ✅ Shipped | iOS audit, performance baselines, accessibility, docs audit |
| Journeys | 🔄 Partial | Canvas + GitHub; wellness + productivity TBD |

## Releases & Deployment

| Milestone | Status | Target |
|-----------|--------|--------|
| App Store entitlement | ⏸ Blocked | Q2 2026 (Apple review) |
| TestFlight beta | ⏸ Waiting | After entitlement approval |
| Public pre-release | 📋 Planned | Q3 2026 |
| v1.0 GA | 📋 Planned | Q4 2026 |

## Test Coverage

| Layer | Status | Notes |
|-------|--------|-------|
| Unit tests (Rust) | 🔄 Partial | Core crates ~60% coverage; expanding |
| Integration tests | ⏸ Blocked | Need entitlement for iOS testing |
| E2E (iOS) | ⏸ Blocked | Device testing deferred until entitlement |
| Property-based (rules) | 🔄 In progress | Proptest suite for DSL parser |

## Legend

- **✅ Shipped** — ready for use or consumption
- **🔄 Partial** — in progress; some features working, others pending
- **⏸ Blocked** — waiting on external dependency (entitlement, decision, etc.)
- **📋 Planned** — on roadmap; not yet started
- **❌ Missing** — descoped or not applicable

## Known Blockers

1. **Apple entitlement review** — `com.apple.developer.family-controls` application pending. Blocks all device-level enforcement tests.
2. **Connector SDK polish** — Auth flows and event schema solid; testing harness needs expansion.
3. **Node builder UI** — Design finalized in Figma; implementation blocked on design system finalization.
4. **Multi-device sync** — Deferred to Phase 6; single-device MVP on track.
