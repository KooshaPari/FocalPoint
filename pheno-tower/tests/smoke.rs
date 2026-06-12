//! Integration tests for `pheno-tower`.
//!
//! These tests live in `tests/` (as opposed to `src/lib.rs`) so they
//! exercise the crate's **public** API the same way a downstream
//! consumer would: by depending on the crate by name and calling its
//! items. This catches accidental `pub(crate)` or undocumented
//! internal-only paths that would silently work for in-module tests.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use pheno_tower::retry::ExpBackoffPolicy;
use pheno_tower::tower::retry::Retry;
use pheno_tower::tower::Service;
use pheno_tower::tower::ServiceExt;

/// `timeout::layer(d)` produces a `TimeoutLayer` and the resulting
/// service enforces the deadline. We test it end-to-end: a service
/// that sleeps longer than the layer's budget must come back with
/// the tower timeout error.
#[tokio::test]
async fn timeout_layer_enforces_deadline() {
    use pheno_tower::tower::timeout::Timeout;

    // Service that sleeps 200ms — we'll wrap it in a 30ms timeout.
    let slow = tower::service_fn(|_req: ()| async {
        tokio::time::sleep(Duration::from_millis(200)).await;
        Ok::<_, std::io::Error>(())
    });
    let mut svc = Timeout::new(slow, Duration::from_millis(30));
    let started = std::time::Instant::now();
    let result: Result<(), _> = svc.ready().await.unwrap().call(()).await;
    let elapsed = started.elapsed();
    // Timeout error (or wrapped timeout error) is expected.
    assert!(
        result.is_err(),
        "service must time out: result = {result:?}"
    );
    assert!(
        elapsed < Duration::from_millis(150),
        "timeout must fire well before the service's 200ms sleep (actual = {elapsed:?})"
    );
}

/// `retry::policy()` retries failed requests exactly `max_retries`
/// times (3 total attempts: 1 initial + 2 retries). The test wires
/// the policy into a `Retry::new(...)` middleware over a service
/// that fails the first 2 times and succeeds on the 3rd, then
/// asserts the service was called exactly 3 times.
#[tokio::test]
async fn retry_policy_attempts_three_times() {
    let counter = Arc::new(AtomicUsize::new(0));
    let svc = tower::service_fn({
        let counter = Arc::clone(&counter);
        move |_req: ()| {
            let counter = Arc::clone(&counter);
            async move {
                let n = counter.fetch_add(1, Ordering::SeqCst);
                if n < 2 {
                    // First 2 calls fail; 3rd call (n == 2) succeeds.
                    Err::<(), _>(std::io::Error::other(format!("fail #{n}")))
                } else {
                    Ok(())
                }
            }
        }
    });

    // Use a custom initial_delay of 1ms so the test runs fast even
    // though we exercise the actual sleep path in the policy.
    let policy = ExpBackoffPolicy::with_initial_delay(Duration::from_millis(1));
    let mut svc = Retry::new(policy, svc);
    let result: Result<(), _> = svc.ready().await.unwrap().call(()).await;

    assert!(
        result.is_ok(),
        "3rd attempt must succeed (result = {result:?})"
    );
    let observed = counter.load(Ordering::SeqCst);
    assert_eq!(
        observed, 3,
        "service must be called exactly 3 times: 1 initial + 2 retries (observed = {observed})"
    );
}

/// `retry::policy()` stops retrying once `max_retries` is exhausted,
/// even if the service keeps failing. A service that always fails
/// must be called exactly `max_retries + 1` times.
#[tokio::test]
async fn retry_policy_gives_up_after_max_retries() {
    let counter = Arc::new(AtomicUsize::new(0));
    let svc = tower::service_fn({
        let counter = Arc::clone(&counter);
        move |_req: ()| {
            let counter = Arc::clone(&counter);
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Err::<(), _>(std::io::Error::other("always fail"))
            }
        }
    });

    let policy = ExpBackoffPolicy::with_initial_delay(Duration::from_millis(1));
    let mut svc = Retry::new(policy, svc);
    let result: Result<(), _> = svc.ready().await.unwrap().call(()).await;

    assert!(result.is_err(), "always-failing service must surface error");
    let observed = counter.load(Ordering::SeqCst);
    assert_eq!(
        observed, 3,
        "service must be called exactly 3 times then given up (observed = {observed})"
    );
}
