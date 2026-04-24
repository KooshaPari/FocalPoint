# Troubleshooting Guide

This guide covers the most common issues and how to fix them. If you don't find your problem here, [file a bug report](/faq#reporting-bugs) or ask in [Discord](https://discord.gg/focalpoint).

## Connector Issues

### "Connector shows 'unauthorized' or 'authentication failed'"

**Symptoms:**
- Connector status shows red/yellow in Settings
- Says "Unauthorized" or "Token expired"
- Sync shows 0 events

**Diagnosis:**

1. Check if the OAuth token expired (connectors refresh every 24 hours)
2. Verify network connectivity (try loading a web page)
3. Check if the connector service is down (visit their status page)

**Fix:**

1. Go to **Settings → Connectors → [Connector Name] → Disconnect**
2. Tap **Connect** to re-authorize
3. Sign in again with your credentials
4. Grant permissions when prompted
5. Tap **Settings → Connectors → Sync now**

If re-authenticating doesn't work:
- Clear the app's cache: **Settings (iOS) → General → iPhone Storage → FocalPoint → Offload App** → **Reinstall App**
- Try again

---

### "Connector hasn't synced in 24 hours"

**Symptoms:**
- Connector status shows a sync time from yesterday or earlier
- "Next sync: 2026-04-20 14:00" (past date)

**Diagnosis:**

1. Connector may have been disconnected
2. Token may have been revoked on the service (password change, session logout)
3. Network issue prevented sync
4. Service returned an error

**Fix:**

1. Manually trigger sync: **Settings → Connectors → Sync now**
2. Check the status text — if it still says the old date after 1 minute, the token expired
3. If token expired, re-authenticate (see above)
4. Check your network: try opening a webpage
5. If still stuck, export your audit chain and [file a bug report](/faq#reporting-bugs)

---

### "Canvas/GitHub/Calendar won't connect"

**Symptoms:**
- Tap **Connect**, get redirected to the service's login page
- After logging in, get sent back to FocalPoint but status still shows "Not connected"
- No error message displayed

**Diagnosis:**

1. Deep-linking may be broken (rare on iOS 16+)
2. Service rejected the authorization request (e.g., revoked permission)
3. Network timeout during token exchange

**Fix:**

1. **Clear browser cookies for that service** (Safari → Settings → Websites → [Service] → Clear Data)
2. Try connecting again
3. If it fails, **try a different Apple ID/account** to see if the issue is account-specific
4. If still stuck, [report the issue on GitHub](https://github.com/KooshaPari/FocalPoint/issues/new?template=bug_report.yml) with:
   - The service name (Canvas, GitHub, etc.)
   - Your iOS version
   - Steps to reproduce
   - A screenshot of the redirect URL (redact sensitive info)

---

## Rules & Evaluation

### "Focus session doesn't trigger / rule never fires"

**Symptoms:**
- Created a rule like "Block TikTok after 6pm"
- 6pm arrives, but TikTok still opens
- No audit log entry for the rule firing

**Diagnosis:**

1. Rule may be disabled
2. Connector may not have synced recently (or not connected)
3. Rule DSL may have a syntax error
4. Rule conditions may not match current state

**Fix:**

1. Check the rule is **enabled**: **Settings → Rules → [Rule Name]** → toggle should be ON (blue)
2. Check the connector: **Settings → Connectors → [Connector]** → status should be green, last sync should be recent
3. Manually trigger evaluation: **Settings → Connectors → Run rules now**
4. Check the audit log: **Settings → Data → Export audit chain** → search for your rule's `rule_id`
5. Review the rule DSL for typos: [Rule DSL reference](/rules/dsl)

**If still stuck:**
- Disable the rule and create a simpler test rule (e.g., "Block Notes at 3:45pm today")
- If the simple rule fires, your first rule had a condition issue
- If no rules fire, the problem is likely the connector or evaluation engine

---

### "Credits don't update after I complete a task"

**Symptoms:**
- Marked a task complete
- Wallet still shows old credit balance
- No audit entry for the credit transaction

**Diagnosis:**

1. Task may not be marked complete (app UI lag)
2. Connector hasn't synced the event yet (Canvas assignment due, but not yet synced)
3. Rule to grant credits is disabled or has wrong conditions

**Fix:**

1. Go back to the task and verify it's marked complete (checkmark visible)
2. Manually sync: **Settings → Connectors → Sync now**
3. Run rules: **Settings → Connectors → Run rules now**
4. Check your wallet: **Home → Wallet** — balance should update within 1 minute
5. If still stuck, export your audit chain and [file a bug report](/faq#reporting-bugs)

---

### "Penalty applied when rule didn't fire"

**Symptoms:**
- You see a penalty in the audit log (e.g., "-50 credits for using TikTok when blocked")
- But you don't remember using the app, or the rule wasn't supposed to trigger

**Diagnosis:**

1. You may have used the app and forgot (check recent notifications)
2. Rule conditions were met, but you didn't notice
3. The audit record timestamp is in a different timezone

**Fix:**

1. Review the audit chain entry for the penalty:
   - **Settings → Data → Export audit chain** → search for the penalty entry
   - Check the `payload_json` for what triggered it
   - Verify the `occurred_at` timestamp
2. If the penalty was wrong:
   - [File a bug report](/faq#reporting-bugs) with the audit chain export
   - Include the exact rule name and conditions
   - Describe what you were doing at that time
3. We can manually issue a credit reversal if the penalty was a bug

---

## iOS & System Issues

### "App crashed on launch"

**Symptoms:**
- FocalPoint opens briefly, then closes immediately
- No error message shown

**Diagnosis:**

1. Corrupted database (rare)
2. Permission issue (FamilyControls, HealthKit)
3. iOS bug or memory pressure

**Fix:**

1. **Force quit the app**: Swipe up from the bottom (or use App Switcher), hold on FocalPoint, tap **Remove**
2. **Reopen the app**: Tap the FocalPoint icon
3. If it crashes again, try **offloading and reinstalling**:
   - **Settings → General → iPhone Storage → FocalPoint → Offload App** → wait 10 seconds
   - **Offload App** again → **Reinstall App**
4. If still crashing:
   - **Check iOS storage**: **Settings → General → iPhone Storage** — if <500MB free, delete something
   - **Restart your iPhone**: Hold power + volume down, drag slider
5. If crashes persist:
   - Capture a crash log: **Settings → Privacy → Analytics → App Analytics** (may have recent FocalPoint crashes)
   - [File a bug report](/faq#reporting-bugs) with the crash log

**To send a crash log to us:**
1. Go to **Settings → Privacy → Analytics → App Analytics**
2. Find "FocalPoint" in the list
3. Tap it and screenshot the stack trace
4. [Email it to support@focalpoint.app](mailto:support@focalpoint.app?subject=Crash%20Report)

---

### "FamilyControls permission pending / app won't enforce"

**Symptoms:**
- You see a yellow banner "FamilyControls pending approval"
- App can pull data from connectors, but doesn't block apps
- Created rules, but they don't actually restrict apps

**Diagnosis:**

1. FamilyControls entitlement is still under review by Apple (1–4 weeks)
2. You're running on a device without Family Sharing set up
3. FamilyControls was revoked (rarely happens)

**Fix:**

**If entitlement is pending (most likely for v0.0.1 TestFlight):**
- This is expected. Enforcement will unlock once Apple approves the entitlement.
- In the meantime, you can:
  - Create and test rules (they show in the audit log as "would trigger")
  - Set up connectors and credits
  - Prepare your workflow for when enforcement activates
- Check the [Status page](/status) for entitlement approval updates

**If entitlement was approved but you still see the warning:**
1. **Restart your iPhone**: Power off, wait 10 seconds, power on
2. Go to **Settings → Family Sharing → Set Up Family Sharing** (if not already set up)
3. Reopen FocalPoint — the banner should disappear

**If you still see the banner after restart:**
- Go to **Settings → Screen Time → Family Controls** and verify FocalPoint is listed
- If FocalPoint is missing, [file a bug report](/faq#reporting-bugs)

---

### "Notification permission pending"

**Symptoms:**
- You see a prompt asking to allow notifications
- You dismissed it, now you don't see prompts anymore
- FocalPoint can't send coaching messages or penalties

**Diagnosis:**

1. You tapped **Don't Allow** on the notification prompt
2. iOS remembers the choice and won't ask again
3. Notifications are now disabled for FocalPoint

**Fix:**

1. **Go to iOS Settings → Notifications → FocalPoint**
2. Toggle **Allow Notifications** to ON
3. Reopen FocalPoint — you should now receive notifications

---

### "Widget shows old data or won't update"

**Symptoms:**
- Your home screen FocalPoint widget shows yesterday's credits or old rule status
- Tapping the widget opens the app, but the widget doesn't refresh
- Widget is stuck on "Loading…"

**Diagnosis:**

1. Widget cache is stale (Widget Center caches for up to 15 minutes)
2. App Group container is corrupted (rare)
3. Widget locked screen access was revoked

**Fix:**

1. **Force reload the widget**:
   - Long press the widget → **Edit Widget** → **Done**
   - Or: Long press → **Remove Widget** → re-add from the app
2. **Force quit FocalPoint**: Swipe up from bottom, hold on the app, tap **Remove**
3. **Wait 1–2 minutes** for the Widget Center cache to expire
4. Reopen FocalPoint and check the widget again

If the widget still shows old data:
1. Go to **Settings → Home Screen Widgets** and verify FocalPoint is toggled ON
2. In FocalPoint **Settings → Data**, tap **Export audit chain** to force the app to sync latest data
3. Try the widget again

---

### "Haptics or sounds not working"

**Symptoms:**
- Rule fires but no vibration/sound
- You hear audio from other apps, but not FocalPoint
- Toggle is ON in Settings

**Diagnosis:**

1. iPhone is in Silent Mode (mute switch on side of phone)
2. FocalPoint sound settings are disabled
3. iOS volume is set to 0 for the app

**Fix:**

1. **Check iPhone mute switch**: The switch on the left side of your iPhone — should be showing the loud speaker icon, not red
2. Go to **Settings → Mascot** and verify:
   - "Sound effects" is ON
   - "Haptic feedback" is ON
3. Tap **Test sounds & haptics** — you should hear/feel feedback immediately
4. If you don't feel vibration:
   - Try a different app (e.g., Messages, to verify haptics work)
   - If other apps have haptics but FocalPoint doesn't, [file a bug report](/faq#reporting-bugs)
5. If Silent Mode is ON:
   - Flip the mute switch off
   - Note: In Silent Mode, vibration may still work depending on your settings

---

## Data & Privacy

### "My audit chain has a gap or missing records"

**Symptoms:**
- Exported audit chain, see records from times A and C, but nothing from time B
- "Last 5000 records" doesn't match your rule firing frequency

**Diagnosis:**

1. Audit log is pruned after 90 days (configurable)
2. Database was corrupted and repaired (records lost)
3. Export limit is 5000 records — if you have more, you'll see the most recent

**Fix:**

1. Check the oldest record in your export: `first_record.occurred_at`
2. If older than 90 days, that's normal (old records were pruned)
3. If gap appears within the past 90 days, [file a bug report](/faq#reporting-bugs) with the export

To preserve your full audit history:
- Export regularly: **Settings → Data → Export audit chain** (weekly or monthly)
- Save exports to a secure location
- FocalPoint will support backup-to-iCloud in Phase 2

---

### "I can't delete my data / deletion failed"

**Symptoms:**
- Tapped **Settings → Data → Delete all my data**
- Got an error or the data is still there after restart

**Diagnosis:**

1. Database is locked by another process (very rare)
2. File permissions issue

**Fix:**

1. **Force quit FocalPoint**: Swipe up from bottom, hold on the app, tap **Remove**
2. Wait 5 seconds
3. Reopen the app
4. Try deletion again: **Settings → Data → Delete all my data**
5. If deletion fails with an error:
   - Screenshot the error
   - [File a bug report](/faq#reporting-bugs) with the error text

**If you need to delete data immediately:**
- **Offload and reinstall the app**: **Settings → General → iPhone Storage → FocalPoint → Offload App** → **Reinstall App**
  - This deletes all app data, but re-downloads the app from the App Store

---

### "Sentry crash reports not sending"

**Symptoms:**
- Enabled "Send crash reports" in Settings
- App crashes, but you don't see an event in Sentry
- Test Sentry event shows "Check Sentry dashboard" but nothing appears

**Diagnosis:**

1. Sentry opt-in toggle is OFF
2. Network issue preventing upload
3. Sentry is unreachable (rare)

**Fix:**

1. Verify Sentry is enabled: **Settings → Diagnostics → Send crash reports** → toggle should be ON (blue)
2. Check your network (try opening a webpage)
3. Tap **Test Sentry event** — wait 5 seconds
4. Check the Sentry dashboard (may take 1–2 minutes for events to appear)

If Sentry still doesn't receive events:
- Go to **Settings → Diagnostics → Privacy & data** to understand what's collected
- [Email support@focalpoint.app](mailto:support@focalpoint.app) with a description of the crash

---

## Performance & Battery

### "App is slow or laggy"

**Symptoms:**
- Scrolling rules list is janky
- Settings screen takes time to load
- Eval tick takes >5 seconds

**Diagnosis:**

1. Too many rules (performance scales with rule count)
2. Too many audit records (database query is slow)
3. Low storage space causing iOS to slow the app

**Fix:**

1. Check your rule count: **Settings → Rules** — count the items
2. If >500 rules, see if you can disable unused ones
3. Check storage: **Settings → General → iPhone Storage** — if <500MB free, delete something
4. Restart iPhone: Power off → wait 10 seconds → power on
5. Try the slow operation again

If still slow:
- [File a bug report](/faq#reporting-bugs) with:
  - Number of rules
  - Number of tasks
  - Total data size (check iPhone Storage)
  - Which operations are slow

---

### "Battery drains fast"

**Symptoms:**
- FocalPoint uses significant battery compared to other apps
- Phone gets warm while FocalPoint is running
- Battery drops 10% in 1 hour

**Diagnosis:**

1. Connector sync is running too frequently (every 5 minutes instead of hourly)
2. Location tracking enabled (some connectors request location)
3. Background app refresh is on and running expensive operations

**Fix:**

1. Go to **Settings (iOS) → Battery → Battery Usage** → find FocalPoint
2. Check if it says "High" — if so, inspect the following:
3. Disable frequent connector syncs: **Settings → Connectors → Sync interval** (if available) → set to "Hourly"
4. Disable location: **Settings (iOS) → Privacy → Location Services → FocalPoint** → toggle OFF (if enabled)
5. Disable background app refresh: **Settings (iOS) → General → Background App Refresh → FocalPoint** → toggle OFF
6. Restart iPhone

If battery still drains:
- [File a bug report](/faq#reporting-bugs) with:
  - Battery usage percentage (from Settings → Battery)
  - Active connectors
  - Number of rules
  - How long the app was open

---

## Sync & CloudKit

### "CloudKit sync is stuck or shows 'unavailable'"

**Symptoms:**
- Enabled CloudKit sync in Settings
- Status says "iCloud unavailable: [reason]"
- Sync won't complete

**Diagnosis:**

1. iCloud account not signed in on the device
2. iCloud+ subscription issue (CloudKit requires iCloud Drive)
3. Network connectivity issue

**Fix:**

1. **Verify iCloud sign-in**: **Settings → [Your Name] → iCloud** — sign in if needed
2. **Enable iCloud Drive**: **Settings → iCloud → iCloud Drive** → toggle ON
3. **Check network**: Try opening a webpage
4. **Restart iPhone**: Power off → wait 10 seconds → power on
5. Go back to FocalPoint and enable CloudKit sync again
6. Tap **Sync now** — should show "Ready to sync" or start syncing

If still stuck:
- Disable CloudKit sync for now: **Settings → Sync across devices → CloudKit Sync** → toggle OFF
- [File a bug report](/faq#reporting-bugs) if you need CloudKit support

---

## Miscellaneous

### "I see an error code or unfamiliar message"

**Symptoms:**
- An error code appears (e.g., "FP-ERR-042" or "DB_LOCK")
- Error message uses technical jargon

**Diagnosis:**

1. Internal app error
2. Rare edge case not covered by the FAQ

**Fix:**

1. Screenshot the error (redact any personal data)
2. [File a bug report](/faq#reporting-bugs) with:
   - The exact error code or message
   - What you were doing when it appeared
   - Your device model and iOS version
   - The audit chain export
3. Or [email support@focalpoint.app](mailto:support@focalpoint.app) directly

---

## Still Stuck?

If your issue isn't here:

1. **Check the [FAQ](/faq)** — there may be a conceptual answer
2. **[Join Discord](https://discord.gg/focalpoint)** — community members can help
3. **[File a bug report](https://github.com/KooshaPari/FocalPoint/issues/new?template=bug_report.yml)**:
   - Include device info, iOS version, app version
   - Describe exact steps to reproduce
   - Attach a screenshot or audit chain export
4. **[Email support@focalpoint.app](mailto:support@focalpoint.app)** for urgent issues

We're here to help!
