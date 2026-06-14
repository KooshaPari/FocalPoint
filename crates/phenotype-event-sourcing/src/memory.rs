use phenotype_error_core::{PhenotypeError, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::event::Envelope;
use crate::store::EventStore;

/// In-memory event store for testing and development.
#[derive(Debug, Clone)]
pub struct InMemoryEventStore {
    events: Arc<Mutex<HashMap<String, Vec<Envelope<serde_json::Value>>>>>,
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn clear(&self) {
        let mut store = self.events.lock().unwrap();
        store.clear();
    }

    pub fn event_count(&self) -> usize {
        let store = self.events.lock().unwrap();
        store.values().map(|v| v.len()).sum()
    }

    pub fn event_count_for(&self, aggregate_id: &str) -> usize {
        let store = self.events.lock().unwrap();
        store.get(aggregate_id).map(|v| v.len()).unwrap_or(0)
    }
}

impl EventStore for InMemoryEventStore {
    async fn append<T: Serialize>(&self, aggregate_id: &str, event: Envelope<T>) -> Result<i64> {
        let mut store = self.events.lock().unwrap();
        let entry = store.entry(aggregate_id.to_string()).or_default();
        let value = serde_json::to_value(&event.payload)
            .map_err(|e| PhenotypeError::serialization(e.to_string()))?;
        let envelope = Envelope {
            id: event.id,
            metadata: event.metadata,
            payload: value,
            aggregate_id: event.aggregate_id,
            sequence: event.sequence,
            timestamp: event.timestamp,
        };
        entry.push(envelope);
        Ok(entry.len() as i64)
    }

    async fn get_events(&self, aggregate_id: &str) -> Result<Vec<Envelope<serde_json::Value>>> {
        let store = self.events.lock().unwrap();
        Ok(store.get(aggregate_id).cloned().unwrap_or_default())
    }

    async fn get_events_from(
        &self,
        aggregate_id: &str,
        sequence: i64,
    ) -> Result<Vec<Envelope<serde_json::Value>>> {
        let events = self.get_events(aggregate_id).await?;
        Ok(events
            .into_iter()
            .filter(|e| e.sequence >= sequence)
            .collect())
    }
}
