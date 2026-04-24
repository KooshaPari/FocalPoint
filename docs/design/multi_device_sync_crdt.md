# FocalPoint Multi-Device Sync: CRDT Architecture

## Overview

This document specifies FocalPoint's evolution from single-device (v0.x) to multi-device (v1.0+) synchronization using Conflict-free Replicated Data Types (CRDTs). Users will be able to start a focus session on their iPhone, pause it on their iPad, and view the session's credits impact on their Mac—all without manual sync, conflict dialogs, or data loss.

---

## Strategic Context

**Current state (v0.x)**: Single-device, CloudKit backup only. Users cannot seamlessly switch between iPhone/iPad/Mac.

**Target state (v1.0)**: Offline-first, multi-device with automatic conflict resolution. Each device is an equal peer; syncs via CloudKit (primary), self-hosted server (future), or LAN peer-to-peer (tertiary).

**Key principles**:
1. No user-visible conflicts (CRDT handles merges automatically)
2. Offline-first: all devices can mutate state independently; sync catches up asynchronously
3. Append-only audit trail (event sourcing) remains source of truth for compliance/forensics
4. Wallet balance is special: monotonic sum only (never LWW for credits)
5. Cross-device determinism: same merge on device A, B, C yields identical state

---

## Library Selection: Loro

**Recommendation: Loro** (`loro-crdt` crate)

### Evaluation Matrix

| Criterion | Loro | Automerge | Yjs |
|-----------|------|-----------|-----|
| Language | Rust-native | JavaScript + Rust binding | JavaScript + Rust binding |
| Tree support | ✅ Full (movable lists) | ⚠️ Limited (no move semantics) | ⚠️ Limited |
| Rust async | ✅ Native tokio | ⚠️ Wrapped | ⚠️ Wrapped |
| Fast merge | ✅ ~1ms for typical docs | ⚠️ ~10ms (slower) | ✅ ~5ms |
| Snapshot size | ✅ Compact (binary) | ⚠️ Large (JSON bloat) | ✅ Compact |
| Community | 🟡 Growing (Chinese-led) | ✅ Large (funding) | ✅ Very large |
| Type safety | ✅ Strong (Rust enums) | ⚠️ Loose (JS-ish) | ⚠️ Loose |
| Mobile support | ✅ WASM (watchOS limitations) | ⚠️ WASM (heavy) | ✅ WASM |

**Decision**: Loro. Native Rust, movable-list support (critical for task reordering across devices), and fast merge semantics align with FocalPoint's event-sourcing foundation.

---

## Data Model: Tables with Per-Table Conflict Semantics

FocalPoint's schema maps cleanly to CRDT tables. Each table has explicit merge semantics:

### Table 1: Rules (Last-Write-Wins by `updated_at`)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub duration_minutes: u32,
    pub updated_at: i64, // Unix timestamp; LWW uses this
    #[serde(skip)]
    pub crdt_lamport: u64, // Loro metadata
}
```

**Merge logic**: If device A and B both edit rule ID `abc123`, the version with the **largest `updated_at`** wins. Discarded edits trigger an audit event `rule_edit_conflict_resolved`.

**Example**:
- Device A (iPhone): edits rule title → "Deep Work" at 2026-04-23 10:00:00
- Device B (Mac): edits rule title → "Focus Sprint" at 2026-04-23 09:50:00
- **Result**: "Deep Work" wins (later timestamp)
- Audit log: `{ type: "conflict", table: "rules", record_id: "abc123", loser_value: "Focus Sprint", winner_value: "Deep Work", resolved_at: 1713872400 }`

### Table 2: Tasks (LWW by `updated_at` + Additive Status Tombstones)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub title: String,
    pub status: TaskStatus, // draft, active, completed, archived
    pub updated_at: i64,
    pub created_at: i64,
    #[serde(skip)]
    pub crdt_tombstones: Vec<TaskStatusTombstone>,
}

pub struct TaskStatusTombstone {
    pub status: TaskStatus,
    pub timestamp: i64,
    pub device_id: Uuid, // Which device made this change
}
```

**Merge logic**: 
- Title/description: LWW by `updated_at` (same as rules)
- Status: **Additive tombstones**. If device A marks task as `completed` and device B marks it as `archived`, both events are preserved in `crdt_tombstones`. The final status is the **most recent** (by timestamp), but the conflict is logged.

**Example**:
- Device A (iPad): marks task → `completed` at 10:05:00
- Device B (Mac): marks task → `archived` at 10:03:00
- **Result**: Status is `completed` (later). Tombstone list shows both events.
- Audit log: `{ type: "task_status_conflict", task_id: "xyz", statuses: ["completed" (10:05), "archived" (10:03)], final_status: "completed" }`

### Table 3: Wallet (Monotonic-Sum Conflict-free Replicated Data Type)

This is the critical table. **Credits can only grow via valid mutations; they can never decrease except via valid redemptions.** LWW is forbidden because it could silently discard earned credits.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ledger: Vec<WalletMutation>, // Append-only
    pub balance: f64, // Derived: sum of all mutations
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletMutation {
    pub id: Uuid,
    pub mutation_type: WalletMutationType, // AddFocus, RedeemPerk, Bonus, Penalty
    pub amount: f64, // Always positive; type determines sign
    pub timestamp: i64,
    pub session_id: Option<Uuid>,
    pub rule_id: Option<Uuid>,
    pub idempotency_key: Uuid, // Prevent double-application
}

pub enum WalletMutationType {
    AddFocus(f64),    // +X credits from session
    RedeemPerk(f64),  // -X credits from perk
    Bonus(f64),       // +X from achievement
    Penalty(f64),     // -X from rule violation
}
```

**Merge logic**: 
- Take the **union of all mutations** from all devices
- Deduplicate by `idempotency_key` (each device generates a unique key when it creates a mutation)
- Recompute balance = sum of all unique mutations
- **Never use LWW**; always append

**Example**:
- Device A (iPhone): completes session → mutation `mut-001` (+10.5 credits) at 10:00:00
- Device B (Mac): offline, completes session → mutation `mut-002` (+8.0 credits) at 10:01:00 (local clock)
- Device C (iPad): already synced with A, starts to sync with B
- **Result**: Balance = 10.5 + 8.0 = 18.5 credits (union of both mutations)
- **No data loss**; both sessions contribute

**Conflict detection**: If two devices independently try to redeem the same perk (same `idempotency_key`), the second redemption is rejected with audit event `duplicate_redemption_attempt`.

### Table 4: FocusSessions (Append-Only Log)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FocusSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub rule_id: Uuid,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub paused_at: Option<i64>,
    pub resumed_at: Option<i64>,
    pub status: SessionStatus, // active, paused, completed, abandoned
    pub device_id: Uuid, // Which device created this session
}
```

**Merge logic**: Append-only. Each session is immutable once created. Status transitions (pause, resume, end) are recorded as separate events in the event log (not as mutations to the session itself).

**Conflict resolution**: Not applicable (immutable log). However, clock skew can cause out-of-order event delivery. Solution: use **Lamport timestamps** in addition to wall-clock time.

### Table 5: AuditLog (Immutable Append-Only Chain)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: AuditEventType,
    pub timestamp: i64,
    pub device_id: Uuid,
    pub previous_hash: String, // SHA-256 of prior event
    pub event_hash: String,    // SHA-256 of this event (for verification)
    pub payload: serde_json::Value,
}

pub enum AuditEventType {
    SessionCreated,
    SessionCompleted,
    CreditsEarned,
    PerkRedeemed,
    RuleCreated,
    RuleEdited,
    ConflictResolved,
    SyncApplied,
}
```

**Merge logic**: Union of all audit events. Deterministic order: sort by `(timestamp, event_hash)` to ensure all devices converge on the same sequence.

**Hash chain**: Each event contains the SHA-256 of the previous event. This allows forensic verification (replay) if needed.

---

## Sync Architecture: Three Tiers

### Tier 1: CloudKit (Primary, Free)

**Why CloudKit?**
- Zero server infrastructure cost
- Automatic encryption (iCloud private key)
- Per-user private databases (no multi-tenancy headache)
- Built-in conflict detection (CKServerChangeToken for incremental sync)
- Works across all Apple devices

**Sync flow**:
```
Device A mutates rule_id=abc → (1) save to local Loro store
                               → (2) queue to CloudKit sync
                               → (3) on next network, push to CloudKit
                               → (4) CloudKit broadcasts change to B, C

Device B receives CKServerChangeToken
                → (1) fetch change from CloudKit
                → (2) merge into local Loro store (CRDT handles conflicts)
                → (3) UI updates via subscription (or polling)
```

**CloudKit data format**: Serialize each CRDT table as binary blob (`Data`) in a custom zone per user:

```
Realm: com.focalpoint
Zones:
  - rules (custom zone)
  - tasks (custom zone)
  - wallet (custom zone)
  - focus_sessions (custom zone)
  - audit_log (custom zone)
```

Each zone has a `lastSyncToken` and `dataBlob` record.

### Tier 2: Self-Hosted Sync Server (Future)

For users who don't trust CloudKit (enterprises, high-privacy), a self-hosted server:
- Runs PostgreSQL + Rust sync engine
- Exposes REST API for device sync
- Maintains same CRDT semantics (Loro on backend)
- Peers with CloudKit (optional failover)

Not in v1.0; scoped for v1.5.

### Tier 3: LAN Peer-to-Peer (MultipeerConnectivity, Offline-First)

When devices are on the same WiFi (or nearby via Bluetooth), they can sync directly:
- **Zero external servers**: ideal for airplane mode, poor connectivity
- Uses Apple's **MultipeerConnectivity** framework
- Each device is a peer; no leader election needed
- Merge happens locally; conflicts resolved by CRDT

**Priority**: After v1.0 (nice-to-have); requires additional integration work.

---

## Migration from v0.x to v1.0

### Phase 1: Data Model Translation (No-Op for Users)

1. Export existing v0.x SQLite schema
2. Translate to CRDT-compatible format:
   - Rules → Rules table (LWW, migrate `updated_at` if missing)
   - Tasks → Tasks table (LWW + tombstones)
   - Wallet → Wallet table (replay all mutations, recompute balance)
   - FocusSessions → FocusSessions table (append-only)
   - Audit events → AuditLog table (immutable)
3. Initialize Loro document with translated data
4. Store both SQLite and Loro for parallel operation (safety net)

### Phase 2: Dual-Write Architecture

1. On mutation (user creates rule, redeems perk):
   - Write to SQLite (v0.x, for backwards compat)
   - Write to Loro (v1.0, for new multi-device)
   - Generate audit event in both stores
2. On read:
   - Prefer Loro if available; fallback to SQLite
   - Validate consistency
3. Run for 1–2 months (gradual rollout)

### Phase 3: Cutover

1. All new devices boot v1.0 (CloudKit + Loro)
2. Existing devices migrate on next app launch
3. Disable v0.x writes; run in read-only mode for 30 days (safety)
4. Audit logs from v0.x period are preserved

### Phase 4: Cleanup

1. Remove SQLite backend code
2. Archive v0.x codebase (reference only)
3. Update docs and SDKs

---

## Conflict Resolution Policies

### Policy 1: Last-Write-Wins (LWW) with Audit Trail

Used for: Rules, Tasks (body), FocusSessions (immutable, N/A), most metadata.

```rust
pub fn resolve_rule_conflict(remote: &Rule, local: &Rule) -> (Rule, AuditEvent) {
    let (winner, loser) = if remote.updated_at > local.updated_at {
        (remote.clone(), local.clone())
    } else {
        (local.clone(), remote.clone())
    };

    let audit = AuditEvent {
        event_type: AuditEventType::ConflictResolved,
        payload: json!({
            "table": "rules",
            "record_id": winner.id.to_string(),
            "conflict_type": "lww_by_timestamp",
            "winner": winner.updated_at,
            "loser": loser.updated_at,
            "loser_value": loser.title,
            "winner_value": winner.title,
        }),
        ..Default::default()
    };

    (winner, audit)
}
```

### Policy 2: Monotonic Sum (Wallet Only)

Used for: Wallet balance.

```rust
pub fn resolve_wallet_conflict(remote: &WalletEntry, local: &WalletEntry) 
    -> (WalletEntry, Vec<AuditEvent>) {
    // Merge mutations
    let mut merged_mutations = local.ledger.clone();
    for rem_mut in &remote.ledger {
        if !merged_mutations.iter().any(|m| m.idempotency_key == rem_mut.idempotency_key) {
            merged_mutations.push(rem_mut.clone());
        }
    }

    // Recompute balance
    let new_balance: f64 = merged_mutations.iter().map(|m| m.amount).sum();

    let audit = AuditEvent {
        event_type: AuditEventType::SyncApplied,
        payload: json!({
            "table": "wallet",
            "mutation_count": merged_mutations.len(),
            "old_balance": local.balance,
            "new_balance": new_balance,
        }),
        ..Default::default()
    };

    (
        WalletEntry {
            ledger: merged_mutations,
            balance: new_balance,
            updated_at: now(),
            ..local
        },
        vec![audit],
    )
}
```

### Policy 3: Additive Tombstones (Task Status)

Used for: Task status transitions.

```rust
pub fn resolve_task_status_conflict(
    remote: &Task, local: &Task
) -> (Task, AuditEvent) {
    let mut merged_tombstones = local.crdt_tombstones.clone();
    
    for rem_ts in &remote.crdt_tombstones {
        if !merged_tombstones.iter().any(|ts| ts.timestamp == rem_ts.timestamp && ts.device_id == rem_ts.device_id) {
            merged_tombstones.push(rem_ts.clone());
        }
    }
    
    // Final status = most recent timestamp
    merged_tombstones.sort_by_key(|ts| ts.timestamp);
    let final_status = merged_tombstones.last().map(|ts| ts.status.clone()).unwrap_or(local.status.clone());
    
    let audit = AuditEvent {
        event_type: AuditEventType::ConflictResolved,
        payload: json!({
            "table": "tasks",
            "record_id": local.id.to_string(),
            "conflict_type": "additive_tombstones",
            "tombstone_count": merged_tombstones.len(),
            "final_status": final_status,
        }),
        ..Default::default()
    };

    (
        Task {
            status: final_status,
            crdt_tombstones: merged_tombstones,
            updated_at: now(),
            ..local
        },
        audit,
    )
}
```

### Policy 4: Append-Only (Audit Log, Focus Sessions)

No conflict resolution needed. Union all events, sort deterministically.

```rust
pub fn merge_audit_logs(events_a: Vec<AuditEvent>, events_b: Vec<AuditEvent>) -> Vec<AuditEvent> {
    let mut merged = events_a.into_iter().chain(events_b).collect::<Vec<_>>();
    merged.sort_by(|a, b| {
        a.timestamp.cmp(&b.timestamp)
            .then_with(|| a.event_hash.cmp(&b.event_hash))
    });
    merged.dedup_by_key(|e| e.id);
    merged
}
```

---

## Implementation Phases (Effort Breakdown)

### Phase 1: Loro Integration & Data Model (8–12 batches)
- Integrate `loro-crdt` crate into phenotype-infrakit
- Define CRDT schema (Rule, Task, Wallet, FocusSession, AuditLog)
- Serialize/deserialize for CloudKit
- Unit tests for each table merge

### Phase 2: CloudKit Backend (10–15 batches)
- CloudKit zone setup (per-user private database)
- Sync state machine (fetch changes, apply conflicts, push mutations)
- CKServerChangeToken polling / subscription
- Error handling (network, quota)
- Integration tests (simulator + device)

### Phase 3: Dual-Write Migration Path (6–10 batches)
- v0.x → v1.0 data translation
- Parallel SQLite + Loro writes (safety net)
- Deprecation timeline (docs, UI warnings)
- Data validation (consistency checks)

### Phase 4: Multi-Device Convergence (4–8 batches)
- CloudKit broadcast (push changes to all devices)
- Watch + iPad + Mac sync
- Offline-first queue (mutations queued locally if network down)
- E2E tests across 3+ simulators

### Phase 5: Optional: LAN P2P & Self-Hosted (12–18 batches, future)
- MultipeerConnectivity integration
- Self-hosted Rust sync server
- Fallback logic (CloudKit → self-hosted → P2P)

**Total for v1.0 (CloudKit MVP)**: 28–45 tool-call batches across 4 phases.

---

## Success Metrics & Validation

1. **Deterministic convergence**: Start 3 simulators, mutate on each, verify all converge to identical state within 5 seconds
2. **Offline survivability**: Disable network on device B, mutate on devices A & C, restore network on B → B applies conflicts correctly
3. **Wallet integrity**: No mutations lost; balance always = sum of ledger
4. **Audit trail completeness**: Every user mutation recorded with device_id, timestamp, hash
5. **CloudKit quota**: Stay <100 MB per user (current typical is 10–30 MB)

---

## Future Roadmap & Extensions

1. **E2EE (End-to-End Encryption)** v1.2: CloudKit encrypted via user's iCloud key (not Apple key). Requires careful key management.
2. **Time Travel / Point-in-Time Restore**: Replay audit log to recover state at any prior date (compliance feature).
3. **Selective Sync**: User chooses which tables to sync across devices (e.g., keep tasks local, sync wallet globally).
4. **Conflict Analytics**: Dashboard showing which tables conflict most often, which devices clash (for product insights).
5. **Batch Operations**: API for bulk imports (CSV) without conflict explosion.

---

## Summary

FocalPoint's multi-device sync unifies smartphone, tablet, and desktop experiences via CRDTs. Loro provides the CRDT engine; CloudKit provides the sync backbone; four distinct conflict policies (LWW, monotonic-sum, additive-tombstones, append-only) ensure correctness without user intervention. The 28–45 batch implementation covers data model, CloudKit integration, migration, and validation across 4 phases. No data loss, no silent conflicts, full offline-first support.
