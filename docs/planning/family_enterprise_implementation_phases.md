# Family Sharing & Enterprise SSO: Implementation Roadmap

## Overview

This document breaks down FocalPoint's Phase 3 (Family Sharing) and Phase 4 (Enterprise SSO) into phased work packages with agent-batch estimates, dependencies, and prerequisites.

**Timeline assumptions**:
- Agent batches: 10–20 min wall-clock per batch (2–5 agent tasks in parallel)
- Personal tier is complete and stable (Phase 1–2 shipped)
- iOS multi-device sync (Loro + CloudKit) is complete (Phase 2, currently in progress)
- No architectural redesign required; new features layer on existing trait surfaces

---

## Phase 3: Family Sharing Tier

**Scope**: Up to 6 family members, one Pro subscription ($14.99/mo), Guardian dashboard, child-safety defaults, shared templates.

**Total effort**: ~50–70 agent-batches (~100–140 min wall-clock)

### Phase 3A: Backend & Data Model

**Effort**: 20–25 agent-batches (~40–50 min)

**Deliverables**:
1. Database schema migrations
2. Family organization type in `organizations` table
3. Family-scoped CloudKit zones
4. Summary aggregation service
5. Family template CRUD
6. Guardian read-only access API

**Subtasks**:

| Task | Est. Batches | Depends On | Notes |
|------|--------|-----------|-------|
| **DA.1** Add `org_id`, `org_role` to users table | 2 | Schema plan | Backward-compatible migration script |
| **DA.2** Create `organizations` table (family type) | 2 | DA.1 | `org_type = 'family'`, `owner_user_id`, `subscription_tier` |
| **DA.3** Add `org_id` to rules, tasks, wallet, penalties, audit | 3 | DA.2 | Per-table migrations, foreign key constraints |
| **DA.4** Create family CloudKit zones (private + aggregate) | 3 | DA.3 | `focus-cloudkit` crate; zone sharing logic |
| **DA.5** Build summary aggregation service | 4 | DA.4 | Nightly CloudKit query → `FamilyWeeklySummary` records |
| **DA.6** Family template store (`focus-storage::family_templates`) | 3 | DA.3 | CRUD operations for shared templates |
| **DA.7** Guardian API (read-only member stats) | 3 | DA.5, DA.6 | Aggregate stats endpoint + member summaries |

**Exit criteria**:
- ✅ Schema migrated; zero SQL errors
- ✅ Family zones created on org creation
- ✅ Summary service runs without errors (E2E test)
- ✅ Guardian API returns correct aggregates (unit + integration tests)

---

### Phase 3B: iOS Family Dashboard & Onboarding

**Effort**: 15–20 agent-batches (~30–40 min)

**Deliverables**:
1. New "Family" tab in iOS app
2. Guardian overview card + member summaries
3. Family account creation flow (3 screens)
4. Family member invitation (email + in-app)
5. Settings for child-safety defaults
6. Member detail view (weekly summary)

**Subtasks**:

| Task | Est. Batches | Depends On | Notes |
|------|--------|-----------|-------|
| **IB.1** Family tab view (SwiftUI) | 3 | Phase 2 complete | Overview card, member cards, actions |
| **IB.2** Guardian dashboard stats view | 2 | DA.7 | Consume Guardian API; format & display |
| **IB.3** Family account creation wizard (3 screens) | 3 | DA.1, DA.2 | "Organizer", "Add Members", "Child Safety" |
| **IB.4** Member invitation modal + email send | 2 | Focus-core FFI | Call backend to invite + send email |
| **IB.5** Child-safety defaults settings UI | 2 | Phase 3A | Toggles for focus cap, restricted hours, connectors |
| **IB.6** Member detail view + weekly summary | 2 | DA.5 | Tap member → see stats, penalties count |
| **IB.7** Family template library (install picker) | 3 | DA.6 | Browse, apply, customize templates |

**Exit criteria**:
- ✅ Family tab displays correctly
- ✅ Create family account flow works E2E
- ✅ Guardian sees member summaries (no stale data)
- ✅ TestFlight build compiles without warnings

---

### Phase 3C: Child-Safety Enforcement & Rule Explanations

**Effort**: 8–12 agent-batches (~15–25 min)

**Deliverables**:
1. Focus duration caps (60 min for minors vs. 120 for adults)
2. Restricted hours (no focus after 10 PM, before 6 AM)
3. Mandatory breaks (15 min cooldown between sessions)
4. Penalty escalation (shorter lockouts, faster escalation)
5. Guardian override flow
6. Rule explanations (kid-friendly language)

**Subtasks**:

| Task | Est. Batches | Depends On | Notes |
|------|--------|-----------|-------|
| **EC.1** Update `focus-rules` engine for child defaults | 2 | DA.3 | Load child-safety settings per rule evaluation |
| **EC.2** Focus duration cap enforcement | 1 | EC.1 | Reject sessions > 60 min for minors; log violation |
| **EC.3** Restricted hours enforcement | 1 | EC.1 | Block sessions outside 6 AM–10 PM (configurable) |
| **EC.4** Penalty escalation for minors | 2 | Focus-penalties | 1h lockout, faster escalation thresholds |
| **EC.5** Guardian override UI (Family tab) | 2 | IB.6 | Tap penalty → "Clear Penalty" button + audit log |
| **EC.6** Rule explanation overlay (kid-friendly text) | 2 | IB.1, IB.5 | Show "Why can't I?" for every restriction |

**Exit criteria**:
- ✅ Child account rejects 75-min focus session (enforced to 60 min)
- ✅ Penalties escalate faster for minors (3 breaches = escalation)
- ✅ Guardian can clear penalty; audit log recorded
- ✅ Rule explanations display in kid-appropriate language

---

### Phase 3D: Testing, Compliance & Release

**Effort**: 6–10 agent-batches (~12–20 min)

**Deliverables**:
1. E2E test suite (family creation, member invite, sync)
2. COPPA compliance audit (for under-13 accounts)
3. TestFlight rollout to family accounts
4. Release notes & onboarding guide
5. Monitoring dashboards (adoption, sync latency)

**Subtasks**:

| Task | Est. Batches | Depends On | Notes |
|------|--------|-----------|-------|
| **RD.1** E2E family creation test | 2 | 3A, 3B, 3C | Create family, invite member, verify sync |
| **RD.2** E2E child-safety test | 1 | 3C | Create minor account, test duration cap, escalation |
| **RD.3** COPPA compliance review + docs | 2 | 3B, 3C | Parental consent flow, data minimization, deletion |
| **RD.4** TestFlight family variant build | 1 | All | Release to beta testers (100 users) |
| **RD.5** Adoption monitoring (Sentry + custom events) | 2 | RD.4 | Track: family created, members invited, templates applied |
| **RD.6** Release notes + onboarding guide | 1 | All | Markdown doc for web + in-app |

**Exit criteria**:
- ✅ E2E tests pass (family + child accounts)
- ✅ COPPA compliance docs reviewed by legal
- ✅ TestFlight rollout to 100 family users
- ✅ Monitoring alerts configured

---

## Phase 4: Enterprise SSO & Organization Tier

**Scope**: SAML 2.0 + OIDC, SCIM 2.0, org-scoped data isolation, admin portal, $49.99/user/mo pricing.

**Total effort**: ~65–90 agent-batches (~130–180 min wall-clock)

**Prerequisite**: Phase 3 complete + multi-device sync stable (Phase 2)

### Phase 4A: Backend & Organization Data Model

**Effort**: 25–30 agent-batches (~50–60 min)

**Deliverables**:
1. Organization schema extension (SAML, SCIM, member roles)
2. Enterprise CloudKit zones (org + audit)
3. SCIM 2.0 service (endpoints, provisioning)
4. WorkOS integration (SAML/OIDC abstraction)
5. Org-scoped data queries
6. SCIM deprovisioning workflow

**Subtasks**:

| Task | Est. Batches | Depends On | Notes |
|------|--------|-----------|-------|
| **EA.1** Extend `organizations` table (SAML, SCIM config) | 2 | Phase 3A | `idp_type`, `idp_metadata_url`, `scim_enabled`, `idp_secret_token` |
| **EA.2** Create enterprise CloudKit zones | 3 | EA.1 | Org zone, org audit zone; sharing with members |
| **EA.3** Build SCIM 2.0 service (Rust, `actix-web`) | 6 | EA.2 | GET/POST/PATCH /Users, /Groups, /ServiceProviderConfig |
| **EA.4** SCIM user provisioning logic | 3 | EA.3 | Create user, assign org role, detect duplicate email |
| **EA.5** SCIM deprovisioning (soft delete) | 2 | EA.4 | PATCH user `active: false`, grace period, data purge |
| **EA.6** WorkOS integration (SAML/OIDC wrapper) | 4 | EA.2 | Call WorkOS API on login, map org + user + attributes |
| **EA.7** Org template store + application logic | 2 | Phase 3A, EA.2 | Create, list, apply org templates; track parent ID |
| **EA.8** Multi-org user queries (access control) | 2 | EA.1, EA.7 | User can belong to N orgs; queries return correct partition |

**Exit criteria**:
- ✅ SCIM endpoints respond to test requests (Postman)
- ✅ User provisioning works E2E (SCIM → database → app)
- ✅ WorkOS SAML flow works (login → user created)
- ✅ Org zones created on org provisioning; members sync correctly

---

### Phase 4B: Web Admin Portal

**Effort**: 20–25 agent-batches (~40–50 min)

**Deliverables**:
1. Admin authentication (SAML + OIDC)
2. Dashboard (7 tabs: overview, members, templates, connectors, audit, billing, settings)
3. Member management UI (invite, bulk actions, deactivate)
4. Template builder + assignment (web)
5. SAML/SCIM configuration console
6. Audit log viewer + export

**Tech stack**: Next.js (TypeScript) + Tailwind CSS, hosted on Vercel

**Subtasks**:

| Task | Est. Batches | Depends On | Notes |
|------|--------|-----------|-------|
| **EB.1** Next.js project setup + auth (OIDC) | 2 | EA.6 | Login via OIDC, org-scoped token, TypeScript |
| **EB.2** Dashboard layout (7-tab shell) | 2 | EB.1 | Sidebar nav, responsive design, dark mode |
| **EB.3** Overview tab (stats, org metadata) | 2 | Phase 4A complete | Fetch org stats API; display adoption metrics |
| **EB.4** Members tab (list, invite, bulk actions) | 3 | EB.3 | Table, filter, deactivate, export to CSV |
| **EB.5** Templates & Policies tab (builder, assign) | 4 | EB.4 | Multi-rule editor, version history, apply to team |
| **EB.6** Connectors tab (allowlist, policies) | 2 | EB.5 | Toggle Canvas, GitHub, GCal; set requirements (2FA) |
| **EB.7** Audit Log tab (viewer, export) | 2 | Phase 4A complete | Display events, filter by admin/action/date, export JSON |
| **EB.8** Billing & Settings tabs | 3 | EB.1 | Billing history, payment method, SAML/SCIM config forms |

**Exit criteria**:
- ✅ Admin can login via org OIDC
- ✅ Member list displays; can invite new user
- ✅ Template builder creates rule; can apply to team
- ✅ Audit log shows admin actions
- ✅ SAML metadata download works

---

### Phase 4C: iOS SSO & Org Integration

**Effort**: 10–15 agent-batches (~20–30 min)

**Deliverables**:
1. SSO login screen (OIDC / SAML option)
2. Org rules + templates in iOS app
3. Org-scoped dashboard (read-only for delegated admins)
4. Org-scoped notifications
5. Personal ↔ Org account linking (merge flow)

**Subtasks**:

| Task | Est. Batches | Depends On | Notes |
|------|--------|-----------|-------|
| **EC.1** SSO login screen (new entry point) | 2 | EA.6 | "Sign in with SSO" button, org domain input |
| **EC.2** OIDC flow in iOS (ASWebAuthenticationSession) | 2 | EA.6 | Call WorkOS SDK, handle callback, store session token |
| **EC.3** Org rules + templates sync (Loro + CloudKit) | 2 | Phase 2 complete, EA.7 | Sync org zone, display org rules in Rules tab |
| **EC.4** Org dashboard tab (admin-only view) | 2 | EB.3, EB.4 | Show member list, template status (read-only) |
| **EC.5** Org-scoped notifications | 1 | Phase 1 complete | Send notifications for template updates, policy changes |
| **EC.6** Personal ↔ Org account merge (UX) | 2 | EA.4 | Detect duplicate email, show merge prompt, link accounts |

**Exit criteria**:
- ✅ iOS app SSO login works with org OIDC
- ✅ Org rules sync + display in iOS
- ✅ Org admin sees member list in iOS
- ✅ Personal account can merge with org account

---

### Phase 4D: Compliance, Testing & Launch

**Effort**: 10–20 agent-batches (~20–40 min)

**Deliverables**:
1. SOC 2 Type II audit prep (docs, controls)
2. COPPA & FERPA compliance (for school deployments)
3. SCIM interop testing (Okta, Azure AD, Google Workspace)
4. E2E migration scenario testing (personal → enterprise)
5. Admin portal E2E tests
6. Enterprise launch marketing + documentation

**Subtasks**:

| Task | Est. Batches | Depends On | Notes |
|------|--------|-----------|-------|
| **ED.1** SOC 2 audit prep (CC & A controls) | 2 | All Phase 4A–4C | Document encryption, access control, audit logs |
| **ED.2** COPPA & FERPA docs (for school tier) | 1 | All | Parental consent, data minimization, deletion policies |
| **ED.3** SCIM interop tests (3 IdPs) | 3 | EA.3, EA.4 | Test user/group sync with Okta, Azure, Google |
| **ED.4** E2E migration tests (personal → org) | 2 | EC.6 | Test merge flow, data retention, billing transition |
| **ED.5** Admin portal E2E tests (Playwright) | 2 | EB.1–EB.8 | Login, create org, invite user, manage templates, view audit |
| **ED.6** Performance testing (SCIM sync, query latency) | 2 | All Phase 4A–4C | Benchmark SCIM bulk provisioning (1000+ users) |
| **ED.7** Enterprise launch doc + marketing | 2 | All | Admin onboarding guide, feature overview, pricing page |

**Exit criteria**:
- ✅ SOC 2 controls documented + tested
- ✅ SCIM sync works with 3+ IdPs
- ✅ E2E migration test passes (personal data retained)
- ✅ Admin portal E2E tests pass
- ✅ Enterprise tier live on web + App Store

---

## Dependency Graph & Critical Path

### Phase 3 Critical Path

```
DA.1 (schema) → DA.2 (org table)
                    ↓
        DA.3 (org_id columns) → DA.4 (CloudKit zones)
            ↓
        DA.6 (templates), DA.5 (aggregation)
            ↓
        DA.7 (Guardian API)
            ↓
        IB.1, IB.3, IB.5 (iOS UI)
            ↓
        IB.2 (Guardian stats display)
            ↓
        EC.1-EC.6 (Child-safety enforcement)
            ↓
        RD.1-RD.6 (Testing & release)

Total: ~50–70 batches, ~100–140 min
```

### Phase 4 Critical Path

```
Phase 3 complete (org_id columns available)
        ↓
EA.1 (org schema) → EA.2 (CloudKit zones) → EA.3–EA.8 (SCIM, WorkOS)
        ↓
EB.1 (Next.js + auth) → EB.2–EB.8 (Portal tabs)
        ↓
EC.1–EC.6 (iOS SSO & org sync)
        ↓
ED.1–ED.7 (Compliance & launch)

Total: ~65–90 batches, ~130–180 min (parallel batches reduce wall-clock)
```

---

## Prerequisites & Blockers

### Phase 3 Prerequisites

- ✅ Personal tier stable (Phase 1 complete)
- ✅ Multi-device sync architecture (Phase 2 MVP complete)
- ⚠️ CloudKit schema finalized (currently iterating)
- ✅ SQLite schema v3 (audit, events, rules, wallets, penalties)

**Blockers**: None. Phase 3 is independent; Phase 2 sync not required (can ship Phase 3 before Phase 2 is complete).

### Phase 4 Prerequisites

- ✅ Phase 3 complete (org_id columns in place)
- ⚠️ Multi-device sync stable (Phase 2 MVP should be complete)
- ✅ WorkOS partnership agreement signed (legal)
- ⚠️ OAuth app registration (Canvas, GitHub, GCal)
- ⚠️ App Store Enterprise agreement (for SSO support)

**Blockers**:
- WorkOS legal agreement (~2 weeks)
- App Store review (Enterprise entitlement, ~1 week)
- SCIM interop testing (currently unknown IdP landscape)

---

## Rollout Strategy

### Family Sharing (Phase 3)

1. **Week 1–2**: Ship Phase 3A (backend) on internal staging
2. **Week 2–3**: Ship Phase 3B (iOS dashboard) to TestFlight
3. **Week 3**: Ship Phase 3C (child-safety) to TestFlight
4. **Week 4**: Complete Phase 3D (testing, COPPA review)
5. **Week 5**: General release (App Store, $14.99/mo)

**Adoption target**: 5–10% of personal users by end of Q2 2026.

### Enterprise SSO (Phase 4)

1. **Week 1–2**: Ship Phase 4A (backend + SCIM) on internal staging + pilot org (internal Acme test)
2. **Week 2–3**: Ship Phase 4B (admin portal) to internal admins
3. **Week 3–4**: Ship Phase 4C (iOS SSO) to TestFlight
4. **Week 4**: Complete Phase 4D (compliance, SCIM interop testing)
5. **Week 5–6**: Enterprise launch (closed beta, 3–5 orgs)
6. **Week 7**: General release ($49.99/user/mo)

**Adoption target**: 20–50 enterprise orgs by end of 2026.

---

## Resource Allocation

### Assumed Availability

- **Rust backend**: 3–4 concurrent agents (Phase 4A tasks)
- **iOS/Swift**: 2–3 concurrent agents (Phase 3B, Phase 4C)
- **Web/Next.js**: 2–3 concurrent agents (Phase 4B)
- **DevOps/QA**: 1–2 agents (testing, monitoring, compliance)

### Parallelization Strategy

**Phase 3**: Sequential phases 3A → 3B → 3C → 3D (mandatory dependencies)

**Phase 4**: Parallel tracks:
- **Track 1 (Backend)**: EA.1–EA.8 (run in parallel)
- **Track 2 (Web)**: EB.1–EB.8 (run in parallel after EB.1 auth complete)
- **Track 3 (iOS)**: EC.1–EC.6 (run in parallel after EA.6 WorkOS complete)
- **Track 4 (Compliance)**: ED.1–ED.7 (run in parallel throughout Phase 4)

Parallel execution reduces Phase 4 wall-clock from 130–180 min to ~80–120 min (3–4 concurrent agent batches).

---

## Success Criteria

### Phase 3 Success

- ✅ 5+ families created in beta
- ✅ Zero data isolation bugs (Guardian cannot see user audit)
- ✅ Child-account enforcement tested & working
- ✅ TestFlight feedback: <2% crash rate
- ✅ COPPA compliance docs approved by legal

### Phase 4 Success

- ✅ 3–5 enterprise pilot orgs (Acme test, 2 beta customers)
- ✅ SCIM provisioning tested with Okta + Azure AD
- ✅ Admin portal usable by non-technical admins
- ✅ E2E migration test passes (personal → enterprise)
- ✅ SOC 2 controls documented + tested

---

## Post-Launch Roadmap

### Phase 4.1: Advanced Enterprise Features

- **Org announcements** (admin broadcasts wellness tips)
- **Team leaderboards** (opt-in focus-time rankings by department)
- **Slack integration** (weekly team stats in Slack)
- **Data export** (org admin exports org data for BI)

### Phase 5: Mobile Expansion (Android)

- Port Phase 3 + Phase 4 features to Android
- Use same Rust FFI core (no reimplementation)
- Estimate: 40–50 agent-batches (~80–100 min)

