//! Plugin manifest schema for capability declaration and metadata.

use serde::{Deserialize, Serialize};

/// Top-level plugin manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub plugin: PluginMetadata,
    pub capabilities: PluginCapabilities,
    #[serde(default)]
    pub interface: PluginInterface,
}

/// Plugin metadata section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: String,
    #[serde(rename = "type")]
    pub plugin_type: String,
    pub api_version: String,
}

/// Capability declaration: phase-1 supports http_client and timer only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapabilities {
    #[serde(default)]
    pub http: bool,
    #[serde(default)]
    pub filesystem: Option<FilesystemScope>,
    #[serde(default)]
    pub timer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemScope {
    pub scope: String,
}

/// Plugin interface definition.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginInterface {
    #[serde(default)]
    pub connector: Option<ConnectorInterface>,
}

/// Connector-specific interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorInterface {
    pub scope: String,
    pub auth: String,
    pub events: Vec<EventDefinition>,
}

/// Event schema definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDefinition {
    pub name: String,
    pub fields: Vec<String>,
}
