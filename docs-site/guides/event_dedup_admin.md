# Event Deduplication — Admin Guide

**Traces to:** FR-EVT-DEDUP-001

## Overview

FocalPoint deduplicates events across polling and webhook ingress paths to prevent duplicate processing. This guide explains how deduplication works, how to monitor it, and how to force re-processing of deduplicated events.

## How Deduplication Works

### Algorithm

1. **Compute canonical hash** from `(connector_id, event_type, normalized_payload_json)`
   - Payload is normalized by recursively sorting JSON object keys
   - This ensures identical events produce identical hashes regardless of JSON key order
   
2. **Check deduplicator state**
   - In-memory: fast HashSet lookup (tests, single-process)
   - SQLite: persistent bloom filter + exact-match table with TTL

3. **On duplicate detection**
   - Event is skipped entirely
   - An `AuditRecord` is created with kind=`dedup_skipped` (count + connector_id + hash)
   - No event is appended to the store

4. **TTL purge**
   - Deduplicated hashes expire after 30 days
   - Background task removes expired entries periodically

### Wiring Points

#### Polling (SyncOrchestrator)

```rust
let dedup = Arc::new(SqliteDeduplicator::new(db));
let mut orch = SyncOrchestrator::with_default_retry()
    .with_deduplicator(dedup);
```

Every connector poll flows through `DeduplicatingEventSink::append()` before reaching the EventStore.

#### Webhooks (focus-webhook-server)

```rust
let state = AppState {
    event_sink: Some(dedup_sink),
    // ...
};
```

Webhook handlers extract events, then call `process_webhook_events_with_dedup()` to append through the dedup sink.

## Monitoring Deduplication

### Audit Table Query

Deduplicated events are recorded in the `audit_records` table:

```sql
SELECT
  id,
  timestamp,
  connector_id,
  kind,
  details
FROM audit_records
WHERE kind = 'dedup_skipped'
ORDER BY timestamp DESC
LIMIT 100;
```

**Sample output:**

```
id  | timestamp           | connector_id | kind           | details
----|-------------------|--------------|----------------|---------------------
123 | 2026-04-24T12:30  | github       | dedup_skipped  | {"hash": "abc123...", "count": 2}
124 | 2026-04-24T13:15  | canvas       | dedup_skipped  | {"hash": "def456...", "count": 1}
```

### Last 24h Dedup Count

```sql
SELECT
  connector_id,
  COUNT(*) as dedup_events
FROM audit_records
WHERE kind = 'dedup_skipped'
  AND timestamp >= datetime('now', '-1 day')
GROUP BY connector_id
ORDER BY dedup_events DESC;
```

### Dedup Statistics by Connector

```sql
SELECT
  connector_id,
  COUNT(*) as total_skipped,
  COUNT(DISTINCT details->>'hash') as unique_hashes,
  MAX(timestamp) as last_dedup
FROM audit_records
WHERE kind = 'dedup_skipped'
GROUP BY connector_id;
```

## Force Re-Processing

### Scenario: False-Positive Deduplication

If a legitimate event was incorrectly deduplicated (e.g., connector returned a logically different event with identical hash):

#### Option 1: Clear the Dedup Entry

Remove the hash from the deduplicator so the next identical event is processed:

```sql
DELETE FROM event_dedup_hashes
WHERE hash = 'abc123...'
  AND connector_id = 'github';
```

Next occurrence of the event will not be deduplicated.

#### Option 2: Manual Event Insert

Insert the event directly into the event store (admin recovery only):

```sql
INSERT INTO events (
  event_id,
  connector_id,
  account_id,
  event_type,
  occurred_at,
  effective_at,
  dedupe_key,
  confidence,
  payload,
  raw_ref
) VALUES (
  'uuid-here',
  'github',
  'account-uuid',
  'PullRequestOpened',
  datetime('now'),
  datetime('now'),
  'dedup-key',
  1.0,
  '{"id": "123", ...}',
  NULL
);
```

**Note:** This bypasses deduplication. Use only after investigation.

#### Option 3: Restart Deduplicator

Clear the entire in-memory dedup state (ephemeral, single-process only):

```rust
// In tests or dev environments
let dedup = Arc::new(InMemoryDeduplicator::new());
// Old entries are forgotten
```

For persistent (SQLite) state, purge manually:

```sql
DELETE FROM event_dedup_hashes
WHERE created_at < datetime('now', '-30 days');
```

## FAQ

### Q: Why was my event deduplicated?

**A:** If two events from the same connector have:
- Same event type
- Same payload (after JSON key normalization)

They produce the same canonical hash and are treated as duplicates within the 30-day TTL window.

**To investigate:**
1. Compute the canonical hash of the event
2. Query the dedup table: `SELECT * FROM event_dedup_hashes WHERE hash = '...'`
3. Check the `marked_at` timestamp — if recent, the event was legitimate but arrived twice

### Q: Can I disable deduplication?

**A:** Yes, for testing or recovery:

```rust
// Polling: omit with_deduplicator() call
let orch = SyncOrchestrator::with_default_retry();

// Webhooks: set event_sink to None
let state = AppState {
    event_sink: None,
    // ...
};
```

Events will no longer be deduplicated. **Note:** This may cause duplicate events in the store.

### Q: What if the dedup table grows too large?

**A:** Run the periodic purge task manually:

```sql
DELETE FROM event_dedup_hashes
WHERE marked_at < datetime('now', '-30 days');
```

Or schedule a cron job:
```bash
0 2 * * * sqlite3 core.db "DELETE FROM event_dedup_hashes WHERE marked_at < datetime('now', '-30 days')"
```

### Q: How do I verify dedup is working?

**A:**

1. **Check audit records:**
   ```sql
   SELECT COUNT(*) FROM audit_records WHERE kind = 'dedup_skipped' AND timestamp >= datetime('now', '-1 hour');
   ```

2. **Trigger a duplicate manually:**
   - Poll the same connector twice in quick succession
   - Verify only one event appears in the store
   - Verify an audit record with kind=`dedup_skipped` is created

3. **Test webhook + polling overlap:**
   - Send a webhook delivery
   - Immediately poll the connector
   - Verify only one event persists

## Audit Trail Format

Each deduplicated event creates an `AuditRecord` with:

```rust
AuditRecord {
    id: Uuid,
    timestamp: DateTime<Utc>,
    connector_id: String,
    kind: "dedup_skipped",
    details: {
        "hash": "sha256-hex-string",
        "original_event_id": "uuid-of-skipped-event",
        "connector_id": "github|canvas|gcal|...",
    }
}
```

This ensures transparency: users can inspect which events were deduplicated and why.

## Performance Notes

- **In-memory dedup:** O(1) lookup, minimal overhead
- **SQLite dedup:** O(log N) lookup + bloom-filter first-pass, ~1–5ms per event
- **Webhook throughput:** Not affected (dedup happens post-validation)
- **Polling throughput:** Minimal impact (<5% overhead per sync)

## Related

- **Specification:** `docs/FUNCTIONAL_REQUIREMENTS.md` → FR-EVT-DEDUP-001
- **Implementation:** `crates/focus-sync/src/dedup_event_sink.rs`, `crates/focus-events/src/dedup.rs`
- **Tests:** `crates/focus-sync/src/lib.rs` (orchestrator integration tests)
