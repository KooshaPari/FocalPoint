# Release Loop: From Main to Community Feedback

This guide walks through the complete FocalPoint release cycle: code merge → TestFlight beta → release notes generation → community announcement → feedback triage.

## Overview

```
main branch commit
  ↓
fastlane beta build (TestFlight)
  ↓
focus release notes generator (markdown/discord/testflight)
  ↓
webhook post to #releases (Discord)
  ↓
community feedback (bug-reports, feature-requests)
  ↓
triage → GitHub Issues → next cycle
```

## Step-by-Step Workflow

### 1. Code Merge to Main

**Prerequisites:**
- Feature branch passes CI (local `task quality` since GitHub Actions is billing-blocked)
- PR approved, code review complete
- CHANGELOG.md and relevant docs updated

**Action:**
```bash
git checkout main
git pull origin main
git merge --no-ff feature/your-feature
git push origin main
```

**What happens next:** GitHub Actions tries to run CI (will fail due to billing), but you've already verified locally.

### 2. Build & TestFlight Beta

**Action:**
```bash
cd apps/ios
fastlane beta
```

**What fastlane does:**
1. Increments build number
2. Runs SwiftUI snapshot tests
3. Signs with distribution cert
4. Uploads to TestFlight
5. Notifies testers (via email)

**Output:** TestFlight build URL + build number

**Example:**
```
Build v0.0.4 (build 42) uploaded to TestFlight
Testers notified: [testers@focalpoint.app]
```

### 3. Generate Release Notes

**Prerequisites:**
- Previous release is tagged (e.g., `v0.0.3`)
- Commits follow conventional commit format (feat/fix/docs/test/perf/chore/refactor)

**Action:**
```bash
# Generate for multiple formats
focus release-notes generate --since v0.0.3 --format md > /tmp/notes.md
focus release-notes generate --since v0.0.3 --format discord > /tmp/notes-discord.json
focus release-notes generate --since v0.0.3 --format testflight > /tmp/notes-testflight.txt
```

**What the generator does:**
1. Walks `git log v0.0.3..HEAD --oneline`
2. Groups commits by conventional type (feat/fix/docs/test/perf/chore/refactor)
3. Extracts subject + body highlights
4. Outputs in requested format (markdown, Discord embeds, or 4000-char TestFlight capped text)
5. Works offline; optional LLM synthesis if `FOCALPOINT_RELEASE_NOTES_LLM` env var is set

**Formats:**

**Markdown** (for CHANGELOG + GitHub Releases):
```markdown
### Added
- Release notes generator (7a1b2c3)
- Discord webhook integration (8d2e3f4)

### Fixed
- CLI formatting issues (9e3f4g5)
```

**Discord** (embeds with emojis):
```json
{
  "content": "🎉 **FocalPoint 0.0.4** Release",
  "embeds": [{
    "title": "FocalPoint 0.0.4",
    "fields": [
      {
        "name": "✨ Added",
        "value": "• Release notes generator\n• Discord webhook integration"
      },
      {
        "name": "🐛 Fixed",
        "value": "• CLI formatting issues"
      }
    ]
  }]
}
```

**TestFlight** (4000-char capped):
```
FocalPoint Release Notes

Added:
• Release notes generator
• Discord webhook integration

Fixed:
• CLI formatting issues
```

### 4. Post to Discord (#releases)

**Prerequisites:**
- Discord server set up with webhook (see Discord Launch Playbook)
- Webhook URL in `.env` (never committed)

**Action:**
```bash
# Load webhook URL
source .env

# Post Discord format release notes
cargo run -p focus-cli -- release-notes generate --since v0.0.3 --format discord | \
  curl -X POST \
    -H 'Content-Type: application/json' \
    -d @- "$FOCALPOINT_DISCORD_WEBHOOK"
```

**Or use the Rust crate directly:**
```rust
use focus_release_bot::*;

let payload = ReleaseNotesPayload::new("0.0.4")
    .with_category("Added", vec![
        "Release notes generator".into(),
        "Discord webhook integration".into(),
    ])
    .with_category("Fixed", vec!["CLI formatting issues".into()]);

post_to_webhook_blocking(&webhook_url, payload)?;
```

**What happens:**
- Bot posts to #releases with categorized embeds
- Community sees summary + links to GitHub discussions
- Reactions (👍, 🐛, 💭) auto-collected for engagement tracking

### 5. Update GitHub Releases

**Action:**
```bash
# Create GitHub release with markdown notes
gh release create v0.0.4 \
  --title "FocalPoint 0.0.4" \
  --notes-file /tmp/notes.md
```

**What happens:**
- Release tagged in git
- Release notes published on GitHub
- TestFlight link posted in release body

### 6. In-App Feedback Capture

**User Journey:**
1. User encounters bug or has idea in TestFlight
2. Taps Settings → Support → "Send Feedback"
3. mailto:feedback@focalpoint.app opens (prefilled with device info + audit summary counts)
4. User types feedback + sends
5. Email arrives at feedback@focalpoint.app (auto-filtered to Zendesk or similar)

**Data Included:**
```
Device: iPhone 15 Pro, iOS 18.2
App Version: 0.0.4 (build 42)
Audit Summary: 8 tasks, 12 rules, 3 connectors active

[User feedback text]
```

**Data Never Included:**
- Rule conditions or names
- Task titles or details
- Connector credentials or tokens
- User identity (device model only)

### 7. Triage & GitHub Issues

**Action (weekly):**
```bash
# Review feedback emails
# For each significant piece of feedback:

gh issue create \
  --title "[TestFlight Feedback] <user issue>" \
  --body "From: feedback@focalpoint.app
  
**User Story:**
<paste feedback summary>

**Device:** iPhone 15 Pro, iOS 18.2
**Build:** v0.0.4 (42)

**Source:** TestFlight in-app feedback
"

# For Discord posts:
gh issue create \
  --title "[Discord #feature-requests] <idea>" \
  --body "From: Discord #feature-requests
  
**Idea:**
<summary from thread>

**Community Interest:** X upvotes, Y comments
**Discord Thread:** [link]
"
```

**Triage Labels:**
- `type:bug` — crashes, unexpected behavior
- `type:feature` — new capability requested
- `type:ux` — flow, clarity, interaction issue
- `feedback:testflight` — from in-app feedback
- `feedback:discord` — from community Discord
- `priority:p0` — blocks critical workflow
- `priority:p1` — affects most users
- `priority:p2` — nice to have

**Action:**
- Assign `p0` issues to next sprint
- Post summary of `p1` issues to Discord #releases ("**This week's top feedback**: 3 bugs fixed, 2 features in progress")

### 8. Next Cycle

**At sprint planning:**
1. Review top GitHub issues created from feedback
2. Include 2–3 feedback-driven fixes in next sprint
3. Post to Discord: "Based on your feedback, next sprint we're shipping [feature]"
4. Loop back to step 1 on main merge

## CI Integration (fastlane-based)

**If using fastlane lanes:**

```ruby
# fastlane/Fastfile
lane :release do |options|
  tag = options[:tag] || "v0.0.4"
  
  # Build & upload beta
  build_number = increment_build_number
  beta
  
  # Generate release notes
  notes_md = sh("cargo run -p focus-cli -- release-notes generate --since #{tag} --format md").strip
  notes_discord = sh("cargo run -p focus-cli -- release-notes generate --since #{tag} --format discord").strip
  
  # Post to Discord
  sh("curl -X POST -H 'Content-Type: application/json' -d '#{notes_discord}' $DISCORD_WEBHOOK")
  
  # Create GitHub release
  set_github_release(
    repository_name: "KooshaPari/FocalPoint",
    api_token: ENV["GITHUB_TOKEN"],
    name: "FocalPoint #{tag}",
    tag_name: tag,
    description: notes_md,
    is_draft: false,
    is_prerelease: false
  )
end
```

**Run:**
```bash
fastlane ios release tag:v0.0.4
```

## Troubleshooting

### Release notes are empty
- Check `git log v0.0.3..HEAD` returns commits
- Verify commits follow conventional format (`feat: ...`, `fix: ...`, etc.)
- Check current HEAD is ahead of tag: `git describe --tags`

### Discord webhook POST fails
- Verify URL starts with `https://discord.com/api/webhooks/`
- Check webhook still exists (hasn't been deleted in Discord settings)
- Verify network access (run `curl` manually to test)

### TestFlight notes show raw markdown
- Use `--format testflight` instead of `--format md`
- TestFlight truncates to 4000 chars automatically

### Feedback emails aren't arriving
- Check mail rules (may be filtered to spam)
- Verify `feedback@focalpoint.app` is a valid mailbox (or alias)
- Test with curl: `curl -X POST https://... -d '{"subject":"test"}'`

## Metrics & Success Criteria

**Track weekly:**
- Time from main merge to TestFlight live: _< 1 hour_
- Release notes generation time: _< 5 sec_
- Discord announcement reach: _X reactions in first hour_
- Feedback response time: _< 24h from in-app submission to GitHub issue_
- Community issue-to-shipped ratio: _3 issues triaged per 1 shipped_

**Monthly summary (post to #announcements):**
```
📊 **April Release Metrics**
• 4 releases shipped
• 28 community feedback items triaged
• 8 features built from feedback
• 12 bugs fixed (avg 2 days to close)
• 150 new testers joined TestFlight

Thanks for making FocalPoint better! 🙏
```

---

**Next:** See [Discord Launch Playbook](../community/discord_launch_playbook.md) for community setup.
