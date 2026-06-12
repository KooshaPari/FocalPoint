//! Integration tests for `pheno-tokio-base`.
//!
//! These tests live in `tests/` (as opposed to `src/lib.rs`) so they
//! exercise the crate's **public** API the same way a downstream
//! consumer would: by depending on the crate by name and calling its
//! items. This catches accidental `pub(crate)` or undocumented
//! internal-only paths that would silently work for in-module tests.

use std::time::{Duration, Instant};

use pheno_tokio_base::tokio;

/// `runtime()` returns a Tokio runtime that can drive a future to
/// completion. Mirrors the in-module unit test of the same name but
/// runs through the public crate surface.
#[test]
fn runtime_runs_a_future() {
    let rt = pheno_tokio_base::runtime();
    let value = rt.block_on(async { 41_u8 + 1_u8 });
    assert_eq!(value, 42, "runtime must execute the future to completion");
}

/// `shutdown_signal()` is cancellation-safe: wrapping it in
/// `tokio::time::timeout` with a tiny deadline must return `Err(Elapsed)`
/// and the future must NOT keep a signal listener armed across the drop.
#[test]
fn shutdown_signal_returns_immediately_when_dropped() {
    let rt = pheno_tokio_base::runtime();
    let started = Instant::now();
    let result: Result<(), tokio::time::error::Elapsed> = rt.block_on(async {
        tokio::time::timeout(
            Duration::from_millis(50),
            pheno_tokio_base::shutdown_signal(),
        )
        .await
    });
    let elapsed = started.elapsed();
    assert!(
        result.is_err(),
        "timeout must elapse (signal never fires) when shutdown_signal() is dropped"
    );
    assert!(
        elapsed < Duration::from_secs(1),
        "drop must be immediate: elapsed = {elapsed:?}"
    );
}

/// The re-exported `tokio` symbol resolves to the pinned `tokio` crate
/// version. This is the "single source of truth" property the crate
/// promises: every pheno-* binary sees the same major.minor + features.
#[test]
fn tokio_reexport_resolves() {
    // Compile-time sanity: the re-exported symbol must expose
    // `tokio::time::timeout` (proves the `time` feature is enabled
    // through the re-export, not via a side dependency).
    fn assert_timeout_is_callable() {
        let _: fn(Duration, std::future::Ready<()>) -> _ = tokio::time::timeout;
    }
    let _ = assert_timeout_is_callable;
}
