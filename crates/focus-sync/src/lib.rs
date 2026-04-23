//! Polling scheduler, cursor, dedupe, retries, backoff.
//!
//! The [`SyncOrchestrator`] owns a registry of [`Connector`](focus_connectors::Connector)
//! implementations plus their polling cadence, next-sync deadline, and last cursor.
//! Callers drive it via [`SyncOrchestrator::tick`], passing in the current timestamp
//! (clock injection) -- the orchestrator never reads the wall clock itself.
//!
//! Traces to: FR-CONN-003, FR-EVT-002

pub mod retry;

pub use retry::{next_delay, RetryPolicy};

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use focus_connectors::{Connector, ConnectorError, HealthState};
use focus_time::ClockPort;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor(pub String);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncTrigger {
    Manual,
    Scheduled,
    Webhook,
    ForegroundResume,
}

/// Errors surfaced by orchestrator APIs themselves (distinct from per-connector errors,
/// which are captured in [`SyncReport::errors`]).
#[derive(Debug, Error)]
pub enum OrchestratorError {
    #[error("connector already registered: {0}")]
    AlreadyRegistered(String),
    #[error("unknown connector: {0}")]
    Unknown(String),
}

/// Per-connector runtime state tracked by the orchestrator.
pub struct ConnectorHandle {
    pub id: String,
    pub connector: Arc<dyn Connector>,
    pub cadence: Duration,
    pub next_sync_at: DateTime<Utc>,
    pub last_cursor: Option<String>,
    /// Current health -- set to [`HealthState::Unauthenticated`] on 401/auth errors.
    pub health: HealthState,
    /// Consecutive generic-error attempts (resets on success, auth, or rate limit).
    pub failed_attempts: u32,
}

impl std::fmt::Debug for ConnectorHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectorHandle")
            .field("id", &self.id)
            .field("cadence", &self.cadence)
            .field("next_sync_at", &self.next_sync_at)
            .field("last_cursor", &self.last_cursor)
            .field("health", &self.health)
            .field("failed_attempts", &self.failed_attempts)
            .finish()
    }
}

/// Summary of what happened during a [`SyncOrchestrator::tick`].
#[derive(Debug, Default, Clone)]
pub struct SyncReport {
    pub events_pulled: usize,
    pub connectors_synced: usize,
    pub errors: Vec<SyncErrorEntry>,
}

#[derive(Debug, Clone)]
pub struct SyncErrorEntry {
    pub connector_id: String,
    pub kind: SyncErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncErrorKind {
    Auth,
    RateLimited { retry_after_s: u64 },
    Schema,
    Network,
    Exhausted,
}

pub struct SyncOrchestrator {
    connectors: HashMap<String, ConnectorHandle>,
    retry: RetryPolicy,
}

impl std::fmt::Debug for SyncOrchestrator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncOrchestrator")
            .field("connectors", &self.connectors.keys().collect::<Vec<_>>())
            .field("retry", &self.retry)
            .finish()
    }
}

impl SyncOrchestrator {
    pub fn new(retry: RetryPolicy) -> Self {
        Self { connectors: HashMap::new(), retry }
    }

    pub fn with_default_retry() -> Self {
        Self::new(RetryPolicy::default())
    }

    /// Register a connector. `now` is the reference clock; the first sync is scheduled
    /// at `now + cadence`.
    pub fn register(
        &mut self,
        id: impl Into<String>,
        connector: Arc<dyn Connector>,
        cadence: Duration,
        now: DateTime<Utc>,
    ) -> Result<(), OrchestratorError> {
        let id = id.into();
        if self.connectors.contains_key(&id) {
            return Err(OrchestratorError::AlreadyRegistered(id));
        }
        let next_sync_at = now + to_chrono(cadence);
        self.connectors.insert(
            id.clone(),
            ConnectorHandle {
                id,
                connector,
                cadence,
                next_sync_at,
                last_cursor: None,
                health: HealthState::Healthy,
                failed_attempts: 0,
            },
        );
        Ok(())
    }

    pub fn unregister(&mut self, id: &str) -> Result<(), OrchestratorError> {
        self.connectors
            .remove(id)
            .map(|_| ())
            .ok_or_else(|| OrchestratorError::Unknown(id.to_string()))
    }

    pub fn connector(&self, id: &str) -> Option<&ConnectorHandle> {
        self.connectors.get(id)
    }

    pub fn len(&self) -> usize {
        self.connectors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.connectors.is_empty()
    }

    /// Drive one tick. Any connector whose `next_sync_at <= now` is synced once.
    pub async fn tick(&mut self, now: DateTime<Utc>) -> SyncReport {
        let mut report = SyncReport::default();

        // Collect due ids up front to avoid holding a mutable borrow across awaits.
        let due: Vec<String> = self
            .connectors
            .iter()
            .filter(|(_, h)| h.next_sync_at <= now && h.health != HealthState::Unauthenticated)
            .map(|(k, _)| k.clone())
            .collect();

        for id in due {
            let (connector, cursor) = {
                let handle = self.connectors.get(&id).expect("present");
                (handle.connector.clone(), handle.last_cursor.clone())
            };

            debug!(connector_id = %id, "syncing");
            let result = connector.sync(cursor).await;

            let handle = self.connectors.get_mut(&id).expect("present");
            match result {
                Ok(outcome) => {
                    report.events_pulled += outcome.events.len();
                    report.connectors_synced += 1;
                    handle.last_cursor = outcome.next_cursor;
                    handle.next_sync_at = now + to_chrono(handle.cadence);
                    handle.failed_attempts = 0;
                    handle.health = HealthState::Healthy;
                    info!(
                        connector_id = %id,
                        events = outcome.events.len(),
                        "sync ok"
                    );
                }
                Err(ConnectorError::Auth(msg)) => {
                    warn!(connector_id = %id, "auth error; marking unauthenticated");
                    handle.health = HealthState::Unauthenticated;
                    handle.failed_attempts = 0;
                    // Do not reschedule -- connector must be reauthed.
                    report.errors.push(SyncErrorEntry {
                        connector_id: id.clone(),
                        kind: SyncErrorKind::Auth,
                        message: msg,
                    });
                }
                Err(ConnectorError::RateLimited(seconds)) => {
                    warn!(connector_id = %id, retry_after = seconds, "rate limited");
                    handle.next_sync_at = now + ChronoDuration::seconds(seconds as i64);
                    handle.failed_attempts = 0;
                    handle.health = HealthState::Degraded(format!("rate_limited:{seconds}"));
                    report.errors.push(SyncErrorEntry {
                        connector_id: id.clone(),
                        kind: SyncErrorKind::RateLimited { retry_after_s: seconds },
                        message: format!("rate limited for {seconds}s"),
                    });
                }
                Err(err) => {
                    handle.failed_attempts = handle.failed_attempts.saturating_add(1);
                    let attempt = handle.failed_attempts;
                    let kind = match &err {
                        ConnectorError::Schema(_) => SyncErrorKind::Schema,
                        ConnectorError::Network(_) => SyncErrorKind::Network,
                        // Auth/RateLimited already handled above.
                        _ => SyncErrorKind::Network,
                    };
                    if attempt >= self.retry.max_attempts {
                        warn!(
                            connector_id = %id,
                            attempt,
                            "retry budget exhausted"
                        );
                        handle.health = HealthState::Failing(err.to_string());
                        // Reschedule on the normal cadence so the connector isn't
                        // permanently stuck; caller can observe `Exhausted`.
                        handle.next_sync_at = now + to_chrono(handle.cadence);
                        handle.failed_attempts = 0;
                        report.errors.push(SyncErrorEntry {
                            connector_id: id.clone(),
                            kind: SyncErrorKind::Exhausted,
                            message: err.to_string(),
                        });
                    } else {
                        let backoff = next_delay(attempt, &self.retry);
                        warn!(
                            connector_id = %id,
                            attempt,
                            backoff_ms = backoff.as_millis() as u64,
                            "scheduling retry"
                        );
                        handle.next_sync_at = now + to_chrono(backoff);
                        handle.health = HealthState::Degraded(format!("retry:{attempt}"));
                        report.errors.push(SyncErrorEntry {
                            connector_id: id.clone(),
                            kind,
                            message: err.to_string(),
                        });
                    }
                }
            }
        }

        report
    }

    /// Drive the orchestrator on a fixed cadence using the supplied clock.
    ///
    /// Loops forever; call in a `tokio::spawn` task. For a bounded loop in tests,
    /// call [`tick`](Self::tick) directly.
    pub async fn run_loop(&mut self, clock: Arc<dyn ClockPort>, interval: Duration) -> ! {
        let mut ticker = tokio::time::interval(interval);
        // Skip the immediate first tick that `interval` fires.
        ticker.tick().await;
        loop {
            ticker.tick().await;
            let _ = self.tick(clock.now()).await;
        }
    }
}

fn to_chrono(d: Duration) -> ChronoDuration {
    // `from_std` only fails for durations > i64::MAX seconds; clamp defensively.
    ChronoDuration::from_std(d).unwrap_or_else(|_| ChronoDuration::seconds(i64::MAX / 2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::TimeZone;
    use focus_connectors::{AuthStrategy, ConnectorManifest, SyncMode, SyncOutcome};
    use focus_events::{DedupeKey, EventType, NormalizedEvent};
    use std::sync::Mutex;
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    enum InjectedError {
        None,
        Auth,
        RateLimited(u64),
        Generic,
        Schema,
    }

    struct MockConnector {
        manifest: ConnectorManifest,
        /// Per-call script: pops front on each sync.
        script: Mutex<Vec<MockResponse>>,
        /// Fallback when script is empty: returns empty success.
        call_log: Mutex<Vec<Option<String>>>,
    }

    #[derive(Debug, Clone)]
    struct MockResponse {
        error: InjectedError,
        event_count: usize,
        next_cursor: Option<String>,
    }

    impl MockConnector {
        fn new(id: &str, script: Vec<MockResponse>) -> Arc<Self> {
            Arc::new(Self {
                manifest: ConnectorManifest {
                    id: id.into(),
                    version: "test".into(),
                    display_name: id.into(),
                    auth_strategy: AuthStrategy::None,
                    sync_mode: SyncMode::Polling { cadence_seconds: 60 },
                    capabilities: vec![],
                    entity_types: vec![],
                    event_types: vec![],
                },
                script: Mutex::new(script),
                call_log: Mutex::new(vec![]),
            })
        }

        fn calls(&self) -> Vec<Option<String>> {
            self.call_log.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl Connector for MockConnector {
        fn manifest(&self) -> &ConnectorManifest {
            &self.manifest
        }

        async fn health(&self) -> HealthState {
            HealthState::Healthy
        }

        async fn sync(&self, cursor: Option<String>) -> focus_connectors::Result<SyncOutcome> {
            self.call_log.lock().unwrap().push(cursor.clone());
            let next = {
                let mut s = self.script.lock().unwrap();
                if s.is_empty() {
                    MockResponse { error: InjectedError::None, event_count: 0, next_cursor: None }
                } else {
                    s.remove(0)
                }
            };

            match next.error {
                InjectedError::Auth => Err(ConnectorError::Auth("401 unauthorized".into())),
                InjectedError::RateLimited(s) => Err(ConnectorError::RateLimited(s)),
                InjectedError::Generic => Err(ConnectorError::Network("boom".into())),
                InjectedError::Schema => Err(ConnectorError::Schema("bad schema".into())),
                InjectedError::None => {
                    let events = (0..next.event_count)
                        .map(|i| synthetic_event(&self.manifest.id, i))
                        .collect();
                    Ok(SyncOutcome { events, next_cursor: next.next_cursor, partial: false })
                }
            }
        }
    }

    fn synthetic_event(connector_id: &str, i: usize) -> NormalizedEvent {
        let ts = Utc.with_ymd_and_hms(2026, 4, 22, 0, 0, 0).unwrap();
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: connector_id.into(),
            account_id: Uuid::nil(),
            event_type: EventType::Custom("test".into()),
            occurred_at: ts,
            effective_at: ts,
            dedupe_key: DedupeKey(format!("{connector_id}:{i}")),
            confidence: 1.0,
            payload: serde_json::json!({}),
            raw_ref: None,
        }
    }

    fn t0() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 4, 22, 12, 0, 0).unwrap()
    }

    fn ok(count: usize, cursor: Option<&str>) -> MockResponse {
        MockResponse {
            error: InjectedError::None,
            event_count: count,
            next_cursor: cursor.map(String::from),
        }
    }

    fn err(kind: InjectedError) -> MockResponse {
        MockResponse { error: kind, event_count: 0, next_cursor: None }
    }

    // Traces to: FR-CONN-003
    #[tokio::test]
    async fn register_schedules_first_sync_at_now_plus_cadence() {
        let conn = MockConnector::new("c1", vec![]);
        let mut orch = SyncOrchestrator::with_default_retry();
        orch.register("c1", conn, Duration::from_secs(60), t0()).unwrap();
        let h = orch.connector("c1").unwrap();
        assert_eq!(h.next_sync_at, t0() + ChronoDuration::seconds(60));
        assert_eq!(h.last_cursor, None);
    }

    // Traces to: FR-CONN-003
    #[tokio::test]
    async fn register_rejects_duplicate_id() {
        let mut orch = SyncOrchestrator::with_default_retry();
        orch.register("c1", MockConnector::new("c1", vec![]), Duration::from_secs(60), t0())
            .unwrap();
        let dup =
            orch.register("c1", MockConnector::new("c1", vec![]), Duration::from_secs(60), t0());
        assert!(matches!(dup, Err(OrchestratorError::AlreadyRegistered(_))));
    }

    // Traces to: FR-CONN-003, FR-EVT-002
    #[tokio::test]
    async fn tick_only_syncs_connectors_whose_deadline_has_passed() {
        let fast = MockConnector::new("fast", vec![ok(3, Some("A"))]);
        let slow = MockConnector::new("slow", vec![ok(5, Some("B"))]);
        let mut orch = SyncOrchestrator::with_default_retry();
        orch.register("fast", fast.clone(), Duration::from_secs(10), t0()).unwrap();
        orch.register("slow", slow.clone(), Duration::from_secs(60), t0()).unwrap();

        // t=0 -- neither is due yet (both scheduled for now + cadence).
        let r0 = orch.tick(t0()).await;
        assert_eq!(r0.connectors_synced, 0);

        // t = 10s -> fast due, slow not due
        let r1 = orch.tick(t0() + ChronoDuration::seconds(10)).await;
        assert_eq!(r1.connectors_synced, 1);
        assert_eq!(r1.events_pulled, 3);
        assert_eq!(fast.calls().len(), 1);
        assert_eq!(slow.calls().len(), 0);

        // t = 60s -> slow now due too (fast was rescheduled to 10+10=20, also due)
        let r2 = orch.tick(t0() + ChronoDuration::seconds(60)).await;
        assert_eq!(r2.connectors_synced, 2);
        assert_eq!(r2.events_pulled, 5 /* slow */);
    }

    // Traces to: FR-CONN-003, FR-EVT-002
    #[tokio::test]
    async fn cursor_is_passed_back_on_next_sync() {
        let conn = MockConnector::new("c1", vec![ok(1, Some("cursor-A")), ok(2, Some("cursor-B"))]);
        let mut orch = SyncOrchestrator::with_default_retry();
        orch.register("c1", conn.clone(), Duration::from_secs(10), t0()).unwrap();

        orch.tick(t0() + ChronoDuration::seconds(10)).await;
        assert_eq!(orch.connector("c1").unwrap().last_cursor.as_deref(), Some("cursor-A"));

        orch.tick(t0() + ChronoDuration::seconds(30)).await;
        let calls = conn.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], None);
        assert_eq!(calls[1].as_deref(), Some("cursor-A"));
        assert_eq!(orch.connector("c1").unwrap().last_cursor.as_deref(), Some("cursor-B"));
    }

    // Traces to: FR-CONN-003
    #[tokio::test]
    async fn auth_error_marks_unauth_and_does_not_block_others() {
        let bad = MockConnector::new("bad", vec![err(InjectedError::Auth)]);
        let good = MockConnector::new("good", vec![ok(2, Some("g"))]);
        let mut orch = SyncOrchestrator::with_default_retry();
        orch.register("bad", bad.clone(), Duration::from_secs(10), t0()).unwrap();
        orch.register("good", good.clone(), Duration::from_secs(10), t0()).unwrap();

        let r = orch.tick(t0() + ChronoDuration::seconds(10)).await;
        assert_eq!(r.connectors_synced, 1);
        assert_eq!(r.events_pulled, 2);
        assert_eq!(r.errors.len(), 1);
        assert_eq!(r.errors[0].connector_id, "bad");
        assert!(matches!(r.errors[0].kind, SyncErrorKind::Auth));
        assert_eq!(orch.connector("bad").unwrap().health, HealthState::Unauthenticated);

        // Next tick: unauth connector must be skipped entirely.
        let r2 = orch.tick(t0() + ChronoDuration::seconds(60)).await;
        assert!(!r2.errors.iter().any(|e| e.connector_id == "bad"));
        assert_eq!(bad.calls().len(), 1, "auth-failed connector is not retried");
    }

    // Traces to: FR-CONN-003
    #[tokio::test]
    async fn rate_limited_pushes_next_sync_by_retry_after() {
        let conn = MockConnector::new("c1", vec![err(InjectedError::RateLimited(60))]);
        let mut orch = SyncOrchestrator::with_default_retry();
        orch.register("c1", conn, Duration::from_secs(10), t0()).unwrap();

        let t_sync = t0() + ChronoDuration::seconds(10);
        let r = orch.tick(t_sync).await;
        assert_eq!(r.errors.len(), 1);
        assert!(matches!(r.errors[0].kind, SyncErrorKind::RateLimited { retry_after_s: 60 }));
        let next = orch.connector("c1").unwrap().next_sync_at;
        assert_eq!(next, t_sync + ChronoDuration::seconds(60));
    }

    // Traces to: FR-CONN-003
    #[tokio::test]
    async fn generic_error_retries_then_exhausts() {
        let policy = RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            jitter: false,
        };
        let conn = MockConnector::new(
            "c1",
            vec![
                err(InjectedError::Generic),
                err(InjectedError::Generic),
                err(InjectedError::Generic),
            ],
        );
        let mut orch = SyncOrchestrator::new(policy);
        orch.register("c1", conn.clone(), Duration::from_secs(10), t0()).unwrap();

        // Attempt 1 -> backoff 1s
        let mut now = t0() + ChronoDuration::seconds(10);
        let r1 = orch.tick(now).await;
        assert_eq!(r1.errors.len(), 1);
        assert!(matches!(r1.errors[0].kind, SyncErrorKind::Network));
        assert_eq!(orch.connector("c1").unwrap().failed_attempts, 1);

        // Attempt 2 -> backoff 2s
        now += ChronoDuration::seconds(5);
        let r2 = orch.tick(now).await;
        assert_eq!(r2.errors.len(), 1);
        assert!(matches!(r2.errors[0].kind, SyncErrorKind::Network));
        assert_eq!(orch.connector("c1").unwrap().failed_attempts, 2);

        // Attempt 3 -> exhausts (max_attempts reached)
        now += ChronoDuration::seconds(5);
        let r3 = orch.tick(now).await;
        assert_eq!(r3.errors.len(), 1);
        assert!(
            matches!(r3.errors[0].kind, SyncErrorKind::Exhausted),
            "got {:?}",
            r3.errors[0].kind
        );
        assert_eq!(orch.connector("c1").unwrap().failed_attempts, 0, "exhausted resets attempts");
        assert!(matches!(orch.connector("c1").unwrap().health, HealthState::Failing(_)));
    }

    // Traces to: FR-CONN-003
    #[tokio::test]
    async fn success_resets_failed_attempts_and_health() {
        let policy = RetryPolicy {
            max_attempts: 5,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            jitter: false,
        };
        let conn = MockConnector::new("c1", vec![err(InjectedError::Generic), ok(1, Some("cur"))]);
        let mut orch = SyncOrchestrator::new(policy);
        orch.register("c1", conn, Duration::from_secs(10), t0()).unwrap();

        let mut now = t0() + ChronoDuration::seconds(10);
        orch.tick(now).await;
        assert_eq!(orch.connector("c1").unwrap().failed_attempts, 1);

        now += ChronoDuration::seconds(5);
        orch.tick(now).await;
        let h = orch.connector("c1").unwrap();
        assert_eq!(h.failed_attempts, 0);
        assert_eq!(h.health, HealthState::Healthy);
        assert_eq!(h.last_cursor.as_deref(), Some("cur"));
    }

    // Traces to: FR-CONN-003
    #[tokio::test]
    async fn schema_error_classified_correctly() {
        let conn = MockConnector::new("c1", vec![err(InjectedError::Schema)]);
        let policy = RetryPolicy {
            max_attempts: 5,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            jitter: false,
        };
        let mut orch = SyncOrchestrator::new(policy);
        orch.register("c1", conn, Duration::from_secs(10), t0()).unwrap();

        let r = orch.tick(t0() + ChronoDuration::seconds(10)).await;
        assert_eq!(r.errors.len(), 1);
        assert!(matches!(r.errors[0].kind, SyncErrorKind::Schema));
    }

    #[tokio::test]
    async fn unregister_removes_handle() {
        let mut orch = SyncOrchestrator::with_default_retry();
        orch.register("c1", MockConnector::new("c1", vec![]), Duration::from_secs(10), t0())
            .unwrap();
        assert_eq!(orch.len(), 1);
        orch.unregister("c1").unwrap();
        assert!(orch.is_empty());
        assert!(matches!(orch.unregister("c1"), Err(OrchestratorError::Unknown(_))));
    }

    // Traces to: FR-EVT-002
    #[tokio::test]
    async fn sync_report_aggregates_events_across_connectors() {
        let a = MockConnector::new("a", vec![ok(4, Some("a1"))]);
        let b = MockConnector::new("b", vec![ok(7, Some("b1"))]);
        let mut orch = SyncOrchestrator::with_default_retry();
        orch.register("a", a, Duration::from_secs(10), t0()).unwrap();
        orch.register("b", b, Duration::from_secs(10), t0()).unwrap();

        let r = orch.tick(t0() + ChronoDuration::seconds(10)).await;
        assert_eq!(r.connectors_synced, 2);
        assert_eq!(r.events_pulled, 11);
        assert!(r.errors.is_empty());
    }
}
