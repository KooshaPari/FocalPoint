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
- **Real calendar adapter.** `RitualsApi` wires `InMemoryCalendarPort`;
  EventKit / GCal remain stubbed (FR-CAL-001 follow-up).
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

- ~12/26 FRs genuinely shipped (+2 from audit-gap closure: FR-STATE-004 real
  audit-on-mutation, persistent audit chain in SQLite; +2 prior storage-gap
  closures: FR-DATA-002 secure token persistence, FR-EVT-003 cursor
  persistence)
- 10 partial
- 4 mocked-only (tests green, real behavior unverified)
- 17+ missing against "real user could actually use this"

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
5. **FFI exposes Coachy only.** Rules / wallet / penalty / audit / policy /
   sync are all unreachable from Swift. iOS app is a mascot demo harness.
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
8. **OPEN.** iOS OAuth bridge (`ASWebAuthenticationSession`) still missing.
   Tracked separately; not a connector-crate change.

**Live-API smoke test:** `tests/integration_live.rs` guarded by
`#[ignore]` and the `live-canvas` cargo feature. Reads `CANVAS_TEST_BASE_URL`
and `CANVAS_TEST_TOKEN`; skips gracefully if unset. Exercises
`list_courses` + `list_assignments` + `list_submissions` +
`list_announcements` against the real API. **Pending real sandbox creds;
not expected to pass in CI.**

## Other significant gaps

- `Rule::trigger::Schedule` and `StateChange` variants exist but `evaluate()`
  returns `Skipped` for them → only event triggers actually work.
- Condition DSL has 2 primitives (`confidence_gte`, `payload_eq`). Missing:
  time-window, count, aggregation.
- Only 6/7 canonical `Action` variants. Missing: emergency-exit reduction,
  stronger-unlock-proof-required, intervention-event, scheduled-unlock-
  window-with-credit-spend.
- `RuleEvaluation` type declared; never constructed or stored.
- `PenaltyState.debt_balance` field present; no mutation ever touches it.
- `EnforcementPolicy.app_targets` always `vec![]` — builder doesn't wire
  user-selected apps into policy.
- `FamilyControlsEnforcementDriver.apply/retract` are TODO stubs with
  `log.info` only.
- No `ConnectorWebhookPort`, no `EnforcementCallbackPort`.
- No ecosystem primitives: no marketplace, no verification tiers, no visual
  builder, no template-pack format, no MCP-bridged connector adapter, no
  derived/meta connectors, no governance/signing.

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
