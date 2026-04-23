//! Canvas -> NormalizedEvent mapping.

use chrono::Utc;
use focus_events::{DedupeKey, EventType, NormalizedEvent, TraceRef};
use serde_json::json;
use uuid::Uuid;

use crate::models::{Assignment, Course, Submission};

pub const CONNECTOR_ID: &str = "canvas";

/// Compute a stable dedupe key for a Canvas entity.
pub fn dedupe_key(entity_type: &str, id: u64, timestamp: i64) -> DedupeKey {
    DedupeKey(format!("canvas:{entity_type}:{id}:{timestamp}"))
}

pub struct CanvasEventMapper;

impl CanvasEventMapper {
    pub fn map_assignment(a: &Assignment, account_id: Uuid) -> NormalizedEvent {
        let occurred = a.due_at.unwrap_or_else(Utc::now);
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::AssignmentDue,
            occurred_at: occurred,
            effective_at: occurred,
            dedupe_key: dedupe_key("assignment", a.id, occurred.timestamp()),
            confidence: 1.0,
            payload: json!({
                "assignment_id": a.id,
                "course_id": a.course_id,
                "name": a.name,
                "due_at": a.due_at,
                "points_possible": a.points_possible,
                "submission_types": a.submission_types,
            }),
            raw_ref: Some(TraceRef {
                source: CONNECTOR_ID.into(),
                id: format!("assignment:{}", a.id),
            }),
        }
    }

    pub fn map_submission(s: &Submission, account_id: Uuid) -> NormalizedEvent {
        let occurred = s.submitted_at.unwrap_or_else(Utc::now);
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::AssignmentGraded,
            occurred_at: occurred,
            effective_at: occurred,
            dedupe_key: dedupe_key("submission", s.id, occurred.timestamp()),
            confidence: 1.0,
            payload: json!({
                "submission_id": s.id,
                "assignment_id": s.assignment_id,
                "score": s.score,
                "workflow_state": s.workflow_state,
                "submitted_at": s.submitted_at,
            }),
            raw_ref: Some(TraceRef {
                source: CONNECTOR_ID.into(),
                id: format!("submission:{}", s.id),
            }),
        }
    }

    pub fn map_course_enrolled(c: &Course, account_id: Uuid) -> NormalizedEvent {
        let occurred = Utc::now();
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::CourseEnrolled,
            occurred_at: occurred,
            effective_at: occurred,
            dedupe_key: dedupe_key("course", c.id, occurred.timestamp()),
            confidence: 1.0,
            payload: json!({
                "course_id": c.id,
                "name": c.name,
                "workflow_state": c.workflow_state,
                "enrollment_term_id": c.enrollment_term_id,
            }),
            raw_ref: Some(TraceRef { source: CONNECTOR_ID.into(), id: format!("course:{}", c.id) }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn acct() -> Uuid {
        Uuid::nil()
    }

    #[test]
    fn maps_assignment_with_due_date() {
        let a = Assignment {
            id: 1,
            name: "HW1".into(),
            due_at: Some(Utc.with_ymd_and_hms(2026, 5, 1, 12, 0, 0).unwrap()),
            submission_types: vec!["online_upload".into()],
            points_possible: Some(10.0),
            course_id: 42,
        };
        let ev = CanvasEventMapper::map_assignment(&a, acct());
        assert_eq!(ev.event_type, EventType::AssignmentDue);
        assert_eq!(ev.connector_id, "canvas");
        assert!(ev.dedupe_key.0.starts_with("canvas:assignment:1:"));
        assert_eq!(ev.payload["course_id"], 42);
    }

    #[test]
    fn maps_assignment_without_due_date_uses_now() {
        let a = Assignment {
            id: 2,
            name: "P".into(),
            due_at: None,
            submission_types: vec![],
            points_possible: None,
            course_id: 7,
        };
        let before = Utc::now();
        let ev = CanvasEventMapper::map_assignment(&a, acct());
        assert!(ev.occurred_at >= before - chrono::Duration::seconds(2));
    }

    #[test]
    fn maps_submission() {
        let s = Submission {
            id: 9,
            submitted_at: Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
            workflow_state: "graded".into(),
            score: Some(95.0),
            assignment_id: 1,
        };
        let ev = CanvasEventMapper::map_submission(&s, acct());
        assert_eq!(ev.event_type, EventType::AssignmentGraded);
        assert_eq!(ev.payload["score"], 95.0);
        assert!(ev.dedupe_key.0.starts_with("canvas:submission:9:"));
    }

    #[test]
    fn maps_course_enrolled() {
        let c = Course {
            id: 42,
            name: "Math".into(),
            workflow_state: "available".into(),
            enrollment_term_id: Some(7),
        };
        let ev = CanvasEventMapper::map_course_enrolled(&c, acct());
        assert_eq!(ev.event_type, EventType::CourseEnrolled);
        assert_eq!(ev.payload["course_id"], 42);
    }

    #[test]
    fn dedupe_keys_are_distinct_per_entity() {
        let a = Assignment {
            id: 1,
            name: "".into(),
            due_at: Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
            submission_types: vec![],
            points_possible: None,
            course_id: 1,
        };
        let s = Submission {
            id: 1,
            submitted_at: Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
            workflow_state: "graded".into(),
            score: None,
            assignment_id: 1,
        };
        let ea = CanvasEventMapper::map_assignment(&a, acct());
        let es = CanvasEventMapper::map_submission(&s, acct());
        assert_ne!(ea.dedupe_key, es.dedupe_key);
    }

    #[test]
    fn traces_reference_canvas_ids() {
        let c = Course {
            id: 42,
            name: "".into(),
            workflow_state: "available".into(),
            enrollment_term_id: None,
        };
        let ev = CanvasEventMapper::map_course_enrolled(&c, acct());
        let tr = ev.raw_ref.unwrap();
        assert_eq!(tr.source, "canvas");
        assert_eq!(tr.id, "course:42");
    }
}
