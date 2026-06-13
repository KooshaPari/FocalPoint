//! Shared result type for the FocalPoint workspace.
//!
//! Re-export this instead of defining `pub type Result<T> = ...` in every
//! crate. Use `Result<T>` for the default `FocusError`, or `Result<T, E>`
//! when you need a different error type.

pub use focus_errors::FocusError;

/// Shared result type alias defaulting to `FocusError`.
pub type Result<T, E = FocusError> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::*;

    fn ok_result() -> Result<i32> {
        Ok(42)
    }

    fn err_result() -> Result<i32> {
        Err(FocusError::NotFound("x".into()))
    }

    #[test]
    fn result_ok() {
        assert_eq!(ok_result().unwrap(), 42);
    }

    #[test]
    fn result_err() {
        assert!(err_result().is_err());
    }

    fn custom_result() -> Result<i32, std::io::Error> {
        Ok(7)
    }

    #[test]
    fn result_custom_error() {
        assert_eq!(custom_result().unwrap(), 7);
    }
}
