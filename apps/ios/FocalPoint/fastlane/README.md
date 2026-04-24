# fastlane — FocalPoint Release Automation

fastlane automates TestFlight uploads, code signing, and build orchestration for FocalPoint iOS app.

## Prerequisites

- **Xcode** 26.3+ with command-line tools
- **fastlane** gem (install via `gem install fastlane`)
- **xcodegen** (install via `brew install xcodegen`)
- **Apple Developer Account** with team ID `GCT2BN8WLL`
- **App Store Connect API Key** (JSON file from https://appstoreconnect.apple.com/access/integrations/api)

## Setup

1. **Copy environment template:**
   ```bash
   cp fastlane/.env.fastlane.example fastlane/.env.fastlane
   ```

2. **Edit `.env.fastlane` and populate:**
   - `FASTLANE_APPLE_ID` — your Apple ID (email)
   - `FASTLANE_TEAM_ID` — your App Store Connect team ID (`GCT2BN8WLL`)
   - `MATCH_GIT_URL` — Git repo URL where certificates/provisioning profiles are stored (GitHub private repo or S3)
   - `APP_STORE_CONNECT_API_KEY_PATH` — path to JSON API key file

3. **Source the environment:**
   ```bash
   source fastlane/.env.fastlane
   ```

## Lanes

### `fastlane smoke`
Build for Mac Designed-for-iPad (local dev loop). Regenerates Xcode project if missing.

```bash
fastlane smoke
```

**Use case:** Quick local builds during development. No signing or packaging required.

### `fastlane snapshot`
Run swift-snapshot-testing. Records new baselines on first run; compares thereafter.

```bash
fastlane snapshot
# Record baselines:
RECORD=true fastlane snapshot
```

**Use case:** Detect UI regressions before TestFlight. Baseline snapshots committed to repo.

### `fastlane beta`
Build for iOS device and upload to TestFlight. **Requires FamilyControls entitlement approval.**

```bash
FAMILY_CONTROLS_ENTITLEMENT_APPROVED=yes fastlane beta
```

**Prerequisites:**
- FamilyControls entitlement must be approved by Apple (see `fastlane/metadata/entitlement_request.md`)
- Code signing certificates and provisioning profiles must be in git (via match)
- App Store Connect API key must be accessible

**What it does:**
1. Regenerates Xcode project (`xcodegen`)
2. Builds Release binary for iOS device
3. Uploads IPA to TestFlight
4. Attaches release notes from `fastlane/metadata/release_notes_latest.txt`

**Entitlement Gate:** If `FAMILY_CONTROLS_ENTITLEMENT_APPROVED != "yes"`, fails loudly with instructions.

### `fastlane screenshots`
Generate App Store screenshots via snapshot testing. **Deferred to Phase 2.**

```bash
fastlane screenshots
```

**Current status:** Scaffolded; requires integration with `focus` CLI to seed fixture database.

## Environment Variables

All options (except `CI`) are read from `.env.fastlane`:

| Variable | Example | Required | Purpose |
|----------|---------|----------|---------|
| `FASTLANE_APPLE_ID` | your@apple.com | Yes | Apple Developer account |
| `FASTLANE_TEAM_ID` | GCT2BN8WLL | Yes | App Store Connect team |
| `MATCH_GIT_URL` | https://github.com/... | Yes (beta lane) | Code signing git repo |
| `APP_STORE_CONNECT_API_KEY_PATH` | ./AuthKey.json | Yes (beta lane) | API key JSON file path |
| `FAMILY_CONTROLS_ENTITLEMENT_APPROVED` | yes/no | Yes (beta lane) | Entitlement approval gate |
| `RECORD` | true/false | No | Record snapshot baselines (default: false) |
| `CI` | true/false | No | CI environment flag (auto-detected) |

## Code Signing (match)

Code signing credentials (certificates + provisioning profiles) are stored in a **private Git repository** and managed by fastlane `match`.

**Setup match:**
1. Create a private GitHub repo: `focalpoint-codesigning`
2. Store certificates and provisioning profiles there (match format)
3. Set `MATCH_GIT_URL` to the repo URL

**On CI:**
- match runs in read-only mode: `readonly true` in Matchfile
- Credentials must exist in Git before the CI job runs

## Troubleshooting

### "Xcode project not found"
Run `xcodegen` first:
```bash
xcodegen generate
```

### "fastlane command not found"
Install fastlane:
```bash
gem install fastlane
```

### "API key not found"
Download JSON API key from App Store Connect:
1. https://appstoreconnect.apple.com/access/integrations/api
2. Create new key (role: "App Manager")
3. Download JSON file
4. Set `APP_STORE_CONNECT_API_KEY_PATH` to file path

### "Code signing certificates not found"
Set up match:
```bash
fastlane match development
fastlane match appstore
```

Or ensure `MATCH_GIT_URL` points to a valid repository with certificates.

### "Entitlement application blocking upload"
The beta lane requires explicit Apple approval for FamilyControls. This is intentional and prevents accidental uploads without the entitlement. To unblock:

1. Submit entitlement request (see `fastlane/metadata/entitlement_request.md`)
2. Wait for Apple approval (1–4 weeks typical)
3. Set `FAMILY_CONTROLS_ENTITLEMENT_APPROVED=yes`

## Local Testing

**Test smoke build (no code signing):**
```bash
fastlane smoke
```

**Test beta build (full signing + upload, dry-run):**
```bash
fastlane beta --verbose
```

## CI Integration

For GitHub Actions, set secrets:
```
FASTLANE_APPLE_ID
FASTLANE_TEAM_ID
MATCH_GIT_URL
MATCH_GIT_PASSWORD (if repo is private, use GitHub token)
APP_STORE_CONNECT_API_KEY_JSON (paste API key JSON inline)
FAMILY_CONTROLS_ENTITLEMENT_APPROVED
```

Example workflow:
```yaml
- name: Upload to TestFlight
  run: |
    source fastlane/.env.fastlane
    fastlane beta
```

## References

- [fastlane docs](https://docs.fastlane.tools/)
- [fastlane match](https://docs.fastlane.tools/actions/match/)
- [App Store Connect API](https://developer.apple.com/app-store-connect/api/)
- [FamilyControls entitlement](fastlane/metadata/entitlement_request.md)
