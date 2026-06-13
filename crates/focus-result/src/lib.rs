//! Shared Result type alias for FocalPoint crates.
//!
//! Provides a consistent `Result<T, E>` type across all `focus-*` and
//! `phenotype-*` crates, defaulting to the shared [`Result`] and [`FocusResult`] types.

pub use focus_errors::{FocusResult, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_type() {
        fn compute() -> Result<i32> {
            Ok(42)
        }
        assert_eq!(compute().unwrap(), 42);
    }

    #[test]
    fn test_focus_result_type() {
        fn compute() -> FocusResult<i32> {
            Ok(42)
        }
        assert_eq!(compute().unwrap(), 42);
    }
}
