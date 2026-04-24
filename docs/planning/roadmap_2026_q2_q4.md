# FocalPoint Q2–Q4 2026 Roadmap with OKRs

**Planning Wave:** 2026-04-24
**Planning Horizon:** Q2–Q4 2026 (Apr–Dec)
**Format:** Quarterly phases with OKRs, KRs, and agent-batch effort estimates

---

## Q2 2026 (Apr–Jun) — Phase 1.5: TestFlight Beta & Community Launch

**Primary OKR:** Ship Phase 1 completion and launch TestFlight beta with 50→500 testers by end of June.

### OKR 1: Fix Workspace & Ship Phase 1 Core
- **KR1.1:** Compilation blockers resolved (5 crates fixed, zero warnings via `cargo clippy`)
- **KR1.2:** Onboarding flow shipped (3–5 screens, user can create account + grant permissions)
- **KR1.3:** GCal/GitHub OAuth wired end-to-end (live sync demonstrated)
- **KR1.4:** iOS app reaches 80% feature parity with Rust core

**Deliverables:**
- Fix `focus-backup` borrow-checker error (E0505) — 1–2 agent-batches, ~5 min
- Fix `focus-rituals` Eq derive on f32 + unused vars — 1 agent-batch, ~3 min
- Fix `connector-gcal`, `connector-github`, `connector-canvas` compilation errors — 2–3 agent-batches, ~8 min
- Ship onboarding flow (3–5 screens: Welcome, Permissions, Rules Intro, Coachy Setup, Dashboard) — 4–6 agent-batches, ~20 min
- Wire GCal/GitHub OAuth callbacks into `FocalPointCore` — 2–3 agent-batches, ~10 min
- **Subtotal:** 10–15 agent-batches, ~46 min wall-clock

**Critical External Dependencies:**
- Apple FamilyControls entitlement approval (submitted ~2026-04-20, pending 1–4 weeks)
- Coachy `.riv` asset delivery (designer, ~1 week turnaround + 1 agent-batch integration)
- Ops signing key ceremony (one-time, 2 batches)

### OKR 2: TestFlight Submission & Canary Testing (50 testers)
- **KR2.1:** TestFlight submission accepted (binary signed + sandboxed)
- **KR2.2:** First 50 testers onboard and report no critical blockers
- **KR2.3:** Crash rate <2% (via Sentry, 7-day rolling window)

**Deliverables:**
- Ops signing key ceremony + code-sign iOS binary — 1 agent-batch, ~10 min
- Provision TestFlight sandbox certificates — 1 agent-batch, ~5 min
- Create release notes (automated generator tested in Phase 1) — 1 agent-batch, ~3 min
- Deploy Sentry monitoring + verify first 50 crash reports ingested — 1 agent-batch, ~5 min
- **Subtotal:** 4 agent-batches, ~23 min wall-clock

### OKR 3: Discord Community Launch (50→500 testers)
- **KR3.1:** Discord server launched with 5+ channels (announcements, feedback, bugs, rules-gallery, connectors)
- **KR3.2:** 500 tester signups collected via GitHub form + Discord invite link
- **KR3.3:** Community-authored rule templates reach 20+ submissions (verified tier candidates)

**Deliverables:**
- Set up Discord server + moderation roles — 1 agent-batch, ~8 min
- Launch GitHub Discussions form + auto-invite to Discord — 1 agent-batch, ~5 min
- Create community rules showcase (markdown gallery, verify tier escalation process) — 1 agent-batch, ~8 min
- Publish 3 first-party rule template packs (focus-deep-work, student-exam-prep, evening-wind-down) — 2 agent-batches, ~10 min
- **Subtotal:** 5 agent-batches, ~31 min wall-clock

### OKR 4: Designer Asset Integration & UX Polish
- **KR4.1:** Coachy `.riv` file integrated and all 14 poses animated
- **KR4.2:** Onboarding flow polished (animations, error states, accessibility)
- **KR4.3:** Community feedback loop: 3+ design improvements shipped based on tester input

**Deliverables:**
- Integrate Coachy `.riv` animation asset + test all pose transitions — 1–2 agent-batches, ~8 min
- Polish onboarding screens (micro-interactions, error handling, voice guidance) — 2 agent-batches, ~10 min
- **Subtotal:** 3–4 agent-batches, ~18 min wall-clock

**Q2 Total: 22–28 agent-batches (~138 min wall-clock)**

---

## Q3 2026 (Jul–Sep) — Phase 2: Multi-Device Sync GA & Subscription Tiers

**Primary OKR:** Achieve multi-device state sync (GA), launch subscription tiers, hit 1,000 DAU + first revenue.

### OKR 1: Multi-Device CRDT Sync (Loro + CloudKit)
- **KR1.1:** CloudKit adapter shipped and tested (bi-directional sync, conflict resolution)
- **KR1.2:** Loro CRDT wrapper integrated for rule/wallet/policy mutations
- **KR1.3:** End-to-end scenario: create rule on iPhone, see it on iPad/Mac within 30s
- **KR1.4:** Offline sync queue survives app kill; queued mutations replay on reconnect

**Deliverables:**
- Implement `focus-cloudkit` adapter (CloudKit record mapping + bidirectional sync orchestrator) — 6–8 agent-batches, ~30 min
- Wire Loro CRDT wrapper into rule/wallet/policy mutations — 4–6 agent-batches, ~20 min
- Build conflict resolution UI (highlight local→remote policy strength divergence, offer merge UI) — 3–4 agent-batches, ~15 min
- Integration tests with CloudKit sandbox (3+ scenarios) — 2 agent-batches, ~10 min
- **Subtotal:** 15–20 agent-batches, ~75 min wall-clock

### OKR 2: Apple Watch Companion App
- **KR2.1:** watchOS shell ships with Glances + Complications
- **KR2.2:** Watch-to-iPhone data sync (score, streak, next focus block)
- **KR2.3:** Native Activity rings integration (daily focus time contributed to rings)

**Deliverables:**
- Build watchOS SwiftUI shell (Home, Shortcuts, Settings tabs) — 2–3 agent-batches, ~12 min
- Implement SharedModel bridge (Watch ↔ iPhone via CloudKit) — 2–3 agent-batches, ~12 min
- Wire Complication data sources (score, streak widgets) — 1–2 agent-batches, ~8 min
- Activity rings ecosystem integration — 1 agent-batch, ~5 min
- **Subtotal:** 6–9 agent-batches, ~37 min wall-clock

### OKR 3: Subscription Tiers & Monetization (StoreKit)
- **KR3.1:** Three tier definitions live (Free, Premium, Family)
- **KR3.2:** Feature gating by tier operational (Free: 3 rules, Premium: 20, Family: admin controls)
- **KR3.3:** First $500 MRR recurring revenue locked in by end of Q3

**Deliverables:**
- Define tier features + pricing (Free, Premium $4.99/mo, Family $7.99/mo) — 1 agent-batch, ~5 min
- Implement tier state persistence (SQLite + CloudKit sync) — 2 agent-batches, ~10 min
- Wire StoreKit client → FFI → Rust tier checks (pre-existing server-side verifier reused) — 2–3 agent-batches, ~12 min
- Build Tier Management UI (upgrade prompts, restore purchases, subscription status) — 2 agent-batches, ~10 min
- Premium bonus modeling (higher credit cap +50%, penalty multiplier −25%) — 1 agent-batch, ~5 min
- **Subtotal:** 8–10 agent-batches, ~42 min wall-clock

### OKR 4: Connector Ecosystem Expansion (→10 total)
- **KR4.1:** Canvas, GCal, GitHub, Notion, Todoist, Health connectors shipping
- **KR4.2:** Marketplace catalog UI live (browse, tier-filter, install templates)
- **KR4.3:** 100+ community rules published (via template submission flow)

**Deliverables:**
- Complete Canvas connector (live-API tests + error recovery) — 2 agent-batches, ~10 min
- Finalize GCal connector (OAuth flow + recurring event support) — 2 agent-batches, ~10 min
- Finalize GitHub connector (event webhooks + activity mapping) — 2 agent-batches, ~10 min
- Notion connector (page + database task sync) — 3 agent-batches, ~15 min
- Todoist connector (project + task + label sync) — 3 agent-batches, ~15 min
- Apple Health connector (workout + activity sync) — 2 agent-batches, ~10 min
- Marketplace UI + install flow — 3 agent-batches, ~15 min
- Community submission CI (manifest validate, sign, tier-escalate) — 2 agent-batches, ~10 min
- **Subtotal:** 19–20 agent-batches, ~95 min wall-clock

### OKR 5: Growth Metrics & User Feedback Loop
- **KR5.1:** TestFlight expands 50→500 testers; Discord hits 300 members
- **KR5.2:** DAU reaches 500; retention (D7) stays >60%
- **KR5.3:** Community rules library hits 50+ packs (all tiers)

**Deliverables:**
- Build community feedback dashboard (sentiment analysis on Discord/GitHub) — 1 agent-batch, ~5 min
- Weekly cohort analysis (feature adoption, churn by cohort) — 1 agent-batch, ~5 min

**Q3 Total: 58–68 agent-batches (~339 min wall-clock)**

---

## Q4 2026 (Oct–Dec) — Phase 3: Android Alpha & Enterprise Tier

**Primary OKR:** Ship Android alpha (parity with iOS Phase 1), hit 5,000 DAU, close first enterprise contract.

### OKR 1: Android MVP (Phase 1 Parity)
- **KR1.1:** JNI bindings for `FocalPointCore` compile cleanly
- **KR1.2:** Compose shell ships with Home, Rules, Activity tabs
- **KR1.3:** Canvas + GCal connectors working on Android via standard OAuth (Custom Tabs)
- **KR1.4:** Milestone: first 100 internal Android testers with zero critical crashes

**Deliverables:**
- Implement JNI bindings for `FocalPointCore` (FFI→JNI layer) — 4 agent-batches, ~20 min
- Build Compose app shell (Home, Rules, Activity, Settings) with Material 3 — 5–6 agent-batches, ~25 min
- Adapt Canvas OAuth for Custom Tabs + keychain-equivalent token storage — 2 agent-batches, ~10 min
- Adapt GCal OAuth for Custom Tabs + token persistence — 2 agent-batches, ~10 min
- Platform-native adapters (UsageStats, AccessibilityService) — 3–4 agent-batches, ~18 min
- Integration testing (3 scenarios, real device) — 2 agent-batches, ~10 min
- **Subtotal:** 18–21 agent-batches, ~93 min wall-clock

### OKR 2: Enterprise/Education Tier & Admin Console
- **KR2.1:** Enterprise tier defined (100+ users, SSO, audit export, custom branding)
- **KR2.2:** Admin console shipped (user management, rule templates, audit trails, billing)
- **KR2.3:** First education institution contract signed (K-12 or university pilot)

**Deliverables:**
- Design enterprise tier SLAs + pricing — 1 agent-batch, ~5 min
- Implement SSO (SAML 2.0 or OpenID Connect) — 3–4 agent-batches, ~18 min
- Build admin console (Svelte + Tauri or web app) — 4–6 agent-batches, ~25 min
- Audit export (CSV + JSON formats, tamper-proof) — 2 agent-batches, ~10 min
- Multi-tenant SQLite schema updates — 2 agent-batches, ~10 min
- **Subtotal:** 12–16 agent-batches, ~68 min wall-clock

### OKR 3: Browser Extension (Safari + Chrome)
- **KR3.1:** Safari Web Extension ships (macOS + iOS 17+)
- **KR3.2:** Chrome/Firefox extension (Manifest V3) stable
- **KR3.3:** Extension blocks distraction URLs, signals focus rules, suggests productivity breaks

**Deliverables:**
- Scaffold Safari Web Extension project + messaging protocol — 1 agent-batch, ~5 min
- Implement distraction URL detection + rule signaling — 2–3 agent-batches, ~12 min
- Wire productivity prompts (in-page injection) — 2 agent-batches, ~10 min
- Chrome/Firefox port (Manifest V3 + content script) — 2 agent-batches, ~10 min
- Sync policy enforcement from iOS/Android — 1 agent-batch, ~5 min
- **Subtotal:** 8–10 agent-batches, ~42 min wall-clock

### OKR 4: Plugin SDK & Ecosystem Expansion
- **KR4.1:** Plugin SDK (Lua + WASM) shipped and documented
- **KR4.2:** First community plugin published (custom rule condition or action)
- **KR4.3:** Plugin marketplace reaches 5+ submissions

**Deliverables:**
- Design plugin architecture (contracts, isolation, security) — 1 agent-batch, ~5 min
- Implement Lua + WASM plugin runtime — 4–5 agent-batches, ~22 min
- Plugin SDK documentation + examples (3 sample plugins) — 2 agent-batches, ~10 min
- Community submission flow + verification tier — 1 agent-batch, ~5 min
- **Subtotal:** 8–9 agent-batches, ~42 min wall-clock

### OKR 5: Performance & Scale
- **KR5.1:** App launch time <2s (iOS + Android)
- **KR5.2:** SQLite database <50 MB under heavy load
- **KR5.3:** Sync latency <500ms on Wi-Fi, <2s on LTE

**Deliverables:**
- Benchmark + profile iOS/Android startup — 1 agent-batch, ~5 min
- Optimize SQLite queries (indices, batch operations) — 2 agent-batches, ~10 min
- CRDT sync optimization (lazy loading, compression) — 2 agent-batches, ~10 min
- Load testing (1000 rules, 10k task archive, 50 concurrent users) — 2 agent-batches, ~10 min
- **Subtotal:** 7 agent-batches, ~35 min wall-clock

### OKR 6: Marketing & Monetization
- **KR6.1:** 5,000 DAU by EOQ (from 500 in Q3)
- **KR6.2:** $3,000+ MRR (from $500 at Q3 close)
- **KR6.3:** NPS >40 (community feedback + survey)

**Deliverables:**
- Launch marketing site (VitePress + SEO) — 2 agent-batches, ~10 min
- Product Hunt / Hacker News launch threads — 1 agent-batch, ~5 min
- NPS survey + feedback loop — 1 agent-batch, ~5 min

**Q4 Total: 62–76 agent-batches (~363 min wall-clock)**

---

## Cross-Quarter Efforts (Ongoing)

### Quality & Testing
- Fix workspace compilation blockers (Q2 start) — 3–5 agent-batches
- Property-based tests for rule engine (proptest, deferred to Q4) — 3 agent-batches
- Security audit (Q2 post-Phase-1, external partner) — external
- iOS UI automation tests (Xcode UI tests, Q3) — 4–6 agent-batches
- End-to-end scenario tests (real device, post-entitlement) — 3–4 agent-batches

### Documentation & Developer Experience
- Connector developer guide (DEV_GUIDE.md, Q2) — 2 agent-batches
- Plugin SDK guide + examples (Q4) — 2 agent-batches
- Video tutorials (onboarding, rule authoring, connector submission, Q3-Q4) — 4–6 agent-batches
- RFC process evolution (plugin SDK, template distribution governance, Q4) — 2 agent-batches

### Security & Compliance
- Privacy policy + terms of service (Q2 pre-TestFlight) — 1 agent-batch
- GDPR data export (Q3, deferred if no EU users) — 2 agent-batches
- Post-launch security review (Q3, external audit firm) — external
- Entitlement scope documentation (Q2) — 1 agent-batch

---

## Timeline Summary

| Quarter | Agent-Batches | Wall-Clock | Phase | Key Milestone |
|---------|----------------|-----------|-------|---------------|
| **Q2** | 22–28 | ~138 min | 1.5 | TestFlight beta launch, 50→500 testers, Discord community |
| **Q3** | 58–68 | ~339 min | 2 | Multi-device sync GA, subscription tiers live, 1,000 DAU |
| **Q4** | 62–76 | ~363 min | 3 | Android MVP, enterprise tier, 5,000 DAU, first contracts |
| **Total** | **142–172** | **~840 min** | 1.5+2+3 | Ecosystem maturity, multi-platform, first revenue |

---

## Risk Register (Summary)

**Critical Path Blockers:**
1. Apple FamilyControls entitlement review (1–4 weeks, external)
2. Coachy designer asset delivery (1–2 weeks, external)
3. Ops signing key ceremony (pending user availability, ~2 batches)

**High-Impact Risks:**
- Loro CRDT merge conflict complexity (Q3 multi-device sync)
- Android AccessibilityService permission gating + user education
- Enterprise SSO integration complexity (federation, token refresh)

**Mitigation:**
- Q2 contingency: Use local-only sync if CloudKit blocked; async migration path
- Q3 contingency: Bundle plugin SDK as separate release if core plugin system slips
- Q4 contingency: Defer browser extension to Phase 4 if Android slips

---

## Notes

- **Agent-batch estimates** assume 2–3 min per batch (code + tests + integration).
- **External blockers** (Apple, designer, ops key) are dependencies, not effort — do not add to batches.
- **Parallel execution** preferred where possible; critical path is sequential entitlement approval.
- **Stretch goals** (if ahead of schedule): plugin SDK (Q4), browser extension (Q4), serverless backend (Phase 5).
