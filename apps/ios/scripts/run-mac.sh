#!/usr/bin/env bash
# run-mac.sh — Build and launch FocalPointApp on Apple Silicon Mac as a
# "Designed for iPad" app, without running the iOS Simulator.
#
# Scripting-policy justification (per repos/docs/governance/scripting_policy.md):
# This script is platform glue around `xcodebuild`, `lsregister`, `codesign`,
# `open`, `osascript`, and `log stream` — all of which are macOS-only tools
# invoked via their CLIs. A Rust/Go rewrite would just shell out to the same
# binaries and add build overhead. Kept as bash because every line is a
# tool invocation or argument assembly; no business logic.
#
# Mechanism discovered (2026-04-23):
#   Apple Silicon Macs run "Designed for iPad" apps through an LS-recognized
#   WRAPPER BUNDLE structure. A normal .app built for `iphoneos` will NOT
#   launch directly ("incorrect executable format"). The App Store installs
#   iPad apps on Mac as:
#     /Applications/<Name>.app/
#       Wrapper/<Name>App.app     # the real iOS bundle (platform=2, arm64)
#       Wrapper/BundleMetadata.plist
#       Wrapper/iTunesMetadata.plist
#       WrappedBundle -> Wrapper/<Name>App.app  # symlink
#   LaunchServices recognizes this shell, routes the executable through the
#   iPadOS-on-Mac runtime, and the app launches in a Mac window (no
#   Simulator). We replicate the wrapper layout under a user-writable path,
#   register with lsregister, and `open` the outer bundle.
#
# Usage: apps/ios/scripts/run-mac.sh [--rebuild] [--no-log]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)/FocalPoint"
SCHEME="FocalPointApp"
CONFIG="Debug"
BUNDLE_ID="com.koosha.focalpoint"
EXECUTABLE="FocalPointApp"
INSTALL_ROOT="${FOCALPOINT_MAC_INSTALL_ROOT:-/Users/Shared}"
WRAPPER_APP="$INSTALL_ROOT/FocalPoint.app"

REBUILD=0
STREAM_LOG=1
for arg in "$@"; do
  case "$arg" in
    --rebuild) REBUILD=1 ;;
    --no-log)  STREAM_LOG=0 ;;
    -h|--help)
      sed -n '1,30p' "$0"; exit 0 ;;
  esac
done

LSREGISTER=/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister

cd "$APP_DIR"

# 1) Regenerate Xcode project from project.yml if xcodegen available.
if command -v xcodegen >/dev/null 2>&1; then
  (xcodegen generate >/dev/null 2>&1) || true
fi

# 2) Build for Designed-for-iPad-on-Mac.
if [[ $REBUILD -eq 1 ]] || [[ ! -d "$(xcodebuild -showBuildSettings -scheme "$SCHEME" -configuration "$CONFIG" 2>/dev/null | awk '/ BUILT_PRODUCTS_DIR /{print $3; exit}')" ]]; then
  echo ">>> xcodebuild ($SCHEME, Designed for iPad on Mac)"
  xcodebuild \
    -scheme "$SCHEME" \
    -configuration "$CONFIG" \
    -destination 'platform=macOS,arch=arm64,variant=Designed for iPad' \
    build | tail -5
fi

BUILT_DIR="$(xcodebuild -showBuildSettings -scheme "$SCHEME" -configuration "$CONFIG" 2>/dev/null | awk '/ BUILT_PRODUCTS_DIR /{print $3; exit}')"
# Designed-for-iPad product lives under Debug-iphoneos/.
APP_SRC="${BUILT_DIR%/Debug*}/Debug-iphoneos/${EXECUTABLE}.app"
if [[ ! -d "$APP_SRC" ]]; then
  echo "ERROR: built .app not found at $APP_SRC" >&2
  exit 1
fi

# 3) Stage wrapper bundle under a user-writable prefix.
echo ">>> staging wrapper bundle at $WRAPPER_APP"
rm -rf "$WRAPPER_APP"
mkdir -p "$WRAPPER_APP/Wrapper"
cp -R "$APP_SRC" "$WRAPPER_APP/Wrapper/${EXECUTABLE}.app"
ln -s "Wrapper/${EXECUTABLE}.app" "$WRAPPER_APP/WrappedBundle"

# 4) Register with LaunchServices.
"$LSREGISTER" -f "$WRAPPER_APP" || true

# 5) Terminate any previous instance.
pkill -f "Wrapper/${EXECUTABLE}.app/${EXECUTABLE}" 2>/dev/null || true

# 6) Launch.
echo ">>> launching"
open "$WRAPPER_APP"

# 7) Wait for PID.
PID=""
for _ in 1 2 3 4 5 6 7 8 9 10; do
  PID="$(pgrep -f "Wrapper/${EXECUTABLE}.app/${EXECUTABLE}" | head -1 || true)"
  [[ -n "$PID" ]] && break
  sleep 0.3
done
if [[ -z "$PID" ]]; then
  echo "ERROR: FocalPointApp did not start" >&2
  exit 2
fi
echo ">>> running as PID $PID"

# 8) Optional log stream (Ctrl-C to exit).
if [[ $STREAM_LOG -eq 1 ]]; then
  echo ">>> streaming log (Ctrl-C to detach; app keeps running)"
  exec log stream --style compact \
    --predicate "process == \"${EXECUTABLE}\"" \
    --level debug
fi
