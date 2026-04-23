#!/usr/bin/env bash
# Pair + install FocalPoint on a physical iPhone (kushphone17 preferred).
# 5-line-glue justification: orchestrates xcrun devicectl + xcodebuild for
# developer-signed installs; Rust alternative would be 150+ lines of Foundation/
# devicectl-IPC shimming for zero user-facing benefit.

set -euo pipefail
cd "$(dirname "$0")/.."

echo "==> Listing paired + connected devices..."
xcrun devicectl list devices

DEVICE_ID="${1:-}"
if [[ -z "$DEVICE_ID" ]]; then
  DEVICE_ID=$(xcrun devicectl list devices 2>/dev/null | awk '
    $4 == "connected" { print $3; exit }
  ')
fi

if [[ -z "$DEVICE_ID" ]]; then
  echo "No connected device found." >&2
  echo "Plug phone in via USB-C, unlock it, tap 'Trust This Computer' when prompted," >&2
  echo "then re-run. Pass a specific UDID as \$1 to override." >&2
  exit 1
fi

echo "==> Target device: $DEVICE_ID"
TEAM_ID="${FOCALPOINT_TEAM_ID:-}"
if [[ -z "$TEAM_ID" ]]; then
  echo "==> DEVELOPMENT_TEAM not set; signing will fall back to automatic." >&2
  echo "    Set FOCALPOINT_TEAM_ID=XXXXXXXXXX for explicit team." >&2
fi

echo "==> Building for device..."
if [[ -n "$TEAM_ID" ]]; then
  xcodebuild \
    -scheme FocalPointApp \
    -destination "platform=iOS,id=$DEVICE_ID" \
    -configuration Debug \
    CODE_SIGN_STYLE=Automatic \
    DEVELOPMENT_TEAM="$TEAM_ID" \
    PRODUCT_BUNDLE_IDENTIFIER="com.${TEAM_ID,,}.focalpoint" \
    build
else
  xcodebuild \
    -scheme FocalPointApp \
    -destination "platform=iOS,id=$DEVICE_ID" \
    -configuration Debug \
    CODE_SIGN_STYLE=Automatic \
    build
fi

APP_PATH=$(find ~/Library/Developer/Xcode/DerivedData -name "FocalPointApp.app" -path "*iphoneos*" -type d 2>/dev/null | head -1)
if [[ -z "$APP_PATH" ]]; then
  echo "App bundle not found. Build may have failed." >&2
  exit 2
fi

echo "==> Installing: $APP_PATH"
xcrun devicectl device install app --device "$DEVICE_ID" "$APP_PATH"

BUNDLE_ID=$(defaults read "$APP_PATH/Info" CFBundleIdentifier)
echo "==> Launching: $BUNDLE_ID"
xcrun devicectl device process launch --device "$DEVICE_ID" "$BUNDLE_ID"

echo "==> Done. Check phone."
