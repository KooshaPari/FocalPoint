//! Bearer token authentication for upload endpoint.

/// Extract bearer token from Authorization header.
pub fn extract_bearer_token(auth_header: Option<&str>) -> Option<String> {
    auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

/// Validate bearer token against the configured token.
pub fn validate_token(provided: &str, expected: &str) -> bool {
    // Constant-time comparison to prevent timing attacks
    provided.len() == expected.len()
        && provided
            .as_bytes()
            .iter()
            .zip(expected.as_bytes())
            .fold(0, |acc, (a, b)| acc | (a ^ b))
            == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_bearer_token_succeeds() {
        let header = "Bearer my-secret-token";
        assert_eq!(extract_bearer_token(Some(header)), Some("my-secret-token".to_string()));
    }

    #[test]
    fn extract_bearer_token_rejects_invalid_format() {
        assert_eq!(extract_bearer_token(Some("Basic abc123")), None);
        assert_eq!(extract_bearer_token(None), None);
    }

    #[test]
    fn validate_token_succeeds() {
        assert!(validate_token("secret", "secret"));
    }

    #[test]
    fn validate_token_rejects_different_tokens() {
        assert!(!validate_token("secret", "wrong"));
    }

    #[test]
    fn validate_token_rejects_different_lengths() {
        assert!(!validate_token("short", "much-longer-token"));
    }
}
