# Multi-Device Sync Architecture: FocalPoint v0.1

**Status:** Design Doc (RFC)  
**Date:** April 2026  
**Scope:** iPhone, iPad, Mac — offline-first eventually-consistent state sharing  
**Version:** 1.0

## Executive Summary

FocalPoint is currently single-device + local-first. Many users want to share state (wallet balance, tasks, rules, connectors) across iPhone + iPad + Mac while maintaining:

- **Offline-first:** The app works without network; conflicting edits on two devices eventually merge.
- **Audit-chain integrity:** The tamper-evident SHA-256 hash chain is never synced; each device maintains its own local chain.
- **E2E encryption:** All sync data is encrypted in transit and at rest.
- **No mandatory server:** Sync must work peer-to-peer via Apple CloudKit (v1) or CRDT replication (v2+).

This document evaluates three approaches, recommends **CloudKit for v1**, and sketches the schema, conflict resolution, signing, and privacy model.

---

## 1. Requirements

### Functional

| Req | Description | Source |
|-----|-------------|--------|
| FR-SYNC-001 | Offline changes (rules, tasks, wallet balance) persist locally without network. | Offline-first mandate |
| FR-SYNC-002 | Two devices editing the same rule/task offline must eventually merge into one consistent state. | Eventual consistency |
| FR-SYNC-003 | The audit chain is device-local; new entries are never synced. Sync does not break the chain. | Audit integrity |
| FR-SYNC-004 | All sync data in CloudKit is E2E encrypted in user's private database. | Privacy-first |
| FR-SYNC-005 | User can opt-in/out of sync in Settings; default OFF until v1 is stable. | Consent |
| FR-SYNC-006 | Sync works peer-to-peer (via CloudKit) with zero mandatory infrastructure. | No-server |
| FR-SYNC-007 | Connector tokens (GitHub PAT, Google refresh token) remain per-device; never synced. | Security |

### Non-Functional

- **Platform:** iOS 15+, iPadOS 15+, macOS 12+.
- **Latency:** Sync round-trip under 5s for typical payloads (<1MB).
- **Bandwidth:** Efficient incremental sync; no full-dump on every change.
- **Conflict window:** Last-writer-wins for tasks and rules (user sees merge UI if >30s apart).
- **Recovery:** If sync fails mid-merge, local state remains intact; next sync retries.

---

## 2. Options Survey

### Option 1: Apple CloudKit + CKShare

**Approach:**  
- Use `CKContainer.default().privateCloudDatabase` to store sync records (Rules, Tasks, Wallets).
- Each record is a `CKRecord` with encrypted `CKAsset` payloads (user's private database = auto E2E encryption).
- Device-specific signing: each device signs records with its own keypair; CloudKit verifies signature on pull.
- Conflict detection: check record's `modificationDate` on pull; if >30s newer, prompt user merge.

**Pros:**
- Zero infrastructure; Apple handles sync, storage, replication.
- Private database = automatic E2E encryption; user doesn't manage keys.
- Built-in account authentication (iCloud Sign-In).
- Native iOS/iPadOS/macOS support; no third-party dependencies.
- Tamper-evident: device signature prevents tampering.
- Graceful fallback: works offline; syncs when network available.

**Cons:**
- Apple-only (iOS/iPadOS/macOS); cannot extend to web later without new backend.
- Requires iCloud Sign-In; users without iCloud account cannot sync.
- CKRecord schema changes are non-trivial (requires migration).
- CloudKit quota limits (10 GB free storage; 40 MB/s upload).
- No real-time sync notification (polling interval ~15s on background).

**Effort:** 2–3 weeks (scaffold + signature integration + basic merge UI).

---

### Option 2: CRDT (Automerge, Yjs, Loro)

**Approach:**  
- Each rule/task is wrapped in a CRDT document (e.g., Automerge or Loro).
- Devices exchange deltas (changes) peer-to-peer or via a sync server.
- CRDT automatically merges concurrent edits without user intervention.
- Loro (Rust-native, 2026-ready) is fastest for Rust integration.

**Pros:**
- Automatic conflict resolution; no merge UI needed.
- Works peer-to-peer or with optional server.
- Language-agnostic; can extend to web, Android easily.
- Auditable: CRDT ops are timestamped and content-addressed.
- No cryptography burden; CRDT is deterministic.

**Cons:**
- CRDT per-record wrapping adds ~2 KB overhead per document.
- Learning curve: users see "last edit wins if in same CRDT op" (non-intuitive).
- Loro / Automerge maturity: Loro is fast but still pre-1.0; Automerge is stable but slower.
- Sync transport: need to build HTTP/QUIC relay or use third-party (Replicache, Partykit).
- Audit chain incompatibility: CRDT versions will diverge per device (audit becomes meaningless).

**Effort:** 4–6 weeks (CRDT integration + sync transport + schema design).

---

### Option 3: Git-Based Sync

**Approach:**  
- Reuse FocalPoint's audit chain structure (content-addressable, SHA-256 hashed).
- Each device is a "branch"; sync merges branches via `git rebase` or `git merge`.
- Conflict resolution: 3-way merge on JSON state.

**Pros:**
- Leverages existing audit chain infrastructure.
- Familiar merge model (rebase vs. merge).
- Strong auditability; every sync is a commit.

**Cons:**
- User-facing model is confusing ("Your rule got rebased" 🤔).
- Git overhead: full history is bulky (~1–5 MB per device).
- Merge conflicts on binary data are unresolvable.
- CloudKit/iCloud File Sync is cumbersome for .git repos.

**Effort:** 3–4 weeks; not recommended due to user-facing complexity.

---

## 3. Recommendation: CloudKit for v1

**Decision:** Implement v1 with **Apple CloudKit**. Rationale:

1. **Zero infrastructure:** User doesn't pay for servers; we don't manage them.
2. **Best UX for Apple users:** Native iCloud integration; no login flow.
3. **E2E encryption out-of-the-box:** Private database = auto encryption.
4. **Tamper-evident:** Device signatures ensure records weren't modified in transit.
5. **Fallback plan:** Export sync records as Automerge docs later; migrate to CRDT backend if web surface is needed.
6. **Effort:** Fastest path to v1 (2–3 weeks).

**v2 Plan:** Add CRDT export (Loro or Automerge) so users can opt into cross-platform sync via an optional self-hosted server. Deferred.

---

## 4. Schema & Sync Scope

### What Syncs

| Entity | Syncs? | Reason |
|--------|--------|--------|
| **Rules** | ✅ Yes | User-authored policies; identical across devices. |
| **Tasks** | ✅ Yes | Shared task list (e.g., "Run 10 miles this week"). |
| **Wallet Balance** | ✅ Yes | Reward points must be consistent across devices. |
| **Rituals** | ✅ Yes | Habit definitions; identical across devices. |
| **Connector Configs** | ❌ No (tokens only) | Per-device; tokens stay in Keychain. |
| **Audit Chain** | ❌ No | Device-local only; not synced. |
| **Penalties** | ✅ Yes | Penalty state (e.g., "app blocked until 10pm"). |

### CloudKit Record Schema

Each synced entity maps to a `CKRecord` with a signed JSON payload.

#### Rule Record

```
recordType: "Rule"
recordName: "<rule-id>"  // uuid
fields:
  - rule_json: CKAsset (encrypted JSON, max 4 MB)
  - device_id: String   (uuid of device that last edited)
  - device_signature: String (Ed25519(rule_json + device_id, device_key))
  - version: Int64      (for conflict detection)
  - synced_at: CKReference (timestamp)
```

#### Task Record

```
recordType: "Task"
recordName: "<task-id>"  // uuid
fields:
  - task_json: CKAsset (encrypted JSON)
  - device_id: String
  - device_signature: String
  - version: Int64
  - synced_at: CKReference
```

#### Wallet Record

```
recordType: "Wallet"
recordName: "<user-id>"  // single record per user
fields:
  - wallet_json: CKAsset (encrypted JSON with balance + mutation log)
  - device_id: String
  - device_signature: String
  - version: Int64
  - synced_at: CKReference
```

---

## 5. Conflict Resolution

### Last-Writer-Wins (LWW) with Merge UI

**Rule:** When pulling a record:
1. **Check version and `modificationDate`.**
   - If `modificationDate` on server is ≤30s older than local edit, accept local version (no conflict).
   - If server version is >30s newer, prompt user: "This rule was edited on another device. Keep your edits or use the latest?"

2. **If user chooses "use latest,"** pull and apply server version.

3. **If user chooses "keep mine,"** push local version back (overwriting server, incrementing version).

4. **For Wallet & Penalties:**
   - Treat as a CRDT counter (device edits are deltas).
   - On sync: `wallet.balance_log.append({ delta: +5, device: "iphone", timestamp: T })`.
   - Compute balance from log to avoid LWW errors.

### Audit Chain Remains Untouched

- **Local audit chain is immutable.** Sync never writes to it.
- **New sync action produces a new audit record:** `{ type: "SyncPush", payload: { pushed_record_ids: [...], status: "ok" } }`.
- Each device has its own audit chain with its own operations; chains never merge.

---

## 6. Signing & Verification

### Per-Device Key Pair

**On first app launch:**
- Generate Ed25519 keypair for this device.
- Store private key in iOS Keychain / macOS Keychain.
- Store public key in CloudKit record: `Device { device_id, public_key }`.

**On every push:**
- Sign the record payload: `signature = Ed25519Sign(canonical_json(rule), device_private_key)`.
- Include `device_id` and `signature` in the CloudKit record.

**On every pull:**
- Fetch the device's public key from CloudKit.
- Verify: `Ed25519Verify(canonical_json(rule), signature, device_public_key)`.
- If verification fails, discard the record and log an error (tamper alert).

---

## 7. Privacy Posture

### E2E Encryption

- **CloudKit Private Database:** All records stored in the user's private database are automatically encrypted with a key derived from their iCloud password.
- **At Rest:** Server-side encryption via Apple's infrastructure (user cannot inspect keys; Apple cannot read data without password).
- **In Transit:** TLS 1.3 between device and CloudKit; payload is ciphertext.
- **User Key Control:** User controls access via iCloud Sign-In; signing out removes device's ability to sync.

### Per-Device Signing

- Device signatures prevent tampering by another app or middleman (even if CloudKit were to be breached).
- If an attacker gains access to CloudKit, they cannot forge a signature without the device's private key.

### Connector Token Isolation

- **GitHub PAT, Google refresh token, Canvas token:** Stored in device-local Keychain.
- **Never synced.** User must re-authenticate on a new device.
- Rationale: tokens are device-ephemeral; refresh tokens can be revoked; reauth is a security checkpoint.

### Audit Chain Privacy

- Audit chain is local-only; sync does not expose it.
- Audit chain can reference synced records but is not itself synced.

---

## 8. Sync Flow

### Push Phase (Device → CloudKit)

```
1. Identify local changes since last sync (check local modification timestamp).
2. For each changed record:
   a. Compute canonical JSON.
   b. Sign with device private key.
   c. Create/update CKRecord with signed payload + device_id.
   d. Increment version.
3. Batch save to privateCloudDatabase.
4. Record SyncPush audit entry: { pushed: N, status: "ok" }.
```

### Pull Phase (CloudKit → Device)

```
1. Query CKRecords with modificationDate > last_sync_ts.
2. For each record:
   a. Verify device signature (fetch device's public key if needed).
   b. If verification fails, skip and alert.
   c. If version > local version, compare modificationDate:
      - >30s newer: prompt user (merge UI).
      - ≤30s newer: accept (no conflict).
      - local is newer: skip (don't downgrade).
3. Apply accepted changes to local SQLite.
4. Record SyncPull audit entry: { pulled: N, conflicts: M, status: "ok" }.
```

### Merge UI (Triggered on Pull)

**User sees:**
```
"Rule 'Study Math' was edited on your Mac at 3:15 PM.
Your iPhone edit (3:45 PM) conflicts. Use which version?"

[Use Mac version] [Use iPhone version]
```

**Background:**
- Shows both versions side-by-side.
- User picks one; local version is applied or discarded.
- Sync continues with chosen version.

---

## 9. Implementation Roadmap

### Phase 1: Rust Scaffold (Week 1)

**New crate:** `crates/focus-sync-store/`

- Trait `SyncStore`: abstract sync backend (trait-only; no CloudKit code).
- Trait methods: `push()`, `pull()`, `verify_signature()`.
- Return type: `SyncOutcome { pushed: u32, pulled: u32, conflicts: Vec<ConflictRecord> }`.
- Impl: `MemorySyncStore` for tests.
- Unit tests: trait surface, MemorySyncStore round-trip.

### Phase 2: Swift Scaffold (Week 2)

**New file:** `apps/ios/FocalPoint/Sources/FocalPointApp/CloudKitSync.swift`

- Struct `CloudKitSyncClient`: wraps `CKContainer.default().privateCloudDatabase`.
- Method `init()`: check iCloud sign-in; return status.
- Stub methods: `push(_ records: [SyncRecord])`, `pull()`, `verifySignature()`.
- `@AppStorage("app.cloudSyncEnabled")` toggle in Settings.
- Do NOT implement full CloudKit record mapping yet.

### Phase 3: Settings Integration (Week 2)

**Update:** `apps/ios/FocalPoint/Sources/SettingsView/SyncSettings.swift`

- Toggle: "Sync across devices" (default OFF until v1 stable).
- Status row: "Last synced 2 min ago" or "iCloud unavailable — sign in from System Settings."
- Button: "Sync now" (manual trigger).
- Diagnostics: "Connected to iCloud" / "No iCloud account" / "Network unavailable".

### Phase 4: iCloud Entitlement (Week 2)

**Update:** `project.yml` (Xcode Project)

Add capabilities:
```yaml
code_sign_identity: "Apple Development"
entitlements:
  - com.apple.developer.icloud-container-identifiers:
    - iCloud.com.koosha.focalpoint
  - com.apple.developer.icloud-services:
    - CloudKit
```

**Provisioning profile:** Requires Apple Developer account (Team ID). Device build will fail without a valid provisioning profile that includes CloudKit entitlement; this is expected and documented.

### Phase 5: Integration & Merge (Week 3)

- Wire `CoreHolder` to call `sync()` on 60s foreground timer.
- Audit trail: SyncPush/SyncPull records append to audit chain.
- E2E test: two simulators, rule edit on one, sync to other, verify state.

---

## 10. Future Directions: CRDT Export (v2)

Once v1 is stable and we need web support:

1. **Export to Loro:** Rules, tasks, wallet as Loro documents (CRDT-compatible format).
2. **Sync Server:** Optional self-hosted Loro sync server (or Replicache provider).
3. **Multi-Platform:** Web, Android, Windows via Loro docs.
4. **Backward Compat:** CloudKit records export → Loro docs on first web login; CRDT history starts from export.

---

## 11. Open Questions

| Q | Answer |
|---|--------|
| Do users want sync enabled by default? | No; default OFF. User can toggle in Settings. Stable v1 only. |
| What if iCloud is disabled on device? | Sync is unavailable. UI shows "Sign in to iCloud in Settings." User can still use app offline. |
| Can a user sync to multiple iCloud accounts? | No; one account per device. Sync is per-account; switch accounts = reset sync state. |
| Can the audit chain reference synced records? | Yes; audit records can reference rule IDs / task IDs. But audit chain itself is not synced. |
| What about encrypted local database? | Separate concern; out of scope. Audit chain is append-only and checksummed; encryption can be added orthogonally. |

---

## 12. Acceptance Criteria

- [x] Design doc (this file) complete.
- [ ] `crates/focus-sync-store` trait surface scaffolded + tests compile.
- [ ] `CloudKitSyncClient` Swift stub compiles.
- [ ] Settings integration (toggle + status) compiles.
- [ ] iCloud entitlement declared in `project.yml`.
- [ ] `cargo test -p focus-sync-store` passes.
- [ ] `xcodebuild` passes with entitlement (device build may fail without provisioning; documented).

---

## 13. References

- **CloudKit Docs:** https://developer.apple.com/cloudkit/
- **Ed25519:** RFC 8032
- **iOS Keychain:** https://developer.apple.com/documentation/security/keychain_services
- **Loro CRDT:** https://loro.dev (for v2 reference)
- **Audit Chain (FocalPoint):** `crates/focus-audit/src/lib.rs`

---

**Glossary:**

- **E2E Encryption:** End-to-end encryption; only sender and receiver have decryption keys.
- **CRDT:** Conflict-free Replicated Data Type; automatic conflict resolution for concurrent edits.
- **LWW:** Last-writer-wins; the most recent edit takes precedence.
- **SyncPush:** Audit record for pushing changes to CloudKit.
- **SyncPull:** Audit record for pulling changes from CloudKit.
- **Tamper-evident:** A mechanism that reveals if data has been modified.

---

**Word Count:** 3,247

**Approval:** RFC; ready for Phase 1 scaffold.
