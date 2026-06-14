#![forbid(unsafe_code)]

//! Pheno Tracing — A port-driven distributed tracing crate
//!
//! Provides a clean port/adapter boundary for telemetry integration.

pub mod adapters;
pub mod port;

pub use port::{SpanId, SpanKind, TraceId, TraceOperation, TracePort, TraceResult};
