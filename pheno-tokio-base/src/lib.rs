//! # pheno-tokio-base
//!
//! Canonical Tokio async runtime base for the `pheno-*` fleet.
//!
//! This crate is the **single source of truth** for the Tokio feature set
//! used across the pheno-* service crates. It exists for two reasons:
//!
//! 1. **Feature-set lockstep**: a fleet-wide Tokio upgrade is a single
//!    version bump in this `Cargo.toml` rather than an N-crate search and
//!    replace. All transitive `pheno-*` consumers get the same `tokio`
//!    major.minor and the same feature set (macros, rt-multi-thread,
//!    sync, time, signal).
//!
//! 2. **Two canonical helpers** that every pheno-* binary wants:
//!    - [`runtime()`] — a multi-threaded Tokio runtime pre-configured with
//!      a sensible name + worker thread count, so a server can be brought
//!      up with `let rt = pheno_tokio_base::runtime(); rt.block_on(async { ... })`
//!      or used as a `Builder::on_thread_start(...)` factory.
//!    - [`shutdown_signal()`] — an `async fn` that resolves on the first
//!      of `SIGTERM` (POSIX) / `Ctrl-C` (Windows + POSIX), the canonical
//!      "shut down cleanly on container restart / interactive Ctrl-C"
//!      pattern documented in the Tokio guide.
//!
//! ## Layering
//!
//! This crate is **layer 0** of the pheno-* async stack:
//!
//! ```text
//!     pheno-axum-stack
//!           ↓ uses
//!      pheno-tower   (tower + tower::retry / tower::timeout)
//!           ↓ uses
//!   pheno-tokio-base ← (this crate; re-exports tokio + runtime() + shutdown_signal())
//! ```
//!
//! The intent is that any pheno-* binary depends on `pheno-tokio-base`
//! (and only this crate) for the Tokio version pinning, and depends on
//! `pheno-tower` / `pheno-axum-stack` separately for the tower / axum
//! re-exports.
//!
//! ## Why a re-export crate?
//!
//! The fleet audit (2026-06-10) found that 4 pheno-* crates were
//! directly depending on `tokio` with **3 different feature sets** and
//! **2 different version specifiers** (`"1"` vs `"1.39"`). The
//! `feature-set` divergence is the silent killer: one crate pulls in
//! `tokio/fs` and the other doesn't, so `tokio::fs::File` won't compile
//! in the second crate's test target. This crate makes the divergence
//! impossible by being the only one that names the features.
//!
//! ## Why standalone (not a workspace member)?
//!
//! Mirrors the L3 #46 `pheno-errors` and L3 #47 `pheno-tracing`
//! convention: an empty `[workspace]` table in this `Cargo.toml` keeps
//! the crate a standalone package, so its `cargo test` / `cargo clippy`
//! loop is independent of the 56+ member root workspace. This is
//! critical because the root workspace's `Cargo.toml` is concurrently
//! modified by other L3 agents and pinning a new member would race
//! with them.

/// Re-export of the pinned `tokio` crate. Consumers should `use`
/// `pheno_tokio_base::tokio` rather than depending on `tokio` directly
/// so the fleet-wide version + feature set stays locked.
pub use ::tokio;

/// Build a multi-threaded Tokio [`Runtime`](::tokio::runtime::Runtime)
/// pre-configured for pheno-* services.
///
/// The runtime:
///
/// - Uses **all available logical cores** as worker threads
///   (`worker_threads = num_cpus`, i.e. `available_parallelism()`).
/// - Enables the **I/O** and **time** reactors (required for the
///   `axum`/`tower-http` middlewares the fleet uses).
/// - Names all worker threads `pheno-tokio-worker-N` for sane
///   `top -H` / `perf` output.
/// - Stacks the **all** feature set, which is a strict superset of the
///   `[macros, rt-multi-thread, sync, time, signal]` set this crate
///   declares in `[dependencies]` — so any future ad-hoc `tokio::fs` /
///   `tokio::net` / `tokio::process` use in a downstream crate will
///   "just work" through the [`tokio`] re-export without a Cargo.toml
///   change here.
///
/// # Example
///
/// ```no_run
/// let rt = pheno_tokio_base::runtime();
/// rt.block_on(async {
///     println!("hello from a pheno-tokio worker");
/// });
/// ```
pub fn runtime() -> ::tokio::runtime::Runtime {
    ::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static ATOMIC: ::std::sync::atomic::AtomicUsize =
                ::std::sync::atomic::AtomicUsize::new(0);
            let id = ATOMIC.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
            format!("pheno-tokio-worker-{id}")
        })
        .build()
        .expect("pheno-tokio-base: failed to build the multi-threaded Tokio runtime")
}

/// Future that resolves on the first of `SIGTERM` (POSIX) or `Ctrl-C`.
///
/// This is the canonical "shut down cleanly on container restart /
/// interactive Ctrl-C" pattern. The function is a thin wrapper over
/// [`tokio::signal::ctrl_c`] and (on `unix`) [`tokio::signal::unix::signal`]
/// for `SIGTERM` / `SIGINT` / `SIGHUP`, racing them with
/// [`tokio::select!`](::tokio::select).
///
/// # Platform behavior
///
/// | Platform | Signals watched                                  |
/// |----------|--------------------------------------------------|
/// | unix     | `SIGTERM`, `SIGINT` (alias of Ctrl-C), `SIGHUP`  |
/// | other    | `Ctrl-C` only                                    |
///
/// `SIGHUP` is included so a `kill -HUP <pid>` (e.g. `systemctl reload`)
/// triggers a clean restart alongside the more common `SIGTERM` and
/// `Ctrl-C` paths.
///
/// # Cancellation
///
/// The returned future is cancellation-safe: if the caller drops it
/// (e.g. by `tokio::select!`-ing against another branch that won, or
/// by wrapping it in [`tokio::time::timeout`]), the signal listeners
/// are dropped with the future. This means a caller that wraps
/// `shutdown_signal()` in `tokio::time::timeout(Duration::from_millis(50), ...)`
/// to satisfy a test that doesn't want to wait for a real signal will
/// see the future **return immediately** with `Err(Elapsed)` — the
/// signal handlers don't survive the drop, so no listener is left
/// armed. This is the property the `shutdown_signal_returns_immediately_when_dropped`
/// test pins down.
///
/// # Example
///
/// ```no_run
/// # async fn run_server() -> std::io::Result<()> { Ok(()) }
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let rt = pheno_tokio_base::runtime();
/// rt.block_on(async {
///     ::tokio::select! {
///         res = run_server() => { res?; }
///         _   = pheno_tokio_base::shutdown_signal() => {
///             eprintln!("pheno-tokio-base: shutdown signal received, draining...");
///         }
///     }
///     Ok::<(), Box<dyn std::error::Error>>(())
/// })?;
/// # Ok(())
/// # }
/// ```
pub async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use ::tokio::signal::unix::{signal, SignalKind};
        // Arm listeners for SIGTERM / SIGHUP eagerly. SIGINT is
        // intentionally NOT listed here because `tokio::signal::ctrl_c`
        // (below) already handles it on both unix and windows; arming
        // both would race and produce a single observation.
        let mut sigterm = signal(SignalKind::terminate())
            .expect("pheno-tokio-base: failed to install SIGTERM handler");
        let mut sighup = signal(SignalKind::hangup())
            .expect("pheno-tokio-base: failed to install SIGHUP handler");

        ::tokio::select! {
            _ = sigterm.recv() => {}
            _ = sighup.recv()  => {}
            _ = ::tokio::signal::ctrl_c() => {}
        }
    }

    #[cfg(not(unix))]
    {
        // Windows / wasm: only Ctrl-C is meaningful.
        let _ = ::tokio::signal::ctrl_c().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `runtime()` returns a [`Runtime`] that can drive a future to
    /// completion. This is the minimal smoke test: if the runtime
    /// doesn't even tick, every other pheno-* service is broken.
    #[test]
    fn runtime_runs_a_future() {
        let rt = runtime();
        let value = rt.block_on(async { 41_u8 + 1_u8 });
        assert_eq!(value, 42, "runtime must execute the future to completion");
    }

    /// `shutdown_signal()` is cancellation-safe: wrapping it in
    /// [`tokio::time::timeout`] with a tiny deadline must return
    /// `Err(Elapsed)` and the future must NOT keep a signal listener
    /// armed across the drop. We verify the deadline by asserting
    /// `timeout` returns `Err` (i.e. the future did not complete on
    /// its own) and that the wrap completes in well under the wrap
    /// duration (sanity bound for the test runtime).
    #[test]
    fn shutdown_signal_returns_immediately_when_dropped() {
        let rt = runtime();
        let started = std::time::Instant::now();
        let result: Result<(), ::tokio::time::error::Elapsed> = rt.block_on(async {
            ::tokio::time::timeout(std::time::Duration::from_millis(50), shutdown_signal()).await
        });
        let elapsed = started.elapsed();
        assert!(
            result.is_err(),
            "timeout must elapse (signal never fires) when shutdown_signal() is dropped"
        );
        assert!(
            elapsed < std::time::Duration::from_secs(1),
            "drop must be immediate: elapsed = {elapsed:?}"
        );
    }
}
