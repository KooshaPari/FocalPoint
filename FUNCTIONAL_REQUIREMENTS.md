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

## FR-STATE (State)

- **FR-STATE-001** — Reward wallet: earned_credits, spent_credits, streaks, unlock_balances, multiplier_state.
- **FR-STATE-002** — Penalty state: escalation_tier, bypass_budget, lockout_windows, debt_balance, strict_mode_until.
- **FR-STATE-003** — Mutations append-only via `WalletMutation` / `PenaltyMutation`.

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

## FR-UX

- **FR-UX-001** — Rule firing shows explanation inline.
- **FR-UX-002** — Connector auth flow is platform-native (SFSafariViewController / Custom Tabs).
- **FR-UX-003** — Penalty escalation shows tier + bypass cost before commit.
- **FR-UX-004** — Streak state is visible on home surface.
