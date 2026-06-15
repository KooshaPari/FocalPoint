# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.12] - 2026-05-06

### Added
- Document v0.0.12 release (tag exists, CHANGELOG entry added retroactively).

## [0.0.11] - 2026-04-20

### Changed
- Dependency hygiene: `cargo-deny` advisory suppression updates, `tokio` tightened.
- CI workflow pinning for `trufflehog`, `cargo-audit`, and `scorecard`.

## [0.0.10] - 2026-04-10

### Added
- `focus-mcp-server` scaffold (MCP SDK types, transport pending).
- `focus-builder` web task editor (12 ReactFlow node types, Coachy preview).

### Fixed
- Connector test fixtures stabilized for Canvas wiremock suite.

## [0.0.9] - 2026-03-25

### Added
- Rituals crate: Morning Brief + Evening Shutdown (15 integration tests).
- Task scheduling: rigidity-aware bin-packing, working-hours constraints, chunk splitting.
- Spanish + Japanese i18n (122 strings, `Localizable.xcstrings`).

### Changed
- Error unification: introduced `focus-errors` and `focus-result` crates for consistent error handling.

## [0.0.8] - 2026-03-10

### Added
- Canvas OAuth2 shipped: `ASWebAuthenticationSession`, keychain persistence, sync heartbeat.
- Rule authoring wizard: 4-step SwiftUI flow (When/If/Then/Settings), JSON preview, DSL catalog.
- Connector manifest + tier system (Official/Verified/MCPBridged/Private).

## [0.0.7] - 2026-02-20

### Added
- FamilyControls driver scaffold: `ManagedSettingsStore` + `DeviceActivityCenter` wired behind `#if FOCALPOINT_HAS_FAMILYCONTROLS` flag.
- Webhook registry: signature verification, handler dispatch, 5 tests.
- Template pack format: ed25519 signing, TOML round-trip, deterministic UUID derivation.

### Fixed
- Audit chain hash verification: tamper-evident records verified on startup.

## [0.0.6] - 2026-02-05

### Added
- iOS app shell: SwiftUI skeleton with 5 tabs (Home, Tasks, Rules, Activity, Settings).
- Connector-canvas: OAuth2, 4 event types, 44 wiremock integration tests.
- `focus-ffi` UniFFI bindings for Rust ↔ Swift interop.
- `focus-cli` (`focus` command) for exploration and automation.

### Changed
- Android placeholder deferred beyond Phase 2.

## [0.0.5] - 2026-01-15

### Added
- Core domain layer end-to-end: event sourcing, rules engine (12 conditions, 6 actions), reward/penalty ledgers, audit chain (SHA256-chained, tamper-evident).
- Workspace setup: 17+ crates, `cargo build --workspace`, `cargo test --workspace` baseline.
- Go services placeholder (`services/`).

### Fixed
- ADR-001: Cross-native frameworks (Tauri / RN / Flutter) rejected in favor of native iOS + Android.
