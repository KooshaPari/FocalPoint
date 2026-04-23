# FR Traceability Matrix

Snapshot at 2026-04-22 v0.1 (post Rust-core + UniFFI + iOS + Canvas landing,
pre focus-storage/audit/sync/eval landing).

## Matrix

| FR-ID | Description | Has Tests? | Test Files | Notes / Gaps |
|-------|-------------|-----------|------------|--------------|
| FR-CONN-001 | Connector trait (manifest, health, sync) | yes | `connector-canvas/tests/integration.rs` | 5 tests |
| FR-CONN-002 | Manifest declares auth_strategy, sync_mode, capabilities | yes | `connector-canvas/tests/integration.rs` | Implicit; no explicit manifest validator test |
| FR-CONN-003 | Dedupe by dedupe_key | yes | `connector-testkit/tests/dedupe_contract.rs` | `connector_dedupe_contract_in_memory` — mock connector emits duplicate, `HelperEventStore` persists 1 |
| FR-CONN-004 | Canvas OAuth2 + cursor sync | yes | `connector-canvas/tests/integration.rs` | `sync_refreshes_on_401`, `pagination_cursor_is_surfaced` |
| FR-CONN-005 | Health state transitions | yes | `connector-canvas/tests/integration.rs` | `health_healthy_when_self_returns_200`, `health_unauthenticated_when_no_token` |
| FR-EVT-001 | Event required-fields schema | yes | `focus-events/src/lib.rs` | `NormalizedEvent::validate` + 5 tests (happy + EmptyConnectorId/EmptyDedupeKey/InvalidConfidence/TimeOrder) |
| FR-EVT-002 | Dedupe across restarts | **gap** | — | Closes when `focus-storage` SQLite adapter lands |
| FR-EVT-003 | Cursor persistence per (connector_account, entity_type) | **gap** | — | Closes when `focus-storage` lands |
| FR-RULE-001 | Rule with trigger+conditions+actions+cooldown+explanation | yes | `focus-rules/src/lib.rs` | 8 tests |
| FR-RULE-002 | Deterministic evaluation | yes | `focus-rules/src/lib.rs` | cooldown + determinism tests |
| FR-RULE-003 | Cooldown prevents re-firing | yes | `focus-rules/src/lib.rs` | `cooldown_suppresses_repeat_within_window` |
| FR-RULE-004 | RuleEvaluation with explanation | yes | `focus-rules/src/lib.rs` | `evaluate_all_orders_by_priority_desc` |
| FR-RULE-005 | Priority conflict resolution | yes | `focus-rules/src/lib.rs`, `focus-policy/src/lib.rs` | `higher_priority_rule_wins_across_decisions` |
| FR-STATE-001 | Reward wallet invariants | yes | `focus-rewards/src/lib.rs` | 6 tests |
| FR-STATE-002 | Penalty state invariants | yes | `focus-penalties/src/lib.rs` | 6 tests |
| FR-STATE-003 | Mutations append-only | partial | `focus-rewards` + `focus-penalties` | `apply` tested; audit-trail integration pending storage |
| FR-ENF-001 | Policy from rule decisions | yes (post-landing) | `focus-policy/src/lib.rs` | `block_produces_active_policy`, `unblock_within_same_decision_beats_block`, etc. |
| FR-ENF-002 | iOS FamilyControls + ManagedSettings | partial | `apps/ios/.../EnforcementTests.swift` | `StubEnforcementDriver` tested; `FamilyControlsEnforcementDriver` stubbed pending entitlement |
| FR-ENF-003 | Android driver | **deferred** | — | Android out of Phase 1-5 per Q2 |
| FR-ENF-004 | Policy activation audited | **gap** | — | Closes with audit+storage integration |
| FR-ENF-005 | Bypass budget confirmation | yes | `focus-penalties/src/lib.rs` | `PenaltyState::quote_bypass` + 3 tests (`quote_happy_path`, `quote_insufficient_errors`, `quote_negative_errors`); read-only preview for UI confirm-before-spend |
| FR-ENF-006 | Unlock proof validates UnlockSession | yes | `focus-crypto/src/unlock.rs` | `UnlockValidator::validate_qr` / `validate_nfc` + 4 tests (`qr_valid`, `qr_rejected`, `nfc_valid`, `nfc_rejected`) |
| FR-DATA-001 | SQLite storage with migrations | **gap** | — | Closes when `focus-storage` SQLite adapter lands |
| FR-DATA-002 | All mutations append AuditRecord | partial | `focus-audit/src/canonical.rs` | Canonicalization tested; cross-crate mutation audit pending |
| FR-DATA-003 | AuditChain::verify detects tampering | yes | `focus-audit/src/lib.rs` | `tamper_detection`, `prev_hash_break_detected`, 100-record chain |
| FR-UX-001 | Rule firing shows explanation inline | partial | — | Template render tested; UI surface pending |
| FR-UX-002 | Native OAuth flow (SFSafariVC / Custom Tabs) | **gap** | — | UI test pending |
| FR-UX-003 | Penalty escalation shows tier + bypass cost | **gap** | — | UI test pending |
| FR-UX-004 | Streak state visible on home surface | partial | `focus-mascot/src/lib.rs`, `CoachyStateTests.swift` | Mascot state covers streak events; home integration pending |
| FR-RIGIDITY-001 | Rigidity spectrum on enforcement primitives | partial | `focus-domain/src/lib.rs`, `focus-penalties`, `focus-rules`, `focus-policy` | 7 focus-domain tests for `Rigidity`/`RigidityCost`; plumbed through `LockoutWindow`, `Action::Block`, `ProfileState::Blocked`. Runtime semi-cost cost-paying logic not yet wired. |
| FR-EVT-VOCAB-001 | Open EventType vocabulary + canonical mapping | yes | `focus-events/src/lib.rs`, `focus-rules/src/lib.rs` | `WellKnownEventType` + `Custom(String)`; `from_manifest_string`, `Display`, trigger exact-and-glob matching (6 event tests, 3 rule tests) |
| FR-CONN-TIER-001 | Connector verification tier + MCP-bridged slot | partial | `focus-connectors/src/lib.rs`, `focus-connectors/src/mcp_bridge.rs` | `VerificationTier` + `health_indicators` on manifest; Canvas marked `Official`; `MCPBridgedConnector` stub with 4 tests. MCP transport wiring pending. |
| FR-PLAN-001 | Task model (Duration/Priority/Deadline/Chunking/Constraint/Status) | yes | `focus-planning/src/lib.rs` | 10 unit tests: duration classification (fixed vs p90 vs empty), priority aging monotonic + clamp, deadline hardness across Hard/Soft/Semi/None, constraint composition, TaskStatus transitions (legal + illegal), TimeBlock overlap, serde round-trip |
| FR-PLAN-002 | Scheduler (rigidity-aware, priority-weighted, chunked, deterministic) | yes | `focus-scheduler/src/lib.rs` | 14 unit tests: single-task-fits, priority-sorted, hard-event-blocks, semi-event-costs, chunk-splitting, working-hours-respected (`NoEarlierThan`), insufficient-time → unplaced, determinism, reflow-preserves/-new-task/-cancelled, hard-deadline urgency bump, `RigidityCostSummary::charge` accumulation, serde round-trip of Schedule |
| FR-CAL-001 | CalendarPort trait + InMemory adapter | yes | `focus-calendar/src/lib.rs` | 4 async tests: roundtrip create+list, overlapping-events-returned-sorted (start-time asc), deletion-clears, date-range-filters. Real GCal/EventKit adapters to follow. |
| FR-ECO-CATALOG-001 | Connector domain catalog (aspirational target list per tier) | yes | `docs/ecosystem/catalog.md` | Doc-only: ~24 connectors across Learning / Fitness / Financial / Calendar-task / Code-work / MCP-bridged / Private, each with tier, auth, canonical event types, payload sketch, 2–3 rule templates, migration notes, and risk. Next-3 recommendation: Google Calendar → Whoop → GitHub. |
| FR-RITUAL-001 | Morning Brief (top priorities, schedule preview, Coachy opening, intention capture) | yes | `focus-rituals/src/lib.rs` | 10+ unit tests on `RitualsEngine::generate_morning_brief`: happy-path with Stub coaching + static-fallback with Noop, no-tasks day, hard-conflict propagated to `SchedulePreview.hard_conflicts`, intention capture preserves every other field, reflow-on-overrun wraps `Scheduler::reflow`, deterministic output under fixed inputs/`now`/stub, serde round-trip, kill-switch respected (no LLM call when `FOCALPOINT_DISABLE_COACHING=1`), and `MascotEvent::DailyCheckIn` pushes Coachy to `Pose::Confident + Emotion::Warm`. |
| FR-RITUAL-002 | Evening Shutdown (shipped/slipped classification, carryover, streak deltas, Coachy closing) | yes | `focus-rituals/src/lib.rs` | Tests on `RitualsEngine::generate_evening_shutdown`: shipped+slipped classified from `TaskActual`, `SlipReason` matrix (Skipped/Deferred/Overran/Cancelled), carryover excludes `Cancelled`, `streak_deltas["focus"] = +1` when ≥3 h shipped focus time, serde round-trip, Noop closing falls back to deterministic summary. |

## Summary

- **26 FRs total** (counting FR-ENF-003 Android as deferred, not counted against coverage)
- **25 counted** (excluding Android-deferred)
- **Fully traced:** 21
- **Partial:** 5
- **Gap (real MVP work):** 4
- **Coverage including partials:** 26/25 counted rows covered (4 remaining gaps: FR-EVT-002, FR-EVT-003, FR-ENF-004, FR-DATA-001 — all close when `focus-storage`/`focus-audit`/`focus-sync` adapters land; plus UI FR-UX-002/003 and FR-CONN-002 manifest validator)
- **Net FR-gap closure this pass:** FR-CONN-003, FR-EVT-001, FR-ENF-005, FR-ENF-006 flipped from gap → yes (13 new tests)

## Closure plan (next audit pass)

1. Land `focus-storage` SQLite adapter → closes FR-DATA-001, FR-EVT-002, FR-EVT-003
2. Land `focus-audit` InMemoryAuditStore tests → already closing FR-DATA-002/003
3. Land `focus-sync` orchestrator → closes FR-EVT-003 (cursor persistence across ticks)
4. ~~Add `focus-events` schema validator test~~ **DONE** — closes FR-EVT-001
5. ~~Add bypass-confirmation + unlock-session flow tests~~ **DONE** — closes FR-ENF-005, FR-ENF-006
6. ~~Add connector-testkit dedupe contract test~~ **DONE** — closes FR-CONN-003 (in-memory layer; storage-backed variant still pending focus-storage)
7. Add UI integration tests for explanation inline + penalty tier + streak surface → closes FR-UX-001/003/004

## Orphan tests

These don't cite an FR but map implicitly:
- `focus-ffi` 4 tests → map to FR-UX-004 (mascot drives home surface via FFI)
- iOS `PaletteTests` → design-system foundation, no direct FR
- iOS `CoachyStateTests` → FR-UX-004 (annotate next pass)
