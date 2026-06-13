# W1-04: `pub use` Re-exports Audit

**Workspace**: `/Users/kooshapari/CodeProjects/Phenotype/repos`  
**Date**: 2026-06-13  
**Scope**: All `.rs` files under workspace crates (excluded `target/`, `vendor/`, `tests/` subdirs, and other excluded directories per root `Cargo.toml`)

## Summary

| Metric | Value |
|--------|-------|
| Crates with `pub use` re-exports | **20** |
| Total `pub use` re-export statements | **56** |

---

## By Crate

### `crates/focus-always-on` (1)
- `src/lib.rs:12` — `pub use focus_events::{NormalizedEvent, EventType, WellKnownEventType};`

### `crates/focus-backup` (1)
- `src/lib.rs:18` — `pub use manifest::{BackupManifest, ContentSection};`

### `crates/focus-connectors-mock-familycontrols` (2)
- `src/lib.rs:24` — `pub use synthetic_events::{SyntheticEventKind, SyntheticEventSchedule};`
- `src/lib.rs:25` — `pub use time_source::{DeterministicTimeSource, TimeSource};`

### `crates/focus-crypto` (5)
- `src/keychain.rs:94` — `pub use apple::AppleKeychainStore;`
- `src/keychain.rs:162` — `pub use linux::LinuxSecretServiceStore;`
- `src/lib.rs:9` — `pub use keychain::AppleKeychainStore;`
- `src/lib.rs:11` — `pub use keychain::LinuxSecretServiceStore;`
- `src/lib.rs:12` — `pub use keychain::{default_secure_store, InMemorySecretStore, NullSecureStore};`

### `crates/focus-errors` (1)
- `src/lib.rs:7` — `pub use phenotype_error_core::{ ErrorContext, PhenotypeError, Result, ResultExt, };` (multi-line)

### `crates/focus-eval` (1)
- `src/lib.rs:34` — `pub use batched::BatchedRuleEvaluationPipeline;`

### `crates/focus-hash` (1)
- `src/lib.rs:23` — `pub use phenotype_crypto::HashAlgorithm;`

### `crates/focus-mcp-server` (2)
- `src/lib.rs:7` — `pub use server::run_stdio;`
- `src/lib.rs:8` — `pub use tools::FocalPointToolsImpl;`

### `crates/focus-observability` (3)
- `src/lib.rs:59` — `pub use metrics::MetricsRegistry;`
- `src/lib.rs:60` — `pub use privacy_filter::SpanPrivacyFilter;`
- `src/lib.rs:61` — `pub use spans::{ AuditSpanAttrs, ConnectorSpanAttrs, RuleSpanAttrs, SpanKind, WalletSpanAttrs, };` (multi-line)

### `crates/focus-plugin-sdk` (5)
- `src/lib.rs:21` — `pub use manifest::PluginManifest;`
- `src/lib.rs:22` — `pub use plugin::ConnectorPlugin;`
- `src/lib.rs:23` — `pub use runtime::{PluginRuntime, RuntimeConfig};`
- `src/lib.rs:24` — `pub use signing::PluginSignature;`
- `src/capabilities/mod.rs:5` — `pub use http::{HttpCapability, HttpProxy, HttpRequest, HttpResponse};`

### `crates/focus-result` (2)
- `src/lib.rs:6` — `pub use focus_errors::{FocusError, FocusResult, PhenotypeError, Result};`
- `src/lib.rs:9` — `pub use focus_errors::Result as StdResult;`

### `crates/focus-rules` (1)
- `src/lib.rs:6` — `pub use builder::{describe_dsl, DslActionSpec, DslCatalog, DslConditionSpec, DslParam, DslTriggerSpec, RuleBuilder};`

### `crates/focus-storage` (3)
- `src/lib.rs:48` — `pub use focus_planning::TaskStore;`
- `src/lib.rs:69` — `pub use sqlite::SqliteAdapter;`
- `src/lib.rs:70` — `pub use wipe::{wipe_all, WipeReceipt};`

### `crates/focus-sync` (5)
- `src/lib.rs:16` — `pub use cursor_store::{CursorStore, InMemoryCursorStore, NoopCursorStore, EVENTS_ENTITY_TYPE};`
- `src/lib.rs:17` — `pub use dedup_event_sink::DeduplicatingEventSink;`
- `src/lib.rs:18` — `pub use event_sink::{EventSink, NoopEventSink};`
- `src/lib.rs:19` — `pub use retry::{next_delay, RetryPolicy};`
- `src/lib.rs:20` — `pub use cloudkit_port::{CloudKitPort, CloudKitRecord, CloudKitPortError, ConflictRecord, ConflictResolution, NoopCloudKitPort, PullOutcome};`

### `crates/focus-telemetry` (2)
- `src/lib.rs:38` — `pub use audit::AuditRecord;`
- `src/lib.rs:39` — `pub use pii_scrubber::PiiScrubber;`

### `crates/focus-templates` (1)
- `src/lib.rs:31` — `pub use signing::{verify_pack, verify_pack_bytes, parse_root_pubkey, PHENOTYPE_ROOT_PUBKEYS};`

### `crates/focus-transpilers` (1)
- `src/lib.rs:30` — `pub use focus_ir::Document;`

### `crates/pheno-tracing` (1)
- `src/lib.rs:8` — `pub use port::{TracePort, TraceOperation, TraceResult, SpanId, TraceId, SpanKind};`

### `crates/phenotype-config` (13)
- `src/lib.rs:34` — `pub use domain::{Config, ConfigValue, Layer, LayerPriority};`
- `src/lib.rs:35` — `pub use domain::errors::ConfigError;`
- `src/lib.rs:36` — `pub use application::builder::ConfigBuilder;`
- `src/lib.rs:37` — `pub use infrastructure::error::ConfigKitError;`
- `src/adapters/mod.rs:6` — `pub use sources::{FileSource, EnvSource};`
- `src/adapters/mod.rs:7` — `pub use formats::{TomlFormat, YamlFormat, JsonFormat};`
- `src/application/mod.rs:6` — `pub use builder::ConfigBuilder;`
- `src/domain/mod.rs:11` — `pub use config::{Config, ConfigValue, ConfigPath};`
- `src/domain/mod.rs:12` — `pub use layers::{Layer, LayerPriority, MergeStrategy, LayerStack};`
- `src/domain/mod.rs:13` — `pub use sources::Source;`
- `src/domain/mod.rs:14` — `pub use validation::Validator;`
- `src/domain/mod.rs:15` — `pub use errors::ConfigError;`
- `src/infrastructure/mod.rs:5` — `pub use error::ConfigKitError;`

### `crates/phenotype-crypto` (5)
- `src/lib.rs:36` — `pub use hashing::{Hasher, Hash, HashAlgorithm};`
- `src/lib.rs:37` — `pub use encryption::{AesGcmEncryptor, AesGcmError};`
- `src/lib.rs:38` — `pub use key_derivation::{Kdf, KdfParams, Pbkdf2Error};`
- `src/lib.rs:39` — `pub use hmac::{HmacSha256, HmacError};`
- `src/lib.rs:40` — `pub use signatures::{Ed25519Signer, Ed25519Verifier, SignatureError};`

---

## Crates with Zero `pub use` Re-exports

The remaining **53 crates** in the workspace (under `crates/`, `tooling/`, `tests/`) have no `pub use` re-exports.

### `crates/` (40 without `pub use`)
`connector-canvas`, `connector-fitbit`, `connector-gcal`, `connector-github`, `connector-linear`, `connector-notion`, `connector-readwise`, `connector-strava`, `connector-testkit`, `focus-asset-fetcher`, `focus-audit`, `focus-calendar`, `focus-ci-watcher`, `focus-cli`, `focus-coaching`, `focus-connectors`, `focus-demo-seed`, `focus-domain`, `focus-entitlements`, `focus-events`, `focus-ffi`, `focus-icon-gen`, `focus-ir`, `focus-lang`, `focus-mascot`, `focus-penalties`, `focus-planning`, `focus-policy`, `focus-release-bot`, `focus-replay`, `focus-rewards`, `focus-rituals`, `focus-rule-suggester`, `focus-scheduler`, `focus-sync-store`, `focus-time`, `focus-ui`, `focus-webhook-server`, `phenotype-derive`, `phenotype-error-core`, `phenotype-policy-engine`

### `tooling/` (12 without `pub use`)
`agent-orchestrator`, `bench-guard`, `commit-msg-check`, `demo-walkthrough`, `disk-check`, `doc-link-check`, `fr-coverage`, `quality-gate`, `release-cut`, `sbom-gen`, `target-pruner`, `worklog-aggregator`

### `tests/` (1 without `pub use`)
`e2e`

---

*Generated by Wave1 W1-04 audit task.*
