# FocalPoint Discord Community Launch Playbook

Welcome to the FocalPoint community! This playbook guides you through setting up and operating the Discord server for community feedback, feature requests, and bug reports.

## Recommended Channel Structure

### Tier 1: Foundation
- **#welcome** — Pinned message with CoC, getting started links, and Coachy introduction
- **#announcements** — Release notes, platform updates, and milestones (announcement-only channel)
- **#rules-showcase** — Community-shared rules, template packs, and best practices

### Tier 2: Feedback & Support
- **#feature-requests** — Upvote-friendly channel for feature ideas; linked to GitHub Issues
- **#bug-reports** — Issue triage; include device/OS info; bot auto-links to GitHub
- **#support** — General questions, how-tos, troubleshooting

### Tier 3: Community
- **#releases** — Automated release notes posts (webhook integration)
- **#general** — Off-topic, wins, user stories
- **#developer-chat** — Rust internals, FFI, connector SDK discussion

### Tier 4: Archive (Optional)
- **#coachy-lore** — Mascot character development, art, sound design
- **#feedback-archive** — Closed/resolved feedback threads for reference

## Bot Setup: Minimal Discord Webhook Integration

### Prerequisites
- Discord server with manage webhooks permission
- Webhook URL from Discord (keep private; use `.env.example` pattern)

### Installation

1. **Create a webhook:**
   - Server Settings → Integrations → Webhooks → New Webhook
   - Name: "FocalPoint Release Bot"
   - Channel: #releases
   - Copy the webhook URL (format: `https://discord.com/api/webhooks/{id}/{token}`)

2. **Store the URL securely:**
   - Never commit to git
   - Use `.env.example` for documentation:
     ```env
     # .env.example
     FOCALPOINT_DISCORD_WEBHOOK=https://discord.com/api/webhooks/...
     ```
   - Load at runtime in CI/CLI: `source .env && focus release-notes ...`

3. **Wire into CI:**
   - After fastlane beta build succeeds:
     ```bash
     focus release-notes generate --since v0.0.3 --format discord | \
       curl -X POST \
         -H 'Content-Type: application/json' \
         -d @- "$FOCALPOINT_DISCORD_WEBHOOK"
     ```
   - Or use `focus-release-bot` Rust crate directly in a CI script

### Release Notes Auto-Post

Every release cycle:
1. `focus release notes generate --format discord --since <previous-tag>` outputs embeds
2. Pipe or POST to webhook URL
3. Bot posts summary to #releases with categorized bullets and reaction emojis

Example output:
```
✨ Added
  • New release notes generator
  • Discord webhook integration
🐛 Fixed
  • CLI formatting issues
```

## Moderation Basics

### Code of Conduct
- Link to **[Contributor Covenant](https://www.contributor-covenant.org/)** or **[MPL-2.0 CoC](https://github.com/mozilla/inclusion)**
- Pinned in #welcome and #rules-showcase
- Violation reporting: private message moderators

### Moderation Tools
- **Auto-archive:** Threads older than 7 days
- **Slowmode:** 1 minute in #general to prevent spam
- **Word filter:** Block spam keywords (managed via Discord settings)
- **Role assignments:** "Contributor" role for issue submitters who become regulars

### Escalation
- **Level 1** — Warn via thread reply
- **Level 2** — Remove from channel (tempban 24h)
- **Level 3** — Full server ban (notify via email if possible)

## Onboarding Message Template

Post to #welcome and pin:

```
Welcome to FocalPoint! 👋

We're building a community-first screen-time platform.
Your feedback shapes what we build next.

🤖 **Meet Coachy** — Your supportive AI coach who learns your rhythm
📱 **iOS-First** — Native app with connectors (Google Calendar, GitHub, Canvas)
🎯 **Rules-Driven** — You control what triggers rewards, penalties, and interventions

**Getting Started**
1. Read #rules-showcase to see how other users structure their rules
2. Share your setup in #general
3. Found a bug or have an idea? Post in #bug-reports or #feature-requests

**Community Norms**
- Be kind; Coachy is learning too
- Share context (device, OS version, steps to reproduce for bugs)
- Link to the issue you're discussing (auto-archived threads)
- [Code of Conduct](https://www.contributor-covenant.org/) applies

**Feedback Funnel**
- 💭 Early idea? Start in #feature-requests
- 🐛 Found a bug? Post in #bug-reports
- 📱 TestFlight beta? Send feedback via in-app "Send Feedback" button
- 🔗 GitHub Issues? Link to them here for visibility

Questions? Ask in #support. We're here to help!
```

## Feedback Funnel: From Discord to GitHub Issues

### Flow
1. **Discord post** (e.g., #feature-requests) → reaction emoji triage
2. **Bot or moderator** scans reactions, creates GitHub Issue with Discord link
3. **GitHub issue** references Discord thread (Discord link in PR/issue comments)
4. **Resolution post** returned to Discord thread with issue link

### Tools
- **Discord bot integration** (optional, v2): auto-create issues from starred/pinned messages
- **Issue template**: Link to Discord thread (requires OpenAPI GitHub integration or manual copy)
- **Cron job** (optional): Weekly digest of top upvoted feature requests → GitHub discussion

### Example GitHub Issue Template

```markdown
## Feedback Source
Discord: #feature-requests / [thread link]

## User Story
[Copy from Discord thread or synthesize from votes]

## Expected Behavior
[What the user wants]

## Current Behavior
[What happens now]

## Impact
[How many upvotes in Discord; community size context]

## Community Interest
- 👍 X upvotes
- 💬 X comments
- Discord link: [URL]
```

## Feedback Capture: iOS In-App

### Implementation
iOS Settings → Support → "Send Feedback" row

**Tap behavior (choose one):**

**Option A: mailto (simpler)**
```
mailto:feedback@focalpoint.app?subject=FocalPoint+Feedback&body=
Device: [iOS version, model]
App Version: [build number]
Audit Summary: [task count, rule count, connector count] (no sensitive data)

[User message]
```

**Option B: GitHub Issue Template (richer)**
```
Safari.open("https://github.com/KooshaPari/FocalPoint/issues/new?title=
[iOS+Feedback]&body=...")
```

**Recommended:** Use **Option A** (mailto) for MVP—no GitHub API auth needed, user defaults to their email client, naturally archives in email.

### Data Captured
- **Allowed:** Device OS version, app build number, audit summary counts (tasks, rules, connectors)
- **Never:** audit contents, user data, rule conditions, task titles, connector tokens

### Feedback Loop Closure
- Automated response from support email with link to GitHub issues
- Monthly digest of top feedback themes posted to #general
- Implement 2–3 top community requests in each release cycle

## Analytics & Metrics

Track weekly:
- **Server growth:** #welcome joins
- **Feedback volume:** Issues per week (created from Discord)
- **Feature request engagement:** Avg reactions per feature request
- **Resolution time:** Days from Discord thread to GitHub issue closed

Dashboard: Post monthly summary to #announcements (e.g., "This month: 15 new members, 8 features shipped, 12 bugs fixed—thanks to your feedback!").

## FAQ for Moderators

**Q: How do I handle duplicate feature requests?**
A: Link to the original GitHub issue in your reply; ask user to comment there to add their use case.

**Q: What if someone reports a security vulnerability in #bug-reports?**
A: Move to DMs immediately. Direct them to SECURITY.md for the disclosure process.

**Q: Can I pin my own rules in #rules-showcase?**
A: No—only moderators or auto-posts. But react with ⭐ to rules you like; top ones get re-shared.

**Q: Release notes are too short. Can we synthesize with an LLM?**
A: Yes! Set `FOCALPOINT_RELEASE_NOTES_LLM=true` and the CLI will call an LLM to summarize. Requires API key (optional).

---

**Ready to launch?** Start with Tier 1 channels (#welcome, #announcements, #rules-showcase) and #releases. Add #feature-requests and #bug-reports once you have 20+ members. Scale to Tier 3 as engagement grows.
