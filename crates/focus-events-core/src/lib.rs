//! # focus-events-core
//!
//! Unified event bus for the Focus ecosystem. Provides a concrete, in-memory
//! pub/sub event bus that integrates with `focus-events` types and `focus-errors`
//! for consistent error handling.
//!
//! ## Usage
//!
//! ```rust
//! use focus_events_core::{EventBus, EventBusConfig};
//!
//! let bus = EventBus::new(EventBusConfig::default());
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use focus_result::FocusResult;
use focus_events::{EventType, NormalizedEvent};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, RwLock};
use tracing::debug;
use uuid::Uuid;

/// Configuration for the event bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBusConfig {
    /// Maximum number of events to retain in the backlog.
    pub backlog_capacity: usize,
    /// Maximum number of concurrent subscribers.
    pub max_subscribers: usize,
    /// Whether to hash event payloads for deduplication.
    pub dedupe_enabled: bool,
}

impl Default for EventBusConfig {
    fn default() -> Self {
        Self {
            backlog_capacity: 1000,
            max_subscribers: 100,
            dedupe_enabled: true,
        }
    }
}

/// A subscription handle that can be used to receive events.
#[derive(Debug)]
pub struct EventSubscription {
    pub id: Uuid,
    pub topic: String,
    pub receiver: broadcast::Receiver<BusEvent>,
}

/// Event envelope used on the bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusEvent {
    pub event_id: Uuid,
    pub topic: String,
    pub event_type: EventType,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub hash: Option<String>,
}

impl BusEvent {
    /// Create a BusEvent from a NormalizedEvent.
    pub fn from_normalized(event: &NormalizedEvent) -> Self {
        let hash = if event.dedupe_key.0.is_empty() {
            None
        } else {
            Some(focus_hash::hash_string_hex(&event.dedupe_key.0))
        };

        Self {
            event_id: event.event_id,
            topic: event.connector_id.clone(),
            event_type: event.event_type.clone(),
            payload: event.payload.clone(),
            occurred_at: event.occurred_at,
            hash,
        }
    }
}

/// The unified event bus for the Focus ecosystem.
#[derive(Debug)]
pub struct EventBus {
    config: EventBusConfig,
    topics: Arc<RwLock<HashMap<String, broadcast::Sender<BusEvent>>>>,
    history: Arc<RwLock<Vec<BusEvent>>>,
}

impl EventBus {
    /// Create a new event bus with the given configuration.
    pub fn new(config: EventBusConfig) -> Self {
        Self {
            config,
            topics: Arc::new(RwLock::new(HashMap::new())),
            history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Publish an event to a topic.
    pub async fn publish(&self, event: BusEvent) -> FocusResult<()> {
        let topic = event.topic.clone();
        debug!("publishing event {} to topic {}", event.event_id, topic);

        // Store in history
        if self.config.backlog_capacity > 0 {
            let mut history = self.history.write().await;
            history.push(event.clone());
            if history.len() > self.config.backlog_capacity {
                history.remove(0);
            }
        }

        // Publish to topic subscribers
        let topics = self.topics.read().await;
        if let Some(sender) = topics.get(&topic) {
            let _ = sender.send(event);
        }
        drop(topics);

        Ok(())
    }

    /// Publish a NormalizedEvent to the bus.
    pub async fn publish_normalized(&self, event: &NormalizedEvent) -> FocusResult<()> {
        // Validate first
        event.validate()?;
        let bus_event = BusEvent::from_normalized(event);
        self.publish(bus_event).await
    }

    /// Subscribe to a topic.
    pub async fn subscribe(&self, topic: &str) -> FocusResult<EventSubscription> {
        let mut topics = self.topics.write().await;

        let sender = topics.entry(topic.to_string()).or_insert_with(|| {
            broadcast::channel(self.config.max_subscribers).0
        });

        let receiver = sender.subscribe();
        let subscription = EventSubscription {
            id: Uuid::new_v4(),
            topic: topic.to_string(),
            receiver,
        };

        debug!("new subscription {} to topic {}", subscription.id, topic);
        Ok(subscription)
    }

    /// Get the event history.
    pub async fn history(&self) -> Vec<BusEvent> {
        self.history.read().await.clone()
    }

    /// Get the number of active topics.
    pub async fn topic_count(&self) -> usize {
        self.topics.read().await.len()
    }

    /// Get the number of events in history.
    pub async fn history_len(&self) -> usize {
        self.history.read().await.len()
    }

    /// Check if an event is a duplicate based on its hash.
    pub async fn is_duplicate(&self, event: &BusEvent) -> bool {
        if !self.config.dedupe_enabled {
            return false;
        }
        let Some(hash) = &event.hash else {
            return false;
        };
        let history = self.history.read().await;
        history.iter().any(|e| e.hash.as_ref() == Some(hash))
    }

    /// Clear the event history.
    pub async fn clear_history(&self) {
        let mut history = self.history.write().await;
        history.clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(EventBusConfig::default())
    }
}

/// A filtered subscription that applies a predicate to incoming events.
#[derive(Debug)]
pub struct FilteredSubscription<F>
where
    F: Fn(&BusEvent) -> bool + Send,
{
    pub inner: EventSubscription,
    pub filter: F,
}

impl<F> FilteredSubscription<F>
where
    F: Fn(&BusEvent) -> bool + Send,
{
    pub fn new(subscription: EventSubscription, filter: F) -> Self {
        Self {
            inner: subscription,
            filter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn t(h: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 1, h, 0, 0).unwrap()
    }

    fn sample_event(topic: &str) -> BusEvent {
        BusEvent {
            event_id: Uuid::new_v4(),
            topic: topic.to_string(),
            event_type: EventType::Custom("test".into()),
            payload: serde_json::json!({"task": "test"}),
            occurred_at: Utc::now(),
            hash: Some(focus_hash::hash_string_hex("test-dedupe")),
        }
    }

    fn sample_normalized_event() -> NormalizedEvent {
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: "test-connector".into(),
            account_id: Uuid::new_v4(),
            event_type: EventType::Custom("test".into()),
            occurred_at: t(1),
            effective_at: t(2),
            dedupe_key: focus_events::DedupeKey("test-dedupe".into()),
            confidence: 1.0,
            payload: serde_json::json!({"task": "test"}),
            raw_ref: None,
        }
    }

    #[tokio::test]
    async fn test_publish_and_subscribe() {
        let bus = EventBus::default();
        let mut sub = bus.subscribe("test").await.unwrap();

        let event = sample_event("test");
        bus.publish(event.clone()).await.unwrap();

        let received = sub.receiver.recv().await.unwrap();
        assert_eq!(received.event_id, event.event_id);
        assert_eq!(received.topic, "test");
    }

    #[tokio::test]
    async fn test_publish_normalized() {
        let bus = EventBus::default();
        let event = sample_normalized_event();
        bus.publish_normalized(&event).await.unwrap();

        let history = bus.history().await;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].event_id, event.event_id);
    }

    #[tokio::test]
    async fn test_history_backlog() {
        let bus = EventBus::new(EventBusConfig {
            backlog_capacity: 3,
            max_subscribers: 10,
            dedupe_enabled: true,
        });

        for i in 0..5 {
            let mut event = sample_event("test");
            event.event_id = Uuid::new_v4();
            event.payload = serde_json::json!({"index": i});
            bus.publish(event).await.unwrap();
        }

        let history = bus.history().await;
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].payload, serde_json::json!({"index": 2}));
        assert_eq!(history[2].payload, serde_json::json!({"index": 4}));
    }

    #[tokio::test]
    async fn test_deduplication() {
        let bus = EventBus::default();
        let event = sample_event("test");

        bus.publish(event.clone()).await.unwrap();
        assert!(bus.is_duplicate(&event).await);

        let mut event2 = sample_event("test");
        event2.hash = Some(focus_hash::hash_string_hex("different-dedupe"));
        assert!(!bus.is_duplicate(&event2).await);
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let bus = EventBus::default();
        let mut sub1 = bus.subscribe("test").await.unwrap();
        let mut sub2 = bus.subscribe("test").await.unwrap();

        let event = sample_event("test");
        bus.publish(event.clone()).await.unwrap();

        let received1 = sub1.receiver.recv().await.unwrap();
        let received2 = sub2.receiver.recv().await.unwrap();
        assert_eq!(received1.event_id, event.event_id);
        assert_eq!(received2.event_id, event.event_id);
    }

    #[tokio::test]
    async fn test_different_topics() {
        let bus = EventBus::default();
        let mut sub1 = bus.subscribe("topic1").await.unwrap();
        let mut sub2 = bus.subscribe("topic2").await.unwrap();

        let event1 = sample_event("topic1");
        let event2 = sample_event("topic2");
        bus.publish(event1.clone()).await.unwrap();
        bus.publish(event2.clone()).await.unwrap();

        let received1 = sub1.receiver.recv().await.unwrap();
        let received2 = sub2.receiver.recv().await.unwrap();
        assert_eq!(received1.event_id, event1.event_id);
        assert_eq!(received2.event_id, event2.event_id);
    }

    #[tokio::test]
    async fn test_topic_count() {
        let bus = EventBus::default();
        bus.subscribe("topic1").await.unwrap();
        bus.subscribe("topic2").await.unwrap();
        bus.subscribe("topic1").await.unwrap(); // same topic

        assert_eq!(bus.topic_count().await, 2);
    }

    #[tokio::test]
    async fn test_clear_history() {
        let bus = EventBus::default();
        let event = sample_event("test");
        bus.publish(event).await.unwrap();

        assert_eq!(bus.history_len().await, 1);
        bus.clear_history().await;
        assert_eq!(bus.history_len().await, 0);
    }

    #[tokio::test]
    async fn test_is_duplicate_disabled() {
        let bus = EventBus::new(EventBusConfig {
            backlog_capacity: 10,
            max_subscribers: 10,
            dedupe_enabled: false,
        });
        let event = sample_event("test");
        bus.publish(event.clone()).await.unwrap();
        assert!(!bus.is_duplicate(&event).await);
    }

    #[tokio::test]
    async fn test_publish_no_subscribers() {
        let bus = EventBus::default();
        let event = sample_event("test");
        // Should not panic or error even with no subscribers
        bus.publish(event).await.unwrap();
    }

    #[tokio::test]
    async fn test_bus_event_from_normalized() {
        let event = sample_normalized_event();
        let bus_event = BusEvent::from_normalized(&event);
        assert_eq!(bus_event.event_id, event.event_id);
        assert_eq!(bus_event.topic, event.connector_id);
        assert_eq!(bus_event.event_type, event.event_type);
        assert!(bus_event.hash.is_some());
    }

    #[tokio::test]
    async fn test_publish_invalid_normalized() {
        let bus = EventBus::default();
        let mut event = sample_normalized_event();
        event.connector_id = String::new(); // invalid
        let result = bus.publish_normalized(&event).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_subscription_id_unique() {
        let bus = EventBus::default();
        let sub1 = bus.subscribe("test").await.unwrap();
        let sub2 = bus.subscribe("test").await.unwrap();
        assert_ne!(sub1.id, sub2.id);
    }

    #[tokio::test]
    async fn test_filtered_subscription() {
        let bus = EventBus::default();
        let sub = bus.subscribe("test").await.unwrap();
        let _filtered = FilteredSubscription::new(sub, |event: &BusEvent| {
            matches!(event.event_type, EventType::WellKnown(_))
        });
    }
}
