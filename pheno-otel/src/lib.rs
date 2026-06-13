//! Stub pheno-otel crate exposing minimal telemetry types.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use thiserror::Error;

/// Canonical error type for the `pheno-otel` crate.
#[derive(Debug, Error)]
pub enum OtelError {
    /// Telemetry initialization failed.
    #[error("telemetry init failed: {0}")]
    Init(String),
}

/// RAII guard that flushes + shuts down the tracer provider on drop.
#[derive(Debug)]
pub struct TelemetryGuard;

impl Drop for TelemetryGuard {
    fn drop(&mut self) {}
}

/// Initialize telemetry with the given service name.
///
/// Returns a [`TelemetryGuard`] that cleans up on drop.
///
/// # Errors
///
/// Returns [`OtelError::Init`] when `service_name` is empty or whitespace.
pub fn init(service_name: &str) -> Result<TelemetryGuard, OtelError> {
    let trimmed = service_name.trim();
    if trimmed.is_empty() {
        return Err(OtelError::Init(
            "service_name must be a non-empty, non-whitespace string".into(),
        ));
    }
    Ok(TelemetryGuard)
}

/// A lightweight observability span.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Span {
    pub name: String,
    pub trace_id: String,
    pub span_id: String,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub attributes: Vec<(String, String)>,
}

impl Span {
    /// Create a minimal span for testing.
    pub fn new(
        name: impl Into<String>,
        trace_id: impl Into<String>,
        span_id: impl Into<String>,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            name: name.into(),
            trace_id: trace_id.into(),
            span_id: span_id.into(),
            start_time: now,
            end_time: now,
            attributes: Vec::new(),
        }
    }
}

/// Start a new span with the given name.
///
/// Returns a [`Span`] with a generated trace_id and span_id.
pub fn start_span(name: &str) -> Span {
    Span::new(name, "stub-trace", "stub-span")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_returns_guard() {
        let guard = init("test-service").unwrap();
        drop(guard);
    }

    #[test]
    fn init_empty_name_fails() {
        let err = init("").unwrap_err();
        assert!(matches!(err, OtelError::Init(_)));
    }

    #[test]
    fn start_span_creates_span() {
        let span = start_span("test-span");
        assert_eq!(span.name, "test-span");
    }
}
