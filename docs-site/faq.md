# Frequently Asked Questions

## What is FocalPoint?

FocalPoint is a connector-first screen-time management app for iOS. Unlike traditional screen-time tools, FocalPoint combines:
- **Connectors** — pull events from Canvas, Google Calendar, GitHub, Apple Health, Fitbit, Strava, and more
- **Rules engine** — define custom policies in a simple DSL to block, delay, or restrict apps when conditions match
- **Rewards & penalties** — earn credits for productive work, spend them on "focus time" exemptions, or face auto-applied penalties for rule violations
- **Audit trail** — tamper-evident append-only log of every decision and state change

All data is stored locally. No cloud sync by default. Enforcement happens via iOS FamilyControls.

## How is FocalPoint different from Apple Screen Time?

Apple Screen Time offers time-based limits: "2 hours of TikTok per day." FocalPoint is **event-driven** and **policy-based**:

| Feature | Screen Time | FocalPoint |
|---------|-----------|-----------|
| Time limits | Yes | Yes (via rules) |
| Require authentication to unlock | Yes | Yes |
| Custom rules based on events | No | Yes |
| Rewards & credits system | No | Yes |
| Connectors (Calendar, GitHub, etc.) | No | Yes |
| Audit trail | No | Yes |
| Local-first, no cloud by default | No | Yes |

**Example:** "Block TikTok after 6pm, unless I earned a 2-hour focus exemption" → impossible in Screen Time, trivial in FocalPoint.

## What connectors does FocalPoint support?

FocalPoint ships with 10 built-in connectors in Phase 1:

1. **Canvas** — LMS events (assignments, submissions)
2. **Google Calendar** — calendar events
3. **GitHub** — commits, PRs, issues, releases
4. **Apple Health** — daily step count, workouts
5. **Fitbit** — sleep, heart rate, steps
6. **Strava** — runs, rides, workouts
7. **Tesla** — vehicle status (Phase 1)
8. **Slack** — message timestamps, reaction events
9. **Linear** — issue tracking (Phase 1)
10. **Discord** — message count, voice time (Phase 2)

See the [Connectors guide](/connectors/) for how to enable each. Need another connector? [Request it on GitHub](https://github.com/KooshaPari/FocalPoint/issues/new?template=feature_request.yml).

## Why rules? Why not just time limits?

Rules are more expressive:

- **"Block Instagram unless I complete my homework"** — Canvas connector watches for completed assignments
- **"Restrict work apps to 9am–6pm"** — Google Calendar connector enforces business hours
- **"Auto-penalty: lost 30 credits if I used TikTok at 2am"** — penalties are data-driven, not arbitrary

See [Write a rule](/rules/) for examples and DSL reference.

## What are credits and penalties?

**Credits** are points you earn for productive actions:
- Complete a Canvas assignment → +50 credits
- Commit to a GitHub repo → +20 credits (configurable)
- Publish a blog post → +100 credits (custom rule)

**Spending credits**: Use them as "focus time" exemptions. For example, "Block TikTok after 6pm" costs 10 credits to bypass for 1 hour.

**Penalties** are points you lose for breaking rules:
- Used TikTok when blocked → -50 credits
- Accessed a restricted app outside business hours → -20 credits

See [Rewards & Penalties](/guides/rewards-penalties) for full reference.

## Do I need an Apple Developer Account?

**No.** FamilyControls entitlement is required, but you don't need a paid developer account:

1. When you first run FocalPoint, the app requests FamilyControls permission via iOS native flow
2. Apple reviews the app and grants the entitlement (1–4 weeks typically)
3. Once approved, any user can download from TestFlight or the App Store — no developer account needed

FocalPoint will launch v0.0.1 via TestFlight while the entitlement is being reviewed.

## Where does my data go?

**Local-first by default**: All rules, tasks, credits, and audit logs live in SQLite on your device. No cloud sync unless you enable it.

**Optional cloud sync** (v0.2): If you enable CloudKit sync, data is encrypted and stored in your iCloud account. Apple does not see contents.

**Sentry crash reporting** (opt-in): If you enable "Send crash reports" in Settings, stack traces and device info are sent to Sentry. **We never collect**:
- Task or rule contents
- Calendar events
- Tokens or credentials
- Personal identifiers
- Usage patterns

See [Privacy & Data](/guides/settings#diagnostics) for full details.

## Is FocalPoint free?

Yes, **FocalPoint Core is free** with these features:
- Unlimited rules and tasks
- All 10 built-in connectors
- Full audit trail
- Local storage

**FocalPoint Plus** (optional, ~$4.99/month) adds:
- AI Coaching (Coachy LLM integration)
- Custom notifications
- Advanced analytics
- Siri shortcuts

**FocalPoint Family** (optional, ~$9.99/month) adds:
- Manage rules for up to 5 family members
- Parental reports dashboard
- Cross-device sync

See [Pricing](/pricing) for details.

## How do I connect Canvas?

1. Go to **Settings → Connectors → Canvas → Connect**
2. Enter your Canvas instance URL (e.g., `https://university.instructure.com`)
3. You'll be redirected to Canvas to authorize FocalPoint
4. Once approved, FocalPoint will pull your assignments and submissions
5. Create rules that respond to Canvas events (e.g., "Block TikTok until Canvas assignment due date")

See [Connect Canvas](/connectors/canvas) for detailed walkthrough.

## How do I connect Google Calendar?

1. Go to **Settings → Connectors → Google Calendar → Connect**
2. Sign in with your Google account
3. Grant FocalPoint permission to read your calendar
4. FocalPoint will sync your events (no modification rights requested)
5. Create rules based on calendar events (e.g., "Restrict work apps to calendar business hours")

See [Connect Google Calendar](/connectors/gcal) for detailed walkthrough.

## How do I connect GitHub?

1. Go to **Settings → Connectors → GitHub → Connect**
2. Sign in with your GitHub account
3. Grant FocalPoint permission to read your public profile and repos
4. FocalPoint will pull your commits, PRs, and issue activity
5. Create rules (e.g., "Block social media if I haven't committed in 4 hours")

See [Connect GitHub](/connectors/github) for detailed walkthrough.

## How do I connect Apple Health?

1. Go to **Settings → Connectors → Apple Health → Connect**
2. Grant FocalPoint permission to read HealthKit data (steps, workouts, sleep)
3. FocalPoint will sync your health metrics
4. Create rules (e.g., "Block TikTok unless I walked 10k steps today")

See [Connect Apple Health](/connectors/apple-health) for detailed walkthrough.

## How do I connect Fitbit?

1. Go to **Settings → Connectors → Fitbit → Connect**
2. Sign in with your Fitbit account
3. Grant FocalPoint permission to read sleep, heart rate, and steps
4. FocalPoint will sync your data
5. Create rules (e.g., "Auto-penalty: lost 50 credits if I got <6 hours sleep last night")

See [Connect Fitbit](/connectors/fitbit) for detailed walkthrough.

## How do I connect Strava?

1. Go to **Settings → Connectors → Strava → Connect**
2. Sign in with your Strava account
3. Grant FocalPoint permission to read activities
4. FocalPoint will pull your runs, rides, and workouts
5. Create rules (e.g., "+100 credits for completing a 10k run")

See [Connect Strava](/connectors/strava) for detailed walkthrough.

## Why is FocalPoint iOS-first?

**Phase 1 focuses on iOS** because:
- iOS FamilyControls API is the gold standard for enforcement
- Most screen-time issues happen on mobile (social media, games, etc.)
- Unified App Store distribution
- Entitlement process is streamlined

**Android support** comes in Phase 5. We'll use `BIND_ACCESSIBILITY_SERVICE` + `PACKAGE_USAGE_STATS` for enforcement. [Follow the roadmap](/roadmap) for updates.

## What is Coachy?

Coachy is your AI coach mascot. When you receive a penalty or have an incomplete task, Coachy offers:
- Supportive nudges ("You're doing great! One more hour of focus…")
- Coaching based on your patterns (Llama 2, Claude, or your configured model)
- Customizable voice (Simlish sounds, text-to-speech, or silent)

Coachy runs **entirely in-device** (no cloud by default). Optional: route to your own LLM endpoint for personalization.

See [Coachy mascot](/mascot/) for customization options.

## Can I disable Coachy's notifications?

Yes. In **Settings → Mascot**, you can:
- Disable "Enable Coachy LLM replies" → use static fallback copy only
- Turn off "Proactive nudges" → no coaching messages
- Toggle "Haptic feedback" and "Sound effects" → mute feedback
- Change voice mode to "Silent" → no audio

See [Mascot settings](/guides/settings#mascot) for full options.

## How do I back up my data?

1. Go to **Settings → Data → Export audit chain**
2. A JSONL file is generated with your last 5,000 audit records (rules, tasks, credits, penalties)
3. Tap **Share** to email, AirDrop, or save to Files
4. The export includes cryptographic hashes for verification

See [Backup & Restore](/guides/backup-restore) for recovery instructions.

## How do I delete my data?

1. Go to **Settings → Data → Delete all my data**
2. Confirm the action (irreversible)
3. All local data is wiped: rules, tasks, credits, audit log, connector tokens
4. App resets to onboarding state

See [Delete my data](/guides/delete_my_data) for details on Sentry and iCloud cleanup.

## What happens if a rule fails?

Rules are evaluated every minute. If a rule fails to fire:

1. Check **Settings → Sync now** → check for errors
2. Verify the connector is connected (green status)
3. Check the rule DSL syntax in **Settings → Rules**
4. Inspect the audit chain: **Settings → Data → Export audit chain**
5. If still stuck, [file a bug report](#reporting-bugs)

See [Troubleshooting](/troubleshooting#focus-session-doesnt-trigger) for more.

## Does FocalPoint work offline?

**Mostly yes**:
- Rules continue to fire based on local events (time, app usage)
- Connectors don't sync (Canvas, GitHub, Calendar won't update)
- Credits and penalties still apply
- Once you go online, connectors sync automatically

## Is my data encrypted?

**On-device**: SQLite uses optional encryption (`cipher`). Currently enabled for v0.1+.

**In transit**: Connector OAuth is HTTPS + TLS 1.3. Sentry is encrypted.

**Audit chain**: Every record is hashed (SHA-256) and linked to the previous record. Tampering is cryptographically detectable.

See [Architecture: Audit Chain](/architecture/audit-chain) for technical details.

## Can I export my data?

Yes. **Settings → Data → Export audit chain** generates a portable JSONL export of your last 5,000 audit records. You own your data.

## Can I contribute to FocalPoint?

Yes! FocalPoint is open-source (MIT OR Apache-2.0). We welcome:
- Bug reports and feature requests
- Connector SDK contributions (add your own data source)
- Translation contributions
- Design and UX feedback

See [Contributing](/governance/contributing) and [SDK](/connector-sdk/) for details.

## How do I report a bug?

1. Go to **Settings → Support → Open bug report**
2. Your device info is pre-populated
3. Describe the issue and include steps to reproduce
4. Hit **Submit** → opens GitHub issues

Or manually: [Create an issue on GitHub](https://github.com/KooshaPari/FocalPoint/issues/new?template=bug_report.yml)

Include:
- Device model and iOS version
- App version (see **Settings → Account**)
- Exact steps to reproduce
- Screenshot if relevant
- Last audit chain export (redacted)

## How do I request a feature?

1. Go to **Settings → Support → Request feature**
2. Describe what you'd like and why
3. Hit **Submit** → opens GitHub

Or manually: [Create a feature request](https://github.com/KooshaPari/FocalPoint/issues/new?template=feature_request.yml)

## How do I get help?

- **Quick answers**: Check this FAQ
- **Troubleshooting**: See [Troubleshooting guide](/troubleshooting)
- **Discord community**: [Join our server](https://discord.gg/focalpoint)
- **Bug reports**: [GitHub issues](https://github.com/KooshaPari/FocalPoint/issues)
- **Email**: support@focalpoint.app

## Is there an enterprise or commercial license?

Yes. FocalPoint is available under:
- **MIT** — open-source, no restrictions
- **Apache-2.0** — open-source, grants patent rights
- **Commercial** — for closed-source/proprietary use

For enterprise, licensing, or partnership inquiries: [Contact maintainers](mailto:commercial@focalpoint.app)

## How is FocalPoint funded?

FocalPoint is open-source and volunteer-maintained. Revenue from optional Plus/Family tiers funds ongoing development.

We do **not** and will **not**:
- Sell user data
- Run ads
- Inject trackers
- Require cloud accounts
- Restrict features by nationality or IP

See [Contributing](/governance/contributing) for sponsorship opportunities.

## What's the roadmap?

See [Roadmap](/roadmap) for Phase 2–5 plans, including:
- Phase 2: Cloud sync, advanced analytics
- Phase 3: Siri shortcuts, HomeKit integration
- Phase 4: Mac app (AppKit)
- Phase 5: Android, Wear OS

Current status: Phase 1 (Core + 10 connectors).
