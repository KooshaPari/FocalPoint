# API Documentation

## Rustdoc (Rust Library Crates)

Generated rustdoc for all public APIs is available at:
- **Local build**: `cargo doc --workspace --no-deps --open`
- **GitHub Pages**: Published on merge to main (via `.github/workflows/cargo-doc.yml`)

## Core Crates

| Crate | Purpose |
|-------|---------|
| `focus-domain` | Canonical domain entities, IDs, invariants, pure types |
| `focus-ir` | Intermediate representation for policy/rule compilation |
| `focus-rules` | Rule authoring, evaluation, condition DSL |
| `focus-events` | Event normalization, source mapping, schema |
| `focus-policy` | Policy engine, enforcement decisions |
| `focus-wallet` | Reward/penalty accumulation, budget tracking |
| `focus-storage` | SQLite persistence layer |
| `focus-sync` | Connector polling scheduler, cursor, dedupe |
| `focus-rituals` | Cadence scheduling (daily, weekly, monthly, custom) |
| `focus-connectors` | Connector trait, manifest, error types |
| `focus-crypto` | Secure secret storage (Apple Keychain, Linux Secret Service) |
| `focus-backup` | Full-system backup/restore with encryption |
| `focus-ffi` | UniFFI bindings for iOS/Android |

## Connector Crates

| Crate | Integration |
|-------|-----------|
| `connector-canvas` | Canvas LMS (OAuth2, REST client) |
| `connector-github` | GitHub (auth, event polling) |
| `connector-gcal` | Google Calendar (OAuth2, calendar events) |

## Binary

| Binary | Purpose |
|--------|---------|
| `focus-mcp-server` | MCP server exposing rule/task/wallet/audit surface |

## Documentation Quality

All public items have doc comments. The workspace follows rustdoc standards with:
- Module-level `//!` documentation
- Public item `///` documentation
- Intra-doc links (`[`Type`]`, `[`function()`]`)
- Traces to functional requirements (e.g., `FR-RULE-001`)

## Building Locally

```bash
# Generate full docs with private items (for development)
cargo doc --workspace --no-deps --document-private-items

# Open in browser
cargo doc --workspace --no-deps --open
```

## CI/CD

Documentation is built and published on every push to `main` via GitHub Actions.
See `.github/workflows/cargo-doc.yml` for the workflow.
