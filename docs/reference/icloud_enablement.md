# iCloud CloudKit Entitlement Setup

**Status:** Declared (scaffolded, not yet implemented)  
**Updated:** April 2026

## Overview

FocalPoint v0.1 scaffolds multi-device sync via Apple CloudKit. This document explains the entitlement setup, provisioning requirements, and expected build behavior.

## Entitlements Declared

In `apps/ios/FocalPoint/project.yml`, CloudKit capabilities are declared:

```yaml
capabilities:
  - identifiers:
      - com.apple.developer.icloud-services
      - com.apple.developer.icloud-container-identifiers
    entitlements:
      com.apple.developer.icloud-services:
        - CloudKit
      com.apple.developer.icloud-container-identifiers:
        - iCloud.com.koosha.focalpoint
```

This generates the following entitlements in the app's `.entitlements` file:

```xml
<key>com.apple.developer.icloud-services</key>
<array>
  <string>CloudKit</string>
</array>
<key>com.apple.developer.icloud-container-identifiers</key>
<array>
  <string>iCloud.com.koosha.focalpoint</string>
</array>
```

## Provisioning Profile Requirement

**Critical:** CloudKit entitlements require a valid provisioning profile that explicitly includes:
1. Team ID: `GCT2BN8WLL` (configured in `project.yml`)
2. CloudKit capability enabled in the provisioning profile
3. iCloud container identifier: `iCloud.com.koosha.focalpoint`

### How to Create/Update the Provisioning Profile

1. **In Xcode (automatic):**
   - Open `FocalPoint.xcodeproj`.
   - Select the `FocalPointApp` target.
   - Go to **Signing & Capabilities**.
   - Click **+ Capability** → Search for "CloudKit".
   - Xcode will automatically fetch/regenerate the provisioning profile.

2. **In Apple Developer Portal (manual):**
   - Go to [developer.apple.com](https://developer.apple.com).
   - Sign in with the Apple Developer account.
   - Navigate to **Certificates, Identifiers & Profiles** → **Identifiers**.
   - Find or create the App ID for `com.koosha.focalpoint`.
   - Ensure CloudKit is enabled under **Capabilities**.
   - Re-download the development provisioning profile and import it into Xcode.

## Build Behavior

### Expected Outcomes

| Scenario | Build Result | Notes |
|----------|--------------|-------|
| Provisioning profile includes CloudKit | ✅ Success | Device & simulator builds work. |
| Provisioning profile missing CloudKit | ❌ Fails with code signing error | Expected. Re-create provisioning profile in Xcode or Apple Developer Portal. |
| No provisioning profile | ❌ Fails during code signing | Run `xcodebuild` with auto-provisioning, or manually import profile. |
| Simulator build | ✅ Success (no provisioning needed) | Simulator doesn't require provisioning; builds work. |
| Mac (Catalyst) build | ✅ Success | Mac doesn't require CloudKit entitlements as of macOS 12+. |

### If Device Build Fails

```bash
# Error: Code signing identity or provisioning profile not found.
# Reason: Provisioning profile missing CloudKit entitlement.

# Solution 1: Let Xcode auto-generate profile
xcodebuild -scheme FocalPointApp -destination 'generic/platform=iOS' -allowProvisioningUpdates build

# Solution 2: Manually regenerate in Apple Developer Portal
# 1. Go to Certificates, Identifiers & Profiles
# 2. Select the FocalPointApp App ID
# 3. Edit → Enable CloudKit
# 4. Download the new provisioning profile
# 5. Import into Xcode (Xcode → Settings → Accounts → Team → Download Manual Profiles)
```

## Runtime Behavior

Once provisioned, the app can:

1. **Check iCloud Status:** `CKContainer.default().accountStatus()`
   - Returns `.available` if user is signed into iCloud.
   - Returns `.noAccount` if not signed in.

2. **Access Private Database:** `CKContainer.default().privateCloudDatabase`
   - Create/update/delete `CKRecord` objects.
   - All records auto-encrypted with user's iCloud password.

3. **Record Signing:** Device signs records with local Ed25519 key (Keychain-stored).
   - `device_signature = Ed25519Sign(canonical_json(record), device_private_key)`
   - Verification on pull prevents tampering.

## Privacy & Security

- **E2E Encryption:** All sync data in CloudKit's private database is encrypted.
- **User Control:** User can disable sync in Settings or revoke iCloud access in System Settings.
- **Connector Tokens:** Never synced; remain in device Keychain.
- **Audit Chain:** Never synced; remains device-local.

## Troubleshooting

### Q: Can I build without provisioning?
**A:** Simulator builds work without provisioning. Device builds require a provisioning profile.

### Q: What if I don't have an Apple Developer account?
**A:** Device builds will fail. Use simulator for testing. To release to App Store, enroll in Apple Developer Program ($99/year).

### Q: Can I use CloudKit on non-Apple platforms?
**A:** No. CloudKit is Apple-only. v0.1 is iOS/iPadOS/macOS. Web support (v2+) would require a CRDT-based server sync.

### Q: What happens if iCloud is disabled on the device?
**A:** The app checks `accountStatus()` and shows "iCloud unavailable" in Settings. User can still use the app offline; sync is simply unavailable.

---

## Next Steps

1. **Device Testing:** Once provisioning profile is created, test on a physical device.
2. **Real Push/Pull:** Implement `CloudKitSyncClient.push()` and `.pull()` (currently stubs).
3. **User Testing:** Enable CloudKit sync in Settings (currently default OFF).
4. **v1 Release:** CloudKit sync is opt-in for v0.1; enable by default in v0.2 after stability testing.

---

**References:**
- [CloudKit Documentation](https://developer.apple.com/cloudkit/)
- [Xcode Signing Guide](https://developer.apple.com/support/code-signing/)
- [iCloud Container Identifiers](https://developer.apple.com/documentation/cloudkit/ckcontainer)
