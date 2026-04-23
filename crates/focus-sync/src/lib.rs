//! Polling scheduler, cursor, dedupe, retries, backoff.

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub jitter: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SyncTrigger {
    Manual,
    Scheduled,
    Webhook,
    ForegroundResume,
}

pub struct SyncOrchestrator {
    // Stub: schedule table, connector registry, cursor store
}

impl SyncOrchestrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SyncOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}
