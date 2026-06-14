W3-10: Plugin system hardening
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:1://! Focus [01;31m[KPlugin[m[K SDK: WASM sandbox runtime for community connectors.
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:3://! Provides `[01;31m[KPlugin[m[KRuntime` wrapping `[01;31m[Kwasmtime[m[K` with strict capability caps:
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:8://! [01;31m[KPlugin[m[Ks implement `Connector[01;31m[KPlugin[m[K` ABI: `poll(config_ptr, config_len) -> (ptr, len)`
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:21:pub use manifest::[01;31m[KPlugin[m[KManifest;
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:22:pub use plugin::Connector[01;31m[KPlugin[m[K;
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:23:pub use runtime::{[01;31m[KPlugin[m[KRuntime, RuntimeConfig};
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:24:pub use signing::[01;31m[KPlugin[m[KSignature;
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:28:pub struct [01;31m[KPlugin[m[KEvent {
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:35:/// [01;31m[KPlugin[m[K execution error types.
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:37:pub enum [01;31m[KPlugin[m[KError {
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:50:    #[error("[01;31m[KPlugin[m[K panicked: {0}")]
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:51:    [01;31m[KPlugin[m[KPanicked(String),
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:80:        let event = [01;31m[KPlugin[m[KEvent {
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:88:        let deserialized: [01;31m[KPlugin[m[KEvent = serde_json::from_str(&json).unwrap();
/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:114:        let manifest: [01;31m[KPlugin[m[KManifest =
===
//! Focus Plugin SDK: WASM sandbox runtime for community connectors.
//!
//! Provides `PluginRuntime` wrapping `wasmtime` with strict capability caps:
//! - 10 MB memory limit
//! - 5s wall-clock timeout
//! - No filesystem or network (host provides config via linear memory)
//!
//! Plugins implement `ConnectorPlugin` ABI: `poll(config_ptr, config_len) -> (ptr, len)`
//! returning NDJSON event stream.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

pub mod capabilities;
pub mod manifest;
pub mod plugin;
pub mod runtime;
pub mod signing;

pub use manifest::PluginManifest;
pub use plugin::ConnectorPlugin;
pub use runtime::{PluginRuntime, RuntimeConfig};
pub use signing::PluginSignature;

/// Event emitted by a plugin in NDJSON format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEvent {
    pub id: String,
    pub kind: String,
    pub timestamp: i64,
    pub data: serde_json::Value,
}

/// Plugin execution error types.
#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Memory limit exceeded: {0}")]
    MemoryLimitExceeded(usize),

