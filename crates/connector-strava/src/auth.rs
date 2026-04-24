//! Strava OAuth2 authorization code flow, token storage (keychain-backed).

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;

/// OAuth2 token returned by Strava's token endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StravaToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub scope: String,
    pub token_type: String,
    pub acquired_at: DateTime<Utc>,
}

impl StravaToken {
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let age = (now - self.acquired_at).num_seconds() as u64;
        age >= self.expires_in - 300 // 5 min buffer
    }
}

/// TokenStore trait — abstract over memory vs. keychain.
#[async_trait]
pub trait TokenStore: Send + Sync {
    async fn get(&self, key: &str) -> Option<StravaToken>;
    async fn put(&self, key: &str, token: StravaToken);
    async fn delete(&self, key: &str);
}

/// In-memory token store (development/testing).
pub struct InMemoryTokenStore {
    tokens: Mutex<HashMap<String, StravaToken>>,
}

impl InMemoryTokenStore {
    pub fn new() -> Self {
        Self {
            tokens: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryTokenStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TokenStore for InMemoryTokenStore {
    async fn get(&self, key: &str) -> Option<StravaToken> {
        self.tokens.lock().await.get(key).cloned()
    }

    async fn put(&self, key: &str, token: StravaToken) {
        self.tokens.lock().await.insert(key.to_string(), token);
    }

    async fn delete(&self, key: &str) {
        self.tokens.lock().await.remove(key);
    }
}

/// Keychain-backed token store (production iOS/macOS).
/// Note: iOS requires the keychain service to be wired up via FFI;
/// this is a stub that delegates to a mock impl for testing.
pub struct KeychainTokenStore;

impl KeychainTokenStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for KeychainTokenStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TokenStore for KeychainTokenStore {
    async fn get(&self, _key: &str) -> Option<StravaToken> {
        // TODO: Call into iOS keychain via FFI (crates/focus-ffi).
        // For now, always return None to trigger oauth flow.
        None
    }

    async fn put(&self, _key: &str, _token: StravaToken) {
        // TODO: Call into iOS keychain via FFI.
    }

    async fn delete(&self, _key: &str) {
        // TODO: Call into iOS keychain via FFI.
    }
}

/// Strava OAuth2 handler — manages authorization flow.
pub struct StravaOAuth2 {
    client_id: String,
    client_secret: String,
}

impl StravaOAuth2 {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    /// Generate the authorization URL for OAuth2 code flow.
    pub fn authorize_url(&self, redirect_uri: &str, state: &str) -> String {
        format!(
            "https://www.strava.com/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.client_id,
            urlencoding::encode(redirect_uri),
            "read,activity:read",
            state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-STRAVA-AUTH-001
    #[tokio::test]
    async fn in_memory_token_store() {
        let store = InMemoryTokenStore::new();
        let token = StravaToken {
            access_token: "test_token".into(),
            refresh_token: Some("test_refresh".into()),
            expires_in: 3600,
            scope: "read,activity:read".into(),
            token_type: "Bearer".into(),
            acquired_at: Utc::now(),
        };

        store.put("athlete:123", token.clone()).await;
        let retrieved = store.get("athlete:123").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().access_token, "test_token");
    }

    // Traces to: FR-STRAVA-AUTH-001
    #[tokio::test]
    async fn token_expiration_check() {
        let expired_token = StravaToken {
            access_token: "expired".into(),
            refresh_token: None,
            expires_in: 60, // 60 seconds
            scope: "read".into(),
            token_type: "Bearer".into(),
            acquired_at: Utc::now() - chrono::Duration::minutes(2),
        };

        assert!(expired_token.is_expired());
    }
}
