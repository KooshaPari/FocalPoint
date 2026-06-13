//! # pheno-axum-stack
//!
//! Canonical Axum HTTP stack for the `pheno-*` fleet. Bundles the three
//! pieces every pheno-* HTTP service ends up depending on — **axum**,
//! **tower**, and **tower-http** — and exposes two opinionated helpers
//! that are easy to forget to add but expensive to miss:
//!
//! 1. [`router()`] — a `Router` with a `/healthz` route that always
//!    returns `200 OK` with body `ok`. The L3 #54 spec turns this into
//!    the fleet-wide health endpoint so orchestrators (k8s liveness /
//!    readiness probes, ALB target groups) can probe any pheno-*
//!    service without knowing its domain.
//!
//! 2. [`with_request_id(router)`] — applies the **X-Request-ID echo
//!    middleware**: if the inbound request carries an `X-Request-ID`
//!    header (which upstream proxies / API gateways / sidecars
//!    typically inject), the response mirrors it back on the same
//!    header. This lets the caller correlate a 5xx response with the
//!    server log line via the same opaque id.
//!
//! ## Re-exports
//!
//! ```text
//!     axum           — HTTP framework
//!     tower          — `Service` trait + middleware combinators
//!     tower_http     — production-grade middleware:
//!                        * `cors`     — CorsLayer (CORS preflight + headers)
//!                        * `trace`    — TraceLayer (HTTP-level structured logs)
//!                        * `timeout`  — TimeoutLayer (request deadline)
//! ```
//!
//! ## Layering
//!
//! ```text
//!     pheno-axum-stack   ← (this crate; axum + tower + tower-http + router + request-id)
//!           ↓ uses
//!      pheno-tower       (tower::retry / tower::timeout shorthands)
//!           ↓ uses
//!   pheno-tokio-base     (tokio re-export + runtime() + shutdown_signal())
//! ```
//!
//! A typical pheno-* binary depends on **all three** crates and wires
//! them up as:
//!
//! ```no_run
//! use std::net::SocketAddr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app = pheno_axum_stack::with_request_id(pheno_axum_stack::router())
//!         .layer(pheno_axum_stack::tower_http::trace::TraceLayer::new_for_http());
//!     let addr: SocketAddr = "0.0.0.0:8080".parse()?;
//!     let listener = tokio::net::TcpListener::bind(addr).await?;
//!     axum::serve(listener, app).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Why standalone (not a workspace member)?
//!
//! Mirrors the L3 #46 `pheno-errors` / L3 #47 `pheno-tracing` / L3 #54
//! `pheno-tokio-base` convention: an empty `[workspace]` table in this
//! `Cargo.toml` keeps the crate a standalone package, so its `cargo
//! test` / `cargo clippy` loop is independent of the 56+ member root
//! workspace.

/// Re-export of the pinned `axum` crate. Consumers should `use
/// pheno_axum_stack::axum` rather than depending on `axum` directly
/// so the fleet-wide version + feature set stays locked.
pub use ::axum;

/// Re-export of the pinned `tower` crate. Exposed at the top level so
/// downstream crates can `use pheno_axum_stack::tower::{Service,
/// ServiceExt, Layer}` without a second `Cargo.toml` dep on `tower`.
pub use ::tower;

/// Re-export of the pinned `tower-http` crate, with the
/// `cors` / `trace` / `timeout` features enabled. Downstream crates
/// reach the canonical middleware set via
/// `pheno_axum_stack::tower_http::trace::TraceLayer` etc.
pub use ::tower_http;

/// Build the canonical pheno-* `axum::Router`.
///
/// The router currently exposes a single route, **`/healthz`**, that
/// returns `200 OK` with body `ok`. This is intentionally minimal: the
/// `Router` is meant to be **composed into** a service-specific router
/// (e.g. via `merge`, `nest`, or `route`) rather than used standalone,
/// so the only thing it owns is the fleet-wide health endpoint.
///
/// # Example
///
/// ```no_run
/// let app = pheno_axum_stack::with_request_id(pheno_axum_stack::router());
/// let _ = app; // hand to axum::serve(listener, app)
/// ```
pub fn router() -> ::axum::Router {
    use ::axum::routing::get;
    ::axum::Router::new().route("/healthz", get(healthz))
}

/// The `/healthz` handler. Returns `200 OK` with body `"ok"`.
///
/// Marked `pub` (crate-internal) so the test suite can also reach it
/// directly when an in-process `axum::Router` is inconvenient; in
/// production the canonical path is the `router()` builder above.
pub async fn healthz() -> &'static str {
    "ok"
}

/// Canonical header name used by [`with_request_id`]'s echo
/// middleware. `x-request-id` is the de-facto standard across the
/// fleet (matches the OpenTelemetry `http.request.header.x-request-id`
/// semantic convention, Envoy's `x-request-id`, and AWS ALB's
/// `X-Amzn-Trace-Id` sibling).
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Apply the X-Request-ID echo middleware to `router`.
///
/// If the inbound request carries an `X-Request-ID` header, the
/// response will mirror that exact value back on the same header. If
/// the request does **not** carry the header, the response will not
/// add it — this is an **echo** middleware, not a generator.
///
/// The implementation is a thin `axum::middleware::from_fn` wrapper
/// that reads the request's `x-request-id` header and, on the
/// response, copies it through. The middleware is intentionally
/// allocation-free in the common path (no UUID generation, no header
/// parsing beyond the standard `http::HeaderName` lookup).
///
/// # Why not `tower_http::request_id::SetRequestIdLayer` +
/// `PropagateRequestIdLayer`?
///
/// The `tower-http` request-id layers *generate* a UUID when the
/// request lacks an id. That is the right behavior for a public
/// ingress, but the wrong behavior for an internal service: an
/// internal service that auto-generates an id silently overwrites the
/// id the upstream gateway / sidecar injected, breaking
/// end-to-end tracing. The echo-only behavior this helper provides
/// preserves the upstream-injected id verbatim, or adds nothing if
/// the upstream did not inject one. This is the property the
/// `with_request_id_echoes_header` test pins down.
///
/// # Example
///
/// ```no_run
/// let app = pheno_axum_stack::with_request_id(pheno_axum_stack::router());
/// let _ = app;
/// ```
pub fn with_request_id(router: ::axum::Router) -> ::axum::Router {
    router.layer(::axum::middleware::from_fn(echo_request_id))
}

/// The actual middleware function. Async (axum 0.7 middleware fn
/// signature), copies the `X-Request-ID` header from request to
/// response, then runs the inner service. On error the response is
/// passed through unchanged so the inner service's error mapping is
/// not disturbed.
async fn echo_request_id(
    req: ::axum::http::Request<::axum::body::Body>,
    next: ::axum::middleware::Next,
) -> ::axum::response::Response {
    // Borrow the header value (it's a `HeaderValue`, cheap to clone).
    let inbound_id = req.headers().get(REQUEST_ID_HEADER).cloned();
    let mut response = next.run(req).await;
    if let Some(value) = inbound_id {
        // Only set the header if the inner service didn't already
        // populate it (a service-specific request-id middleware that
        // runs *inside* this one wins over the echo).
        if !response.headers().contains_key(REQUEST_ID_HEADER) {
            response.headers_mut().insert(REQUEST_ID_HEADER, value);
        }
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `/healthz` returns 200 "ok" on the router returned by
    /// `router()`. Driven via `tower::ServiceExt::oneshot` so no real
    /// port is bound.
    #[tokio::test]
    async fn healthz_returns_200() {
        use ::tower::ServiceExt;
        let app = router();
        let response = app
            .oneshot(
                ::axum::http::Request::builder()
                    .uri("/healthz")
                    .body(::axum::body::Body::empty())
                    .expect("build request"),
            )
            .await
            .expect("router should respond");
        assert_eq!(response.status(), ::axum::http::StatusCode::OK);
        let body = response.into_body();
        let bytes = http_body_util::BodyExt::collect(body)
            .await
            .expect("collect body")
            .to_bytes();
        assert_eq!(bytes.as_ref(), b"ok", "healthz body must be exactly \"ok\"");
    }

    /// `with_request_id` echoes a request's `X-Request-ID` header
    /// back on the response.
    #[tokio::test]
    async fn with_request_id_echoes_header() {
        use ::tower::ServiceExt;
        let app = with_request_id(router());
        let response = app
            .oneshot(
                ::axum::http::Request::builder()
                    .uri("/healthz")
                    .header(REQUEST_ID_HEADER, "test-req-id-12345")
                    .body(::axum::body::Body::empty())
                    .expect("build request"),
            )
            .await
            .expect("router should respond");
        assert_eq!(response.status(), ::axum::http::StatusCode::OK);
        let echoed = response
            .headers()
            .get(REQUEST_ID_HEADER)
            .expect("response must echo X-Request-ID");
        assert_eq!(
            echoed.as_bytes(),
            b"test-req-id-12345",
            "X-Request-ID must be echoed verbatim"
        );
    }

    /// `router()` accepts multiple concurrent in-flight requests. This
    /// is a smoke test for the multi-threaded runtime path that
    /// `pheno-tokio-base::runtime()` exposes: each request is fanned
    /// out to a worker via `tokio::spawn` and the join handles are
    /// awaited in parallel. All 8 must succeed.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn router_accepts_concurrent_requests() {
        use ::tower::ServiceExt;
        // We share a single Router across many tasks. `Router` is
        // cheaply `Clone` (it's an `Arc` internally) and `ServiceExt::
        // oneshot` takes `self`, so cloning once per request is the
        // canonical pattern.
        let app = router();
        let mut handles = Vec::new();
        for i in 0..8 {
            let app = app.clone();
            handles.push(tokio::spawn(async move {
                let response = app
                    .oneshot(
                        ::axum::http::Request::builder()
                            .uri("/healthz")
                            .header("x-test-iteration", i.to_string())
                            .body(::axum::body::Body::empty())
                            .expect("build request"),
                    )
                    .await
                    .expect("router should respond under concurrency");
                assert_eq!(
                    response.status(),
                    ::axum::http::StatusCode::OK,
                    "concurrent request {i} must return 200"
                );
                i
            }));
        }
        for (i, h) in handles.into_iter().enumerate() {
            let n = h.await.expect("task must not panic");
            assert_eq!(n, i, "task {i} returned its own iteration index");
        }
    }
}
