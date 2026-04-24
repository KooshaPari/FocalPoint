# Diagnostics & Crash Reporting

**Version:** 1.0  
**Updated:** April 23, 2026

---

## Overview

FocalPoint includes **optional crash reporting** powered by Sentry. This document explains what data is collected, how it's protected, and how to control it.

---

## Crash Reporting (Opt-In)

### What It Does

When enabled in Settings > Diagnostics > "Send crash reports", FocalPoint automatically reports app crashes to Sentry. This helps us quickly identify and fix bugs that affect your experience.

### User Consent

- **Default:** OFF. Crash reporting is disabled by default per Apple privacy guidelines.
- **How to Enable:** Go to Settings > Diagnostics and toggle "Send crash reports" ON.
- **How to Disable:** Same location; toggle OFF anytime. No additional action required.

---

## What Data Is Sent

### Collected (Only When Enabled)

1. **Crash Stack Traces**
   - File names and line numbers where the crash occurred
   - Function/method names in the call stack
   - Thread state (active, paused, etc.)

2. **Device & Environment**
   - iOS version (e.g., "17.4.1")
   - App build number and version (e.g., "0.0.1+42")
   - Device model (e.g., "iPhone 15 Pro")
   - Build configuration (Debug vs Production)

3. **Crash Metadata**
   - Timestamp of the crash
   - App memory usage at crash time
   - Whether device was low on memory

4. **Breadcrumbs (Redacted)**
   - Recent app actions (e.g., "Opened Settings", "Tapped Sync")
   - User interactions (taps, screen transitions)
   - **All personal data is redacted:**
     - Email addresses → [REDACTED_EMAIL]
     - UUIDs (task/rule IDs) → [REDACTED_UUID]
     - OAuth tokens → [REDACTED_TOKEN]

### NOT Collected

- ❌ Task or rule contents
- ❌ Calendar event details
- ❌ Personal identifiers (names, emails, phone numbers)
- ❌ OAuth tokens or API keys
- ❌ Canvas, Google Calendar, or GitHub account information
- ❌ Wallet balance, penalty records, or reward history
- ❌ Frequency of app usage or behavior patterns
- ❌ Network traffic or IP address

---

## Data Privacy & Protection

### Transmission

- All crash data is **encrypted in transit** using HTTPS TLS 1.3+
- Encryption happens on your device before leaving the network
- We verify Sentry's SSL certificate to prevent interception

### Storage

- Crash data is stored on Sentry's secure servers in the United States
- Sentry applies its own data protection measures (see https://sentry.io/security/)
- We do not store crash data; Sentry manages retention

### Retention

- Crash reports are automatically **deleted after 90 days**
- You can delete any crash report manually in the Sentry dashboard
- Upon app uninstall, no further crash data is sent

### Sharing

- FocalPoint **never shares** crash data with third parties
- Sentry may process data on our behalf per their Data Processing Agreement (DPA)
- Only FocalPoint's development team views crash reports (via Sentry)

---

## Redaction Examples

### Before & After

| Scenario | Original | Redacted |
|----------|----------|----------|
| Email in breadcrumb | "Synced events for alice@example.com" | "Synced events for [REDACTED_EMAIL]" |
| Task UUID | "Opened task 550e8400-e29b-41d4-a716-446655440000" | "Opened task [REDACTED_UUID]" |
| OAuth token | "Canvas token starts: abcdef123456789..." | "Canvas token starts: [REDACTED_TOKEN]..." |

---

## Your Control

### Enable/Disable

1. Open FocalPoint
2. Go to **Settings** tab
3. Scroll to **Diagnostics** section
4. Toggle **"Send crash reports"** ON or OFF
5. Changes take effect immediately

### View Privacy Info

- Tap the **"Privacy & data"** link in Settings > Diagnostics to see what data Sentry collects

### Revoke Consent

Disabling crash reporting:
- Stops sending new crash data immediately
- Does NOT delete previously sent reports (they're already on Sentry's servers)
- To delete all your crash data, contact us at privacy@focalpoint.app

---

## Technical Details

### Implementation

- **Service:** Sentry (https://sentry.io)
- **SDK:** sentry-cocoa v8.45.0+
- **Sampling:** 100% (all crashes reported); 10% of performance transactions
- **Automatic Unwinding:** Stack traces are automatically unwound and uploaded

### Framework

Crash reporting is integrated into the app's startup sequence:
1. App checks user's consent (stored in @AppStorage("app.sentryEnabled"))
2. If consent is ON, Sentry initializes with PII redaction rules
3. Crashes are automatically caught and reported
4. User can toggle consent in Settings > Diagnostics

---

## App Store Compliance

FocalPoint's privacy declarations include:

- **NSPrivacyCollectedDataType:** CrashData (when enabled)
- **NSPrivacyTracking:** false (crash data is not used for tracking)
- **Linked Data:** false (crashes are not linked to user identity)

This meets Apple's App Store privacy requirements for crash reporting SDKs.

---

## FAQ

### Q: Is crash reporting on by default?
**A:** No. It is OFF by default per Apple privacy guidelines. You must explicitly enable it in Settings.

### Q: Can you see my tasks or rules in crash reports?
**A:** No. Crash stack traces only include file names and line numbers. Task/rule data is redacted by Sentry's filters before transmission.

### Q: What if I delete the app?
**A:** When you uninstall FocalPoint, no more crash data is sent. Previously sent reports remain on Sentry's servers (with automatic 90-day deletion). You can contact us to request manual deletion.

### Q: Does disabling crash reporting delete old reports?
**A:** No. Disabling stops future reports but does not delete past ones (they're on Sentry's servers). Sentry automatically deletes reports after 90 days.

### Q: Who can see my crash reports?
**A:** Only FocalPoint's development team (via Sentry). We do not sell or share crash data with other companies.

### Q: Is my data encrypted?
**A:** Yes. In transit (HTTPS TLS 1.3+) and at rest (Sentry's servers use encryption). Your device encrypts the crash payload before sending it.

### Q: Can I opt out on behalf of a minor using my device?
**A:** Yes. Go to Settings > Diagnostics and keep "Send crash reports" OFF. This setting applies to all users of the app on the device.

---

## Contact & Support

If you have questions about crash reporting or privacy:

**Email:** privacy@focalpoint.app  
**GitHub Issues:** https://github.com/KooshaPari/FocalPoint/issues  
**App Support:** In Settings, tap Help & Support

---

## Updates

This document is kept up-to-date whenever crash reporting functionality or privacy practices change. Last updated: **April 23, 2026**.
