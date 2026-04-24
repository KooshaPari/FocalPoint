# Privacy Policy — FocalPoint

FocalPoint is a local-first, privacy-respecting screen-time platform. Your data lives on your device.

## Data Collection & Storage

- **On-device:** All core data (tasks, rules, wallet, audit trail, penalties) stored in encrypted SQLite database
- **No telemetry:** FocalPoint does not collect usage metrics, event logs, or crash reports by default
- **No cloud sync:** Data does not leave your device unless you explicitly export via backup

## Encrypted Backups

When you create a backup via **Settings → Data → "Create encrypted backup"**:

1. All data is serialized to JSON (manifest)
2. **Encrypted on-device** with age (ChaCha20-Poly1305) using your chosen passphrase
3. Scrypt KDF (N=2^16) derives encryption key from passphrase
4. Result: `.backup` file (typically 10–50 KB)
5. **FocalPoint never sees your passphrase** — encryption happens client-side

### What's in the Backup

- Wallet state, penalties, rules, tasks, templates, audit trail
- NOT included: OAuth tokens (Keychain only), connector cursors (ephemeral)

### What's NOT Backed Up

- **OAuth tokens:** Stored in device Keychain; re-authenticate on new device
- **Connector cursors:** Ephemeral pagination pointers; re-hydrated on sync

## Connectors & External Services

When you enable a connector (Google Calendar, GitHub, Canvas, Apple Health, Fitbit):

- **OAuth flow:** FocalPoint exchanges your token for a secure credential in Keychain
- **Scopes:** Minimal; calendar read-only, GitHub org/public repos, Canvas courses only
- **Data flow:** Events fetched from external services and stored locally in FocalPoint's SQLite
- **No sharing:** Your data is never re-uploaded to connectors

## Health Data (Apple Health / Fitbit)

**What we collect** (only when you connect):
- **Workouts:** activity type, duration, calories burned, distance
- **Sleep:** total hours, sleep efficiency, bed/wake times
- **Steps:** daily count, milestone events (≥10,000)
- **Heart rate:** resting heart rate only (no beat-by-beat data)

**Where it's stored:**
- Local SQLite database (`core.db`), encrypted at rest
- OAuth tokens in iOS Keychain (production)

**What we DON'T collect:**
- Heart rate variability (HRV) or minute-by-minute samples
- Blood pressure, glucose, ECG, or other clinical metrics
- Location tracking
- Raw HealthKit samples beyond daily aggregates

**Regulatory note:** FocalPoint is a consumer wellness app, not a HIPAA-covered entity. We follow data minimization principles but are not subject to HIPAA obligations.

**Deletion:** To disconnect health connectors, go to Settings → Connectors → [Apple Health/Fitbit] → Disconnect. Access tokens are purged immediately; event history retained unless manually cleared via Settings → Data Export → Clear All.

## Network Access

FocalPoint makes network requests only when:

1. **Connector sync:** Hydrating calendar events, GitHub activity, Canvas assignments
2. **Coaching API** (optional): Sending anonymized task/penalty state for AI suggestions
3. **Automatic updates** (future): Checking for app updates

All requests use TLS 1.3 (encrypted in transit).

## iOS-Specific Permissions

- **Calendar:** via EventKit (read-only; required for morning brief)
- **Usage Stats:** via ScreenTime framework (required for screen-time awareness)
- **Family Controls:** required for enforcing blockages (requires App Store entitlement)
- **Keychain:** stores OAuth credentials securely per-device
- **File access:** backup/restore only via DocumentPicker or ShareLink

No permissions are requested without explicit user consent.

## Android-Specific Permissions

- **PACKAGE_USAGE_STATS:** screen-time monitoring (user-grantable)
- **BIND_ACCESSIBILITY_SERVICE:** app launch detection (user-grantable)
- **Keystore:** encrypted OAuth token storage per-device

## Data Deletion

- **On-device deletion:** All data wiped from SQLite on uninstall
- **Keychain/Keystore:** OAuth tokens removed on logout or app uninstall
- **Backups:** Yours to manage; FocalPoint does not retain copies

## Third-Party Libraries

- **age (encryption):** Modern GPG successor; audited cryptography
- **Zstd (compression):** Meta-maintained; no data exfiltration
- **Serde (serialization):** JSON only; no custom protocols
- **Reqwest (HTTP):** Mozilla-backed; standard TLS 1.3

All dependencies are OSS and auditable.

## Contact

Privacy questions? File an issue in the repository or contact the maintainer.

---

**Last updated:** 2026-04-23
