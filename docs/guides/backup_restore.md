# Full Backup & Restore Guide

FocalPoint encrypts and exports your complete data — wallet, penalties, rules, tasks, audit trail, templates, and settings — into a single passphrase-protected file. Walk away from your device with everything, restore on a new one.

## How It Works

**Architecture:**
1. **Manifest** — JSON metadata: version, device ID, timestamp, data sections
2. **TAR archive** — Contains manifest + SHA-256 hash (tamper detection)
3. **ZSTD compression** — Reduces size, faster I/O
4. **Age encryption** — Modern GPG successor; passphrase → Scrypt KDF → ChaCha20-Poly1305

All encryption happens on-device. FocalPoint never sees your passphrase.

## iOS / iPad

### Create Backup

1. Open FocalPoint → **Settings** → **Data** section
2. Tap "Create encrypted backup"
3. A sheet appears: enter and confirm a strong passphrase
   - Passphrase strength indicator warns if weak (zxcvbn-like entropy score)
4. Tap "Create"
5. System saves to `Downloads` or temp file; **Share sheet** appears
   - Send via AirDrop to another device
   - Email to yourself (attachments supported)
   - Save to iCloud Drive / Google Drive

### Restore Backup

1. **On new device:** FocalPoint → **Settings** → **Data**
2. Tap "Restore from backup"
3. **DocumentPicker** opens; select encrypted `.backup` file
4. Passphrase prompt sheet; enter decryption passphrase
5. On success: table shows counts per section
   - ✅ 5 rules, 120 tasks, 1 wallet, 3 penalties...
6. Data is merged into local SQLite (non-destructive; existing items not overwritten unless conflict)

## CLI

### Create Backup

```bash
# Passphrase from environment variable
export BACKUP_PASS="your-long-passphrase"
focus backup create --out ./focalpoint-backup.age --passphrase-from-env BACKUP_PASS

# Passphrase from stdin (secure prompt)
focus backup create --out ./focalpoint-backup.age
# → Prompts: "Enter passphrase: "
```

**Output:** Binary blob (`focalpoint-backup.age`), ~10–50 KB depending on data size.

### Restore Backup

```bash
focus backup restore --in ./focalpoint-backup.age --passphrase-from-env BACKUP_PASS
# → Decrypts, unpacks, upserts into SQLite
# → Prints restore report:
#    Audit records: 42
#    Events: 128
#    Rules: 5
#    Wallets: 1
#    Penalties: 3
#    Tasks: 120
#    Templates: 4
```

## What's Backed Up

- ✅ **Wallet** — credit balance, multiplier state
- ✅ **Penalties** — lockout windows, escalation tiers
- ✅ **Rules** — rule definitions, priorities, triggers/actions
- ✅ **Tasks** — planning domain (title, deadline, duration, status)
- ✅ **Templates** — task templates, ritual templates, starter packs
- ✅ **Audit trail** — tamper-evident SHA-256 hash chain
- ✅ **Events** — normalized sync events for replay/recovery

## What's NOT Backed Up

- ❌ **OAuth tokens** — stored in device Keychain, by design. Re-authenticate on new device.
- ❌ **Connector cursors** — pagination pointers into Calendar, GitHub, Canvas. Re-hydrated on first sync after restore.

**Rationale:** Tokens are per-device secrets; cursors are ephemeral and rebuilt from the event stream. Backing them up creates a security/sync risk.

## Security

### Encryption Strength

| Component | Algorithm | Details |
|-----------|-----------|---------|
| Key derivation | Scrypt | N=2^16, r=8, p=1 (age default) |
| Cipher | ChaCha20-Poly1305 | AEAD (authenticated encryption) |
| Passphrase entropy | User-chosen | Strength meter recommends 80+ bits |

**Recommendation:** Use a passphrase with 4+ words (e.g., "apple-sunrise-rhythm-bold") for ~60 bits; aim for 5–6 words for comfortable security.

### Verification

- **Manifest hash:** SHA-256 computed at creation time, embedded in archive, verified at restore
- **Archive integrity:** tar + zstd payloads check for corruption
- **Wrong passphrase:** Decryption fails with clear error ("Invalid passphrase or corrupted archive")
- **Version mismatch:** Backup from future FocalPoint versions rejected cleanly

## Backup Files

### File Format

- **Extension:** `.backup` (convention; actually a tar+zstd+age blob)
- **MIME type:** `application/octet-stream`
- **Size:** 10–50 KB (compressed JSON manifest + metadata)
- **Readable by:** FocalPoint CLI + iOS/Android apps only

### Naming Convention

```
FocalPoint-YYYY-MM-DD-HHmmss.backup
```

Example: `FocalPoint-2026-04-23-143022.backup`

### Storage

- **Best practice:** CloudKit, iCloud Drive, or encrypted storage (Bitwarden, 1Password)
- **Avoid:** email inbox, unencrypted cloud
- **Rotate:** Keep 2–3 recent backups; discard monthly

## Troubleshooting

### "Wrong passphrase or corrupted archive"

- Passphrase incorrect → check caps lock, spaces
- File corrupted → re-download/re-create backup
- Age version mismatch (rare) → ensure FocalPoint is up to date

### "Version mismatch: expected 0.0.1, got X.Y.Z"

Backup was created with a newer FocalPoint version. Update FocalPoint to match.

### "File not found" (CLI)

Check path: `ls -lh ./focalpoint-backup.age`

### Large backups take time to encrypt

Normal: 10 MB of audit records + events may take 5–10 seconds on older devices. Progress indicator in UI.

## Examples

### Scenario: iPhone → iPad Transfer

1. **iPhone:** Settings → Data → "Create encrypted backup"
   - Passphrase: `my-secret-pass-123`
   - AirDrop to Mac
2. **Mac:** Open in Finder, email to self or upload to Drive
3. **iPad:** Settings → Data → "Restore from backup"
   - Pick file from iCloud Drive
   - Enter: `my-secret-pass-123`
   - ✅ All tasks, rules, wallet restored

### Scenario: Local Backup & Restore (CLI)

```bash
# On laptop
export BACKUP_PASS="ultra-secret-phrase"
cd ~/focalpoint-data
focus backup create --out backup-$(date +%Y%m%d).backup --passphrase-from-env BACKUP_PASS

# Later: move to new machine, restore
focus backup restore --in backup-20260423.backup --passphrase-from-env BACKUP_PASS
```

## Privacy

**FocalPoint never transmits:**
- Passphrases
- Encryption keys
- Backup files (unless you explicitly export)

**Backups are encrypted on-device with your passphrase.** No server involvement.

## Advanced: Custom Entropy Estimator

If the UI passphrase meter feels off, measure yourself:

```python
import math
alphabet_sizes = {
    'lowercase': 26,
    'uppercase': 26,
    'digits': 10,
    'symbols': 32,
}
charset_size = sum(alphabet_sizes.values())
entropy = len("your-passphrase") * math.log2(charset_size)
print(f"Entropy: {entropy:.1f} bits")
```

Aim for **80+ bits** (very secure) or **128+ bits** (paranoid).

---

**Questions?** Check `docs/research/open_questions.md` for known gaps (e.g., multi-device sync scope, encrypted transport layer).
