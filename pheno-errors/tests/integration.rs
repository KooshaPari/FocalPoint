//! Integration tests for the `pheno_errors::{Error, Result}` surface.
//!
//! These tests exercise the public API the way downstream consumer crates
//! will: from a separate compilation unit, importing via the crate name.

use std::error::Error as StdError;

use pheno_errors::{Error, Result};

#[test]
fn test_error_creation() {
    // Direct construction (the newtype's only field is `pub`).
    let direct = Error("from struct literal".to_string());
    assert_eq!(direct.as_str(), "from struct literal");
    assert_eq!(direct.to_string(), "from struct literal");

    // Error::new constructor
    let from_new = Error::new("from new()");
    assert_eq!(from_new.as_str(), "from new()");

    // From<&str>
    let from_str: Error = "from &str".into();
    assert_eq!(from_str.as_str(), "from &str");

    // From<String>
    let from_string: Error = String::from("from String").into();
    assert_eq!(from_string.as_str(), "from String");

    // into_string consumes and returns the inner String
    let consumed = Error("consume me".to_string()).into_string();
    assert_eq!(consumed, "consume me");

    // It's a std::error::Error (via the manual impl in lib.rs)
    fn assert_is_std_error<E: StdError>(_: &E) {}
    let e = Error("is error".to_string());
    assert_is_std_error(&e);
    assert!(StdError::source(&e).is_none(), "Error has no source");
}

#[test]
fn test_error_in_result() {
    // Ok path
    let ok: Result<i32> = Ok(7);
    let unwrapped = ok.unwrap();
    assert_eq!(unwrapped, 7);

    // Err path with map_err
    fn parse_id(s: &str) -> Result<u64> {
        s.parse().map_err(|e| Error(format!("bad id {s:?}: {e}")))
    }

    let parsed = parse_id("12345").unwrap();
    assert_eq!(parsed, 12345);

    let failed = parse_id("not-a-number").unwrap_err();
    assert_eq!(failed.as_str(), "bad id \"not-a-number\": invalid digit found in string");

    // ? propagation across an inner fn
    fn double_id(s: &str) -> Result<u64> {
        let id = parse_id(s)?;
        Ok(id * 2)
    }
    assert_eq!(double_id("21").unwrap(), 42);
    assert!(double_id("nope").is_err());

    // The type alias is just `Result<T>` = `std::result::Result<T, Error>`
    fn explicit_type() -> std::result::Result<&'static str, Error> {
        Ok("aliased")
    }
    assert_eq!(explicit_type().unwrap(), "aliased");
}
