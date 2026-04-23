#!/usr/bin/env bash
# Build + install FocalPoint via the CLASSIC libimobiledevice path
# (ideviceinstaller), bypassing xcrun devicectl / CoreDevice. Works with the
# usbmuxd pair we already have, and the install attempt itself will surface
# iOS's Developer Mode toggle on the phone (even before first DM activation).
#
# Justification for shell: ~30-line glue wrapping xcodebuild + ditto + zip +
# ideviceinstaller. A Rust alternative would wrap 5 CLI tools for zero benefit.
#
# Requires:
#   - FOCALPOINT_TEAM_ID (Apple Developer Team ID, free personal teams OK)
#   - Physical iPhone plugged in + trusted via libimobiledevice (idevicepair
#     validate → SUCCESS).
#
# Usage:
#   FOCALPOINT_TEAM_ID=ABCDEF1234 apps/ios/scripts/build-dev-ipa.sh
#   apps/ios/scripts/build-dev-ipa.sh --dry-run    # build only, no install

set -euo pipefail
cd "$(dirname "$0")/../FocalPoint"

SCHEME="FocalPointApp"
CONFIG="Debug"
DRYRUN="${1:-}"

TEAM_ID="${FOCALPOINT_TEAM_ID:-}"
if [[ -z "$TEAM_ID" ]]; then
  echo "ERROR: FOCALPOINT_TEAM_ID not set." >&2
  echo "Find yours: Xcode → Settings → Accounts → your Apple ID → Team." >&2
  echo "Or from keychain: security find-identity -p codesigning -v" >&2
  exit 1
fi

TEAM_LC=$(printf '%s' "$TEAM_ID" | tr '[:upper:]' '[:lower:]')
BUNDLE_ID="com.${TEAM_LC}.focalpoint"
echo "==> Team: $TEAM_ID / Bundle: $BUNDLE_ID"

UDID=$(idevice_id -l 2>/dev/null | head -1 || true)
if [[ -z "$UDID" ]]; then
  echo "ERROR: no device from idevice_id. Plug phone in, unlock, trust." >&2
  exit 2
fi
echo "==> Device: $UDID ($(ideviceinfo -k DeviceName -u $UDID))"

echo "==> xcodebuild build (iphoneos) for device $UDID..."
DERIVED="$(mktemp -d)"
# Xcode 26.0 has a regression where -destination "generic/platform=iOS" fails
# to resolve even when iPhoneOS.platform is installed. Targeting the device
# id directly bypasses the placeholder resolver.
xcodebuild \
  -scheme "$SCHEME" \
  -destination "id=$UDID" \
  -configuration "$CONFIG" \
  -derivedDataPath "$DERIVED" \
  -sdk iphoneos \
  CODE_SIGN_STYLE=Automatic \
  DEVELOPMENT_TEAM="$TEAM_ID" \
  PRODUCT_BUNDLE_IDENTIFIER="$BUNDLE_ID" \
  build

APP_PATH=$(find "$DERIVED" -name "$SCHEME.app" -path "*iphoneos*" -type d 2>/dev/null | head -1)
if [[ -z "$APP_PATH" ]]; then
  echo "ERROR: built .app not found under $DERIVED" >&2
  exit 3
fi
echo "==> App: $APP_PATH"

IPA_DIR="$(mktemp -d)"
mkdir -p "$IPA_DIR/Payload"
ditto "$APP_PATH" "$IPA_DIR/Payload/$SCHEME.app"
IPA="$IPA_DIR/FocalPoint-dev.ipa"
( cd "$IPA_DIR" && zip -qr "$IPA" Payload )
echo "==> IPA: $IPA ($(du -h "$IPA" | cut -f1))"

if [[ "$DRYRUN" == "--dry-run" ]]; then
  echo "==> Dry-run; skipping install."
  exit 0
fi

echo "==> Installing via ideviceinstaller (classic path)..."
ideviceinstaller -u "$UDID" -i "$IPA"

echo "==> Install attempt complete."
echo "    If launch fails with 'Untrusted Developer' or 'Developer Mode required',"
echo "    that prompt surfaces the Developer Mode toggle on your phone."
echo "    Toggle it on (Settings → Privacy & Security → Developer Mode → ON → restart)."
echo "    Then launch FocalPoint from the Home Screen."
