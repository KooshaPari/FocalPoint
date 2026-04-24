# FocalPoint Enterprise SSO & Organization Tier

## Overview

This document specifies FocalPoint's Enterprise tier, enabling organizations to deploy FocalPoint with single sign-on (SAML 2.0 + OIDC), automated user provisioning (SCIM 2.0), and organization-scoped data isolation at **$49.99/user/month**. An admin portal grants IT leaders visibility into organizational adoption, policy enforcement, and connector governance. The tier is designed for mid-market tech companies, schools, and enterprises seeking to improve team focus and reduce context-switching costs.

---

## Strategic Rationale

**Enterprise market**: Organizations are adopting focus management as a wellness benefit (alongside mental health, fitness stipends). FocalPoint's Enterprise tier positions it as:

1. **Identity integration**: Admins use their existing IdP (Okta, Azure AD, Google Workspace, Auth0) without asking users to remember another password
2. **Automated provisioning**: New hires are automatically added to FocalPoint via SCIM; off-boarding removes access
3. **Data isolation**: Each organization's data is isolated in its own CloudKit zones; admins can enforce policy templates across teams
4. **Compliance ready**: SOC 2 Type II path, audit logging, admin consent flows, data residency guarantees
5. **Team adoption**: Org-wide rules (e.g., "No Slack notifications after 6 PM") boost adoption and wellness ROI

**Competitive advantage**: Most focus-time apps are single-user. FocalPoint's multi-device sync + CRDT + event sourcing provides a foundation for rich organizational analytics and compliance that other tools cannot match.

---

## Business Model

### Pricing

| Tier | Price | Scope | Admin Portal | IdP | SCIM | Data Isolation |
|------|-------|-------|--------------|-----|------|---|
| **Personal** | $9.99/mo | 1 user | — | — | — | Personal zone |
| **Family** | $14.99/mo | 6 users | Family dashboard | — | — | Family + personal zones |
| **Enterprise** | $49.99/user/mo | Unlimited | Org admin console | SAML 2.0 + OIDC | ✅ | Org zones + personal |

### Feature Parity

- Enterprise users inherit all Personal + Family features (multi-device sync, CRDT, rules engine, rituals)
- Additional enterprise features: org-scoped templates, admin console, bulk policy management, audit logs

---

## Identity & Access Management

### SAML 2.0 & OIDC Integration

**Architecture**: FocalPoint acts as a service provider (SP) or OIDC relying party (RP). Organizations configure their IdP to trust FocalPoint.

#### SAML 2.0 Flow

1. **Metadata Exchange**:
   - Org admin provides IdP metadata (or FocalPoint SP metadata)
   - FocalPoint SP endpoint: `https://focalpoint.app/saml/metadata`
   - IdP provides: certificate, SSO URL, entity ID

2. **User Login**:
   - User taps "Sign in with SSO" on FocalPoint app or web
   - Redirected to IdP login (e.g., Okta login page)
   - After auth, IdP posts SAML assertion to `https://focalpoint.app/saml/acs`
   - FocalPoint validates signature, creates/updates user account
   - User session established; redirected to app

3. **Attributes Passed**:
   - `email` (required)
   - `givenName`, `sn` (optional, for profile)
   - `uid` / `employeeId` (optional, for mapping to SCIM)
   - `department`, `title` (optional, for org structure)
   - `birthDate` (optional; if under-13, requires special consent)

#### OIDC Flow

**Alternative to SAML** (easier for modern IdPs like Google Workspace, Azure AD):

1. **Configuration**:
   - Org admin provides issuer URL, client ID, client secret
   - FocalPoint redirects to IdP authorization endpoint
   - After approval, IdP returns ID token + access token

2. **User Login**:
   - User taps "Sign in with OIDC"
   - Redirected to IdP consent screen
   - After approval, FocalPoint exchanges code for token
   - ID token decoded, user account created/updated
   - Session established

**Supported IdPs** (tested):
- Okta
- Azure AD (Microsoft Entra)
- Google Workspace
- Auth0
- Keycloak (self-hosted)

### Implementation via WorkOS

**Recommendation**: Partner with **WorkOS** (or **Clerk**, as fallback) to abstract SAML/OIDC complexity.

- **WorkOS Organizations API** handles SAML/OIDC setup, metadata management, assertion validation
- **FocalPoint integration**: Call WorkOS to validate assertion → returns org + user + attributes
- **Cost**: WorkOS charges per organization ($x/mo), but eliminates custom SAML parsing (security risk)
- **Alternative**: In-house with `samael` (Rust SAML library) + `openidconnect` crate, but higher maintenance

---

## SCIM 2.0 Provisioning

### Overview

SCIM (System for Cross-Domain Identity Management) automates user lifecycle:

- **Create**: New hire joins company → IdP creates user → SCIM push to FocalPoint → account auto-created
- **Update**: Employee changes department → SCIM updates FocalPoint
- **Deactivate**: Employee leaves → IdP deactivates user → SCIM deactivates FocalPoint account (no data deletion)

### SCIM Endpoints

**FocalPoint SCIM Service Root**: `https://focalpoint.app/scim/v2/`

| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/ServiceProviderConfig` | Advertise SCIM features |
| GET | `/Schemas` | List supported schemas |
| GET | `/Users` | List all org users (paginated) |
| POST | `/Users` | Create new user |
| GET | `/Users/:id` | Get user details |
| PUT | `/Users/:id` | Replace user |
| PATCH | `/Users/:id` | Update user fields |
| DELETE | `/Users/:id` | Deactivate user (soft delete) |
| GET | `/Groups` | List org groups/departments |

### SCIM User Schema

**Minimal payload** (IdP → FocalPoint):

```json
{
  "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
  "id": "okta-12345",
  "externalId": "emp-67890",
  "userName": "alice@company.com",
  "name": {
    "givenName": "Alice",
    "familyName": "Chen"
  },
  "emails": [
    {
      "value": "alice@company.com",
      "primary": true
    }
  ],
  "active": true,
  "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User": {
    "department": "Engineering",
    "manager": "bob@company.com",
    "employeeNumber": "67890"
  }
}
```

### SCIM Authentication

- IdP provides bearer token (long-lived, rotate annually)
- FocalPoint validates token on every request
- Token stored in org settings; admins cannot see it (encrypted at rest)

### Deprovisioning Workflow

When IdP marks user as `"active": false`:

1. FocalPoint receives PATCH `/Users/:id` with `active: false`
2. User account marked as `deactivated_at` (soft delete, no data purge)
3. User cannot login (SSO + local password disabled)
4. Org admin can reactivate within 90 days (grace period)
5. After 90 days, user data is auto-purged (or manually by org admin)

---

## Organization Data Model

### Schema Changes

**Users table additions**:
```sql
ALTER TABLE users ADD COLUMN (
  org_id UUID,              -- Organization ID (NULL for personal users)
  org_role ENUM('admin', 'member'),  -- Role within org
  created_by_scim BOOLEAN,  -- True if created via SCIM provisioning
  deactivated_at TIMESTAMP, -- For soft-delete on offboarding
  external_id VARCHAR,      -- IdP-provided ID (e.g., Okta uid)
  department VARCHAR,       -- From SCIM extension
  manager_email VARCHAR     -- From SCIM extension
);
```

**Rules table additions**:
```sql
ALTER TABLE rules ADD COLUMN (
  org_id UUID,              -- Owner org (NULL for personal rules)
  is_org_template BOOLEAN,  -- True if org-wide policy
  org_enforcement_level ENUM('required', 'suggested', 'optional')
);
```

**Tasks, Wallet, Penalties, Audit**: Add `org_id` similarly.

### Data Isolation

**Three-tier model**:

1. **Personal Data** (`org_id IS NULL`):
   - User's own rules, tasks, wallet, audit
   - No visibility to org admin
   - Syncs via personal CloudKit zone

2. **Organization Data** (`org_id = <org-uuid>`):
   - Org-wide templates, policies, team stats
   - Visible to org admins and members
   - Syncs via organization CloudKit zone

3. **Shared Rules** (org template applied to personal rules):
   - Org admin publishes template → members can apply
   - Applied rule gets `org_id` AND `is_org_template = true`
   - Member can customize applied rule (child rule inherits parent template ID)

### CloudKit Organization Zones

**Per-organization structure**:

```
Organization Zone (org-<org-id>)
├── OrgTemplate records (shared policies, rules)
├── OrgSettings records (admin config, SCIM endpoint, enrolled users)
├── OrgAuditLog records (admin actions, policy changes, offboarding events)
└── OrgStats records (aggregate weekly/monthly adoption, focus time, penalties)

Member Personal Zone (user-<user-id>)
├── Rule records (personal + applied org templates)
├── Task records
├── Wallet, Penalty, AuditRecord (personal)
└── SharedRuleRef records (links to org-applied templates)
```

**Zone Sharing**:
- Org zone is shared with all org members via CloudKit bulk share
- Members have read-only access to OrgTemplate + OrgStats
- Members have read/write access to their own personal zones
- Org admin has read-only access to all member zones (via background service, not direct CloudKit)

---

## Admin Portal

### Web-Based Console

**URL**: `https://focalpoint.app/admin/orgs/<org-id>/`

**Authentication**: SAML/OIDC with `org-admin` or `org-owner` role claim

**Dashboard Views**:

#### 1. Overview Tab
- **Org Name**, **Subscription Status** (Active, Trial, Expired)
- **Member Count**: 47 of 100 licensed
- **Adoption Metrics** (past 7 days):
  - Active users: 35 (74%)
  - Avg focus time per user: 12.3 hours
  - Avg daily streak: 4.2 days
  - Total rules created: 89
- **Quick Actions**: "Invite Bulk Users" (CSV upload), "Manage Templates", "Settings"

#### 2. Members Tab
- **Table**: User, Email, Department, Status, Last Active, Actions
- **Filters**: Status (active/deactivated), department, role
- **Bulk Actions**: Deactivate selected, change role, export to CSV
- **Invite**: Paste email list or SCIM auto-sync indicator ("Syncing from Okta")

#### 3. Templates & Policies Tab
- **Table**: Template Name, Created By, Applied To (count), Status, Actions
- **Create Template**: Multi-rule builder (same UX as iOS, but web-based)
- **Assign to Team**: Pick template → select members/departments → confirm
- **Versioning**: "Update Template" → bump version → notify members of change
- **Enforcement Level**: Required, Suggested, or Optional

#### 4. Connectors Tab
- **Allowlist**: Toggle which connectors org members can use
  - Canvas (default allowed)
  - GitHub (allow? require 2FA?)
  - GCal (default allowed)
  - Custom integrations (require approval)
- **Policies**: E.g., "GitHub requires 2FA for all members"

#### 5. Audit Log Tab
- **Table**: Timestamp, Admin, Action, Target, Details
- **Sample actions**:
  - "Created template 'No Slack After 6PM'"
  - "Deactivated user alice@company.com"
  - "Updated org settings: max focus session = 90 min"
  - "Activated 23 users via SCIM sync"
- **Filters**: Date range, action type, admin
- **Export**: JSON or CSV (auditor-friendly)

#### 6. Billing & Usage Tab
- **Current Plan**: Enterprise, 47 active users @ $49.99/mo
- **Monthly Cost**: $2,349.53
- **Overages**: Auto-add users up to next tier
- **Invoice History**: PDF download
- **Payment Method**: Card on file, edit button

#### 7. Settings Tab
- **Org Name, Logo, Contact Email**
- **SAML Configuration**:
  - IdP Metadata URL (paste or upload XML)
  - Entity ID, SSO URL, Logout URL
  - "Test Connection" button
  - "Download SP Metadata" for IdP upload
- **SCIM Configuration**:
  - SCIM Service Root: `https://focalpoint.app/scim/v2/`
  - Bearer Token (regenerate, not viewable)
  - Last Sync: timestamp
  - Sync Frequency: "Real-time" or "Nightly"
- **Data Residency**: "EU" / "US" (if supported)
- **Admin Notifications**: "Email on bulk actions", "Weekly adoption summary"

---

## Compliance & Security

### SOC 2 Type II Readiness

**Checklist for Phase 4** (before general availability):

- ✅ **CC6.1**: Data encryption at rest (CloudKit + SQLite encryption)
- ✅ **CC6.2**: Data in transit (TLS 1.3)
- ✅ **CC7.2**: User authentication (SAML/OIDC, MFA support)
- ✅ **CC9.1**: Logical access control (SCIM role-based)
- ✅ **A1.2**: Detailed audit logs (all admin actions, user mutations)
- 🟡 **CC8.1**: Change management (deployment runbook, approval process)
- 🟡 **CC9.2**: Account management (provisioning / deprovisioning workflow docs)

**Post-Phase 4**:
- Achieve SOC 2 Type II certification (via third-party auditor)
- Publish compliance report to enterprise customers

### COPPA Compliance

**Rules for organizations with under-13 users**:

- Org admin must sign COPPA acknowledgment
- Under-13 accounts created via SCIM must include parental consent (handled offline; org admin confirms)
- Child-safety defaults (focus duration caps, restricted hours) auto-applied
- Org admin cannot access child audit logs (privacy-preserving)

### Data Residency

**Phase 4 only** (if needed):
- EU customers: CloudKit zones in EU data center
- Feature gate: `org.data_residency = "EU" | "US"`

---

## Migration Path: Personal → Enterprise

### Scenario: Alice Joins Acme Corp

**Current state**: Alice has a Personal ($9.99/mo) FocalPoint account with 5 rules, 12 tasks, 45-day streak.

**Migration flow**:

1. **Admin invites Alice via SCIM**:
   - Okta user `alice@acme.com` created
   - SCIM push: Alice's profile sent to FocalPoint
   - FocalPoint receives: email, givenName, department

2. **FocalPoint detects duplicate email**:
   - Existing Personal account: alice@example.com (old personal email)
   - Incoming SCIM: alice@acme.com (org email)
   - System prompts: "Link accounts? Or create new org account?"

3. **Alice signs in (first time at Acme)**:
   - Redirected to Okta SSO
   - After auth, FocalPoint shows: "Your personal account (45-day streak, 5 rules) can be merged into your Acme org account."
   - Options:
     - A) "Merge & keep my personal data" → Org admin cannot see personal rules; personal zone remains private
     - B) "Create new org account" → Start fresh with org templates; old personal account archived

4. **Alice chooses A (Merge)**:
   - FocalPoint creates org account for Alice
   - Personal data migrated to personal zone (private)
   - Org data synced to org zone
   - Streak carries over (merged historical audit)
   - Old Personal subscription cancelled; Alice charged $49.99/mo via org

5. **Alice leaves Acme** (future):
   - Okta deactivates alice@acme.com
   - SCIM PATCH: alice deactivated
   - FocalPoint: org account locked; personal account remains (if merged)
   - After 90-day grace, org data purged; personal data retained

---

## Implementation Phases

(See `docs/planning/family_enterprise_implementation_phases.md` for detailed breakdown.)

**Phase 4A — Backend & Data Model** (~25–30 agent-batches, 75–120 min):
- Add `org_id` to user, rule, task, wallet, penalty, audit tables
- Implement organization zone creation + sharing
- Build SCIM service (endpoints, CRUDL logic, deprovisioning)
- WorkOS integration for SAML/OIDC

**Phase 4B — Admin Portal (Web)** (~20–25 agent-batches, 60–90 min):
- Dashboard UI (7 tabs)
- Member management + bulk actions
- Template builder + assignment
- SAML/SCIM configuration console
- Audit log viewer + export

**Phase 4C — App Integration** (~10–15 agent-batches, 30–50 min):
- SSO login screen (OIDC / SAML option)
- Org rules + templates in iOS app
- Org dashboard (read-only admin view for delegated admins on iOS)
- Org-scoped notifications

**Phase 4D — Compliance & Testing** (~8–12 agent-batches, 25–40 min):
- COPPA review + docs
- SOC 2 audit prep
- SCIM interop testing (Okta, Azure AD, Google Workspace)
- E2E migration scenario testing

---

## Future Extensions

- **Single Sign-Out (SLO)**: SAML logout propagates to FocalPoint
- **Org Announcements**: Admin broadcasts focus challenges, wellness tips
- **Team Leaderboards**: Dept-level focus-time rankings (opt-in)
- **Analytics Dashboard**: Org-level insights (ROI on focus time, productivity gains)
- **Integrations**: Slack notifications ("Your team's focus time: 120 hours this week"), Asana task sync
- **Multi-tenant Data Export**: Admins export org data for BI platforms

