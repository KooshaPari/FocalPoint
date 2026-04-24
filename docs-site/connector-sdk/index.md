---
title: Connector SDK
description: Build custom connectors to feed external data into FocalPoint rules.
---

# Connector SDK

A **connector** is a plugin that brings external data into FocalPoint's rule engine. Connectors emit events (deadlines, schedule changes, health metrics) that trigger rules and feed the decision ledger.

## Architecture

Connectors run as **WASM modules** loaded by the Rust core. This ensures:

- **Isolation**: A buggy connector can't crash the core
- **Security**: Connectors run in a sandboxed environment with explicit capabilities
- **Portability**: Same connector code works on iOS (via UniFFI) and Android (via JNI)

## Getting Started

1. **[Manifest Format](./manifest)** — Define your connector's metadata and capabilities
2. **[Event Schema](./events)** — Declare the event types your connector emits
3. **[Auth Flows](./auth)** — Implement OAuth 2.0 or token-based authentication
4. **[Testing](./testing)** — Write tests and validate your connector

## SDK Tools

### WASM Compilation

```bash
# Requires wasm32-unknown-unknown target
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

### Manifest Validation

```bash
focalpoint connector validate-manifest connector.toml
```

### Local Testing

```bash
focalpoint connector test --manifest connector.toml --events test-events.json
```

## Example Connector (Canvas LMS)

See the Canvas connector source in `crates/connector-canvas/` for a complete example.

## Publishing

Once your connector is ready, publish to the [Connector Marketplace](../ecosystem/) for community use.
