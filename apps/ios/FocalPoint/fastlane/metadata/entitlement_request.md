# FamilyControls Entitlement Request — FocalPoint

**App:** FocalPoint  
**Bundle ID:** com.koosha.focalpoint  
**Request Date:** 2026-04-23  
**Status:** Draft (awaiting user submission to Apple Developer Relations)

---

## Executive Summary

FocalPoint is a dual-mode screen-time companion that enables **parental controls** (guardians managing usage rules for their children) and **self-control** (individuals enforcing their own focus habits). The app integrates with iOS FamilyControls to enforce usage restrictions at the system level, ensuring rules are tamper-proof and persistent even when the app is closed.

---

## Why FamilyControls Entitlement is Required

FamilyControls is Apple's official framework for parental control and device management. Without this entitlement, FocalPoint cannot:
- Restrict app usage based on user-defined rules
- Enforce screen-time limits at the system level
- Prevent users from uninstalling the app or disabling restrictions
- Synchronize rules across multiple devices

This entitlement is essential for FocalPoint's core functionality.

---

## App Purpose & Use Cases

### Use Case 1: Parental Controls
A parent can use FocalPoint to manage their child's device usage:
- Define usage rules (e.g., "Instagram limited to 2 hours per day")
- Set focus blocks (e.g., "No apps 8pm–8am")
- Receive alerts when usage thresholds are exceeded
- Audit a tamper-evident log of all rule changes

### Use Case 2: Self-Control (Personal Focus)
An individual can use FocalPoint to enforce their own focus habits:
- Create personal focus blocks for work or study
- Limit access to distracting apps during focus hours
- Set daily usage budgets
- View an audit chain proving they did not circumvent the rules

---

## Technical Implementation

### Local-First Architecture
- **No cloud dependency:** Rules, wallet state, and audit logs are stored **locally** in encrypted SQLite
- **Open-source core:** The Rust FFI core (`crates/focus-ffi`) is the source of truth for rule evaluation
- **Transparent audit chain:** Every rule change is appended to an immutable audit log with SHA-256 hash chains
- **No user tracking:** FocalPoint does NOT collect telemetry, location data, or usage analytics

### Tamper Detection
FocalPoint implements cryptographic verification to detect tampering:
1. Every mutation (rule change, reward claimed, penalty applied) produces an audit record
2. Records are hashed with SHA-256 and chained (each record includes the hash of the previous)
3. On app startup, the chain is verified; any break indicates tampering
4. A tamper is logged and reported to the user; further operations are blocked until manual resolution

### Privacy & GDPR Compliance
- **No PII collection:** App does not collect names, email addresses, or identifiers
- **No data syncing:** Rules stay on device; no cloud backup or sharing (local-only sync via `confp` connector)
- **Transparent data model:** Users can export their audit chain as JSON for inspection
- **Compliance:** See `docs/PRIVACY.md` and `docs/TERMS.md`

---

## Security Measures

1. **Device Ownership Verification:** Only the device owner (or designated guardian) can modify rules
2. **Secure Secret Storage:** Secrets (connector API keys, recovery codes) stored in Keychain
3. **Audit Immutability:** Audit records are append-only; deletion is impossible without crashing the app
4. **No Privilege Escalation:** Even if a user gains developer mode access, they cannot modify historical audit records

---

## References

- **Privacy Policy:** https://github.com/KooshaPari/FocalPoint/blob/main/docs/PRIVACY.md
- **Terms of Service:** https://github.com/KooshaPari/FocalPoint/blob/main/docs/TERMS.md
- **Security Audit Chain:** https://github.com/KooshaPari/FocalPoint/blob/main/docs/SECURITY.md
- **GitHub Repository:** https://github.com/KooshaPari/FocalPoint

---

## Submission Instructions

1. **Log in** to [Apple Developer Portal](https://developer.apple.com/)
2. **Navigate** to Certificates, Identifiers & Profiles → Identifiers → `com.koosha.focalpoint`
3. **Request entitlement:** Click "Edit" → Enable "Family Controls" → Submit request
4. **Wait for approval:** Expect review within 1–4 weeks
5. **Update in Xcode:** Once approved, the entitlement will appear in your Team Provisioning Profile
6. **Enable in Xcode:** Add `com.apple.developer.family-controls` to `Signing & Capabilities` in Xcode

---

## Approval Timeline Estimate

- **Standard review:** 1–4 weeks
- **Expedited review:** Available via App Store Connect if urgent (contact Apple Support)

---

**Prepared by:** FocalPoint Development Team  
**Date:** 2026-04-23  
**Next Review:** 2026-06-01
