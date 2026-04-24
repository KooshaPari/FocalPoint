# FocalPoint Telemetry Dashboard

## Overview

FocalPoint collects **anonymous, opt-in usage analytics** to prioritize feature development based on what people actually use. All telemetry is:

- **Strictly opt-in** — default OFF, user controls via Settings → Diagnostics → "Share usage analytics"
- **Local-first** — events buffered in SQLite before transmission
- **PII-scrubbed** — emails, phone numbers, tokens, UUIDs redacted before buffering
- **Separate from crash reporting** — independent of Sentry integration
- **Auditable** — every flush logged with event count + endpoint domain

## Event Schema

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "connector.connected",
  "ts": "2026-04-24T14:32:18.123Z",
  "session_id": "8f2d5b4a9e1c3f6d",
  "app_version": "0.0.1",
  "os_version": "iOS 17.0",
  "props": {
    "connector_type": "github",
    "auth_method": "oauth",
    "event_version": "v1"
  }
}
```

### Fields

| Field | Type | Cardinality | Notes |
|-------|------|-------------|-------|
| `event_id` | UUID | Required | Unique per event |
| `name` | String | Required | e.g., `app.opened`, `rule.created` |
| `ts` | ISO8601 | Required | Event timestamp (UTC) |
| `session_id` | String | Required | Anonymous hash of (install_time + device_model); rotates on wipe |
| `app_version` | String | Required | SemVer format (e.g., `0.0.1`) |
| `os_version` | String | Required | e.g., `iOS 17.0`, `Android 14` |
| `props` | JSON | Optional | Custom event properties (pre-scrubbed) |

### What is NEVER Collected

- ❌ `user_id`, email, real name
- ❌ Task titles, rule conditions, or content
- ❌ Connector credentials, API keys, bearer tokens
- ❌ IP addresses, device identifiers (IDFA, etc.)
- ❌ Health data (heart rate, steps, sleep)

## Emitted Events

| Event | Trigger | Props |
|-------|---------|-------|
| `app.opened` | App launch | `cold_start: bool` |
| `app.closed` | App backgrounded/closed | `session_duration_minutes: int` |
| `app.onboarded` | User completes onboarding | `completed_at: ISO8601` |
| `connector.connected` | User authorizes a connector | `connector_type: str` |
| `connector.synced` | Sync completes successfully | `events_pulled: int, duration_ms: int` |
| `rule.created` | User creates a rule | `trigger_type: str, action_count: int` |
| `rule.fired` | Rule evaluated & triggered | `rule_id_hash: SHA256, credits_awarded: int` |
| `session.completed` | Focus session ends | `duration_minutes: int, credits_earned: int` |
| `review.viewed` | User opens Weekly or Monthly review | `review_type: str` |
| `onboarding.step_completed` | User completes an onboarding step | `step_name: str` |

## Data Storage & Retention

### Local Buffer (Device)

Events are stored in `$DB_PATH/telemetry_events` SQLite table:

```sql
CREATE TABLE telemetry_events (
    event_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    ts TEXT NOT NULL,
    session_id TEXT NOT NULL,
    app_version TEXT NOT NULL,
    os_version TEXT NOT NULL,
    props TEXT NOT NULL,
    flushed INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Retention:** Buffered events never expire locally. When user disables telemetry, all buffered events are **immediately purged**.

### Remote Endpoint (Server)

- **Default endpoint:** None (set via `FOCALPOINT_TELEMETRY_URL` env var)
- **Flush interval:** Every 15 minutes (when opted in)
- **Batch size:** Up to 1,000 events per flush
- **Retention policy:** 90 days max, then automatic deletion
- **No sharing:** Data never shared with third parties

## PII Scrubbing

All event properties are scrubbed **before buffering** using patterns reused from `SentryPrivacyFilter` (commit 5a4ab69):

| Pattern | Example | Redacted |
|---------|---------|----------|
| Email | `alice@example.com` | `[REDACTED_EMAIL]` |
| Phone | `(555) 555-0123` | `[REDACTED_PHONE]` |
| OAuth token | `Bearer sk_live_...` | `[REDACTED_TOKEN]` |
| UUID/Task ID | `550e8400-...` | `[REDACTED_UUID]` |
| HealthKit metric | `heart_rate: 72` | `[REDACTED_HEALTHKIT]` |

## Audit Logging

Every `telemetry.flush_batch()` creates an audit record:

```sql
CREATE TABLE telemetry_audit (
    id INTEGER PRIMARY KEY,
    event_count INTEGER NOT NULL,
    endpoint_domain TEXT,
    flushed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

Users can verify:
1. When events were sent
2. How many events were in each batch
3. Which domain received the data (e.g., `api.focalpoint.app`)

**Example audit query:**
```sql
SELECT event_count, endpoint_domain, flushed_at 
FROM telemetry_audit 
ORDER BY flushed_at DESC 
LIMIT 10;
```

## Configuration

### Environment Variables

| Var | Type | Default | Effect |
|-----|------|---------|--------|
| `FOCALPOINT_TELEMETRY_URL` | URL | Unset | If unset, buffer persists locally; no transmission |

### AppStorage Keys (iOS)

| Key | Type | Default | Effect |
|-----|------|---------|--------|
| `app.telemetryEnabled` | Bool | `false` | User opt-in toggle (Settings → Diagnostics) |

## User-Facing Privacy Narrative

**From Settings → Diagnostics:**

> **Share usage analytics**
>
> Strictly opt-in. Events are anonymized and scrubbed of personal data. Helps us prioritize features based on actual usage.
>
> What gets sent:
> - App open/close events
> - Feature interactions (rule creation, connector sync, session completion)
> - Device OS and app version
>
> What is NOT sent:
> - Task titles, rule conditions, personal content
> - Email, phone, credentials, API keys
> - Health data (steps, heart rate, sleep)
> - IP address or device ID
>
> You can disable anytime. All buffered events are immediately deleted.

## Integration with iOS App

### Initialization

```swift
let telemetry = TelemetryApi(
    dbPath: "/path/to/focalpoint.db",
    sessionId: sessionHash,      // hash(install_time + device_model)
    appVersion: "0.0.1",
    osVersion: "iOS 17.0"
)
```

### Tracking Events

```swift
let props = ["connector_type": "github", "auth_method": "oauth"]
try telemetry.track(
    eventName: "connector.connected",
    propsJson: JSONEncoder().encode(props)
)
```

### Flush on Timer

The iOS app should flush periodically (e.g., every 15 minutes):

```swift
Task {
    try await telemetry.flush(optedIn: appStorage.telemetryEnabled)
}
```

### Purge on Opt-Out

When user disables telemetry toggle:

```swift
try telemetry.purge()
```

## Testing

### Unit Tests

6+ tests cover:
1. ✅ Event buffering (no endpoint configured)
2. ✅ Flush respect for opt-in flag
3. ✅ Purge on opt-out
4. ✅ PII scrubbing (emails, phones, tokens, UUIDs)
5. ✅ Audit record creation
6. ✅ Session ID persistence

**Run tests:**
```bash
cargo test -p focus-telemetry --lib
```

### Manual QA

1. **Enable telemetry** in Settings → Diagnostics → "Share usage analytics"
2. **Trigger events** (open app, create rule, sync connector, complete session)
3. **Inspect local buffer:**
   ```bash
   sqlite3 ~/.focalpoint/focalpoint.db \
     "SELECT event_id, name, ts FROM telemetry_events WHERE flushed = 0 LIMIT 5;"
   ```
4. **Verify PII scrubbing** — inspect `props` JSON for `[REDACTED_*]` placeholders
5. **Verify audit trail** — check `telemetry_audit` table after flush

## FAQ

**Q: Is my personal data sent to FocalPoint servers?**

A: No. All PII (email, phone, tokens, health data) is automatically scrubbed before buffering. Only anonymized event names, types, and counts are transmitted.

**Q: How long is data kept?**

A: 90 days on our servers, then automatically deleted. No backups, no archival.

**Q: Can I disable telemetry?**

A: Yes, anytime. Toggle off in Settings → Diagnostics → "Share usage analytics". All buffered events are immediately deleted.

**Q: Does this work offline?**

A: Yes. Events are buffered locally and sent once you have internet access (if enabled). If you disable telemetry while offline, buffered events are still purged.

**Q: What about Sentry crash reporting? Is that separate?**

A: Yes. Telemetry and Sentry are independent. You can enable one, both, or neither.

**Q: Can third parties access this data?**

A: No. Telemetry data is never shared with partners, analytics providers, or advertisers.

## References

- **Functional Requirements:** FR-TEL-001 (Event Collection), FR-TEL-002 (Opt-in Consent), FR-TEL-003 (PII Scrubbing), FR-TEL-004 (Audit Logging)
- **Source Code:**
  - `crates/focus-telemetry/src/lib.rs` — Client + event buffering
  - `crates/focus-telemetry/src/pii_scrubber.rs` — PII redaction patterns
  - `crates/focus-telemetry/src/audit.rs` — Audit trail
  - `crates/focus-ffi/src/lib.rs::TelemetryApi` — FFI surface for Swift
  - `apps/ios/FocalPoint/Sources/.../Settings/SettingsView.swift` — Telemetry toggle
- **SentryPrivacyFilter Reference:** commit 5a4ab69
