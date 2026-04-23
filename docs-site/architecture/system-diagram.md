# System diagram

End-to-end view of a single rule-fire cycle, from external event to audited block.

```mermaid
sequenceDiagram
  autonumber
  participant Ext as External system<br/>(Canvas)
  participant Conn as connector-canvas
  participant Runtime as focus-connectors<br/>runtime
  participant Evt as focus-events<br/>(append-only)
  participant Rules as focus-rules<br/>engine
  participant Policy as focus-policy<br/>decision
  participant Ledger as focus-rewards /<br/>focus-penalties
  participant Audit as focus-audit<br/>hash chain
  participant Store as focus-storage<br/>(SQLite)
  participant Swift as iOS app<br/>(UniFFI)
  participant FC as FamilyControls /<br/>ManagedSettings
  participant Coachy as Coachy mascot

  Ext->>Conn: HTTP GET /assignments (polled)
  Conn->>Runtime: yield Events [assignment.upcoming]
  Runtime->>Evt: append(event, cursor)
  Evt->>Store: INSERT INTO events
  Evt-->>Rules: notify(new events)
  Rules->>Rules: match rules vs event window
  Rules->>Policy: fire(rule_id, event, targets)
  Policy->>Ledger: apply(reward? penalty?)
  Policy->>Audit: append(decision record)
  Audit->>Store: INSERT INTO audit (hash = sha256(prev||record))
  Policy-->>Swift: notify(decision)
  Swift->>FC: ManagedSettings.shield([bundle_ids])
  Swift->>Coachy: transition(state = "locked")
  Coachy->>Swift: render(line = "Instagram is locked. Canvas says PSYC 101 is due in 3h.")
```

## Data flows

### Inbound (external → core)

1. Connector runtime polls (or receives webhook, where supported).
2. Each raw payload is normalized into one or more `Event` values.
3. Events are appended to `focus-events` with a monotonically increasing cursor.
4. Rules engine subscribes to the event stream; each new event may match one or more rules.

### Outbound (core → platform)

1. Rule match → `Policy` produces a `Decision` (lock-apps, unlock-apps, reward, penalty, notify).
2. `Decision` appends an `AuditRecord` before any side effect.
3. The platform adapter (Swift) observes `Decision` via a UniFFI callback and actuates: `ManagedSettings.store.shield.applications = ...`.
4. `DeviceActivityMonitor` callbacks in Swift relay usage attempts back as `Event`s (for penalty escalation).

### Audit chain verification

At launch, the iOS app calls into `focus-audit::verify_chain_from_genesis`. The function walks every `AuditRecord` in order and recomputes `sha256(prev_hash || record_bytes)`. On mismatch it returns `ChainBroken { at_index, expected, actual }` and the app refuses to operate until the user resolves the break (restore from backup, or accept reset with evidence).

## Threading model

- The core exposes a single `Core` handle. All public methods are `Send + Sync`.
- Under the hood the core uses a single-threaded runtime (`tokio::runtime::Builder::new_current_thread`) for the connector poll loop and a lock-free `crossbeam` channel for event delivery.
- Swift calls UniFFI-bound methods from the main thread; long-running operations (connector sync, full-chain verify) are dispatched to `DispatchQueue.global(qos: .utility)` by the Swift adapter.
