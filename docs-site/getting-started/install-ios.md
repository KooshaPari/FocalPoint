# Install on iOS

> **Status:** pre-alpha. The iOS app builds from source; TestFlight distribution starts once the FamilyControls entitlement is approved.

## Prerequisites

- macOS with **Xcode 15+**.
- **Apple Developer account** (free tier works for personal testing; paid required for FamilyControls entitlement).
- **iPhone or iPad** on iOS 16+ paired over USB or Wi-Fi.
- `libimobiledevice` for device install: `brew install libimobiledevice ideviceinstaller`.

## 1. Request the FamilyControls entitlement

FocalPoint's enforcement loop requires `com.apple.developer.family-controls`. This is a gated entitlement — Apple reviews each application (typically 1–4 weeks).

1. Open the [Apple Developer Contact form](https://developer.apple.com/contact/request/family-controls-distribution/).
2. Describe FocalPoint honestly: "Connector-first screen-time rules platform for self-regulating adults. Enforces user-authored rules driven by external events (Canvas, calendars, health)."
3. Wait. Start authoring rules locally (`task test`) while you wait.

## 2. Build the app

```bash
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint
task build-ios-sim       # simulator
task build-ios-device    # physical device (unsigned)
```

For signed builds, open `apps/ios/FocalPoint.xcodeproj` in Xcode, set your team id, enable the FamilyControls capability, and archive.

## 3. Pair and install

```bash
task pair-and-install
```

This runs `idevice_id -l` to find your device, then `ideviceinstaller -i <built-ipa>`. If pairing fails, plug the device into your Mac, tap "Trust" on the device prompt, and retry.

## 4. Grant FamilyControls

On first launch the app requests Family Controls / Screen Time permission. This is a **one-time system prompt**; denying it means the enforcement loop cannot run. You can revisit in Settings → Screen Time → FocalPoint.

## 5. Connect Canvas

1. Open FocalPoint → Connectors → Canvas.
2. Tap "Authenticate". An in-app browser starts the OAuth flow with your school's Canvas instance.
3. Grant the `url:GET|/api/v1/courses`, `url:GET|/api/v1/assignments` scopes (the app requests only these).
4. Wait for the first sync to complete (≤30 seconds for a typical course load).

## 6. Apply your first rule template

Settings → Rules → Add from template → "Assignment-driven focus".

That rule reads Canvas assignment due dates and locks a list of apps (default: Instagram, TikTok, X) from 4 hours before due until submission is detected.

See [First rule walkthrough](/getting-started/first-rule) for authoring a rule from scratch.

## Troubleshooting

| Symptom | Likely cause | Fix |
|--------|-------------|-----|
| "FamilyControls unavailable" banner | Entitlement not granted, or iOS Screen Time disabled | Settings → Screen Time → Turn On |
| Canvas stays "syncing" forever | School-specific Canvas install rejects generic OAuth client | Open an issue with the Canvas base URL |
| `idevice_id -l` prints nothing | Device not trusted or cable data-disabled | Replug, tap Trust on device |
| App crashes on rule evaluation | Rule DSL parse error masked by a stale build | Re-run `task build-ios-device`, check logs via Console.app |

## Next

- [Write your first rule](/getting-started/first-rule)
- [Explore connectors](/connectors/)
- [How the rules engine works](/rules/)
