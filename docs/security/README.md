# FocalPoint Security Documentation

Index of security-related documentation for FocalPoint v0.0.3+.

## Core Security Documents

### **threat_model.md** — Living Threat Model
Comprehensive STRIDE analysis covering:
- **6 asset classes:** OAuth tokens, audit chain integrity, rules/task config, wallet/penalty state, template packs, FamilyControls profile
- **6 attacker personas:** Curious roommate, malicious template author, network-adjacent attacker, compromised connector API, compromised Apple ID, malicious MCP client
- **STRIDE per asset:** Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, Elevation of Privilege
- **Mitigations shipped:** Certificate pinning, audit signing, template pack signing, MCP authentication (status by phase)
- **Known-open gaps:** 8 documented gaps with timeline + interim mitigations
- **Incident response:** 6 scenarios (API compromise, malicious pack, audit tampering, token revocation, MCP exfiltration, wipe/restore)
- **Risk ratings:** CVSS-like summary table

**Read when:** Planning security features, responding to vulnerabilities, reviewing threat-impacting PRs.

---

### **SECURITY.md** — Vulnerability Reporting & Disclosure (Root)
Located at `/repos/FocalPoint/SECURITY.md`. Covers:
- **Reporting process:** Email security@focalpoint.app with vulnerability details
- **Triage timeline:** 72 hours acknowledgement, 7 days triage, 90 days to disclosure
- **CVE workflow:** Eligibility, application, public advisory
- **Patch release policy:** Critical fixes released within 7 days
- **Hall of fame:** Researchers credited in CHANGELOG.md

**Read when:** You discover a vulnerability, or need to report one.

---

## Planned Security Documents (Phase 2+)

### **audit_chain_recovery.md** — Audit Chain Tampering Recovery
User guide for recovering from audit chain tampering:
- Detection (hash chain validation failure at startup)
- Diagnosis (`focus audit verify --details`)
- Recovery options (restore from backup, wipe + reset, escalate to support)
- Evidence collection for security team

**Target ship:** Phase 2 (Q3 2026)

---

### **SIGNING_CEREMONY.md** — Root Key & Signing Operations
Operational procedures for:
- Root signing key generation (ed25519-dalek, offline hardware wallet recommended)
- Key backup + recovery (Shamir's secret sharing recommended)
- Key rotation schedule (annual)
- Template pack signing + verification workflow
- Incident response (key compromise, revocation)

**Target ship:** Phase 2, before ecosystem launch (Q3 2026)  
**Audience:** Ops team, release managers

---

### **privacy_manifest_checklist.md** — Privacy Manifest & App Tracking Transparency
Audit checklist for Apple's App Privacy questionnaire:
- Data categories collected (health, analytics, device ID)
- Purposes (rule evaluation, connector sync, backup)
- Tracking: Does FocalPoint use IDFA? (No)
- Fingerprinting: Does FocalPoint use device identifiers? (No; audit chain uses local UUIDs only)
- User controls (opt-out MCP server, disable iCloud backup)
- Third-party disclosures (Sentry error reporting, Apple FamilyControls API)

**Target ship:** Phase 1 finalization (by App Store submission)  
**Current status:** draft in `docs/release/privacy_manifest_checklist.md`

---

## Security Features by Phase

### **Phase 1 (v0.0.3 — Current, Shipped)**
- ✅ Audit chain SHA-256 hash linkage + startup validation
- ✅ OAuth token storage in iOS Keychain + refresh via platform APIs
- ✅ Template pack user consent flow on import
- ✅ Wipe receipt generation (app-signed)
- ✅ SECURITY.md + DCO governance
- ✅ MCP server with read-only tools (optional, opt-in)

### **Phase 2 (v0.1.0 — Q3 2026, Planned)**
- [ ] TLS certificate pinning for all connector APIs
- [ ] Ed25519 signatures on audit records + batch verification
- [ ] Template pack signing + ecosystem verification
- [ ] MCP client authentication + user approval modal
- [ ] SIGNING_CEREMONY.md + key rotation playbook
- [ ] audit_chain_recovery.md user guide
- [ ] Connector compromise detection + rollback tooling

### **Phase 3+ (v0.2.0 — Q4 2026+)**
- [ ] Runtime integrity checks on Rust binary
- [ ] Multi-device sync + CloudKit encryption
- [ ] Jailbreak detection (low priority; deferred)

---

## Security Contacts & Policies

| Role | Contact | Responsibilities |
|------|---------|------------------|
| Security Team | security@focalpoint.app | Triage vulnerabilities, coordinate disclosure, update threat model |
| Release Manager | releases@focalpoint.app | Root key access, template pack signing, patch releases |
| Legal | legal@focalpoint.app | Privacy manifests, terms updates, compliance |

---

## Compliance & Standards

- **Responsible Disclosure:** 90-day coordinated disclosure (SECURITY.md)
- **Cryptography:** SHA-256 (audit chain), Ed25519 (signatures, planned), AES-256-GCM (backups via age)
- **Identity & Access:** iOS Keychain for OAuth secrets, no plaintext storage, device-local UUIDs
- **Privacy:** GDPR-friendly (local-first, no server-side analytics), Apple Privacy Manifest, CCPA opt-out via Settings
- **Code Signing:** DCO on all commits, Apple code signing on IPA, template pack signing (planned)

---

## Quick References

### Threat Model Assets
1. [OAuth Tokens](threat_model.md#asset-1-oauth-tokens) — Canvas, GCal, GitHub, Fitbit, Readwise, Notion, Linear, Strava
2. [Audit Chain Integrity](threat_model.md#asset-2-audit-chain-integrity) — Tamper-evident event log
3. [Rules & Task Config](threat_model.md#asset-3-rules--task-configuration) — Rule DSL + templates
4. [Wallet & Penalties](threat_model.md#asset-4-wallet--penalty-state) — Rewards + escalation tiers
5. [Template Packs](threat_model.md#asset-5-template-packs) — Rule bundles + starter packs
6. [FamilyControls Profile](threat_model.md#asset-6-familycontrols-profile--device-enforcement) — iOS enforcement

### Threat Model Personas
1. [Curious Roommate](threat_model.md#persona-a-curious-roommate-physical-access) — Unlocked phone access
2. [Malicious Template Author](threat_model.md#persona-b-malicious-template-pack-author) — Unsigned ecosystem packs
3. [Network-Adjacent Attacker](threat_model.md#persona-c-network-adjacent-attacker-same-wifi) — MITM on WiFi
4. [Compromised Connector API](threat_model.md#persona-d-compromised-connector-api-canvasgithubetc-hacked) — Forged events
5. [Compromised Apple ID](threat_model.md#persona-e-compromised-apple-id-icloud-backupsync) — iCloud key access
6. [Malicious MCP Client](threat_model.md#persona-f-malicious-mcp-client-in-process-socket) — Socket exfiltration

### Known Security Gaps
| Gap | Severity | Timeline | Ref |
|-----|----------|----------|-----|
| No TLS cert pinning | High | Phase 2 | [Gap #1](threat_model.md#gap-1-no-tls-certificate-pinning) |
| Unsigned audit chain | Medium | Phase 2 | [Gap #2](threat_model.md#gap-2-audit-chain-not-cryptographically-signed) |
| Unsigned template packs | High | Phase 2 | [Gap #3](threat_model.md#gap-3-template-pack-signing-not-yet-shipped) |
| No runtime integrity check | Medium | Phase 3+ | [Gap #4](threat_model.md#gap-4-no-runtime-integrity-check-on-rust-binary) |
| No jailbreak detection | Low | Deferred | [Gap #5](threat_model.md#gap-5-no-jailbreak-detection-intentional) |
| FamilyControls unapproved | High | Q2 2026 | [Gap #6](threat_model.md#gap-6-familycontrols-entitlement-not-yet-approved) |
| MCP socket no auth | Medium | Phase 2 | [Gap #7](threat_model.md#gap-7-mcp-server-has-no-authentication) |
| Ops root key not run | High | Phase 2 | [Gap #8](threat_model.md#gap-8-ops-root-key-ceremony-not-run) |

### Incident Response Scenarios
- [Compromised Connector API](threat_model.md#scenario-1-compromised-connector-api-eg-canvas-hacked)
- [Malicious Template Pack Published](threat_model.md#scenario-2-malicious-template-pack-published)
- [Audit Chain Tampering Detected](threat_model.md#scenario-3-audit-chain-tampering-detected)
- [OAuth Token Compromised](threat_model.md#scenario-4-oauth-token-compromised)
- [MCP Socket Exfiltration](threat_model.md#scenario-5-mcp-socket-exfiltration)
- [Device Wipe/Restore](threat_model.md#scenario-6-device-wipestore)

---

## How to Use This Documentation

### **For Security Reviewers**
1. Start with [threat_model.md](threat_model.md) — 2000-word STRIDE analysis
2. Reference STRIDE per asset + persona to evaluate PRs
3. Track gaps + incident response scenarios for release readiness

### **For Developers**
1. Read [SECURITY.md](../SECURITY.md) — understand vulnerability reporting
2. Review threat_model.md for the assets you're working on:
   - Connector work? See [Asset 1: OAuth Tokens](threat_model.md#asset-1-oauth-tokens) + [Persona C/D](threat_model.md#persona-c-network-adjacent-attacker-same-wifi)
   - Audit changes? See [Asset 2: Audit Chain](threat_model.md#asset-2-audit-chain-integrity)
   - Rule DSL? See [Asset 3: Rules](threat_model.md#asset-3-rules--task-configuration)
   - MCP work? See [Asset 6: MCP Client](threat_model.md#persona-f-malicious-mcp-client-in-process-socket)
3. Apply mitigations from threat_model.md (check "Mitigations shipped" + "Mitigations planned")

### **For Release Managers**
1. Consult [SIGNING_CEREMONY.md](SIGNING_CEREMONY.md) (Phase 2) before signing template packs
2. Use [audit_chain_recovery.md](audit_chain_recovery.md) (Phase 2) to onboard users on recovery procedures
3. Cross-check Phase release checklist against [Security Features by Phase](README.md#security-features-by-phase)

### **For Users**
1. See [PRIVACY.md](../release/PRIVACY.md) for data handling + privacy controls
2. See [docs/guides/backup_restore.md](../guides/backup_restore.md) for backup/wipe procedures
3. See [SECURITY.md](../SECURITY.md) to report vulnerabilities

---

## Document Maintenance

- **Annual review:** threat_model.md reviewed every April (anniversary)
- **Issue-driven updates:** threat_model.md updated when a vulnerability is reported or mitigated
- **Phase completions:** Security gaps updated when Phase 2/3 features ship
- **Versioning:** threat_model.md tracks version + date in header

**Last reviewed:** 2026-04-23  
**Next review:** 2027-04-23
