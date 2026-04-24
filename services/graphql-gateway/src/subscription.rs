//! Subscription support for live audit feed via WebSocket (future implementation).
//!
//! Currently stubbed; full streaming implementation to come when
//! async-graphql subscription infrastructure is fully integrated.

use crate::model::AuditRecord;
use tokio::sync::broadcast;

/// Broadcast channel for audit events (capacity: 100 events).
pub type AuditBroadcaster = broadcast::Sender<AuditRecord>;

/// Create a new audit event broadcaster.
pub fn create_broadcaster() -> AuditBroadcaster {
    let (tx, _rx) = broadcast::channel(100);
    tx
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn broadcast_audit_events() {
        let tx = create_broadcaster();
        let mut rx = tx.subscribe();

        let record = AuditRecord {
            id: "audit-1".to_string(),
            record_type: "test_event".to_string(),
            subject_ref: "test-1".to_string(),
            occurred_at: Utc::now(),
            prev_hash: "genesis".to_string(),
            hash: "abc123".to_string(),
            payload: serde_json::json!({}),
        };

        tx.send(record.clone()).ok();

        if let Ok(received) = rx.recv().await {
            assert_eq!(received.id, record.id);
        }
    }
}
