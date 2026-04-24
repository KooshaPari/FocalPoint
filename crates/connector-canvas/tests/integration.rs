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
        issued_at: chrono::Utc::now(),
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
    // 2 courses enrolled + 1 assignment + 1 submission + 1 grade_posted = 5.
    // (Announcement endpoint not mocked; sync warns + continues.)
    assert_eq!(out.events.len(), 5);
    let kinds: Vec<_> = out.events.iter().map(|e| format!("{:?}", e.event_type)).collect();
    assert!(kinds.iter().any(|k| k.contains("CourseEnrolled")));
    assert!(kinds.iter().any(|k| k.contains("AssignmentDue")));
    assert!(kinds.iter().any(|k| k.contains("AssignmentGraded")));
    assert!(kinds.iter().any(|k| k.contains("grade_posted")));
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

// ============================================================================
// NEW ENDPOINT TESTS (6 HIGH-priority missing endpoints)
// ============================================================================

#[tokio::test]
async fn get_user_profile_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/users/self"))
        .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("user_profile.json")))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "TOKEN");
    let profile = client.get_user_profile().await.unwrap();
    assert_eq!(profile.name, "Alice Student");
    assert_eq!(profile.email, Some("alice@example.edu".into()));
    assert_eq!(profile.avatar_url, Some("https://canvas.example.com/images/avatars/42.png".into()));
    assert_eq!(profile.locale, Some("en".into()));
}

#[tokio::test]
async fn get_user_profile_401_unauthorized() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/users/self"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "BADTOKEN");
    let err = client.get_user_profile().await.unwrap_err();
    assert!(matches!(err, focus_connectors::ConnectorError::Auth(_)));
}

#[tokio::test]
async fn get_course_progress_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/users/self/courses/101/progress$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("course_progress.json")))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "TOKEN");
    let progress = client.get_course_progress(101, None).await.unwrap();
    assert_eq!(progress.progress, Some(75.5));
    assert_eq!(progress.requirement_count, Some(10));
    assert_eq!(progress.requirement_completed_count, Some(8));
}

#[tokio::test]
async fn get_course_progress_403_permission_denied() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/users/.+/courses/999/progress$"))
        .respond_with(ResponseTemplate::new(403).set_body_string("User lacks permission to view progress"))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "TOKEN");
    let err = client.get_course_progress(999, None).await.unwrap_err();
    assert!(matches!(err, focus_connectors::ConnectorError::Auth(_)));
}

#[tokio::test]
async fn list_students_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/courses/101/users$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("enrollments.json")))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "TOKEN");
    let page = client.list_students(101, None).await.unwrap();
    assert_eq!(page.items.len(), 2);
    assert_eq!(page.items[0].user_id, Some(10));
    assert_eq!(page.items[0].user.as_ref().unwrap().name, "Bob Student");
    assert_eq!(page.items[1].user_id, Some(11));
}

#[tokio::test]
async fn list_students_403_requires_teacher_permission() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/courses/101/users$"))
        .respond_with(ResponseTemplate::new(403).set_body_string("This user lacks permission to view students"))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "STUDENT_TOKEN");
    let err = client.list_students(101, None).await.unwrap_err();
    assert!(matches!(err, focus_connectors::ConnectorError::Auth(_)));
}

#[tokio::test]
async fn get_assignment_single_detail() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/courses/101/assignments/9001$"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 9001,
                "name": "HW1",
                "course_id": 101,
                "due_at": "2026-05-01T23:59:00Z",
                "submission_types": ["online_upload"],
                "points_possible": 100.0,
                "description": "Complete exercises 1-10",
                "published": true
            })),
        )
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "TOKEN");
    let assign = client.get_assignment(101, 9001).await.unwrap();
    assert_eq!(assign.id, 9001);
    assert_eq!(assign.name, "HW1");
    assert_eq!(assign.points_possible, Some(100.0));
}

#[tokio::test]
async fn get_assignment_404_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/courses/101/assignments/99999$"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "TOKEN");
    let err = client.get_assignment(101, 99999).await.unwrap_err();
    assert!(matches!(err, focus_connectors::ConnectorError::Network(_)));
}

#[tokio::test]
async fn get_user_grades_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/users/self/enrollments$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("user_grades.json")))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "TOKEN");
    let grades = client.get_user_grades().await.unwrap();
    assert_eq!(grades.len(), 2);
    assert_eq!(grades[0].current_score, Some(92.5));
    assert_eq!(grades[0].current_grade, Some("A-".into()));
    assert_eq!(grades[1].final_score, None);
}

#[tokio::test]
async fn get_user_grades_401_unauthorized() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/users/self/enrollments$"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "BADTOKEN");
    let err = client.get_user_grades().await.unwrap_err();
    assert!(matches!(err, focus_connectors::ConnectorError::Auth(_)));
}

#[tokio::test]
async fn list_calendar_events_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/calendar_events$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("calendar_events.json")))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "TOKEN");
    let page = client.list_calendar_events(101, None).await.unwrap();
    assert_eq!(page.items.len(), 2);
    assert_eq!(page.items[0].title, "Midterm Exam");
    assert_eq!(page.items[0].assignment_id, None);
    assert_eq!(page.items[1].title, "Final Project Due");
    assert_eq!(page.items[1].assignment_id, Some(9002));
}

#[tokio::test]
async fn list_calendar_events_401_unauthorized() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/api/v1/calendar_events$"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = connector_canvas::api::CanvasClient::new(server.uri(), "BADTOKEN");
    let err = client.list_calendar_events(101, None).await.unwrap_err();
    assert!(matches!(err, focus_connectors::ConnectorError::Auth(_)));
}
