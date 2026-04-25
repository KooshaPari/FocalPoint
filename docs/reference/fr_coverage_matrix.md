# FR-to-Test Traceability Matrix

## Summary

- **Total FRs:** 67
- **Covered (≥1 test):** 59
- **Missing (0 tests):** 8
- **Orphan tests:** 127

## Coverage Matrix

| FR ID | Description | Test Files | Status |
|-------|-------------|-----------|--------|
| FR-APPSTORE-001 | App icon generation and versioning for app store distribution. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-CONN-001 | Connector implements the `Connector` trait (`manifest`, `health`, `sync(cursor)`). | `lib.rs` | ✅ GREEN |
| FR-CONN-002 | Manifest declares auth_strategy, sync_mode, capabilities, entity_types, event_types. | `lib.rs` | ✅ GREEN |
| FR-CONN-003 | Connector emits `NormalizedEvent`s with dedupe_key; same event ingested twice → exactly one persisted record. | `connector_registration.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `retry.rs`, `retry.rs`, `retry.rs`, `retry.rs`, `dedupe_contract.rs`, `dedupe_contract.rs`, `lib.rs` | ✅ GREEN |
| FR-CONN-004 | Canvas connector supports OAuth2 code flow + cursor-based assignment/course sync. |  | ❌ MISSING |
| FR-CONN-005 | Connector health transitions observable via `HealthState`. | `lib.rs` | ✅ GREEN |
| FR-CONNECTOR-001 | Connector trait: manifest, health, sync implementations across all supported providers. | `connector_transpiler.rs`, `lib.rs`, `lib.rs`, `models.rs`, `models.rs`, `models.rs`, `api.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `EventKitCalendarHost.swift`, `CoreHolder.swift` | ✅ GREEN |
| FR-DATA-001 | SQLite storage with migrations in `focus-storage::sqlite`. | `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `task_store.rs`, `task_store.rs`, `task_store.rs`, `task_store.rs`, `task_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `mod.rs`, `migrations.rs`, `migrations.rs`, `migrations.rs`, `rule_store.rs`, `wallet_store.rs`, `event_store.rs`, `penalty_store.rs`, `lib.rs`, `lib.rs`, `CoreHolderE2ETests.swift`, `CoreHolderE2ETests.swift` | ✅ GREEN |
| FR-DATA-002 | All state mutations append an `AuditRecord`. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `canonical.rs`, `canonical.rs`, `canonical.rs`, `canonical.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `keychain.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `mod.rs`, `audit_store.rs`, `auth.rs`, `auth.rs`, `auth.rs`, `auth.rs`, `CoreHolderE2ETests.swift`, `CoreHolderE2ETests.swift`, `CoreHolderE2ETests.swift`, `EndToEndLoopTests.swift`, `EndToEndLoopTests.swift`, `EndToEndLoopTests.swift` | ✅ GREEN |
| FR-DATA-003 | `AuditChain::verify_chain()` detects tampering (hash mismatch). | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `canonical.rs`, `canonical.rs`, `canonical.rs`, `canonical.rs` | ✅ GREEN |
| FR-DIAG-001 | Sentry integration for crash reporting and error tracking. | `SentryIntegrationTests.swift`, `SentrySetup.swift` | ✅ GREEN |
| FR-DIAG-002 | Privacy filters applied to diagnostic telemetry (no PII). | `SentryIntegrationTests.swift`, `SentrySetup.swift`, `SentryPrivacyFilter.swift` | ✅ GREEN |
| FR-ENF-001 | Enforcement policy generated from active rule decisions. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENF-002 | iOS driver applies policy via FamilyControls + ManagedSettings. |  | ❌ MISSING |
| FR-ENF-003 | Android driver applies policy via UsageStats + AccessibilityService. | `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENF-004 | Policy activation/deactivation is audited. | `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENF-005 | Bypass budget spend requires user confirmation. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENF-006 | Unlock proof (QR/NFC) validates against an `UnlockSession` record. | `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENFORCE-001 | Enforcement policy visual presentation and management. | `EnforcementSnapshotTests.swift` | ✅ GREEN |
| FR-ENTITLEMENTS-001 | Subscription tiers: Free, Plus, Pro, Family with distinct feature gates. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENTITLEMENTS-002 | Tier-specific limits enforced: max_rules, max_tasks, connector_cadence, voice_synthesis. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-ENTITLEMENTS-003 | Feature gates evaluated consistently across iOS app and Rust backend. |  | ❌ MISSING |
| FR-EVT-001 | Every event has `event_id`, `connector_id`, `account_id`, `event_type`, `occurred_at`, `effective_at`, `dedupe_key`, `confidence`, `payload`. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-EVT-002 | Dedupe by `dedupe_key` across restarts. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `event_store.rs` | ✅ GREEN |
| FR-EVT-003 | Cursor progress persisted per (connector_account, entity_type). | `lib.rs`, `lib.rs`, `lib.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `cursor_store.rs`, `migrations.rs` | ✅ GREEN |
| FR-FOCUS-001 | Rule evaluation engine: matches events against active rules. | `lib.rs` | ✅ GREEN |
| FR-FOCUS-002 | State snapshot incorporation for contextual rule decisions. | `lib.rs` | ✅ GREEN |
| FR-FOCUS-003 | Asynchronous rule evaluation with retry semantics. | `lib.rs` | ✅ GREEN |
| FR-FOCUS-004 | Evaluation result caching for performance. | `lib.rs` | ✅ GREEN |
| FR-FOCUS-005 | Rule conflict resolution during multi-rule firings. | `lib.rs` | ✅ GREEN |
| FR-FOCUS-006 | Evaluation metrics and observability signals. | `lib.rs` | ✅ GREEN |
| FR-FOCUS-007 | Rule explanation generation for fired actions. | `lib.rs` | ✅ GREEN |
| FR-FOCUS-008 | Evaluation determinism guarantees across restarts. | `lib.rs` | ✅ GREEN |
| FR-HEALTHKIT-001 | HealthKit data adapter and health metrics integration. | `HealthKitHost.swift` | ✅ GREEN |
| FR-MASCOT-001 | Mascot UI surfaces personalized coaching messages. | `EndToEndLoopTests.swift`, `MascotUISnapshotTests.swift` | ✅ GREEN |
| FR-MASCOT-002 | Coaching message generation from rule evaluations and streaks. |  | ❌ MISSING |
| FR-ONBOARD-001 | Consent flow for permissions and tracking. | `OnboardingSnapshotTests.swift` | ✅ GREEN |
| FR-ONBOARDING-005 | Entitlement tier selection during signup. | `OnboardingConsentTests.swift` | ✅ GREEN |
| FR-PLAN-001 | Tasks and goals stored with priority, due_date, completed_at, status enum. | `task_schedule_transpiler.rs`, `task_store.rs`, `task_store.rs`, `task_store.rs`, `migrations.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `CoreHolderE2ETests.swift`, `EndToEndLoopTests.swift`, `TasksView.swift` | ✅ GREEN |
| FR-PLAN-002 | Task scheduling with temporal triggers (cron-style recurrence, date-specific, duration-based). | `packing.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-POLICY-001 | Policy generation from rule decisions, conflict resolution via priority. | `enforcement_policy_transpiler.rs`, `CoreHolderE2ETests.swift`, `CoreHolderE2ETests.swift` | ✅ GREEN |
| FR-PRIVACY-001 | Data wiping: purge all user events, rules, state, audit logs on request. | `wipe.rs`, `wipe.rs`, `wipe.rs`, `wipe.rs`, `wipe.rs` | ✅ GREEN |
| FR-REWARDS-001 | Reward wallet mutations, credit earning/spending/streak tracking. | `wallet_mutation_transpiler.rs`, `CoreHolderE2ETests.swift`, `CoreHolderE2ETests.swift`, `CoreHolderE2ETests.swift`, `EndToEndLoopTests.swift` | ✅ GREEN |
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
| FR-RULE-006 | Rule DSL supports temporal conditions and scheduler integration. | `lib.rs` | ✅ GREEN |
| FR-RULE-007 | Rule builder API enables fluent construction and validation. | `lib.rs` | ✅ GREEN |
| FR-RULE-008 | Rule action explainability: each action has rationale text. | `lib.rs`, `builder.rs` | ✅ GREEN |
| FR-STATE-001 | Reward wallet: earned_credits, spent_credits, streaks, unlock_balances, multiplier_state. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `wallet_store.rs` | ✅ GREEN |
| FR-STATE-002 | Penalty state: escalation_tier, bypass_budget, lockout_windows, debt_balance, strict_mode_until. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `sqlite_adapter.rs`, `sqlite_adapter.rs`, `penalty_store.rs` | ✅ GREEN |
| FR-STATE-003 | Mutations append-only via `WalletMutation` / `PenaltyMutation`. | `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-STATE-004 | All state mutations are immutably recorded in audit log; mutations can be replayed from audit. | `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `lib.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `sqlite_audit_store.rs`, `audit_store.rs`, `migrations.rs`, `lib.rs`, `lib.rs`, `lib.rs` | ✅ GREEN |
| FR-SYNC-001 | Cross-device sync via connector event and state replication. | `cloudkit_port.rs`, `cloudkit_port.rs`, `cloudkit_port.rs`, `cloudkit_port.rs`, `cloudkit_port.rs`, `cloudkit_port.rs`, `CoreHolderE2ETests.swift`, `EndToEndLoopTests.swift` | ✅ GREEN |
| FR-TAB-001 | Core tab navigation and routing. | `CoreTabsSnapshotTests.swift` | ✅ GREEN |
| FR-UI-001 | Design system components and visual language. | `DesignSystemSnapshotTests.swift` | ✅ GREEN |
| FR-UX-001 | Rule firing shows explanation inline. |  | ❌ MISSING |
| FR-UX-002 | Connector auth flow is platform-native (SFSafariViewController / Custom Tabs). |  | ❌ MISSING |
| FR-UX-003 | Penalty escalation shows tier + bypass cost before commit. |  | ❌ MISSING |
| FR-UX-004 | Streak state is visible on home surface. |  | ❌ MISSING |

## Orphan Tests

Tests that reference non-existent FRs:

- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-lang/src/bulk.rs**: FR-BULK-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/audit.rs**: FR-TEL-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/audit.rs**: FR-TEL-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/audit.rs**: FR-TEL-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/audit.rs**: FR-TEL-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/audit.rs**: FR-TEL-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/lib.rs**: FR-TEL-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/pii_scrubber.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/pii_scrubber.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/pii_scrubber.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/pii_scrubber.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/pii_scrubber.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/pii_scrubber.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-telemetry/src/pii_scrubber.rs**: FR-TEL-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/examples/slack-reference/src/lib.rs**: FR-PLUGIN-014
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/examples/slack-reference/src/lib.rs**: FR-PLUGIN-015
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/examples/hello-connector/src/lib.rs**: FR-PLUGIN-012
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/runtime.rs**: FR-PLUGIN-007
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/runtime.rs**: FR-PLUGIN-008
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/runtime.rs**: FR-PLUGIN-009
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/capabilities/http.rs**: FR-PLUGIN-010
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/capabilities/http.rs**: FR-PLUGIN-010
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/capabilities/http.rs**: FR-PLUGIN-011
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/capabilities/http.rs**: FR-PLUGIN-011
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/capabilities/http.rs**: FR-PLUGIN-011
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/capabilities/http.rs**: FR-PLUGIN-012
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/capabilities/http.rs**: FR-PLUGIN-013
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/lib.rs**: FR-PLUGIN-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/lib.rs**: FR-PLUGIN-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/lib.rs**: FR-PLUGIN-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/lib.rs**: FR-PLUGIN-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/signing.rs**: FR-PLUGIN-010
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/signing.rs**: FR-PLUGIN-011
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/plugin.rs**: FR-PLUGIN-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/src/plugin.rs**: FR-PLUGIN-006
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/spans.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/privacy_filter.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/privacy_filter.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/privacy_filter.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/privacy_filter.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/privacy_filter.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/privacy_filter.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/privacy_filter.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/lib.rs**: FR-OBS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/lib.rs**: FR-OBS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/lib.rs**: FR-OBS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/lib.rs**: FR-OBS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/lib.rs**: FR-OBS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/lib.rs**: FR-OBS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/metrics.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-observability/src/integration_tests.rs**: FR-OBS-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-006
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-005
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-006
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/websocket_tests.rs**: FR-MCP-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-mcp-server/tests/http_sse_tests.rs**: FR-MCP-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-rule-suggester/src/lib.rs**: FR-SUGGEST-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-rule-suggester/src/lib.rs**: FR-SUGGEST-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-rule-suggester/src/lib.rs**: FR-SUGGEST-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-rule-suggester/src/lib.rs**: FR-SUGGEST-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-replay/src/lib.rs**: FR-REPLAY-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-replay/src/lib.rs**: FR-REPLAY-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-replay/src/lib.rs**: FR-REPLAY-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-replay/src/lib.rs**: FR-REPLAY-003
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-replay/src/lib.rs**: FR-REPLAY-004
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointIntegrationTests/CoreHolderE2ETests.swift**: FR-DEMO-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointAppSnapshotTests/ResumeOnboardingSnapshotTests.swift**: FR-ONBOARD-002
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Tests/FocalPointAppSnapshotTests/KeyboardShortcutsTests.swift**: FR-KB-001
- **/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/apps/ios/FocalPoint/Sources/FocalPointApp/Settings/SettingsView.swift**: FR-TEL-002
