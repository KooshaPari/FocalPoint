# FocalPoint Release Process

End-to-end TestFlight release orchestration via `release-cut` binary and fastlane.

## When to Cut a Release

**Criteria:**
- All PRs to main merged and verified locally
- `task verify` passes (fmt + lint + test + iOS build)
- No blocking issues in the current sprint
- At least one shipping feature or critical fix since last tag

**Typical cadence:** Weekly (Fridays) or ad-hoc for critical fixes.

## Dry-Run: Review the Plan

```bash
# From FocalPoint root
task release:dry
# Prompts: "Version (e.g., v0.0.7): " → enter v0.0.7

# Output:
# ┌─ Release Plan: 0.0.7 ─────────────────────────────────┐
# │ 1. Git Tag: git tag -a v0.0.7 -m 'FocalPoint 0.0.7'
# │ 2. Version Bumps: Cargo.toml, iOS plist
# │ 3. CHANGELOG: Prepend release section
# │ 4. Discord: Post to #releases webhook
# │ 5. FastLane: fastlane ios beta version:0.0.7
# └────────────────────────────────────────────────────────┘

# To execute: task release:cut
```

**What the plan includes:**
- Git tag creation and push
- `Cargo.toml` version bump (workspace-level)
- `apps/ios/FocalPoint/Sources/FocalPointApp/Info.plist` version bump
- CHANGELOG.md prepended with release notes (from `focus release-notes`)
- Discord #releases webhook post with categorized changes + commit summary
- FastLane beta lane invocation (increments build, signs, uploads to TestFlight)

## Execute the Release

```bash
# Set Discord webhook (from Discord server settings → integrations)
export FOCALPOINT_DISCORD_WEBHOOK="https://discord.com/api/webhooks/..."

# Execute
task release:cut
# Prompts: "Version (e.g., v0.0.7): " → enter v0.0.7

# Process (15–45 min depending on fastlane build time):
# ✓ Bumping Cargo.toml version to 0.0.7
# ✓ Bumping iOS plist to 0.0.7
# ✓ Generating CHANGELOG section via 'focus release-notes'
# ✓ Committing version and CHANGELOG updates
# ✓ Creating annotated git tag: v0.0.7
# ✓ Pushing tag to origin: v0.0.7
# ✓ Posting release announcement to Discord #releases
# ✓ Invoking fastlane beta build
#   $ cd apps/ios && fastlane ios beta version:0.0.7
# ✅ fastlane beta build submitted to TestFlight

# ✅ Release 0.0.7 complete
```

## What Happens After Execution

### 1. TestFlight Build

FastLane:
- Increments build number (e.g., 42 → 43)
- Runs SwiftUI snapshot tests
- Codesigins with distribution certificate
- Uploads `.ipa` to TestFlight
- Notifies testers via email

**Timeline:** ~10–30 min (most time is provisioning + upload)

### 2. GitHub Release

If `GITHUB_TOKEN` is set, release-cut creates a GitHub release with:
- Release title: "FocalPoint 0.0.7"
- Release notes: Full CHANGELOG section
- TestFlight link (appended by fastlane)

### 3. Discord Announcement

#releases channel post:
```
🎉 **FocalPoint 0.0.7** Release

✨ Added
• Feature A (abc1234)
• Feature B (def5678)

🐛 Fixed
• Bug fix X (ghi9012)

📲 Install via TestFlight (link in reply)
```

Reactions auto-collected for engagement tracking.

### 4. Community Feedback Loop

Users can:
1. Install beta via TestFlight email link
2. Encounter bug or request feature
3. Tap Settings → Support → "Send Feedback"
4. Email arrives at feedback@focalpoint.app
5. Triage → GitHub Issues next sprint

## Failure Scenarios & Recovery

### Fastlane Build Fails Midway

**Symptoms:** FastLane timeout, cert error, or provisioning profile issue.

**Recovery:**
```bash
# 1. Review fastlane logs
# 2. Fix the underlying issue (e.g., renew cert, update provisioning)
# 3. Rollback the incomplete release
task release:rollback -- v0.0.7

# 4. Verify git state
git status                    # Should show main clean
git tag | grep v0.0.7         # Tag deleted

# 5. Verify CHANGELOG preserved
git log --oneline | head -5   # Commit for v0.0.7 still there
cat CHANGELOG.md | head -20   # Release section preserved

# 6. Retry
task release:cut
```

### Rollback Procedure

```bash
task release:rollback -- v0.0.7

# What it does:
# ✓ Deletes local tag: v0.0.7
# ✓ Deletes remote tag: git push origin :v0.0.7
# ✓ Resets Cargo.toml to previous version
# ✓ Resets iOS plist to previous version
# ✓ Commits rollback with message: "chore(release): rollback 0.0.7"
# ✓ Pushes rollback commit
# ✓ Preserves CHANGELOG (intentional — documents the attempt)
```

**After rollback:**
- You're back on main with version = previous
- CHANGELOG section for 0.0.7 remains (as a historical record)
- All git state is clean and ready to retry

### Discord Webhook Not Set

If `FOCALPOINT_DISCORD_WEBHOOK` is unset:
- Release proceeds normally
- Discord post skipped (logged as warning)
- Manual workaround: post to #releases manually with `focus release-notes`

### Partial Execution (Network Fails During Push)

If network fails after tag creation but before fastlane:
- Local tag exists but remote doesn't
- Manual recovery: `git push origin v0.0.7`
- Continue with fastlane manually: `cd apps/ios && fastlane ios beta version:0.0.7`

## Rollback Safety Guarantees

| Artifact | Rollback Behavior | Reason |
|----------|-------------------|--------|
| Git tag | ✅ Deleted (local + remote) | Tag is release metadata |
| Cargo.toml version | ✅ Reverted to previous | Version tied to tag |
| iOS plist version | ✅ Reverted to previous | Version tied to build number |
| CHANGELOG.md | ⚠️ Preserved | Documents release attempt; historical record |
| GitHub Release | ❌ Not deleted | Manual cleanup needed (create new release or delete via web UI) |
| TestFlight build | ❌ Not rejected | Manual rejection needed in App Store Connect |

**Manual cleanup after rollback:**
```bash
# If you created a GitHub release
gh release delete v0.0.7 --yes

# If TestFlight build is undesirable
# Log into App Store Connect → TestFlight → FocalPoint → Internal Testing
# Click the build number, then "Reject" or delete
```

## Verification Checklist

After a release executes successfully:

- [ ] `git tag | grep v0.0.7` returns the tag
- [ ] `git log --oneline | head -3` shows version bump commit + tag
- [ ] `CHANGELOG.md` contains v0.0.7 section at top
- [ ] `Cargo.toml` shows version = "0.0.7"
- [ ] iOS plist shows `<string>0.0.7</string>`
- [ ] TestFlight shows new build in Internal Testing
- [ ] Discord #releases has release announcement
- [ ] GitHub releases show v0.0.7 (if GITHUB_TOKEN set)

## Fastlane Lanes Reference

| Lane | Command | Purpose |
|------|---------|---------|
| `beta` | `fastlane ios beta version:0.0.7` | Build, sign, upload to TestFlight, notify testers |

**Manual invocation** (if you need to re-run outside release-cut):
```bash
cd apps/ios
fastlane ios beta version:0.0.7
```

## Troubleshooting

### "not in a git repo with Cargo.toml"
- Ensure you're in the FocalPoint root directory
- Check: `ls Cargo.toml .git` (both should exist)

### "git tag ... already exists"
- Tag already created in a previous run
- Recover: `git tag -d v0.0.7` (delete locally), retry

### "CHANGELOG.md not found"
- Ensure you're in the FocalPoint root
- File should be: `./CHANGELOG.md`

### "failed to push tag; check network and GitHub permissions"
- Verify GitHub SSH key configured: `ssh -T git@github.com`
- Verify write access to repo: `gh repo view`
- Retry manually: `git push origin v0.0.7`

### "fastlane ios beta failed: cert error"
- Review fastlane output for specific cert/provisioning error
- Rerun cert ceremony if needed: `fastlane ios certificates`
- Rollback and retry: `task release:rollback -- v0.0.7`

### "Discord post fails silently"
- Check webhook URL: should start with `https://discord.com/api/webhooks/`
- Verify webhook still exists in Discord (may have been deleted)
- Test manually:
  ```bash
  curl -X POST -H 'Content-Type: application/json' \
    -d '{"content":"test"}' "$FOCALPOINT_DISCORD_WEBHOOK"
  ```

## Environment Setup

**Required:**
- `git` and `gh` CLI configured with GitHub SSH key
- Xcode + fastlane installed
- Distribution certificate + provisioning profile in Keychain

**Optional but recommended:**
- `FOCALPOINT_DISCORD_WEBHOOK` — Discord #releases webhook (skip Discord post if unset)
- `GITHUB_TOKEN` — GitHub API token (skip GitHub release creation if unset)

**One-time setup:**

```bash
# Install fastlane (if not present)
brew install fastlane

# Install fastlane gems
cd apps/ios
bundle install

# Configure certificate ceremony (one-time)
fastlane ios certificates
fastlane ios sigh  # Provisioning profiles

# Test fastlane locally
fastlane ios beta version:0.0.0  # Dry-run with dummy version

# Store webhook in .env (never commit)
echo 'FOCALPOINT_DISCORD_WEBHOOK=https://discord.com/api/webhooks/...' >> .env
source .env
```

## Metrics & Success Criteria

**Per-release:**
- Time from main merge to TestFlight live: < 1 hour (usually 15–30 min)
- Release-cut execution time: < 10 min (excluding fastlane build)
- Discord announcement reach: X reactions in first hour
- Testers notified: All TestFlight testers receive email

**Monthly:**
- Releases shipped: 4+ (weekly or on-demand)
- Community feedback items triaged: 10+
- Features shipped from feedback: 2–3

---

**See also:** `tooling/release-cut/README.md` for `release-cut` CLI reference.
