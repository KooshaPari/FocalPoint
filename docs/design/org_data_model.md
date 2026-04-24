# FocalPoint Organization Data Model

## Overview

This document specifies the schema and data-flow changes required to support Family Sharing (Phase 3) and Enterprise SSO (Phase 4). The key addition is an **organization ID** (`org_id`) that partitions data at the table level, enabling multiple data-isolation tiers: personal (user-scoped), family (up to 6 members), and organization (unlimited members with role-based access).

---

## Principles

1. **Backward compatibility**: Personal tier (Phase 1–2) continues unchanged (`org_id IS NULL`)
2. **Explicit nullability**: `org_id = NULL` means personal; `org_id = <uuid>` means family or org-scoped
3. **Hybrid users**: A single user can belong to multiple orgs (e.g., Alice at Acme AND consultant for BigCorp)
4. **Zone mapping**: Each org gets its own CloudKit zone; personal data stays in personal zones
5. **Privacy**: Org admins cannot access user audit chains or personal rules without explicit delegation

---

## Core Table Changes

### Users

**New columns**:

```sql
ALTER TABLE users ADD COLUMN (
  -- Organization membership (nullable)
  org_id UUID,                        -- NULL = personal user, <uuid> = member of org

  -- Organization metadata
  org_role ENUM('member', 'admin', 'owner'),  -- Role within org (NULL for personal)
  org_joined_at TIMESTAMP,            -- When user joined this org (NULL for personal)
  
  -- Provisioning
  external_id VARCHAR UNIQUE SPARSE,  -- IdP-provided identifier (e.g., Okta uid)
  created_by_scim BOOLEAN DEFAULT FALSE,  -- True if provisioned via SCIM
  
  -- Offboarding
  deactivated_at TIMESTAMP,           -- When org admin deactivated; NULL = active
  
  -- Department / team (for org context)
  department VARCHAR,                 -- From SCIM extension
  manager_email VARCHAR,              -- From SCIM extension
  
  -- Constraints
  FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE RESTRICT,
  CHECK (
    -- Personal: org_id NULL, org_role NULL, external_id NULL
    -- Org member: org_id NOT NULL, org_role NOT NULL, may have external_id
    (org_id IS NULL AND org_role IS NULL) OR 
    (org_id IS NOT NULL AND org_role IS NOT NULL)
  )
);

-- Indexes for org lookup
CREATE INDEX idx_users_org_id ON users(org_id);
CREATE INDEX idx_users_external_id ON users(external_id) WHERE external_id IS NOT NULL;
```

**Unique constraint**: A user can be member of multiple orgs, so `(user_id, org_id)` is unique, not `external_id` alone.

### Organizations

**New table**:

```sql
CREATE TABLE organizations (
  id UUID PRIMARY KEY,
  
  -- Identity
  name VARCHAR NOT NULL,
  slug VARCHAR UNIQUE NOT NULL,      -- For URL: /orgs/acme-corp
  logo_url VARCHAR,
  
  -- Org type
  org_type ENUM('family', 'enterprise') NOT NULL,
  
  -- Ownership
  owner_user_id UUID NOT NULL,       -- User who created org
  co_owner_ids JSONB,                -- [uuid, uuid, ...] for family co-organizers
  
  -- Subscription
  subscription_tier ENUM('family', 'enterprise') NOT NULL,
  subscription_status ENUM('active', 'trial', 'paused', 'expired') NOT NULL,
  billing_email VARCHAR,
  stripe_customer_id VARCHAR,
  
  -- Configuration
  idp_type VARCHAR,                  -- 'okta', 'azure-ad', 'google-workspace', etc. (NULL for family)
  idp_metadata_url VARCHAR,          -- For SAML metadata
  idp_secret_token VARCHAR ENCRYPTED, -- Bearer token for SCIM (encrypted at rest)
  scim_enabled BOOLEAN DEFAULT FALSE,
  
  data_residency VARCHAR DEFAULT 'us',  -- 'us' or 'eu'
  
  -- Audit
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  
  -- Constraints
  FOREIGN KEY (owner_user_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE INDEX idx_organizations_owner ON organizations(owner_user_id);
```

### Rules

**New columns**:

```sql
ALTER TABLE rules ADD COLUMN (
  org_id UUID,                        -- NULL = personal, <uuid> = org-scoped

  -- Template tracking
  is_org_template BOOLEAN DEFAULT FALSE,  -- True if org-wide policy
  org_enforcement_level ENUM('required', 'suggested', 'optional'),  -- How strictly enforced
  parent_template_id UUID,            -- If this rule is a customized org template
  
  -- Constraints
  FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
  FOREIGN KEY (parent_template_id) REFERENCES rules(id) ON DELETE SET NULL,
  CHECK (
    -- Personal: org_id NULL, is_org_template FALSE, parent_template_id NULL
    -- Org template: org_id NOT NULL, is_org_template TRUE
    -- Applied org template: org_id NULL, is_org_template FALSE, parent_template_id NOT NULL
    (org_id IS NULL AND NOT is_org_template AND parent_template_id IS NULL) OR
    (org_id IS NOT NULL AND is_org_template) OR
    (org_id IS NULL AND NOT is_org_template AND parent_template_id IS NOT NULL)
  )
);

CREATE INDEX idx_rules_org_id ON rules(org_id);
CREATE INDEX idx_rules_parent_template ON rules(parent_template_id);
```

**Migration logic**:
- Existing personal rules: `org_id = NULL, is_org_template = FALSE`
- Family tier: Rules authored by family member have `org_id = <family-org-id>, is_org_template = FALSE`
- Org templates: Created by org admin with `org_id = <org-id>, is_org_template = TRUE`
- Applied templates: User applies org template → creates child rule with `parent_template_id = <template-id>`

### Tasks

```sql
ALTER TABLE tasks ADD COLUMN (
  org_id UUID,                        -- NULL = personal, <uuid> = org-scoped
  
  FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
);

CREATE INDEX idx_tasks_org_id ON tasks(org_id);
```

**Scoping**: Tasks inherit org from parent rule. If rule is org-scoped, task is org-scoped.

### Wallet

```sql
ALTER TABLE wallets ADD COLUMN (
  org_id UUID,                        -- NULL = personal wallet, <uuid> = org-shared wallet
  
  FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
);

CREATE INDEX idx_wallets_org_id ON wallets(org_id);
```

**Family wallet model**:
- Each family has one shared wallet (`org_id = <family-id>`)
- All family members can spend/earn credits to this wallet
- Balance is aggregate (all members draw from same pool)

**Enterprise wallet model**:
- Each org has one shared wallet (`org_id = <org-id>`) for team-level stats
- Users also have personal wallets (`org_id = NULL`) for individual tracking
- Credits earned in personal wallet can optionally flow to org wallet

### Penalties

```sql
ALTER TABLE penalties ADD COLUMN (
  org_id UUID,                        -- NULL = personal, <uuid> = org-scoped
  
  FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
);

CREATE INDEX idx_penalties_org_id ON penalties(org_id);
```

**Scoping**: A penalty is org-scoped if triggered by an org-scoped rule.

### Audit Records

```sql
ALTER TABLE audit_records ADD COLUMN (
  org_id UUID,                        -- NULL = personal, <uuid> = org-scoped
  
  FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
);

CREATE INDEX idx_audit_records_org_id ON audit_records(org_id);
```

**Audit visibility**:
- Personal audit: only visible to the user and org admin (if user is org member)
- Org admin audit: all actions by org (templates, member changes, settings)
- Privacy: org admin CANNOT see user's personal audit chain (unless explicitly logged as org action)

### Cursors (Sync State)

```sql
ALTER TABLE cursors ADD COLUMN (
  org_id UUID,                        -- NULL = personal sync, <uuid> = org-scoped sync
  
  FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
);

CREATE INDEX idx_cursors_org_id ON cursors(org_id);
```

**Purpose**: Track CloudKit sync cursor per org zone.

---

## CloudKit Zone Architecture

### Personal User

**Zones created for every user**:

```
Personal Zone (user-<user-id>)
├── Rules (org_id IS NULL)
├── Tasks (org_id IS NULL)
├── Wallet (org_id IS NULL)
├── Penalties (org_id IS NULL)
├── AuditRecords (org_id IS NULL)
└── Sync cursors (org_id IS NULL)
```

**Accessibility**: User + root app (for sync).

### Family Organization

**Zones created when family is formed** (org_type = 'family'):

```
Family Private Zone (org-<org-id>)
├── FamilySettings (guardian config, member list, child-safety defaults)
├── FamilyTemplate (shared rule templates)
└── FamilyWeeklySummary (aggregate stats, cache)

Member Personal Zone (user-<user-id>) — unchanged
├── Rules (both org_id IS NULL AND parent_template_id NOT NULL)
├── Tasks (applied family templates)
├── Wallet (personal wallet; may reference org wallet for family summary)
├── AuditRecords (personal; org admin cannot access)
└── Cursors
```

**Sharing**:
- Family zone shared with all family members (read/write for templates, read-only for summaries)
- Personal zones remain private (org admin has background read-only access for summaries)

### Enterprise Organization

**Zones created when org is provisioned** (org_type = 'enterprise'):

```
Org Zone (org-<org-id>)
├── OrgSettings (SAML/SCIM config, member list, policies)
├── OrgTemplate (org-wide rule policies, templates)
├── OrgAuditLog (admin actions, user provisioning events, policy changes)
└── OrgStats (weekly/monthly adoption, focus time rollup, performance metrics)

Member Personal Zone (user-<user-id>) — unchanged
├── Rules (personal + applied org templates)
├── Tasks (personal + org-templated tasks)
├── Wallet (personal + optional org rollup)
├── AuditRecords (personal; org admin cannot access)
└── Cursors
```

**Sharing**:
- Org zone shared with all org members (read-only to templates/stats; write only for members with admin role)
- Personal zones remain private; org admin gets read-only background access for analytics

---

## Multi-Org User

A single user can belong to multiple organizations (e.g., employee + consultant):

**Example: Alice**
```
alice@example.com (Personal)
├── Rules (org_id IS NULL) — personal rules
├── Wallet (org_id IS NULL) — personal wallet

alice@acme.com (Enterprise)
├── Rules (org_id = <acme-org-id>) — applied org templates
├── org_role = 'member'
├── org_joined_at = 2026-04-01

alice@bigcorp.com (Enterprise)
├── Rules (org_id = <bigcorp-org-id>) — applied org templates
├── org_role = 'admin'
├── org_joined_at = 2025-08-15
```

**CloudKit sync**:
- Alice's device syncs 3 org zones (personal + acme + bigcorp)
- Each zone has independent cursors for tracking sync progress
- Conflicts resolved per-org (Loro CRDT)

---

## Data Isolation & Querying

### Query Examples

**Get user's personal rules**:
```sql
SELECT * FROM rules WHERE user_id = ? AND org_id IS NULL;
```

**Get all rules in an org** (as admin):
```sql
SELECT * FROM rules WHERE org_id = ? AND is_org_template = TRUE;
```

**Get user's rules in an org** (personal + applied templates):
```sql
SELECT * FROM rules 
WHERE user_id = ? 
  AND (org_id IS NULL OR parent_template_id IN (
    SELECT id FROM rules WHERE org_id = ? AND is_org_template = TRUE
  ));
```

**Get all users in an org**:
```sql
SELECT * FROM users WHERE org_id = ?;
```

**Org admin cannot query**:
```sql
-- FORBIDDEN: Returns user's personal rules
SELECT * FROM rules WHERE user_id = ? AND org_id IS NULL;

-- FORBIDDEN: Returns user's personal audit
SELECT * FROM audit_records WHERE user_id = ? AND org_id IS NULL;
```

### Access Control Rules

| Query | Personal User | Org Member | Org Admin |
|-------|---|---|---|
| See own rules | ✅ | ✅ (personal + org) | ✅ (all org rules) |
| See own audit | ✅ | ✅ | ❌ (cannot see) |
| See org templates | ❌ | ✅ (read-only) | ✅ (read/write) |
| See org audit | ❌ | ❌ | ✅ (admin-only actions) |
| See other users' personal data | ❌ | ❌ | ❌ |
| Invite users to org | ❌ | ❌ | ✅ |
| Deactivate user in org | ❌ | ❌ | ✅ |

---

## Migration Scenario: Personal → Family

**Alice creates a family account**:

1. **Create organization**:
   ```sql
   INSERT INTO organizations (id, name, slug, org_type, owner_user_id, subscription_tier)
   VALUES (gen_uuid(), 'Alice Family', 'alice-family', 'family', alice_id, 'family');
   ```

2. **Update Alice's user record**:
   ```sql
   UPDATE users 
   SET org_id = <family-id>, org_role = 'owner'
   WHERE id = alice_id;
   ```

3. **Invite family member (Bob)**:
   ```sql
   INSERT INTO users (email, org_id, org_role, created_by_scim)
   VALUES ('bob@family.local', <family-id>, 'member', FALSE)
   RETURNING id;
   ```

4. **Alice's personal rules remain personal**:
   ```sql
   -- No change; personal rules have org_id IS NULL
   SELECT * FROM rules WHERE user_id = alice_id AND org_id IS NULL;
   ```

5. **Family template created by Alice**:
   ```sql
   INSERT INTO rules (user_id, org_id, title, is_org_template, ...)
   VALUES (alice_id, <family-id>, 'Homework Block', TRUE, ...)
   RETURNING id;
   ```

6. **Bob applies family template**:
   ```sql
   INSERT INTO rules (user_id, org_id, parent_template_id, title, ...)
   VALUES (bob_id, NULL, <template-id>, 'Homework Block (custom)', ...)
   RETURNING id;
   ```

---

## Migration Scenario: Personal → Enterprise (SCIM)

**Okta provisions Alice for Acme**:

1. **SCIM POST /Users**:
   ```json
   {
     "userName": "alice@acme.com",
     "emails": [{"value": "alice@acme.com"}],
     "externalId": "okta-12345",
     "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User": {
       "department": "Engineering"
     }
   }
   ```

2. **FocalPoint detects duplicate email** (if Alice had personal account with alice@acme.com):
   - Prompt: "Merge personal + org accounts?"
   - If yes: Link accounts; set `org_id = <acme-id>, org_role = 'member'`

3. **Create org membership**:
   ```sql
   UPDATE users 
   SET org_id = <acme-id>, 
       org_role = 'member', 
       external_id = 'okta-12345',
       created_by_scim = TRUE,
       department = 'Engineering'
   WHERE email = 'alice@acme.com';
   ```

4. **Alice's personal rules remain personal**:
   - Stored with `org_id IS NULL`
   - Not visible to Acme admin

5. **Acme admin creates org template**:
   ```sql
   INSERT INTO rules (org_id, user_id, title, is_org_template, org_enforcement_level, ...)
   VALUES (<acme-id>, admin_user_id, 'No Slack After 6PM', TRUE, 'required', ...)
   RETURNING id;
   ```

6. **Alice's app syncs org template** (on next launch):
   - Receives org template from Acme zone
   - Can choose to apply or ignore (if enforcement = 'optional')
   - If applied, creates local rule with `parent_template_id = <template-id>`

---

## Sync & CRDT Integration

### Sync State per Zone

Each zone (personal, family, org) has independent sync cursors:

```sql
SELECT * FROM cursors WHERE user_id = alice_id AND org_id = <acme-id>;
```

Returns the last CloudKit cursor for Acme's org zone.

### Conflict Resolution

CRDT rules apply per organization:

- **Rules (LWW by updated_at)**: If Alice and Bob both edit a shared org rule, highest `updated_at` wins
- **Tasks (LWW + additive tombstones)**: Status changes are logged; last status is source of truth
- **Wallet (additive)**: Credits are monotonically incremented (never conflict)
- **Penalties (append-only)**: New penalties are always added; never removed (only cleared by guardian)

### Personal Zone Never Syncs Org Data

- Org rules do NOT appear in personal zone
- Family rules do NOT appear in personal zone
- Only personal rules (org_id IS NULL) are in personal zone
- Prevents confusion and reduces sync payload

---

## Deactivation & Offboarding

### Soft Delete (Org Deactivation)

When org admin deactivates a user:

```sql
UPDATE users 
SET deactivated_at = NOW() 
WHERE id = user_id AND org_id = <org-id>;
```

**Effects**:
- User cannot login via org SAML/OIDC
- User's org account marked as inactive (not deleted)
- Personal account (if separate) remains active
- Org data remains (for audit trail) for 90 days
- After 90 days, org data can be purged (if org admin approves)

### Data Retention

- **Personal tier**: Deleted on account deletion; after 30-day grace period, purged
- **Family tier**: Deleted when family dissolved; after 30-day grace, purged
- **Enterprise tier**: Deactivated on offboarding; after 90-day grace, org data purged (personal data if owned by user remains)

---

## Backward Compatibility

All existing queries continue to work:

```sql
-- Queries on personal data (no org_id column used)
SELECT * FROM rules WHERE user_id = ?;  -- Returns personal rules (org_id IS NULL)

-- New queries must explicitly filter org
SELECT * FROM rules WHERE user_id = ? AND org_id IS NULL;  -- Personal
SELECT * FROM rules WHERE org_id = ?;   -- Org-scoped
```

**Migration checklist**:
- Add `org_id` columns (nullable, no defaults)
- Add `CHECK` constraints to enforce valid combinations
- Add indexes on `(org_id)` and `(org_id, user_id)`
- Update queries to be explicit about org scope
- Test: personal tier queries MUST NOT break

