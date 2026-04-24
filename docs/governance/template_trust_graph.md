# Template Pack Trust Graph

**Status:** v0.1.0 — established 2026-04-24  
**Owners:** FocalPoint ops (@KooshaPari), Community Template Council (TBD)

---

## Overview

FocalPoint's template pack ecosystem uses a **multi-tier trust graph** to balance community contribution with user safety. Each tier has distinct signing keys, verification requirements, and UI presentation.

### Trust Tiers

| Tier | Key Holder | Signature | Verification | UI Badge | Risk Model |
|------|-----------|-----------|--------------|----------|-----------|
| **Tier 1: Author-Signed** | Template author | Author's ed25519 | GitHub identity + human review | "Author-signed" + fingerprint | Community + GitHub trust |
| **Tier 2: Org-Verified** | FocalPoint team | FocalPoint root key | Code audit + security review | "FocalPoint Verified" ✓ | Org endorsement + vetting |
| **Tier 3: Connector-Backed** | Connector vendor | Vendor's ed25519 | Vendor maintains pack | "Canvas Verified" (vendor name) | Vendor partnership |

---

## Tier 1: Author-Signed (Community)

### Definition

A template pack signed by the author's personal ed25519 key. Community authors can submit packs via pull request without organizational endorsement.

### Requirements

- **Author identity:** GitHub account (used for PR attribution)
- **Signing key:** Author-generated ed25519 keypair
- **Code review:** FocalPoint humans review rules, connectors, and author intent
- **Attestation:** Author asserts no malicious behavior in PR description

### Verification

```
pack → canonical JSON → sign with author's private key
↓
signature in manifest → user trusts author's public key
```

**User perspective:**
1. Search/install pack from registry
2. App displays: "Author-signed by @jane-doe (fp: c742e5e5fa536e56)"
3. User decides: trust this author? (Telegram, personal knowledge, Discord, etc.)
4. Install proceeds; signature verified on load

### Promotion to Tier 2

A Tier 1 pack is eligible for promotion if:

1. **Adoption:** ≥50 installs over 2 weeks (or equivalent activity proxy)
2. **Stability:** No critical bugs reported; author responsive to issues
3. **Safety:** Rules reviewed; no evidence of hidden tracking or consent violations
4. **Author participation:** Author willing to co-maintain with FocalPoint team
5. **Alignment:** Pack reflects FocalPoint values (health, privacy, productivity)

**Process:**
1. Author or community nominates: "Promote pack-id to Tier 2"
2. FocalPoint team: security audit (rules, connectors, integrations)
3. Decision: yes/no within 1 week; public rationale posted
4. If approved: team re-signs pack with root key; merge to `main`
5. Notification: installed users see "upgrade to Tier 2" in Settings → Packs
6. No action required; install auto-validates new signature on next load

### Revocation

**Conditions:**
- **Compromise:** Author's private key is leaked or misused
- **Malicious rule:** Pack discovered to violate consent (hidden tracking, etc.)
- **Abandonment:** Author unresponsive; critical bugs unfixed for ≥3 months
- **Policy violation:** Connector integrations break FocalPoint ToS or law

**Process:**
1. Issue filed with evidence (security@focalpoint.app for sensitive cases)
2. FocalPoint team investigates (internal, 1 week max)
3. Decision + public statement posted
4. Pack removed from `index.json`
5. Installed instances: badge changes to "Revoked"; warning in Settings → Packs
6. User must explicitly acknowledge warning to continue using

---

## Tier 2: Org-Verified (FocalPoint)

### Definition

A template pack signed by the FocalPoint team's root ed25519 key, stored in `PHENOTYPE_ROOT_PUBKEYS` (compile-time). Represents organizational endorsement: the pack has been audited and is maintained as part of the official ecosystem.

### Examples

All 7 starter packs:
- `deep-work-starter` — focus discipline
- `student-canvas` — academic / Canvas LMS
- `dev-flow` — developer productivity
- `sleep-hygiene` — health / sleep
- `gym-routine` — fitness
- `reading-habit` — learning
- `research-writing` — academic / research

### Requirements

- **Author:** FocalPoint team or promoted community author
- **Code review:** Full security audit (rules, connectors, event triggers)
- **Testing:** E2E test of pack on iOS + Android simulators
- **Maintenance:** FocalPoint team owns bug fixes and version bumps
- **Documentation:** Pack README and connector integration docs

### Signing & Distribution

```
pack (author-created or promoted from Tier 1)
  ↓
FocalPoint ops: audit + test
  ↓
Sign with PHENOTYPE_ROOT_PUBKEYS private key
  ↓
Merge to main; distributed in app bundle + registry
```

**Root key location:**
```rust
// crates/focus-templates/src/signing.rs
pub const PHENOTYPE_ROOT_PUBKEYS: &[&str] = &[
    "c742e5e5fa536e56e7b38fee4b91caed9f172c09b3b9d33817c4c87ce9729d1e",  // 2026-04 demo
    // "ops-key-1" when ready for production
];
```

### Verification

App trusts root keys **compile-time** (no network needed):

```rust
let trusted_roots = vec![PHENOTYPE_ROOT_PUBKEYS...];
pack.verify_and_apply(&mut store, &manifest, &trusted_roots, false)?;
```

User sees: "FocalPoint Verified ✓"  
No fingerprint; implicit trust via app distribution.

### Updates & Deprecation

Tier 2 packs are maintained in lockstep with app releases:
- Bug fixes → version bump (e.g., `0.1.0` → `0.1.1`)
- Major changes → minor version (e.g., `0.2.0`) with migration guide
- Deprecation: 2 releases before removal (e.g., `0.1.0` deprecated in `0.3.0`, removed in `0.4.0`)

---

## Tier 3: Connector-Backed (Partnership)

### Definition

A template pack co-signed by a connector vendor (e.g., Canvas LMS, Readwise, Strava). The vendor endorses and maintains the pack; often distributed alongside the connector SDK or vendor's own app.

### Examples

**Canvas Student Pack** (hypothetical):
- Canvas LMS team authors & signs pack
- FocalPoint registers as Tier 3
- Canvas exports pack to students during account setup
- App verifies via Canvas public key

### Requirements

- **Vendor partnership:** MOU or open-source agreement with connector vendor
- **Vendor signing:** Vendor generates and manages their own ed25519 key
- **Co-authorship:** Pack developed jointly with vendor; vendor accountable
- **Maintenance:** Vendor owns rules + connectors; FocalPoint supports registry
- **Documentation:** Vendor docs + FocalPoint docs mirror each other

### Signing & Distribution

```
pack (vendor authors)
  ↓
Vendor ops: signs with vendor key
  ↓
Submitted to FocalPoint registry (vendor PR or API)
  ↓
FocalPoint register with vendor key in index.json
  ↓
App verifies signature against vendor key (at runtime)
```

**Vendor key location:**
```json
// examples/templates/index.json
{
  "catalog": [...],
  "trusted_vendors": {
    "canvas-lms": "canvas-vendor-pubkey-hex",
    "readwise": "readwise-vendor-pubkey-hex"
  }
}
```

### Verification

```rust
let vendor_key = get_trusted_vendor_key("canvas-lms")?;
pack.verify_and_apply(&mut store, &manifest, &[vendor_key], false)?;
```

User sees: "Canvas Verified" (vendor name as badge)

---

## Key Lifecycle Management

### Root Key (Tier 2: FocalPoint)

**Status:** Ephemeral demo key (2026-04-24)  
**Fingerprint:** `c742e5e5fa536e56`  
**Location:** `examples/templates/_keys/demo.*` (git-checked-in for dev)  

**Production (scheduled:**
1. Generate new permanent key via HSM or secure offline setup
2. Update `PHENOTYPE_ROOT_PUBKEYS` in crates/focus-templates/src/signing.rs
3. Re-sign all Tier 2 packs
4. Bake new signatures into app release
5. Retire demo key; remove from git

### Vendor Keys (Tier 3)

**Distribution:**
- Each vendor publishes public key via:
  - GitHub README (with security.txt linking)
  - OIDC issuer (if using OpenID Connect federation)
  - DNS TXT record (future; not implemented)
- FocalPoint maintains registry of trusted vendors
- Add new vendor key: PR to `examples/templates/index.json`

**Rotation:**
- Vendor rotates key every 2–5 years
- Old key added to deprecated list (accepts old signatures for 1 year)
- New key becomes primary

### Author Keys (Tier 1)

**Lifecycle:**
- Author generates and backs up `~/.focalpoint-sign-key.priv`
- Never shared; never checked into git
- Author responsible for security (password manager, USB backup, etc.)

**Compromise:**
- Author files issue: "Revoke my key (compromised)"
- FocalPoint removes packs signed with old key from trusted registry
- Existing installs: warning badge
- Author generates new key; resubmit packs

---

## Safety Policies

### Rule Validation

All packs (Tier 1–3) must pass:

1. **Schema validation:** TOML parses; rule types exist
2. **Action audit:** No `EmergencyExit` or undocumented actions (Tier 1+)
3. **Connector check:** Connectors exist and are open-source (Tier 1+)
4. **Event trigger check:** Known event types; no fabricated events (Tier 2+)
5. **Duration sanity:** Rules don't lock user out indefinitely (Tier 2+)

**Tier 1 rules:** Community-reviewed by FocalPoint humans; authors can override with caveat  
**Tier 2 rules:** Full audit; no overrides  
**Tier 3 rules:** Vendor responsible; FocalPoint spot-checks  

### Connector Integrations

**Allowed:**
- Read-only: calendar events, health data, code history
- Write: rule state, user scores/streaks, audit logs
- External: verified OAuth flow; no long-lived token storage in pack

**Forbidden:**
- Hidden tracking (IP geolocation, analytics without consent)
- Long-lived secrets in pack (API keys, auth tokens)
- Behavioral nudging beyond rule scope
- Cross-user data sharing

### Transparency

All packs must declare:
1. **Which connectors** (in TOML: `recommended_connectors`)
2. **What data** (in TOML: `description`)
3. **User consent** (in rule explanation templates)

Example:
```toml
description = "Integrates with Canvas LMS to track assignment deadlines and grant credits on submission."
recommended_connectors = ["canvas"]

[[rules]]
explanation_template = "Canvas assignment due within 24h; social apps locked."
# ^ User sees this before rule triggers
```

---

## Audit & Compliance

### Pack Audit Trail

Every pack load writes:
```rust
AuditRecord {
    pack_id: "my-pack",
    author: "jane-doe",
    tier: 2,  // Org-Verified
    action: "install",
    timestamp: now,
    signature_verified: true,
    key_fingerprint: "c742e5e5fa536e56",
}
```

Stored in SQLite; exportable via Settings → Export Audit Logs.

### Revocation Audit

When a pack is revoked:
```rust
AuditRecord {
    pack_id: "compromised-pack",
    tier: 2,
    action: "revoked",
    reason: "author-key-compromise",
    revocation_date: now,
    public_statement: "https://github.com/KooshaPari/FocalPoint/issues/123",
}
```

All installed instances log revocation event.

### Quarterly Review

FocalPoint team publishes quarterly report:
- New packs approved (Tier 1 → 2)
- Revocations + reasons
- Security incidents (if any)
- Community metrics (installs, ratings, feedback)

---

## Community Template Council (Future)

**Proposal:** Establish a council of 3–5 community members to review Tier 1 promotions and revocations.

**Goals:**
- Distribute trust decisions beyond single team
- Gather diverse perspectives on safety + value
- Improve community participation in governance

**Details:** (TBD after Phase 1 launch; to be formalized in ADR)

---

## Related Documentation

- [Submit Template Pack Guide](../guides/submit_template_pack.md)
- [Rule Schema Reference](../reference/rule_schema.md)
- [Connector SDK](../guides/connector_sdk.md)
- [ed25519 Signature Verification](../architecture/security/signatures.md) (when written)

---

## Appendix: Key Rotation Timeline (Production)

| Date | Action | Owner |
|------|--------|-------|
| 2026-04-24 | Demo key generated; seed 7 starter packs | FocalPoint ops |
| 2026-05-01 | Phase 1 launch; Tier 1–2 live | FocalPoint ops |
| 2026-06-01 | Generate permanent root key (HSM/offline) | FocalPoint ops |
| 2026-06-15 | Update PHENOTYPE_ROOT_PUBKEYS; re-sign all Tier 2 packs | FocalPoint ops |
| 2026-07-01 | Release app with new root key | FocalPoint ops |
| 2026-07-15 | Deprecate demo key; remove from git | FocalPoint ops |
| 2027-Q1 | Establish Community Template Council | FocalPoint ops + community |

---

**Last Updated:** 2026-04-24  
**Next Review:** 2026-07-01
