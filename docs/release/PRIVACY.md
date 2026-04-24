# FocalPoint Privacy Policy

**Effective Date:** April 23, 2026  
**Last Updated:** April 23, 2026

---

## 1. About FocalPoint

FocalPoint is a connector-first focus management app for iOS 16+ that helps you build sustainable study and work habits through rules-driven screen-time enforcement. The app evaluates your calendar, task assignments, and health data to provide personalized focus policies and track your progress through a reward and penalty ledger.

This Privacy Policy explains how FocalPoint collects, uses, and protects your personal data.

---

## 2. What Data We Collect

### 2.1 Data You Directly Create

When you use FocalPoint, you create the following data on your device:

- **Tasks & Goals:** Title, description, due date, status
- **Focus Sessions:** Start time, duration, number of interruptions
- **Rules & Policies:** Conditions you set (e.g., "lock social apps during morning study hour")
- **Reward & Penalty Ledger:** Your credit balance, penalty tier, streak count, unlock budget

### 2.2 Data We Access with Your Permission

When you connect FocalPoint to external services, we access and read the following:

#### Calendar
- **Source:** Your device's calendar (via EventKit)
- **What we access:** Event titles, times, duration, location metadata
- **How we use it:** To create schedule-aware focus rules (e.g., "lock distraction apps during meetings")
- **Permission:** You grant this explicitly; iOS shows a permission prompt the first time we access your calendar

#### Canvas LMS (Optional)
- **Source:** Your Canvas account (via Canvas REST API)
- **What we access:** Course names, assignment titles, due dates, submission status
- **How we use it:** To trigger focus rules when assignments are due; inform your home screen with upcoming work
- **Authentication:** You authorize via Canvas OAuth; we ask for `assignments:read` and `courses:read` scopes only
- **Token Security:** Your OAuth token is stored securely in your device's Keychain (iOS encrypts this automatically) and is never sent to FocalPoint servers

#### Google Calendar (Optional)
- **Source:** Your Google account (via Google Calendar API)
- **What we access:** Calendar events, titles, times, attendee count
- **How we use it:** Same as device calendar; enables multi-calendar-aware rules
- **Authentication:** You authorize via Google OAuth; scope is `calendar.readonly`
- **Token Security:** Stored in Keychain; never sent to FocalPoint servers

#### GitHub (Optional)
- **Source:** Your GitHub account (via GitHub REST API)
- **What we access:** Public repository names, PR titles, commit activity
- **How we use it:** Optional; some users create rules tied to development productivity (e.g., "unlock reward after 3 commits")
- **Authentication:** You provide a GitHub Personal Access Token (PAT) with `public_repo` scope
- **Token Security:** Stored in Keychain; never sent to FocalPoint servers

### 2.3 Technical Data (Automatically Collected)

- **Device Identifiers:** A locally-generated UUID (created once on first app launch; never sent anywhere)
- **App Crash Logs:** ❌ We do not collect app crashes or analytics
- **App Usage Metrics:** ❌ We do not track how often you use the app or which screens you visit

---

## 3. Where Your Data Lives

### On Your Device (Primary)
All your personal data is stored in a local SQLite database on your device:
- Task/goal metadata
- Rules and policies
- Reward/penalty ledger
- Audit chain (tamper-evident record of all changes)

**Data at Rest:** Protected by iOS file encryption (happens automatically when your device is locked)

**Backup:** If you back up your iOS device to iCloud or a computer, this data is included in that backup. You control whether backups happen via your iOS Settings.

### Secure Keychain (Token Storage)
OAuth tokens for Canvas, Google Calendar, and GitHub are stored in iOS Keychain:
- OS-level encryption
- Accessible only to the FocalPoint app
- Survives device restarts and backups
- Deleted when you uninstall the app

### In Transit (API Calls)
When FocalPoint syncs data from Canvas, Google Calendar, or GitHub, all communication is encrypted via HTTPS. We never intercept, log, or store these communications.

### No Cloud/Server Storage
FocalPoint does not have a backend service. Your data never leaves your device except when you explicitly authorize API calls to Canvas, Google, or GitHub.

---

## 4. How We Use Your Data

### Rule Evaluation & Enforcement
We use your calendar events, task assignments, and health data to evaluate your active rules. For example:
- **Trigger:** Canvas assignment due today + rule says "lock social apps before 3pm on due date"
- **Action:** Device enforces app block until you submit the assignment
- **Data Used:** Assignment title, due date; no personal context beyond this

### Explainability & Transparency
Every time a rule fires, we store a record that explains:
- Which rule triggered
- What conditions were met
- What action was taken (e.g., app block)
- When and why

You can review this history in the app's Activity tab.

### Audit Trail & Tamper Detection
All changes to your reward/penalty state, rules, and focus policies are recorded in an append-only audit chain. This helps you detect unauthorized changes and provides a complete history of your focus behavior.

### Performance Optimization
We analyze your rule evaluation logs locally (on-device only) to optimize sync frequency and rule triggers. This helps us reduce battery drain.

---

## 5. Who We Share Your Data With

**Short answer: Nobody.**

FocalPoint does not share your personal data with third parties. However, when you authorize Canvas, Google Calendar, or GitHub integrations, **you are sharing data directly with those services**, not with FocalPoint:

- **Canvas:** When you authorize FocalPoint to read your assignments, Canvas sees the request coming from your device. Canvas's privacy policy governs their use of that data.
- **Google Calendar:** Same as Canvas. Google sees that the FocalPoint app is reading your calendar on your device. Google's privacy policy applies.
- **GitHub:** Same as above.

**Important:** FocalPoint never acts as an intermediary. We never receive, log, or cache the raw data from these services. We only process it locally on your device to evaluate rules.

---

## 6. How Long We Keep Your Data

### On Your Device
Your data persists until you delete the app. If you delete FocalPoint:
- SQLite database is erased
- Keychain tokens are erased
- No remnant files are left on your device

### In Backups
If your device is backed up to iCloud or a computer, FocalPoint data is included. When you restore from that backup, FocalPoint data is restored. To permanently delete your data, delete the app and then delete the backup.

### On Third-Party Services
Canvas, Google, and GitHub retain logs according to their own privacy policies. FocalPoint does not control their retention.

---

## 7. Your Privacy Rights

### Access Your Data
All your data is stored locally on your device. You can:
- Export your database using the Files app (FocalPoint stores its database at the standard iOS app sandbox location)
- View your audit chain within the app (Activity tab)
- Review your reward/penalty history (Ledger tab)

### Delete Your Data
Simply delete the FocalPoint app. This erases:
- SQLite database
- Keychain tokens
- All cached data

To also remove iCloud backups containing FocalPoint data, go to **Settings → [Your Name] → iCloud → Manage Storage** and remove the backup.

### Revoke Third-Party Access
If you want to stop FocalPoint from accessing your Canvas, Google Calendar, or GitHub account:
1. Open FocalPoint and go to **Settings → Connectors**
2. Select the connector (e.g., Canvas) and tap **Disconnect**
3. (Optional) Visit Canvas, Google, or GitHub directly and revoke the FocalPoint app from your account settings

---

## 8. Data Security

### Local Encryption
Your device's operating system (iOS) encrypts data at rest. When your device is locked, all data is encrypted. When unlocked, only FocalPoint can read its data.

### Keychain Security
OAuth tokens are stored in iOS Keychain, which is:
- Encrypted by the OS
- Protected by your device passcode or biometric (Face ID / Touch ID)
- Never backed up unencrypted

### HTTPS for API Calls
All communication with Canvas, Google, and GitHub is encrypted via HTTPS. We verify SSL certificates to prevent man-in-the-middle attacks.

### Crash Reporting (Sentry, Opt-In)
FocalPoint includes **optional** crash reporting via Sentry. When you enable "Send crash reports" in Settings > Diagnostics:
- **What's sent:** Stack traces (file names + line numbers only), OS version, app build number, and redacted breadcrumbs
- **What's NOT sent:** Task/rule contents, personal identifiers, emails, tokens, or any user data
- **How long:** Crashes are retained for 90 days, then deleted automatically
- **Your control:** Crash reporting is **off by default**. You must explicitly enable it in Settings. Disable anytime to stop sending data.
- **Provider:** Sentry (https://sentry.io) — industry-standard crash tracking service

### No Other External Analytics
We do not use Firebase, Segment, Mixpanel, or similar analytics services. Sentry is the only external service we contact, and only with your consent.

---

## 9. Children's Privacy

FocalPoint is designed for students and adults. If a minor uses FocalPoint:

- **Parental Consent:** Parents/guardians should be aware that FocalPoint can enforce screen-time restrictions on the device
- **Data Minimization:** We only collect task and calendar data necessary to evaluate focus rules; we do not collect personal information like full name, email, or age
- **No Advertising:** FocalPoint does not contain targeted ads or analytics
- **Compliance:** If FocalPoint is used by children under 13 in the US, we comply with COPPA (Children's Online Privacy Protection Act) by not collecting personal information without parental consent

---

## 10. International Compliance

### GDPR (European Union)
If you are in the EU, you have the right to:
- Access all data FocalPoint holds about you (it's all on your device)
- Correct inaccurate data (edit rules/tasks directly in the app)
- Delete your data (uninstall the app)
- Data portability (export your SQLite database)

FocalPoint is the data controller. Since all data is processed locally on your device, we have no legal basis to retain data longer than necessary. You control deletion; we do not operate a remote database.

### CCPA (California)
If you are in California, you have the right to:
- Know what personal data we collect (listed above)
- Delete your data (uninstall the app)
- Opt out of sale or sharing (N/A; we do not sell or share data)

### LGPD (Brazil)
If you are in Brazil, similar rights apply as GDPR. All data is held locally on your device and is under your control.

---

## 11. Changes to This Privacy Policy

We may update this policy occasionally to reflect changes in law or app functionality. If we make material changes, we will notify you via:
- An in-app notification
- Update to the "Last Updated" date at the top of this policy

Your continued use of FocalPoint after changes indicates acceptance of the updated policy.

---

## 12. Contact Us

If you have questions about this privacy policy or FocalPoint's data practices:

**Email:** privacy@focalpoint.app  
**GitHub Issues:** https://github.com/KooshaPari/FocalPoint/issues  
**Mailing Address:**  
```
Koosha Parikh
FocalPoint
[Address to be updated with incorporation]
```

---

## 13. Summary: What We Do & Don't Do

| What FocalPoint Does | Status |
|----------------------|--------|
| Store your data locally on your device | ✅ Yes |
| Encrypt your data with iOS encryption | ✅ Yes |
| Evaluate rules using your calendar & task data | ✅ Yes |
| Sync with Canvas, Google Calendar, GitHub if you authorize it | ✅ Yes |
| Keep an audit trail of all changes (tamper-evident) | ✅ Yes |

| What FocalPoint Does NOT Do | Status |
|----|--------|
| Sell or share your data | ❌ No |
| Send your data to third-party analytics services | ❌ No |
| Log app crashes or usage analytics | ❌ No |
| Host your data on remote servers | ❌ No |
| Track your location | ❌ No |
| Track your web browsing outside the app | ❌ No |
| Display targeted ads | ❌ No |
| Require account creation or login | ❌ No |

---

**Word Count:** 1,547 words  
**Revision:** 1.0  
**License:** CC0 (public domain; feel free to adapt)

---

*Last Updated: 2026-04-23*
