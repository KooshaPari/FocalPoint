//! Integration tests for the `pheno-plugin` crate (L3 #57).
//!
//! These are the 6 tests required by the L3 #57 spec. Cargo discovers
//! them as the `registry_test` integration-test binary:
//!
//! ```text
//! $ cargo test -p pheno-plugin
//!   ...
//!      Running tests/registry_test.rs
//! ```
//!
//! All six tests live in this one file (no helper modules) so the
//! integration-test binary has a single, self-contained surface that
//! matches the spec 1:1.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use pheno_plugin::{Plugin, PluginError, PluginRegistry};

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

/// A minimal plugin that records how many times its `init` was
/// invoked. Used by `init_all_invokes_each_plugin_init` to verify that
/// `init_all` dispatches exactly once per registered plugin.
struct CountingPlugin {
    n: &'static str,
    v: &'static str,
    init_calls: Arc<AtomicUsize>,
}

impl CountingPlugin {
    fn new(name: &'static str, version: &'static str) -> Self {
        Self {
            n: name,
            v: version,
            init_calls: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl Plugin for CountingPlugin {
    fn name(&self) -> &str {
        self.n
    }
    fn version(&self) -> &str {
        self.v
    }
    fn init(&self) -> Result<(), PluginError> {
        self.init_calls.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// L3 #57 spec — required tests (6)
// ---------------------------------------------------------------------------

/// Test 1 — a freshly-constructed `PluginRegistry` has no plugins.
#[test]
fn registry_starts_empty() {
    let reg = PluginRegistry::new();
    assert!(
        reg.get("anything").is_none(),
        "a fresh registry must not contain 'anything'"
    );
    assert!(
        reg.names().is_empty(),
        "a fresh registry must have no names"
    );
}

/// Test 2 — `register` actually adds a plugin (round-trips through
/// `get` and `names`).
#[test]
fn register_adds_plugin() {
    let mut reg = PluginRegistry::new();
    reg.register(Box::new(CountingPlugin::new("alpha", "0.1.0")))
        .expect("first register must succeed");
    assert!(reg.get("alpha").is_some(), "alpha must be registered");
    assert_eq!(reg.get("alpha").unwrap().name(), "alpha");
    assert_eq!(reg.get("alpha").unwrap().version(), "0.1.0");
    assert_eq!(reg.names(), vec!["alpha".to_string()]);
}

/// Test 3 — a second registration under the same name is rejected
/// with `DuplicateName` and the first registration is preserved.
#[test]
fn register_rejects_duplicate() {
    let mut reg = PluginRegistry::new();
    reg.register(Box::new(CountingPlugin::new("dup", "0.1.0")))
        .expect("first register must succeed");
    let err = reg
        .register(Box::new(CountingPlugin::new("dup", "0.2.0")))
        .expect_err("second register must fail");
    match err {
        PluginError::DuplicateName(name) => assert_eq!(name, "dup"),
        other => panic!("expected DuplicateName, got {other:?}"),
    }
    // First registration wins — the surviving plugin has v0.1.0.
    assert_eq!(reg.get("dup").unwrap().version(), "0.1.0");
    assert_eq!(reg.names(), vec!["dup".to_string()]);
}

/// Test 4 — `get` returns the registered plugin for known names and
/// `None` for unknown names.
#[test]
fn get_returns_registered_plugin() {
    let mut reg = PluginRegistry::new();
    reg.register(Box::new(CountingPlugin::new("alpha", "0.1.0")))
        .unwrap();
    reg.register(Box::new(CountingPlugin::new("beta", "0.2.0")))
        .unwrap();

    let alpha = reg.get("alpha").expect("alpha must be present");
    assert_eq!(alpha.name(), "alpha");
    assert_eq!(alpha.version(), "0.1.0");

    let beta = reg.get("beta").expect("beta must be present");
    assert_eq!(beta.name(), "beta");
    assert_eq!(beta.version(), "0.2.0");

    assert!(reg.get("gamma").is_none(), "gamma was never registered");
    assert!(reg.get("").is_none(), "empty key must not match");
}

/// Test 5 — `init_all` invokes `init` on every registered plugin
/// exactly once, and succeeds when all plugins return `Ok`.
#[test]
fn init_all_invokes_each_plugin_init() {
    let mut reg = PluginRegistry::new();
    let alpha = CountingPlugin::new("alpha", "0.1.0");
    let beta = CountingPlugin::new("beta", "0.2.0");
    let gamma = CountingPlugin::new("gamma", "0.3.0");
    let counter_alpha = alpha.init_calls.clone();
    let counter_beta = beta.init_calls.clone();
    let counter_gamma = gamma.init_calls.clone();
    reg.register(Box::new(alpha)).unwrap();
    reg.register(Box::new(beta)).unwrap();
    reg.register(Box::new(gamma)).unwrap();

    reg.init_all().expect("all inits are default-Ok");

    assert_eq!(counter_alpha.load(Ordering::SeqCst), 1);
    assert_eq!(counter_beta.load(Ordering::SeqCst), 1);
    assert_eq!(counter_gamma.load(Ordering::SeqCst), 1);
}

/// Test 6 — `names()` returns the registered names sorted ascending,
/// independent of registration order.
#[test]
fn names_returns_sorted() {
    let mut reg = PluginRegistry::new();
    // Register out of order on purpose: zeta, alpha, mu, beta.
    reg.register(Box::new(CountingPlugin::new("zeta", "0.1.0")))
        .unwrap();
    reg.register(Box::new(CountingPlugin::new("alpha", "0.1.0")))
        .unwrap();
    reg.register(Box::new(CountingPlugin::new("mu", "0.1.0")))
        .unwrap();
    reg.register(Box::new(CountingPlugin::new("beta", "0.1.0")))
        .unwrap();
    assert_eq!(
        reg.names(),
        vec![
            "alpha".to_string(),
            "beta".to_string(),
            "mu".to_string(),
            "zeta".to_string(),
        ]
    );
}
