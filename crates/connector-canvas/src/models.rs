//! Canvas LMS domain types. JSON field names match Canvas API.

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Assignment {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub due_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub submission_types: Vec<String>,
    #[serde(default)]
    pub points_possible: Option<f64>,
    pub course_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Submission {
    pub id: u64,
    #[serde(default)]
    pub submitted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub workflow_state: String,
    #[serde(default)]
    pub score: Option<f64>,
    pub assignment_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasUser {
    pub id: u64,
    #[serde(default)]
    pub name: String,
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
        let j = r#"{"id":1,"name":"HW","course_id":42}"#;
        let a: Assignment = serde_json::from_str(j).unwrap();
        assert!(a.due_at.is_none());
        assert!(a.points_possible.is_none());
    }

    #[test]
    fn parses_submission() {
        let j = r#"{"id":9,"assignment_id":1,"workflow_state":"graded","score":95.0,"submitted_at":"2026-01-01T00:00:00Z"}"#;
        let s: Submission = serde_json::from_str(j).unwrap();
        assert_eq!(s.score, Some(95.0));
        assert!(s.submitted_at.is_some());
    }
}
