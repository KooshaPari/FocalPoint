//! Normalized event schema, dedupe keys, trace references.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub struct EventFactory;

impl EventFactory {
    pub fn new_dedupe_key(source: &str, id: &str, occurred_at: DateTime<Utc>) -> DedupeKey {
        DedupeKey(format!("{source}:{id}:{}", occurred_at.timestamp()))
    }
}
