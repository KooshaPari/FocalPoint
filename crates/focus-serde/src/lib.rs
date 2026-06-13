//! Shared serialization utilities for the FocalPoint ecosystem.

use focus_errors::FocusError;
use focus_result::FocusResult;
use serde::{de::DeserializeOwned, Serialize};

/// Serialize a value to a canonical JSON string.
pub fn to_json<T: Serialize>(value: &T) -> FocusResult<String> {
    serde_json::to_string(value).map_err(|e| FocusError::Serialization {
        format: "json".into(),
        message: e.to_string(),
    })
}

/// Serialize a value to a pretty-printed JSON string.
pub fn to_json_pretty<T: Serialize>(value: &T) -> FocusResult<String> {
    serde_json::to_string_pretty(value).map_err(|e| FocusError::Serialization {
        format: "json".into(),
        message: e.to_string(),
    })
}

/// Parse a JSON string into a typed value.
pub fn parse_json<T: DeserializeOwned>(input: &str) -> FocusResult<T> {
    serde_json::from_str(input).map_err(|e| FocusError::Deserialization {
        format: "json".into(),
        message: e.to_string(),
    })
}

/// Parse a JSON byte slice into a typed value.
pub fn parse_json_bytes<T: DeserializeOwned>(input: &[u8]) -> FocusResult<T> {
    serde_json::from_slice(input).map_err(|e| FocusError::Deserialization {
        format: "json".into(),
        message: e.to_string(),
    })
}

/// Serialize a value to a JSON byte vector.
pub fn to_json_bytes<T: Serialize>(value: &T) -> FocusResult<Vec<u8>> {
    serde_json::to_vec(value).map_err(|e| FocusError::Serialization {
        format: "json".into(),
        message: e.to_string(),
    })
}

/// JSON serializer type for generic contexts.
#[derive(Debug, Clone, Copy, Default)]
pub struct JsonSerializer;

impl JsonSerializer {
    pub fn new() -> Self {
        Self
    }

    pub fn serialize<T: Serialize>(&self, value: &T) -> FocusResult<String> {
        to_json(value)
    }

    pub fn deserialize<T: DeserializeOwned>(&self, input: &str) -> FocusResult<T> {
        parse_json(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestConfig {
        name: String,
        value: u32,
    }

    #[test]
    fn test_json_roundtrip() {
        let cfg = TestConfig { name: "test".into(), value: 42 };
        let json = to_json(&cfg).unwrap();
        let back: TestConfig = parse_json(&json).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn test_json_pretty() {
        let cfg = TestConfig { name: "test".into(), value: 42 };
        let pretty = to_json_pretty(&cfg).unwrap();
        assert!(pretty.contains('\n'));
        let back: TestConfig = parse_json(&pretty).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn test_json_bytes_roundtrip() {
        let cfg = TestConfig { name: "test".into(), value: 42 };
        let bytes = to_json_bytes(&cfg).unwrap();
        let back: TestConfig = parse_json_bytes(&bytes).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn test_json_deserialization_error() {
        let result: FocusResult<TestConfig> = parse_json("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_json_serializer_type() {
        let ser = JsonSerializer::new();
        let cfg = TestConfig { name: "type-test".into(), value: 99 };
        let json = ser.serialize(&cfg).unwrap();
        let back: TestConfig = ser.deserialize(&json).unwrap();
        assert_eq!(back, cfg);
    }
}
