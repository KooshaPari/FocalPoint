# FamilyControls entitlement enablement

FocalPoint ships with `FamilyControlsEnforcementDriver` compiled in log-only
mode. The real enforcement branch is behind the Swift Active Compilation
Condition `FOCALPOINT_HAS_FAMILYCONTROLS`. This doc is the runbook for
flipping the switch once Apple grants `com.apple.developer.family-controls`.

## Apple prerequisites

1. Request the entitlement on the Apple Developer portal: account
   `GCT2BN8WLL` → **Services** → **Family Controls (Distribution)** →
   submit the usage description that matches the App Store review copy.
2. Wait (weeks). The entitlement is human-reviewed and there is no
   accelerated path. Do not fake it — provisioning will still fail to sign.
3. On approval, regenerate the provisioning profile for bundle
   `com.koosha.focalpoint` with **Family Controls** checked, and download
   the new profile into Xcode / Transporter.

## Repo changes (all local, reviewable in one PR)

### 1. Entitlements file

Create `apps/ios/FocalPoint/Sources/FocalPointApp/FocalPointApp.entitlements`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>com.apple.developer.family-controls</key>
  <true/>
</dict>
</plist>
```

### 2. `project.yml` wiring

Under the `FocalPointApp` target, add:

```yaml
    entitlements:
      path: Sources/FocalPointApp/FocalPointApp.entitlements
    settings:
      base:
        CODE_SIGN_ENTITLEMENTS: Sources/FocalPointApp/FocalPointApp.entitlements
        SWIFT_ACTIVE_COMPILATION_CONDITIONS: "$(inherited) FOCALPOINT_HAS_FAMILYCONTROLS"
```

And under the `Enforcement` target (so the flagged branch compiles):

```yaml
    settings:
      base:
        SWIFT_ACTIVE_COMPILATION_CONDITIONS: "$(inherited) FOCALPOINT_HAS_FAMILYCONTROLS"
```

Regenerate the Xcode project:

```bash
cd apps/ios/FocalPoint && xcodegen generate
```

### 3. No other code changes

`FamilyControlsEnforcementDriver` and `FamilyControlsStatusView` are already
written against the real APIs. Flipping the flag is the whole code change.

## Detailed test plan (real iPhone — CI cannot exercise this)

FamilyControls does **not** load under the simulator or "Designed for iPad on
Mac". All verification happens on a signed build on a physical device.

### Phase 1: Build & Deployment

1. **Build on real device with entitlement:**
   ```bash
   xcodebuild -project apps/ios/FocalPoint/FocalPoint.xcodeproj \
     -scheme FocalPointApp \
     -destination 'generic/platform=iOS' \
     -configuration Debug \
     build
   ```
   Must succeed with new entitlement signed in provisioning profile.

2. **Install on real iPhone via Xcode** (signed with updated profile).

### Phase 2: Authorization Flow

3. **Open Settings → Screen Time → Family Controls**
   - Tap **Request Access**
   - System sheet appears: *"FocalPoint" wants to manage your device.*
   - User approves (grants permission)
   - Status should flip to `Approved` (checkmark visible)

### Phase 3: Shield Engagement

4. **Create or activate a focus block:**
   - Open FocalPoint app
   - Create a new rule or activate existing one
   - Use **FamilyActivityPicker** to select apps (e.g., Instagram, Discord)
   - Set schedule: now → now + 10 minutes (for quick test)
   - Save rule
   
5. **Verify shield engages:**
   - While rule is active, launch one of the blocked apps
   - **Expected:** Full-screen shield appears with custom UI (FocalPoint branding)
   - Cannot dismiss by swiping, tapping, or pressing Home
   - Shield persists until rule ends or app is unlocked via Enforcement.retract()

### Phase 4: Shield Retraction

6. **Test automatic shield teardown:**
   - Let the 10-minute window elapse (or manually stop rule in app)
   - **Expected:** Shield disappears automatically
   - App becomes usable again immediately
   - `DeviceActivityCenter` monitoring stops (no stale schedule in Settings → Screen Time)

7. **Test manual shield retraction:**
   - Create another 5-minute block
   - During the block, tap app's "Stop Blocking" button (if available)
   - Call `FamilyControlsEnforcementDriver.retract()` (Rust core initiates)
   - **Expected:** Shield clears immediately
   - No delay or residual blocking

### Phase 5: Background Enforcement

8. **Test background enforcement (process-agnostic):**
   - Create a new rule: now → now + 2 minutes
   - Immediately **background FocalPoint app** (swipe up or lock screen)
   - **Expected:** Shield remains active in background apps
   - System-level monitoring continues (no app process needed)
   - After 2 minutes, shield clears automatically

### Phase 6: Logging & Diagnostics

9. **Check Console logs:**
   ```bash
   # In Xcode Debugger or Console.app
   log stream --predicate 'subsystem == "app.focalpoint.enforcement"' --level debug
   ```
   Expected log lines:
   ```
   FamilyControls apply blocked=3 appTokens=3 endsAt=2026-04-23T10:00:00Z
   intervalDidStart for activity: app.focalpoint.shield
   intervalDidEnd for activity: app.focalpoint.shield
   FamilyControls retract cleared shield + stopped monitoring
   ```
   No error messages about `DeviceActivity startMonitoring failed`.

10. **Verify audit trail:**
    - Navigate to app's Activity Log or Admin view
    - Each shield engagement/disengagement is recorded
    - Timestamp, actor (system/user), action (engage/retract), rule ID present
    - No missing entries or out-of-order records

## Off-Device Testing (Flag OFF, No Entitlement)

Always test off-device behavior to ensure safe fallback:

### Simulator (iOS Simulator)

```bash
xcodebuild -project apps/ios/FocalPoint/FocalPoint.xcodeproj \
  -scheme FocalPointApp \
  -destination 'platform=iOS Simulator,name=iPhone 15' \
  -configuration Debug \
  build
```

**Expected:**
- App builds without errors
- App runs in simulator
- No FamilyControls code executes (framework not available on simulator)
- `FamilyControlsEnforcementDriver.apply(policy:)` logs "FamilyControls unavailable; no-op apply"
- No shields engage (correct fallback)

### Designed for iPad on Mac

```bash
xcodebuild -project apps/ios/FocalPoint/FocalPoint.xcodeproj \
  -scheme FocalPointApp \
  -destination 'platform=macOS,arch=arm64,variant=Designed for iPad' \
  -configuration Debug \
  build
```

**Expected:**
- App builds and runs in "Designed for iPad" mode
- All `#if FOCALPOINT_HAS_FAMILYCONTROLS` blocks are dead code (compiled away)
- Enforcement is no-op (log-only)
- No shields appear (intended behavior)

## Success Criteria

- ✅ Build succeeds with entitlement on real device
- ✅ FamilyControls authorization prompt appears and user can approve
- ✅ FamilyActivityPicker launches when creating/editing rules
- ✅ Shield engages (full-screen, non-dismissible) when rule is active
- ✅ Shield lifts when rule time expires or `retract()` is called
- ✅ DeviceActivityMonitor extension logs `intervalDidStart` and `intervalDidEnd`
- ✅ Audit trail records all state changes (timestamp, actor, action, rule ID)
- ✅ App does NOT crash or hang during shield transitions
- ✅ Background enforcement works (shield persists when app is backgrounded)
- ✅ Off-device builds (simulator, Mac) work without FamilyControls (flag OFF)

## Rollback

If a release ships with the flag on but the entitlement is revoked or the
profile regresses:

1. Remove `FOCALPOINT_HAS_FAMILYCONTROLS` from
   `SWIFT_ACTIVE_COMPILATION_CONDITIONS` on both targets.
2. Remove `CODE_SIGN_ENTITLEMENTS` from `FocalPointApp`.
3. `xcodegen generate` + new build.
4. Driver falls back to log-only — safe, no crashes, no silent partial
   enforcement.

## Why no CI coverage

- The FamilyControls framework is not present on macOS/simulator SDK paths.
- Even on a device, the APIs refuse to do anything without a
  correctly-signed entitlement.
- Mocking at the Apple API layer would invert the whole point of the flag.

The flagged branch is reviewed by API shape (see `Enforcement.swift`
comments) and verified by the device test plan above on every entitlement
bring-up or Xcode / iOS major version bump.
