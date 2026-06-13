//! Unified error type for the FocalPoint workspace.
//!
//! This crate eliminates per-crate `Error` enums by providing a single
//! `FocusError` that covers all domain concerns. Each crate maps its
//! legacy variants into the relevant `FocusError` category via `From`
//! impls.

use thiserror::Error;

/// Unified error type for all FocalPoint crates.
#[derive(Debug, Error)]
pub enum FocusError {
    // ── Domain / invariant ──
    #[error("invariant violation: {0}")]
    Invariant(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),

    // ── Auth / network (connector) ──
    #[error("auth: {0}")]
    Auth(String),
    #[error("network: {0}")]
    Network(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("rate_limited: retry after {0}s")]
    RateLimited(u64),
    #[error("rate_limited_until: {0}")]
    RateLimitedUntil(chrono::DateTime<chrono::Utc>),
    #[error("schema: {0}")]
    Schema(String),

    // ── Penalty / reward ──
    #[error("insufficient bypass budget: {balance} < {requested}")]
    InsufficientBypass { balance: i64, requested: i64 },
    #[error("negative amount: {0}")]
    NegativeAmount(i64),

    // ── IO / serialization ──
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization: {0}")]
    Serialization(String),
    #[error("deserialization: {0}")]
    Deserialization(String),

    // ── Crypto ──
    #[error("crypto: {0}")]
    Crypto(String),

    // ── Catch-all for crate-specific edges during migration ──
    #[error("{context}: {message}")]
    Context { context: &'static str, message: String },
}

impl FocusError {
    /// Convenience constructor for `Context` variant.
    pub fn context(context: &'static str, message: impl Into<String>) -> Self {
        Self::Context {
            context,
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invariant_display() {
        let e = FocusError::Invariant("user must have device".into());
        assert_eq!(e.to_string(), "invariant violation: user must have device");
    }

    #[test]
    fn insufficient_bypass_display() {
        let e = FocusError::InsufficientBypass {
            balance: 3,
            requested: 10,
        };
        assert_eq!(e.to_string(), "insufficient bypass budget: 3 < 10");
    }

    #[test]
    fn io_from_std_io_error() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "file gone");
        let e: FocusError = io.into();
        assert!(matches!(e, FocusError::Io(_)));
    }

    #[test]
    fn context_variant() {
        let e = FocusError::context("focus-penalties", "overflow");
        assert_eq!(e.to_string(), "focus-penalties: overflow");
    }
}
