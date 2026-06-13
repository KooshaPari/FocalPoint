# FocalPoint Workspace Dependency Snapshot (W1-08)

Generated: 2026-06-13

## Table of Contents

1. [Root Workspace (`Cargo.toml`)](#root-workspace)
2. [Tooling Crates](#tooling-crates)
3. [Core Crates](#core-crates)
4. [Connector Crates](#connector-crates)
5. [Test & Example Crates](#test--example-crates)

---

## Root Workspace

### `FocalPoint/Cargo.toml`

#### `[workspace.dependencies]`

| Crate | Version |
|-------|---------|
| serde | `1.0` (features: ["derive"]) |
| serde_json | `1.0` |
| thiserror | `2.0` |
| anyhow | `1.0` |
| uuid | `1.11` (features: ["v4", "serde"]) |
| chrono | `0.4` (features: ["serde"]) |
| tokio | `1.39` (features: ["full"]) |
| async-trait | `0.1` |
| futures | `0.3` |
| reqwest | `0.12` (features: ["json", "rustls-tls"], default-features: false) |
| oauth2 | `5.0` |
| url | `2.5` |
| rusqlite | `0.33` (features: ["bundled"]) |
| sha2 | `0.10` |
| ring | `0.17` |
| secrecy | `0.10` |
| tracing | `0.1` |
| tracing-subscriber | `0.3` (features: ["env-filter"]) |
| parking_lot | `0.12` |
| rayon | `1.10` |
| clap | `4.5` (features: ["derive", "env"]) |
| async-graphql | `>=0.13` (features: ["chrono", "uuid"]) |
| async-graphql-axum | `>=0.13` |
| axum | `0.8` |
| tower | `0.4` |
| tower-http | `0.5` (features: ["trace"]) |
| uniffi | `0.28` |
| toml | `0.8` |
| ed25519-dalek | `2.1` (features: ["rand_core", "std"]) |
| rand_core | `0.6` |
| mcp-sdk | `0.0.3` |
| dirs | `5.0` |
| criterion | `0.5` (default-features: false) |
| tar | `0.4` |
| hex | `0.4` |
| zstd | `0.13` |
| csv | `1.3` |
| serde_yaml | `0.9` |
| focus-errors | path: `crates/focus-errors` |
| focus-result | path: `crates/focus-result` |
| focus-serde | path: `crates/focus-serde` |
| focus-connectors | path: `crates/focus-connectors` |
| focus-events | path: `crates/focus-events` |
| focus-ir | path: `crates/focus-ir` |
| focus-lang | path: `crates/focus-lang` |
| focus-rules | path: `crates/focus-rules` |
| focus-rewards | path: `crates/focus-rewards` |
| focus-penalties | path: `crates/focus-penalties` |
| focus-policy | path: `crates/focus-policy` |
| focus-storage | path: `crates/focus-storage` |
| focus-audit | path: `crates/focus-audit` |
| focus-time | path: `crates/focus-time` |
| focus-domain | path: `crates/focus-domain` |
| focus-demo-seed | path: `crates/focus-demo-seed` |
| focus-observability | path: `crates/focus-observability` |
| focus-telemetry | path: `crates/focus-telemetry` |

---

## Tooling Crates

### `tooling/agent-orchestrator/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| anyhow | `1.0` |
| clap | `4.5` (features: ["derive", "env"]) |
| serde | `1.0` (features: ["derive"]) |
| serde_json | `1.0` |
| toml | `0.8` |
| thiserror | `2.0` |
| tracing | `0.1` |
| tracing-subscriber | `0.3` (features: ["env-filter"]) |
| chrono | `0.4` (features: ["serde"]) |
| uuid | `1.11` (features: ["v4", "serde"]) |
| regex | `1.10` |
| walkdir | `2.4` |
| glob | `0.3` |

---

### `tooling/bench-guard/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| anyhow | workspace |
| thiserror | workspace |
| clap | workspace (features: ["derive"]) |

---

### `tooling/commit-msg-check/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| clap | `4.5` (features: ["derive"]) |
| regex | `1.11` |
| anyhow | `1.0` |

---

### `tooling/disk-check/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| anyhow | `1.0` |
| clap | `4.5` (features: ["derive"]) |

---

### `tooling/doc-link-check/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| walkdir | `2` |
| pulldown-cmark | `0.9` |
| anyhow | `1` |
| thiserror | `1` |

---

### `tooling/fr-coverage/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| regex | `1` |
| walkdir | `2` |
| anyhow | `1` |

---

### `tooling/quality-gate/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| anyhow | `1` |
| serde_json | `1` |
| walkdir | `2` |
| regex | `1` |

---

### `tooling/release-cut/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| clap | workspace (features: ["derive"]) |
| anyhow | workspace |
| thiserror | workspace |
| serde | workspace (features: ["derive"]) |
| serde_json | workspace |
| toml | workspace |
| chrono | workspace (features: ["serde"]) |
| reqwest | workspace (features: ["blocking"]) |
| semver | `1.0` |
| regex | `1.10` |
| walkdir | `2.4` |
| focus-release-bot | path: `../../crates/focus-release-bot` |

---

### `tooling/sbom-gen/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| cargo_metadata | `0.18` |
| serde_json | `1.0` |
| anyhow | `1.0` |
| chrono | `0.4` |

---

### `tooling/target-pruner/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| clap | `4.5` (features: ["derive"]) |
| anyhow | `1.0` |
| serde | `1.0` (features: ["derive"]) |
| toml | `0.8` |
| walkdir | `2.4` |
| chrono | `0.4` |
| human-panic | `1.2` |
| tracing | `0.1` |
| tracing-subscriber | `0.3` (features: ["env-filter"]) |


---

## Core Crates

### `crates/focus-always-on/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| chrono | workspace |
| tokio | workspace (features: ["sync"]) |
| async-trait | workspace |
| tracing | workspace |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |
| focus-events | path: `../focus-events` |
| focus-domain | path: `../focus-domain` |

---

### `crates/focus-audit/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| chrono | workspace |
| uuid | workspace |
| sha2 | workspace |
| hex | `0.4` |
| focus-observability | path: `../focus-observability` |
| tracing | workspace |

---

### `crates/focus-backup/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| chrono | workspace |
| uuid | workspace |
| sha2 | workspace |
| tokio | workspace |
| async-trait | workspace |
| age | `0.11` |
| zstd | `0.13` |
| tar | workspace |
| hex | workspace |
| focus-storage | path: `../focus-storage` |
| focus-audit | path: `../focus-audit` |
| focus-events | path: `../focus-events` |
| focus-rules | path: `../focus-rules` |
| focus-rewards | path: `../focus-rewards` |
| focus-penalties | path: `../focus-penalties` |
| focus-planning | path: `../focus-planning` |
| focus-templates | path: `../focus-templates` |

---

### `crates/focus-cli/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| clap | workspace |
| anyhow | workspace |
| chrono | workspace |
| serde | workspace |
| serde_json | workspace |
| toml | workspace |
| tokio | workspace (features: ["macros", "rt-multi-thread"]) |
| uuid | workspace |
| dirs | workspace |
| focus-storage | path: `../focus-storage` |
| focus-audit | path: `../focus-audit` |
| focus-planning | path: `../focus-planning` |
| focus-templates | path: `../focus-templates` |
| focus-rules | path: `../focus-rules` |
| focus-replay | path: `../focus-replay` |
| focus-rewards | path: `../focus-rewards` |
| focus-penalties | path: `../focus-penalties` |
| focus-domain | path: `../focus-domain` |
| focus-demo-seed | path: `../focus-demo-seed` |
| focus-lang | path: `../focus-lang` |
| focus-observability | path: `../focus-observability` |
| tracing | workspace |
| reqwest | workspace (features: ["blocking"]) |
| urlencoding | `2.1` |
| csv | workspace |
| serde_yaml | workspace |

---

### `crates/focus-coaching/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| async-trait | workspace |
| tokio | workspace (features: ["sync", "macros", "rt", "time"]) |
| reqwest | workspace |
| secrecy | workspace |
| tracing | workspace |
| chrono | workspace |

---

### `crates/focus-connectors/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-domain | path: `../focus-domain` |
| focus-events | path: `../focus-events` |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |
| serde | workspace |
| serde_json | workspace |
| async-trait | workspace |
| thiserror | workspace |
| chrono | workspace |
| uuid | workspace |
| anyhow | workspace |
| secrecy | workspace |
| hmac | `0.12` |
| sha2 | workspace |
| subtle | `2.1` |
| hex | `0.4` |
| base64 | `0.22` |
| reqwest | workspace (features: ["json"]) |
| jsonwebtoken | `10` |
| tokio | workspace (features: ["sync"]) |
| tracing | workspace |

---

### `crates/focus-connectors-mock-familycontrols/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-connectors | path: `../focus-connectors` |
| focus-events | path: `../focus-events` |
| focus-storage | path: `../focus-storage` |
| async-trait | workspace |
| chrono | workspace (features: ["serde"]) |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| uuid | workspace (features: ["v4"]) |
| tokio | workspace (features: ["time"]) |

---

### `crates/focus-crypto/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| thiserror | workspace |
| anyhow | workspace |
| ring | workspace |
| secrecy | workspace |
| uuid | workspace |
| chrono | workspace |
| security-framework | `3.0` (target: `cfg(target_vendor = "apple")`) |
| secret-service | `5.0` (features: ["rt-tokio-crypto-rust"], target: `cfg(target_os = "linux")`) |
| tokio | workspace (features: ["rt"], target: `cfg(target_os = "linux")`) |

---

### `crates/focus-domain/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| thiserror | workspace |
| uuid | workspace |
| chrono | workspace |

---

### `crates/focus-entitlements/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-errors | workspace |
| focus-result | workspace |
| serde | workspace |
| serde_json | workspace |
| chrono | workspace (features: ["serde"]) |
| anyhow | workspace |
| uuid | workspace |
| async-trait | workspace |
| tokio | workspace |
| uniffi | workspace (features: ["cli"]) |

---

### `crates/focus-errors/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| thiserror | `1` |

---

### `crates/focus-eval/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-audit | path: `../focus-audit` |
| focus-domain | path: `../focus-domain` |
| focus-events | path: `../focus-events` |
| focus-rewards | path: `../focus-rewards` |
| focus-penalties | path: `../focus-penalties` |
| focus-rules | path: `../focus-rules` |
| focus-storage | path: `../focus-storage` |
| focus-sync | path: `../focus-sync` |
| focus-observability | path: `../focus-observability` |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |
| serde | workspace |
| serde_json | workspace |
| chrono | workspace |
| uuid | workspace |
| async-trait | workspace |
| anyhow | workspace |
| tokio | workspace |
| tracing | workspace |
| rayon | workspace |
| parking_lot | workspace |

---

### `crates/focus-events/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-domain | path: `../focus-domain` |
| serde | workspace |
| serde_json | workspace |
| chrono | workspace |
| uuid | workspace |
| thiserror | workspace |
| async-trait | workspace |
| tokio | workspace |
| sha2 | workspace |
| hex | workspace |
| parking_lot | workspace |


---

### `crates/focus-ffi/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| thiserror | workspace |
| serde | workspace |
| serde_json | workspace |
| chrono | workspace |
| anyhow | workspace |
| async-trait | workspace |
| tokio | workspace (features: ["rt-multi-thread", "sync", "macros"]) |
| uuid | workspace |
| uniffi | workspace (features: ["cli"]) |
| focus-mascot | path: `../focus-mascot` |
| focus-domain | path: `../focus-domain` |
| focus-events | path: `../focus-events` |
| focus-rules | path: `../focus-rules` |
| focus-eval | path: `../focus-eval` |
| focus-rewards | path: `../focus-rewards` |
| focus-penalties | path: `../focus-penalties` |
| focus-audit | path: `../focus-audit` |
| focus-storage | path: `../focus-storage` |
| focus-policy | path: `../focus-policy` |
| focus-sync | path: `../focus-sync` |
| focus-connectors | path: `../focus-connectors` |
| focus-coaching | path: `../focus-coaching` |
| focus-rituals | path: `../focus-rituals` |
| focus-planning | path: `../focus-planning` |
| focus-scheduler | path: `../focus-scheduler` |
| focus-calendar | path: `../focus-calendar` |
| focus-crypto | path: `../focus-crypto` |
| focus-templates | path: `../focus-templates` |
| focus-always-on | path: `../focus-always-on` |
| focus-backup | path: `../focus-backup` |
| focus-demo-seed | path: `../focus-demo-seed` |
| focus-connectors-mock-familycontrols | path: `../focus-connectors-mock-familycontrols` (optional) |
| connector-canvas | path: `../connector-canvas` (features: ["keychain"]) |
| connector-gcal | path: `../connector-gcal` (features: ["keychain"]) |
| connector-github | path: `../connector-github` (features: ["keychain"]) |
| reqwest | workspace |
| secrecy | workspace |

#### `[build-dependencies]`

| Crate | Version |
|-------|---------|
| uniffi_build | `0.28` (features: ["builtin-bindgen"]) |

---

### `crates/focus-icon-gen/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| png | `0.17` |
| clap | workspace (features: ["derive"]) |
| focus-result | workspace |
| serde | workspace |
| serde_json | `1.0` |
| sha2 | workspace |
| hex | workspace |

---

### `crates/focus-ir/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| chrono | workspace |
| uuid | workspace |
| thiserror | workspace |
| sha2 | `0.10` |
| hex | `0.4` |
| focus-domain | path: `../focus-domain` |
| focus-planning | path: `../focus-planning` |
| focus-storage | path: `../focus-storage` |

---

### `crates/focus-lang/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| starlark | `0.13` |
| serde | workspace |
| serde_json | workspace |
| chrono | workspace |
| uuid | workspace |
| thiserror | workspace |
| anyhow | workspace |
| focus-ir | path: `../focus-ir` |
| csv | workspace |
| serde_yaml | workspace |

---

### `crates/focus-mascot/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| thiserror | workspace |
| chrono | workspace |
| async-trait | workspace |
| tracing | workspace |
| focus-coaching | path: `../focus-coaching` |

---

### `crates/focus-mcp-server/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-storage | path: `../focus-storage` |
| focus-planning | path: `../focus-planning` |
| focus-audit | path: `../focus-audit` |
| focus-rules | path: `../focus-rules` |
| focus-templates | path: `../focus-templates` |
| focus-rewards | path: `../focus-rewards` |
| focus-penalties | path: `../focus-penalties` |
| focus-domain | path: `../focus-domain` |
| focus-events | path: `../focus-events` |
| focus-connectors | path: `../focus-connectors` |
| focus-observability | path: `../focus-observability` |
| mcp-sdk | workspace |
| tokio | workspace (features: ["macros", "rt-multi-thread", "io-util", "sync"]) |
| serde | workspace |
| serde_json | workspace |
| anyhow | workspace |
| thiserror | workspace |
| tracing | workspace |
| tracing-subscriber | workspace |
| clap | workspace |
| uuid | workspace |
| chrono | workspace |
| async-trait | workspace |
| dirs | workspace |
| futures | workspace |
| axum | `0.7` (optional) |
| tower | `0.5` (optional) |
| tower-http | `0.6` (features: ["trace"], optional) |
| tokio-tungstenite | `0.23` (optional) |
| http | `1` (optional) |

---

### `crates/focus-observability/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| tokio | workspace (features: ["rt-multi-thread"]) |
| tracing | workspace |
| tracing-subscriber | workspace (features: ["json", "fmt", "ansi", "time"]) |
| tracing-opentelemetry | `0.24` |
| opentelemetry | `0.24` (features: ["metrics", "trace"]) |
| opentelemetry-otlp | `0.17` (features: ["trace"]) |
| prometheus | `0.14` |
| regex | `1.10` |
| parking_lot | workspace |

---

### `crates/focus-penalties/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-audit | path: `../focus-audit` |
| focus-domain | path: `../focus-domain` |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| chrono | workspace |
| uuid | workspace |

---

### `crates/focus-planning/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-domain | path: `../focus-domain` |
| anyhow | workspace |
| serde | workspace |
| thiserror | workspace |
| uuid | workspace |
| chrono | workspace |

---

### `crates/focus-policy/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-audit | path: `../focus-audit` |
| focus-domain | path: `../focus-domain` |
| focus-rules | path: `../focus-rules` |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| chrono | workspace |
| uuid | workspace |

---

### `crates/focus-replay/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-audit | path: `../focus-audit` |
| focus-domain | path: `../focus-domain` |
| focus-events | path: `../focus-events` |
| focus-rewards | path: `../focus-rewards` |
| focus-penalties | path: `../focus-penalties` |
| focus-rules | path: `../focus-rules` |
| focus-storage | path: `../focus-storage` |
| serde | workspace |
| serde_json | workspace |
| chrono | workspace |
| uuid | workspace |
| async-trait | workspace |
| anyhow | workspace |
| tokio | workspace (features: ["macros", "rt-multi-thread"]) |
| tracing | workspace |

---

### `crates/focus-result/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-errors | path: `../focus-errors` |

---

### `crates/focus-rewards/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-audit | path: `../focus-audit` |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| chrono | workspace |
| uuid | workspace |

---

### `crates/focus-rituals/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-domain | path: `../focus-domain` |
| focus-planning | path: `../focus-planning` |
| focus-scheduler | path: `../focus-scheduler` |
| focus-calendar | path: `../focus-calendar` |
| focus-coaching | path: `../focus-coaching` |
| focus-rewards | path: `../focus-rewards` |
| focus-penalties | path: `../focus-penalties` |
| focus-mascot | path: `../focus-mascot` |
| focus-events | path: `../focus-events` |
| focus-audit | path: `../focus-audit` |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |
| chrono | workspace |
| uuid | workspace |
| serde | workspace |
| serde_json | workspace |
| async-trait | workspace |
| anyhow | workspace |
| thiserror | workspace |
| tracing | workspace |
| tokio | workspace (features: ["sync"]) |


---

### `crates/focus-rule-suggester/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| async-trait | workspace |
| tokio | workspace |
| tracing | workspace |
| focus-audit | workspace |
| focus-events | workspace |
| focus-rules | workspace |
| focus-storage | workspace |
| focus-domain | workspace |

---

### `crates/focus-rules/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-domain | path: `../focus-domain` |
| focus-events | path: `../focus-events` |
| serde | workspace |
| serde_json | workspace |
| chrono | workspace |
| uuid | workspace |
| async-trait | workspace |
| anyhow | workspace |
| tracing | workspace |
| focus-coaching | path: `../focus-coaching` |
| cron | `0.16` |
| regex | `1` |

---

### `crates/focus-scheduler/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-domain | path: `../focus-domain` |
| focus-planning | path: `../focus-planning` |
| focus-calendar | path: `../focus-calendar` |
| serde | workspace |
| thiserror | workspace |
| uuid | workspace |
| chrono | workspace |
| anyhow | workspace |
| async-trait | workspace |
| tracing | workspace |

---

### `crates/focus-serde/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-errors | path: `../focus-errors` |
| focus-result | path: `../focus-result` |
| serde | `1` (features: ["derive"]) |
| serde_json | `1` |
| toml | `0.8` |
| chrono | `0.4` (features: ["serde"]) |
| thiserror | `1` |

---

### `crates/focus-storage/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-audit | path: `../focus-audit` |
| focus-domain | path: `../focus-domain` |
| focus-events | path: `../focus-events` |
| focus-rules | path: `../focus-rules` |
| focus-rewards | path: `../focus-rewards` |
| focus-penalties | path: `../focus-penalties` |
| focus-planning | path: `../focus-planning` |
| focus-sync | path: `../focus-sync` |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| async-trait | workspace |
| uuid | workspace |
| rusqlite | workspace |
| chrono | workspace |
| tokio | workspace |

---

### `crates/focus-sync/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| thiserror | workspace |
| tokio | workspace |
| async-trait | workspace |
| chrono | workspace |
| uuid | workspace |
| tracing | workspace |
| anyhow | workspace |
| focus-connectors | path: `../focus-connectors` |
| focus-events | path: `../focus-events` |
| focus-time | path: `../focus-time` |
| focus-observability | path: `../focus-observability` |

---

### `crates/focus-sync-store/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| async-trait | workspace |
| tokio | workspace (features: ["sync"]) |
| ed25519-dalek | workspace |
| tracing | workspace |

---

### `crates/focus-telemetry/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| tokio | workspace (features: ["rt-multi-thread"]) |
| async-trait | workspace |
| rusqlite | workspace |
| sha2 | workspace |
| hex | workspace |
| tracing | workspace |
| regex | `1.10` |
| reqwest | workspace |
| url | workspace |

---

### `crates/focus-templates/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-rules | path: `../focus-rules` |
| focus-domain | path: `../focus-domain` |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| chrono | workspace |
| uuid | workspace |
| toml | workspace |
| ed25519-dalek | workspace |
| rand_core | workspace |
| sha2 | workspace |

---

### `crates/focus-time/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| chrono | workspace |
| focus-result | workspace |

---

### `crates/focus-transpilers/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-ir | path: `../focus-ir` |
| focus-rules | path: `../focus-rules` |
| focus-templates | path: `../focus-templates` |
| focus-domain | path: `../focus-domain` |
| serde | workspace |
| serde_json | workspace |
| toml | workspace |
| anyhow | workspace |
| thiserror | workspace |
| uuid | workspace (features: ["serde", "v5"]) |
| chrono | workspace (features: ["serde"]) |

---

### `crates/focus-ui/Cargo.toml`

#### `[dependencies]`

*(no dependencies declared)*

---

### `crates/focus-webhook-server/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-domain | path: `../focus-domain` |
| focus-events | path: `../focus-events` |
| focus-connectors | path: `../focus-connectors` |
| focus-sync | path: `../focus-sync` |
| connector-github | path: `../connector-github` |
| focus-plugin-sdk | path: `../focus-plugin-sdk` |
| focus-observability | path: `../focus-observability` |
| serde | workspace |
| serde_json | workspace |
| async-trait | workspace |
| thiserror | workspace |
| chrono | workspace |
| uuid | workspace |
| tokio | workspace (features: ["macros", "rt", "sync"]) |
| anyhow | workspace |
| axum | `0.7` |
| bytes | `1.7` |
| tower | `0.4` |
| tower-http | `0.5` (features: ["trace"]) |
| hyper | `1.4` |
| hmac | `0.12` |
| sha2 | workspace |
| subtle | `2.1` |
| jsonwebtoken | `10` |
| reqwest | workspace (features: ["json"]) |
| base64 | `0.22` |
| hex | `0.4` |
| tracing | workspace |
| tracing-subscriber | workspace (features: ["env-filter", "fmt"]) |
| clap | workspace |
| secrecy | workspace |

---

### `crates/phenotype-error-core/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| thiserror | workspace |
| serde | workspace |
| serde_json | workspace |


---

## Connector Crates

### `crates/connector-canvas/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-connectors | path: `../focus-connectors` |
| focus-events | path: `../focus-events` |
| focus-crypto | path: `../focus-crypto` (optional) |
| secrecy | workspace (optional) |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| async-trait | workspace |
| reqwest | workspace |
| oauth2 | workspace |
| tokio | workspace |
| chrono | workspace |
| uuid | workspace |
| tracing | workspace |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |
| url | `2.5` |

---

### `crates/connector-fitbit/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-events | workspace |
| focus-connectors | workspace |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| tokio | workspace |
| reqwest | `0.12` (features: ["json"]) |
| http | `1.1` |
| urlencoding | `2.1` |
| async-trait | workspace |
| tracing | workspace |

---

### `crates/connector-gcal/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-connectors | path: `../focus-connectors` |
| focus-events | path: `../focus-events` |
| focus-crypto | path: `../focus-crypto` (optional) |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |
| secrecy | workspace (optional) |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| async-trait | workspace |
| reqwest | workspace |
| oauth2 | workspace |
| tokio | workspace |
| chrono | workspace |
| uuid | workspace |
| tracing | workspace |
| url | `2.5` |

---

### `crates/connector-github/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-connectors | path: `../focus-connectors` |
| focus-events | path: `../focus-events` |
| focus-crypto | path: `../focus-crypto` (optional) |
| secrecy | workspace |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| async-trait | workspace |
| reqwest | workspace |
| tokio | workspace |
| chrono | workspace |
| uuid | workspace |
| tracing | workspace |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |

---

### `crates/connector-linear/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-events | workspace |
| focus-connectors | workspace |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| tokio | workspace |
| reqwest | `0.12` (features: ["json"]) |
| http | `1.1` |
| async-trait | workspace |
| tracing | workspace |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |

---

### `crates/connector-notion/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-events | workspace |
| focus-connectors | workspace |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| tokio | workspace |
| reqwest | `0.12` (features: ["json"]) |
| http | `1.1` |
| async-trait | workspace |
| tracing | workspace |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |

---

### `crates/connector-readwise/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-events | workspace |
| focus-connectors | workspace |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| tokio | workspace |
| reqwest | `0.12` (features: ["json"]) |
| http | `1.1` |
| async-trait | workspace |
| tracing | workspace |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |

---

### `crates/connector-strava/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-events | workspace |
| focus-connectors | workspace |
| phenotype-observably-macros | path: `../../../PhenoObservability/crates/phenotype-observably-macros` |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| uuid | workspace |
| chrono | workspace |
| tokio | workspace |
| reqwest | `0.12` (features: ["json"]) |
| http | `1.1` |
| urlencoding | `2.1` |
| async-trait | workspace |
| tracing | workspace |

---

### `crates/connector-testkit/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-connectors | path: `../focus-connectors` |
| focus-events | path: `../focus-events` |
| serde | workspace |
| thiserror | workspace |


---

## Test & Example Crates

### `crates/focus-asset-fetcher/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| reqwest | workspace (features: ["blocking"]) |
| tokio | workspace (features: ["rt", "macros"]) |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| chrono | workspace |
| uuid | workspace (features: ["v4"]) |
| clap | `4.5` (features: ["derive"]) |
| regex | `1.11` |
| walkdir | `2.4` |
| sha2 | `0.10` |
| hex | `0.4` |
| tracing | `0.1` |
| tracing-subscriber | `0.3` (features: ["env-filter"]) |
| url | `2.5` |
| tempfile | `3.10` |

---

### `crates/focus-ci-watcher/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| anyhow | workspace |
| clap | workspace (features: ["derive"]) |
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| tokio | workspace (features: ["sync", "time", "process"]) |
| tracing | workspace |
| tracing-subscriber | workspace (features: ["env-filter"]) |
| uuid | workspace |
| chrono | workspace |
| focus-release-bot | path: `../focus-release-bot` |
| reqwest | workspace |

---

### `crates/focus-demo-seed/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| anyhow | workspace |
| chrono | workspace |
| serde | workspace |
| serde_json | workspace |
| uuid | workspace (features: ["v4"]) |
| thiserror | workspace |
| tracing | workspace |
| focus-storage | path: `../focus-storage` |
| focus-domain | path: `../focus-domain` |
| focus-audit | path: `../focus-audit` |
| focus-planning | path: `../focus-planning` |
| focus-rewards | path: `../focus-rewards` |
| focus-rules | path: `../focus-rules` |
| focus-connectors | path: `../focus-connectors` |
| focus-events | path: `../focus-events` |

---

### `crates/focus-plugin-sdk/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | workspace |
| serde_json | workspace |
| thiserror | workspace |
| anyhow | workspace |
| chrono | workspace |
| tracing | workspace |
| ed25519-dalek | workspace |
| toml | workspace |
| sha2 | workspace |
| wasmtime | `^44.0` |
| wasmtime-wasi | `^44.0` |
| reqwest | workspace |
| tokio | workspace (features: ["full"]) |

---

### `crates/focus-plugin-sdk/examples/hello-connector/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde_json | `1.0` |

---

### `crates/focus-plugin-sdk/examples/slack-reference/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| serde | `1` (features: ["derive"]) |
| serde_json | `1` |
| chrono | `0.4` (features: ["serde"]) |

---

### `crates/focus-release-bot/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| reqwest | workspace (features: ["blocking"]) |
| serde | workspace (features: ["derive"]) |
| serde_json | workspace |
| anyhow | workspace |
| thiserror | workspace |

---

### `tests/e2e/Cargo.toml`

#### `[dependencies]`

| Crate | Version |
|-------|---------|
| focus-audit | path: `../../crates/focus-audit` |
| focus-events | path: `../../crates/focus-events` |
| focus-rewards | path: `../../crates/focus-rewards` |
| focus-rules | path: `../../crates/focus-rules` |
| chrono | `0.4` (features: ["serde"]) |
| serde_json | `1.0` |
| uuid | `1.11` (features: ["v4", "serde"]) |
| anyhow | `1.0` |

---

## Summary

- **Total Cargo.toml files scanned:** 70 (1 root workspace + 69 crate manifests)
- **Workspace crates using `workspace = true`:** 55
- **Standalone crates with `[workspace]` override:** 6
  - `tooling/commit-msg-check`
  - `tooling/doc-link-check`
  - `tooling/fr-coverage`
  - `tooling/quality-gate`
  - `tooling/sbom-gen`
  - `tooling/target-pruner`
- **Crates with `[workspace.dependencies]` defined:** 1 (root `Cargo.toml`)
- **Crates with `[dependencies]` entries:** 69
- **Crates with no `[dependencies]`:** 1 (`focus-ui`)
