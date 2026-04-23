//! Canvas -> NormalizedEvent mapping.
#![allow(clippy::disallowed_methods)]

use chrono::{DateTime, Duration, Utc};
use focus_events::{DedupeKey, EventType, NormalizedEvent, TraceRef, WellKnownEventType};
use serde_json::json;
use uuid::Uuid;

use crate::models::{Announcement, Assignment, Course, Submission};

pub const CONNECTOR_ID: &str = "canvas";

/// Compute a stable dedupe key for a Canvas entity.
pub fn dedupe_key(entity_type: &str, id: u64, timestamp: i64) -> DedupeKey {
    DedupeKey(format!("canvas:{entity_type}:{id}:{timestamp}"))
}

/// Window used by [`CanvasEventMapper::map_assignment_due_soon`] — emitted
/// when `due_at` falls within this many hours of `now`.
pub const DUE_SOON_WINDOW_HOURS: i64 = 24;

pub struct CanvasEventMapper;

impl CanvasEventMapper {
    /// Canonical assignment-due event. `course_id_hint` is threaded from the
    /// parent course scope because Canvas omits `course_id` on some listing
    /// endpoints (`/courses/:id/assignments`). The hint takes precedence over
    /// any value on the assignment itself.
    pub fn map_assignment(
        a: &Assignment,
        account_id: Uuid,
        course_id_hint: Option<u64>,
    ) -> NormalizedEvent {
        let occurred = a.due_at.unwrap_or_else(Utc::now);
        let course_id = course_id_hint.or(a.course_id);
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::WellKnown(WellKnownEventType::AssignmentDue),
            occurred_at: occurred,
            effective_at: occurred,
            dedupe_key: dedupe_key("assignment", a.id, occurred.timestamp()),
            confidence: 1.0,
            payload: json!({
                "assignment_id": a.id,
                "course_id": course_id,
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

    /// "Assignment due within [`DUE_SOON_WINDOW_HOURS`] hours of `now`".
    /// Returns `None` if the assignment has no due date or is outside the
    /// window (including overdue — use [`map_assignment_overdue`] for that).
    pub fn map_assignment_due_soon(
        a: &Assignment,
        account_id: Uuid,
        now: DateTime<Utc>,
        course_id_hint: Option<u64>,
    ) -> Option<NormalizedEvent> {
        let due = a.due_at?;
        let window_end = now + Duration::hours(DUE_SOON_WINDOW_HOURS);
        if due < now || due > window_end {
            return None;
        }
        let course_id = course_id_hint.or(a.course_id);
        Some(NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::Custom("canvas:assignment_due_soon".into()),
            occurred_at: due,
            effective_at: due,
            dedupe_key: dedupe_key("assignment_due_soon", a.id, due.timestamp()),
            confidence: 1.0,
            payload: json!({
                "assignment_id": a.id,
                "course_id": course_id,
                "name": a.name,
                "due_at": due,
                "hours_until_due": (due - now).num_hours(),
            }),
            raw_ref: Some(TraceRef {
                source: CONNECTOR_ID.into(),
                id: format!("assignment:{}", a.id),
            }),
        })
    }

    /// "Assignment is past due and has no submission". Returns `None` if the
    /// assignment has no due date, is not yet past due, or already has a
    /// submission.
    pub fn map_assignment_overdue(
        a: &Assignment,
        account_id: Uuid,
        now: DateTime<Utc>,
        has_submission: bool,
        course_id_hint: Option<u64>,
    ) -> Option<NormalizedEvent> {
        let due = a.due_at?;
        if due >= now || has_submission {
            return None;
        }
        let course_id = course_id_hint.or(a.course_id);
        Some(NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::Custom("canvas:assignment_overdue".into()),
            occurred_at: due,
            effective_at: now,
            dedupe_key: dedupe_key("assignment_overdue", a.id, due.timestamp()),
            confidence: 1.0,
            payload: json!({
                "assignment_id": a.id,
                "course_id": course_id,
                "name": a.name,
                "due_at": due,
                "hours_overdue": (now - due).num_hours(),
            }),
            raw_ref: Some(TraceRef {
                source: CONNECTOR_ID.into(),
                id: format!("assignment:{}", a.id),
            }),
        })
    }

    pub fn map_submission(s: &Submission, account_id: Uuid) -> NormalizedEvent {
        let occurred = s.submitted_at.unwrap_or_else(Utc::now);
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::WellKnown(WellKnownEventType::AssignmentGraded),
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

    /// Distinct "grade posted" signal — fires only when the submission has a
    /// numeric score AND its workflow_state is `"graded"`. Returns `None`
    /// otherwise (e.g. "submitted" or "pending_review" without a score).
    pub fn map_grade_posted(s: &Submission, account_id: Uuid) -> Option<NormalizedEvent> {
        if s.score.is_none() || s.workflow_state != "graded" {
            return None;
        }
        let occurred = s.graded_at.or(s.submitted_at).unwrap_or_else(Utc::now);
        Some(NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::Custom("canvas:grade_posted".into()),
            occurred_at: occurred,
            effective_at: occurred,
            dedupe_key: dedupe_key("grade_posted", s.id, occurred.timestamp()),
            confidence: 1.0,
            payload: json!({
                "submission_id": s.id,
                "assignment_id": s.assignment_id,
                "score": s.score,
                "grade": s.grade,
                "graded_at": s.graded_at,
            }),
            raw_ref: Some(TraceRef {
                source: CONNECTOR_ID.into(),
                id: format!("submission:{}", s.id),
            }),
        })
    }

    pub fn map_course_enrolled(c: &Course, account_id: Uuid) -> NormalizedEvent {
        let occurred = Utc::now();
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::WellKnown(WellKnownEventType::CourseEnrolled),
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

    /// "Course announcement posted". Scoped to a specific course because
    /// Canvas's `/announcements` endpoint is queried per-course.
    pub fn map_announcement_posted(
        ann: &Announcement,
        account_id: Uuid,
        course_id: u64,
    ) -> NormalizedEvent {
        let occurred = ann.posted_at.or(ann.delayed_post_at).unwrap_or_else(Utc::now);
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: CONNECTOR_ID.into(),
            account_id,
            event_type: EventType::Custom("canvas:announcement_posted".into()),
            occurred_at: occurred,
            effective_at: occurred,
            dedupe_key: dedupe_key("announcement", ann.id, occurred.timestamp()),
            confidence: 1.0,
            payload: json!({
                "announcement_id": ann.id,
                "course_id": course_id,
                "title": ann.title,
                "posted_at": ann.posted_at,
                "html_url": ann.html_url,
            }),
            raw_ref: Some(TraceRef {
                source: CONNECTOR_ID.into(),
                id: format!("announcement:{}", ann.id),
            }),
        }
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn acct() -> Uuid {
        Uuid::nil()
    }

    fn assignment_with_due(
        id: u64,
        course_id: Option<u64>,
        due: Option<DateTime<Utc>>,
    ) -> Assignment {
        Assignment {
            id,
            name: "HW".into(),
            description: None,
            due_at: due,
            unlock_at: None,
            lock_at: None,
            submission_types: vec!["online_upload".into()],
            points_possible: Some(10.0),
            course_id,
            html_url: None,
            published: None,
        }
    }

    #[test]
    fn maps_assignment_with_due_date() {
        let a = assignment_with_due(
            1,
            Some(42),
            Some(Utc.with_ymd_and_hms(2026, 5, 1, 12, 0, 0).unwrap()),
        );
        let ev = CanvasEventMapper::map_assignment(&a, acct(), None);
        assert_eq!(ev.event_type, EventType::WellKnown(WellKnownEventType::AssignmentDue));
        assert_eq!(ev.payload["course_id"], 42);
    }

    #[test]
    fn map_assignment_prefers_course_id_hint_over_field() {
        // Canvas omits course_id on /courses/:id/assignments responses; mapper
        // must use the caller-provided hint.
        let a = assignment_with_due(1, None, Some(Utc::now()));
        let ev = CanvasEventMapper::map_assignment(&a, acct(), Some(999));
        assert_eq!(ev.payload["course_id"], 999);
    }

    #[test]
    fn map_assignment_falls_back_to_field_if_no_hint() {
        let a = assignment_with_due(1, Some(42), Some(Utc::now()));
        let ev = CanvasEventMapper::map_assignment(&a, acct(), None);
        assert_eq!(ev.payload["course_id"], 42);
    }

    #[test]
    fn maps_assignment_without_due_date_uses_now() {
        let a = assignment_with_due(2, Some(7), None);
        let before = Utc::now();
        let ev = CanvasEventMapper::map_assignment(&a, acct(), None);
        assert!(ev.occurred_at >= before - chrono::Duration::seconds(2));
    }

    #[test]
    fn due_soon_fires_within_window() {
        let now = Utc::now();
        let a = assignment_with_due(1, Some(1), Some(now + Duration::hours(6)));
        let ev = CanvasEventMapper::map_assignment_due_soon(&a, acct(), now, None).unwrap();
        assert_eq!(ev.event_type, EventType::Custom("canvas:assignment_due_soon".into()));
    }

    #[test]
    fn due_soon_skips_when_outside_window() {
        let now = Utc::now();
        let far = assignment_with_due(1, Some(1), Some(now + Duration::hours(48)));
        assert!(CanvasEventMapper::map_assignment_due_soon(&far, acct(), now, None).is_none());

        let past = assignment_with_due(2, Some(1), Some(now - Duration::hours(2)));
        assert!(CanvasEventMapper::map_assignment_due_soon(&past, acct(), now, None).is_none());

        let undated = assignment_with_due(3, Some(1), None);
        assert!(CanvasEventMapper::map_assignment_due_soon(&undated, acct(), now, None).is_none());
    }

    #[test]
    fn overdue_fires_when_past_due_and_no_submission() {
        let now = Utc::now();
        let a = assignment_with_due(1, Some(1), Some(now - Duration::hours(5)));
        let ev = CanvasEventMapper::map_assignment_overdue(&a, acct(), now, false, None).unwrap();
        assert_eq!(ev.event_type, EventType::Custom("canvas:assignment_overdue".into()));
        assert_eq!(ev.payload["hours_overdue"], 5);
    }

    #[test]
    fn overdue_skips_when_submission_exists_or_not_past_due() {
        let now = Utc::now();
        let past = assignment_with_due(1, Some(1), Some(now - Duration::hours(5)));
        assert!(CanvasEventMapper::map_assignment_overdue(&past, acct(), now, true, None).is_none());

        let future = assignment_with_due(2, Some(1), Some(now + Duration::hours(5)));
        assert!(
            CanvasEventMapper::map_assignment_overdue(&future, acct(), now, false, None).is_none()
        );
    }

    #[test]
    fn maps_submission() {
        let s = Submission {
            id: 9,
            submitted_at: Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
            graded_at: None,
            workflow_state: "graded".into(),
            score: Some(95.0),
            grade: None,
            assignment_id: 1,
            user_id: None,
            late: None,
            missing: None,
        };
        let ev = CanvasEventMapper::map_submission(&s, acct());
        assert_eq!(ev.event_type, EventType::WellKnown(WellKnownEventType::AssignmentGraded));
        assert_eq!(ev.payload["score"], 95.0);
    }

    #[test]
    fn grade_posted_fires_only_when_scored_and_graded() {
        let mut s = Submission {
            id: 9,
            submitted_at: Some(Utc::now()),
            graded_at: Some(Utc::now()),
            workflow_state: "graded".into(),
            score: Some(95.0),
            grade: Some("A".into()),
            assignment_id: 1,
            user_id: None,
            late: None,
            missing: None,
        };
        let ev = CanvasEventMapper::map_grade_posted(&s, acct()).unwrap();
        assert_eq!(ev.event_type, EventType::Custom("canvas:grade_posted".into()));

        s.workflow_state = "submitted".into();
        assert!(CanvasEventMapper::map_grade_posted(&s, acct()).is_none());

        s.workflow_state = "graded".into();
        s.score = None;
        assert!(CanvasEventMapper::map_grade_posted(&s, acct()).is_none());
    }

    #[test]
    fn maps_course_enrolled() {
        let c = Course {
            id: 42,
            name: "Math".into(),
            workflow_state: "available".into(),
            enrollment_term_id: Some(7),
            course_code: None,
            start_at: None,
            end_at: None,
        };
        let ev = CanvasEventMapper::map_course_enrolled(&c, acct());
        assert_eq!(ev.event_type, EventType::WellKnown(WellKnownEventType::CourseEnrolled));
        assert_eq!(ev.payload["course_id"], 42);
    }

    #[test]
    fn maps_announcement_posted() {
        let ann = Announcement {
            id: 55,
            title: "Welcome".into(),
            message: "<p>hi</p>".into(),
            posted_at: Some(Utc.with_ymd_and_hms(2026, 4, 1, 12, 0, 0).unwrap()),
            delayed_post_at: None,
            html_url: Some("https://c/x".into()),
            context_code: Some("course_42".into()),
        };
        let ev = CanvasEventMapper::map_announcement_posted(&ann, acct(), 42);
        assert_eq!(ev.event_type, EventType::Custom("canvas:announcement_posted".into()));
        assert_eq!(ev.payload["course_id"], 42);
        assert_eq!(ev.payload["announcement_id"], 55);
        assert!(ev.dedupe_key.0.starts_with("canvas:announcement:55:"));
    }

    #[test]
    fn dedupe_keys_are_distinct_per_entity() {
        let a = assignment_with_due(
            1,
            Some(1),
            Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
        );
        let s = Submission {
            id: 1,
            submitted_at: Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
            graded_at: None,
            workflow_state: "graded".into(),
            score: None,
            grade: None,
            assignment_id: 1,
            user_id: None,
            late: None,
            missing: None,
        };
        let ea = CanvasEventMapper::map_assignment(&a, acct(), None);
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
            course_code: None,
            start_at: None,
            end_at: None,
        };
        let ev = CanvasEventMapper::map_course_enrolled(&c, acct());
        let tr = ev.raw_ref.unwrap();
        assert_eq!(tr.source, "canvas");
        assert_eq!(tr.id, "course:42");
    }
}
