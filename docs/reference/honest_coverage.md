# Honest Coverage Audit — 2026-04-22

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

## Canvas connector = 0% real

24 wiremock tests, zero live calls. Likely bugs on real API:

1. `Assignment.course_id: u64` is required; Canvas omits it on
   `/courses/:id/assignments` responses → deserialize fails.
2. Hardcoded scopes (`"url:GET|/api/v1/courses"`) will 400 `invalid_scope`
   on instances that haven't enabled those specific scopes on the
   Developer Key. Canvas scopes are optional + per-instance.
3. 403 conflated with rate-limit. Canvas uses 403 for permission denied
   and 429 for throttling; current code classifies all 403s as rate-limit
   → masks real permission errors.
4. Per-course assignment pagination silently truncated to first page
   (`lib.rs:197` passes `None` cursor).
5. Only 3/7 canonical events mapped. Missing: `grade_posted`,
   `announcement_posted`, `assignment_due_soon`, `assignment_overdue`.
6. No `#[serde(default)]` on optional Canvas fields → brittle to schema drift.
7. Token expiry with `expires_at: None` → `is_expired` forever false → no
   proactive refresh.
8. No iOS OAuth bridge (`ASWebAuthenticationSession`) — all Rust-side OAuth
   sophistication is unreachable from the app.

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
