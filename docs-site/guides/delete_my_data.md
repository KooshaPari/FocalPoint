# Delete All My Data (Right-to-Erasure)

FocalPoint respects your right to be forgotten. This guide walks you through permanently deleting all your personal data from the app.

## What Gets Deleted

When you choose to delete all data, FocalPoint permanently removes:

- **Events** — All connector events and sync history from Canvas, Google Calendar, GitHub, and Fitbit.
- **Rules** — Custom rules and template packs you've installed or created.
- **Tasks** — All tasks in your planning pool, including rituals history.
- **Rewards & Streaks** — Credits earned, spent, streaks, unlocks, and multiplier state.
- **Penalties & Lockouts** — Escalation tier, bypass budget, debt, strict mode, and lockout windows.
- **Audit Log** — The complete tamper-evident audit chain (see below for receipt).
- **Connector Tokens** — Stored credentials for Canvas, Google Calendar, GitHub, and other integrations.
- **Caches & Backups** — Temporary files and local backup archives.

## What Is NOT Deleted

- **Wipe Receipt** — A JSON proof of deletion is saved to your device for your records. You can delete this file manually from `~/Library/Application Support/FocalPoint/wipe-receipts/` at any time.
- **App Installation** — The FocalPoint app itself remains installed. You can uninstall it manually via Settings > General > iPhone Storage.

## How to Delete Your Data

### On iOS

1. Open **FocalPoint** and tap the **Settings** tab.
2. Scroll to the **Data** section.
3. Tap **Delete all my data**.
4. Read the list of what will be deleted.
5. Tap **Delete All Data** (red button).
6. Confirm: tap **I Understand** when prompted.
7. Final confirmation: tap **Delete Everything**.

### On Android (Coming Soon)

Delete functionality will be available in FocalPoint v0.1. Contact support if you need to delete your data now.

## Understanding Your Wipe Receipt

When deletion completes, FocalPoint saves a **tamper-evident receipt** to prove the wipe happened. This receipt:

- **Is not reversible proof** — it shows only that data was deleted, not recovered.
- **Contains no sensitive data** — only timestamps, table counts, and a hash of the audit chain.
- **Is saved locally** — on iOS, in `~/Library/Application Support/FocalPoint/wipe-receipts/<timestamp>.json`.
- **Is yours to keep or delete** — copy it somewhere safe if you want proof, or delete it manually.

### Sample Receipt

```json
{
  "wiped_at": "2026-04-23T14:30:45.123Z",
  "pre_wipe_chain_hash": "abc123def456...",
  "deleted_counts": {
    "events": 245,
    "rules": 12,
    "tasks": 8,
    "wallet": 1,
    "penalty_state": 1,
    "audit_records": 1847,
    "connector_cursors": 3
  },
  "deleted_keychain_items": [],
  "deleted_paths": []
}
```

## Uninstalling After Deletion

After deleting your data:

1. Open Settings > General > iPhone Storage.
2. Find **FocalPoint** in the list.
3. Tap **Offload App** (keeps data) or **Delete App** (removes everything, including the app).

## Recovering Accidentally Deleted Data

**Data deletion is permanent and cannot be reversed.** FocalPoint does not maintain backups of deleted accounts. If you've deleted your data by mistake:

- You **cannot recover** events, rules, or tasks.
- You must **reconnect** your integrations (Canvas, Google Calendar, GitHub, Fitbit) and start fresh.
- Your **reward streaks** and wallet credits are gone.

If you created a manual backup before deletion (via the **Backup** feature in Settings), you can restore from that backup.

## GDPR Compliance

FocalPoint's data deletion complies with the **EU General Data Protection Regulation (GDPR)** Article 17 (right to erasure / right to be forgotten):

- You can request deletion of personal data at any time.
- All data is removed within the app and from local storage.
- Third-party integrations (Canvas, Google Calendar, GitHub) maintain their own data — delete those separately if needed.
- We do not send deletion requests to third-party services on your behalf.

## Questions?

If you have questions about data deletion or privacy, reach out to us:

- **Discord**: [FocalPoint Community](https://discord.gg/focalpoint)
- **Email**: support@focalpoint.app
- **GitHub Issues**: [github.com/KooshaPari/FocalPoint/issues](https://github.com/KooshaPari/FocalPoint/issues)

---

**Data deletion is irreversible. Please make sure this is what you want before proceeding.**
