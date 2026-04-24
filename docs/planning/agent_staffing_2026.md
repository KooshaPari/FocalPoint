# FocalPoint Agent Staffing Plan 2026

**Planning Wave:** 2026-04-24
**Scope:** Agent-batch allocation and dispatch strategy for Q2–Q4 2026

---

## Agent Roles & Responsibilities

### 1. **Impl Agent** (Primary Executor)
**Responsibility:** Code implementation, tests, CI integration.

**Typical work:**
- Multi-file feature implementation (4–8 files per task)
- Rust crate development + FFI/Swift bindings
- iOS/Android native code (Compose, SwiftUI, JNI)
- Database migrations + query optimization

**Dispatch profile:**
- 1–2 impl agents per feature work package
- 3–5 agent-batches per feature (avg. 10–20 min of tool calls)
- Serial for dependent work (e.g., FFI layer must precede Swift bindings)
- Parallel for independent feature branches

### 2. **Test Agent** (Verification)
**Responsibility:** Unit + integration testing, property-based tests, CI harness.

**Typical work:**
- Write test suites (wiremock, proptest, UI automation)
- Verify FR traceability (every test → FR tag)
- Integration scenario validation (end-to-end flows)
- Performance benchmarking (startup time, memory, sync latency)

**Dispatch profile:**
- 1 test agent per feature (runs after impl agent completes)
- 1–2 agent-batches per feature
- Parallelizable with impl work (test fixtures + test doubles ready early)
- Blocker for merge if code coverage <80%

### 3. **Doc Agent** (Knowledge Capture)
**Responsibility:** Design docs, API docs, guides, RFC process.

**Typical work:**
- Architecture decision records (ADRs)
- Connector developer guides + API documentation
- Onboarding materials + troubleshooting FAQs
- RFC drafting (process + community feedback synthesis)

**Dispatch profile:**
- 1 doc agent per phase (started at phase kickoff)
- 2–3 agent-batches per phase (docs written in parallel with implementation)
- Critical for ecosystem (plugin SDK, connector manifests)

### 4. **Audit Agent** (Quality Gate)
**Responsibility:** Static analysis, dependency audit, security review.

**Typical work:**
- Clippy pass (zero warnings)
- Workspace compilation verification
- Dependency CVE scan + updates
- Entitlement scope documentation
- OWASP + SAST review (pre-public-release)

**Dispatch profile:**
- 1 audit agent per phase
- 1–2 agent-batches at phase end (consolidation)
- Blocker for release branches
- Nightly: continuous CVE monitoring

### 5. **Design Agent** (UX/UI)
**Responsibility:** Visual design, interaction flows, accessibility.

**Typical work:**
- SwiftUI/Compose layout + animation
- Onboarding flow UX (screen sequencing, microcopy)
- Accessibility audit (VoiceOver, text scaling)
- Design system token updates (colors, typography, spacing)

**Dispatch profile:**
- 1 design agent per major UI feature
- 2–3 agent-batches per feature
- Parallel with impl (design system + component library ready first)
- Dependent on designer asset delivery (Coachy `.riv`, icons, etc.)

---

## Quarterly Staffing Plan

### Q2 2026 (Apr–Jun) — Phase 1.5 Fix + TestFlight

**Total Effort:** 22–28 agent-batches over ~10 weeks

#### Month 1 (Apr 24–May 23)

**Work Packages:**

1. **Workspace Compilation Fix** (Priority: Critical)
   - Agents: Impl (1) + Audit (1)
   - Duration: 1 week (Apr 24–May 1)
   - Effort: 3–5 agent-batches
   - Dependencies: None
   - Output: All 17 crates compile, `cargo clippy` green

2. **Onboarding Flow v2** (Priority: High)
   - Agents: Design (1) + Impl (2) + Test (1)
   - Duration: 2 weeks (May 1–15)
   - Effort: 6–8 agent-batches
   - Dependencies: Workspace compilation fix
   - Output: 5 screens (Welcome, Permissions, Rules Intro, Coachy Setup, Dashboard), integration tests

3. **GCal/GitHub OAuth Wire-Up** (Priority: High)
   - Agents: Impl (1) + Test (1)
   - Duration: 1 week (May 15–22)
   - Effort: 3–4 agent-batches
   - Dependencies: Workspace fix
   - Output: Live OAuth callback handling, connector registration

4. **Designer Asset Integration** (Parallel, Priority: Medium)
   - Agents: Impl (1) [blocked on designer delivery] + Design (1)
   - Duration: 1 week (May 15–22, waits for `.riv` file)
   - Effort: 2–3 agent-batches (if asset arrives on time)
   - Dependencies: Coachy `.riv` from designer
   - Output: Animated Coachy with all 14 poses, fallback handling

**May Dispatch Cadence:**
- Week 1: 3 batches (compilation fix)
- Week 2: 4 batches (onboarding screens)
- Week 3: 2 batches (OAuth wiring)
- Week 4: 1 batch (asset integration)
- **May Total: 10 batches**

#### Month 2 (May 24–Jun 23)

**Work Packages:**

5. **TestFlight Submission & Sandbox Ops** (Priority: Critical)
   - Agents: Impl (1) + Audit (1)
   - Duration: 1 week (May 24–Jun 1)
   - Effort: 3–4 agent-batches
   - Dependencies: Phase 1 completion, ops key ceremony
   - Output: Binary signed, TestFlight submission accepted, Sentry monitoring live

6. **Discord Community Setup & Rules Gallery** (Priority: High)
   - Agents: Doc (1) + Impl (1)
   - Duration: 1 week (Jun 1–8)
   - Effort: 3–4 agent-batches
   - Dependencies: None (can start early)
   - Output: Discord server (5 channels), GitHub form, template showcase

7. **First-Party Rule Template Packs** (Priority: Medium)
   - Agents: Impl (1) + Doc (1)
   - Duration: 1 week (Jun 8–15)
   - Effort: 2–3 agent-batches
   - Dependencies: Template pack format (already done)
   - Output: 3 packs (deep-work, exam-prep, wind-down) signed + published

8. **Q2 Security & Compliance** (Priority: High)
   - Agents: Audit (1) + Doc (1)
   - Duration: 1 week (Jun 15–22)
   - Effort: 2–3 agent-batches
   - Dependencies: None
   - Output: Privacy policy, terms of service, entitlement scope doc

**June Dispatch Cadence:**
- Week 1: 2 batches (TestFlight submission)
- Week 2: 2 batches (Discord + community setup)
- Week 3: 2 batches (template packs)
- Week 4: 1–2 batches (compliance docs)
- **June Total: 7–8 batches**

**Q2 Total: 17–18 batches (within estimated 22–28; contingency for designer delay absorbed)**

### Q3 2026 (Jul–Sep) — Phase 2 Multi-Device Sync & Subscriptions

**Total Effort:** 58–68 agent-batches over ~13 weeks

#### Month 1 (Jul 1–31)

**Work Packages:**

9. **CloudKit Adapter & Loro CRDT Integration** (Priority: Critical)
   - Agents: Impl (2) + Test (1)
   - Duration: 2 weeks (Jul 1–15)
   - Effort: 8–10 agent-batches
   - Dependencies: None
   - Output: `focus-cloudkit` crate, bi-directional sync tests, conflict resolution UI scaffold

10. **StoreKit Integration & Tier Definitions** (Priority: High)
    - Agents: Impl (1) + Test (1)
    - Duration: 1 week (Jul 15–22)
    - Effort: 3–4 agent-batches
    - Dependencies: StoreKit server verifier (pre-Phase-2)
    - Output: Free/Premium/Family tiers, feature gating, restore purchases

11. **Canvas Connector Hardening** (Priority: Medium)
    - Agents: Impl (1) + Test (1)
    - Duration: 1 week (Jul 15–22)
    - Effort: 2–3 agent-batches
    - Dependencies: Canvas scaffold (Phase 1)
    - Output: Live-API tests (sandbox creds obtained), error recovery, rate-limit handling

**July Dispatch Cadence:**
- Week 1: 3 batches (CloudKit setup)
- Week 2: 3 batches (CloudKit integration, Loro)
- Week 3: 3 batches (StoreKit + Canvas)
- Week 4: 2 batches (conflict resolution UI)
- **July Total: 11 batches**

#### Month 2 (Aug 1–31)

**Work Packages:**

12. **Apple Watch Companion App** (Priority: High)
    - Agents: Impl (2) + Design (1) + Test (1)
    - Duration: 2.5 weeks (Aug 1–17)
    - Effort: 8–10 agent-batches
    - Dependencies: CloudKit sync (Q3-M1)
    - Output: watchOS app (Home, Complications), SharedModel bridge, Activity rings integration

13. **Notion & Todoist Connectors** (Priority: High)
    - Agents: Impl (2) + Test (1)
    - Duration: 2 weeks (Aug 1–15)
    - Effort: 6–7 agent-batches
    - Dependencies: Connector scaffold (Phase 1)
    - Output: OAuth flows, event mapping, rate-limit handling, wiremock test suites

14. **Marketplace UI & Community Submission Flow** (Priority: Medium)
    - Agents: Impl (1) + Design (1) + Doc (1)
    - Duration: 2 weeks (Aug 15–29)
    - Effort: 4–5 agent-batches
    - Dependencies: Template pack format + verification tiers
    - Output: Browse UI, tier-filtered catalog, submit → verify → install flow

**August Dispatch Cadence:**
- Week 1: 3 batches (Watch app, connectors)
- Week 2: 3 batches (Watch + Notion/Todoist)
- Week 3: 3 batches (Marketplace UI)
- Week 4: 2 batches (submission CI, polish)
- **August Total: 11 batches**

#### Month 3 (Sep 1–30)

**Work Packages:**

15. **Apple Health Connector & GitHub Finalization** (Priority: Medium)
    - Agents: Impl (1) + Test (1)
    - Duration: 1 week (Sep 1–8)
    - Effort: 2–3 agent-batches
    - Dependencies: Connector scaffold
    - Output: Health API integration, webhook handling, GitHub events

16. **Performance Benchmarking & Optimization** (Priority: Medium)
    - Agents: Impl (1) + Audit (1)
    - Duration: 1.5 weeks (Sep 8–18)
    - Effort: 3–4 agent-batches
    - Dependencies: All Q3 features (to benchmark together)
    - Output: Launch time <2s, sync latency <500ms, database <50 MB

17. **Q3 Wrap-Up: Metrics, Docs, Security** (Priority: High)
    - Agents: Doc (1) + Audit (1) + Design (1)
    - Duration: 2 weeks (Sep 18–30)
    - Effort: 4–5 agent-batches
    - Dependencies: All Q3 features
    - Output: Connector dev guide, cohort analysis dashboard, post-Q3 security review

**September Dispatch Cadence:**
- Week 1: 2 batches (Health, GitHub connectors)
- Week 2: 2 batches (performance optimization)
- Week 3: 2 batches (Marketplace polish, metrics)
- Week 4: 2 batches (docs, security review)
- **September Total: 8 batches**

**Q3 Total: 30 batches (within estimated 58–68; contingency for Loro merge complexity provides buffer)**

### Q4 2026 (Oct–Dec) — Phase 3 Android MVP & Enterprise Tier

**Total Effort:** 62–76 agent-batches over ~13 weeks

#### Month 1 (Oct 1–31)

**Work Packages:**

18. **JNI Bindings & Compose Shell** (Priority: Critical)
    - Agents: Impl (2) + Design (1) + Test (1)
    - Duration: 2.5 weeks (Oct 1–19)
    - Effort: 10–12 agent-batches
    - Dependencies: FocalPointCore stable (Q3)
    - Output: JNI layer, Compose app shell (Home, Rules, Activity, Settings), Material 3 design

19. **Enterprise SSO & Admin Console** (Priority: High)
    - Agents: Impl (2) + Design (1) + Doc (1)
    - Duration: 2.5 weeks (Oct 1–19)
    - Effort: 8–10 agent-batches
    - Dependencies: Multi-tenant schema design (early Q4)
    - Output: SAML 2.0 provider integration, admin console (user mgmt, audit export), tier definitions

20. **Safari Web Extension** (Priority: Medium)
    - Agents: Impl (1) + Test (1)
    - Duration: 1.5 weeks (Oct 8–19)
    - Effort: 3–4 agent-batches
    - Dependencies: None
    - Output: Safari WKWebView content script, distraction URL detection, iOS 17+ build

**October Dispatch Cadence:**
- Week 1: 3 batches (JNI setup, Compose scaffold)
- Week 2: 3 batches (Admin console, SSO)
- Week 3: 3 batches (Android connectors, Safari)
- Week 4: 3 batches (integration, testing)
- **October Total: 12 batches**

#### Month 2 (Nov 1–30)

**Work Packages:**

21. **Android Canvas/GCal OAuth & Platform Adapters** (Priority: High)
    - Agents: Impl (2) + Test (1)
    - Duration: 1.5 weeks (Nov 1–12)
    - Effort: 4–5 agent-batches
    - Dependencies: JNI bindings, Compose shell
    - Output: Custom Tabs OAuth, token storage, UsageStats + AccessibilityService adapters

22. **Plugin SDK (Lua + WASM Runtime)** (Priority: High)
    - Agents: Impl (2) + Doc (1) + Test (1)
    - Duration: 2 weeks (Nov 1–15)
    - Effort: 6–8 agent-batches
    - Dependencies: Plugin architecture design (early Q4)
    - Output: Lua VM integration, WASM loader, 3 sample plugins, developer guide

23. **Chrome/Firefox Extension** (Priority: Medium)
    - Agents: Impl (1) + Test (1)
    - Duration: 1.5 weeks (Nov 12–26)
    - Effort: 3–4 agent-batches
    - Dependencies: Safari extension (Oct)
    - Output: Manifest V3 ported, content script, policy sync

**November Dispatch Cadence:**
- Week 1: 3 batches (Android OAuth, plugin SDK setup)
- Week 2: 3 batches (plugin SDK + Chrome extension)
- Week 3: 2 batches (platform adapters, extension testing)
- Week 4: 2 batches (plugin examples, documentation)
- **November Total: 10 batches**

#### Month 3 (Dec 1–31)

**Work Packages:**

24. **Android Finalization & Real Device Testing** (Priority: Critical)
    - Agents: Impl (1) + Test (2)
    - Duration: 2 weeks (Dec 1–15)
    - Effort: 4–5 agent-batches
    - Dependencies: All Android work (Nov)
    - Output: UI automation tests, 100 internal testers onboarded, crash rate <2%

25. **Q4 Marketing & Community** (Priority: High)
    - Agents: Doc (1) + Design (1)
    - Duration: 2 weeks (Dec 1–15)
    - Effort: 3–4 agent-batches
    - Dependencies: Android stable, plugin SDK docs
    - Output: Marketing site, Product Hunt launch, NPS survey

26. **Performance & Scale Verification** (Priority: Medium)
    - Agents: Audit (1) + Test (1)
    - Duration: 2 weeks (Dec 1–15)
    - Effort: 2–3 agent-batches
    - Dependencies: All features (load testing)
    - Output: Benchmarks (launch <2s, sync <500ms), database <50 MB

27. **Year-End Retrospective & Q1 Planning** (Priority: Medium)
    - Agents: Doc (1)
    - Duration: 2 weeks (Dec 15–31)
    - Effort: 2–3 agent-batches
    - Dependencies: Q4 completion
    - Output: Retrospective doc, Q1 2027 roadmap, lessons learned

**December Dispatch Cadence:**
- Week 1: 3 batches (Android testing, marketing setup)
- Week 2: 3 batches (performance benchmarks, plugin examples)
- Week 3: 2 batches (marketing launch, NPS survey)
- Week 4: 2 batches (retrospective, Q1 planning)
- **December Total: 10 batches**

**Q4 Total: 32 batches (within estimated 62–76; contingency for plugin SDK complexity provides buffer)**

---

## Dispatch Priority Matrix

### Serial Dependencies (Blocking)

| Phase | Dependency | Unblocks | Min Delay |
|-------|-----------|----------|-----------|
| Q2 | Workspace compilation | Onboarding, OAuth, design | 1 week |
| Q2 | Designer Coachy asset | Animation integration | 1–2 weeks |
| Q2 | TestFlight submission | Community launch, canary testing | 3–5 days |
| Q3 | CloudKit adapter | Watch app, conflict resolution UI | 2 weeks |
| Q3 | StoreKit tier definitions | Feature gating, subscription UI | 1 week |
| Q4 | JNI bindings + Compose shell | Android OAuth, platform adapters | 2–3 weeks |
| Q4 | Plugin SDK architecture | Sample plugins, community submissions | 2 weeks |

### Parallel Opportunities (No Blocking)

- Q2: Discord setup (parallel with compilation, onboarding, OAuth)
- Q2: Compliance docs (parallel with all Phase 1.5 work)
- Q3: Marketplace UI (parallel with connector implementation)
- Q3: Performance benchmarking (can happen after connectors, async)
- Q4: Chrome extension (parallel with Safari, independent)
- Q4: Marketing site (parallel with Android testing, design-only dependency)

---

## Monthly Batch Allocation Summary

| Month | Batches | Lead Agent | Contingency |
|-------|---------|-----------|-------------|
| **Q2-Apr** | 10 | Impl (3), Audit (2) | Designer delay: −2 batches → Q2-May |
| **Q2-May** | 7–8 | Impl (2), Design (1) | Onboarding scope creep: +2 batches |
| **Q3-Jul** | 11 | Impl (2), Test (1) | Loro complexity: +3 batches available |
| **Q3-Aug** | 11 | Impl (2), Design (1) | Connector pacing flexibility |
| **Q3-Sep** | 8 | Doc (1), Audit (1) | Performance bottleneck: +2 batches if needed |
| **Q4-Oct** | 12 | Impl (2), Design (1) | Android scope: +2 batches if ahead |
| **Q4-Nov** | 10 | Impl (2), Test (1) | Plugin SDK complexity: +3 batches available |
| **Q4-Dec** | 10 | Test (2), Doc (1) | Marketing + retrospective: flexible timeline |

---

## Cost Estimates (Tool-Call Units)

**Assumptions:**
- 1 agent-batch = 50–150 tool calls (code, tests, commits, verifications)
- Average: 100 tool calls per batch
- **Total Q2–Q4:** 142–172 agent-batches = **14,200–17,200 tool calls**

**Breakdown by category:**
- **Impl (code):** ~60% of calls (8,500–10,300 calls)
- **Test (verification):** ~20% of calls (2,800–3,400 calls)
- **Audit (quality):** ~10% of calls (1,400–1,700 calls)
- **Doc (knowledge):** ~7% of calls (1,000–1,200 calls)
- **Design (UX):** ~3% of calls (400–500 calls)

---

## Staffing Risks & Mitigations

### Risk 1: Designer Asset Delay (Q2)
**Impact:** Coachy animation blocks Phase 1.5 polish
**Probability:** Medium (external dependency)
**Mitigation:**
- SwiftUI fallback shipped (no animation blocking submission)
- Designer asset as post-TestFlight enhancement
- Batch contingency: −2 (reassign to compliance docs if delayed beyond May 15)

### Risk 2: Loro CRDT Merge Complexity (Q3)
**Impact:** Multi-device sync slips beyond Q3
**Probability:** Medium (first implementation of conflict resolution)
**Mitigation:**
- Batches available: 3 contingency in Q3-Jul/Aug
- Fallback: Local-only sync in Q3, CloudKit-sync only in Q4
- Early property-based testing (Aug)

### Risk 3: Android Platform Adapters (Q4)
**Impact:** AccessibilityService permissions + UsageStats API complex to wire
**Probability:** Medium (Android-specific challenges)
**Mitigation:**
- Batches available: 2 contingency in Q4-Oct/Nov
- Fallback: Simulator-only testing (no real device features) for Q4 alpha
- Early platform research (Oct)

### Risk 4: Enterprise SSO Integration (Q4)
**Impact:** SAML/OIDC federation complexity, token refresh
**Probability:** Medium
**Mitigation:**
- Batches available: 2 contingency in Q4-Oct
- Fallback: Simplified OAuth + JWTs (delay SAML to Phase 4)
- Early architecture review (late Q3)

### Risk 5: Plugin SDK Testing (Q4)
**Impact:** Sandbox isolation + performance overhead
**Probability:** Low (Lua is mature)
**Mitigation:**
- Batches available: 3 contingency in Q4-Nov
- Sample plugins cover 80% of use cases

---

## Notes

- **Agent-batch estimates** are wall-clock batches (2–3 min per batch for a single agent).
- **Parallelism** is assumed where listed; serial work stretches timeline proportionally.
- **Contingency buffer:** ~15% reserved across Q2–Q4 (covered within 142–172 range).
- **External blockers** (Apple entitlement, designer, ops key) are not included in agent-batch counts.
- **Dispatch strategy:** Prefer parallel work; serial only where explicitly dependent.
