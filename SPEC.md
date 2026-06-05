# FocalPoint — Specification

> **Spec status:** `implemented` — this document reflects the current codebase, not aspirational design.
> Last audited against tree: `adea91bc62` (2026-05-06).

## 1. What

### 1.1 Purpose

FocalPoint is a **connector-first screen-time management platform** with native iOS enforcement
built on a portable Rust core. It combines behavioral data ingestion (Canvas LMS, Google Calendar,
GitHub, fitness trackers) with a rules engine, reward/penalty ledger, and AI coaching to help
users (primarily parents managing children's screen habits) build structured digital routines.

The platform's differentiating bet is **connectors as first-class behavioral inputs**: rather than
treating screen-time as a black-box, FocalPoint ingests structured signals from productivity,
education, and health platforms to make enforcement context-aware.

### 1.2 Users

| Persona | Primary need |
|---------|-------------|
| **Parent (primary)** | Enforce screen-time policies on child's iOS device, track compliance, manage rewards/penalties |
| **Individual (future)** | Self-directed behavioral coaching with calendar-synced focus sessions |

### 1.3 Scope boundaries

**In scope:**
- Rules engine with DSL, cooldowns, schedule triggers, state-change triggers, priority conflict resolution
- Connector runtime with OAuth2, polling, and webhook ingestion pipelines
- Reward wallet (credits, streaks, multipliers) and penalty ledger (lockout tiers, rigidity)
- Hash-chained audit chain with tamper-evident verification
- iOS app shell with FamilyControls enforcement (pending Apple entitlement)
- SwiftUI rule authoring wizard, mascot (Coachy) UI, onboarding flow
- CLI (`focus-cli`) for exploration and automation
- Multi-agent orchestration tooling (agent-orchestrator, bench-guard, target-pruner, disk-check)
- MCP server for AI tool integration
- Release tooling (release-cut, commit-msg-check, doc-link-check, sbom-gen)

**Out of scope (explicitly deferred):**
- Android native app (JNI stubs exist; no runtime)
- Backend services beyond webhook-ingest placeholder (sync-api, auth-broker)
- Full production OAuth flows for GCal and GitHub (scaffolded only)
- External security audit

### 1.4 MVP Traceability IDs

| ID | Type | User story / requirement | MVP evidence |
|---|---|---|---|
| FR-CONNECTOR-001 | FR | As a parent, I can ingest structured connector signals so policy decisions use real context. | Connector crate tests plus connector journey manifest. |
| FR-RULES-001 | FR | As a parent, I can define rule conditions and schedules so screen-time policy is explicit. | Rules crate tests plus CLI rule journey. |
| FR-REWARDS-001 | FR | As a parent, I can inspect rewards and penalties so behavior feedback is transparent. | Rewards/penalties tests plus ledger docs. |
| NFR-AUDIT-001 | NFR | Audit records remain tamper-evident and verifiable. | Audit chain tests and traceability gate. |
| NFR-JOURNEY-001 | NFR | User-facing flows carry honest evidence, with blank stubs when capture is blocked. | Journey manifest with `NEEDS_CAPTURE` and `blind_eval: "skip"`. |
| NFR-GATE-001 | NFR | Tests, lint, docs, and journey manifests act as an autograder for MVP readiness. | `traceability-gate` plus cargo/clippy/fmt commands. |

---

## 2. How

### 2.1 System architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         SwiftUI iOS App                                 │
│  FamilyControls (ManagedSettings / DeviceActivity)  ←  enforced blocks  │
│  Coachy mascot (SwiftUI + Rive animation)                               │
│  Rule authoring wizard (4-step: When/If/Then/Settings)                │
└───────────────────────────────┬─────────────────────────────────────────┘
                                │ UniFFI FFI
┌───────────────────────────────▼────────────────────────────────────────┐
│                          Rust Core (54 crates)                         │
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌────────────┐ │
│  │ focus-rules  │  │focus-rewards │  │focus-penalties│ │focus-audit │ │
│  │ DSL, engine, │  │wallet, streaks│  │lockout tiers │  │hash chain │ │
│  │ cooldowns    │  │multipliers   │  │rigidity      │  │tamper-evid│ │
│  └──────────────┘  └──────────────┘  └──────────────┘  └────────────┘ │
│                                                                          │
│  ┌──────────────────────────────┐  ┌──────────────────────────────┐    │
│  │     focus-connectors         │  │      focus-sync              │    │
│  │  trait + registry + webhook  │  │  multi-device sync store      │    │
│  │  8 connectors: Canvas,       │  │  SQLite + optional PostgreSQL │    │
│  │  GCal, GitHub, Fitbit,       │  │                              │    │
│  │  Strava, Readwise, Notion,    │  │                              │    │
│  │  Linear                      │  │                              │    │
│  └──────────────────────────────┘  └──────────────────────────────┘    │
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────────┐  │
│  │focus-events  │  │focus-domain  │  │    focus-coaching            │  │
│  │Normalize,    │  │Rigidity,     │  │  LLM explanation rendering,   │  │
│  │dedupe, chain │  │entities,     │  │  natural-language rule        │  │
│  │              │  │value objects │  │  authoring via CoachingProvider │ │
│  └──────────────┘  └──────────────┘  └──────────────────────────────┘  │
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                    Tooling Crates (tooling/)                      │  │
│  │  quality-gate, disk-check, bench-guard, target-pruner,           │  │
│  │  agent-orchestrator, release-cut, fr-coverage,                    │  │
│  │  commit-msg-check, doc-link-check, sbom-gen                        │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────────────┘

External integrations:
  Canvas LMS API    ← connector-canvas (OAuth2, 4 event types, wiremock tests)
  Google Calendar   ← connector-gcal (OAuth2, EventKit on iOS)
  GitHub API        ← connector-github (OAuth2, event mapping stub)
  Fitbit / Strava   ← connector-fitbit / connector-strava (OAuth2 stubs)
  Readwise / Notion / Linear ← connector-* stubs
  MCP servers       ← focus-mcp-server (MCP SDK, type defined, transport pending)
```

### 2.2 Rules engine (focus-rules)

The rules engine is the central decision-making component. A `Rule` has:
- **Trigger**: `Event(String)` (event type name), `Schedule(String)` (cron 6-field), `StateChange(String)` (dotted JSON path)
- **Conditions**: 11 built-in condition kinds — `confidence_gte`, `payload_eq`, `payload_in`, `payload_gte`, `payload_lte`, `payload_exists`, `payload_matches`, `source_eq`, `occurred_within`, `all_of`, `any_of`, `not`
- **Actions**: `GrantCredit`, `DeductCredit`, `Block` (with `Rigidity`), `Unblock`, `StreakIncrement`, `StreakReset`, `Notify`, `EmergencyExit` (break-glass bypass), `Intervention` (with severity), `ScheduledUnlockWindow`
- **Priority**: integer, resolved at conflict time; higher wins
- **Cooldown**: `Duration`, deduplicates rapid re-fires
- **Explanation template**: static or LLM-rendered, consumed by iOS UI

`RuleEngine::evaluate()` is deterministic given (rule, event, cooldown state, now).
`RuleEngine::evaluate_all()` sorts by priority descending and returns all decisions.

LLM integration:
- `propose_rule_from_nl()` — natural-language → `Rule` JSON via `CoachingProvider`
- `render_llm_explanation()` — static template fallback on error

### 2.3 Connector runtime (focus-connectors)

`Connector` trait (async):
- `manifest()` — `ConnectorManifest` (id, version, auth, sync mode, capabilities, entity/event types, verification tier)
- `health()` — `HealthState` (Healthy / Degraded / Unauthenticated / Failing)
- `sync(cursor)` — `SyncOutcome` (events, next cursor, partial flag)

**Verification tiers**: `Official` > `Verified` > `MCPBridged` > `Private`

**Auth strategies**: `OAuth2 { scopes }`, `ApiKey`, `DeviceBrokered`, `None`

**Sync modes**: `Polling { cadence_seconds }`, `Webhook`, `Hybrid`

**ConnectorRegistry** — marketplace catalog for the connector picker UI. Grouped by tier, sorted by (tier, display_order, id).

**WebhookRegistry** — maps connector ids to `WebhookHandler` implementations. Handlers verify signatures before trusting payloads. Dispatches to `WebhookDelivery → Vec<NormalizedEvent>`.

Connectors shipped in-tree:
- `connector-canvas` — Canvas LMS (OAuth2, 4 event types, 44 wiremock tests)
- `connector-gcal` — Google Calendar (OAuth2 scaffold)
- `connector-github` — GitHub (OAuth2 scaffold, event mapping stub)
- `connector-fitbit`, `connector-strava` — fitness (OAuth2 stubs)
- `connector-readwise`, `connector-notion`, `connector-linear` — event mapping stubs only
- `connector-testkit` — test harness

### 2.4 Reward/penalty ledgers

**RewardWallet** (`focus-rewards`):
- Fields: `earned_credits`, `spent_credits`, `streaks: HashMap<name, Streak>`, `unlock_balances`, `multiplier_state`
- Mutations: `GrantCredit`, `SpendCredit`, `StreakIncrement`, `StreakReset`, `SetMultiplier`
- Invariants: balance >= 0, spent <= earned, multiplier >= 0 (NaN rejected)
- Every successful mutation records `wallet.<variant>` audit line
- Failed mutations (insufficient credit, negative amount) write no audit

**PenaltyLedger** (`focus-penalties`):
- Lockout tiers with `Rigidity`: `Hard` (cannot bypass), `Semi` (warning + grace), `Soft` (notification only)
- Bypass budget tracking, escalation state machine
- Traces to: FR-STATE-001..005, FR-PEN-001..004

### 2.5 Audit chain (focus-audit)

Hash-chained tamper-evident log. Each record contains:
- Sequential index
- Timestamp (UTC)
- Record type string
- Actor ID
- JSON payload
- SHA-256 hash of (prev_hash + index + timestamp + type + actor + payload)

On startup, chain is verified by re-computing hashes. Any mismatch = tamper detected.

### 2.6 Coaching provider (focus-coaching)

`CoachingProvider` async trait with `complete()` returning `Option<String>`:
- `StubCoachingProvider` — returns hardcoded single response (testing)
- `NoopCoachingProvider` — always returns `None` (silent fallthrough)
- Real provider (production): routes through a configured LLM endpoint

Used for:
1. Natural-language rule authoring (`propose_rule_from_nl`)
2. Dynamic explanation rendering (`render_llm_explanation`)
3. Rituals: Morning Brief schedule derivation, Evening Shutdown task classification

### 2.7 Tooling (tooling/)

| Tool | Purpose |
|------|---------|
| `quality-gate` | Aggregates fmt/clippy/test/doc/deny/fr-coverage/build checks; exits 1 on first failure; `--quick` skips slow checks |
| `disk-check` | Pre-dispatch disk space gate: exit 0 if >=30GB free, exit 2 if 10–30GB (warn), exit 1 if <10GB (block) |
| `bench-guard` | Tracks benchmark regressions across commits; blocks PRs on performance cliff |
| `target-pruner` | Prunes `target/` dirs in worktrees to reclaim disk |
| `agent-orchestrator` | Pre-dispatch disk check + spawns subagents with per-agent output files |
| `release-cut` | Version bump planner + executor for multi-crate workspace releases |
| `commit-msg-check` | Validates conventional commit format |
| `doc-link-check` | Crawls markdown files, verifies links |
| `fr-coverage` | Maps FR-XXX codes in source to test coverage |
| `sbom-gen` | Generates CycloneDX SBOM from Cargo.lock |

---

## 3. Interface

### 3.1 Rust crate API (primary)

```rust
// Rules engine
use focus_rules::{RuleEngine, Rule, Action, Trigger, RuleDecision};
let mut engine = RuleEngine::new();
let decision = engine.evaluate(&rule, &event, Utc::now());
match decision {
    RuleDecision::Fired(actions) => { /* apply each Action */ }
    RuleDecision::Suppressed { reason } => { /* cooldown, skip */ }
    RuleDecision::Skipped { reason } => { /* trigger mismatch, condition failed, disabled */ }
}

// Connector registry
use focus_connectors::{ConnectorRegistry, ConnectorListing, ConnectorManifest};
let registry = ConnectorRegistry::new();
registry.register(listing);
let catalog = registry.catalog(); // sorted by tier then display_order

// Reward wallet
use focus_rewards::{RewardWallet, WalletMutation, Credit};
let mut wallet = RewardWallet::default();
wallet.apply(WalletMutation::GrantCredit(Credit { amount: 100, .. }), Utc::now(), &audit_sink)?;

// Audit chain
use focus_audit::{AuditChain, AuditSink};
let chain = AuditChain::new()?;
chain.verify()?; // panics on tamper

// Coaching
use focus_coaching::{CoachingProvider, StubCoachingProvider};
let provider = StubCoachingProvider::single("{\"name\":\"Test\"}".into());
let rule = propose_rule_from_nl("give 5 credits per task completion", &provider).await?;
```

### 3.2 FFI (UniFFI)

`focus-ffi` exports the core Rust types via UniFFI. iOS consumes via generated Swift bindings.
Android JNI stubs exist in `focus-ffi` but no Kotlin runtime integration yet.

### 3.3 CLI (focus-cli)

```bash
focus demo seed --db=/tmp/focus.db       # populate demo data
focus tasks list --db=/tmp/focus.db --json
focus rules list --db=/tmp/focus.db
focus wallet show --db=/tmp/focus.db
focus audit verify --db=/tmp/focus.db
focus sync run --db=/tmp/focus.db
focus eval event --db=/tmp/focus.db --event-type=TaskCompleted
focus templates list
focus release cut --dry-run
```

### 3.4 MCP server

`focus-mcp-server` exposes FocalPoint as a Model Context Protocol tool:
- Tool: list connectors
- Tool: trigger rule evaluation
- Tool: query wallet balance
- Tool: dispatch sync

Status: type-defined, transport pending (RFC-0001).

---

## 4. Status

### 4.1 Compilation

**Workspace does not fully compile.** 5 crates have E-series errors:

| Crate | Error | Cause |
|-------|-------|-------|
| `focus-backup` | E0505 | Borrow-check failure in backup operation |
| `focus-rituals` | E0277 | Missing `Eq` impl on `f32` |
| `connector-gcal` | type error | OAuth2 flow incompletion |
| `connector-github` | type error | Event mapping incompletion |
| `connector-canvas` | type error | Sync cursor handling |

See `docs/reference/honest_coverage.md` for details.

### 4.2 Feature matrix

| Domain | Status | Key files |
|--------|--------|-----------|
| Rules engine | SHIPPED | `crates/focus-rules/src/lib.rs` |
| Connector runtime | SHIPPED | `crates/focus-connectors/src/lib.rs` |
| Reward wallet | SHIPPED | `crates/focus-rewards/src/lib.rs` |
| Penalty ledger | SHIPPED | `crates/focus-penalties/src/lib.rs` |
| Audit chain | SHIPPED | `crates/focus-audit/src/lib.rs` |
| Events | SHIPPED | `crates/focus-events/src/lib.rs` |
| Sync | PARTIAL | `crates/focus-sync` (scaffolded) |
| Coaching / LLM | PARTIAL | `crates/focus-coaching` (trait defined) |
| Calendar integration | PARTIAL | `crates/focus-calendar` (trait + mock) |
| Rituals | PARTIAL | `crates/focus-rituals` (E0277 blocking) |
| Backup/restore | SCAFFOLD | `crates/focus-backup` (E0505 blocking) |
| MCP server | SCAFFOLD | `crates/focus-mcp-server` (transport pending) |

**iOS app:**
- SwiftUI shell compiles (5 tabs: Home, Tasks, Rules, Activity, Settings)
- Rule authoring wizard shipped (4-step)
- Canvas OAuth shipped
- GCal/GitHub OAuth scaffolded
- FamilyControls behind `#if FOCALPOINT_HAS_FAMILYCONTROLS` flag (awaiting Apple entitlement)
- Coachy mascot: SwiftUI render shipped, `.riv` Rive animation pending designer

### 4.3 Test coverage

- ~80 unit tests pass when workspace compiles
- 44 Canvas wiremock integration tests
- Ritual integration tests (15)
- Sync cursor persistence tests
- Connector trait contract tests
- Wallet invariant tests

### 4.4 CI

| Check | Status |
|-------|--------|
| Clippy lint | Green (when workspace compiles) |
| cargo fmt | Green |
| Vale markdown | Green |
| commit-msg validator | Green |
| FR coverage mapping | Shipped (fr-coverage tool) |
| cargo deny | Configured, deny.toml present |
| SBOM generation | Shipped (sbom-gen tool) |

---

## 5. TODO

### 5.1 Must-fix before any release

- [ ] **Fix 5 E-series compilation errors** (E0505, E0277, 3× type errors) — blocks all testing
- [ ] **Merge FamilyControls entitlement** — Apple review SLA is 1–4 weeks
- [ ] **Complete GCal and GitHub OAuth flows** — scaffolded but non-functional
- [ ] **Onboarding UX** — zero screens shipped; users cannot self-serve setup

### 5.2 Should-fix for production quality

- [ ] **Real-device QA** — currently simulator-only
- [ ] **Coachy Rive animation** — designer asset pending
- [ ] **Backup/restore iOS FFI** — E0505 borrow-check blocks iOS integration
- [ ] **MCP transport** — type definitions done, transport layer not started

### 5.3 Would-nice

- [ ] Android native app (JNI bindings exist, no Kotlin runtime)
- [ ] Backend services (auth-broker, sync-api) currently placeholders only
- [ ] External security audit
- [ ] Production LLM endpoint for coaching provider

### 5.4 Stack hygiene

- [ ] External dependency audit (see `deny.toml`)
- [ ] Feature requirement trace coverage: FR-CONN-004 (Canvas OAuth2 cursor sync) is `unimplemented!()`
- [ ] `tooling/fr-coverage` and `tooling/doc-link-check` binaries not built by default

---

## References

- `Cargo.toml` — workspace membership, MSRV (1.82), shared dependencies
- `deny.toml` — cargo-deny security advisories config
- `rust-toolchain.toml` — nightly channel pin
- `crates/focus-rules/src/lib.rs` — rule engine implementation + 60+ tests
- `crates/focus-connectors/src/lib.rs` — connector trait, registry, webhook registry + 15+ tests
- `crates/focus-rewards/src/lib.rs` — wallet aggregate + 15+ tests
- `tooling/quality-gate/src/main.rs` — quality gate aggregator
- `tooling/disk-check/src/main.rs` — disk space gate
- `FUNCTIONAL_REQUIREMENTS.md` — FR-CONN/EVT/RULE/STATE/ENF/DATA/UX traceability matrix
- `docs/roadmap_v2.md` — 6-phase roadmap with effort estimates
- `docs/reference/honest_coverage.md` — shipped vs scaffold vs partial vs blocked audit
