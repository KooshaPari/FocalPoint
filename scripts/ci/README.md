# FocalPoint Self-Hosted CI Setup

This guide walks through setting up autonomous CI on a Mac Mini (or local dev Mac) using launchd + focus-ci-watcher + fastlane.

## Prerequisites

- **macOS 12+** (launchd support)
- **Rust 1.82+** (for focus-ci-watcher)
- **Fastlane** (installed via `gem install fastlane` or Homebrew)
- **1Password CLI** (for secure secret management)
- **Git** (already present)
- **Xcode 15+** (for xcodebuild, smoke/snapshot/integration lanes)

## Step 1: Install 1Password CLI

The 1Password CLI (`op`) is used to inject secrets at runtime without storing them in code.

```bash
# macOS (Homebrew)
brew install 1password-cli

# Verify installation
op --version
```

Configure your 1Password account:

```bash
# Log in to 1Password (creates ~/.op/config for future commands)
op account add --address kooshapari.1password.com --email kooshapari@gmail.com
```

## Step 2: Store CI Secrets in 1Password

Create a vault or folder called `focalpoint-ci` in 1Password with these items:

1. **App Store Connect API Key** (item name: `appstore-api-key`)
   - Get from: https://appstoreconnect.apple.com/access/api
   - Store as text field: `api_key`

2. **Discord Webhook URL** (item name: `ci-webhook`)
   - Get from: https://discord.com/developers/applications (your bot)
   - Store as text field: `webhook_url`

3. **GitHub Token** (item name: `github-token`, optional)
   - Get from: https://github.com/settings/tokens
   - Store as text field: `token`

Test access:

```bash
# Should print your webhook URL
op read "op://focalpoint-ci/ci-webhook/webhook_url"
```

## Step 3: Build focus-ci-watcher

The watcher polls origin/main and triggers fastlane CI on new commits.

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint

# Build release binary
cargo build --release -p focus-ci-watcher

# Install to /usr/local/bin
sudo cp target/release/focus-ci-watcher /usr/local/bin/

# Verify
focus-ci-watcher --help
```

## Step 4: Set Up Launchd

Copy the plist template and customize for your paths:

```bash
# Copy template
cp scripts/ci/focalpoint-ci.plist ~/Library/LaunchAgents/dev.focalpoint.ci.watcher.plist

# Edit paths if needed (default assumes standard FocalPoint repo location)
nano ~/Library/LaunchAgents/dev.focalpoint.ci.watcher.plist
```

Key settings in the plist:

- `ProgramArguments` — path to `focus-ci-watcher` binary and args
- `StartInterval` — poll frequency in seconds (default: 300 = 5 min)
- `StandardOutPath` / `StandardErrorPath` — log locations (requires writable `/var/log/`)

## Step 5: Prepare Fastlane

Fastlane requires several dependencies. Ensure these are installed:

```bash
# Fastlane itself
gem install fastlane

# Xcode project generation (used by fastlane smoke lane)
brew install xcodegen

# SwiftLint (optional, but recommended for pre-commit checks)
brew install swiftlint
```

Copy the fastlane example env file:

```bash
cd apps/ios/FocalPoint/fastlane

# Copy and review
cp .env.fastlane.example .env.fastlane

# Edit with your values (or use 1Password CLI integration in lane)
nano .env.fastlane
```

## Step 6: Bootstrap Secrets into Environment

The focus-ci-watcher runs as a launchd agent and inherits environment variables. Use 1Password's `op run` to inject them:

```bash
# Create a wrapper script that loads secrets and starts launchctl
cat > ~/.focalpoint-ci-start.sh << 'EOF'
#!/bin/bash
export FOCALPOINT_CI_WEBHOOK=$(op read "op://focalpoint-ci/ci-webhook/webhook_url")
launchctl load ~/Library/LaunchAgents/dev.focalpoint.ci.watcher.plist
launchctl start dev.focalpoint.ci.watcher
EOF

chmod +x ~/.focalpoint-ci-start.sh

# Run once to load the agent
op run -- ~/.focalpoint-ci-start.sh
```

## Step 7: Manual Test

Trigger CI manually to verify everything works:

```bash
# Make a test commit
git commit --allow-empty -m "test: trigger CI"
git push origin main

# Watch the watcher poll and fire
tail -f /var/log/focalpoint-ci-watcher.log

# Check for Discord webhook success in Discord channel
```

Alternatively, force a run without waiting for the next poll:

```bash
# Dry run (shows what would execute, no side effects)
focus-ci-watcher \
  --main-branch main \
  --repo-path /Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint \
  --poll-interval-secs 5 \
  --dry-run
```

## Step 8: Permanent Installation

Once manual testing passes, install as a permanent launchd agent:

```bash
# Load the agent (runs at next boot)
launchctl load ~/Library/LaunchAgents/dev.focalpoint.ci.watcher.plist

# Verify it's running
launchctl list | grep focalpoint

# Start immediately
launchctl start dev.focalpoint.ci.watcher
```

## Operational Commands

### Check Status

```bash
# List agent
launchctl list | grep focalpoint

# View logs (last 50 lines)
tail -50 /var/log/focalpoint-ci-watcher.log

# View errors
tail -50 /var/log/focalpoint-ci-watcher.err
```

### Stop/Start

```bash
# Stop
launchctl stop dev.focalpoint.ci.watcher

# Start
launchctl start dev.focalpoint.ci.watcher

# Unload (removes from auto-start on boot)
launchctl unload ~/Library/LaunchAgents/dev.focalpoint.ci.watcher.plist
```

### Troubleshooting

**Issue: "permission denied" when writing logs**
```bash
# Ensure /var/log is writable
sudo touch /var/log/focalpoint-ci-watcher.log
sudo chmod 644 /var/log/focalpoint-ci-watcher.log
```

**Issue: "focus-ci-watcher not found"**
```bash
# Verify binary in PATH
which focus-ci-watcher

# Reinstall if needed
cargo build --release -p focus-ci-watcher
sudo cp target/release/focus-ci-watcher /usr/local/bin/
```

**Issue: "git clone: permission denied"**
```bash
# Ensure SSH or HTTPS credentials are configured
git config --global credential.helper osxkeychain
git fetch origin  # test connectivity
```

**Issue: Fastlane fails with "xcodegen not found"**
```bash
brew install xcodegen
# Ensure it's in PATH:
which xcodegen
```

## Configuration

### Adjust Poll Interval

Edit the plist and change `<integer>300</integer>` to your desired interval (in seconds):

```xml
<key>StartInterval</key>
<integer>300</integer>  <!-- Change to 60 for 1-minute polls -->
```

Then reload:

```bash
launchctl unload ~/Library/LaunchAgents/dev.focalpoint.ci.watcher.plist
launchctl load ~/Library/LaunchAgents/dev.focalpoint.ci.watcher.plist
```

### Disable Temporarily

```bash
launchctl stop dev.focalpoint.ci.watcher
```

## Integration with Release Bot

On CI success, the watcher posts a success embed to Discord via the `focus-release-bot` library. On failure, it posts the full error output (truncated to 1500 chars for Discord's 2000-char embed limit).

Discord webhook URL is loaded from the `FOCALPOINT_CI_WEBHOOK` env var at agent startup.

## Next Steps

Once launchd MVP is stable for 2 weeks of autonomous CI:

1. **Logs & Monitoring** — Set up log rotation (`logrotate`) and optional centralized logging.
2. **Multi-Machine Scaling** — Deploy Forgejo + Woodpecker runner for distributed builds.
3. **Dashboard** — Add a simple web UI to view past CI runs and logs.
4. **Notifications** — Expand Discord alerts with performance trends (build time, test count).

## References

- [focus-ci-watcher docs](../../crates/focus-ci-watcher/Cargo.toml)
- [Self-hosted CI design](../deployment/self_hosted_ci_2026_04.md)
- [1Password CLI docs](https://developer.1password.com/docs/cli/)
- [Fastlane docs](https://docs.fastlane.tools/)
- [macOS launchd reference](https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html)
