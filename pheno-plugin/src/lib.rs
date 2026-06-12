//! # pheno-plugin — canonical plugin registry for the pheno-* fleet
//!
//! `pheno-plugin` is the L3 #57 substrate for in-process plugin loading
//! in the pheno-* fleet. It defines three things:
//!
//! 1. The [`Plugin`] trait — the contract every plugin implements.
//!    `Send + Sync` (so the registry can hand `&dyn Plugin` across thread
//!    boundaries), object-safe (so it can be stored as `Box<dyn Plugin>`),
//!    and with a default-no-op [`Plugin::init`] hook for the common
//!    case where a plugin just needs to be "present" at startup.
//! 2. The [`PluginError`] enum — the two-variant error type
//!    ([`PluginError::DuplicateName`] and
//!    [`PluginError::InitFailed`]) that covers the full surface of the
//!    registry.
//! 3. The [`PluginRegistry`] store — a name-indexed
//!    `HashMap<String, Box<dyn Plugin>>` with duplicate-name rejection,
//!    sorted `names()`, and bulk `init_all()`.
//!
//! ## Why a new crate?
//!
//! - `focus-plugin-sdk` (root `crates/` workspace) is an FFI-facing SDK
//!   that exposes plugins across a uniffi boundary to Swift/Kotlin. It
//!   is too heavy (depends on `uniffi`, a connector surface, `tokio`
//!   runtime plumbing) for an in-process Rust-only registry.
//! - `phenotype-registry` (in `phenotype-registry/`) is a
//!   JSON-Schema-driven *provider* registry. It is a different shape of
//!   thing (config-driven providers with discovery, not in-process
//!   plugins).
//!
//! `pheno-plugin` is the third path: tiny, in-process, type-strict, and
//! the L3 #57 spec verbatim.
//!
//! ## Consumers
//!
//! - L5 #88 (`helioscli` — wire HeliosCLI to pheno-plugin and load
//!   `helios-plugin-*` crates at startup).
//! - Any other L5 pheno-* host that wants a uniform plugin entrypoint.
//!
//! ## Example
//!
//! ```
//! use pheno_plugin::{Plugin, PluginRegistry, PluginError};
//!
//! struct EchoPlugin;
//! impl Plugin for EchoPlugin {
//!     fn name(&self) -> &str { "echo" }
//!     fn version(&self) -> &str { "0.1.0" }
//! }
//!
//! let mut reg = PluginRegistry::new();
//! reg.register(Box::new(EchoPlugin)).expect("first echo");
//! assert_eq!(reg.names(), vec!["echo".to_string()]);
//! reg.init_all().expect("init is default-Ok");
//! ```

use std::collections::HashMap;

use thiserror::Error;

// ---------------------------------------------------------------------------
// Plugin trait
// ---------------------------------------------------------------------------

/// The canonical plugin contract for the pheno-* fleet.
///
/// Every plugin must be `Send + Sync` so the registry can hand
/// `&dyn Plugin` references across thread boundaries (e.g., a TUI
/// thread + a worker thread), and must expose a stable `name()` (used
/// as the registry key) plus a `version()` (used for diagnostics and
/// capability negotiation).
///
/// `init` is intentionally non-`async`: plugin initialization is
/// expected to be cheap (load config, register handlers, log a "ready"
/// line). Anything heavier belongs inside a separate `start`/`run`
/// method that the host can drive asynchronously after `init_all`
/// succeeds.
///
/// The trait is object-safe by construction: no associated types, no
/// generic methods, only `&self` receivers, and `Send + Sync` as
/// super-traits. This is what enables `Box<dyn Plugin>` storage and
/// runtime plugin loading from crates the host does not statically
/// depend on.
pub trait Plugin: Send + Sync {
    /// Stable, registry-unique name. Used as the `HashMap` key in
    /// [`PluginRegistry`]. Two plugins that return the same `name()`
    /// collide at [`PluginRegistry::register`] time and the second
    /// registration is rejected with [`PluginError::DuplicateName`].
    fn name(&self) -> &str;

    /// Semantic-version string (e.g., `"0.1.0"`, `"1.2.3-rc.1"`).
    /// Not parsed by the registry; surfaced for diagnostics only.
    fn version(&self) -> &str;

    /// One-shot initialization hook. The default is a no-op (`Ok(())`)
    /// because most plugins just need to be "present" — they don't
    /// have anything to do at registration time. Override only if you
    /// have real work to do (e.g., opening a file handle, validating
    /// a license key, registering a CLI subcommand).
    ///
    /// Errors propagate up through [`PluginRegistry::init_all`], which
    /// short-circuits on the first failure and returns it.
    fn init(&self) -> Result<(), PluginError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// The error type returned by [`PluginRegistry`] operations.
///
/// Two variants cover the full surface of the registry:
///
/// - [`PluginError::DuplicateName`]: a second plugin tried to
///   register under a name that is already taken. The first
///   registration is preserved (no overwrite); the offending plugin
///   is dropped.
/// - [`PluginError::InitFailed`]: a plugin's `init` hook returned an
///   error. Carries a free-form reason (typically the `Display` of
///   the plugin's own error type, or a short sentence describing what
///   went wrong).
#[derive(Debug, Error)]
pub enum PluginError {
    /// A plugin tried to register under a name that is already in the
    /// registry. The wrapped `String` is the conflicting name.
    #[error("plugin name already registered: {0}")]
    DuplicateName(String),

    /// A plugin's `init` hook returned an error. The wrapped `String`
    /// is a free-form reason (typically the plugin's own error
    /// rendered via `Display`).
    #[error("plugin init failed: {0}")]
    InitFailed(String),
}

// ---------------------------------------------------------------------------
// PluginRegistry
// ---------------------------------------------------------------------------

/// The canonical name-indexed plugin store.
///
/// Backed by a `HashMap<String, Box<dyn Plugin>>`. The map's key is
/// the plugin's [`Plugin::name()`] value at registration time;
/// subsequent renames of the same `Box<dyn Plugin>` are not reflected
/// (the name is captured at registration).
///
/// # Examples
///
/// ```
/// use pheno_plugin::{Plugin, PluginRegistry};
///
/// struct Alpha;
/// impl Plugin for Alpha {
///     fn name(&self) -> &str { "alpha" }
///     fn version(&self) -> &str { "0.1.0" }
/// }
///
/// let mut reg = PluginRegistry::new();
/// reg.register(Box::new(Alpha)).unwrap();
/// assert!(reg.get("alpha").is_some());
/// assert!(reg.get("beta").is_none());
/// assert_eq!(reg.names(), vec!["alpha".to_string()]);
/// ```
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Register a plugin under its [`Plugin::name()`].
    ///
    /// Returns [`PluginError::DuplicateName`] (with the conflicting
    /// name) if a plugin with the same name is already registered.
    /// The first registration wins; the second `Box<dyn Plugin>` is
    /// dropped.
    pub fn register(&mut self, p: Box<dyn Plugin>) -> Result<(), PluginError> {
        let name = p.name().to_owned();
        if self.plugins.contains_key(&name) {
            return Err(PluginError::DuplicateName(name));
        }
        self.plugins.insert(name, p);
        Ok(())
    }

    /// Look up a registered plugin by name.
    ///
    /// Returns `None` if no plugin is registered under that name. The
    /// returned reference is borrowed for the lifetime of `&self`, so
    /// the registry can hand `&dyn Plugin` to multiple consumers
    /// concurrently.
    pub fn get(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Return the names of all registered plugins, sorted ascending.
    ///
    /// The sort is stable (uses `Vec::sort`) and lexicographic on
    /// `String`. The order is independent of registration order,
    /// which keeps test assertions and CLI help output deterministic.
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.plugins.keys().cloned().collect();
        names.sort();
        names
    }

    /// Invoke [`Plugin::init`] on every registered plugin, in
    /// registration (insertion) order.
    ///
    /// Returns the first [`PluginError`] it sees and stops. Plugin
    /// `init` hooks return `Result<(), PluginError>`, so any failure
    /// is propagated directly through the `?` operator.
    pub fn init_all(&self) -> Result<(), PluginError> {
        for plugin in self.plugins.values() {
            plugin.init()?;
        }
        Ok(())
    }
}

impl Default for PluginRegistry {
    /// An empty registry, equivalent to [`PluginRegistry::new`].
    fn default() -> Self {
        Self::new()
    }
}
