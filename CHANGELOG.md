# Changelog

## 0.0.5 — 2026-04-23 (session-2 tail: honest coverage rollup)

**Status:** Workspace compilation broken by recent commits. See `docs/reference/honest_coverage.md#v005—2026-04-23` for breakdown.

### Shipping-Ready
- `focus-webhook-server` crate: Axum HTTP server, per-provider signature verifiers (GitHub HMAC-SHA256, Canvas JWT stub, GCal token), GitHub event handler. 5 tests. **SHIPPED**.
- `focalpoint-mcp-server` crate: 15 MCP tools (8 read, 7 write), async STDIO transport via mcp-sdk 0.0.3. 5 tests. **SHIPPED**.
- `focus release notes` CLI: Generates markdown/Discord/TestFlight release notes from git log. Groups by conventional commit. Works offline; optional LLM synthesis. 3 tests. **SHIPPED**.
- `apps/builder` web app: ReactFlow node builder with 12 primitives (Task, Schedule, Connector, etc.) in 6 categorized palettes. **SHIPPED** (vite build 400KB, 210 modules).
- `Localizable.xcstrings`: 122 user-visible string entries (en source). Extracted from all Text/Label/Section/accessibility labels. Ready for translation partnerships. **SHIPPED**.

### Partial / Scaffolded
- `focus-backup` crate: Tar+zstd+age encryption pipeline, BackupManifest, create_backup/restore_backup — has **E0505 borrow-checker error**; unfixed.
- `focus-rituals` crate: Morning Brief + Evening Shutdown routing — has **E0277 (Eq on f32)** in weekly.rs; unfixed.
- `connector-gcal` crate: Google Calendar OAuth2 + REST — has **E0599 (ConnectorError::Config missing)**; unfixed.
- `connector-github` crate: PAT-based contributions — has **E0063 (GitHubEvent missing required fields)**; unfixed.
- `connector-canvas` crate: Canvas OAuth — has **E0107 (Page<Page> generic nesting)**; unfixed.
- `feat(onboarding-v2)`: Mascot-first Duolingo-grade rework (6 pages, matched-geometry transitions, SFX/haptics) — **blocked by focus-rituals compilation failure**.
- `docs-site` build: Sidebar reorganization, home page rewrite, status dashboard — **vitepress compilation broken**; temp cache deleted but deps fail.

### External Blockers (unchanged from v0.0.4)
- Apple FamilyControls entitlement review pending
- Ops ed25519 root pubkey ceremony not run
- Designer assets for Coachy 3D (.riv, Lottie, SVG) not delivered

## 0.0.4 — 2026-04-23 (community feedback loop)

### Added — Release & Community
- `focus release notes` CLI subcommand — generates markdown/Discord/TestFlight release notes from git log, groups by conventional commit type (feat/fix/docs/test/perf/chore/refactor), synthesizes user-facing summaries (LLM-optional via `FOCALPOINT_RELEASE_NOTES_LLM` env var).
- `focus-release-bot` crate — Discord webhook poster; takes release notes JSON, formats embeds, POSTs to webhook URL (supports async and blocking APIs). Webhook URL passed at runtime; never stored.
- Discord launch playbook — `docs-site/community/discord_launch_playbook.md` with recommended channel structure, bot setup, moderation basics, CoC, onboarding templates with Coachy mascot, feedback funnel to GitHub Issues + TestFlight.
- Release loop guide — `docs-site/guides/release_loop.md` walking main → fastlane beta → release notes → webhook post → community feedback triage.
- iOS feedback capture — Settings "Send feedback" row (Support section) triggers mailto:feedback@focalpoint.app prefilled with device info + audit-summary counts.

### Changed — CLI
- `focus` CLI now includes `release-notes` subcommand with `generate --since <tag> --format <md|discord|testflight>` (default: v0.0.3, format: md).
- Release notes work offline (no LLM required); optional synthesis when `FOCALPOINT_RELEASE_NOTES_LLM` is set.

## 0.0.3 — 2026-04-23 (end-to-end loop)

### Added — Rust core
- `focus-eval` crate: `RuleEvaluationPipeline` closes the events → rule → action loop. Cursor-persisted, cooldown-aware, appends `rule.fired` audit lines, dispatches into wallet/penalty/policy.
- `focus-rituals` crate: Morning Brief + Evening Shutdown with LLM-driven Coachy opening/closing and static fallback.
- `focus-planning` / `focus-scheduler` / `focus-calendar`: task model, priority-weighted bin-packing scheduler with rigidity-aware chunking, CalendarPort trait.
- `connector-gcal` (Google Calendar v3 + OAuth2), `connector-github` (PAT-based contributions). Both persist tokens via keychain.
- `focus-sync::EventSink` port: connector events now write to the SQLite events table on every sync (migration v1 dedupe honored).
- `focus-connectors`: `WebhookRegistry` + `WebhookHandler` (push-side counterpart to pull sync), `ConnectorRegistry` catalog with tier-ordered listings.
- `focus-policy`: `EnforcementCallbackPort` for driver→core reporting; `PolicyBuilder::from_rule_decisions_with_targets` populates `app_targets` from a profile→targets registry.
- `focus-rules`: 12 condition primitives (was 2) incl. `all_of`/`any_of`/`not` composables + dotted paths; `Trigger::Schedule` (cron) and `Trigger::StateChange` evaluators; `evaluate_with_trace` family constructs `RuleEvaluation` records; three new Action variants — `EmergencyExit`, `Intervention{severity}`, `ScheduledUnlockWindow`.
- `focus-penalties`: `SpendBypassOrDebt` / `RepayDebt` mutations activate `debt_balance`.
- SQLite migration v4: persistent `tasks` table + `SqliteTaskStore`.

### Added — FFI surface
- `TaskApi` (add/list/remove/mark_done), `EvalApi::tick`, `AuditApi::recent`, `ConnectorApi::connect_gcal` + `connect_github`, `RitualsApi::capture_intention`, `CalendarHost` callback interface (EventKit).
- `SyncApi::connectors()` returns live handles from the orchestrator.

### Added — iOS app
- Today tab — Morning Brief + Evening Shutdown consuming `RitualsApi`; mascot-first layout; per-window "Mark done".
- Tasks tab — full CRUD, priority bar, deadline chips, swipe-to-delete, Coachy empty state.
- Activity tab — live tail of the audit chain (wallet/penalty/policy/connector/task/ritual) with verify-chain button.
- Settings: GCal + GitHub connect flows (ASWebAuthenticationSession / PAT), "Sync now" + "Run rules now" buttons, real connector status from orchestrator.
- Foreground heartbeat (60s) drives `syncTick()` + `evalTick()` so the loop runs whenever the app is active.
- Onboarding: mascot-first pages (Coachy per step), real OS permission prompts for Notifications + Calendar, honest "Pending Apple entitlement" state for FamilyControls, unconditional advance past Finish.
- Mac (Designed for iPad) launch path via `apps/ios/scripts/run-mac.sh`.

### Documentation
- `docs/living-platform/` — design doc for shapeshifting agent-operated app shell, FocalPoint Morning Brief slice proposal, discrete-swap vs continuous-morph reconciliation (Teleport/Blend/Ghost verbs + identity-continuity algorithm), per-element variant gallery + catch-up notifications.
- `docs/reference/honest_coverage.md` — verdict: 22/26 FRs genuinely shipped.

### Status
- End-to-end flow works: onboarding → add task → connect tool → sync → rules evaluate → wallet/penalty mutate → audit records everything → UI surfaces it.
- 4 remaining gaps are external blockers: Apple FamilyControls entitlement, visual connector builder (Task #20), template-pack format + manifest signing, Coachy 3D redesign (Task #16, art-gated).

## 0.0.2 — 2026-04-22 (Q-resolutions)

### Decided (see `docs/research/open_questions.md`)
- Q1: project name final = **FocalPoint**
- Q2: **iOS-only MVP**, single-device. Android deferred beyond Phase 5.
- Q3: QR-only unlock MVP; NFC in Phase 1.5
- Q4: Rust default for services; Zig considered case-by-case; services deferred
- Q5: **Foqos** (MIT, active, 465★) approved for `apps/ios/` donor. Reef deferred with Android.
- Q6: **mascot scaffolded** — new crate `focus-mascot` + `apps/ios/Mascot/` placeholder
- Q7: template marketplace deferred to Phase 3
- Q8: entitlement app non-urgent (no publish till year-end)

### Added
- `crates/focus-mascot/` — `Pose` / `Emotion` / `MascotEvent` / `MascotState` / `MascotDriver` trait / `MascotMachine` stub
- `apps/ios/Mascot/README.md` — iOS Spline renderer placeholder
- Workspace now has 17 crates.

### Changed
- PLAN.md phase graph: iOS-focused; Android marked deferred
- README stack section: SwiftUI-only; cross-native frameworks explicitly rejected per ADR-001

## 0.0.1 — 2026-04-22 (scaffold)

### Added
- Rust workspace with 16 crate stubs (no business logic).
- iOS + Android app directory placeholders.
- Spec docs carved from `ChatGPT-App Architecture for Screen Time.md`:
  PRD, ADR index + 9 ADR files, FUNCTIONAL_REQUIREMENTS, PLAN, USER_JOURNEYS.
- Connector SDK spec stub + ecosystem strategy stub.
- `.gitignore`, `LICENSE` (MIT OR Apache-2.0).

### Status
- No impls — only type names, trait signatures, module boundaries.
- Stack decision locked (ADR-001..ADR-009).
- Open questions tracked in `docs/research/open_questions.md`.
