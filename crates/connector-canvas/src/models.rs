//! Canvas LMS domain types. JSON field names match Canvas API.
//!
//! Optional fields lean heavily on `#[serde(default)]`: Canvas API responses
//! vary by endpoint, account permissions, and feature flags. Fields that are
//! documented as optional OR that Canvas is observed to omit in practice are
//! wrapped in `Option`/`Vec`/`String` defaults rather than required.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Course {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub workflow_state: String,
    #[serde(default)]
    pub enrollment_term_id: Option<u64>,
    #[serde(default)]
    pub course_code: Option<String>,
    #[serde(default)]
    pub start_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub end_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Assignment {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub due_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub unlock_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub lock_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub submission_types: Vec<String>,
    #[serde(default)]
    pub points_possible: Option<f64>,
    /// Canvas omits `course_id` on some listings (e.g.
    /// `/courses/:id/assignments`). The caller threads the parent course id
    /// through the event mapper instead of relying on this field.
    #[serde(default)]
    pub course_id: Option<u64>,
    #[serde(default)]
    pub html_url: Option<String>,
    #[serde(default)]
    pub published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Submission {
    pub id: u64,
    #[serde(default)]
    pub submitted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub graded_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub workflow_state: String,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub grade: Option<String>,
    pub assignment_id: u64,
    #[serde(default)]
    pub user_id: Option<u64>,
    #[serde(default)]
    pub late: Option<bool>,
    #[serde(default)]
    pub missing: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasUser {
    pub id: u64,
    #[serde(default)]
    pub name: String,
}

/// Canvas announcement (a DiscussionTopic with `is_announcement=true`).
/// Fetched from `/api/v1/announcements?context_codes[]=course_<id>`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Announcement {
    pub id: u64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub posted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub delayed_post_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub html_url: Option<String>,
    /// Canvas returns context info in a few shapes; we only surface the
    /// string form when present. The mapper threads the known course id
    /// through instead of trusting this field.
    #[serde(default)]
    pub context_code: Option<String>,
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;

    #[test]
    fn parses_course_json() {
        let j = r#"{"id":42,"name":"Math","workflow_state":"available","enrollment_term_id":7}"#;
        let c: Course = serde_json::from_str(j).unwrap();
        assert_eq!(c.id, 42);
        assert_eq!(c.workflow_state, "available");
    }

    #[test]
    fn parses_assignment_missing_optional() {
        let j = r#"{"id":1,"name":"HW"}"#;
        let a: Assignment = serde_json::from_str(j).unwrap();
        assert!(a.due_at.is_none());
        assert!(a.points_possible.is_none());
        assert!(
            a.course_id.is_none(),
            "course_id must be optional — Canvas omits it on /courses/:id/assignments"
        );
    }

    #[test]
    fn parses_assignment_with_explicit_course_id() {
        let j = r#"{"id":1,"name":"HW","course_id":42}"#;
        let a: Assignment = serde_json::from_str(j).unwrap();
        assert_eq!(a.course_id, Some(42));
    }

    #[test]
    fn parses_submission() {
        let j = r#"{"id":9,"assignment_id":1,"workflow_state":"graded","score":95.0,"submitted_at":"2026-01-01T00:00:00Z"}"#;
        let s: Submission = serde_json::from_str(j).unwrap();
        assert_eq!(s.score, Some(95.0));
        assert!(s.submitted_at.is_some());
    }

    #[test]
    fn parses_announcement_json() {
        let j = r#"{"id":5,"title":"Welcome","message":"<p>Hi</p>","posted_at":"2026-04-01T12:00:00Z"}"#;
        let a: Announcement = serde_json::from_str(j).unwrap();
        assert_eq!(a.id, 5);
        assert_eq!(a.title, "Welcome");
        assert!(a.posted_at.is_some());
    }

    #[test]
    fn parses_announcement_minimal() {
        let j = r#"{"id":6}"#;
        let a: Announcement = serde_json::from_str(j).unwrap();
        assert!(a.title.is_empty());
        assert!(a.posted_at.is_none());
    }
}
