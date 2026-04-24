# FocalPoint — Functional Requirements

> Source: arch doc lines 829–878. Full prose + rationale there; this is the
> indexed list for traceability.

## FR-CONN (Connectors)

- **FR-CONN-001** — Connector implements the `Connector` trait (`manifest`, `health`, `sync(cursor)`).
- **FR-CONN-002** — Manifest declares auth_strategy, sync_mode, capabilities, entity_types, event_types.
- **FR-CONN-003** — Connector emits `NormalizedEvent`s with dedupe_key; same event ingested twice → exactly one persisted record.
- **FR-CONN-004** — Canvas connector supports OAuth2 code flow + cursor-based assignment/course sync.
- **FR-CONN-005** — Connector health transitions observable via `HealthState`.

## FR-EVT (Events)

- **FR-EVT-001** — Every event has `event_id`, `connector_id`, `account_id`, `event_type`, `occurred_at`, `effective_at`, `dedupe_key`, `confidence`, `payload`.
- **FR-EVT-002** — Dedupe by `dedupe_key` across restarts.
- **FR-EVT-003** — Cursor progress persisted per (connector_account, entity_type).

## FR-RULE (Rules)

- **FR-RULE-001** — Rule with trigger + conditions + actions + cooldown + explanation_template.
- **FR-RULE-002** — Rule evaluation is deterministic given (rule, event, state_snapshot).
- **FR-RULE-003** — Cooldown prevents re-firing within window.
- **FR-RULE-004** — Each evaluation produces a `RuleEvaluation` record with explanation.
- **FR-RULE-005** — Priority resolves contradicting rule actions (higher priority wins).
- **FR-RULE-006** — Rule DSL supports temporal conditions and scheduler integration.
- **FR-RULE-007** — Rule builder API enables fluent construction and validation.
- **FR-RULE-008** — Rule action explainability: each action has rationale text.

## FR-STATE (State)

- **FR-STATE-001** — Reward wallet: earned_credits, spent_credits, streaks, unlock_balances, multiplier_state.
- **FR-STATE-002** — Penalty state: escalation_tier, bypass_budget, lockout_windows, debt_balance, strict_mode_until.
- **FR-STATE-003** — Mutations append-only via `WalletMutation` / `PenaltyMutation`.
- **FR-STATE-004** — All state mutations are immutably recorded in audit log; mutations can be replayed from audit.

## FR-ENF (Enforcement)

- **FR-ENF-001** — Enforcement policy generated from active rule decisions.
- **FR-ENF-002** — iOS driver applies policy via FamilyControls + ManagedSettings.
- **FR-ENF-003** — Android driver applies policy via UsageStats + AccessibilityService.
- **FR-ENF-004** — Policy activation/deactivation is audited.
- **FR-ENF-005** — Bypass budget spend requires user confirmation.
- **FR-ENF-006** — Unlock proof (QR/NFC) validates against an `UnlockSession` record.

## FR-DATA (Data)

- **FR-DATA-001** — SQLite storage with migrations in `focus-storage::sqlite`.
- **FR-DATA-002** — All state mutations append an `AuditRecord`.
- **FR-DATA-003** — `AuditChain::verify_chain()` detects tampering (hash mismatch).

## FR-PLAN (Planning)

- **FR-PLAN-001** — Tasks and goals stored with priority, due_date, completed_at, status enum.
- **FR-PLAN-002** — Task scheduling with temporal triggers (cron-style recurrence, date-specific, duration-based).

## FR-RITUAL (Rituals)

- **FR-RITUAL-001** — Ritual model with name, schedule, checkpoint_window, description, enabled flag.
- **FR-RITUAL-002** — Ritual completion recorded as completion event with timestamp and metadata.
- **FR-RITUAL-003** — Weekly/monthly ritual cadences supported via temporal expressions.
- **FR-RITUAL-004** — Ritual streak tracking: current, longest, last_completion date.

## FR-DOMAIN (Domain & Rigidity)

- **FR-RIGIDITY-001** — Block actions carry rigidity level: Soft (dismissible) vs Hard (non-dismissible).

## FR-ENTITLEMENTS (Subscriptions & Feature Gates)

- **FR-ENTITLEMENTS-001** — Subscription tiers: Free, Plus, Pro, Family with distinct feature gates.
- **FR-ENTITLEMENTS-002** — Tier-specific limits enforced: max_rules, max_tasks, connector_cadence, voice_synthesis.
- **FR-ENTITLEMENTS-003** — Feature gates evaluated consistently across iOS app and Rust backend.

## FR-CONNECTOR (Generic Connector Support)

- **FR-CONNECTOR-001** — Connector trait: manifest, health, sync implementations across all supported providers.

## FR-POLICY (Policy & Enforcement)

- **FR-POLICY-001** — Policy generation from rule decisions, conflict resolution via priority.

## FR-REWARDS (Reward System)

- **FR-REWARDS-001** — Reward wallet mutations, credit earning/spending/streak tracking.

## FR-APPSTORE (App Store & Packaging)

- **FR-APPSTORE-001** — App icon generation and versioning for app store distribution.

## FR-PRIVACY (Privacy & Data)

- **FR-PRIVACY-001** — Data wiping: purge all user events, rules, state, audit logs on request.

## FR-FOCUS (Focus Engine Evaluation)

- **FR-FOCUS-001** — Rule evaluation engine: matches events against active rules.
- **FR-FOCUS-002** — State snapshot incorporation for contextual rule decisions.
- **FR-FOCUS-003** — Asynchronous rule evaluation with retry semantics.
- **FR-FOCUS-004** — Evaluation result caching for performance.
- **FR-FOCUS-005** — Rule conflict resolution during multi-rule firings.
- **FR-FOCUS-006** — Evaluation metrics and observability signals.
- **FR-FOCUS-007** — Rule explanation generation for fired actions.
- **FR-FOCUS-008** — Evaluation determinism guarantees across restarts.

## FR-DIAG (Diagnostics & Observability)

- **FR-DIAG-001** — Sentry integration for crash reporting and error tracking.
- **FR-DIAG-002** — Privacy filters applied to diagnostic telemetry (no PII).

## FR-SYNC (Data Synchronization)

- **FR-SYNC-001** — Cross-device sync via connector event and state replication.

## FR-MASCOT (AI Coach)

- **FR-MASCOT-001** — Mascot UI surfaces personalized coaching messages.
- **FR-MASCOT-002** — Coaching message generation from rule evaluations and streaks.

## FR-ONBOARD (Onboarding)

- **FR-ONBOARD-001** — Consent flow for permissions and tracking.
- **FR-ONBOARDING-005** — Entitlement tier selection during signup.

## FR-ENFORCE (Enforcement UI)

- **FR-ENFORCE-001** — Enforcement policy visual presentation and management.

## FR-TAB (Core Navigation)

- **FR-TAB-001** — Core tab navigation and routing.

## FR-UI (Design System)

- **FR-UI-001** — Design system components and visual language.

## FR-HEALTHKIT (Platform Integration)

- **FR-HEALTHKIT-001** — HealthKit data adapter and health metrics integration.

## FR-UX

- **FR-UX-001** — Rule firing shows explanation inline.
- **FR-UX-002** — Connector auth flow is platform-native (SFSafariViewController / Custom Tabs).
- **FR-UX-003** — Penalty escalation shows tier + bypass cost before commit.
- **FR-UX-004** — Streak state is visible on home surface.
