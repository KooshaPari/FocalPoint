# Self-Hosted CI for FocalPoint (2026-04)

## Executive Summary

GitHub Actions billing constraints block cloud CI. This document designs a self-hosted CI system for FocalPoint using **launchd + fastlane + Rust watcher daemon** (macOS-native) with a **Forgejo Actions fallback** for multi-platform scalability.

**Recommendation: Dual-path approach**
- **Primary:** Launchd + Rust watcher (native, zero infrastructure, <5 min setup)
- **Fallback:** Forgejo Actions (GitHub-compatible YAML, self-hosted, Woodpecker-based)

---

## Problem Statement

- **GitHub Actions billing:** Suspended due to spending limits; no cloud runners available.
- **Local fastlane lanes exist:** `smoke`, `snapshot`, `integration`, `bench`, `bench_full` are proven and runnable.
- **Need:** Automated CI gate that runs tests, lint, clippy, xcodebuild smoke/integration on every `main` push without manual intervention.
- **Constraints:**
  - No user interaction required (hands-off automation).
  - Secrets never stored in code (1Password CLI integration).
  - Concurrent job isolation (per-job temp directories).
  - Failure notifications via Discord webhook (release-bot already built).

---

## Option Analysis

### Option 1: GitHub Actions Self-Hosted Runner (Recommended Primary)

**What:** Official GitHub Actions runner binary, installed as launchd plist on Mac Mini.

**Pros:**
- Uses existing workflow YAML (future integration).
- Official support for macOS runners.
- Secrets stored in GitHub (encrypted).
- No infrastructure changes needed.

**Cons:**
- Requires `.github/workflows/` CI files (currently missing).
- Heavy (~500MB footprint).
- GitHub Runners API adds complexity for multi-machine scaling.

**Verdict:** Good long-term fallback but requires workflow YAML investment. Skip for MVP.

---

### Option 2: Drone CI

**What:** Open-source CI platform with Docker-native pipelines.

**Pros:**
- Excellent for container-based workloads.
- Webhooks trigger builds.
- Secrets via environment variables.

**Cons:**
- **No native macOS runner** without VM orchestration.
- Docker-first design; iOS builds (xcodebuild) awkward in containers.
- Overkill for single-machine scenario.

**Verdict:** Ruled out for iOS-native builds.

---

### Option 3: Woodpecker CI

**What:** Drone fork with better UX and Forgejo integration.

**Pros:**
- Lightweight, self-hosted.
- YAML pipelines.
- Forgejo-native support.

**Cons:**
- Still container-oriented.
- macOS VM support requires additional setup.
- Adds infrastructure complexity.

**Verdict:** Useful as Forgejo fallback, not primary.

---

### Option 4: Dagger

**What:** CI/CD framework using TypeScript/Python to describe pipelines.

**Pros:**
- Language-native pipelines (no YAML).
- Container-agnostic.
- Good for monorepos.

**Cons:**
- Steep learning curve.
- Not "set and forget" — requires active pipeline code.
- macOS support via containers is still awkward.

**Verdict:** Overengineered for this use case.

---

### Option 5: Plain Launchd + Fastlane + Rust Watcher (Recommended Primary) ✅

**What:** Native macOS job scheduler (launchd) running a Rust daemon that polls origin/main and triggers fastlane lanes via shell.

**Pros:**
- **Zero infrastructure.** Native macOS, no Docker, no third-party services.
- **Minimal setup.** Single plist, single Rust binary, single shell script.
- **Proven lanes.** Fastlane smoke/snapshot/integration already work locally.
- **Fast feedback.** No container overhead; native xcodebuild, cargo.
- **Native secrets.** 1Password CLI integration via `op run`.
- **Discord notifications.** release-bot already sends webhooks.
- **Transparent.** Full visibility into what's running; easy to debug.

**Cons:**
- Single-machine only (Mac Mini or dev Mac).
- No distributed scaling.
- No built-in UI dashboard (can add later).

**Verdict:** **Perfect for FocalPoint MVP.** Ship this immediately; scale to Forgejo later if needed.

---

### Option 6: Forgejo Actions (Fallback for Scaling)

**What:** Self-hosted Forgejo instance (fork of Gitea) with Woodpecker CI actions runner.

**Pros:**
- Fully compatible with GitHub Actions YAML syntax.
- True self-hosted: runs on your infrastructure.
- Multi-machine scalability via runner registration.
- Same workflows as GitHub Actions (future portability).

**Cons:**
- Requires Forgejo server (Docker container or binary).
- Requires Woodpecker runner setup.
- More infrastructure to manage.
- Overkill for single machine.

**Verdict:** **Excellent for Phase 2 scaling.** Use as upgrade path when load demands multi-machine runners.

---

## Final Architecture Decision

### Phase 1 (MVP): Launchd + Rust Watcher + Fastlane

**Components:**

1. **`focus-ci-watcher` crate** (Rust, new)
   - Polls origin/main HEAD SHA every 5 minutes.
   - On new commit, clones into `/tmp/focalpoint-ci-${uuid}/` sandbox.
   - Runs `fastlane ci` in that sandbox.
   - Captures output; posts failure alert to Discord via release-bot.
   - Cleans up temp directory after completion.

2. **`focalpoint-ci.plist`** (launchd template)
   - `RunAtLoad=false` (user must `launchctl load`).
   - `KeepAlive=false` (watcher is one-shot; cron triggers via timer).
   - `StartInterval=300` (5-minute polls).
   - Runs: `/usr/local/bin/focus-ci-watcher --main-branch main --discord-webhook-env FOCALPOINT_CI_WEBHOOK`.

3. **`fastlane ci` lane** (Fastfile addition)
   - Sequence: `cargo test`, `cargo clippy`, `cargo fmt --check`, `xcodebuild smoke`, `xcodebuild snapshot`, `xcodebuild integration`.
   - On any failure, returns non-zero exit; watcher posts Discord alert.
   - On success, posts success embed to Discord.

4. **`.env.ci.example`**
   - Lists all required env vars: `APP_STORE_CONNECT_API_KEY`, `FOCALPOINT_CI_WEBHOOK`, etc.
   - No secrets committed.
   - User sources via 1Password CLI: `op run -- launchctl start focalpoint-ci`.

### Phase 2 (Scaling): Forgejo Actions

- Deploy Forgejo + Woodpecker runner on-prem.
- Copy fastlane lanes into `.forgejo-ci.yml`.
- Register runners for macOS (on Mac Mini) and Linux (on shared VM).
- Retire launchd watcher; use Forgejo webhook instead.

---

## Implementation Details

### 1. focus-ci-watcher Crate

**File:** `crates/focus-ci-watcher/src/lib.rs` and `src/main.rs`

**Functions:**

- `fn poll_origin_main(repo_path: &Path) -> Result<String>` — Runs `git ls-remote origin main` and parses HEAD SHA.
- `fn clone_sandbox(repo_url: &str, sha: &str) -> Result<PathBuf>` — Clones into `/tmp/focalpoint-ci-${uuid}`.
- `async fn run_fastlane_ci(sandbox_path: &Path) -> Result<(bool, String)>` — Executes fastlane, captures output.
- `async fn post_discord_alert(webhook_url: &str, success: bool, output: &str) -> Result<()>` — Uses release-bot library to post embed.
- `async fn cleanup_sandbox(path: PathBuf) -> Result<()>` — Removes temp dir.

**Tests:**
- `test_parse_git_sha()` — Mock `git ls-remote` output.
- `test_sandbox_isolation()` — Verify temp dir cleanup.
- `test_fastlane_success_parse()` — Parse exit code correctly.
- `test_discord_webhook_payload()` — Validate webhook format.

**CLI Args (clap):**
```
--main-branch <BRANCH>              (default: main)
--repo-path <PATH>                  (default: .)
--poll-interval-secs <SECS>         (default: 300)
--discord-webhook-env <VAR_NAME>    (e.g., FOCALPOINT_CI_WEBHOOK)
--max-concurrent-jobs <N>           (default: 1)
--temp-base <PATH>                  (default: /tmp)
--dry-run                           (don't actually run fastlane)
```

### 2. Launchd Plist

**File:** `scripts/ci/focalpoint-ci.plist`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>dev.focalpoint.ci.watcher</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/focus-ci-watcher</string>
        <string>--main-branch</string>
        <string>main</string>
        <string>--repo-path</string>
        <string>/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint</string>
        <string>--poll-interval-secs</string>
        <string>300</string>
        <string>--discord-webhook-env</string>
        <string>FOCALPOINT_CI_WEBHOOK</string>
    </array>
    <key>RunAtLoad</key>
    <false/>
    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
    </dict>
    <key>StartInterval</key>
    <integer>300</integer>
    <key>StandardOutPath</key>
    <string>/var/log/focalpoint-ci-watcher.log</string>
    <key>StandardErrorPath</key>
    <string>/var/log/focalpoint-ci-watcher.err</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin</string>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>
</dict>
</plist>
```

### 3. Fastlane :ci Lane

Added to `apps/ios/FocalPoint/fastlane/Fastfile`:

```ruby
lane :ci do
  UI.message("🤖 FocalPoint CI Pipeline Starting...")
  
  # 1. Rust cargo test
  UI.message("1️⃣  Testing Rust crates...")
  Dir.chdir("../../../..") do
    sh("cargo test --workspace")
  end
  
  # 2. Clippy lint
  UI.message("2️⃣  Linting with clippy...")
  Dir.chdir("../../../..") do
    sh("cargo clippy --workspace --tests -- -D warnings")
  end
  
  # 3. Format check
  UI.message("3️⃣  Checking format...")
  Dir.chdir("../../../..") do
    sh("cargo fmt --check")
  end
  
  # 4. Xcodebuild smoke
  UI.message("4️⃣  Running xcodebuild smoke (Mac Designed-for-iPad)...")
  smoke
  
  # 5. Snapshot tests
  UI.message("5️⃣  Running snapshot tests...")
  snapshot
  
  # 6. Integration tests
  UI.message("6️⃣  Running integration tests...")
  integration
  
  UI.success("✅ All CI checks passed!")
end
```

### 4. Secrets & Env Setup

**File:** `.env.ci.example`

```bash
# App Store Connect API Key (Base64-encoded JSON from Apple)
# Source: https://appstoreconnect.apple.com/access/api
# Usage: export APP_STORE_CONNECT_API_KEY=$(op read "op://Private/focalpoint-appstore/api_key")
APP_STORE_CONNECT_API_KEY=

# Discord Webhook for CI failure alerts
# Source: https://discord.com/developers/applications
# Usage: export FOCALPOINT_CI_WEBHOOK=$(op read "op://Private/focalpoint-ci/webhook_url")
FOCALPOINT_CI_WEBHOOK=

# 1Password Service Account (if using op CLI for automation)
OP_SERVICE_ACCOUNT_TOKEN=

# GitHub token (for release notes generation)
GITHUB_TOKEN=

# Fastlane-specific
FASTLANE_USER=
FASTLANE_PASSWORD=
FASTLANE_SESSION=

# Xcodebuild signing identity (optional; usually auto-detected)
DEVELOPMENT_TEAM=GCT2BN8WLL
PROVISIONING_PROFILE_SPECIFIER=
```

**Bootstrap Instructions:**
```bash
# 1. Ensure 1Password CLI is installed
brew install 1password-cli

# 2. Configure 1Password service account
export OP_SERVICE_ACCOUNT_TOKEN="<token>"

# 3. Load secrets into environment
op run -- launchctl load ~/Library/LaunchAgents/dev.focalpoint.ci.watcher.plist

# 4. Verify watcher is running
launchctl list | grep focalpoint

# 5. Tail logs
tail -f /var/log/focalpoint-ci-watcher.log
```

### 5. README: First-Time Setup

**File:** `scripts/ci/README.md`

See below for full 500-word walkthrough.

---

## Deployment Checklist

- [ ] Create `crates/focus-ci-watcher` crate with tests.
- [ ] Add `fastlane ci` lane to Fastfile.
- [ ] Create `scripts/ci/focalpoint-ci.plist` template.
- [ ] Create `scripts/ci/run-build.sh` wrapper (sources .env.ci, runs fastlane).
- [ ] Create `scripts/ci/watch.sh` wrapper (calls focus-ci-watcher binary).
- [ ] Create `.env.ci.example` with all required env vars.
- [ ] Document first-time setup in `scripts/ci/README.md`.
- [ ] Test on Mac Mini (or dev Mac): `launchctl load`, trigger commit, verify Discord alert.
- [ ] Commit with two-part story: docs + implementation.

---

## Future: Forgejo Actions (Phase 2)

**File:** `deploy/forgejo-runner.yml`

When ready to scale beyond single machine:

1. Run Forgejo container: `docker run -d forgejo/forgejo:latest`
2. Configure Forgejo OAuth (GitHub-compatible settings).
3. Deploy Woodpecker runner: `docker run -d woodpecker/runner-docker`
4. Register runner for macOS targets.
5. Create `.forgejo-ci.yml` with fastlane lanes (GitHub Actions–compatible YAML).
6. Push `.forgejo-ci.yml` to repo; Woodpecker automatically triggers on commits.

This document will be updated with Forgejo-specific details when Phase 2 begins.

---

## Summary

| Aspect | Launchd MVP | Forgejo Phase 2 |
|--------|------------|-----------------|
| **Setup time** | <5 min | ~30 min |
| **Infrastructure** | None (native macOS) | Docker Forgejo + Woodpecker |
| **Machines** | 1 (single Mac Mini) | N (multi-runner) |
| **Scaling** | Manual (add more Macs) | Automated (register runners) |
| **Secrets** | 1Password CLI | GitHub / Forgejo secrets |
| **Cost** | $0 | $0 (self-hosted) |
| **UI Dashboard** | CLI logs | Woodpecker web UI |

**Next steps:** Implement launchd MVP immediately; validate with two weeks of autonomous CI runs; then architect Forgejo upgrade.
