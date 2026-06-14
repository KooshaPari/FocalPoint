use serde::{Deserialize, Serialize};

/// Configuration for snapshot creation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotConfig {
    pub snapshot_every: i64,
    pub max_events_before_snapshot: i64,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            snapshot_every: 100,
            max_events_before_snapshot: 500,
        }
    }
}

impl SnapshotConfig {
    pub fn should_snapshot(&self, event_count: i64) -> bool {
        event_count > 0 && event_count % self.snapshot_every == 0
    }
}
