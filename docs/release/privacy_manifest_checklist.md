# Privacy Manifest Checklist — App Store Submission

**Last Verified:** 2026-04-23  
**Compliance Level:** iOS 17+

## Manifest Completeness

- [x] **NSPrivacyAccessedAPITypes** declared for all sensitive APIs
  - EventKit (Calendar read-only): `DBA.1` (previewing day schedule, detecting conflicts)
  - HealthKit (Health data): `CA92.1` (reading workouts, sleep, steps for reward rules)
  - UserDefaults (Settings): `CA92.1` (storing user preferences, audit record IDs)
  - FileTimestamp (Local file access): `C617.1` (audit chain timestamp verification)
  - UserNotificationCenter (Notifications): `AC6.1` (rule-triggered alerts)

- [x] **NSPrivacyTracking: false** — No user tracking enabled
- [x] **NSPrivacyTrackingDomains: []** — Empty (tracking disabled)

- [x] **NSPrivacyCollectedDataTypes** enumerated
  - ✅ UserID (UUID, unlinked, for app functionality)
  - ✅ ProductInteraction (task metadata, linked, for rules)
  - ✅ PhotoLibrary (placeholder, linked; actual use TBD with photo picker Phase)
  - ✅ OtherData (OAuth tokens, focus sessions, rules, rewards, audit chain) — all linked
  - ✅ CrashData (Sentry opt-in, unlinked, crash reporting only)

## Info.plist Usage Descriptions

All required keys present and **human-readable** (not generic):

| Key | Description | Status |
|-----|-------------|--------|
| `NSCalendarsFullAccessUsageDescription` | Calendar access for Morning Brief + schedule conflict detection | ✅ |
| `NSHealthShareUsageDescription` | HealthKit read for workouts, sleep, steps (reward rules) | ✅ |
| `NSHealthUpdateUsageDescription` | HealthKit read access (same as share) | ✅ |
| `NSUserNotificationsUsageDescription` | Notifications for rule triggers, rewards, penalties | ✅ |
| `NSLocalNetworkUsageDescription` | Local sync across devices on same WiFi | ✅ |

**Future Additions (Phase 1.5+):**
- `NSCameraUsageDescription` — QR code unlock (Phase 1.5)
- `NSFaceIDUsageDescription` — Biometric unlock (Phase 1.5)
- `NSBluetoothAlwaysUsageDescription` — (unlikely unless Wear integration added)
- `NSContactsUsageDescription` — (if contact-based invites added)

## Audit Tool Verification

Run privacy manifest audit before submission:

```bash
cd apps/ios/FocalPoint/scripts/audit-privacy
cargo build --release
./target/release/audit-privacy
```

**Expected Output:**
```
✅ Privacy manifest is consistent with Swift source
```

**Failure Modes:**
- **ERROR: Undeclared privacy APIs** — New framework import detected but not in PrivacyInfo.xcprivacy
  - Add the API reason code to `Resources/PrivacyInfo.xcprivacy`
  - Run audit-privacy again
  
- **WARNING: Declared but unused APIs** — API declared but no framework import found
  - Remove from PrivacyInfo.xcprivacy if genuinely unused
  - Or verify the import is present in Swift source

## Sentry Integration

- **Crash Reporting:** Opt-in model
  - User enables in Settings → App collects crash data to Sentry
  - Declared as `NSPrivacyCollectedDataTypeCrashData` (unlinked, AppFunctionality only)
  - Sentry SDK declares its own PrivacyInfo.xcprivacy; FocalPoint does not declare sentry.io as a tracking domain
  - **No tracking markers** (`NSPrivacyTracking: false` override in app manifest)

## Widget Privacy (FocalPointWidget)

Separate PrivacyInfo.xcprivacy for widget target:
- FileTimestamp reason `C617.1` (audit chain access)
- DiskSpace reason `E174.1` (widget cache management)
- No user tracking; no personal data collected beyond app context

## Pre-Submission Checklist

Before uploading to App Store:

1. [ ] Run `./scripts/audit-privacy/target/release/audit-privacy` — must pass
2. [ ] Info.plist has all usage descriptions (NSCalendars*, NSHealth*, NSUserNotifications, NSLocalNetwork)
3. [ ] Each description is **specific** (why the app needs this, not generic)
4. [ ] PrivacyInfo.xcprivacy uses correct API type strings (no typos)
5. [ ] NSPrivacyTracking is `false`; NSPrivacyTrackingDomains is empty array
6. [ ] All declared data types have `NSPrivacyCollectedDataTypePurposes: ["AppFunctionality"]`
7. [ ] Sentry SDK (cocoapod) is up-to-date (declares its own privacy manifest)
8. [ ] TestFlight beta testers informed if collecting crash data (Sentry opt-in)
9. [ ] Privacy Policy updated (reference data collection, retention, user rights)
10. [ ] **App Review Notes** mention HealthKit, EventKit, local-only architecture

## Privacy Policy Sections (Reference)

Ensure Privacy Policy covers:

1. **Data Collection**
   - Calendar events (names, times, duration)
   - Health data (workouts, sleep, steps)
   - User settings (app preferences, rules)
   - Audit chain (immutable record of rule enforcement)

2. **Data Retention**
   - All data stored locally on device (SQLite)
   - No server upload (Phase 0)
   - Sentry crash data: 30 days (Sentry default)

3. **User Rights**
   - Delete data: Reset app (clears local database)
   - Control Health access: iOS Settings → Privacy → Health
   - Control Calendar access: iOS Settings → Privacy → Calendars
   - Disable crash reporting: App Settings → Crash Reporting (toggle)

4. **Third-Party Sharing**
   - Sentry: Only crash stack traces (opt-in)
   - Canvas, Google, GitHub: OAuth tokens stored locally only (no server sync)

## App Store Review Notes

Include in submission:

> FocalPoint is a focus management and rule engine with local-first architecture:
> 
> - **HealthKit:** Read-only access to workouts, sleep, steps to fuel reward/penalty rules. User controls this in Settings → Privacy → Health.
> - **EventKit:** Read-only access to calendar events to preview the day and detect schedule conflicts with focus blocks.
> - **UserNotifications:** Sends local notifications when rules trigger (rewards, penalties, focus updates).
> - **Crash Reporting:** Opt-in Sentry integration (user can disable in Settings).
> - **Data Storage:** 100% local SQLite; no cloud backup or server sync in Phase 0.
> - **No Tracking:** NSPrivacyTracking is disabled. We do not track user behavior for advertising.

---

**References:**
- [Apple Privacy Policy Requierments](https://developer.apple.com/app-store/app-privacy-policy-requirements/)
- [Privacy Manifest API Reason Codes](https://developer.apple.com/documentation/bundleresources/privacy_manifest_files/privacy_apis_requiring_reasons)
- [PrivacyInfo.xcprivacy Schema](https://developer.apple.com/documentation/bundleresources/privacy_manifest_files)
