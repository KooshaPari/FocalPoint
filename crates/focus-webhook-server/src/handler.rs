use async_trait::async_trait;
use connector_github::webhook::GitHubWebhookHandler;
use focus_connectors::{
    signature_verifiers::{CanvasLtiVerifier, GCalChannelVerifier, GitHubHmacVerifier, SignatureVerifier},
    ConnectorError, Result, WebhookDelivery, WebhookHandler, WebhookRegistry,
};
use focus_events::NormalizedEvent;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum WebhookError {
    SignatureInvalid,
    UnknownConnector,
    ProcessingFailed(String),
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookError::SignatureInvalid => write!(f, "signature verification failed"),
            WebhookError::UnknownConnector => write!(f, "unknown connector"),
            WebhookError::ProcessingFailed(msg) => write!(f, "processing failed: {}", msg),
        }
    }
}

impl std::error::Error for WebhookError {}

/// Handle a generic webhook delivery by routing to the registered handler.
pub async fn handle_webhook(
    registry: &WebhookRegistry,
    connector_id: &str,
    body: Vec<u8>,
) -> std::result::Result<Vec<NormalizedEvent>, WebhookError> {
    let handler = registry
        .get(connector_id)
        .ok_or(WebhookError::UnknownConnector)?;

    let delivery = WebhookDelivery {
        connector_id: connector_id.to_string(),
        kind: "push".to_string(), // TODO: extract from headers
        headers: HashMap::new(),   // TODO: extract from request headers
        body,
        received_at: chrono::Utc::now(),
    };

    handler
        .handle(&delivery)
        .await
        .map_err(|e| WebhookError::ProcessingFailed(e.to_string()))
}

// ---------------------------------------------------------------------------
// Per-provider handler implementations
// ---------------------------------------------------------------------------

/// GitHub webhook handler with HMAC verification.
pub struct GitHubHandlerImpl {
    pub account_id: Uuid,
    pub verifier: Arc<GitHubHmacVerifier>,
}

#[async_trait]
impl WebhookHandler for GitHubHandlerImpl {
    async fn handle(&self, delivery: &WebhookDelivery) -> Result<Vec<NormalizedEvent>> {
        // Verify signature
        self.verifier
            .verify(&delivery.headers, &delivery.body)
            .await
            .map_err(|_e| ConnectorError::Forbidden("invalid github hmac".to_string()))?;

        // Delegate to GitHub handler
        let handler = GitHubWebhookHandler {
            account_id: self.account_id,
        };
        handler.handle(delivery).await
    }
}

/// Canvas webhook handler with JWT verification (stub).
pub struct CanvasHandlerImpl {
    pub account_id: Uuid,
    pub verifier: Arc<CanvasLtiVerifier>,
}

#[async_trait]
impl WebhookHandler for CanvasHandlerImpl {
    async fn handle(&self, delivery: &WebhookDelivery) -> Result<Vec<NormalizedEvent>> {
        // Verify signature
        self.verifier
            .verify(&delivery.headers, &delivery.body)
            .await
            .map_err(|_e| ConnectorError::Forbidden("invalid canvas jwt".to_string()))?;

        // TODO: map Canvas event payload to NormalizedEvents
        Ok(vec![])
    }
}

/// Google Calendar webhook handler with channel token verification (stub).
pub struct GCalHandlerImpl {
    pub account_id: Uuid,
    pub verifier: Arc<GCalChannelVerifier>,
}

#[async_trait]
impl WebhookHandler for GCalHandlerImpl {
    async fn handle(&self, delivery: &WebhookDelivery) -> Result<Vec<NormalizedEvent>> {
        // Verify signature
        self.verifier
            .verify(&delivery.headers, &delivery.body)
            .await
            .map_err(|_e| ConnectorError::Forbidden("invalid gcal channel token".to_string()))?;

        // TODO: map GCal event payload to NormalizedEvents
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_github_handler_verifies_signature() {
        let account_id = Uuid::new_v4();
        let secret = secrecy::SecretString::new("test-secret".to_string());
        let verifier = Arc::new(GitHubHmacVerifier {
            secret: secret.clone(),
        });

        let handler = GitHubHandlerImpl { account_id, verifier };

        // Create a valid HMAC signature
        use hmac::Mac;
        let body = b"test payload";
        let key = secret.expose_secret().as_bytes();
        let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(key).unwrap();
        mac.update(body);
        let digest = mac.finalize();
        let sig = format!("sha256={}", hex::encode(digest.into_bytes()));

        let mut headers = HashMap::new();
        headers.insert("x-hub-signature-256".to_string(), sig);

        let delivery = WebhookDelivery {
            connector_id: "github".to_string(),
            kind: "push".to_string(),
            headers,
            body: body.to_vec(),
            received_at: chrono::Utc::now(),
        };

        // Should succeed (JSON parse will fail, but signature verification passes)
        let result = handler.handle(&delivery).await;
        // We expect JSON parse error, not signature error
        assert!(matches!(result, Err(ConnectorError::Schema(_))));
    }
}
