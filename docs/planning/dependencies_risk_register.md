# FocalPoint Dependencies & Risk Register

**Planning Wave:** 2026-04-24
**Scope:** Known blockers, external dependencies, risk assessment, and mitigation for Q2–Q4 2026

---

## Critical Path Blockers (Honest Assessment)

### 1. Apple FamilyControls Entitlement Review

**Status:** Submitted ~2026-04-20; pending Apple review (1–4 weeks typical, can exceed 8 weeks in edge cases)

**Impact on Timeline:**
- **Blocks:** Real device testing, TestFlight submission, enforcement driver implementation
- **Consequence:** Cannot verify actual app enforcement on iPhone; simulator testing is incomplete confidence signal
- **Q2 Mitigation:** Phase 1.5 ships with flag-gated enforcement (`#[if FOCALPOINT_HAS_FAMILYCONTROLS]`); real driver activates only when entitlement is approved
- **If delayed beyond Q2:** TestFlight launch pushed to Q3, but canary testing on simulator proceeds

**Trigger Threshold:** Escalate if not approved by 2026-06-01 (50-day window from submission)

**Escalation Plan:**
- Day 45: Contact Apple Developer Relations for status check
- Day 60: Consider alternative enforcement approach (AccessibilityService workaround, UNUserNotificationCenter escalation)
- Day 75: Consider resubmission with revised entitlement scope or developer account switch

**Mitigation:**
- Enforcement logic already implemented, flag-gated (ready to activate)
- Simulator enforcement testing possible (logs without real blocking)
- Community feedback loop can begin on TestFlight with non-enforcing binary

---

### 2. Ops Signing Key Ceremony

**Status:** Pending user availability; one-time ceremony required for iOS code signing

**Impact on Timeline:**
- **Blocks:** TestFlight submission (binary must be signed with prod certificate)
- **Consequence:** Cannot ship to TestFlight until key ceremony is completed
- **Effort:** 1–2 agent-batches, ~10 min (mechanical: key generation, CSR upload, cert provisioning)
- **Duration:** 1–2 hours user time (async handoff with dev team)

**Trigger Threshold:** Schedule ceremony no later than 2026-05-15 (8 weeks before end of Q2)

**Escalation Plan:**
- May 15: Formal request for ceremony scheduling
- May 30: Escalate to team lead if ceremony not scheduled
- Jun 1: TestFlight submission may slip if not complete

**Mitigation:**
- Pre-stage all signing infrastructure (provisioning profiles, CSR templates)
- Parallel CI setup (self-hosted Woodpecker, no GitHub Actions needed)
- Developer account already created (2026-04-XX)

---

### 3. Designer Coachy Asset Delivery

**Status:** Art direction + SwiftUI fallback shipped; `.riv` file pending designer production

**Impact on Timeline:**
- **Blocks:** Polished onboarding (Coachy animation in Morning Brief + Today tab)
- **Consequence:** SwiftUI static fallback acceptable for MVP; animation enhancement deferred
- **Effort if delayed:** 0 blocking batches (fallback is shipped)
- **Integration effort if delivered:** 1–2 agent-batches, ~8 min (load `.riv`, test all 14 poses)

**Expected Delivery:** 2026-05-15 (designer internal timeline)

**Trigger Threshold:** Request delivery by 2026-05-10; if not received, proceed with fallback (animation as post-Phase-1.5 feature)

**Escalation Plan:**
- May 10: Formal delivery request
- May 17: If not received, lock design as "fallback only" and remove from Phase 1.5 scope
- Q3: Schedule designer for animation enhancements (lower priority)

**Mitigation:**
- SwiftUI fallback shipped in Phase 1 (no animation blocker)
- Rive loader already integrated (load `.riv` file when present, graceful fallback if missing)
- 14-pose spec already documented (designer knows exact deliverable)

---

## High-Impact External Dependencies

### 4. Canvas Sandbox Test Credentials

**Status:** Awaiting Canvas institution account with API token + test course data

**Impact on Timeline:**
- **Blocks:** Live-API integration tests for Canvas connector (Q3)
- **Consequence:** Wiremock tests cover happy path; edge cases (rate-limiting, pagination, error recovery) tested in sandbox only
- **Workaround:** Skip live-API test (marked `#[ignore]`), proceed with wiremock coverage

**Trigger Threshold:** Request credentials by 2026-06-15; if not available by 2026-07-15, mark as deferred and prioritize GCal/GitHub

**Escalation Plan:**
- Jun 15: Formal request to Canvas contact
- Jul 1: Follow-up if no response
- Jul 15: Move live-API test to Q4 or defer to post-MVP
- Jul 30: Proceed with wiremock coverage only (Notion/Todoist prioritized instead)

**Mitigation:**
- 44 wiremock tests already written (comprehensive coverage)
- Real-API test guarded by feature flag (`live-canvas`)
- Connector logic validated without live credentials

---

### 5. Apple CloudKit Quota & Sandbox Setup

**Status:** CloudKit sandbox available; quota needs verification for scale testing

**Impact on Timeline:**
- **Blocks:** Q3 multi-device sync load testing (500+ concurrent users)
- **Consequence:** May need to mock CloudKit for performance benchmarking if quota is too restrictive
- **Workaround:** Local SQLite + CloudKit emulation for load tests

**Trigger Threshold:** Benchmark by 2026-08-15; if quota insufficient, request increase by 2026-08-30

**Escalation Plan:**
- Aug 1: Request Apple CloudKit quota increase
- Aug 15: Run load test; assess if quota increase approved
- Aug 30: If denied, implement local CloudKit mock for scale testing
- Sep 15: Reassess quota for Phase 3 Android sync

**Mitigation:**
- Local SQLite can be used for load testing (same schema, no network latency)
- CloudKit quota increase typically approved for production apps
- Mock CloudKit adapter already in test suite

---

## Technology & Architecture Risks

### 6. Loro CRDT Merge Conflict Complexity (Q3)

**Status:** Loro selected; first implementation will reveal edge cases

**Impact on Timeline:**
- **Blocks:** Q3 multi-device sync release
- **Consequence:** Conflict resolution logic may require more iterations than estimated
- **Effort Estimate:** 10–15 agent-batches (range accounts for unknown complexity)

**Risk Probability:** Medium (first CRDT implementation; Loro docs mature)

**Trigger Threshold:** First integration test run (Jul 15); if >25% of merge scenarios fail, escalate

**Escalation Plan:**
- Jul 15: Run property-based tests on CRDT merge
- Jul 25: Assess conflict rate; if >25% of scenarios fail, add 3–5 more batches
- Aug 8: If unresolved, consider alternative (simpler last-write-wins + UI merge assistant)
- Aug 22: Decision point: proceed with CRDT or defer to Q4

**Mitigation:**
- 3–5 contingency batches reserved in Q3
- Loro community support available (GitHub issues)
- Fallback: Local-only sync in Q3 (CloudKit sync deferred to Q4)
- Property-based testing started early (Jul, not Aug)

---

### 7. Android AccessibilityService Permission Gating (Q4)

**Status:** Platform design unclear; requires API level negotiation + user education

**Impact on Timeline:**
- **Blocks:** Q4 Android MVP enforcement driver
- **Consequence:** Cannot block distracting apps without AccessibilityService; fallback to notifications only
- **Effort Estimate:** 3–4 agent-batches (Android-specific platform work)

**Risk Probability:** Medium (Android permission model is strict)

**Trigger Threshold:** First implementation attempt (Oct 15); if permission denied by default, escalate

**Escalation Plan:**
- Oct 15: Test AccessibilityService binding with test device
- Oct 22: If permission model is too restrictive, implement notification-only fallback
- Nov 1: Decide: enforce commitment to users (optional enforcement) or defer to Q4 enhancement
- Nov 15: Document permission flow in onboarding

**Mitigation:**
- Fallback: Notifications + recommendations (no hard blocks)
- Documentation: Clear permission requests + explanation of why AccessibilityService is needed
- Contingency batches: 2 reserved in Q4
- Alternative: Consider MDM (Mobile Device Management) for enterprise tier (Q4)

---

### 8. Enterprise SAML 2.0 Federation Complexity (Q4)

**Status:** Architecture designed; first implementation TBD

**Impact on Timeline:**
- **Blocks:** Q4 enterprise tier SSO
- **Consequence:** Fallback to simplified OAuth + JWTs (less secure, no SAML federation)
- **Effort Estimate:** 4–6 agent-batches (includes token refresh, attribute mapping, assertion validation)

**Risk Probability:** Medium (SAML is mature but integration points are many)

**Trigger Threshold:** Architecture review (Sep 30); if complexity exceeds 6 batches, escalate

**Escalation Plan:**
- Sep 30: Detailed SAML implementation plan + complexity review
- Oct 15: First integration test with test SAML provider (e.g., Keycloak mock)
- Oct 30: If >6 batches needed, consider Okapi or oauth2-proxy wrapper (reduce scope)
- Nov 15: Decision point: proceed with full SAML or use OAuth + JWTs + defer SAML to Phase 4

**Mitigation:**
- 2 contingency batches reserved in Q4-Oct
- Pre-built crate options: `saml` + `openssl` + `xmlsec1` (mature ecosystem)
- Fallback: OAuth 2.0 + refresh token flow (simpler, acceptable for MVP)
- Early complexity assessment (Sep, not Oct)

---

## External Partner & Process Dependencies

### 9. GitHub Actions Billing Block (All Quarters)

**Status:** Known hard constraint (KooshaPari org has $450/mo limit, exhausted by agent swarms)

**Impact on Timeline:**
- **Blocks:** CI checks on public GitHub (macOS, Windows runners; billed runners fail immediately)
- **Consequence:** No automated CI for FocalPoint public repo; local + self-hosted CI only
- **Mitigation:** Self-hosted Woodpecker CI (free tier) or local cargo test before push

**Trigger Threshold:** N/A (known blocker, accepted risk)

**Escalation Plan:** None (GH Actions billing is not solvable in FocalPoint context; work around via Woodpecker)

**Mitigation:**
- Woodpecker CI self-hosted (0 cost)
- Local `cargo test --workspace` gate (pre-push, 2–3 min)
- Manual verification for iOS builds (Xcode + fastlane, local dev machine)

---

### 10. Apple Developer Program Account Status

**Status:** Developer account active; team membership TBD

**Impact on Timeline:**
- **Blocks:** TestFlight submission, entitlement requests, code signing
- **Consequence:** Account revoked → all work stops
- **Probability:** Very low (account in good standing)

**Trigger Threshold:** Quarterly renewal check; payment status verified monthly

**Escalation Plan:**
- Monthly: Verify subscription active + payment method valid
- Quarterly: Check entitlements + capabilities enabled

**Mitigation:** Payment auto-renewal enabled; team account setup (minimize single-point-of-failure)

---

## Scheduled Risk Reviews

| Date | Review | Leads | Output |
|------|--------|-------|--------|
| 2026-05-15 | Q2 Checkpoint: Designer asset + entitlement status | Impl, Audit | Go/no-go for TestFlight |
| 2026-06-15 | Q2 Close: Canvas credentials, compliance docs | Doc, Audit | Q3 readiness assessment |
| 2026-07-15 | Q3 Kickoff: Loro CRDT integration health | Impl, Test | Contingency plan if >25% test failure |
| 2026-08-30 | Q3 Midpoint: CloudKit quota assessment | Test, Audit | Load test results, quota decision |
| 2026-09-30 | Q3 Close: Enterprise SAML architecture review | Impl, Audit | Q4 complexity estimate, go/no-go |
| 2026-10-30 | Q4 Midpoint: Android permission model assessment | Impl, Test | AccessibilityService binding success |
| 2026-11-30 | Q4 Close: All external dependencies recap | Doc, Audit | Phase 4 pre-reqs assessment |

---

## Risk Scoring Matrix

| Risk | Probability | Impact | Exposure | Mitigation | Status |
|------|-------------|--------|----------|-----------|--------|
| Apple entitlement delay | Medium | Critical | **HIGH** | Flag-gated fallback, simulator testing | Monitoring |
| Ops key ceremony | Low | High | Medium | Pre-stage infra, async handoff | Scheduled |
| Designer asset delay | Medium | Medium | **Medium–HIGH** | SwiftUI fallback shipped | Expected on-time |
| Canvas credentials | Low | Medium | Low | Wiremock coverage, skip live-API | Escalated Jun 15 |
| CloudKit quota | Low | Medium | Low | Local mock, quota increase request | TBD Aug 15 |
| Loro CRDT complexity | Medium | High | **Medium** | 3–5 batches reserved, property-based tests | First test Jul 15 |
| Android permissions | Medium | High | **Medium** | Fallback: notifications-only | Assessment Oct 15 |
| Enterprise SAML | Medium | Medium | Medium | 2 batches reserved, OAuth fallback | Architecture review Sep 30 |
| GitHub Actions billing | N/A | Low | Low | Woodpecker self-hosted | Accepted, mitigated |
| Dev account status | Very Low | Critical | Low | Monthly monitoring, team account | Routine |

---

## Contingency Reserve

**Total Q2–Q4 Estimate:** 142–172 agent-batches
**Contingency Absorbed:** ~15% (for Loro complexity, Android platform, SAML)
**Reserve Available:** 2–5 batches per quarter (allocated in staffing plan)

**Trigger for Reserve Usage:**
- Q2: Designer delay OR entitlement approval slips beyond Q2 (activate fallback scenario)
- Q3: Loro merge tests show >25% failure rate (add 3–5 batches)
- Q4: Android permission binding fails (activate notification-only fallback) OR SAML >6 batches (use OAuth fallback)

---

## Escalation Contacts & Process

**Escalation Chain:**
1. **Impl Lead:** Code complexity, technical blockers
2. **Audit Lead:** Quality gate, security risks
3. **Project Lead** (User): External dependencies, scope trade-offs

**Escalation Criteria:**
- Effort estimate exceeds budget by >20%
- Risk probability increases from "Medium" to "High"
- External blocker decision point reached
- Schedule impact >1 week

**Escalation Format:**
- Brief summary (1–2 sentences)
- Impact (timeline, scope, budget)
- Options (proceed, defer, fallback)
- Recommendation

---

## Notes

- **Entitlement review** is the single most critical blocker; no code workaround exists.
- **Designer asset** acceptable to defer (fallback shipped); low risk if delayed.
- **Loro CRDT** complexity will become clear by mid-Q3; early testing mitigates risk.
- **Android platform** challenges expected (permission model strict); fallback to notifications acceptable.
- **External credentials** (Canvas, CloudKit) escalated early; contingency plans in place.
