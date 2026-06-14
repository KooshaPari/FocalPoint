#![allow(async_fn_in_trait)]

use phenotype_error_core::Result;
use serde::Serialize;

use crate::event::Envelope;

/// Core event store trait.
pub trait EventStore: Send + Sync {
    async fn append<T: Serialize>(&self, aggregate_id: &str, event: Envelope<T>) -> Result<i64>;
    async fn get_events(&self, aggregate_id: &str) -> Result<Vec<Envelope<serde_json::Value>>>;
    async fn get_events_from(
        &self,
        aggregate_id: &str,
        sequence: i64,
    ) -> Result<Vec<Envelope<serde_json::Value>>>;
}
