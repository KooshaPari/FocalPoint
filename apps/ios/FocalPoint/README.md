# FocalPoint iOS (SwiftPM scaffold)

SwiftPM-only scaffold for the FocalPoint iOS app. No `.xcodeproj` is checked in;
Xcode opens the package directly.

- Swift 5.9+, iOS 17+
- SwiftUI, AVFoundation, FamilyControls (entitlement-gated)
- UniFFI bindings to `crates/focus-ffi` land later — `FocalPointCore` is a placeholder today.

## Targets

| Target            | Kind        | Summary                                           |
|-------------------|-------------|---------------------------------------------------|
| `FocalPointApp`   | executable  | `@main` SwiftUI app; tab root (Home/Rules/Coachy/Settings) |
| `DesignSystem`    | library     | `Color.app.*`, `Color.coachy.*`, `AppTypography`  |
| `MascotUI`        | library     | `CoachyView`, `CoachyPose`, `CoachyEmotion`, `CoachyState` |
| `Enforcement`     | library     | `EnforcementDriver`, stub + FamilyControls shell  |
| `UnlockProof`     | library     | `QRScannerView` (AVFoundation)                    |
| `FocalPointCore`  | library     | Placeholder for UniFFI-generated bindings         |

## Open in Xcode

```bash
cd apps/ios/FocalPoint
open Package.swift
```

## Build (command line)

Package-level build (host platform, no iOS simulator):

```bash
swift build
swift test
```

Full iOS simulator build via Xcode:

```bash
xcodebuild \
  -scheme FocalPointApp \
  -destination 'platform=iOS Simulator,name=iPhone 16' \
  build
```

## Run on-device

- USB: plug in a device, select it as destination in Xcode, sign with your
  personal team. No special provisioning needed for `FocalPointApp` until
  FamilyControls is enabled.
- Tailscale pairing (for remote dev): pair the Mac + iPhone on the same
  tailnet, then use Xcode's "Connect via Network" option.
- Once `com.apple.developer.family-controls` is provisioned, switch signing to
  the matching profile before running on-device.

## Info.plist keys (required in the eventual Xcode project)

- `NSCameraUsageDescription` — QR scan unlock proof.
- `NSFaceIDUsageDescription` — later, for panic unlock.
- Capabilities: Family Controls (blocker), Background Modes (processing), App
  Groups (shared container with DeviceActivityMonitor extension).

## Notes

- Mascot is a Canvas/SF Symbols placeholder; Spline scene wiring is deferred.
- `FamilyControlsEnforcementDriver` is entitlement-gated via
  `#if canImport(FamilyControls) && !targetEnvironment(simulator)`; simulator
  builds and CI use `StubEnforcementDriver`.
- Do not add Rust/domain logic in Swift — it belongs in `crates/`.
