# FocalPoint — Architecture Decision Record Index

**Spec ID:** FocalPoint-ADR | **Status:** CURRENT | **Last Updated:** 2026-04-25

Individual ADR files live under `docs/adr/`. Source: arch doc lines 649–827.

| ADR | Decision | Status |
|---|---|---|
| [ADR-001](docs/adr/ADR-001-native-clients.md) | Native iOS (SwiftUI) + Android (Compose); no Flutter/RN/Tauri for enforcement clients | Accepted |
| [ADR-002](docs/adr/ADR-002-rust-core.md) | Rust core via UniFFI/JNI for portable domain logic | Accepted |
| [ADR-003](docs/adr/ADR-003-connector-contract.md) | Narrow Connector trait: manifest + health + sync(cursor) | Accepted |
| [ADR-004](docs/adr/ADR-004-local-first.md) | Local-first SQLite; sync services are optional backend | Accepted |
| [ADR-005](docs/adr/ADR-005-dual-ledger.md) | Separate reward + penalty ledgers (not a single score) | Accepted |
| [ADR-006](docs/adr/ADR-006-rule-dsl.md) | Explicit rule DSL with explanation templates; no implicit ML | Accepted |
| [ADR-007](docs/adr/ADR-007-audit-chain.md) | SHA-256 hash-chained audit log for tamper evidence | Accepted |
| [ADR-008](docs/adr/ADR-008-platform-apis.md) | iOS: FamilyControls/DeviceActivity/ManagedSettings. Android: UsageStats + AccessibilityService | Accepted |
| [ADR-009](docs/adr/ADR-009-unlock-proofs.md) | Unlock proofs (QR/NFC) as a pluggable adapter; NFC optional | Accepted |

Each ADR file has: Context, Decision, Alternatives, Consequences. Full prose
lives in the source arch doc and can be carved in as they solidify.
