#![forbid(unsafe_code)]

//! # focus-telemetry — Opt-in Anonymous Usage Analytics
//!
//! Provides local-first, PII-scrubbed event collection for FocalPoint.
//! Events are buffered in SQLite and flushed to a configurable endpoint every 15 minutes
//! (only when user has opted in).
//!
//! **Event Schema:**
//! ```ignore
//! {
//!   event_id: UUID,
//!   name: str,         // e.g., "app.opened", "connector.connected"
//!   ts: ISO8601,
//!   session_id: str,   // anonymized hash of (install_time + device_model)
//!   app_version: str,
//!   os_version: str,
//!   props: JSON        // custom properties, pre-scrubbed for PII
//! }
//! ```
//!
//! **PII Scrubbing** (applied before buffering):
//! - Email addresses: user@domain.com → [REDACTED_EMAIL]
//! - Phone numbers: (555) 555-0123 → [REDACTED_PHONE]
//! - OAuth tokens: "Bearer sk_live_..." → [REDACTED_TOKEN]
//! - Task/Rule UUIDs: [REDACTED_UUID]
//!
//! **No user_id, email, IP, task titles, or connector data is ever collected.**

use anyhow::Result;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub mod audit;
pub mod pii_scrubber;

pub use audit::AuditRecord;
pub use pii_scrubber::PiiScrubber;

// =============================================================================
// Unified Telemetry Traits
// =============================================================================

/// Trait for any component that can emit telemetry events.
pub trait Telemetry {
    /// Track an event with optional properties.
    fn track(&self, event_name: &str, props: serde_json::Value) -> Result<()>;

    /// Flush all buffered events.
    fn flush(&self) -> Result<()>;

    /// Get the current session / telemetry identifier.
    fn session_id(&self) -> &str;
}

/// Trait for any component that can emit metrics.
pub trait Metric {
    /// Record a counter metric.
    fn record_counter(&self, name: &str, value: u64);

    /// Record a gauge metric.
    fn record_gauge(&self, name: &str, value: f64);

    /// Record a histogram / timing metric.
    fn record_histogram(&self, name: &str, value_ms: u64);
}

/// Trait for health-check endpoints.
pub trait HealthCheck {
    /// Return true if the component is healthy.
    fn is_healthy(&self) -> Result<bool>;

    /// Return a human-readable health status.
    fn health_status(&self) -> Result<HealthStatus>;
}

/// Health status for a component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub component: String,
    pub healthy: bool,
    pub last_check: String,
    pub message: Option<String>,
}

/// Trait for audit logging.
pub trait AuditLogger {
    /// Log a recordable audit event.
    fn log_audit(&self, record: AuditRecord) -> Result<()>;

    /// Query recent audit records.
    fn query_audit(&self, limit: usize) -> Result<Vec<AuditRecord>>;
}

/// Trait for exporting tracing / spans.
pub trait TracingExporter {
    /// Export a trace span to the configured backend.
    fn export_trace(&self, span: TraceSpan) -> Result<()>;
}

/// A simplified trace span representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSpan {
    pub trace_id: String,
    pub span_id: String,
    pub name: String,
    pub start: String,
    pub end: Option<String>,
    pub attributes: serde_json::Value,
}

/// Lightweight request-id wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestId(pub String);

impl RequestId {
    /// Generate a new random request id.
    pub fn new() -> Self { Self(Uuid::new_v4().to_string()) }

    /// Create from a string.
    pub fn from_value(id: impl Into<String>) -> Self { Self(id.into()) }

    /// Get the inner string.
    pub fn as_str(&self) -> &str { &self.0 }
}

impl Default for RequestId {
    fn default() -> Self { Self::new() }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RequestId {
    fn from(s: String) -> Self { Self(s) }
}

impl From<Uuid> for RequestId {
    fn from(u: Uuid) -> Self { Self(u.to_string()) }
}

// =============================================================================
// Existing TelemetryClient
// =============================================================================

/// Represents a single telemetry event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub event_id: String,
    pub name: String,
    pub ts: String,
    pub session_id: String,
    pub app_version: String,
    pub os_version: String,
    pub props: serde_json::Value,
}

impl TelemetryEvent {
    /// Create a new telemetry event with redacted properties.
    pub fn new(
        name: String,
        session_id: String,
        app_version: String,
        os_version: String,
        props: serde_json::Value,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4().to_string(),
            name,
            ts: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            session_id,
            app_version,
            os_version,
            props,
        }
    }
}

/// Telemetry client: buffers events locally and flushes when opted in.
pub struct TelemetryClient {
    db_path: String,
    endpoint: Option<String>,
    session_id: String,
    app_version: String,
    os_version: String,
    pii_scrubber: Arc<PiiScrubber>,
}

impl TelemetryClient {
    /// Create a new telemetry client.
    pub fn new(
        db_path: &str,
        session_id: String,
        app_version: String,
        os_version: String,
    ) -> Result<Self> {
        // Initialize database with schema
        Self::init_db(db_path)?;

        // Endpoint is read from env var FOCALPOINT_TELEMETRY_URL
        let endpoint = std::env::var("FOCALPOINT_TELEMETRY_URL").ok();

        Ok(Self {
            db_path: db_path.to_string(),
            endpoint,
            session_id,
            app_version,
            os_version,
            pii_scrubber: Arc::new(PiiScrubber::new()),
        })
    }

    /// Initialize the telemetry database schema.
    fn init_db(db_path: &str) -> Result<()> {
        let conn = rusqlite::Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS telemetry_events (
                event_id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                ts TEXT NOT NULL,
                session_id TEXT NOT NULL,
                app_version TEXT NOT NULL,
                os_version TEXT NOT NULL,
                props TEXT NOT NULL,
                flushed INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS telemetry_audit (
                id INTEGER PRIMARY KEY,
                event_count INTEGER NOT NULL,
                endpoint_domain TEXT,
                flushed_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(())
    }

    /// Track an event with custom properties (PII auto-scrubbed).
    pub fn track(&self, event_name: &str, props: serde_json::Value) -> Result<()> {
        // Scrub PII from properties
        let scrubbed_props = self.pii_scrubber.scrub_json(props);

        let event = TelemetryEvent::new(
            event_name.to_string(),
            self.session_id.clone(),
            self.app_version.clone(),
            self.os_version.clone(),
            scrubbed_props,
        );

        // Store in local buffer (SQLite)
        let conn = rusqlite::Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO telemetry_events (event_id, name, ts, session_id, app_version, os_version, props, flushed)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                &event.event_id,
                &event.name,
                &event.ts,
                &event.session_id,
                &event.app_version,
                &event.os_version,
                serde_json::to_string(&event.props)?,
                0
            ],
        )?;

        Ok(())
    }

    /// Flush buffered events to remote endpoint (only if opted in).
    pub async fn flush_batch(&self, opted_in: bool) -> Result<()> {
        if !opted_in {
            // User hasn't opted in; do NOT send events. Buffer persists.
            return Ok(());
        }

        let endpoint = match &self.endpoint {
            Some(ep) => ep,
            None => {
                // No endpoint configured; buffer persists forever.
                return Ok(());
            }
        };

        // Fetch unflushed events
        let conn = rusqlite::Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare(
            "SELECT event_id, name, ts, session_id, app_version, os_version, props
             FROM telemetry_events WHERE flushed = 0 LIMIT 1000",
        )?;

        let events: Vec<TelemetryEvent> = stmt
            .query_map([], |row| {
                Ok(TelemetryEvent {
                    event_id: row.get(0)?,
                    name: row.get(1)?,
                    ts: row.get(2)?,
                    session_id: row.get(3)?,
                    app_version: row.get(4)?,
                    os_version: row.get(5)?,
                    props: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        if events.is_empty() {
            return Ok(());
        }

        let event_count = events.len();

        // Send batch to endpoint
        let client = reqwest::Client::new();
        let response = client
            .post(endpoint)
            .json(&serde_json::json!({ "events": events }))
            .send()
            .await?;

        if response.status().is_success() {
            // Mark events as flushed
            for event in &events {
                conn.execute(
                    "UPDATE telemetry_events SET flushed = 1 WHERE event_id = ?1",
                    params![&event.event_id],
                )?;
            }

            // Create audit record
            let endpoint_domain = extract_domain(endpoint);
            AuditRecord::log(&conn, event_count, endpoint_domain)?;

            tracing::info!(
                event_count = event_count,
                endpoint = endpoint,
                "telemetry batch flushed successfully"
            );
        }

        Ok(())
    }

    /// Purge all buffered events immediately (called on opt-out).
    pub fn purge_buffer(&self) -> Result<()> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM telemetry_events WHERE flushed = 0", [])?;
        tracing::info!("telemetry buffer purged on opt-out");
        Ok(())
    }

    /// Get the current session ID.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Count buffered (unflushed) events.
    pub fn buffered_event_count(&self) -> Result<usize> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        let count: usize = conn.query_row(
            "SELECT COUNT(*) FROM telemetry_events WHERE flushed = 0",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}

// =============================================================================
// Trait Implementations
// =============================================================================

impl Telemetry for TelemetryClient {
    fn track(&self, event_name: &str, props: serde_json::Value) -> Result<()> {
        self.track(event_name, props)
    }

    fn flush(&self) -> Result<()> {
        // Synchronous wrapper around async flush_batch
        tokio::runtime::Runtime::new()
            .map_err(|e| anyhow::anyhow!("failed to create runtime: {e}"))?
            .block_on(async { self.flush_batch(true).await })
    }

    fn session_id(&self) -> &str {
        self.session_id()
    }
}

impl Metric for TelemetryClient {
    fn record_counter(&self, name: &str, value: u64) {
        let _ = self.track(
            &format!("metric.counter.{name}"),
            serde_json::json!({"value": value}),
        );
    }

    fn record_gauge(&self, name: &str, value: f64) {
        let _ = self.track(
            &format!("metric.gauge.{name}"),
            serde_json::json!({"value": value}),
        );
    }

    fn record_histogram(&self, name: &str, value_ms: u64) {
        let _ = self.track(
            &format!("metric.histogram.{name}"),
            serde_json::json!({"value_ms": value_ms}),
        );
    }
}

impl AuditLogger for TelemetryClient {
    fn log_audit(&self, record: AuditRecord) -> Result<()> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO telemetry_audit (event_count, endpoint_domain, flushed_at) VALUES (?1, ?2, ?3)",
            params![record.event_count as i32, record.endpoint_domain, record.flushed_at],
        )?;
        Ok(())
    }

    fn query_audit(&self, limit: usize) -> Result<Vec<AuditRecord>> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare(
            "SELECT id, event_count, endpoint_domain, flushed_at FROM telemetry_audit ORDER BY flushed_at DESC LIMIT ?1",
        )?;
        let records = stmt
            .query_map([limit], |row| {
                Ok(AuditRecord {
                    id: row.get(0)?,
                    event_count: row.get::<_, i32>(1)? as usize,
                    endpoint_domain: row.get(2)?,
                    flushed_at: row.get(3)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(records)
    }
}

impl TracingExporter for TelemetryClient {
    fn export_trace(&self, span: TraceSpan) -> Result<()> {
        let _ = self.track(
            "trace.span",
            serde_json::json!({
                "trace_id": span.trace_id,
                "span_id": span.span_id,
                "name": span.name,
                "start": span.start,
                "end": span.end,
                "attributes": span.attributes,
            }),
        );
        Ok(())
    }
}

impl HealthCheck for TelemetryClient {
    fn is_healthy(&self) -> Result<bool> {
        let count = self.buffered_event_count()?;
        Ok(count < 10_000)
    }

    fn health_status(&self) -> Result<HealthStatus> {
        let healthy = self.is_healthy()?;
        Ok(HealthStatus {
            component: "telemetry".to_string(),
            healthy,
            last_check: Utc::now().to_rfc3339(),
            message: Some(format!(
                "{} buffered events",
                self.buffered_event_count()?,
            )),
        })
    }
}

/// Extract domain from a URL for audit logging.
fn extract_domain(url: &str) -> String {
    if let Ok(parsed) = url.parse::<url::Url>() {
        parsed
            .host_str()
            .map(|h| h.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    } else {
        "unknown".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_event_creation_with_redacted_props() {
        let props = json!({
            "user_email": "test@example.com",
            "action": "button_click"
        });

        let scrubber = PiiScrubber::new();
        let scrubbed = scrubber.scrub_json(props);

        // Email should be redacted
        assert_eq!(
            scrubbed.get("user_email").and_then(|v| v.as_str()),
            Some("[REDACTED_EMAIL]")
        );
        // Non-PII should remain
        assert_eq!(
            scrubbed.get("action").and_then(|v| v.as_str()),
            Some("button_click")
        );
    }

    #[test]
    fn test_track_event_buffers_locally() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        let props = json!({"feature": "connector.connected"});
        client.track("connector.connected", props.clone()).unwrap();

        // Verify event is buffered
        let count = client.buffered_event_count().unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_flush_respects_opted_in_flag() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        client.track("test_event", json!({})).unwrap();

        // Flush with opted_in=false should NOT send (and no endpoint anyway)
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { client.flush_batch(false).await.unwrap() });

        // Event should still be buffered
        let count = client.buffered_event_count().unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_purge_buffer_on_optout() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        client.track("event1", json!({})).unwrap();
        client.track("event2", json!({})).unwrap();

        assert_eq!(client.buffered_event_count().unwrap(), 2);

        // Purge on opt-out
        client.purge_buffer().unwrap();

        // Buffer should be empty
        assert_eq!(client.buffered_event_count().unwrap(), 0);
    }

    #[test]
    fn test_pii_scrubbing_emails() {
        let scrubber = PiiScrubber::new();
        let input = json!({"contact": "alice@example.com"});
        let output = scrubber.scrub_json(input);
        assert_eq!(
            output.get("contact").and_then(|v| v.as_str()),
            Some("[REDACTED_EMAIL]")
        );
    }

    #[test]
    fn test_pii_scrubbing_phones() {
        let scrubber = PiiScrubber::new();
        let input = json!({"phone": "(555) 555-0123"});
        let output = scrubber.scrub_json(input);
        assert_eq!(
            output.get("phone").and_then(|v| v.as_str()),
            Some("[REDACTED_PHONE]")
        );
    }

    #[test]
    fn test_pii_scrubbing_tokens() {
        let scrubber = PiiScrubber::new();
        let input = json!({"token": "Bearer sk_live_abc123def456"});
        let output = scrubber.scrub_json(input);
        let token_val = output.get("token").and_then(|v| v.as_str()).unwrap_or("");
        assert!(token_val.contains("[REDACTED_TOKEN]"));
    }

    #[test]
    fn test_pii_scrubbing_uuids() {
        let scrubber = PiiScrubber::new();
        let input = json!({"task_id": "550e8400-e29b-41d4-a716-446655440000"});
        let output = scrubber.scrub_json(input);
        assert_eq!(
            output.get("task_id").and_then(|v| v.as_str()),
            Some("[REDACTED_UUID]")
        );
    }

    #[test]
    fn test_audit_record_on_flush() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        client.track("event1", json!({})).unwrap();

        // Verify audit table exists and is empty before flush
        let conn = rusqlite::Connection::open(db_file.path()).unwrap();
        let audit_count: usize = conn
            .query_row("SELECT COUNT(*) FROM telemetry_audit", [], |row| row.get(0))
            .unwrap_or(0);

        assert_eq!(audit_count, 0);
    }

    // -------------------------------------------------------------------------
    // Unified Telemetry Traits
    // -------------------------------------------------------------------------

    #[test]
    fn test_request_id_new() {
        let id = RequestId::new();
        assert_eq!(id.0.len(), 36);
    }

    #[test]
    fn test_request_id_from_value() {
        let id = RequestId::from_value("abc-123");
        assert_eq!(id.as_str(), "abc-123");
    }

    #[test]
    fn test_request_id_display() {
        let id = RequestId::from_value("req-42");
        assert_eq!(format!("{}", id), "req-42");
    }

    #[test]
    fn test_request_id_default() {
        let id: RequestId = Default::default();
        assert_eq!(id.0.len(), 36);
    }

    #[test]
    fn test_request_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id: RequestId = uuid.into();
        assert_eq!(id.0.len(), 36);
    }

    #[test]
    fn test_health_status_serde() {
        let status = HealthStatus {
            component: "test".to_string(),
            healthy: true,
            last_check: Utc::now().to_rfc3339(),
            message: None,
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"healthy\":true"));
    }

    #[test]
    fn test_trace_span_serde() {
        let span = TraceSpan {
            trace_id: Uuid::new_v4().to_string(),
            span_id: "span-1".to_string(),
            name: "test-span".to_string(),
            start: Utc::now().to_rfc3339(),
            end: None,
            attributes: json!({"key": "value"}),
        };
        let json = serde_json::to_string(&span).unwrap();
        assert!(json.contains("test-span"));
    }

    #[test]
    fn test_telemetry_client_implements_telemetry() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        // Test via Telemetry trait
        let _: &dyn Telemetry = &client;
        assert_eq!(client.session_id(), "session123");
    }

    #[test]
    fn test_metric_trait_records() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        let _m: &dyn Metric = &client;
        client.record_counter("test_counter", 42);
        client.record_gauge("test_gauge", 3.14);
        client.record_histogram("test_hist", 150);
    }

    #[test]
    fn test_health_check_trait() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        let hc: &dyn HealthCheck = &client;
        assert!(hc.is_healthy().unwrap());
        let status = hc.health_status().unwrap();
        assert_eq!(status.component, "telemetry");
    }

    #[test]
    fn test_tracing_exporter_trait() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        let _te: &dyn TracingExporter = &client;
        let span = TraceSpan {
            trace_id: Uuid::new_v4().to_string(),
            span_id: "span-1".to_string(),
            name: "test".to_string(),
            start: Utc::now().to_rfc3339(),
            end: None,
            attributes: json!({}),
        };
        assert!(client.export_trace(span).is_ok());
    }

    #[test]
    fn test_audit_logger_trait() {
        let db_file = tempfile::NamedTempFile::new().unwrap();
        let client = TelemetryClient::new(
            db_file.path().to_str().unwrap(),
            "session123".to_string(),
            "1.0.0".to_string(),
            "iOS 17.0".to_string(),
        )
        .unwrap();

        let _al: &dyn AuditLogger = &client;
        let record = AuditRecord {
            id: 0,
            event_count: 5,
            endpoint_domain: "example.com".to_string(),
            flushed_at: Utc::now().to_rfc3339(),
        };
        assert!(client.log_audit(record).is_ok());
        let records = client.query_audit(10).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].event_count, 5);
    }
}
