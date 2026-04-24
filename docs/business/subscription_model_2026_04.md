# FocalPoint Subscription & Freemium Tier Model (2026-04)

## Executive Summary

FocalPoint v1 release requires a sustainable revenue model paired with a compelling free tier that preserves our local-first, no-lock-in promise. This document proposes a four-tier subscription model (`Free`, `Plus`, `Pro`, `Family`) using Apple's StoreKit 2 for native iOS IAP, with intentional cross-platform openness for future Android parity.

---

## Proposed Tier Structure

### Free Tier (Forever, No Credit Card)

**Positioning:** "Focus management, no paywall. Start here."

**Included Features:**
- Up to 3 custom rules (unlimited evaluation cadence)
- Up to 3 tasks/goals (unlimited tracking)
- 1 active connector at limited refresh cadence (Canvas, Google Calendar, or GitHub; refreshes every 4 hours)
- 25-minute focus sessions (single duration; no customization)
- 45-minute break timer (fixed)
- Basic Coachy: silent simlish animations, 3 static pose variants, no voice
- 7-day audit retention (older records auto-pruned; local export always available)
- Manual export: rule set, audit chain, rewards/penalties ledger (JSON/CSV)
- Core rule engine: all condition types, all connector-context variables, all penalty/reward mechanics

**What Stays Free Forever (Non-Negotiable):**
- Audit chain append-and-verify (tamper-evident proof)
- Export of audit + rules (no vendor lock-in)
- The rule evaluation engine itself (open Rust core)
- All 4 connectors at _limited cadence_ (first refresh free; then gated)
- Basic focus/break sessions (5-minute minimum to 60-minute maximum)
- Penalty/reward ledger (visible but limited cosmetics)

**Intended User:**
- Student validating FocalPoint workflows before commitment
- Adult prototyping rules without subscription friction
- Power user who wants audit + export but no connectors/sessions

---

### Plus Tier ($4.99/mo or $39.99/yr; ~50% discount on yearly)

**Positioning:** "Serious focus. Full connectors, custom sessions, sync."

**Included Features:**
- Unlimited custom rules
- Unlimited tasks/goals
- All 4 connectors (Canvas, Google Calendar, GitHub, + placeholder for one more) at full refresh cadence (15-min intervals during active hours)
- Custom focus durations: 5–180 minutes in 5-minute increments
- Custom break durations: 1–60 minutes
- Coachy voice: native AVSpeechSynthesizer (iOS), localized to user language, 3 daily nudge slots
- Live Activity support: focus session progress + active rule state on lock screen
- HomeKit widget (optional): at-a-glance rule status on home screen
- 90-day audit retention (local; export historical data anytime)
- CloudKit sync (optional): rules + focus templates + reward tracking across user's Apple devices (requires iCloud sign-in)
- Penalty/reward dashboard: weekly breakdown, streak tracking, net earned vs. spent

**Intent:**
- Serious students or adults committing to FocalPoint as daily driver
- Cross-device sync for iPad/Mac rule management
- Proactive coaching voice for habit formation

---

### Pro Tier ($9.99/mo or $79.99/yr; ~50% discount on yearly)

**Positioning:** "Advanced focus. Marketplace, analytics, priority support."

**Included Features:**
- Everything in Plus, plus:
- ElevenLabs premium voice synthesis: hyper-realistic voice for nudges + rule explanations, custom tone selection
- Focus session template marketplace: pre-built routines (Pomodoro+study break, deep work 90-min block, sleep-debt recovery)
- Advanced analytics dashboard:
  - Daily/weekly/monthly rule adherence heat map
  - Connector data source contribution (which connector fired most rules?)
  - Penalty severity over time (trend: getting stricter or more lenient?)
  - Reward efficacy (which rewards correlate with highest d-next adherence?)
- Always-On proactive nudges: 2-hour window before rule violation predicted, send iOS notification ("Your assignment is due in 2h; study block unlocks")
- Custom Coachy cosmetics: outfit selections, pose variants, animated accessories (hats, glasses, etc.)
- 180-day audit retention
- Priority support: 24-hour response time, early access to beta features

---

### Family Tier ($14.99/mo; no annual discount offered to discourage lock-in)

**Positioning:** "Parental oversight. Manage up to 5 family members."

**Included Features:**
- Everything in Pro for the account owner
- Up to 5 family member sub-accounts (requires Apple Family Sharing enrollment)
- Parental dashboard: aggregate view of all family members' rule compliance
  - Child has 3 rules? See which fired today; see audit trail (FERPA-compliant: only rule names + timestamps)
  - Do not expose child's data to other children (sibling isolation)
- Family-shared rule templates: owner publishes templates; children inherit + customize
- Parent-set overrides: temporarily relax or tighten rules for a child (e.g., "exam week; unlock 2 extra hours")
- Weekly family digest: email summary of all members' focus metrics (owner only)
- Child safety: rule cannot be disabled by child; parent-enforced minimum penalty (cannot be erased by app restart)
- 365-day audit retention (family archive)

**Non-Goals (out of scope):**
- Real-time monitoring of child activity (Apple Family Sharing does that; FocalPoint focuses on rules, not spyware)
- Cross-device lock (use iOS/macOS built-in family controls for that)
- Grades sync (intentionally separate from family data; no 3rd-party integration with school systems v1)

---

## Pricing & Duration Rationale

| Tier | Monthly | Annual | Discount | Total Annual |
|------|---------|--------|----------|--------------|
| Free | $0 | N/A | N/A | $0 |
| Plus | $4.99 | $39.99/yr | 33% | $39.99 |
| Pro | $9.99 | $79.99/yr | 33% | $79.99 |
| Family | $14.99 | N/A | —— | $179.88 |

**Rationale:**
- **Free:** No expiry. Intentionally limited; converts on utility + habit formation (3→5 rules signals need for Plus).
- **Plus:** Undercut productivity SaaS ($10–20/mo); aligned with language-learning apps (Duolingo $6.99) + focus app (Forest $1.99/mo). Annual discount incentivizes commitment without over-discounting (33% matches industry standard).
- **Pro:** Premium voice + analytics justify 2× Plus cost; positioned as "power user" tier.
- **Family:** Flat monthly (no discount) to signal it's a fundamentally different product; ~$3/person if 5 members splits favorably vs. individual Plus subscriptions.

---

## Revenue Forecasting (Illustrative; Not Binding)

**Assumptions:**
- v1 launch: 5,000 users (Test Flight + App Store first 3 months)
- Free-to-Plus conversion: 5% (235 → 14 × $4.99 = ~$67/mo)
- Plus-to-Pro conversion: 10% of Plus subscribers (1 × $9.99 = ~$10/mo)
- Family adoption: 2% of total (100 × $14.99 = ~$1,500/mo)

**Conservative MRR (Month 3):** ~$1,577 / month
**Conservative ARR:** ~$18,924 / year

(Real numbers will depend on viral growth, retention, and pricing elasticity; this is a floor for break-even server costs.)

---

## Feature Gates & Enforcement

Every gated feature includes:

1. **Server-side source of truth:** StoreKit 2 receipt validation on Cloudflare Worker or lightweight Rust endpoint
2. **On-device cache:** `EntitlementStore` trait holds current tier (refreshed every app launch + every 24h)
3. **Fallback logic:** If receipt verification fails (no network), assume user's last known tier (cached; expires after 30 days offline)

### Feature Gate Table

| Feature | Free | Plus | Pro | Family | Enforcement |
|---------|------|------|-----|--------|-------------|
| Max rules | 3 | Unlimited | Unlimited | Unlimited | UI prevents 4th rule; `can_add_rule()` gate |
| Max tasks | 3 | Unlimited | Unlimited | Unlimited | UI prevents 4th task; `can_add_task()` gate |
| Connectors active | 1 (4h cadence) | 4 (15m cadence) | 4 (15m cadence) | 4 (15m cadence) | `connector_refresh_allowed()` + rate limit |
| Focus duration range | 25 / 45 | 5–180 min | 5–180 min | 5–180 min | `validate_session_duration()` gate |
| Coachy voice | No | AVSpeechSynthesizer | ElevenLabs | ElevenLabs | Voice enum in cache; fallback to silent |
| Widget support | No | Yes | Yes | Yes | SwiftUI `@Environment(\.entitlements)` check |
| Live Activity | No | Yes | Yes | Yes | Feature gate at session start |
| Audit retention | 7d | 90d | 180d | 365d | `should_prune_audit()` based on tier |
| Proactive nudges | No | 3/day | 24h-ahead | 24h-ahead | `nudge_cadence()` returns count |
| Custom cosmetics | No | No | Yes | Yes | UI lists available poses; gate unlock |
| Template marketplace | No | No | Yes | Yes | `marketplace_access()` returns bool |
| Analytics dashboard | No | Basic | Advanced | Advanced | Feature flag + UI branching |
| Parental dashboard | No | No | No | Yes | `family_tier()` checks account type |

### Gate Implementation: Rust Surface

Every gate function in `crates/focus-entitlements/src/lib.rs`:

```rust
pub fn can_add_rule(current_count: u32, entitlement: &Entitlement) -> Result<bool, GateError> {
    match entitlement.tier {
        Tier::Free => Ok(current_count < 3),
        Tier::Plus | Tier::Pro | Tier::Family => Ok(true),
    }
}

pub fn connector_refresh_cadence_minutes(entitlement: &Entitlement) -> u32 {
    match entitlement.tier {
        Tier::Free => 240, // 4 hours
        Tier::Plus | Tier::Pro | Tier::Family => 15, // 15 minutes
    }
}
```

---

## StoreKit 2 vs RevenueCat: Decision & Scope

### StoreKit 2 (Chosen for v1)

**Pros:**
- Apple-native, zero external dependencies
- Free (no per-user cost)
- Works offline-first (transactions stored locally until sync)
- Automatic app-transaction validation

**Cons:**
- Requires backend for receipt verification (we'll use Cloudflare Worker)
- No Android support built-in (RevenueCat handles this)
- No unified dashboard (need custom analytics)

**v1 Implementation:**
- Use StoreKit 2's `Transaction.currentEntitlements` loop in iOS app
- Validate signatures on Cloudflare Worker (1 HTTP POST per app launch)
- Cache result in `EntitlementStore` (local SQLite); re-validate every 24h or on manual refresh
- Stub App Store Connect product IDs (use `.storekit` test config for dev)

### RevenueCat (Defer to Phase 2+Android)

Once FocalPoint Android ships, migrate:
1. Create RevenueCat API wrapper in `crates/focus-entitlements`
2. iOS StoreKit 2 adapts to RevenueCat SDK (no code change in Rust core)
3. Android uses RevenueCat Android SDK natively
4. Unified entitlement source across platforms

**Not shipping v1 because:** RevenueCat introduces external dependency (SaaS), costs ~$0.02/user/month at scale, and Android is Phase 3+.

---

## Server-Side Receipt Verification

### Cloudflare Worker (Recommended for v1)

**Purpose:** Validate StoreKit 2 app transaction signatures.

**Endpoint:**
```
POST /api/verify-receipt
Content-Type: application/json
{
  "app_transaction": "<base64-signed-transaction>",
  "product_id": "com.focalpoint.plus.monthly"
}
Response:
{
  "tier": "plus",
  "expires_at": "2025-05-23T21:00:00Z",
  "bundle_id": "com.focalpoint.ios"
}
```

**Implementation:**
- Use Apple's `JwtDecoder` library (JavaScript/Node on CF Workers) to verify signature
- Check expiry + bundle ID
- Return tier + expiry to client
- Cache result in `EntitlementStore`; client re-verifies every 24h

**Why not a full backend?**
- Lighter than spinning up a Rust server for v1
- Handles signature validation only (no database needed)
- Can upgrade to full Rust endpoint later if needed (analytics, user service, etc.)

---

## Apple Guidelines 3.1: IAP Compliance

**Requirement:** Digital goods (subscriptions, in-app features) **must** use In-App Purchase. No external links to "sign up" pages.

**FocalPoint Compliance Checklist:**

- ✅ All paid features accessible **only** via StoreKit 2 `.purchase()` (no external URL)
- ✅ Pricing clearly disclosed in paywall (tier name, price, billing cycle, auto-renewal terms)
- ✅ Subscription cancellation: easy to find in Settings > Subscriptions (Apple-handled; no app-side config)
- ✅ Trial eligibility: offer 7-day free trial for Plus tier (optional; drives conversion)
- ✅ Refund policy: link to Apple's standard policy (we don't override)
- ✅ Accessibility: paywall text readable; colors WCAG AA compliant
- ✅ No dark patterns: "Upgrade to Plus" CTA is secondary; free tier is default
- ✅ Kids app consideration: v1 is age 12+; no special handling (no kids IAP rules apply unless we pursue child safety tier)

**Red Flags to Avoid:**
- Don't hide pricing (must show before purchase prompt)
- Don't require email/payment info for free tier (collect on Plus signup only)
- Don't artificially limit free tier to frustrate users (3 rules is genuinely useful)
- Don't offer external payment methods (gift cards, stripe, paypal) — not allowed for digital goods

---

## Feature Gate Paywall UX Flow

### Scenario: User tries to add 4th rule (Free tier)

1. **Rule editor:** "Add rule" button tapped
2. **Gate check:** `can_add_rule(3, free_entitlement)` → `Err(LimitExceeded)`
3. **Paywall sheet:** Slides up from bottom (half-sheet)
   - Headline: "Unlimited rules with Plus"
   - Feature comparison table: Free (3 rules), Plus (Unlimited), Pro (Unlimited + analytics)
   - "Upgrade to Plus" primary button (teal)
   - "View all features" secondary button (gray) → links to full tier comparison in Settings
   - "Maybe later" dismiss button
4. **Purchase flow:** Tap "Upgrade to Plus" → StoreKit 2 purchase sheet (Apple-controlled)
5. **Success:** Transaction validated, `EntitlementStore` updated, paywall dismisses, rule editor re-enabled

### Scenario: Free user opens app, Plus trial expired

1. **App launch:** `EntitlementStore` checks receipt age (24h stale or expired)
2. **Background fetch:** Cloudflare Worker verifies; receipt is expired (no active Plus subscription)
3. **Downgrade:** Tier reverts to Free; features auto-gate
4. **UI feedback:** Banner in settings: "Plus trial ended. Resubscribe to continue voice coaching."
5. **One-tap re-subscribe:** "Resubscribe" button → `.purchase()` again

---

## Privacy & Data Handling

### App Store Privacy Manifest

Required by Apple (as of iOS 17.2):

```
Tracking Data: None (FocalPoint does not use IDFA or cross-app tracking)
Financial Info: Used (In-App Purchase receipt for entitlement verification)
Health & Fitness: Used (optional connector to Apple Health for sleep debt rules)
User ID: Used (Apple account ID for CloudKit sync)
Sensitive Info: None (we never see card details; Apple handles all payment PII)
```

### Privacy.md Statement

To include in App Store submission:

> **Subscription & Payment Data:**
> FocalPoint uses Apple StoreKit 2 for in-app purchases. All payment information (credit card, Apple ID details) is processed by Apple and encrypted in transit. FocalPoint never receives or stores your payment details beyond the signed entitlement certificate.
>
> **Entitlement Verification:**
> When you purchase a subscription, your device securely communicates with Apple's servers to verify your entitlement. This verification is cached locally on your device and refreshed every 24 hours. If offline, the app uses the cached result until network is restored.
>
> **Audit Chain & Export:**
> All rule decisions and penalties are logged locally on your device in an append-only audit chain. You own this data and can export it anytime (no account required). We never upload audit logs to our servers.

---

## Transition Plan: Free → Paid v1 Launch

### Pre-Launch (Weeks 1–2)

1. Create App Store Connect app entry (bundle ID: `com.focalpoint.ios`)
2. Add in-app purchase products:
   - `com.focalpoint.plus.monthly` (recurring, $4.99, 7-day trial)
   - `com.focalpoint.plus.annual` (recurring, $39.99, 7-day trial)
   - `com.focalpoint.pro.monthly` (recurring, $9.99, 3-day trial)
   - `com.focalpoint.pro.annual` (recurring, $79.99, 3-day trial)
   - `com.focalpoint.family.monthly` (recurring, $14.99, no trial)
3. Configure Cloudflare Worker endpoint for receipt validation
4. Publish entitlements crate (test-only; no real StoreKit products wired until app review)

### Launch Day

1. Submit app to App Store review with `.storekit` config (test only)
2. Once approved: enable real product IDs in release build
3. Monitor first 48h: revenue funnel, churn, gate effectiveness

### Post-Launch (Week 3+)

1. A/B test paywall copy (e.g., "$4.99/month" vs. "Save 40% with annual plan")
2. Track conversion funnels: free → Plus / Plus → Pro
3. Iterate trial duration (7d vs. 14d for Plus)
4. Plan Phase 2: RevenueCat migration + Android parity

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| **Free tier too generous** (high churn to paid) | Monitor conversion; reduce to 2 rules if <5% Plus conversion after Month 2 |
| **Plus tier not valuable enough** | Connector breadth (4 connectors + integrations) is the moat; landing page emphasizes "unlock all 4" |
| **StoreKit 2 receipt verification fails** (network down) | 30-day offline cache; after 30d, assume Free tier (conservative fallback) |
| **Apple rejects IAP setup** | Unlikely (standard compliance); resubmit with clear pricing disclosure |
| **Family Sharing not enrolled** (affects Family tier adoption) | Family tier is optional; Plus tier is default upsell; Family Sharing rollout is Phase 2 |
| **Users churn after trial expiry** | Send re-engagement email (or in-app notification) 1 day before expiry; test 3-day vs. 7-day trial length |
| **Android parity pressure** (users expect multi-platform) | Clear "iOS only, v1.0" messaging until Android ships; don't promise Android before it's certain |

---

## Success Metrics (Post-Launch Tracking)

By end of Month 1:
- **Free → Plus conversion rate:** ≥ 5%
- **Plus → Pro conversion rate:** ≥ 2%
- **Trial completion rate:** ≥ 30% (of starters) → subscription renewal
- **Churn (Pro):** ≤ 15% / month (acceptable for premium SaaS)
- **App review:** 0 rejections on IAP compliance

By end of Month 3:
- **MRR:** ≥ $500 (supports cloud costs + modest team)
- **Retention (D30 free tier):** ≥ 40%
- **Average revenue per user (ARPU):** ≥ $0.15 / user / month

---

## Next Steps (Immediate)

1. ✅ Implement `focus-entitlements` Rust crate (this doc + scaffold)
2. ✅ Build iOS `StoreKit2Manager` + `PaywallView` stubs (wired; not functional until App Store products live)
3. ✅ Add FFI layer (`EntitlementsApi`) for Rust ↔ Swift communication
4. ⚠️ Create App Store Connect products (real IDs; deferred until app review approval)
5. ⚠️ Deploy Cloudflare Worker receipt validator (deferred; not needed until real receipts)
6. ⚠️ Marketing copy + landing page tier comparison (Phase 1.1 after app launch)

---

## Appendix: Tier Comparison Table (User-Facing)

| Feature | Free | Plus | Pro | Family |
|---------|------|------|-----|--------|
| **Core** | | | | |
| Custom rules | 3 | ∞ | ∞ | ∞ |
| Tasks/goals | 3 | ∞ | ∞ | ∞ |
| Rule audit | ✓ | ✓ | ✓ | ✓ |
| Export (JSON/CSV) | ✓ | ✓ | ✓ | ✓ |
| **Connectors** | | | | |
| Active connectors | 1 (4h cadence) | 4 (15m cadence) | 4 (15m cadence) | 4 (15m cadence) |
| Connector types | Canvas, GCal, GitHub, +1 | All | All | All |
| **Focus & Breaks** | | | | |
| Session durations | 25 / 45 min fixed | 5–180 min | 5–180 min | 5–180 min |
| Live Activity | — | ✓ | ✓ | ✓ |
| HomeKit widget | — | ✓ | ✓ | ✓ |
| **Coachy** | | | | |
| Coachy appearance | Silent, basic | Silent or native voice | Custom voice (ElevenLabs) | Custom voice |
| Nudge frequency | — | 3/day | Proactive (24h-ahead) | Proactive |
| Custom cosmetics | — | — | ✓ | ✓ |
| **Analytics & Data** | | | | |
| Audit retention | 7 days | 90 days | 180 days | 365 days |
| Basic dashboard | — | ✓ | ✓ | ✓ |
| Advanced dashboard | — | — | ✓ | ✓ |
| Template marketplace | — | — | ✓ | ✓ |
| **CloudKit Sync** | — | ✓ | ✓ | ✓ |
| **Family Features** | — | — | — | ✓ |
| Family dashboard | — | — | — | ✓ |
| Shared templates | — | — | — | ✓ |
| Parent overrides | — | — | — | ✓ |
| Up to 5 members | — | — | — | ✓ |
| **Support** | Community | 48h response | 24h response | 24h response |
| **Pricing** | $0 | $4.99/mo or $39.99/yr | $9.99/mo or $79.99/yr | $14.99/mo |

---

## Document History

- **2026-04-23:** Initial design, 4-tier model, StoreKit 2 decision, Apple 3.1 compliance.
