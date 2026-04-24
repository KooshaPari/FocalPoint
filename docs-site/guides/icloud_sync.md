# iCloud Sync — Phase 1 Guide

FocalPoint v0.1 introduces **CloudKit-based multi-device sync** so your rules, wallet, and audit records stay in sync across iPhone, iPad, and macOS.

## What Syncs (Phase 1)

✅ **Wallet** — Credit balance, earned/spent totals, streaks, multiplier state  
✅ **Audit Records** — Tamper-evident append-only chain of all state mutations  
✅ **Rules** — Rule definitions, enabled/disabled status, trigger/condition/action payloads

❌ **NOT synced yet** (Phase 2+):
- Tasks, sub-tasks, and completions
- Connector connections and cached event data
- Templates and ritual schedules

## Enabling iCloud Sync

1. Open **Settings** → **Sync across devices**
2. Toggle **CloudKit Sync** to **ON**
3. A status message appears: "Checking iCloud status…"
4. Once iCloud is verified, you'll see "Ready to sync" or a reason why it's unavailable

## Syncing Manually

Tap the **Sync now** button in Settings to pull the latest changes from other devices and push local changes to iCloud.

A banner appears at the top of the Home tab during sync:
- ⏳ **Syncing...** — sync in progress
- ✅ **Synced** — last sync timestamp
- ⚠️ **N sync conflicts** — conflicts detected (see below)
- 🔴 **iCloud unavailable** — iCloud account is not signed in

## How Sync Works

### Conflict Resolution Strategies

Each record type uses a deterministic merge strategy when a conflict is detected:

| Record Type | Strategy | Details |
|-------------|----------|---------|
| **Wallet** | Monotonic-sum merge | Earned credits always increase; spent credits always increase. Streaks use max-count. Multiplier uses remote if newer. |
| **AuditRecord** | Append-only union | All audit records from all devices form a single tamper-evident chain. Duplicates are deduplicated by ID. |
| **Rule** | Last-Write-Wins (LWW) by `updated_at` | The device with the most recent update timestamp wins. |

### Device Signatures

Every record pushed to iCloud includes:
- **Device ID** — which device originated the record
- **Device signature** — Ed25519 signature over the payload (currently stubbed for testing; production uses Keychain keys)

If a signature verification fails, the record is logged as a tamper alert but does not block the sync.

### Sync Tokens

FocalPoint stores a **sync token** per record type in UserDefaults. This token is a CloudKit `CKServerChangeToken` that allows pulling only changes since the last sync, not the entire history.

- Tokens persist across app launches
- Syncing without a token re-fetches all records (slower, but safe)
- If a token is corrupted, the app falls back to a fresh fetch

## Troubleshooting

### "iCloud is unavailable"

**Cause:** You're not signed into iCloud with your Apple ID.

**Fix:** Open Settings (device Settings, not app) → [Your Name] → iCloud and sign in.

### "Could not determine iCloud status"

**Cause:** Apple's iCloud check service didn't respond.

**Fix:** Check your internet connection and try again. This is usually transient.

### Sync conflicts after enabling on a second device

**Expected behavior.** When you enable sync on a second device that already has local rules/wallet data, the two devices' versions may differ. FocalPoint applies the conflict resolution strategy (LWW for Rules, monotonic-sum for Wallet, union for Audit) to merge them automatically.

**You don't need to do anything** — the merge happens in the background and both devices will converge to the same state within a few seconds.

### Sync doesn't appear to be happening

1. Check that **Settings > Sync across devices > CloudKit Sync** is toggled **ON**.
2. Tap the **Sync now** button to force a manual sync.
3. Check that you have internet connectivity.
4. If the banner shows "iCloud unavailable," fix your iCloud sign-in (see above).

### I want to disable sync

1. Open **Settings > Sync across devices**
2. Toggle **CloudKit Sync** to **OFF**

Your local data is **not deleted** — syncing just stops. You can re-enable it anytime.

## How Your Data is Protected

- **End-to-end encrypted:** All records are stored in your iCloud private database (not shared with Apple or FocalPoint servers).
- **Device-signed:** Every record carries a device signature, so you can audit which device made each change.
- **Append-only audit chain:** All mutations are recorded in a tamper-evident hash chain; the chain cannot be altered without breaking the hash.
- **No personal data exposed:** Sensitive fields (tokens, emails, calendar URLs) are **never** synced. Only the logical state (rule IDs, credit amounts, audit record types) is synced.

## What Happens on Logout / Deletion

If you:
- Sign out of iCloud
- Delete the app
- Turn off iCloud Sync in Settings

Your **local data is not affected**. The remote CloudKit records remain in your private iCloud space until you manually delete them.

To completely remove your synced data from iCloud, you must:
1. Manually delete the `focalpoint-sync-v1` CloudKit zone (via Apple's CloudKit dashboard or programmatically)
2. OR delete the entire iCloud backup associated with this device

## Implementation Notes (for developers)

- **CloudKitSyncClient** (Swift): Wraps CKContainer and manages record push/pull
- **CloudKitPort** (Rust): Trait that Rust-side sync orchestration calls to invoke CloudKit operations
- **Record types in CloudKit:** `Wallet`, `AuditRecord`, `Rule` with payload stored as base64-encoded JSON
- **Sync zone:** `focalpoint-sync-v1` in the private database
- **Background sync:** BGTask subscription enabled for all three record types (subject to iOS background app refresh limits)

## Future Phases

- **Phase 2:** Tasks, Connectors, Templates
- **Phase 3:** Richer conflict UI (user chooses resolution per conflict)
- **Phase 4:** Cross-user sharing (sync wallet/rules with a family member or accountability partner)
