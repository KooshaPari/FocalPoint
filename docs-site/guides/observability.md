# Observability & Crash Reporting

FocalPoint collects crash reports through **Sentry** to help us identify and fix bugs faster. All data collection respects strict privacy standards: no personal data, task contents, or tokens are ever sent.

## How It Works

### Opt-In Model

Crash reporting is **off by default**. Users must explicitly opt in via:

**Settings → Diagnostics → "Send crash reports"**

When disabled:
- No crash data is collected or sent
- No Sentry client is initialized
- Zero network overhead

When enabled:
- On-device stack traces are captured when the app crashes
- Breadcrumb trail of recent app actions is included (redacted)
- Data is encrypted and sent to Sentry servers
- Retained for 90 days, then automatically deleted

### Privacy-First Design

Every crash event is processed through a **PII redaction filter** before it reaches Sentry. The filter removes:

| Pattern | Example | Redacted |
|---------|---------|----------|
| Email addresses | `user@example.com` | `[REDACTED_EMAIL]` |
| Phone numbers | `(555) 555-0123` | `[REDACTED_PHONE]` |
| OAuth tokens | `Bearer sk_live_abc123` | `Bearer [REDACTED_TOKEN]` |
| Task/rule IDs (UUIDs) | `550e8400-e29b-41d4-a716-446655440000` | `[REDACTED_UUID]` |
| HealthKit values | `heart rate: 75 bpm` | `[REDACTED_HEALTH]` |

**What is NEVER sent:**
- Task or rule contents
- Calendar event details
- Connector credentials or OAuth tokens
- Personal identifiers (names, email, phone)
- Usage patterns or behavior

## Architecture

### Core Components

#### 1. SentrySetup.swift

Initializes the Sentry SDK with user consent and environment configuration.

```swift
// Called when user toggles "Send crash reports" in Settings
SentrySetup.shared.setupIfConsented(userOptedIn: true)
```

**Key features:**
- Respects opt-in preference (`@AppStorage("app.sentryEnabled")`)
- Configures environment (debug vs production)
- Enables crash handler and breadcrumb tracking
- Gracefully handles missing DSN (no-op)
- Idempotent initialization (safe to call multiple times)

#### 2. SentryPrivacyFilter.swift

Implements the `beforeSend` callback to scrub PII before transmission.

```swift
options.beforeSend = { event in
    return SentryPrivacyFilter.filter(event)
}
```

**Redaction scope:**
- Event messages
- Exception values
- Breadcrumb messages and data
- Request/response bodies
- Nested dictionaries

#### 3. Developer Test Button

Available in **Settings → Developer → "Test Sentry event"** (DEBUG builds only).

- Respects the user's Sentry opt-in preference
- Fires a test error with tags: `test_event=true`, `source=settings_debug`
- Displays toast: "✅ Event queued (check Sentry dashboard)"
- Useful for QA and local development

## Testing

### Integration Tests

Tests are located in `Tests/FocalPointIntegrationTests/SentryIntegrationTests.swift`.

| Test | Verifies |
|------|----------|
| `test_sentry_initializes_with_opt_in_only` | No Sentry client until user opts in |
| `test_sentry_initializes_when_opted_in` | Sentry activates when enabled + DSN present |
| `test_captured_message_sanitizes_email_pii` | Email redaction regex works |
| `test_captured_message_sanitizes_phone_pii` | Phone number redaction |
| `test_captured_message_sanitizes_oauth_tokens` | Bearer token redaction |
| `test_captured_message_sanitizes_uuids` | UUID/task ID redaction |
| `test_crash_breadcrumbs_honor_privacy` | Breadcrumbs don't leak PII |
| `test_sentry_setup_singleton` | Singleton pattern enforcement |
| `test_sentry_setup_is_idempotent` | Multiple calls are safe |
| `test_sentry_can_be_disabled_after_opt_in` | User can disable anytime |

**Run tests:**

```bash
# Run Sentry integration tests
xcodebuild test \
  -scheme FocalPointApp \
  -destination "generic/platform=iOS" \
  -testPlan FocalPointIntegrationTests \
  -only-testing "SentryIntegrationTests"
```

### Manual QA

1. **Enable crash reports** in Settings → Diagnostics
2. **Visit Settings → Developer** (unlock via: tap Version 5+ times)
3. **Tap "Test Sentry event"** → Toast appears: "✅ Event queued (check Sentry dashboard)"
4. **Check Sentry dashboard** at https://sentry.io/organizations/focalpoint/issues/
5. **Verify redaction** by inspecting the event's breadcrumbs and messages for any PII

### Local Dry-Run

To verify Sentry works without network access:

1. Set `options.enabled = false` in `SentrySetup.swift` (temporary)
2. Add a breakpoint in `SentryPrivacyFilter.filter(_:)`
3. Run test: Settings → Developer → "Test Sentry event"
4. Verify the filter receives the event and redacts PII

## Configuration

### DSN (Data Source Name)

Sentry requires a project DSN to route events. It's configured in `Info.plist`:

```xml
<key>FocalpointSentryDsn</key>
<string>https://examplePublicKey@o0.ingest.sentry.io/0</string>
```

**If missing or empty:**
- Sentry remains inactive
- No errors are logged (silent no-op)
- App continues normally

### Environment

Environment is automatically set based on build configuration:

- `DEBUG` builds → `"debug"`
- Release builds → `"production"`

This helps filter events in Sentry's dashboard and avoids spam from dev builds.

### Sample Rate

Traces are sampled at **10% of transactions** (`tracesSampleRate = 0.1`). Adjust if needed:

```swift
options.tracesSampleRate = 0.5  // 50% of transactions
```

## Troubleshooting

### "Sentry: No DSN found" (Debug log)

**Expected:** DSN isn't configured in `Info.plist`. Crash reporting is inactive.

**Fix:** Add `FocalpointSentryDsn` to `Info.plist` or ignore for local dev.

### "Sentry: DSN configured but user consent disabled" (Debug log)

**Expected:** DSN is set but user hasn't opted in to crash reports.

**Fix:** User must enable in Settings → Diagnostics → "Send crash reports".

### Test event not appearing in dashboard

1. **Verify opt-in:** Check Settings → Diagnostics → "Send crash reports" is enabled
2. **Check DSN:** Verify `FocalpointSentryDsn` is set in `Info.plist`
3. **Check network:** Ensure app has internet access
4. **Check Sentry project:** Log in to https://sentry.io and select the FocalPoint project
5. **Check filters:** Go to Project Settings → Inbound Filters to ensure no rules are dropping events

### Events showing [REDACTED_*] placeholders

This is **correct behavior**. All PII is intentionally redacted. If you see legitimate app errors masked, review the regex patterns in `SentryPrivacyFilter.swift`.

## References

- **Sentry Documentation:** https://docs.sentry.io/platforms/apple/
- **Functional Requirements:** FR-DIAG-001 (Consent), FR-DIAG-002 (Redaction)
- **Source Code:**
  - `Sources/FocalPointApp/SentrySetup.swift`
  - `Sources/FocalPointApp/Observability/SentryPrivacyFilter.swift`
  - `Tests/FocalPointIntegrationTests/SentryIntegrationTests.swift`

## FAQ

**Q: Is my personal data sent to Sentry?**

A: No. All PII (email, phone, tokens) is automatically scrubbed before transmission. You can disable crash reports anytime in Settings.

**Q: How long is crash data kept?**

A: 90 days from capture, then automatically deleted. Sentry never shares data with third parties.

**Q: Can I disable crash reporting?**

A: Yes, anytime. Go to Settings → Diagnostics and toggle off "Send crash reports". No data will be sent from that point onward.

**Q: Does this work offline?**

A: Crash events are queued locally when offline and sent once network is restored. If you disable crash reporting, offline crashes are discarded (not queued).

**Q: What about breadcrumbs? Is that PII?**

A: Breadcrumbs are automatically redacted along with the rest of the event. For example, if a breadcrumb says "Synced user alice@example.com", it becomes "Synced user [REDACTED_EMAIL]".
