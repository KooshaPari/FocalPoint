//! Common test fixtures for Phenotype crates

use serde::{Deserialize, Serialize};

/// Sample fixture data for contract testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFixture {
    pub id: String,
    pub name: String,
    pub payload: serde_json::Value,
}

impl ContractFixture {
    pub fn sample() -> Self {
        Self {
            id: "test-001".to_string(),
            name: "Test Contract".to_string(),
            payload: serde_json::json!({"key": "value"}),
        }
    }
}

/// Sample fixture data for webhook testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookFixture {
    pub event_type: String,
    pub payload: String,
    pub signature: String,
}

impl WebhookFixture {
    pub fn sample() -> Self {
        Self {
            event_type: "user.created".to_string(),
            payload: r#"{"user_id": "123"}"#.to_string(),
            signature: "sha256=test".to_string(),
        }
    }
}

/// Sample fixture data for auth testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthFixture {
    pub token: String,
    pub user_id: String,
    pub permissions: Vec<String>,
}

impl AuthFixture {
    pub fn sample() -> Self {
        Self {
            token: "test-token".to_string(),
            user_id: "user-123".to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
        }
    }
}
