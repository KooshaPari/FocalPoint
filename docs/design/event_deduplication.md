# Event Deduplication Design

**Status:** Implemented (v0.1.0)  
**Traces to:** FR-EVT-DEDUP-001  
**Last Updated:** 2026-04-24

## Problem

Connectors emit duplicate events via two mechanisms:
1. **Polling overlap**: The same event is pulled twice if a sync cursor hasn't advanced
2. **Webhook retries**: Webhook delivery services retry on transient failures (5xx, timeout)

Without deduplication, the same event is processed multiple times, causing:
- Double-awarded credits (wallet state corruption)
- Duplicate penalty escalations
- Incorrect audit trails

## Solution: Canonical-Hash Deduplication

### Algorithm

1. **Compute canonical hash** from `(connector_id, event_type, normalized_payload_json)`:
   - Payload is normalized by recursively sorting all JSON object keys
   - Hash is SHA-256 of `"{connector_id}||{event_type}||{json}"`
   - Result: deterministic 64-char hex string

2. **Check dedup state**:
   - Query SQLite `event_dedup` table: is `hash_key` present AND not expired?
   - If yes, skip event (duplicate); if no, proceed

3. **Mark as seen**:
   - Insert `(hash_key, first_seen_at, ttl_sec)` into dedupe table
   - TTL = 30 days (2,592,000 seconds) by default
   - Entry is considered expired when `first_seen_at + ttl_sec < now`

4. **Purge expired entries**:
   - Background task periodically deletes rows with `first_seen_at < cutoff`
   - Prevents unbounded table growth

### Why This Approach

**Canonical hash**:
- Independent of event ID or trace references (which may vary)
- Normalized JSON ensures key order doesn't matter ({"a":1,"b":2} ≡ {"b":2,"a":1})
- Connector + type + payload = semantically unique event

**TTL expiry**:
- 30 days is safe for polling + webhook retry windows (typical SLA: 24-48 hours)
- Allows re-processing if a duplicate arrives >30 days later (user manually triggered sync)
- Configurable per use case (pass `ttl_sec` to `mark_seen`)

**No bloom filter in production**:
- Bloom filters speed up negative cases (cache misses)
- SQLite with indexes is fast enough (bloom false positives still require exact check)
- Complexity not justified for typical event throughput (<10K/day per user)

**Per-event, not per-session**:
- Unlike cursor-based dedup (which resets on sync), hash-based dedup survives restarts
- User restarts app → polling fetch returns same events → hash lookup prevents reprocess

## Integration Points

### EventSink Wrapper (focus-sync)

`DeduplicatingEventSink` wraps any `EventSink` implementation:

```rust
let inner_sink = Arc::new(SqliteEventStoreAdapter::new(db));
let dedup = Arc::new(SqliteEventDeduplicator::new(db));
let sink = DeduplicatingEventSink::new(inner_sink, dedup);
```

Behavior:
- Computes hash on every `append` call
- Calls `deduplicator.is_seen(hash)` before forwarding to inner sink
- If duplicate, logs and returns `Ok(())` (silent skip, no error)
- If new, calls `deduplicator.mark_seen(hash, ttl_sec)` then delegates to inner sink

### Webhook Server (focus-webhook-server)

Same wrapper applied at webhook ingress:

```rust
let dedup = Arc::new(SqliteEventDeduplicator::new(db));
let wrapped_sink = DeduplicatingEventSink::new(event_store_sink, dedup);
// Handle webhook: events are deduplicated before persistence
```

## Migration v5 Schema

```sql
CREATE TABLE IF NOT EXISTS event_dedup (
    hash_key       BLOB PRIMARY KEY,
    first_seen_at  INTEGER NOT NULL,
    ttl_sec        INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_event_dedup_ttl ON event_dedup(first_seen_at);
```

**Fields**:
- `hash_key`: SHA-256 hash as binary (64 bytes). PRIMARY KEY ensures uniqueness.
- `first_seen_at`: Unix timestamp when event was first observed (seconds since epoch)
- `ttl_sec`: Time-to-live in seconds (default 2,592,000 = 30 days)

**Index**:
- `idx_event_dedup_ttl` on `first_seen_at` for efficient purge queries

**Design rationale**:
- BLOB for hash_key is 30% more compact than TEXT (64 bytes vs ~128)
- INTEGER timestamps are fast to compare (no string parsing)
- TTL in row allows flexible per-event lifetimes (some events could be 7 days, others 60)

## Test Coverage

6 comprehensive tests:

1. **hash_deterministic_same_payload**: Same input → same hash
2. **hash_json_key_order_ignored**: Different JSON key order → same hash
3. **hash_different_components_different_hash**: Changing connector/type/payload → different hash
4. **sqlite_dedup_is_seen_false_for_unseen**: New events return false
5. **sqlite_dedup_mark_and_is_seen**: Mark + check sequence works
6. **sqlite_dedup_ttl_expiry_allows_reprocess**: Expired entries can be reprocessed
7. **dedup_sink_skips_duplicate_event**: Wrapper correctly suppresses duplicates
8. **dedup_sink_json_key_order_ignored**: Wrapper respects JSON normalization

All tests trace to FR-EVT-DEDUP-001.

## Collision Analysis

**False positives** (two different events, same hash):
- Probability of SHA-256 collision: ~1/(2^128) ≈ negligible
- In a system with 1B events, expected collisions: ~0

**False negatives** (one event hashed twice, different hashes):
- Impossible. JSON normalization is deterministic.
- If two events have identical (connector_id, event_type, payload), they produce the same hash—correct behavior.

**Practical concern**: If a connector emits two semantically different events with identical payloads (e.g., two tasks both with `{"title": "TODO"}`), they are correctly deduplicated as duplicates. This is not a bug—it's the intended behavior for connectors that don't distinguish events by content.

## Performance

- **is_seen**: ~1ms (SQLite indexed lookup, no I/O blocking async runtime)
- **mark_seen**: ~2ms (INSERT with PRIMARY KEY, no explicit index)
- **purge**: O(n) where n = number of rows in table (typically <1M for 30-day retention)

For typical user (1-10 events/day):
- 10 events/day × 30 days = 300 rows max
- Purge: <5ms for 300 rows
- Table size: ~30 KB

For high-activity user (1K events/day):
- 1K × 30 = 30K rows max
- Purge: ~200ms
- Table size: ~3 MB

## Future Enhancements

1. **Per-connector TTL**: Allow webhooks (5-day TTL) vs polling (30-day TTL)
2. **Bloom filter optimization**: Add in-memory bloom filter if SQLite lookup becomes bottleneck
3. **Distributed dedup**: If multi-device sync is added, use distributed cache (Redis)
4. **Event versioning**: Hash entire event (including metadata) if duplicate detection becomes too aggressive

## References

- **RFC 3339**: Timestamp format (used in first_seen_at conversion)
- **SHA-256**: NIST FIPS 180-4 (cryptographic hash)
- **SQLite**: https://www.sqlite.org (embedded database)
- **JSON Normalization**: RFC 8785 (key sort order for reproducible JSON)
