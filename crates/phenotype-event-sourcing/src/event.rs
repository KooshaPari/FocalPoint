use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An envelope that wraps a domain event with metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Envelope<T> {
    pub id: Uuid,
    pub metadata: Metadata,
    pub payload: T,
    pub aggregate_id: String,
    pub sequence: i64,
    pub timestamp: DateTime<Utc>,
}

/// Metadata attached to every event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Metadata {
    pub causation_id: Option<Uuid>,
    pub correlation_id: Option<Uuid>,
    pub actor: String,
    pub extra: std::collections::HashMap<String, String>,
}

impl<T> Envelope<T> {
    pub fn new(
        aggregate_id: impl Into<String>,
        sequence: i64,
        payload: T,
        actor: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata: Metadata {
                actor: actor.into(),
                ..Default::default()
            },
            payload,
            aggregate_id: aggregate_id.into(),
            sequence,
            timestamp: Utc::now(),
        }
    }

    pub fn with_causation_id(mut self, id: Uuid) -> Self {
        self.metadata.causation_id = Some(id);
        self
    }

    pub fn with_correlation_id(mut self, id: Uuid) -> Self {
        self.metadata.correlation_id = Some(id);
        self
    }
}
