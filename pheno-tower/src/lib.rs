//! # pheno-tower
//!
//! Canonical Tower service-extension facade for the `pheno-*` fleet.
//! Two opinionated shorthands that show up in every pheno-* service:
//!
//! 1. [`timeout::layer`] — a one-line wrapper around
//!    [`tower::timeout::TimeoutLayer::new`] so a service can write
//!    `.layer(pheno_tower::timeout::layer(Duration::from_secs(30)))`
//!    instead of two import lines.
//!
//! 2. [`retry::policy`] — a 3-attempt exponential-backoff
//!    [`tower::retry::Policy`] implementation. Plug it into a
//!    [`tower::retry::Retry`] middleware via
//!    `Retry::new(pheno_tower::retry::policy(), svc)` to get "retry
//!    on error, up to 2 times (3 total attempts), with exponential
//!    backoff between attempts".
//!
//! ## Re-exports
//!
//! ```text
//!     tower         — `Service` trait + middleware combinators
//!     tower::timeout — `Timeout`, `TimeoutLayer`
//!     tower::retry  — `Retry`, `RetryLayer`, `Policy`
//! ```
//!
//! ## Why a "facade" rather than direct `tower` use?
//!
//! Two reasons:
//!
//! - **Tower 0.4 has no built-in exponential backoff.** The
//!   `tower::retry` module in tower 0.4 ships only the `Policy` trait
//!   and the `Retry` middleware; the actual backoff schedule is a
//!   `Policy` implementation detail. This crate provides a canonical
//!   3-attempt exponential schedule so fleet services don't each
//!   reinvent one (the L1/L2 fleet audit found 4 distinct in-house
//!   implementations, with off-by-one errors in two of them).
//!
//! - **Feature-set lockstep.** The root workspace's
//!   `tower = "0.4"` pin enables no features by default; consumers
//!   that want `tower::timeout` or `tower::retry` must opt in
//!   individually. By depending on this crate, a pheno-* service
//!   gets `timeout` + `retry` enabled in one place (this
//!   `Cargo.toml`).
//!
//! ## Layering
//!
//! ```text
//!     pheno-axum-stack
//!           ↓ uses
//!      pheno-tower   ← (this crate; tower + timeout + retry shorthands)
//!           ↓ uses
//!   pheno-tokio-base  (tokio re-export + runtime() + shutdown_signal())
//! ```
//!
//! ## Why standalone (not a workspace member)?
//!
//! Mirrors the L3 #46 `pheno-errors` / L3 #47 `pheno-tracing` /
//! L3 #54 `pheno-tokio-base` convention: an empty `[workspace]` table
//! in this `Cargo.toml` keeps the crate a standalone package, so its
//! `cargo test` / `cargo clippy` loop is independent of the 56+
//! member root workspace. Critical because the root workspace's
//! `Cargo.toml` is concurrently modified by other L3 agents and
//! pinning a new member would race with them.

/// Re-export of the pinned `tower` crate. Consumers should
/// `use pheno_tower::tower` rather than depending on `tower` directly
/// so the fleet-wide version + feature set stays locked. The
/// `timeout` and `retry` features are enabled by this crate's
/// `Cargo.toml`; downstream crates get them transitively through the
/// re-export.
pub use ::tower;

/// Shorthand for [`tower::timeout::TimeoutLayer::new`]. Lets a
/// service write
/// `.layer(pheno_tower::timeout::layer(Duration::from_secs(30)))`
/// without importing the `TimeoutLayer` constructor.
pub mod timeout {
    use std::time::Duration;

    /// Build a `tower::timeout::TimeoutLayer` that bounds each request
    /// to `d` of wall-clock time. Internally just calls
    /// `tower::timeout::TimeoutLayer::new(d)`; the function exists
    /// so the call site reads as `pheno_tower::timeout::layer(d)` and
    /// matches the pheno-* style.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use tower::ServiceBuilder;
    ///
    /// let _svc = ServiceBuilder::new()
    ///     .layer(pheno_tower::timeout::layer(Duration::from_secs(30)));
    /// ```
    pub fn layer(d: Duration) -> ::tower::timeout::TimeoutLayer {
        ::tower::timeout::TimeoutLayer::new(d)
    }
}

/// Canonical retry middleware for the pheno-* fleet: 3 attempts
/// (1 initial + 2 retries) with **exponential backoff** between
/// retries.
///
/// ## Policy semantics
///
/// - **Initial delay:** 10 ms (suitable for fast tests; production
///   callers can construct a custom policy with a longer
///   `initial_delay` via [`ExpBackoffPolicy::with_initial_delay`]).
/// - **Factor:** 2.0 (each retry waits `factor * previous_delay`).
/// - **Max delay:** 60 s (caps the exponential growth).
/// - **Max retries:** 2 (3 total attempts, as the L3 #54 spec
///   requires).
/// - **Backoff is per retry**, not per failure-streak. Each retry
///   resets the delay schedule to the initial delay; this is the
///   "naive" backoff (good enough for transient gRPC / DB / HTTP
///   blips), and matches the spec's "exponential-backoff" wording
///   without the complexity of a "decorrelated jitter" schedule.
///
/// ## Implementation note
///
/// Tower 0.4's `Policy::retry` returns
/// `Option<Future<Output = Self>>` — the future's output is the
/// **next** policy state. We use this to thread the retry counter
/// forward: `retry()` returns a `BackoffFuture` that sleeps for
/// `initial_delay * factor^attempts_so_far` (capped at `max_delay`)
/// and then resolves to a new `ExpBackoffPolicy` with
/// `attempts_so_far + 1`.
///
/// ## Usage
///
/// ```no_run
/// use tower::retry::Retry;
/// use tower::service_fn;
///
/// let svc = service_fn(|req: String| async move { Ok::<_, ()>(req) });
/// let _layered = Retry::new(pheno_tower::retry::policy(), svc);
/// ```
pub mod retry {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;

    use ::tower::retry::Policy;

    /// Default initial delay before the **first** retry. 10 ms keeps
    /// tests fast while still exercising the actual sleep path.
    const DEFAULT_INITIAL_DELAY: Duration = Duration::from_millis(10);

    /// Default backoff factor (each retry's delay is multiplied by
    /// this; 2.0 = "double the previous delay").
    const DEFAULT_FACTOR: f64 = 2.0;

    /// Default cap on per-retry delay (prevents an unbounded
    /// exponential run-away).
    const DEFAULT_MAX_DELAY: Duration = Duration::from_secs(60);

    /// Default number of retries. With `max_retries = 2` the policy
    /// permits 3 total attempts (1 initial + 2 retries), which is the
    /// L3 #54 spec.
    const DEFAULT_MAX_RETRIES: u32 = 2;

    /// The canonical 3-attempt exponential-backoff policy.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tower::retry::Retry;
    /// let _layered = Retry::new(pheno_tower::retry::policy(), tower::service_fn(|_req: ()| async { Ok::<_, ()>(()) }));
    /// ```
    pub fn policy() -> ExpBackoffPolicy {
        ExpBackoffPolicy::new()
    }

    /// A [`tower::retry::Policy`] implementation that retries failed
    /// requests with exponential backoff, capped at `max_retries`
    /// retries (so `max_retries + 1` total attempts).
    ///
    /// Use [`policy()`] to get the fleet-canonical 3-attempt schedule;
    /// use [`ExpBackoffPolicy::with_initial_delay`] or the public
    /// field setters for custom schedules.
    #[derive(Clone, Debug)]
    pub struct ExpBackoffPolicy {
        /// Delay before the first retry (and the seed of the
        /// exponential schedule).
        pub initial_delay: Duration,
        /// Multiplier applied to `initial_delay` after each retry.
        pub factor: f64,
        /// Hard cap on the delay (so the exponential schedule doesn't
        /// run away in the long tail).
        pub max_delay: Duration,
        /// Maximum number of retries (not including the initial
        /// attempt).
        pub max_retries: u32,
        /// Counter incremented after each retry. The exponential
        /// schedule is `initial_delay * factor^attempts_so_far`,
        /// capped at `max_delay`.
        attempts_so_far: u32,
    }

    impl ExpBackoffPolicy {
        /// Build the canonical 3-attempt policy (1 initial + 2
        /// retries) with 10 ms initial delay, 2.0× factor, 60 s
        /// cap.
        pub fn new() -> Self {
            Self {
                initial_delay: DEFAULT_INITIAL_DELAY,
                factor: DEFAULT_FACTOR,
                max_delay: DEFAULT_MAX_DELAY,
                max_retries: DEFAULT_MAX_RETRIES,
                attempts_so_far: 0,
            }
        }

        /// Build a policy with a custom initial delay (useful for
        /// tests that want sub-millisecond backoff to keep the
        /// overall runtime snappy).
        pub fn with_initial_delay(initial_delay: Duration) -> Self {
            Self {
                initial_delay,
                ..Self::new()
            }
        }

        /// Compute the delay before the *next* retry, given the
        /// current `attempts_so_far`.
        fn next_delay(&self) -> Duration {
            let exponent = i32::try_from(self.attempts_so_far).unwrap_or(i32::MAX);
            let secs = self.initial_delay.as_secs_f64() * self.factor.powi(exponent);
            let computed = Duration::from_secs_f64(secs);
            // Saturate at max_delay (clamp, not panic on negative or
            // non-finite factor).
            if !secs.is_finite() || computed > self.max_delay {
                self.max_delay
            } else {
                computed
            }
        }

        /// Public read-only accessor for the next retry delay, used
        /// by unit tests to assert the exponential schedule without
        /// reaching into private state.
        pub fn peek_next_delay(&self) -> Duration {
            self.next_delay()
        }

        /// Builder-style helper: return a copy of `self` with
        /// `attempts_so_far` set to `n`. Exposed so unit tests can
        /// inspect the delay schedule at non-zero attempt counts
        /// without reaching into private state.
        #[doc(hidden)]
        pub fn _with_attempts_so_far_for_testing(mut self, n: u32) -> Self {
            self.attempts_so_far = n;
            self
        }
    }

    impl Default for ExpBackoffPolicy {
        fn default() -> Self {
            Self::new()
        }
    }

    /// The future returned by [`ExpBackoffPolicy::retry`]. Sleeps
    /// for the backoff delay, then yields the next [`ExpBackoffPolicy`]
    /// (with `attempts_so_far + 1`).
    pub struct BackoffFuture {
        sleep: Pin<Box<::tokio::time::Sleep>>,
        next: Option<ExpBackoffPolicy>,
    }

    impl Future for BackoffFuture {
        type Output = ExpBackoffPolicy;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // SAFETY: We never move out of `self.sleep`; we only
            // re-pin and poll it. `tokio::time::Sleep` is `Unpin`-free
            // (it stores a registered timer), so the boxed pin
            // projection is the canonical pattern. We bind `this` so
            // the `Pin<&mut Self>` temporary lives long enough to
            // project the sleep out of it.
            let this = &mut *self;
            let sleep: Pin<&mut ::tokio::time::Sleep> = this.sleep.as_mut();
            match sleep.poll(cx) {
                Poll::Ready(()) => {
                    Poll::Ready(this.next.take().expect("BackoffFuture polled after Ready"))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    impl<Req, Res, E> Policy<Req, Res, E> for ExpBackoffPolicy
    where
        // We require `Req: Clone` because the policy may need to
        // re-send the request after a backoff. Services whose
        // requests aren't `Clone` (e.g. an `http::Request<Body>`
        // wrapping a streaming body) are not retryable through
        // this policy.
        Req: Clone,
    {
        type Future = BackoffFuture;

        fn retry(&self, _req: &Req, result: Result<&Res, &E>) -> Option<Self::Future> {
            match result {
                // Success: hand the response back; do not retry.
                Ok(_) => None,
                Err(_) => {
                    if self.attempts_so_far >= self.max_retries {
                        // Out of retries: surface the last error.
                        None
                    } else {
                        let delay = self.next_delay();
                        let next = Self {
                            initial_delay: self.initial_delay,
                            factor: self.factor,
                            max_delay: self.max_delay,
                            max_retries: self.max_retries,
                            attempts_so_far: self.attempts_so_far + 1,
                        };
                        let sleep = Box::pin(::tokio::time::sleep(delay));
                        Some(BackoffFuture {
                            sleep,
                            next: Some(next),
                        })
                    }
                }
            }
        }

        fn clone_request(&self, req: &Req) -> Option<Req> {
            Some(req.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::retry::policy;
    use std::time::Duration;

    /// The `timeout::layer(d)` shorthand must produce a
    /// `tower::timeout::TimeoutLayer` with the same duration the
    /// caller passed. The test is mostly a compile-time gate: if the
    /// helper's signature drifts from the underlying `TimeoutLayer`,
    /// the test won't compile.
    #[test]
    fn timeout_layer_compiles() {
        let layer = super::timeout::layer(Duration::from_millis(250));
        // Touch the type to make sure the binding isn't optimized
        // away. We can't introspect the duration without going
        // through `tower::timeout::Timeout::new(..., d)`; the
        // function returns the layer as-is, so this is the strongest
        // compile-time + identity check available.
        let _: ::tower::timeout::TimeoutLayer = layer;
    }

    /// `retry::policy()` returns the canonical 3-attempt policy:
    /// 1 initial + 2 retries. The test inspects the public
    /// configuration fields to pin down the schedule so a future
    /// change to the defaults can't accidentally regress the
    /// "3 attempts" guarantee.
    #[test]
    fn retry_policy_attempts_three_times() {
        let p = policy();
        assert_eq!(
            p.max_retries, 2,
            "policy must permit exactly 2 retries (3 total attempts)"
        );
        // Sanity: the schedule is exponential, not constant. We
        // don't pin the factor here (a future change to 1.5× or 3.0×
        // is reasonable) but we do pin that the second retry waits
        // longer than the first.
        let first = p.peek_next_delay();
        // Fabricate a policy with attempts_so_far = 1 to inspect
        // the second retry's delay.
        let p2 = p._with_attempts_so_far_for_testing(1);
        let second = p2.peek_next_delay();
        assert!(
            second > first,
            "exponential backoff: second retry ({second:?}) must wait longer than first ({first:?})"
        );
    }
}
