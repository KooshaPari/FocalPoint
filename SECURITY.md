# Security Policy

## Reporting a vulnerability

**Do not open a public issue for security problems.**

### Reporting Process

**Email:** **security@focalpoint.app** with subject line `[SECURITY] Brief description`.

**Please include:**

- **Summary:** 1–2 sentences describing the vulnerability.
- **Impact:** Why is this a problem? (e.g., "Attacker can forge audit records," "Secrets may leak to logs").
- **Affected crate(s):** List specific crate(s) and version(s) affected (e.g., `focus-crypto@0.0.1`, `focus-audit@0.0.1`).
- **Steps to reproduce:** Minimal rule DSL + event stream or code snippet that triggers the issue.
- **Proof of concept (if applicable):** A small reproducible test case.
- **Timeline:** When did you discover this? Is it actively exploited?
- **Your name/handle:** How you want to be credited (or "anonymous" for anonymous reports).
- **Contact:** Email or preferred method to reach you.

### Example Report

```
Subject: [SECURITY] Audit chain accepts invalid Ed25519 signatures

The focus-audit crate does not validate Ed25519 signature length
before parsing, allowing crafted signatures to bypass verification.

Affected: focus-audit@0.0.1, focus-crypto@0.0.1 (as dependency)

Proof of concept: [... minimal test case ...]

This was discovered on 2026-04-20 and has not been disclosed publicly.

Credit: Jane Researcher (jane@example.com)
```

**Response timeline:**
- **Initial acknowledgement:** within 72 hours.
- **Triage:** within 7 days (severity assessment, reproduction confirmation).
- **Coordinated disclosure:** 90 days from acknowledgement (see below).
- **Updates:** weekly if work is in progress; at minimum, every 2 weeks.

**If you don't hear from us:** Escalate to the project lead via GitHub (public mention in an issue, tagged `@<maintainer-name>`). We will respond.

## Scope

### In scope

- The Rust core crates (`crates/focus-*`, `crates/connector-*`).
- The UniFFI-generated Swift bindings and the iOS app under `apps/ios/`.
- The audit chain, crypto (`focus-crypto`), and storage (`focus-storage`) paths.
- The connector OAuth / token handling surface.
- Rule DSL parsing (injection, sandbox escape).

### Out of scope

- Issues in upstream dependencies (report upstream; we will bump once fixed).
- Attacks requiring local root on the user's device (we trust the device).
- Apple FamilyControls / ManagedSettings bypasses that require user cooperation.
- The docs-site (report as a regular issue).
- Services under `services/*` (stubs; not yet shipped).

## Coordinated Disclosure & CVE Workflow

### The 90-Day Window

We follow a **90-day coordinated disclosure** policy from the date we acknowledge your report:

1. **Days 0–7:** We triage and confirm the issue.
2. **Days 7–70:** We develop, test, and review a fix.
3. **Days 70–85:** We prepare a release and CVE application (if applicable).
4. **Day 85–90:** We notify you of the patch release and CVE number (if any).
5. **Day 90:** We publicly disclose the vulnerability in the release notes and/or a GitHub security advisory.

**Earlier disclosure:** If a vulnerability is actively exploited in the wild or poses imminent risk, we may compress this timeline with your agreement.

### CVE Process

- **Eligibility:** We apply for CVEs for vulnerabilities affecting confidentiality, integrity, or availability of user data or system security.
- **No CVE:** Low-impact issues (e.g., typos, cosmetic bugs) do not warrant a CVE.
- **Application:** We apply via NVD or MITRE after a fix is ready; you will be notified of the CVE ID.
- **Public advisory:** Published on GitHub's Security Advisories page and linked in release notes.

### Patch Release

- Critical security fixes are released in a dedicated patch release (e.g., `v0.0.2`) as soon as possible (target: within 7 days of fix completion).
- No batching with other features; security patches are prioritized.

### Your Role

- You are welcome (and encouraged!) to help verify the patch.
- Your name will be credited in the CVE, GitHub advisory, and `CHANGELOG.md` (unless you request anonymity).
- If you have a responsible disclosure embargo, let us know; we will respect it.

## Hall of fame

Researchers whose reports lead to a fix are credited in `CHANGELOG.md` and listed here (with permission).

| Date | Researcher | Issue |
|------|-----------|-------|
| _none yet_ | _—_ | _—_ |

## Cryptographic material

- The audit chain uses SHA-256 (via `sha2`) and Ed25519 signatures (via `ring`).
- Secrets are stored in the platform keychain (iOS Keychain, Android Keystore) via `secrecy` + `focus-crypto::keychain`.
- We never log secret values. If you find one logged, it is a security bug — report it.

## Responsible Use Policy

If you discover a vulnerability, please **do not:**

- Share the vulnerability publicly before we disclose it.
- Exploit the vulnerability to access user data or systems.
- Use the vulnerability for financial gain.
- Disclose the vulnerability to a third party without our consent.

**In return, we will:**

- Treat your report confidentially.
- Credit you (unless you request anonymity).
- Work transparently to fix the issue.
- Notify you before any public disclosure.

Violations of this policy may result in legal action and will permanently ban you from contributing to FocalPoint.

## PGP / Signing

We do not currently require PGP-signed reports; plain email to the address above is fine. If you prefer end-to-end encryption, request a public key in your first email and we will provide one.
