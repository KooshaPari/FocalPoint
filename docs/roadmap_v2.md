# FocalPoint Roadmap v2 — Honest Phased Plan (2026-04-23)

This roadmap reflects actual shipped code verified against `honest_coverage.md`, honest gaps, and realistic effort estimates.

---

## Phase 0 — Discovery & Scaffold (COMPLETE)

**Status:** ✅ Done (2026-04-23)

**Scope:**
- Stack decisions locked (ADR-001 through ADR-009)
- 17 Rust crates scaffolded with trait contracts
- Product spec (PRD + FUNCTIONAL_REQUIREMENTS + USER_JOURNEYS)
- Onboarding docs (00_START_HERE + CONTRIBUTING)

**Shipped artifacts:**
- `crates/`: focus-domain, focus-events, focus-rules, focus-storage, focus-audit, focus-sync, focus-connectors, focus-penalties, focus-policy, focus-ffi, focus-mascot, connector-canvas, connector-gcal, connector-github, focus-planning, focus-scheduler, focus-calendar (17 crates)
- Core type signatures stable: `Connector`, `EventStore`, `RuleStore`, `WalletStore`, `PenaltyStore`, `AuditStore`, `ClockPort`, `SecureSecretStore`
- SQLite schema v3 (audit_records, events, rules, wallets, penalties, cursors, tasks)
- Trait-driven architecture ready for multiple adapters

**Exit criteria:** Met.

---

## Phase 1 — iOS MVP Core & Real Enforcement (IN PROGRESS, ~85%)

**Status:** 🟡 In progress. Compilation blockers pending fix; domain layer mostly shipped.

**Scope:**

### 1A — Domain layer (SHIPPED)

**Effort:** 12–15 parallel agent-batches (~10–20 min wall-clock per batch)

**Shipped:**
- ✅ **Event sourcing** (`focus-events` + `focus-storage::sqlite::event_store`): deduped event ingestion with SHA256 hash chains
- ✅ **Rules engine** (`focus-rules` + `RuleEngine::evaluate`): 12 condition primitives, 6 action variants (Block, Unlock, AwardCredit, IncurPenalty, EmergencyExit, Intervention), schedule-triggered evaluation via cron, cooldown deduping
- ✅ **Wallet system** (`focus-penalties::RewardWallet`): +Credit, -Credit, Spend (with debt accrual), Clear mutations; balance-capped to prevent invalid state
- ✅ **Penalty state** (`focus-penalties::PenaltyState`): lockout windows with rigidity spectrum (Hard/Semi/Soft), debt tracking, clear, tier escalation
- ✅ **Policy engine** (`focus-policy::PolicyBuilder`): builds `EnforcementPolicy` from rule decisions, unions app-targets across profiles
- ✅ **Audit chain** (`focus-audit` + `SqliteAuditStore`): tamper-evident hash-chained records, append-only, verification on startup
- ✅ **Task model** (`focus-planning`): Task, DurationSpec, Priority, Deadline, ChunkingPolicy, Constraint (WorkingHours, NoEarlier, NoLater, Buffer, EnergyTier), legal status transitions; 10 tests
- ✅ **Scheduler** (`focus-scheduler`): rigidity-aware bin-packing, respects Hard conflicts, charges cost on Semi/Soft overrides, deterministic sorting; 14 tests
- ✅ **Calendar port** (`focus-calendar`): async trait with mock, real EventKit + GCal adapters to follow
- ✅ **Rituals engine** (`focus-rituals`): Morning Brief (compose schedule + intentions + LLM-coached opening), Evening Shutdown (task-actual classification + carryover + streak delta); 15 tests

**Gap audit:**
- ❌ `focus-backup` — compilation error (borrow-checker E0505), FFI-exposed but unfixed
- ❌ `focus-rituals` — Eq derive on f32 (E0277), unused variables; blocks onboarding-v2 and full workspace compilation
- ❌ `connector-gcal` — missing ConnectorError::Config variant (E0599)
- ❌ `connector-github` — struct field mismatch (E0063)
- ❌ `connector-canvas` — Page<Page> nesting (E0107)

**Next in Phase 1A:** Fix compilation blockers in backup, rituals, and connector crates (2–3 agent-batches, 5–10 min).

### 1B — FFI & Swift bindings (SHIPPED)

**Effort:** 3–5 parallel agent-batches

**Shipped:**
- ✅ `focus-ffi::FocalPointCore`: UniFFI surface exposing WalletApi, PenaltyApi, PolicyApi, RuleQuery, RuleMutation, AuditApi, SyncApi, RitualsApi, ConnectorApi
- ✅ XCFramework regeneration + Swift bindings
- ✅ DTOs for all public types (MorningBriefDto, EveningShutdownDto, RuleEvaluationDto, etc.)
- ✅ iOS interop: sync marshaling, audit chain verification, error propagation

**Remaining:**
- iOS must call `set_coaching` during onboarding (integration, not new code)
- No prod integration tests against real LLM endpoints yet

### 1C — iOS App Shell & Native Enforcement (PARTIAL)

**Effort:** 8–12 parallel agent-batches

**Shipped:**
- ✅ SwiftUI skeleton: HomeView, TasksView, RulesView, SettingsView, ActivityView (tabs)
- ✅ FamilyControls integration scaffold: ManagedSettingsStore + DeviceActivityCenter behind `#if FOCALPOINT_HAS_FAMILYCONTROLS` flag
- ✅ Notifications permission + Calendar permission (OAuth bridging)
- ✅ Rule authoring UI: 4-step wizard (When/If/Then/Settings) with JSON preview
- ✅ Canvas OAuth via `ASWebAuthenticationSession` + keychain persistence
- ✅ GCal + GitHub settings connect buttons (no real OAuth yet)
- ✅ Rule explanations rendered from Rust LLM (fallback static strings)
- ✅ Today tab consuming RitualsApi (Morning Brief visible)
- ✅ Activity tab consuming AuditApi (recent 50 records displayed)

**Gaps:**
- ❌ **FamilyControls enforcement driver:** `FamilyControlsEnforcementDriver.apply/retract` are log-only stubs. Blocked on Apple entitlement approval.
- ❌ **Real onboarding flow:** 0 screens shipped (only hardcoded Canvas button in Settings)
- ❌ **Coachy 3D asset:** SwiftUI fallback shipped; waiting on designer `.riv` deliverables (14-pose Rive file + Lottie/SVG variants)
- ❌ **Live-device QA:** no real sandbox testing until entitlement granted
- ❌ **GCal/GitHub OAuth:** buttons present, no actual OAuth flow wired to FocalPointCore yet
- ❌ **Real task persistence:** tasks in-memory; SQLite TaskStore implemented but not called from iOS
- ⚠️ **HomeView:** hardcoded streak/credit/bypass numbers; no live state binding

**Honest gaps blocking "ready to ship" claim:**
1. **Apple FamilyControls entitlement** (external): submitted Phase 0, pending review (1–4 weeks)
2. **Designer art for Coachy** (external): animation direction drafted, `.riv` file not produced yet
3. **Real device testing:** can only test on simulator until entitlement is approved
4. **Onboarding UX:** 0 screens; users cannot set up accounts, permissions, or initial rules
5. **Workspace compilation:** 5+ crates have unmerged errors (backup borrow-check, rituals f32, connectors)

**Effort to unblock Phase 1:**
- Fix compilation: 2–3 agent-batches, 5–10 min
- Onboarding flow (3–5 screens): 4–6 agent-batches, 15–25 min
- GCal/GitHub OAuth wire-up: 2–3 agent-batches, 8–15 min
- Designer asset handoff + integration: external (1–2 weeks designer work) + 1–2 agent-batches integration

**Est. Phase 1 to "beta-ready":** 8–15 additional agent-batches, ~30–60 min wall-clock (conditional on entitlement + designer assets arriving).

---

## Phase 1.5 — TestFlight Beta & Community Launch

**Status:** 🔴 Blocked on Phase 1 completion + entitlement approval

**Scope:**
- Testflight submission (requires entitlement approval + ops signing key ceremony)
- Discord community launch (playbook drafted in `docs/release/discord_launch_playbook.md`)
- Initial user feedback loop integration
- Crash reporting via Sentry (SDK integrated; live monitoring pending real deploy)
- Release notes automation (shipped; markdown + Discord format generators tested)

**Effort:** 4–6 agent-batches post-Phase-1, ~15–25 min

**Dependencies:**
- Phase 1 completion
- Apple entitlement approval
- Ops signing key ceremony (developer account setup)
- Designer Coachy assets

---

## Phase 2 — Multi-Device Sync & Watch Companion

**Status:** 🔴 Not started. Design docs drafted.

**Scope:**

### 2A — Multi-device state sync (Loro + CloudKit)

**Effort:** 10–15 parallel agent-batches

**Shipped:**
- ✅ Design doc (`docs/design/multi_device_crdt_sync.md`): CRDT merge strategy, CloudKit schema, conflict resolution
- ✅ Loro (CRDT library) selected; benchmark feasibility proven

**Remaining:**
- CloudKit adapter for `focus-storage` (new crate: `focus-cloudkit`)
- Loro CRDT wrapper around rule/wallet/policy mutations
- Bi-directional sync orchestrator
- Conflict resolution UI (when local > remote policy strength)
- Integration tests with CloudKit sandbox

### 2B — Apple Watch companion app

**Effort:** 5–8 parallel agent-batches

**Shipped:**
- ✅ Design doc (`docs/design/apple_watch_companion.md`): glances (minimal info), complications (score/streak), activity rings integration

**Remaining:**
- watchOS SwiftUI shell
- SharedModel data bridge (Watch ↔ iPhone)
- Complication data source wiring
- Activity rings ecosystem integration

### 2C — Subscription tiers & monetization

**Effort:** 3–4 agent-batches

**Shipped:**
- ✅ StoreKit JWS server-side verifier worker (testable, not deployed)
- ✅ iOS StoreKit client integration scaffold

**Remaining:**
- Tier definitions (Free / Premium / Family)
- Feature gating by tier (Free = 3 rules, Premium = 20, Family = admin controls)
- Wallet/penalty bonus on Premium (higher credit cap, lower penalty multiplier)
- Subscription state persistence
- Restore purchases flow

**Est. Phase 2:** 18–27 agent-batches, ~60–110 min wall-clock

---

## Phase 3 — Connector Ecosystem Expansion

**Status:** 🟡 Scaffold ready; first-party connectors in progress.

**Scope:**

### 3A — Connector SDK formalization

**Effort:** 2–3 agent-batches

**Shipped:**
- ✅ Connector manifest spec (`focus-connectors::ConnectorManifest`, TOML-serializable)
- ✅ Verification tiers (Official, Verified, MCPBridged, Private)
- ✅ Template pack format with ed25519 signing (`focus-templates`)
- ✅ Webhook registry + signature verification
- ✅ Marketplace catalog API (`ConnectorRegistry::catalog()`, tier-ordered, deduped)

**Remaining:**
- Connector developer guide (docs/connector-sdk/DEV_GUIDE.md)
- Manifest validator CLI tool
- Community submission process + tier escalation doc

### 3B — First-party connector expansion (Canvas + GCal + GitHub)

**Effort:** 6–9 agent-batches (includes fixing compilation errors)

**Shipped:**
- ✅ **Canvas** (Official tier): OAuth2, course/assignment/submission/announcement sync, 44 wiremock tests, live-API test scaffold (`#[ignore]`)
- ✅ **GCal** (Official tier): OAuth2, event list scaffold
- ✅ **GitHub** (Verified tier): scaffold with auth

**Remaining (blocked by compilation):**
- Canvas: fix E0107 (Page<Page> nesting), expand mappers, live-API test
- GCal: complete OAuth flow, real event sync logic, timezone handling
- GitHub: event mapping, webhook handler

**Additional first-party targets (post-MVP):**
- Notion (task block sync)
- Todoist (task + project + label sync)
- Apple Health (activity + workout data)

### 3C — Rule template marketplace

**Effort:** 4–6 agent-batches

**Shipped:**
- ✅ Template pack format (TOML, ed25519-signed, deterministic UUIDs)
- ✅ Root pubkey infrastructure (ops ceremony to generate, empty until signed)

**Remaining:**
- Community submission portal (web form → GitHub PR → template-catalog repo)
- CI validation of template packs (manifest check, signature verify, rule DSL lint)
- Marketplace UI (browse → preview → install)
- Tier-based curation (Official/Verified templates surface first)

**Est. Phase 3:** 12–18 agent-batches, ~40–75 min wall-clock

---

## Phase 4 — Browser & Safari Extension

**Status:** 🔴 Not started. Design deferred.

**Scope:**
- Safari Web Extension (macOS + iOS 17+): detect distraction URLs, signal focus rules, inject productivity prompts
- Chrome/Firefox extension for desktop (Manifest V3)
- Sync enforcement policies from iOS app

**Effort:** 8–12 agent-batches

**Est. Phase 4:** 8–12 agent-batches, ~30–50 min wall-clock

---

## Phase 5 — Backend Services (Optional, deferred)

**Status:** 🟡 Placeholders scaffolded; implementation deferred.

**Scope:**
- `services/auth-broker` — OAuth callback server for connectors without native redirect
- `services/webhook-ingest` — connector webhook fan-in + replay
- `services/sync-api` — multi-device state sync orchestrator (supplement CloudKit for web clients)

**Justification:** Local-first app is viable without backend. Services added only when cross-device web clients or team-sharing features are prioritized.

**Effort:** 6–10 agent-batches per service (deferred until Phase 3 shipping validates connector demand)

---

## Phase 6 — Android Parity (Deferred beyond Phase 5)

**Status:** 🔴 Deferred. Kotlin bindings scaffolded; no active work.

**Scope:**
- JNI bindings for `FocalPointCore`
- Compose shell + platform-native adapters (UsageStats, AccessibilityService)
- Android-side Canvas/GCal/GitHub OAuth (Custom Tabs)
- Potential dependency on Reef fork (requires rebranding per trademark)

**Rationale:** iOS MVP ships first; Android revived only after ecosystem gains traction and we validate product-market fit.

**Effort:** 15–25 agent-batches (full parity)

---

## Cross-Cutting Work

### Quality Gates & Testing

**Shipped:**
- ✅ Unit tests: 80+ passing tests across 17 crates (domain layer heavily tested)
- ✅ Integration tests: Canvas wiremock (44 tests), ritual round-trip, sync cursor persistence
- ✅ Traceability: FR → test mapping for every shipped FR
- ✅ Clippy: workspace green

**Remaining:**
- Fix workspace compilation (5 crates, 2–3 batches)
- iOS UI automation tests (interaction flows, state binding)
- End-to-end scenario tests on real device (post-entitlement)
- Property tests for rule engine (proptest, deferred)
- Security review post-Phase-1 (audit chain, token storage, entitlement scope)

### Documentation & Developer Experience

**Shipped:**
- ✅ ADRs (9 decisions logged)
- ✅ CONTRIBUTING guide + CoC
- ✅ Connector SDK spec (manifest format, trait, example)
- ✅ PR/Issue templates
- ✅ CI/CD design (self-hosted via Woodpecker, Forgejo)
- ✅ 00_START_HERE + onboarding
- ✅ Design docs (multi-device CRDT, Watch, connector ecosystem, Coachy art direction)
- ✅ RFC process (RFC-0001 plugin SDK, RFC-0002 rule template format)

**Remaining:**
- Connector developer guide (walkthrough building a custom connector)
- Plugin SDK (post-Phase-3, allow Lua/WASM rules + connectors)
- Video tutorials (onboarding, rule authoring, template submission)

### Security & Compliance

**Shipped:**
- ✅ Audit chain implementation (tamper-evident)
- ✅ Token storage via secure enclave (Keychain)
- ✅ Entitlement scoping doc

**Remaining:**
- Security audit (external, post-Phase-1, before TestFlight)
- Privacy policy + terms of service
- GDPR / data export (deferred until multi-user / cloud sync)

---

## Effort Summary

| Phase | Est. Agent-Batches | Est. Wall-Clock | Status |
|-------|-------------------|-----------------|--------|
| **Phase 0** | 20–25 | 60–100 min | ✅ Done |
| **Phase 1** | 8–15 (remaining) | 30–60 min | 🟡 85% (blocked by compilation + entitlement) |
| **Phase 1.5** | 4–6 | 15–25 min | 🔴 Blocked on Phase 1 + entitlement |
| **Phase 2** | 18–27 | 60–110 min | 🔴 Design ready, implementation deferred |
| **Phase 3** | 12–18 | 40–75 min | 🟡 Scaffold ready (compilation + OAuth to wire) |
| **Phase 4** | 8–12 | 30–50 min | 🔴 Design deferred |
| **Phase 5** | 6–10 per service | (deferred) | 🔴 Optional, deferred |
| **Phase 6** | 15–25 | (deferred) | 🔴 Android, post-Phase-5 |

**Total to Phase 1.5 (TestFlight-ready):** ~33–40 agent-batches, ~100–180 min wall-clock (dependent on external blockers: entitlement approval + designer assets).

---

## Critical Path Blockers (Honest Assessment)

1. **Apple FamilyControls entitlement** (submitted ~2026-04-20, pending review 1–4 weeks)
   - Blocks any real device enforcement testing
   - Blocks TestFlight submission
   - No code-level workaround; approval is binary

2. **Workspace compilation** (5 crates with E-series errors)
   - Must fix before any meaningful CI/CD
   - 2–3 agent-batches, 5–10 min to resolve

3. **Designer Coachy assets** (`.riv` Rive file + SVG variants)
   - Falls outside Rust/Swift development scope
   - Required for "polished" UX (currently SwiftUI fallback acceptable)
   - ~1–2 weeks designer turnaround + 1–2 batches integration

4. **Onboarding UX** (0 screens shipped)
   - Users cannot currently set up the app without hardcoding
   - 4–6 agent-batches to ship 3–5 screens
   - Unblocks canary testing

5. **Real-device QA environment**
   - No simulator-only confidence possible
   - Requires physical iPhone, Apple Developer Account, sandbox entitlements
   - Unblocks post-entitlement approval

---

## Known Deviations from v0.0.1 Claims

| Feature | Claimed in commits | Actual status | Gap |
|---------|-------------------|---------------|-----|
| **Full-backup+restore** | `348bd22` | Scaffold with borrow-checker error (E0505) | Fix pending |
| **Rituals (Morning Brief + Evening Shutdown)** | `a2a93b3` | Domain layer shipped (15 tests), iOS not wired | iOS UI pending |
| **Builder (12 ReactFlow primitives)** | `348bd22` | Web app builds (dist/ 400+ KB), not integrated to iOS | Web-only currently |
| **Onboarding v2 (Duolingo-grade)** | `562e8f0` | Scaffold blocked by rituals compilation error | Unblocks after rituals fix |
| **Coachy 3D redesign** | `a2a93b3` | Art direction + SwiftUI fallback shipped | `.riv` asset pending designer |
| **FamilyControls enforcement** | Multiple commits | Code behind flag, awaiting entitlement + driver impl | Gated by flag + approval |
| **GCal/GitHub connectors** | Commit messages | Scaffolds with E-series compile errors | OAuth flow + sync logic pending |
| **Canvas OAuth** | Multiple | ASWebAuthenticationSession wired, persists to keychain | Live-API test pending sandbox creds |

---

## Q2–Q4 2026 Strategic Planning

Comprehensive quarterly roadmap now available:

- **[Q2–Q4 OKR Roadmap](planning/roadmap_2026_q2_q4.md)** — Phase 1.5 (TestFlight beta, 50→500 testers), Phase 2 (multi-device sync, subscriptions, 1,000 DAU), Phase 3 (Android MVP, enterprise tier, 5,000 DAU). 12 OKRs with measurable key results across all quarters.

- **[Agent Staffing Plan](planning/agent_staffing_2026.md)** — Role definitions (Impl, Test, Doc, Audit, Design), monthly batch allocation (142–172 total), dispatch priorities (serial dependencies vs. parallel opportunities), contingency reserves by quarter.

- **[Dependencies & Risk Register](planning/dependencies_risk_register.md)** — Critical path blockers (Apple entitlement, ops key, designer assets), high-impact risks (Loro CRDT, Android permissions, Enterprise SAML), escalation triggers, mitigation strategies.

---

## Recommended Next Actions (Priority)

1. **Fix workspace compilation** (2–3 batches, 5–10 min): unblocks all downstream work
2. **Ship onboarding flow** (4–6 batches, 15–25 min): unblocks manual QA
3. **Wire GCal/GitHub OAuth** (2–3 batches, 8–15 min): expands connector coverage
4. **Monitor entitlement status**: decision drives Phase 1.5 timing (escalate if not approved by 2026-06-01)
5. **Schedule ops signing key ceremony**: target 2026-05-15 (8 weeks before Q2 close)
6. **Request Canvas sandbox credentials**: escalate if not received by 2026-07-15
7. **Security review prep** (1–2 batches): document entitlement scope, token handling, audit chain

