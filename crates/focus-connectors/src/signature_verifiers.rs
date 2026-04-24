//! Per-provider webhook signature verification.
//!
//! Each verifier implements constant-time comparison to prevent timing attacks.

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};

/// Signature verifier trait for webhook payloads.
#[async_trait]
pub trait SignatureVerifier: Send + Sync {
    /// Verify the signature of a webhook delivery.
    /// Returns Ok(()) if valid, Err(_) if invalid or verification failed.
    async fn verify(&self, headers: &HashMap<String, String>, body: &[u8]) -> Result<()>;
}

// ---------------------------------------------------------------------------
// GitHub HMAC-SHA256 verifier
// ---------------------------------------------------------------------------

/// GitHub webhook HMAC-SHA256 verifier.
/// Reads `X-Hub-Signature-256` header and compares against HMAC-SHA256(secret, body).
pub struct GitHubHmacVerifier {
    pub secret: SecretString,
}

#[async_trait]
impl SignatureVerifier for GitHubHmacVerifier {
    async fn verify(&self, headers: &HashMap<String, String>, body: &[u8]) -> Result<()> {
        let header_sig = headers
            .get("x-hub-signature-256")
            .ok_or_else(|| anyhow!("missing x-hub-signature-256 header"))?;

        let computed = compute_github_hmac(&self.secret, body)?;
        if constant_time_eq(header_sig.as_bytes(), computed.as_bytes()) {
            Ok(())
        } else {
            Err(anyhow!("github hmac signature mismatch"))
        }
    }
}

fn compute_github_hmac(secret: &SecretString, body: &[u8]) -> Result<String> {
    use hmac::Mac;

    let key = secret.expose_secret().as_bytes();
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(key)?;
    mac.update(body);
    let digest = mac.finalize();
    Ok(format!("sha256={}", hex::encode(digest.into_bytes())))
}

// ---------------------------------------------------------------------------
// Canvas LTI JWT verifier
// ---------------------------------------------------------------------------

/// Canvas LTI JWT verifier (stub implementation).
/// Full implementation requires JWKS fetch + JWT validation.
/// For now: validates `X-Canvas-LTI-JWT` header presence and basic structure.
pub struct CanvasLtiVerifier {
    pub jwks_url: String,
}

#[async_trait]
impl SignatureVerifier for CanvasLtiVerifier {
    async fn verify(&self, headers: &HashMap<String, String>, _body: &[u8]) -> Result<()> {
        // Stub: check header presence
        let jwt = headers
            .get("x-canvas-lti-jwt")
            .ok_or_else(|| anyhow!("missing x-canvas-lti-jwt header"))?;

        // Stub: just check it looks like a JWT (3 dot-separated parts)
        let parts: Vec<&str> = jwt.split('.').collect();
        if parts.len() == 3 {
            // TODO: fetch JWKS from jwks_url, validate signature + iss/aud/exp
            Ok(())
        } else {
            Err(anyhow!("invalid jwt format"))
        }
    }
}

// ---------------------------------------------------------------------------
// Google Calendar channel token verifier
// ---------------------------------------------------------------------------

/// Google Calendar watch channel token verifier.
/// Validates `X-Goog-Channel-Token` matches the registered secret.
pub struct GCalChannelVerifier {
    pub channel_token: SecretString,
}

#[async_trait]
impl SignatureVerifier for GCalChannelVerifier {
    async fn verify(&self, headers: &HashMap<String, String>, _body: &[u8]) -> Result<()> {
        let header_token = headers
            .get("x-goog-channel-token")
            .ok_or_else(|| anyhow!("missing x-goog-channel-token header"))?;

        let expected = self.channel_token.expose_secret();
        if constant_time_eq(header_token.as_bytes(), expected.as_bytes()) {
            // TODO: validate X-Goog-Channel-Id references a known watch channel
            Ok(())
        } else {
            Err(anyhow!("google calendar channel token mismatch"))
        }
    }
}

// ---------------------------------------------------------------------------
// Constant-time comparison
// ---------------------------------------------------------------------------

/// Constant-time comparison using the `subtle` crate to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    use subtle::ConstantTimeEq;

    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_github_hmac_valid_signature() {
        let secret = SecretString::new("my-secret".to_string());
        let body = b"test payload";
        let verifier = GitHubHmacVerifier { secret };

        let computed = compute_github_hmac(&verifier.secret, body).unwrap();
        let mut headers = HashMap::new();
        headers.insert("x-hub-signature-256".to_string(), computed);

        assert!(verifier.verify(&headers, body).await.is_ok());
    }

    #[tokio::test]
    async fn test_github_hmac_tampered_body() {
        let secret = SecretString::new("my-secret".to_string());
        let body = b"test payload";
        let tampered = b"tampered payload";
        let verifier = GitHubHmacVerifier { secret };

        let computed = compute_github_hmac(&verifier.secret, body).unwrap();
        let mut headers = HashMap::new();
        headers.insert("x-hub-signature-256".to_string(), computed);

        assert!(verifier.verify(&headers, tampered).await.is_err());
    }

    #[tokio::test]
    async fn test_github_hmac_missing_header() {
        let secret = SecretString::new("my-secret".to_string());
        let body = b"test payload";
        let verifier = GitHubHmacVerifier { secret };

        let headers = HashMap::new();
        assert!(verifier.verify(&headers, body).await.is_err());
    }

    #[tokio::test]
    async fn test_canvas_lti_valid_jwt() {
        let verifier = CanvasLtiVerifier {
            jwks_url: "https://canvas.example.com/.well-known/jwks.json".to_string(),
        };

        let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let mut headers = HashMap::new();
        headers.insert("x-canvas-lti-jwt".to_string(), jwt.to_string());

        assert!(verifier.verify(&headers, b"").await.is_ok());
    }

    #[tokio::test]
    async fn test_canvas_lti_invalid_jwt_format() {
        let verifier = CanvasLtiVerifier {
            jwks_url: "https://canvas.example.com/.well-known/jwks.json".to_string(),
        };

        let jwt = "not.a.valid.jwt";
        let mut headers = HashMap::new();
        headers.insert("x-canvas-lti-jwt".to_string(), jwt.to_string());

        assert!(verifier.verify(&headers, b"").await.is_err());
    }

    #[tokio::test]
    async fn test_canvas_lti_missing_header() {
        let verifier = CanvasLtiVerifier {
            jwks_url: "https://canvas.example.com/.well-known/jwks.json".to_string(),
        };

        let headers = HashMap::new();
        assert!(verifier.verify(&headers, b"").await.is_err());
    }

    #[tokio::test]
    async fn test_gcal_channel_valid_token() {
        let secret = SecretString::new("channel-secret-123".to_string());
        let verifier = GCalChannelVerifier {
            channel_token: secret,
        };

        let mut headers = HashMap::new();
        headers.insert("x-goog-channel-token".to_string(), "channel-secret-123".to_string());

        assert!(verifier.verify(&headers, b"").await.is_ok());
    }

    #[tokio::test]
    async fn test_gcal_channel_tampered_token() {
        let secret = SecretString::new("channel-secret-123".to_string());
        let verifier = GCalChannelVerifier {
            channel_token: secret,
        };

        let mut headers = HashMap::new();
        headers.insert("x-goog-channel-token".to_string(), "wrong-secret".to_string());

        assert!(verifier.verify(&headers, b"").await.is_err());
    }

    #[tokio::test]
    async fn test_gcal_channel_missing_header() {
        let secret = SecretString::new("channel-secret-123".to_string());
        let verifier = GCalChannelVerifier {
            channel_token: secret,
        };

        let headers = HashMap::new();
        assert!(verifier.verify(&headers, b"").await.is_err());
    }
}
