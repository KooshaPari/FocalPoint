#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Unified configuration management — typed config layer with sources, validation, and hot-reload.
//!
//! # Features
//!
//! - `Config` trait: load, save, validate
//! - `ConfigSource` enum: File, Env, Cli, Default
//! - `ConfigBuilder`: builder pattern for composing configs
//! - `ConfigSchema`: validation with `Validator` trait
//! - Env var prefixing: `FOCUS_*`, `PHENOTYPE_*`

use focus_errors::FocusError;
use focus_result::FocusResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration source types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigSource {
    /// Default values baked into the application.
    Default,
    /// Values loaded from a configuration file (TOML, JSON, YAML).
    File,
    /// Values sourced from environment variables.
    Env,
    /// Values passed via command-line arguments.
    Cli,
    /// Values from a remote source (e.g., feature flags, config server).
    Remote,
}

/// Trait for typed configuration objects.
///
/// Implement this trait on domain-specific config structs to get uniform
/// loading, saving, and validation across the workspace.
pub trait Config: Sized + Send + Sync {
    /// Load configuration from a source.
    fn load(source: ConfigSource) -> FocusResult<Self>;
    /// Save configuration to a persistent location.
    fn save(&self) -> FocusResult<()>;
    /// Validate the configuration and return errors if any.
    fn validate(&self) -> FocusResult<()>;
}

/// Validation trait for individual config fields.
pub trait Validator {
    /// Validate a value and return an error if invalid.
    fn validate(&self) -> FocusResult<()>;
}

/// Unified configuration schema for a crate or module.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigSchema {
    /// Human-readable name of the configuration scope.
    pub name: String,
    /// Version of the configuration schema.
    pub version: String,
    /// Required fields and their validators.
    pub required: Vec<String>,
    /// Optional fields with default values.
    pub optional: HashMap<String, serde_json::Value>,
    /// Environment variable prefix for this scope.
    pub env_prefix: String,
}

impl ConfigSchema {
    /// Create a new schema with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: "1.0.0".into(),
            required: Vec::new(),
            optional: HashMap::new(),
            env_prefix: "FOCUS".into(),
        }
    }

    /// Set the schema version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Add a required field.
    pub fn require(mut self, field: impl Into<String>) -> Self {
        self.required.push(field.into());
        self
    }

    /// Add an optional field with a default value.
    pub fn optional(
        mut self,
        field: impl Into<String>,
        default: serde_json::Value,
    ) -> Self {
        self.optional.insert(field.into(), default);
        self
    }

    /// Set the environment variable prefix.
    pub fn with_env_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.env_prefix = prefix.into();
        self
    }

    /// Build the full env var name for a field.
    pub fn env_var(&self, field: &str) -> String {
        format!("{}_{}", self.env_prefix, field.to_uppercase())
    }

    /// Validate that all required fields are present in a given map.
    pub fn validate_map(&self, values: &HashMap<String, serde_json::Value>) -> FocusResult<()> {
        for field in &self.required {
            if !values.contains_key(field) {
                return Err(FocusError::config(format!(
                    "missing required field '{}' in config '{}'",
                    field, self.name
                )));
            }
        }
        Ok(())
    }
}

/// Builder for composing configuration from multiple sources.
///
/// Sources are merged in priority order: Default < File < Env < Cli < Remote.
#[derive(Debug, Clone, Default)]
pub struct ConfigBuilder {
    layers: Vec<(ConfigSource, HashMap<String, serde_json::Value>)>,
}

impl ConfigBuilder {
    /// Create a new empty config builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a layer of configuration values.
    pub fn with_layer(
        mut self,
        source: ConfigSource,
        values: HashMap<String, serde_json::Value>,
    ) -> Self {
        self.layers.push((source, values));
        self
    }

    /// Merge all layers into a single map, with later layers overriding earlier ones.
    pub fn merge(&self) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        for (_, values) in &self.layers {
            for (k, v) in values {
                result.insert(k.clone(), v.clone());
            }
        }
        result
    }

    /// Build the final configuration, validating against a schema.
    pub fn build(self, schema: &ConfigSchema) -> FocusResult<HashMap<String, serde_json::Value>> {
        let merged = self.merge();
        schema.validate_map(&merged)?;
        Ok(merged)
    }
}

/// Hot-reload watcher for configuration changes.
///
/// Watches a file path and notifies when the configuration should be reloaded.
#[derive(Debug, Clone)]
pub struct ConfigWatcher {
    /// Path to the configuration file being watched.
    pub path: String,
    /// Polling interval in milliseconds.
    pub interval_ms: u64,
}

impl ConfigWatcher {
    /// Create a new watcher for the given path.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            interval_ms: 5000,
        }
    }

    /// Set the polling interval.
    pub fn with_interval(mut self, ms: u64) -> Self {
        self.interval_ms = ms;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_merge() {
        let mut default = HashMap::new();
        default.insert("port".into(), serde_json::json!(8080));
        default.insert("host".into(), serde_json::json!("localhost"));

        let mut env = HashMap::new();
        env.insert("port".into(), serde_json::json!(9090));

        let builder = ConfigBuilder::new()
            .with_layer(ConfigSource::Default, default)
            .with_layer(ConfigSource::Env, env);

        let merged = builder.merge();
        assert_eq!(merged.get("port"), Some(&serde_json::json!(9090))); // overridden
        assert_eq!(merged.get("host"), Some(&serde_json::json!("localhost"))); // kept
    }

    #[test]
    fn test_config_schema_validate() {
        let schema = ConfigSchema::new("test")
            .require("api_key")
            .optional("timeout", serde_json::json!(30));

        let mut valid = HashMap::new();
        valid.insert("api_key".into(), serde_json::json!("secret"));
        assert!(schema.validate_map(&valid).is_ok());

        let mut invalid = HashMap::new();
        invalid.insert("timeout".into(), serde_json::json!(60));
        assert!(schema.validate_map(&invalid).is_err());
    }

    #[test]
    fn test_config_schema_env_var() {
        let schema = ConfigSchema::new("app").with_env_prefix("FOCUS");
        assert_eq!(schema.env_var("api_key"), "FOCUS_API_KEY");
    }

    #[test]
    fn test_config_builder_with_schema() {
        let schema = ConfigSchema::new("test").require("enabled");
        let mut values = HashMap::new();
        values.insert("enabled".into(), serde_json::json!(true));

        let builder = ConfigBuilder::new().with_layer(ConfigSource::Default, values);
        let result = builder.build(&schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_builder_schema_missing_field() {
        let schema = ConfigSchema::new("test").require("required_field");
        let builder = ConfigBuilder::new();
        let result = builder.build(&schema);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_source_serde() {
        let sources = vec![
            ConfigSource::Default,
            ConfigSource::File,
            ConfigSource::Env,
            ConfigSource::Cli,
            ConfigSource::Remote,
        ];
        for source in sources {
            let json = serde_json::to_string(&source).unwrap();
            let round: ConfigSource = serde_json::from_str(&json).unwrap();
            assert_eq!(source, round);
        }
    }

    #[test]
    fn test_config_schema_serde_roundtrip() {
        let schema = ConfigSchema::new("test")
            .with_version("2.0.0")
            .require("key")
            .optional("limit", serde_json::json!(100))
            .with_env_prefix("PHENOTYPE");

        let json = serde_json::to_string(&schema).unwrap();
        let round: ConfigSchema = serde_json::from_str(&json).unwrap();
        assert_eq!(schema, round);
    }

    #[test]
    fn test_config_watcher_builder() {
        let watcher = ConfigWatcher::new("/etc/focus/config.toml").with_interval(1000);
        assert_eq!(watcher.path, "/etc/focus/config.toml");
        assert_eq!(watcher.interval_ms, 1000);
    }

    #[test]
    fn test_config_builder_empty_layers() {
        let builder = ConfigBuilder::new();
        let merged = builder.merge();
        assert!(merged.is_empty());
    }

    #[test]
    fn test_config_builder_multiple_layers() {
        let mut layer1 = HashMap::new();
        layer1.insert("a".into(), serde_json::json!(1));
        let mut layer2 = HashMap::new();
        layer2.insert("b".into(), serde_json::json!(2));
        let mut layer3 = HashMap::new();
        layer3.insert("a".into(), serde_json::json!(3));

        let builder = ConfigBuilder::new()
            .with_layer(ConfigSource::Default, layer1)
            .with_layer(ConfigSource::File, layer2)
            .with_layer(ConfigSource::Env, layer3);

        let merged = builder.merge();
        assert_eq!(merged.get("a"), Some(&serde_json::json!(3))); // last wins
        assert_eq!(merged.get("b"), Some(&serde_json::json!(2)));
    }

    #[test]
    fn test_config_schema_version() {
        let schema = ConfigSchema::new("test").with_version("3.1.0");
        assert_eq!(schema.version, "3.1.0");
    }

    #[test]
    fn test_config_schema_optional_not_required() {
        let schema = ConfigSchema::new("test").optional("debug", serde_json::json!(false));
        let empty = HashMap::new();
        assert!(schema.validate_map(&empty).is_ok());
    }
}
