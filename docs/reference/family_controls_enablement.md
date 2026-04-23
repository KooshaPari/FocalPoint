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

## Test plan (real iPhone — CI cannot exercise this)

FamilyControls does **not** load under the simulator or "Designed for iPad on
Mac". All verification happens on a signed build on a physical device.

1. `xcodebuild -project apps/ios/FocalPoint/FocalPoint.xcodeproj -scheme
   FocalPointApp -destination 'generic/platform=iOS' -configuration Debug
   build` — must succeed with new entitlement.
2. Install on a real iPhone via Xcode (signed with the updated profile).
3. Settings → Family Controls: tap **Request access** → system sheet appears
   → approve. Status flips to `approved`.
4. Open a focus block that has a non-empty `FamilyActivitySelection` (via
   the picker UI, once wired). Confirm blocked apps show the shield when
   launched during the block.
5. End the block (or call `retract()`). Confirm shields clear immediately
   and `DeviceActivityCenter` monitoring stops (no stale schedule in
   Settings → Screen Time).
6. Re-apply with `endsAt` in 2 minutes, background the app, wait. Shield
   must clear automatically at the scheduled end — proving the
   `DeviceActivitySchedule` is doing its job.

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
