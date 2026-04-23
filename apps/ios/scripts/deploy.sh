#!/usr/bin/env bash
# FocalPoint iOS deploy — auto-targets the first available physical iPhone
# (preferring name match kushphone*); falls back to iPhone 16 Pro Max simulator.
# 5-line shell glue is justified here: wraps xcodebuild + xcrun devicectl into
# one command. A Rust replacement would be a 200-line xcrun subprocess shim.

set -euo pipefail
cd "$(dirname "$0")/.."

SCHEME="FocalPointApp"
PROJECT_FLAG="-scheme $SCHEME"

# Find a physical device: prefer kushphone* name; else first available.
DEVICE_ID=$(xcrun xctrace list devices 2>&1 | awk '
  /^== Devices$/{dev=1; next}
  /^== Devices Offline$/{dev=0; next}
  /^== Simulators$/{dev=0; next}
  dev && /^(kushphone|iPhone)/ && !/Laptop/ { print; exit }
' | sed -E 's/.*\(([0-9A-Fa-f-]+)\).*/\1/' || true)

if [[ -n "$DEVICE_ID" && "$DEVICE_ID" != *"Offline"* ]]; then
  echo "==> Physical device detected: $DEVICE_ID"
  DEST="platform=iOS,id=$DEVICE_ID"
else
  echo "==> No physical iPhone online; falling back to iPhone 17 Pro Max simulator."
  SIM_ID=$(xcrun simctl list devices available | awk '/iPhone 17 Pro Max/ {gsub(/[()]/,""); print $NF; exit}')
  if [[ -z "${SIM_ID:-}" ]]; then
    # Fall back further — any iPhone Pro Max simulator
    SIM_ID=$(xcrun simctl list devices available | awk '/iPhone 16 Pro Max/ {gsub(/[()]/,""); print $NF; exit}')
  fi
  if [[ -z "${SIM_ID:-}" ]]; then
    echo "No iPhone simulator available; aborting." >&2
    exit 1
  fi
  xcrun simctl boot "$SIM_ID" 2>/dev/null || true
  DEST="platform=iOS Simulator,id=$SIM_ID"
fi

echo "==> Destination: $DEST"
xcodebuild -scheme "$SCHEME" -destination "$DEST" build

# After build, attempt install+launch if simulator.
if [[ "$DEST" == *"Simulator"* ]]; then
  APP_PATH=$(find ~/Library/Developer/Xcode/DerivedData -name "FocalPointApp.app" -path "*iphonesimulator*" -type d 2>/dev/null | head -1)
  if [[ -n "$APP_PATH" ]]; then
    echo "==> Installing to simulator $SIM_ID from $APP_PATH"
    xcrun simctl install "$SIM_ID" "$APP_PATH"
    xcrun simctl launch --console-pty "$SIM_ID" "com.kooshapari.focalpoint" || true
    open -a Simulator
  else
    echo "App bundle not found in DerivedData; build succeeded but install skipped."
  fi
else
  echo "==> Physical device install requires provisioning profile + Team ID."
  echo "    See apps/ios/FocalPoint/README.md § Deploy to device."
fi
