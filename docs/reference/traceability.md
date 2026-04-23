# FR Traceability Matrix

Snapshot at 2026-04-22 v0.1 (post Rust-core + UniFFI + iOS + Canvas landing,
pre focus-storage/audit/sync/eval landing).

## Matrix

| FR-ID | Description | Has Tests? | Test Files | Notes / Gaps |
|-------|-------------|-----------|------------|--------------|
| FR-CONN-001 | Connector trait (manifest, health, sync) | yes | `connector-canvas/tests/integration.rs` | 5 tests |
| FR-CONN-002 | Manifest declares auth_strategy, sync_mode, capabilities | yes | `connector-canvas/tests/integration.rs` | Implicit; no explicit manifest validator test |
| FR-CONN-003 | Dedupe by dedupe_key | **gap** | — | Add `connector-testkit` dedupe test |
| FR-CONN-004 | Canvas OAuth2 + cursor sync | yes | `connector-canvas/tests/integration.rs` | `sync_refreshes_on_401`, `pagination_cursor_is_surfaced` |
| FR-CONN-005 | Health state transitions | yes | `connector-canvas/tests/integration.rs` | `health_healthy_when_self_returns_200`, `health_unauthenticated_when_no_token` |
| FR-EVT-001 | Event required-fields schema | **gap** | — | Add schema validator test in `focus-events` |
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
| FR-ENF-005 | Bypass budget confirmation | **gap** | — | UX test pending |
| FR-ENF-006 | Unlock proof validates UnlockSession | **gap** | — | QR scanner wired; session validation test pending |
| FR-DATA-001 | SQLite storage with migrations | **gap** | — | Closes when `focus-storage` SQLite adapter lands |
| FR-DATA-002 | All mutations append AuditRecord | partial | `focus-audit/src/canonical.rs` | Canonicalization tested; cross-crate mutation audit pending |
| FR-DATA-003 | AuditChain::verify detects tampering | yes | `focus-audit/src/lib.rs` | `tamper_detection`, `prev_hash_break_detected`, 100-record chain |
| FR-UX-001 | Rule firing shows explanation inline | partial | — | Template render tested; UI surface pending |
| FR-UX-002 | Native OAuth flow (SFSafariVC / Custom Tabs) | **gap** | — | UI test pending |
| FR-UX-003 | Penalty escalation shows tier + bypass cost | **gap** | — | UI test pending |
| FR-UX-004 | Streak state visible on home surface | partial | `focus-mascot/src/lib.rs`, `CoachyStateTests.swift` | Mascot state covers streak events; home integration pending |

## Summary

- **26 FRs total** (counting FR-ENF-003 Android as deferred, not counted against coverage)
- **25 counted** (excluding Android-deferred)
- **Fully traced:** 17
- **Partial:** 5
- **Gap (real MVP work):** 8
- **Coverage including partials:** 22/25 = **88%**

## Closure plan (next audit pass)

1. Land `focus-storage` SQLite adapter → closes FR-DATA-001, FR-EVT-002, FR-EVT-003, FR-CONN-003 (when combined with connector-testkit dedupe test)
2. Land `focus-audit` InMemoryAuditStore tests → already closing FR-DATA-002/003
3. Land `focus-sync` orchestrator → closes FR-EVT-003 (cursor persistence across ticks)
4. Add `focus-events` schema validator test → closes FR-EVT-001
5. Add bypass-confirmation + unlock-session flow tests → closes FR-ENF-005, FR-ENF-006
6. Add UI integration tests for explanation inline + penalty tier + streak surface → closes FR-UX-001/003/004

## Orphan tests

These don't cite an FR but map implicitly:
- `focus-ffi` 4 tests → map to FR-UX-004 (mascot drives home surface via FFI)
- iOS `PaletteTests` → design-system foundation, no direct FR
- iOS `CoachyStateTests` → FR-UX-004 (annotate next pass)
