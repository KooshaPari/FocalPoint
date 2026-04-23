# Architecture overview

FocalPoint is a Rust core + iOS shell. The core owns all domain logic (connectors, events, rules, ledger, audit). The iOS app is a SwiftUI + FamilyControls adapter that calls into the core over UniFFI.

## Layers

```mermaid
flowchart TB
  subgraph app["iOS app (apps/ios)"]
    ui[SwiftUI views]
    fc[FamilyControls / ManagedSettings / DeviceActivity]
    mascot[Coachy — Spline runtime]
  end

  subgraph ffi["UniFFI bindings (crates/focus-ffi)"]
    bindings[Generated Swift bindings]
  end

  subgraph core["Rust core (crates/focus-*)"]
    conn[focus-connectors<br/>runtime + manifest loader]
    evt[focus-events<br/>append-only store]
    rul[focus-rules<br/>DSL + engine]
    rew[focus-rewards]
    pen[focus-penalties]
    pol[focus-policy<br/>decision engine]
    aud[focus-audit<br/>SHA-256 hash chain]
    sto[focus-storage<br/>SQLite]
    cry[focus-crypto<br/>keychain + signing]
    tim[focus-time<br/>clock port + timezones]
    syn[focus-sync<br/>cursor + delta]
    ccan[connector-canvas]
  end

  subgraph ext["External (no direct coupling)"]
    canvas[Canvas LMS]
    cal[Calendar APIs]
    health[Apple Health]
    tasks[Task apps]
  end

  ui --> bindings
  bindings --> pol
  bindings --> conn
  bindings --> rul
  bindings --> mascot

  ccan --> conn
  canvas --> ccan

  conn --> evt
  evt --> rul
  rul --> pol
  pol --> rew
  pol --> pen
  pol --> aud
  rew --> aud
  pen --> aud

  pol --> fc
  pol --> mascot

  evt --> sto
  aud --> sto
  rew --> sto
  pen --> sto
  sto --> cry
```

## Read more

- [System diagram](/architecture/system-diagram) — deeper view with data flow annotations.
- [Connector framework](/architecture/connector-framework) — trait surface, manifest format, lifecycle.
- [FFI topology](/architecture/ffi-topology) — UniFFI boundaries, ownership, threading model.
- [ADRs](/architecture/adrs) — accepted architecture decisions.

## Design invariants

1. **Core is platform-free.** `crates/focus-*` must build on Linux (CI enforces).
2. **Platform glue owns no state.** Swift and (future) Kotlin hold view state only. Rule state, wallet balances, audit records live in SQLite via `focus-storage`.
3. **All mutation produces an audit record.** Any code path that changes reward balance, penalty state, or policy decision appends an `AuditRecord`.
4. **Traits are stable.** `Connector`, `EventStore`, `RuleStore`, `WalletStore`, `PenaltyStore`, `ClockPort`, `SecureSecretStore` are the public contracts. Changes require an ADR.
5. **Fail loudly.** No silent fallbacks; every error surfaces in the UI with a specific actionable message.
