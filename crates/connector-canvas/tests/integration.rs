//! Integration tests exercising a full Canvas sync against wiremock.
#![allow(clippy::disallowed_methods)]

use std::sync::Arc;

use connector_canvas::auth::{CanvasToken, InMemoryTokenStore};
use connector_canvas::CanvasConnector;
use focus_connectors::{Connector, HealthState};
use serde_json::Value;
use uuid::Uuid;
use wiremock::matchers::{header, method, path, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn load_fixture(name: &str) -> Value {
    let p =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join(name);
    let s = std::fs::read_to_string(p).expect("fixture");
    serde_json::from_str(&s).expect("json")
}

async fn seeded_store(token: &str) -> Arc<InMemoryTokenStore> {
    Arc::new(InMemoryTokenStore::with_token(CanvasToken {
        access_token: token.into(),
        refresh_token: Some("refresh".into()),
        expires_at: None,
    }))
}

#[tokio::test]
async fn full_sync_emits_course_assignment_submission_events() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/users/self/courses$"))
        .and(header("authorization", "Bearer ACC"))
        .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("courses.json")))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/courses/101/assignments$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("assignments.json")))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/courses/202/assignments$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/courses/101/assignments/9001/submissions$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("submissions.json")))
        .mount(&server)
        .await;

    let store = seeded_store("ACC").await;
    let conn =
        CanvasConnector::builder(server.uri()).account_id(Uuid::nil()).token_store(store).build();

    let out = conn.sync(None).await.expect("sync ok");
    // 2 courses enrolled + 1 assignment + 1 submission = 4.
    assert_eq!(out.events.len(), 4);
    let kinds: Vec<_> = out.events.iter().map(|e| format!("{:?}", e.event_type)).collect();
    assert!(kinds.iter().any(|k| k.contains("CourseEnrolled")));
    assert!(kinds.iter().any(|k| k.contains("AssignmentDue")));
    assert!(kinds.iter().any(|k| k.contains("AssignmentGraded")));
}

#[tokio::test]
async fn pagination_cursor_is_surfaced() {
    let server = MockServer::start().await;
    let base = server.uri();
    let next_url = format!("{base}/api/v1/users/self/courses?page=2");
    let link_hdr = format!("<{next_url}>; rel=\"next\"");

    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/users/self/courses$"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Link", link_hdr.as_str())
                .set_body_json(serde_json::json!([])),
        )
        .mount(&server)
        .await;

    let conn = CanvasConnector::builder(&base).token_store(seeded_store("ACC").await).build();

    let out = conn.sync(None).await.unwrap();
    assert_eq!(out.next_cursor.as_deref(), Some(next_url.as_str()));
}

#[tokio::test]
async fn health_healthy_when_self_returns_200() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/users/self"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": 1, "name": "test"})),
        )
        .mount(&server)
        .await;

    let conn =
        CanvasConnector::builder(server.uri()).token_store(seeded_store("ACC").await).build();
    assert_eq!(conn.health().await, HealthState::Healthy);
}

#[tokio::test]
async fn health_unauthenticated_when_no_token() {
    let server = MockServer::start().await;
    let conn = CanvasConnector::builder(server.uri()).build();
    assert_eq!(conn.health().await, HealthState::Unauthenticated);
}

#[tokio::test]
async fn sync_refreshes_on_401_then_succeeds() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let server = MockServer::start().await;

    // Token refresh endpoint — returns a fresh access token "NEW".
    Mock::given(method("POST"))
        .and(path("/login/oauth2/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "NEW",
            "token_type": "Bearer",
            "expires_in": 3600
        })))
        .mount(&server)
        .await;

    // First call to courses with OLD token → 401, second with NEW → 200.
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_resp = counter.clone();
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/users/self/courses$"))
        .respond_with(move |_req: &wiremock::Request| {
            let n = counter_resp.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                ResponseTemplate::new(401)
            } else {
                ResponseTemplate::new(200).set_body_json(serde_json::json!([]))
            }
        })
        .mount(&server)
        .await;

    let oauth_cfg = connector_canvas::auth::CanvasAuthConfig {
        client_id: "cid".into(),
        client_secret: "secret".into(),
        base_url: server.uri(),
        redirect_uri: "http://localhost/cb".into(),
    };
    let oauth = Arc::new(connector_canvas::auth::CanvasOAuth2::new(oauth_cfg).unwrap());

    let conn = CanvasConnector::builder(server.uri())
        .token_store(seeded_store("OLD").await)
        .oauth(oauth)
        .build();

    let out = conn.sync(None).await.expect("retry succeeds");
    assert_eq!(out.events.len(), 0);
    assert!(counter.load(Ordering::SeqCst) >= 2);
}
