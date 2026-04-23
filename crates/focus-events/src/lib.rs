//! Normalized event schema, dedupe keys, trace references.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum EventError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),
    #[error("invalid confidence: {0} (must be in [0.0, 1.0])")]
    InvalidConfidence(f32),
    #[error("connector_id is empty")]
    EmptyConnectorId,
    #[error("dedupe_key is empty")]
    EmptyDedupeKey,
    #[error("time order invalid: occurred_at must be <= effective_at")]
    TimeOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedEvent {
    pub event_id: Uuid,
    pub connector_id: String,
    pub account_id: Uuid,
    pub event_type: EventType,
    pub occurred_at: DateTime<Utc>,
    pub effective_at: DateTime<Utc>,
    pub dedupe_key: DedupeKey,
    pub confidence: f32,
    pub payload: serde_json::Value,
    pub raw_ref: Option<TraceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DedupeKey(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRef {
    pub source: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    // Learning
    AssignmentDue,
    AssignmentGraded,
    CourseEnrolled,
    // Calendar
    EventStarted,
    EventEnded,
    // Task
    TaskCompleted,
    TaskAdded,
    // Health
    SleepRecorded,
    ExerciseLogged,
    // App usage
    AppSessionStarted,
    AppSessionEnded,
    // Custom
    Custom(String),
}

impl NormalizedEvent {
    /// Validate required-fields schema.
    /// Traces to: FR-EVT-001.
    pub fn validate(&self) -> std::result::Result<(), EventError> {
        if self.connector_id.is_empty() {
            return Err(EventError::EmptyConnectorId);
        }
        if self.dedupe_key.0.is_empty() {
            return Err(EventError::EmptyDedupeKey);
        }
        if !self.confidence.is_finite() || self.confidence < 0.0 || self.confidence > 1.0 {
            return Err(EventError::InvalidConfidence(self.confidence));
        }
        if self.occurred_at > self.effective_at {
            return Err(EventError::TimeOrder);
        }
        Ok(())
    }
}

pub struct EventFactory;

impl EventFactory {
    pub fn new_dedupe_key(source: &str, id: &str, occurred_at: DateTime<Utc>) -> DedupeKey {
        DedupeKey(format!("{source}:{id}:{}", occurred_at.timestamp()))
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn t(h: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 1, h, 0, 0).unwrap()
    }

    fn sample() -> NormalizedEvent {
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: "canvas".into(),
            account_id: Uuid::new_v4(),
            event_type: EventType::AssignmentDue,
            occurred_at: t(1),
            effective_at: t(2),
            dedupe_key: DedupeKey("canvas:1:1".into()),
            confidence: 1.0,
            payload: serde_json::json!({}),
            raw_ref: None,
        }
    }

    // Traces to: FR-EVT-001
    #[test]
    fn validate_happy_path() {
        assert!(sample().validate().is_ok());
    }

    // Traces to: FR-EVT-001
    #[test]
    fn validate_rejects_empty_connector_id() {
        let mut e = sample();
        e.connector_id = String::new();
        assert_eq!(e.validate().unwrap_err(), EventError::EmptyConnectorId);
    }

    // Traces to: FR-EVT-001
    #[test]
    fn validate_rejects_empty_dedupe_key() {
        let mut e = sample();
        e.dedupe_key = DedupeKey(String::new());
        assert_eq!(e.validate().unwrap_err(), EventError::EmptyDedupeKey);
    }

    // Traces to: FR-EVT-001
    #[test]
    fn validate_rejects_out_of_range_confidence() {
        let mut e = sample();
        e.confidence = 1.5;
        assert_eq!(e.validate().unwrap_err(), EventError::InvalidConfidence(1.5));
        let mut e2 = sample();
        e2.confidence = -0.1;
        assert!(matches!(e2.validate().unwrap_err(), EventError::InvalidConfidence(_)));
    }

    // Traces to: FR-EVT-001
    #[test]
    fn validate_rejects_time_order() {
        let mut e = sample();
        e.occurred_at = t(5);
        e.effective_at = t(1);
        assert_eq!(e.validate().unwrap_err(), EventError::TimeOrder);
    }
}
