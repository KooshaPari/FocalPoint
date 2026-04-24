# API Documentation

## Rustdoc

FocalPoint provides comprehensive rustdoc for all crates in the Rust workspace. Browse the API documentation:

- **Local**: Build with `cargo doc --workspace --no-deps --open` to view generated docs in your browser
- **Online**: [Published docs on gh-pages](https://kooshapari.github.io/FocalPoint/focus_ir/) (when available)

## Key Crates

### Core Domain

- **`focus-ir`** — Intermediate Representation (IR). Single canonical JSON format for all FocalPoint documents (Rule, Connector, Template, Task, Schedule, etc.). Content-addressed via SHA-256.
- **`focus-domain`** — Canonical domain entities, aggregate roots, and invariants. Pure types with no persistence or I/O.

### Storage & Persistence

- **`focus-storage`** — Storage abstraction ports and SQLite implementations. Defines trait interfaces for `EventStore`, `RuleStore`, `WalletStore`, `TaskStore`, and `PenaltyStore`.
- **`focus-audit`** — Tamper-evident audit log with SHA-256 hash chains. Provides read-only verification and integrity proofs.

### Features

- **`focus-rewards`** — Reward wallet and mutation domain logic
- **`focus-penalties`** — Penalty application and state management
- **`focus-rules`** — Rule evaluation engine
- **`focus-planning`** — Task planning and scheduling
- **`focus-events`** — Normalized event types and event sourcing primitives

### Connectors

- **`connector-github`**, **`connector-gcal`**, **`connector-notion`**, **`connector-readwise`**, **`connector-canvas`** — External service adapters

### Supporting

- **`focus-ffi`** — FFI bindings for iOS (Swift) and Android (Kotlin) consumption
- **`focus-mcp-server`** — Model Context Protocol server implementation

## Building Locally

```bash
# Generate docs with private items
cargo doc --workspace --no-deps --document-private-items

# Open in browser
cargo doc --workspace --no-deps --open
```

## Rustdoc Quality

All public API surfaces have rustdoc with field/parameter descriptions. Key crates enforce `#![deny(missing_docs)]` for coverage guarantees:

- ✅ `focus-ir`
- ✅ `focus-domain`
- ✅ `focus-storage`

See [rustdoc quality backlog](../rustdoc_backlog.md) for remaining work.
