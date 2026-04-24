# Crates Map: FocalPoint at a Glance

FocalPoint comprises 45 crates across 6 categories. This table maps each crate to its role in the system.

## Core Domain (5 crates)

The immutable heart: types, events, IR, and audit chain.

| Crate | Purpose | Key Types | Stability |
|-------|---------|-----------|-----------|
| **focus-ir** | Intermediate Representation of rules | `Rule`, `Condition`, `Action`, `IR` | Stable (trait boundary) |
| **focus-domain** | Core domain types | `User`, `Device`, `Session`, `FocusSession` | Stable |
| **focus-events** | Event types for all transitions | `Event`, `AppOpened`, `RuleMatched`, `CreditsEarned` | Stable |
| **focus-audit** | Append-only SHA-256 chain | `AuditRecord`, `AuditChain` | Stable |
| **focus-crypto** | Signing & encryption | `Ed25519Signer`, `AES_GCM`, `Sha256Hash` | Stable |

## Orchestration (6 crates)

The evaluation pipeline: from events to actions.

| Crate | Purpose | Key Types | Stability |
|-------|---------|-----------|-----------|
| **focus-eval** | Rule engine (event ↔ rule matching) | `RuleEvaluator`, `Match` | Core (no breaking changes) |
| **focus-rules** | Rule store + compile-time validation | `RuleStore`, `RuleId` | Core (no breaking changes) |
| **focus-rituals** | Habit sequences (setup, warmup, cool-down) | `Ritual`, `RitualStep` | Experimental |
| **focus-scheduler** | Task scheduling & timers | `Scheduler`, `ScheduledTask` | Core |
| **focus-planning** | Schedule creation (sessions, breaks) | `Schedule`, `ScheduledSession` | Experimental |
| **focus-calendar** | Time-block awareness & busy hours | `Calendar`, `TimeBlock`, `FocusWindow` | Experimental |

## State Management (6 crates)

Ledgers, policies, and persistence.

| Crate | Purpose | Key Types | Stability |
|-------|---------|-----------|-----------|
| **focus-storage** | SQLite adapter (schema, queries, migrations) | `StorageBackend`, `Transaction` | Core (breaking changes via ADR) |
| **focus-sync-store** | In-memory cache layer on storage | `SyncStore`, `LocalCache` | Core |
| **focus-rewards** | Wallet ledger (credits, redemptions) | `Wallet`, `LedgerEntry`, `Redemption` | Core |
| **focus-penalties** | Penalty ledger & rate-limiting | `PenaltyLedger`, `Penalty`, `RateLimit` | Experimental |
| **focus-policy** | Policy engine (entitlements, enforcement) | `Policy`, `PolicyRule`, `PolicyStore` | Core |
| **focus-entitlements** | Permission model | `Permission`, `EntitlementSet`, `Role` | Experimental |

## Connectors (11 crates)

Data ingestion from external services. All implement the `Connector` trait.

| Crate | Service | Status | Auth Type |
|-------|---------|--------|-----------|
| **focus-connectors** | Trait + registry | Core | — |
| **connector-github** | GitHub (PRs, commits, issues) | Stable | OAuth2 |
| **connector-canvas** | Canvas LMS | Stable | LTI 1.3 |
| **connector-gcal** | Google Calendar | Experimental | OAuth2 |
| **connector-strava** | Strava (workouts) | Experimental | OAuth2 |
| **connector-fitbit** | Fitbit (health) | Experimental | OAuth2 |
| **connector-notion** | Notion (databases) | Experimental | Notion API |
| **connector-readwise** | Readwise (highlights) | Experimental | API key |
| **connector-linear** | Linear (issues) | Experimental | API key |
| **connector-testkit** | Mock connector (testing) | Test-only | — |
| *(future)* | Slack, Jira, Asana | Planned | OAuth2 |

## Interfaces & Surfaces (5 crates)

Native bindings, servers, and user entry points.

| Crate | Purpose | Entry Point | Target |
|-------|---------|-------------|--------|
| **focus-ffi** | UniFFI bindings | Swift/Kotlin auto-generated | iOS/Android |
| **focus-cli** | Command-line interface | `focus` binary | Terminal |
| **focus-mcp-server** | Claude MCP integration | MCP protocol | Claude/AI agents |
| **focus-webhook-server** | HTTP webhook receiver | `:8080/webhooks/*` | GitHub, Canvas, etc. |
| **focus-sync** | Polling & webhook orchestrator | Background daemon | All connectors |

## Language & Templating (4 crates)

DSL, compilation, and template management.

| Crate | Purpose | Stability | Note |
|--------|---------|-----------|------|
| **focus-lang** | FPL parser (text DSL → IR) | Experimental | See `/rules/dsl` |
| **focus-transpilers** | IR → platform formats | Experimental | Targets: Swift DSL, GraphQL query |
| **focus-templates** | Template pack manager (TOML + IR) | Experimental | Bundled rule collections |
| **focus-backup** | Full export & integrity checks | Experimental | Encrypted wallet backup |

## Packaging & Tooling (9 crates + 5 tools)

Operational crates and CI/build tools.

### Operational Crates

| Crate | Purpose | Note |
|-------|---------|------|
| **focus-time** | Clock abstraction (testable time) | Used in tests, scheduler |
| **focus-coaching** | Nudge generation & feedback | AI-generated encouragement |
| **focus-mascot** | Coachy persona & dialogue | Character responses |
| **focus-always-on** | Background app lifecycle | iOS NotificationCenter listener |
| **focus-demo-seed** | Fixture data for demos | Pre-seeded users, rules, events |
| **focus-icon-gen** | Icon generation for templates | SVG → PNG for app icons |
| **focus-asset-fetcher** | Download connector logos | Caches remote assets |
| **focus-ci-watcher** | CI event ingestion | Watches GitHub Actions |
| **focus-telemetry** | Anonymous metrics (opt-in) | Aggregates usage data |
| **focus-rule-suggester** | AI rule recommendations | Claude-powered suggestions |

### Build & CI Tools (in `tooling/`)

| Tool | Purpose |
|------|---------|
| **bench-guard** | Benchmark regression detection |
| **quality-gate** | Pre-commit lint, type-check, test gate |
| **doc-link-check** | Markdown link validation |
| **sbom-gen** | Software Bill of Materials generation |
| **commit-msg-check** | DCO + conventional commit validation |
| **fr-coverage** | FR traceability report generator |

## Services (2 crates)

Optional local or remote services.

| Service | Purpose | Protocol | Optional? |
|---------|---------|----------|-----------|
| **templates-registry** | Template pack catalog | HTTP REST | Yes |
| *(future)* | Sync service | WebSocket + HTTP | Yes |

## Dependencies at a Glance

**Async Runtime:** `tokio` (full feature set)  
**Serialization:** `serde` + `serde_json`  
**HTTP:** `reqwest` + `oauth2`  
**Database:** `rusqlite` (bundled SQLite)  
**Crypto:** `ring` + `sha2` + `secrecy`  
**CLI:** `clap` (derive macros)  
**Logging:** `tracing` + `tracing-subscriber`  
**Testing:** `proptest`, `mockall`

## Crate Stability Levels

- **Stable**: Public trait boundaries are frozen. Breaking changes require ADR.
- **Core**: High-priority crates (storage, eval, rewards). Breaking changes via ADR.
- **Experimental**: Early-stage, breaking changes acceptable until v0.1.0.
- **Test-only**: Internal use only, no stability guarantees.

## Entry Points for Feature Work

| Feature | Start Here | Then | Finally |
|---------|-----------|------|---------|
| Add a new connector | `connector-*` template | `focus-connectors` registry | `focus-sync` ingestion |
| Create a rule DSL | `focus-lang` parser | `focus-ir` IR | `focus-rules` store |
| Build a mobile surface | `focus-ffi` bindings | Platform adapter (Swift/Kotlin) | Bridge via `ClockPort` |
| Add a reward type | `focus-rewards` ledger | `focus-eval` matcher | `focus-audit` record |
| Implement a policy | `focus-policy` engine | `focus-entitlements` check | `focus-rules` enforcement |

## Summary Statistics

- **Total crates:** 45 (11 connectors, 9 operational, 5 interface, 6 state, 6 orchestration, 5 core, 5 tools, 2 services)
- **Core Rust traits:** `Connector`, `EventStore`, `RuleStore`, `WalletStore`, `PenaltyStore`, `ClockPort`, `SecureSecretStore`
- **Languages:** Rust (primary), Go (optional CLI), Swift/Kotlin (FFI bindings only)
- **Test coverage:** >80% on core crates (focus-ir, focus-eval, focus-rules, focus-rewards)

See `/architecture/` for system overview, data flow diagrams, and testing strategy.
