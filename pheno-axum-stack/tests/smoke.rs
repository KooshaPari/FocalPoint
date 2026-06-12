//! Integration tests for `pheno-axum-stack`.
//!
//! These tests live in `tests/` (as opposed to `src/lib.rs`) so they
//! exercise the crate's **public** API the same way a downstream
//! consumer would: by depending on the crate by name and calling its
//! items. This catches accidental `pub(crate)` or undocumented
//! internal-only paths that would silently work for in-module tests.

use pheno_axum_stack::axum;
use pheno_axum_stack::tower::ServiceExt;

/// `/healthz` returns 200 "ok" on the router returned by `router()`.
#[tokio::test]
async fn healthz_returns_200() {
    let app = pheno_axum_stack::router();
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/healthz")
                .body(axum::body::Body::empty())
                .expect("build request"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body_bytes = http_body_util::BodyExt::collect(response.into_body())
        .await
        .expect("collect body")
        .to_bytes();
    assert_eq!(
        body_bytes.as_ref(),
        b"ok",
        "healthz body must be exactly \"ok\""
    );
}

/// `with_request_id` echoes a request's `X-Request-ID` header back on
/// the response verbatim, and does NOT add a header when the request
/// lacks one (echo semantics, not generator semantics).
#[tokio::test]
async fn with_request_id_echoes_header() {
    let app = pheno_axum_stack::with_request_id(pheno_axum_stack::router());
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/healthz")
                .header(pheno_axum_stack::REQUEST_ID_HEADER, "external-req-abc-987")
                .body(axum::body::Body::empty())
                .expect("build request"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let echoed = response
        .headers()
        .get(pheno_axum_stack::REQUEST_ID_HEADER)
        .expect("response must echo X-Request-ID");
    assert_eq!(echoed.as_bytes(), b"external-req-abc-987");
}

/// `with_request_id` does NOT add a header when the request did not
/// carry one (echo semantics). This is the property that distinguishes
/// the helper from `tower_http::request_id::SetRequestIdLayer`, which
/// auto-generates a UUID.
#[tokio::test]
async fn with_request_id_omits_header_when_request_lacks_it() {
    let app = pheno_axum_stack::with_request_id(pheno_axum_stack::router());
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/healthz")
                .body(axum::body::Body::empty())
                .expect("build request"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert!(
        !response
            .headers()
            .contains_key(pheno_axum_stack::REQUEST_ID_HEADER),
        "echo middleware must NOT auto-generate a request id when the request lacks one"
    );
}

/// `router()` accepts many concurrent in-flight requests without
/// dropping or serializing them. Drives the multi-threaded runtime
/// path exposed by `pheno-tokio-base::runtime()`.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn router_accepts_concurrent_requests() {
    let app = pheno_axum_stack::router();
    let mut handles = Vec::new();
    for i in 0..16 {
        let app = app.clone();
        handles.push(tokio::spawn(async move {
            let response = app
                .oneshot(
                    axum::http::Request::builder()
                        .uri("/healthz")
                        .body(axum::body::Body::empty())
                        .expect("build request"),
                )
                .await
                .expect("router should respond under concurrency");
            assert_eq!(
                response.status(),
                axum::http::StatusCode::OK,
                "concurrent request {i} must return 200"
            );
            i
        }));
    }
    for (i, h) in handles.into_iter().enumerate() {
        let n = h.await.expect("task must not panic");
        assert_eq!(n, i);
    }
}
