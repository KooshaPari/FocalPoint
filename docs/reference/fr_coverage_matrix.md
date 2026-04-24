# FR-to-Test Traceability Matrix

## Summary

- **Total FRs:** 37
- **Covered (≥1 test):** 28
- **Missing (0 tests):** 9
- **Orphan tests:** 104

## Coverage Matrix

| FR ID | Description | Test Files | Status |
|-------|-------------|-----------|--------|
| FR-CONN-001 | Connector implements the `Connector` trait (`manifest`, `health`, `sync(cursor)`). | `lib.rs` | ✅ GREEN |
| FR-CONN-002 | Manifest declares auth_strategy, sync_mode, capabilities, entity_types, event_types. | `lib.rs` | ✅ GREEN |
| FR-CONN-003 | Connector emits `NormalizedEvent`s with dedupe_key; same event ingested twice → exactly one persisted record. | `connector_registration.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `retry.rs`, `retry.rs`, `retry.rs`, `retry.rs`, `dedupe_contract.rs`, `dedupe_contract.rs`, `lib.rs` | ✅ GREEN |
| FR-CONN-004 | Canvas connector supports OAuth2 code flow + cursor-based assignment/course sync. |  | ❌ MISSING |
| FR-CONN-005 | Connector health transitions observable via `HealthState`. | `lib.rs` | ✅ GREEN |
| FR-DATA-001 | SQLite storage with migrations in `focus-storage::sqlite`. | `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `task_store.rs`, `task_store.rs`, `task_store.rs`, `task_store.rs`, `task_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `mod.rs`, `migrations.rs`, `migrations.rs`, `migrations.rs`, `rule_store.rs`, `wallet_store.rs`, `event_store.rs`, `penalty_store.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-DATA-002 | All state mutations append an `AuditRecord`. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `canonical.rs`, `canonical.rs`, `canonical.rs`, `canonical.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `mod.rs`, `audit_store.rs`, `auth.rs`, `auth.rs`, `auth.rs`, `auth.rs` | ✅ GREEN |
| FR-DATA-003 | `AuditChain::verify_chain()` detects tampering (hash mismatch). | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `canonical.rs`, `canonical.rs`, `canonical.rs`, `canonical.rs` | ✅ GREEN |
| FR-ENF-001 | Enforcement policy generated from active rule decisions. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENF-002 | iOS driver applies policy via FamilyControls + ManagedSettings. |  | ❌ MISSING |
| FR-ENF-003 | Android driver applies policy via UsageStats + AccessibilityService. |  | ❌ MISSING |
| FR-ENF-004 | Policy activation/deactivation is audited. | `lib.rs` | ✅ GREEN |
| FR-ENF-005 | Bypass budget spend requires user confirmation. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENF-006 | Unlock proof (QR/NFC) validates against an `UnlockSession` record. |  | ❌ MISSING |
| FR-EVT-001 | Every event has `event_id`, `connector_id`, `account_id`, `event_type`, `occurred_at`, `effective_at`, `dedupe_key`, `confidence`, `payload`. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-EVT-002 | Dedupe by `dedupe_key` across restarts. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `event_store.rs` | ✅ GREEN |
| FR-EVT-003 | Cursor progress persisted per (connector_account, entity_type). | `lib.rs`, `lib.rs`, `lib.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `migrations.rs` | ✅ GREEN |
| FR-PLAN-001 | Tasks and goals stored with priority, due_date, completed_at, status enum. | `task_schedule_transpiler.rs`, `task_store.rs`, `task_store.rs`, `task_store.rs`, `migrations.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `EndToEndLoopTests.swift`, `TasksView.swift` | ✅ GREEN |
| FR-PLAN-002 | Task scheduling with temporal triggers (cron-style recurrence, date-specific, duration-based). | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-RIGIDITY-001 | Block actions carry rigidity level: Soft (dismissible) vs Hard (non-dismissible). | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-RITUAL-001 | Ritual model with name, schedule, checkpoint_window, description, enabled flag. | `ritual_transpiler.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-RITUAL-002 | Ritual completion recorded as completion event with timestamp and metadata. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-RITUAL-003 | Weekly/monthly ritual cadences supported via temporal expressions. | `weekly.rs`, `weekly.rs`, `weekly.rs`, `weekly.rs`, `weekly.rs`, `weekly.rs`, `weekly.rs` | ✅ GREEN |
| FR-RITUAL-004 | Ritual streak tracking: current, longest, last_completion date. | `monthly.rs`, `monthly.rs`, `monthly.rs`, `monthly.rs`, `monthly.rs`, `monthly.rs`, `monthly.rs` | ✅ GREEN |
| FR-RULE-001 | Rule with trigger + conditions + actions + cooldown + explanation_template. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-RULE-002 | Rule evaluation is deterministic given (rule, event, state_snapshot). | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-RULE-003 | Cooldown prevents re-firing within window. | `lib.rs`, `lib.rs`, `lib.rs`, `builder.rs` | ✅ GREEN |
| FR-RULE-004 | Each evaluation produces a `RuleEvaluation` record with explanation. | `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-RULE-005 | Priority resolves contradicting rule actions (higher priority wins). | `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-STATE-001 | Reward wallet: earned_credits, spent_credits, streaks, unlock_balances, multiplier_state. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `wallet_store.rs` | ✅ GREEN |
| FR-STATE-002 | Penalty state: escalation_tier, bypass_budget, lockout_windows, debt_balance, strict_mode_until. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `penalty_store.rs` | ✅ GREEN |
| FR-STATE-003 | Mutations append-only via `WalletMutation` / `PenaltyMutation`. |  | ❌ MISSING |
| FR-STATE-004 | All state mutations are immutably recorded in audit log; mutations can be replayed from audit. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `audit_store.rs`, `migrations.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-UX-001 | Rule firing shows explanation inline. |  | ❌ MISSING |
| FR-UX-002 | Connector auth flow is platform-native (SFSafariViewController / Custom Tabs). |  | ❌ MISSING |
| FR-UX-003 | Penalty escalation shows tier + bypass cost before commit. |  | ❌ MISSING |
| FR-UX-004 | Streak state is visible on home surface. |  | ❌ MISSING |

## Orphan Tests

Tests that reference non-existent FRs:

- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-transpilers/src/enforcement_policy_transpiler.rs**: FR-POLICY-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-transpilers/src/connector_transpiler.rs**: FR-CONNECTOR-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-transpilers/src/wallet_mutation_transpiler.rs**: FR-REWARDS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-icon-gen/src/lib.rs**: FR-APPSTORE-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-icon-gen/src/lib.rs**: FR-APPSTORE-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-icon-gen/src/lib.rs**: FR-APPSTORE-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-icon-gen/src/lib.rs**: FR-APPSTORE-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-linear/src/lib.rs**: FR-LINEAR-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-linear/src/lib.rs**: FR-LINEAR-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-fitbit/src/models.rs**: FR-FITBIT-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-fitbit/src/models.rs**: FR-FITBIT-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-fitbit/src/models.rs**: FR-FITBIT-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-fitbit/src/api.rs**: FR-FITBIT-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-006
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-007
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-008
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-009
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-010
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-011
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-012
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-013
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-014
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-015
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-016
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-017
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-018
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-006
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-006
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-007
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-008
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-009
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-010
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-011
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-012
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-013
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-014
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-015
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-016
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-017
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-018
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-entitlements/src/lib.rs**: FR-ENTITLEMENTS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-ffi/src/lib.rs**: FR-CAL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-rules/src/lib.rs**: FR-RULE-006
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-rules/src/lib.rs**: FR-RULE-008
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-rules/src/lib.rs**: FR-RULE-007
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-rules/src/builder.rs**: FR-RULE-008
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-eval/src/lib.rs**: FR-FOCUS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-eval/src/lib.rs**: FR-FOCUS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-eval/src/lib.rs**: FR-FOCUS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-eval/src/lib.rs**: FR-FOCUS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-eval/src/lib.rs**: FR-FOCUS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-eval/src/lib.rs**: FR-FOCUS-006
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-eval/src/lib.rs**: FR-FOCUS-007
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-eval/src/lib.rs**: FR-FOCUS-008
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-storage/src/wipe.rs**: FR-PRIVACY-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-storage/src/wipe.rs**: FR-PRIVACY-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-storage/src/wipe.rs**: FR-PRIVACY-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-storage/src/wipe.rs**: FR-PRIVACY-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-storage/src/wipe.rs**: FR-PRIVACY-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-readwise/src/lib.rs**: FR-READWISE-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-readwise/src/lib.rs**: FR-READWISE-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-notion/src/lib.rs**: FR-NOTION-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/connector-notion/src/lib.rs**: FR-NOTION-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-calendar/src/lib.rs**: FR-CAL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-calendar/src/lib.rs**: FR-CAL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-calendar/src/lib.rs**: FR-CAL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-calendar/src/lib.rs**: FR-CAL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-calendar/src/lib.rs**: FR-CAL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/EndToEndLoopTests.swift**: FR-REWARD-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/EndToEndLoopTests.swift**: FR-AUDIT-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/EndToEndLoopTests.swift**: FR-AUDIT-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/EndToEndLoopTests.swift**: FR-AUDIT-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/EndToEndLoopTests.swift**: FR-SYNC-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/EndToEndLoopTests.swift**: FR-MASCOT-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/SentryIntegrationTests.swift**: FR-DIAG-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/SentryIntegrationTests.swift**: FR-DIAG-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointAppSnapshotTests/EnforcementSnapshotTests.swift**: FR-ENFORCE-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointAppSnapshotTests/MascotUISnapshotTests.swift**: FR-MASCOT-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointAppSnapshotTests/CoreTabsSnapshotTests.swift**: FR-TAB-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointAppSnapshotTests/DesignSystemSnapshotTests.swift**: FR-UI-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointAppSnapshotTests/OnboardingSnapshotTests.swift**: FR-ONBOARD-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointAppTests/OnboardingConsentTests.swift**: FR-ONBOARDING-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Sources/FocalPointApp/SentrySetup.swift**: FR-DIAG-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Sources/FocalPointApp/SentrySetup.swift**: FR-DIAG-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Sources/FocalPointApp/Observability/SentryPrivacyFilter.swift**: FR-DIAG-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Sources/FocalPointApp/Adapters/EventKitCalendarHost.swift**: FR-CAL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Sources/FocalPointApp/Adapters/HealthKitHost.swift**: FR-HEALTHKIT-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Sources/FocalPointApp/CoreHolder.swift**: FR-CAL-001
