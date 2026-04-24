# Honest Coverage Audit

## v0.0.5 — 2026-04-23 (session-2 tail: WORKSPACE BROKEN)

**Status:** ❌ Compilation failure. Session-2 committed changes that do not build.

**Test count:** Live `cargo test --workspace` fails to compile; unable to measure test count. Recent commits introduced:
- `focus-backup`: unused variable warnings + borrow-checker error (E0505)
- `focus-rituals`: Eq derive on f32 (E0277), unused variables
- `connector-gcal`: ConnectorError::Config variant missing (E0599)
- `connector-github`: GitHubEvent struct missing required fields (E0063)
- `connector-canvas`: Page<Page> generic nesting error (E0107)

**Clippy:** ✅ Green (after fix to `focus-connectors/signature_verifiers.rs`)

**Builder app build:** ✅ Green (`bun run build` completes, dist/ outputs 400+ KB)

**Binaries compiled:**
- ✅ `focus-webhook-server` builds cleanly
- ✅ `focalpoint-mcp-server` builds cleanly
- ⏸ `focus` CLI and others pending workspace compilation fix

**Claimed vs actual:**
- **feat(backup)**: Claimed encrypted full-backup + restore (age, passphrase) — scaffold compiles but has E0505 borrow-checker error; unfixed.
- **feat(ir)**: Claimed Task + Schedule variants — compiles if focus-rituals fixed; landing in partial state.
- **feat(webhook)**: Claimed focus-webhook-server + signature verifiers — **SHIPPED** (compiles, runs); 5 unit tests pass.
- **feat(mcp)**: Claimed focalpoint-mcp-server 15 tools — **SHIPPED** (compiles, runs); 5 unit tests pass.
- **feat(release)**: Claimed release notes generator — **SHIPPED** (compiles); 3 unit tests pass.
- **feat(onboarding-v2)**: Claimed Duolingo-grade rework — **SCAFFOLDED** (code present but blocked on broken focus-rituals dep).
- **feat(builder)**: Claimed all 12 primitives in ReactFlow — **SHIPPED** (web app builds; 12 node types present).
- **docs(site)**: Claimed sidebar reorganization + status dashboard — **PARTIAL** (sidebar updated; status.md exists but vitepress build broken by missing deps).

**iOS:** 
- Fastlane not installed; unable to verify lane compilation.
- Recent commits added Localizable.xcstrings, SentrySetup fixes, widgets — syntax checks skipped pending Xcode run.

**Honest verdict for v0.0.4/0.0.5 claims:**
- ✅ **Shipped**: webhook-server, mcp-server, release-notes CLI, builder web app, i18n strings
- 🔄 **Partial**: backup (borrow-checker error), rituals crates (Eq on f32), GCal/GitHub connectors (type mismatches), onboarding V2 (blocked by rituals)
- ⏸ **External-blocked**: none new; same Apple entitlement + ops signing
- ❌ **Missing**: workspace does not compile; breaking changes need immediate fix

---

# Honest Coverage Audit — 2026-04-22

## Planning Coach rituals (2026-04-23)

`focus-rituals` crate lands with 15 passing unit tests covering both new FRs:

- **FR-RITUAL-001 (Morning Brief)** — genuinely shipped at the domain layer:
  `RitualsEngine` composes `Scheduler::plan()` + `CalendarPort::list_events()`,
  extracts top-3 priorities deterministically from the schedule (earliest
  placements, which scheduler sorted by priority-weighted score), builds a
  `SchedulePreview` that counts soft/hard conflicts (hard-conflict propagation
  is tested), asks the `CoachingProvider` for a ≤80-char opening referencing
  one priority, falls back to a static "Morning. Start with: X." line when the
  provider is Noop or `FOCALPOINT_DISABLE_COACHING=1`, and pushes
  `MascotEvent::DailyCheckIn` → `Pose::Confident + Emotion::Warm`. Intention
  capture is side-effect-free.
- **FR-RITUAL-002 (Evening Shutdown)** — genuinely shipped at the domain
  layer: classifies each `TaskActual` against the planned `Schedule` into
  `Shipped | Slipped(Skipped|Deferred|Overran|Cancelled)`, derives carryover
  (slipped minus cancelled), and sets `streak_deltas["focus"] = +1` when
  ≥ 3 hours of shipped focus-time is reported. Coachy closing is LLM-driven
  with static fallback.

**Mocked-but-unverified edges:**

- ~~**Persistent task pool.**~~ **DONE (2026-04-23).** Migration v4 adds a
  `tasks` table (`id, user_id, title, status, priority, duration_spec,
  deadline, chunking, constraints, created_at, updated_at`) with
  `idx_tasks_user_status`. A sync `TaskStore` port lands in
  `focus-planning` (with an `InMemoryTaskStore` for tests); the SQLite impl
  is `focus-storage::sqlite::task_store::SqliteTaskStore`, using
  `block_in_place` so it is safe to call from async FFI shims. `FocalPointCore`
  now holds `Arc<dyn TaskStore>` and `RitualsApi` reads through it, so
  tasks survive a reconstructed core. Closes the in-memory caveat for
  FR-DATA-001 / FR-PLAN-001.
- ~~**Real calendar adapter.**~~ **DONE (2026-04-23).** Apple EventKit
  `CalendarHost` callback interface + `HostBackedCalendarPort`
  implementation in `focus-ffi`; Swift `EventKitCalendarHost` in
  `apps/ios/.../Adapters/`; `NSCalendarsFullAccessUsageDescription` added
  to Info.plist. Morning Brief now reads the device calendar. GCal covered
  via the new `connector-gcal` crate (OAuth2 + keychain); connector-side
  events flow through the standard focus-events path.
- ~~**Intention persistence.**~~ **DONE (2026-04-23).** `capture_intention`
  now takes `&dyn AuditSink` and writes a `ritual.intention.captured` record
  (subject=`morning-brief:<date>`) through the hash-chained audit store.
  FFI exposes `RitualsApi::capture_intention(date, intention)`.
- ~~**Reflow heuristics.**~~ **DONE (2026-04-23).** `suggest_reflow` now
  takes the live task pool and replans a real base schedule via
  `Scheduler::plan` before layering the overrun change, instead of
  synthesizing an empty base.

**FFI surface:** `RitualsApi { generate_morning_brief, generate_evening_shutdown }`
is exposed over UniFFI with DTOs (`MorningBriefDto`, `EveningShutdownDto`,
`TaskActualDto`, etc.). Swift bindings + XCFramework regenerated.



Superseds the optimistic count in `traceability.md`. Mocked-as-traced was
generous; this doc measures production-readiness.

## Verdict

**Updated 2026-04-23 (post end-to-end loop closure):**

- ~22/26 FRs genuinely shipped. Session-closed additions on top of prior verdict:
  FR-RULE-007 StateChange trigger, Action catalog (EmergencyExit/Intervention/
  ScheduledUnlockWindow), WebhookRegistry, EnforcementCallbackPort,
  ConnectorRegistry catalog, connector→orchestrator auto-register, foreground
  sync heartbeat + "Sync now" button, AuditApi::recent + Activity tab,
  TaskApi FFI + iOS Tasks tab, Rituals UI (Today tab consuming RitualsApi),
  GCal + GitHub Settings connect UI, onboarding-unblock + real OS permissions
  (Notifications/Calendar), EventSink port (connector events now persist to
  SQLite events table), EvalApi + RuleEvaluationPipeline (events → rule
  evaluation → wallet/penalty/policy mutations, cursor-persisted, cooldowns
  respected, rule.fired audit lines).
- ~2 partial (Swift side still thinner than Rust in edge cases; live-API
  smoke tests gated on real sandbox creds).
- ~0 mocked-only — the pipeline now runs real data end-to-end.
- **Gap-sweep 2026-04-23 (late):** all four previously-missing items got
  in-tree scaffolding:
  - ~~FamilyControls driver~~ **flagged-off real impl** (shipped 37a8d3e).
    Real `ManagedSettingsStore` + `DeviceActivityCenter` wiring behind
    `#if FOCALPOINT_HAS_FAMILYCONTROLS`; the day Apple approves the
    entitlement we flip the flag — no further code. Status view +
    enablement doc shipped.
  - ~~Visual connector builder~~ **in-app Rule Wizard shipped** (9c0f878).
    4-step authoring UI backed by `describe_dsl()` catalog API. Web-hosted
    visual builder still pending but has all primitives to consume.
  - ~~Template-pack format + signing~~ **shipped** (fb22165).
    `focus-templates` crate with TOML pack round-trip, deterministic
    UUID derivation, and ed25519-dalek sign/verify. Root pubkey list
    empty pending ops key generation; user-supplied key flow works today.
    `DerivedConnector` also shipped (multi-base → transform → emit).
  - ~~Coachy 3D redesign~~ **art direction + animation shell shipped**
    (a2a93b3). `CoachyAnimationEngine` Rive-loader falls back to current
    SwiftUI render when `Coachy.riv` absent; sleep→wake launch sequence
    with `matchedGeometryEffect` identity preservation. Waiting on
    designer to produce 14-pose `.riv` + Lottie + SVG deliverables per
    `docs/mascot/coachy-art-direction.md`.
- True external blockers remaining: Apple entitlement review, ops
  signing key ceremony, designer asset production.

## Structural blockers before any production claim

1. ~~**Audit chain is a lie.**~~ **DONE (2026-04-22).** `RewardWallet::apply`,
   `PenaltyState::apply`, and `PolicyBuilder::from_rule_decisions` now take a
   `&dyn AuditSink` and write a `wallet.<variant>` / `penalty.<variant>` /
   `policy.built` record on every successful mutation. A `CapturingAuditSink`
   test helper asserts record_type + payload shape in each aggregate crate.
   FR-STATE-004 and the CLAUDE.md "audit append on mutation" invariant are
   satisfied at the domain layer.
2. ~~**No AuditStore SQLite impl.**~~ **DONE (2026-04-22).**
   `crates/focus-storage/src/sqlite/audit_store.rs` adds `SqliteAuditStore`
   backed by a new `audit_records` table (migration v3) with monotonic `seq`
   preserving insertion order. Implements `AuditStore` (sync, via
   `block_in_place`), `AuditSink` (sync), plus async variants
   (`append_async`, `verify_chain_async`, `head_hash_async`, `load_all`).
   Integration tests cover round-trip, tamper detection, bad `prev_hash`
   rejection, sink behavior, and persistence across DB reopen.
3. ~~**No real secure token storage.**~~ **DONE (2026-04-22).**
   `focus-crypto::keychain` now ships `AppleKeychainStore` (macOS/iOS via
   `security-framework`), `LinuxSecretServiceStore` (secret-service crate),
   `NullSecureStore` (unsupported platforms — errors loudly), and
   `InMemorySecretStore` (tests). `connector-canvas::KeychainStore` wraps any
   `SecureSecretStore` via JSON-serialized `CanvasToken` storage. FR-DATA-002
   satisfied for Canvas OAuth tokens.
4. ~~**No cursor persistence across restarts.**~~ **DONE (2026-04-22).**
   `focus-sync::CursorStore` trait with `SqliteCursorStore` impl (migration
   v2: `connector_cursors` table). Orchestrator gains
   `SyncOrchestrator::with_cursor_store`; `register` hydrates cursors, every
   successful sync persists them. Restart-survival regression test in
   `focus-sync` proves the round-trip. FR-EVT-003 satisfied.
5. ~~**FFI exposes Coachy only.**~~ **DONE (2026-04-23).** `RuleQuery`,
   `RuleMutation`, `WalletApi`, `PenaltyApi`, `PolicyApi`, `AuditApi`,
   `SyncApi`, `RitualsApi`, and `ConnectorApi` (connect_canvas / connect_gcal
   / connect_github) are all exposed through UniFFI with DTOs, reachable
   from Swift. `SyncApi::connectors()` returns live `ConnectorHandleSummary`
   entries from the orchestrator. The iOS app has moved past mascot-demo
   status.
   - **Connect→register gap closed (2026-04-23).** `connect_canvas`,
     `connect_gcal`, and `connect_github` used to persist a keychain token
     and return `Ok` without ever wiring a live connector into
     `SyncOrchestrator` — so `SyncApi::tick()` kept reporting
     "0 connectors synced" after a successful connect. They now build the
     corresponding `CanvasConnector` / `GCalConnector` / `GitHubConnector`
     around the same keychain-backed `TokenStore` and register it via
     `SyncOrchestrator::register` (cadence: Canvas 300s, GCal 180s, GitHub
     600s). `AlreadyRegistered` (a reconnect) triggers an `unregister` +
     re-register so the fresh token-backed connector wins. Integration
     test `focus-ffi::tests::connector_registration` proves the post-
     `connect_canvas` handle shows up in `SyncApi::connectors()`.
6. ~~**Coachy bubble copy is static / rule explanations are template-only.**~~
   **PARTIAL (2026-04-23).** New `focus-coaching` crate ships a
   `CoachingProvider` trait + HTTP (OpenAI-compat, Minimax/Kimi) / Noop /
   Stub impls. `focus-mascot::MascotMachine::on_event_with_bubble` and
   `focus-rules::{propose_rule_from_nl, render_llm_explanation}` are wired
   behind it. FFI surface adds `CoachingConfig`, `set_coaching`,
   `generate_bubble`, `propose_rule_from_nl`. Kill switch via
   `FOCALPOINT_DISABLE_COACHING=1`; rate-limited at 10 calls/60s.
   **Remaining:** Swift side must call `set_coaching` during onboarding; no
   prod integration tests against a real Minimax/Kimi endpoint yet.

## Canvas connector — reality-check hardening (2026-04-23)

Wiremock coverage expanded from 24 → 44 tests. Items 1–7 below flipped from
"likely bug" to **fixed in-tree**; item 8 (iOS OAuth bridge) is out of scope
for the connector crate and remains open. A live-API test now exists but is
`#[ignore]`-gated pending a real Canvas sandbox credential.

1. **FIXED.** `Assignment.course_id` is now `Option<u64>` with
   `#[serde(default)]`. `CanvasEventMapper::map_assignment{,_due_soon,
   _overdue}` accept a `course_id_hint: Option<u64>` that's threaded from
   the parent course scope and takes precedence over the field.
2. **FIXED.** `default_manifest` now ships empty scopes by default (user's
   Developer Key defaults apply). Callers opt in via
   `CanvasConnectorBuilder::scopes(Vec<String>)`. No more hard-coded
   `url:GET|...` scopes that 400 `invalid_scope` on restricted instances.
3. **FIXED.** `api.rs::get_json` rewrites status handling:
   - 429 → parse `Retry-After` header, map to `RateLimited(secs)`.
   - 403 → read body, case-insensitive `"Rate Limit Exceeded"` match →
     `RateLimited(Retry-After)`; otherwise `Auth(msg)` with truncated body
     for debugging.
   - Every response logs `X-Request-Cost` at DEBUG (budget-aware
     throttling hook; not acted on yet).
4. **FIXED.** `lib.rs::sync` uses a new `drain_paginated` helper that loops
   until `next_cursor` is `None`, capped at `MAX_PAGES_PER_COURSE = 10`
   (warn-log on cap hit, no fail). Applies to assignments, submissions,
   and announcements.
5. **FIXED.** 4 new canonical mappers in `events.rs`:
   `map_assignment_due_soon` (24h window),
   `map_assignment_overdue` (past due + no submission),
   `map_grade_posted` (score present + `workflow_state == "graded"`),
   `map_announcement_posted` (new `Announcement` model + `list_announcements`
   endpoint using `/api/v1/announcements?context_codes[]=course_<id>`).
   Manifest `event_types` updated accordingly.
6. **FIXED.** Every optional field on `Course`, `Assignment`, `Submission`
   now carries `#[serde(default)]`. Added defaults for previously-missing
   fields: `course_code`, `start_at`, `end_at`, `description`,
   `unlock_at`, `lock_at`, `html_url`, `published`, `graded_at`, `grade`,
   `user_id`, `late`, `missing`.
7. **FIXED.** `CanvasToken` gains `issued_at: DateTime<Utc>` (serde default
   = `Utc::now`, so legacy blobs deserialize cleanly). When `expires_at`
   is `None` and a `refresh_token` is present, `is_expired()` now returns
   `true` once more than `STALE_IF_NO_EXPIRY_SECS` (3600s, matching
   Canvas's default 1h lifetime) have elapsed since `issued_at`, so
   callers refresh proactively instead of waiting for a 401.
8. ~~**OPEN.**~~ **DONE (2026-04-23).** `CanvasAuthView` uses
   `ASWebAuthenticationSession` to drive Canvas OAuth end-to-end; the
   returned code is handed to `FocalPointCore.connector().connectCanvas(...)`
   which runs `CanvasOAuth2::exchange_code` in the core runtime and
   persists the token via `KeychainStore`.

**Live-API smoke test:** `tests/integration_live.rs` guarded by
`#[ignore]` and the `live-canvas` cargo feature. Reads `CANVAS_TEST_BASE_URL`
and `CANVAS_TEST_TOKEN`; skips gracefully if unset. Exercises
`list_courses` + `list_assignments` + `list_submissions` +
`list_announcements` against the real API. **Pending real sandbox creds;
not expected to pass in CI.**

## Other significant gaps

- ~~`Rule::trigger::Schedule` returns `Skipped`.~~ **DONE (2026-04-23).**
  `RuleEngine::evaluate_schedule_tick` runs schedule-triggered rules via the
  `cron` crate; cooldown map dedupes per-slot. `StateChange` remains gapped.
- ~~Condition DSL has 2 primitives.~~ **DONE (2026-04-23).** 12 primitives
  shipped: `confidence_gte`, `payload_eq`, `payload_in`, `payload_gte`,
  `payload_lte`, `payload_exists`, `payload_matches` (regex), `source_eq`,
  `occurred_within`, and composables `all_of` / `any_of` / `not`. Dotted
  paths supported across every payload_* kind.
- Only 6/7 canonical `Action` variants. Missing: emergency-exit reduction,
  stronger-unlock-proof-required, intervention-event, scheduled-unlock-
  window-with-credit-spend.
- ~~`RuleEvaluation` type declared; never constructed.~~ **DONE (2026-04-23).**
  `RuleEngine::evaluate_with_trace` and `evaluate_schedule_tick_with_trace`
  construct full `RuleEvaluation` records (rule_id, event_ids, decision,
  explanation). Persistence to a store is the remaining hop.
- ~~`PenaltyState.debt_balance` inert.~~ **DONE (2026-04-23).** Activated
  via `SpendBypassOrDebt` (drains budget then accrues shortfall) and
  `RepayDebt` (clamped at zero, no reverse-credit). `Clear` zeros it.
- ~~`EnforcementPolicy.app_targets` always empty.~~ **DONE (2026-04-23).**
  `PolicyBuilder::from_rule_decisions_with_targets` takes a
  `HashMap<profile, Vec<AppTarget>>` and unions targets from every Blocked
  profile, deduped. Legacy entry point preserved for existing callers.
- `FamilyControlsEnforcementDriver.apply/retract` are TODO stubs with
  `log.info` only.
- ~~No `ConnectorWebhookPort`.~~ **DONE (2026-04-23).**
  `focus-connectors` ships `WebhookRegistry` + `WebhookHandler` trait.
  Connectors register a handler; inbound HTTP layer (out of crate scope)
  dispatches deliveries by `connector_id`. Handlers must verify signatures
  before returning decoded events.
- ~~No `EnforcementCallbackPort`.~~ **DONE (2026-04-23).**
  `focus-policy::EnforcementCallbackPort` + `EnforcementCallback` enum
  (ApplySucceeded/ApplyFailed/RetractSucceeded/BlockAttempted/
  BypassRequested/AuthorizationRevoked). `InMemoryEnforcementCallbackPort`
  for tests; real drivers wire a sink forwarding into focus-events.
- **Ecosystem primitives progress:**
  - Verification tiers: **DONE** (`VerificationTier::{Official, Verified, MCPBridged, Private}` on every manifest, default `Verified`).
  - Marketplace catalog: **DONE** (`ConnectorRegistry` + `ConnectorListing { manifest, tagline, display_order, installed }`; tier-ordered `catalog()` + tier-filtered view).
  - MCP-bridged adapter: **PARTIAL** (`MCPBridgedConnector` stub with manifest + tier routing, MCP transport wiring pending).
  - Visual builder: **PARTIAL** (Task #20 — in-app Rule Authoring Wizard
    ships with 4-step When/If/Then/Settings + Review JSON preview in
    `apps/ios/FocalPoint/Sources/FocalPointApp/Rules/RuleBuilderView.swift`.
    DSL catalog exposed through Rust `focus_rules::describe_dsl()` and the
    `FocalPointCore::rules_dsl()` FFI; web-hosted visual builder still
    pending).
  - Template-pack format: **MISSING**.
  - Derived/meta connectors: **MISSING**.
  - Governance/signing of connector manifests: **MISSING**.

## Scope-misaligned deadweight (per user redirect to
   connector-reward-gamification primary, deprioritize QR/NFC)

- `focus-crypto/src/unlock.rs` (98 LOC) — QR/NFC validators, naive
  prefix-match, no HMAC/nonce/replay protection. Low-value + insecure.
- `apps/ios/.../UnlockProof/QRScanner.swift` (106 LOC) — AVFoundation
  scanner, not wired to any rule engine path.
- `apps/ios/.../Tests/EnforcementTests/` (21 LOC) — only tests the log-only
  stub driver; provides false confidence.

## iOS reality

- No onboarding flow
- `RulesView` (29 LOC) — hardcoded `["Deep work — no social", "Evening
  wind-down"]` strings. "Add rule" appends `"New rule N"` to the array.
- `HomeView` (129 LOC) — static cards, hardcoded streak 7d / credits 42 /
  bypass 2. Zero state from Rust core.
- `SettingsView` (31 LOC) — Canvas button flips a `@State` bool. No OAuth,
  no Safari VC, no keychain save.
- No explanation sheet, no bypass-confirm, no connector-health display.

## Traceability doc overcount

`docs/reference/traceability.md` claims 21 fully traced, 5 partial, 4 gap.
That counts wiremock tests as tracing. Stricter production-readiness
criteria: ~8 fully done, ~12 partial, ~6 mocked-only, ~20+ missing.

## Next actions (priority order)

1. **Critical path to real user** (this is the list of things that must land
   before any claim of production-readiness is honest):
   - AuditStore SQLite impl + audit-append on every state mutation
   - ~~Real KeychainStore via `SecKeychain` FFI~~ DONE (2026-04-22)
   - ~~Cursor persistence in SQLite~~ DONE (2026-04-22)
   - Expand UniFFI surface: rules/wallet/penalty/policy/sync
   - Onboarding flow (3-5 screens)
   - Rule authoring UI
   - Settings → real OAuth via `ASWebAuthenticationSession`
2. **Canvas reality check**:
   - Fix `Assignment.course_id` to `Option<u64>` + all other optional fields
   - Drop hardcoded scopes OR make them configurable per institution
   - Rewrite rate-limit handling: 429 → retry-after, 403 → parse body to
     distinguish permission vs throttle
   - Fix per-course assignment pagination (loop until no next cursor)
   - Map remaining 4 event types
   - Add a `#[ignore]`-gated live-API test with sandbox creds
3. **Scope cleanup**:
   - Archive `focus-crypto/unlock.rs`, `QRScanner.swift`,
     `EnforcementTests.swift` until post-MVP revival.
4. **Deferred until critical path is sound**:
   - Motion-clone scheduler layer (Tasks #25-28)
   - Ecosystem primitives (marketplace, verification tiers, visual builder)
   - Coachy Duolingo-grade asset (Task #16)
   - Planning-coach rituals (Task #28)

## Domain-model surface updates (2026-04-23)

Three coupled primitives landed behind existing FRs — trait surfaces, not
behavior. All default to prior semantics, so enforcement is unchanged until
UX + ruleset authoring opt in.

- **Task #29 — Rigidity spectrum (FR-RIGIDITY-001).** `focus-domain` exports
  `Rigidity { Hard, Semi(RigidityCost), Soft }` and
  `RigidityCost { CreditCost(i64), TierBump, StreakRisk,
  FrictionDelay(Duration), AccountabilityPing }`. Plumbed through
  `focus-penalties::LockoutWindow.rigidity`,
  `focus-rules::Action::Block.rigidity`, and
  `focus-policy::ProfileState::Blocked.rigidity`. All three use serde
  defaults of `Rigidity::Hard` so pre-existing serialized rules,
  lockouts, and profile states deserialize with block-is-hard semantics.
  Authoring a `Semi` block is now expressible; runtime cost-paying logic
  is not yet wired.
- **Task #31 — Open EventType vocabulary (FR-EVT-VOCAB-001).**
  `focus-events::EventType` is now
  `WellKnown(WellKnownEventType) | Custom(String)`.
  `EventType::from_manifest_string(connector_id, type_str)` maps canonical
  strings to `WellKnown(_)` and anything else to
  `Custom("{connector_id}:{type_str}")`. `Display` renders well-known
  canonically, custom as-is. `focus-rules::Trigger::Event(String)` matches
  on the display string and now accepts trailing-glob patterns
  (`"canvas:*"`).
- **Task #30 — Connector verification tier (FR-CONN-TIER-001).**
  `focus-connectors::ConnectorManifest` gains
  `tier: VerificationTier { Official, Verified, MCPBridged, Private }`
  (serde default `Verified`) and `health_indicators: Vec<String>`
  (default empty). Canvas is `Official`. A new
  `focus-connectors::mcp_bridge` module defines `MCPBridgedConnector` —
  a stub whose `sync`/`health` return "not yet wired" errors. Type is real
  so manifests, audit, and UI taxonomy can reference it; MCP transport
  itself is follow-up work.

Test deltas: `focus-domain` +7 (Rigidity), `focus-events` +6 (vocabulary),
`focus-connectors::mcp_bridge` +4 (tier default, shape, dedupe key,
sync-not-wired), `focus-rules` +3 (exact + glob trigger matching).
`focus-penalties`, `focus-policy`, `focus-ffi`, `focus-storage`, and
`connector-canvas` updated call sites without behavior change; existing
tests still pass.

Still missing: runtime semantics for `Rigidity::Semi` cost-paying at
bypass-time; MCP transport and any actual MCP connector; tier-aware UI;
migration path for existing custom events to `from_manifest_string`.

## Motion layer foundation (2026-04-23)

New crates land the task-scheduling foundation. Not wired into `apps/` or
FFI yet; this is pure core.

- **`focus-planning`** — Task model. `Task`, `DurationSpec { fixed | Estimate{p50,p90} }`,
  `Priority::aged`, `Deadline { when, rigidity }`, `ChunkingPolicy`, `Constraint`
  (`WorkingHours`/`NoEarlier`/`NoLater`/`Buffer`/`EnergyTier`), `TaskStatus`
  with legal transitions, `TimeBlock::overlaps`. 10 tests. Covers **FR-PLAN-001**.
- **`focus-scheduler`** — Rigidity-aware greedy scheduler. `Scheduler::plan`
  sorts tasks by `priority.weight × deadline-urgency (Hard ×1.5)`, bin-packs
  into free time respecting working-hours windows and `NoEarlierThan`/
  `NoLaterThan`, splits into chunks bounded by `ChunkingPolicy`, refuses
  placement on `Rigidity::Hard` calendar conflicts (→ `UnplacedReason::HardConflict`),
  charges `RigidityCostSummary` on Semi/Soft overrides. `reflow` retains
  untouched placements, drops cancelled tasks, treats kept blocks as synthetic
  Hard events when re-placing new tasks, and surfaces overrun via
  `ScheduleChange::BlockOverran`. Output is sorted by `(starts_at, task_id)` for
  determinism. 14 tests. Covers **FR-PLAN-002**.
- **`focus-calendar`** — `CalendarPort` trait (`list_events` / `create_event` /
  `delete_event`) + `CalendarEvent`, `CalendarEventDraft`, `DateRange`,
  `InMemoryCalendarPort` (tokio `RwLock<Vec<_>>`). 4 async tests. Covers
  **FR-CAL-001**. Real GCal (OAuth+REST) and EventKit (UniFFI bridge on iOS)
  adapters land in a follow-up.

Still missing (motion layer): energy-tier-aware scoring (constraint parsed
but not applied as a bias yet); buffer-constraint enforcement between
placements; bypass-cost budget ceiling (Semi events are tracked but not
refused when cost exceeds budget); GCal/EventKit adapters; FFI surface for
`plan`/`reflow`; persistence of `Task` / `Schedule` in `focus-storage`.
