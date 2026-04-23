# Mac Launch Status — Designed-for-iPad on Apple Silicon

**Status:** WORKING as of 2026-04-23. FocalPointApp launches on Apple Silicon Macs from the command line with no iOS Simulator.

## TL;DR

Run:

```bash
apps/ios/scripts/run-mac.sh            # build + launch + stream logs
apps/ios/scripts/run-mac.sh --no-log   # launch without attaching to log stream
apps/ios/scripts/run-mac.sh --rebuild  # force xcodebuild clean rebuild
```

The app window appears as a regular Mac window (titlebar + traffic lights, iPad canvas inside). Memory footprint is ~1/10th of the iOS Simulator.

## Why the naive approach fails

`xcodebuild -destination 'platform=macOS,arch=arm64,variant=Designed for iPad'` produces a valid arm64 Mach-O under `Debug-iphoneos/FocalPointApp.app`, signed with a valid Apple Development identity. The binary carries `LC_BUILD_VERSION platform=2` (iOS device). Double-clicking it (or `open FocalPointApp.app`) returns:

> The application cannot be opened because it has an incorrect executable format.

LaunchServices refuses to route a bare iOS `.app` to the iPadOS-on-Mac runtime. It looks for a Mac-style shell around the iOS bundle.

## The mechanism Apple actually uses

Apps shipped to Apple Silicon via the Mac App Store (e.g. `Earth Editor.app` used as reference) are installed with a **wrapper bundle**:

```
/Applications/<Name>.app/
    Wrapper/<exec>.app/           # the real iOS bundle (arm64, platform=2)
    Wrapper/BundleMetadata.plist  # optional, App Store metadata
    Wrapper/iTunesMetadata.plist  # optional, App Store metadata
    WrappedBundle -> Wrapper/<exec>.app   # symlink at outer bundle root
```

The outer `.app` has NO `Contents/`, NO `Info.plist`, NO `MacOS/` binary — it is just a directory + symlink. LaunchServices recognizes the `Wrapper/` + `WrappedBundle` symlink pair as the "Designed for iPad on Mac" layout and launches the inner bundle through the iOSSupport runtime.

## What works

1. Build with `-destination 'platform=macOS,arch=arm64,variant=Designed for iPad'` (Debug-iphoneos product).
2. Stage the wrapper layout at a user-writable prefix (default `/Users/Shared/FocalPoint.app`; override with `FOCALPOINT_MAC_INSTALL_ROOT`). `/Applications/` works too but needs sudo.
3. Register with `lsregister -f <wrapper.app>`.
4. `open <wrapper.app>` — LaunchServices spawns the process inside `/private/var/folders/.../X/<uuid>/d/Wrapper/FocalPointApp.app/FocalPointApp` (the iOS-on-Mac sandbox).
5. `log stream --predicate 'process == "FocalPointApp"'` for runtime output.

## What does NOT work (dead ends tried)

- `open FocalPointApp.app` on the raw DerivedData product: "incorrect executable format".
- `open -b com.koosha.focalpoint` after `lsregister -f` on the raw bundle: `LSCopyApplicationURLsForBundleIdentifier() failed`.
- `xcrun simctl` — that is the iOS Simulator path, which is what we are avoiding.
- `xcrun devicectl device install app` — targets connected iOS devices, not the Mac host.
- `xcodebuild ... run` — not a valid xcodebuild action for this destination.

## Signing / entitlements

The existing code signing from `xcodebuild` (Apple Development cert, team `GCT2BN8WLL`, entitlements `application-identifier`, `get-task-allow`) is sufficient. No `com.apple.developer.iphone-on-mac` entitlement was required. No re-signing needed.

## Verification

- `pgrep -f "Wrapper/FocalPointApp.app/FocalPointApp"` returns a PID.
- `osascript -e 'tell application "System Events" to get name of every process whose visible is true'` includes `FocalPointApp`.
- Screenshot: `/tmp/focalpoint-launched.png` (1.5 MB, captured during end-to-end verification of `run-mac.sh`).

## Caveats

- The `Debug-iphoneos` product is still nominally an iOS build; hot-reload / preview workflows that expect a Mac target will not see it. Use Xcode's GUI Run button for inner-loop iteration when the Simulator cost is acceptable; use this script when running headless or when memory is tight.
- `lsregister` caches can go stale. If the app stops launching after a long period, re-run the script (it re-registers every invocation) or `lsregister -kill -r -domain local -domain system -domain user`.
- If you later add `/Applications/FocalPoint.app` as a root-owned wrapper, delete it before running this script or `FOCALPOINT_MAC_INSTALL_ROOT=/Applications` with sudo.
