//! # pheno-errors
//!
//! Minimal [`Error`] newtype + [`Result`] alias for the `pheno-*` fleet.
//!
//! This crate intentionally ships zero dependencies: the entire surface is
//! a `String` wrapper and a `Result` type alias. Consumers that need a
//! richer taxonomy should layer on `thiserror`/`anyhow` at the call site.
//!
//! ## Example
//!
//! ```rust
//! use pheno_errors::{Error, Result};
//!
//! fn parse_port(s: &str) -> Result<u16> {
//!     s.parse().map_err(|e| Error(format!("invalid port {s:?}: {e}")))
//! }
//!
//! let port = parse_port("8080").unwrap();
//! assert_eq!(port, 8080);
//! ```

use std::fmt;

/// The canonical fleet-wide error type.
///
/// A thin newtype wrapper around [`String`] so consumers don't have to
/// depend on `thiserror`/`anyhow` just to bubble up an error message.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error(pub String);

impl Error {
    /// Construct a new [`Error`] from any string-like value.
    pub fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }

    /// Borrow the wrapped message as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the error and return the wrapped message.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Self(msg.to_owned())
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self(msg)
    }
}

/// `Result<T, Error>` — the canonical return type for fallible functions
/// in the `pheno-*` fleet.
pub type Result<T> = std::result::Result<T, Error>;

// ---------------------------------------------------------------------------
// Inline unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let e = Error("boom".to_string());
        assert_eq!(e.to_string(), "boom");
        assert_eq!(format!("{e}"), "boom");
        assert_eq!(e.as_str(), "boom");
    }

    #[test]
    fn test_error_equality() {
        let a = Error("same".to_string());
        let b = Error("same".to_string());
        let c = Error("different".to_string());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_result_ok() {
        let ok: Result<i32> = Ok(42);
        assert_eq!(ok.unwrap(), 42);

        let err: Result<i32> = Err(Error("nope".to_string()));
        assert_eq!(err.unwrap_err().as_str(), "nope");

        // From<&str> and From<String> impls
        let from_str: Error = "via &str".into();
        let from_string: Error = String::from("via String").into();
        assert_eq!(from_str.as_str(), "via &str");
        assert_eq!(from_string.as_str(), "via String");

        // Error::new convenience
        let new_err = Error::new("convenience");
        assert_eq!(new_err.as_str(), "convenience");
        assert_eq!(new_err.into_string(), "convenience");
    }
}
