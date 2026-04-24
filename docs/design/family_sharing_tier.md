# FocalPoint Family Sharing Tier

## Overview

This document specifies FocalPoint's Family Sharing tier, enabling up to 6 family members to share a single Pro subscription at **$14.99/month**. A designated Guardian (parent) can monitor aggregate focus statistics and child-safety metrics for minors, while respecting privacy and age-appropriate transparency. Child accounts inherit stricter default rules and feature restrictions, with clear explanations to help kids understand their boundaries.

---

## Strategic Rationale

**Market context**: Apple's Family Sharing is the de facto family account model on iOS. FocalPoint's family tier capitalizes on this:

1. **Billing efficiency**: One subscription covers a household; reduces friction for parents buying for kids
2. **Shared infrastructure**: Family templates (reusable rule packs) reduce setup time across members
3. **Parental oversight**: Parents want aggregate visibility (e.g., "How much screen time across the family?") without invasive surveillance
4. **Child safety**: Stricter defaults (shorter focus windows, mandatory breaks) with explainable rules prevent misuse
5. **Data locality**: CloudKit's family-scoped zones keep family data within iCloud, no external server required

**Privacy principle**: Guardians see *aggregate* stats (total family focus time, average streak length, policy compliance) and *member-level summaries* (Alice's weekly average), but NOT the full audit chain (which specific rules were broken, when).

---

## Business Model

### Pricing Tiers

| Tier | Price | Users | Guardian | Templates | Connectors |
|------|-------|-------|----------|-----------|-----------|
| **Personal** | $9.99/mo | 1 | — | 5 custom | All (Canvas, GCal, GitHub) |
| **Family** | $14.99/mo | Up to 6 | 1 required | 15 (5 custom + 10 shared) | All; restrictions per role |
| **Enterprise** | $49.99/user/mo | Unlimited | Org admin | Unlimited | Org-approved only |

### Family Role Model

1. **Organizer** (Guardian):
   - Creates family account; invites up to 5 other family members
   - Can promote one other adult to co-Guardian
   - Sees aggregate family statistics
   - Can install/remove shared templates
   - Can restrict connectors (disable Canvas for under-13s)
   - Can reset child-account penalties (parental override)

2. **Adult Member**:
   - Full Personal tier features within family context
   - Can see aggregate family stats (opt-in)
   - Can create shared templates for family
   - Cannot see other adults' individual rules/tasks

3. **Minor Member** (under 18):
   - Inherits child-safety defaults (see below)
   - Cannot change core account settings without Guardian consent
   - Cannot remove family membership
   - Guardian sees weekly summary (focus time, streaks, penalties incurred)

---

## Feature Set

### Shared Packs & Templates

**Family Template Library**:
- Organizer can create rule packs tagged as "Family Shared"
- Examples: "After-School Focus", "Weekend Homework Block", "No-Phone Before Bed"
- Templates are private to the family; not published to app store
- Members can apply templates with one tap; customize afterwards
- Template versions tracked; family members notified when Organizer updates a shared template

**Inheritance Model**:
- Personal rules: member-only, cannot be shared
- Shared templates: Organizer-created, all members can apply
- Child accounts: auto-inherit Core template from family defaults on signup

### Guardian Dashboard

**Family View** (iOS app, new "Family" tab):

1. **Family Overview Card**:
   - Total members: "6 people"
   - Subscription status: "Family Tier (renews 2026-05-23)"
   - Storage: "12.5 GB of 200 GB used"

2. **Weekly Aggregate Stats**:
   - "Family focus time this week: 58 hours" (roll-up of all members' logged sessions)
   - "Average member streak: 8 days"
   - "Members on focus: 3 right now" (real-time count, anonymized)

3. **Member Summary Cards** (one per family member, scrollable):
   - Member name, avatar, age badge
   - "Alice: 14h focus time this week, 7-day streak"
   - Tap to see "Weekly Summary" (not detailed rule/task audit)
   - For minors: "2 penalties this week" (count only, no details)

4. **Actions**:
   - "Install Template" → pick from family library → push to one or all members
   - "Manage Family" → invite new member, remove member, revoke co-Guardian
   - "Settings" → control child-safety defaults, connector restrictions, penalties

### Child-Safety Defaults

**Automatic restrictions** for accounts created with Guardian consent as "minor":

1. **Focus Duration Caps**:
   - Max focus window: 60 minutes per session (vs. 120 min for adults)
   - Cooldown between sessions: 15 minutes mandatory (auto-enforced)
   - Auto-breakpoint: every 45 minutes, 5-minute mandatory break

2. **Restricted Hours**:
   - No focus sessions before 6 AM or after 10 PM
   - School-day lockout: 8 AM – 3 PM (with Guardian override available)
   - Weekend bonus: +30 min allowed per session

3. **Connector Restrictions** (Guardian-configurable):
   - Canvas OAuth: disabled by default (Guardian can enable)
   - GitHub OAuth: disabled by default
   - GCal integration: allowed (read-only scheduling advice)
   - Custom connectors: disabled until age 16

4. **Penalty Escalation**:
   - Shorter lockout windows (1 hour vs. 2 hours for rule breach)
   - Auto-escalate after 3 breaches in a week (vs. 5 for adults)
   - Hard penalties: cannot be manually cleared (Guardian override only)

5. **Explainability** (UX requirement):
   - Every restriction shows a reason: "Why can't I focus after 10 PM? Because your parents set a bedtime focus boundary to protect your sleep."
   - Rules for minors must include a brief explanation in kid-friendly language
   - Guardian can customize explanations per rule

---

## CloudKit Data Model

### Family Zone Architecture

CloudKit provides **three zone types** per family:

1. **Family Private Zone** (`family-<family-id>`):
   - Stores: shared templates, family settings, Guardian dashboard data (summaries only)
   - Visible to: all family members
   - Records: `FamilyTemplate`, `FamilySettings`, `FamilyAggregateStats`
   - Sync: bi-directional (any member can add a template)

2. **Member Private Zone** (`user-<user-id>`):
   - Stores: personal rules, tasks, wallet, penalties, full audit chain
   - Visible to: member only (+ Guardian sees summaries via roll-up service)
   - Records: `Rule`, `Task`, `Wallet`, `AuditRecord`, `Penalty`
   - Encryption: end-to-end (Guardian cannot decrypt)

3. **Family Aggregate Cache Zone** (`family-aggregate-<family-id>`):
   - Stores: pre-computed weekly/monthly summaries (denormalized for performance)
   - Visible to: all members
   - Records: `FamilyWeeklySummary`, `MemberWeeklySummary`
   - Rebuilt: nightly via background CloudKit trigger (or on-demand)
   - Purpose: Guardian dashboard queries hit cache, not raw audit records

### Record Schema Additions

**`FamilyTemplate`**:
```
id: UUID (primary)
family_id: UUID (foreign key)
creator_id: UUID (member who created)
name: String ("After-School Focus")
description: String
rules_json: String (serialized Rule array)
tags: [String] ("homework", "bedtime")
version: Int (incremented on update)
created_at: Timestamp
updated_at: Timestamp
```

**`FamilySettings`**:
```
id: UUID
family_id: UUID
organizer_id: UUID
members: [MemberRef]  // { user_id, name, role, age }
child_defaults: {
  max_focus_minutes: Int
  mandatory_break_minutes: Int
  restricted_hours: { start, end }
  escalation_threshold: Int
}
connector_restrictions: {
  canvas_enabled_for_minors: Bool
  github_enabled_for_minors: Bool
  custom_disabled_until_age: Int
}
created_at: Timestamp
```

**`FamilyWeeklySummary`** (aggregate, cache):
```
id: UUID
family_id: UUID
week_start_date: Date
member_count: Int
total_focus_minutes: Int
avg_streak_length: Float
members_on_focus_now: Int
timestamp: Timestamp (for cache expiry)
```

**`MemberWeeklySummary`** (per-member rollup):
```
id: UUID
family_id: UUID
member_id: UUID
week_start_date: Date
focus_minutes: Int
sessions_completed: Int
streak_length: Int
penalties_incurred: Int
conformance_score: Float (0–100, based on rule compliance)
timestamp: Timestamp
```

### Zone Sharing & Permissions

**CloudKit share model**:

- Organizer creates family account → creates Family Private Zone + Family Aggregate Cache Zone
- Organizer invites member → shares zones via CloudKit share invitation (email)
- Member accepts invite → gets read/write access to Family Private Zone
- Member's Personal Zone remains private; Guardian access is read-only (via background service, not direct CloudKit access)
- Zones are deleted when family is dissolved (Organizer action)

---

## Guardian Experience: Onboarding & Consent

### Family Account Creation Flow

1. **New User → "Create Account"**:
   - Option A: "Personal Tier ($9.99/mo)" → standard signup
   - Option B: "Family Tier ($14.99/mo)" → new flow

2. **Family Tier Setup (3 screens)**:
   - **Screen 1: "You're the Organizer"**
     - "Set up a family account. One subscription covers up to 6 family members."
     - "You'll manage settings, shared templates, and family privacy."
     - Button: "Create Family"
   
   - **Screen 2: "Add Family Members"**
     - "Invite family members via email (you can add them later)"
     - Text fields: member name, email, age (to auto-apply child defaults)
     - Checkboxes: "Invite as adult" or "Invite as minor"
     - Button: "Send Invites" (or "Skip for Now")
   
   - **Screen 3: "Child Safety Settings"**
     - "Choose default restrictions for minors in your family"
     - Toggles: max focus time, connector access, restricted hours
     - Example: "No Canvas OAuth for under-16s"
     - Button: "Create Family & Set Up Payment"

3. **Payment & Confirmation**:
   - Confirm $14.99/mo charge
   - Send invites via email (with link to accept)
   - Show "Family Created" success screen

### Minor Onboarding (Invited Member)

1. **Email Invite** + tap link in FocalPoint app
2. **Consent Flow** (2 screens):
   - **Screen 1: "Join [Guardian Name]'s Family"**
     - "You're invited to join a FocalPoint family account."
     - Show child-safety defaults: "Max 60-min focus sessions, no sessions after 10 PM"
     - "Your parent/guardian can see your weekly focus time and streaks."
     - Button: "Accept & Create Account"
   
   - **Screen 2: "Age Confirmation"**
     - If under 13: "Your parent needs to confirm your age with Family Link"
     - If 13+: "Confirm your birth date"
     - Button: "Confirm & Continue"

3. **First-Time Setup**:
   - QuickStart rules (inherited from family template)
   - Explanation screen: "Why does your account have these limits?" (with kid-friendly copy)

---

## Penalties & Enforcement

### Child-Account Penalties

**Enhanced Escalation for Minors**:

- **Breach of focus-duration cap**: 1-hour lockout (vs. 2-hour for adults)
- **3 breaches in 7 days**: auto-escalate to Semi penalty (requires Guardian manual reset)
- **5 breaches in 7 days**: Hard penalty (24-hour lockout, no override)

**Guardian Override** (on Family tab):
- Tap member card → "Penalties" → see locked-out sessions
- Button: "Clear Penalty" (admin override, logged as audit event `guardian_penalty_override`)
- Message to child: "Your parent cleared a penalty. Stay focused!"

### Transparency & Explainability

- **Every penalty** shows "Why was I locked out?" with rule reference
- **Example**: "You tried to start a 75-minute focus session, but your max is 60 minutes. You're now locked out for 1 hour."
- **Rule explanation** shown in child's language (e.g., "Your parents set a 60-minute max because shorter sessions help you stay focused without burnout.")

---

## Privacy & Audit

### What Guardian Can See

- ✅ Aggregate family stats (total focus time, average streaks, member count)
- ✅ Member-level summaries (Alice's weekly focus time, penalties count)
- ✅ Shared template library and usage
- ✅ Family member list and roles
- ✅ Guardian action history (templates installed, penalties cleared)

### What Guardian Cannot See

- ❌ Individual rules or tasks
- ❌ Specific focus sessions or timing
- ❌ Full audit chain (who did what, when)
- ❌ Canvas/GitHub integration details
- ❌ Personal notes or task descriptions

### Audit Logging for Guardians

- `guardian_invite_sent`: { family_id, invitee_email, role, timestamp }
- `guardian_template_installed`: { family_id, template_id, installed_to_member, timestamp }
- `guardian_penalty_override`: { family_id, member_id, penalty_id, reason, timestamp }
- `guardian_settings_changed`: { family_id, field, old_value, new_value, timestamp }

These events are logged in Family Private Zone and visible to all Organizers/co-Guardians.

---

## Implementation Phases

(See `docs/planning/family_enterprise_implementation_phases.md` for detailed phased breakdown.)

**Phase 3A — Backend & Data Model** (~20–25 agent-batches, 60–90 min):
- Add `family_id`, `role`, `member_list` to user schema
- Create CloudKit family zones (private + aggregate cache)
- Implement family-aware event sourcing (summary aggregation service)
- Build Guardian summary API (read-only access to member stats)

**Phase 3B — iOS Family UX** (~15–20 agent-batches, 45–75 min):
- New "Family" tab (dashboard, member cards, overview stats)
- Guardian onboarding flow (3 screens)
- Minor account onboarding + consent
- Child-safety defaults UI + rule explanations

**Phase 3C — Child-Safety Enforcement** (~8–12 agent-batches, 25–40 min):
- Child-account rule restrictions (focus duration, restricted hours)
- Escalation logic for penalties
- Guardian override flow
- Explanation overlays (UX)

**Phase 3D — Testing & Release** (~6–10 agent-batches, 20–30 min):
- E2E testing (family invite, sync, Guardian dashboard)
- Compliance review (COPPA for under-13 accounts)
- TestFlight rollout to family accounts
- Release notes & family onboarding guide

---

## Future Extensions

- **Shared Calendar View**: All family members' focus sessions on one calendar
- **Family Challenges**: "This week, let's log 50 hours of family focus time together"
- **Parental Controls Portal** (web): Manage family on desktop
- **Family Rituals**: Shared morning brief and evening shutdown (group intentions)
- **Peer Comparison** (opt-in): Streaks leaderboard within family

