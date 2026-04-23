# Security Policy

## Reporting a vulnerability

**Do not open a public issue for security problems.**

Email: **kooshapari@gmail.com** with subject prefix `[FocalPoint SECURITY]`.

Please include:

- A clear description of the issue and its impact.
- Steps to reproduce, ideally with a minimal rule DSL + connector state fixture.
- The crate(s) and versions affected (`cargo pkgid` output is helpful).
- Whether the issue is known publicly or still private.
- Your name / handle as you want it credited (or "anonymous").

Response time target: initial acknowledgement within 72 hours, triage within 7 days.

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

## Coordinated disclosure

We follow a 90-day coordinated disclosure window from the first acknowledged report. Earlier disclosure may be negotiated if the issue is actively exploited.

## Hall of fame

Researchers whose reports lead to a fix are credited in `CHANGELOG.md` and listed here (with permission).

| Date | Researcher | Issue |
|------|-----------|-------|
| _none yet_ | _—_ | _—_ |

## Cryptographic material

- The audit chain uses SHA-256 (via `sha2`) and Ed25519 signatures (via `ring`).
- Secrets are stored in the platform keychain (iOS Keychain, Android Keystore) via `secrecy` + `focus-crypto::keychain`.
- We never log secret values. If you find one logged, it is a security bug — report it.

## PGP / signing

We do not currently require PGP-signed reports; plain email to the address above is fine. If you prefer end-to-end encryption, request a key in your first email and we will supply one.
