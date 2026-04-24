//! Linear token auth — PAT/OAuth2 token storage trait and implementation.

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Token storage contract for Linear API tokens.
#[async_trait]
pub trait TokenStore: Send + Sync {
    async fn get_token(&self) -> Option<String>;
    async fn set_token(&self, token: String);
}

/// In-memory token store (ephemeral).
pub struct InMemoryTokenStore {
    token: Arc<Mutex<Option<String>>>,
}

impl InMemoryTokenStore {
    pub fn new() -> Self {
        Self {
            token: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl TokenStore for InMemoryTokenStore {
    async fn get_token(&self) -> Option<String> {
        self.token.lock().await.clone()
    }

    async fn set_token(&self, token: String) {
        *self.token.lock().await = Some(token);
    }
}

/// Linear token auth helper (supports PAT or OAuth2 access token).
pub struct LinearAuth {
    pub token: String,
}

impl LinearAuth {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    pub fn bearer_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-LINEAR-AUTH-001
    #[tokio::test]
    async fn in_memory_token_store_set_get() {
        let store = InMemoryTokenStore::new();
        assert!(store.get_token().await.is_none());
        store.set_token("test-pat".into()).await;
        assert_eq!(store.get_token().await, Some("test-pat".into()));
    }

    // Traces to: FR-LINEAR-AUTH-001
    #[test]
    fn linear_auth_bearer_header() {
        let auth = LinearAuth::new("my-pat-token");
        assert_eq!(auth.bearer_header(), "Bearer my-pat-token");
    }
}
