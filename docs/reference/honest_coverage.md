# Honest Coverage Audit — 2026-04-22

Superseds the optimistic count in `traceability.md`. Mocked-as-traced was
generous; this doc measures production-readiness.

## Verdict

- ~8/26 FRs genuinely shipped
- 12 partial
- 6 mocked-only (tests green, real behavior unverified)
- 20+ missing against "real user could actually use this"

## Five structural blockers before any production claim

1. **Audit chain is a lie.** `RewardWallet::apply`, `PenaltyState::apply`,
   `PolicyBuilder::from_rule_decisions` never call any `AuditStore`. The
   tamper-evident chain has nothing real to verify. FR-STATE-004 + CLAUDE.md
   "audit append on mutation" invariant both violated.
2. **No AuditStore SQLite impl.** `focus-storage/src/sqlite/` has event/rule/
   wallet/penalty stores — no `audit_store.rs`. Audit survives only in RAM.
3. **No real secure token storage.** `KeychainStore` is `Err("not wired")`.
   Tokens in `InMemoryTokenStore` → no restart survival → FR-DATA-002 violated.
4. **No cursor persistence across restarts.** Orchestrator holds cursors in
   `ConnectorHandle` only. Every relaunch re-ingests from scratch; dedupe is
   in-memory too → duplicate events forever.
5. **FFI exposes Coachy only.** Rules / wallet / penalty / audit / policy /
   sync are all unreachable from Swift. iOS app is a mascot demo harness.

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
   - Real KeychainStore via `SecKeychain` FFI
   - Cursor persistence in SQLite
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
