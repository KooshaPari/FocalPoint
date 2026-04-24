//! Bearer token authentication for GraphQL API.
//!
//! Checks `Authorization: Bearer <token>` header against `FOCALPOINT_GRAPHQL_TOKEN` env var.

use axum::http::HeaderMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("missing authorization header")]
    MissingHeader,
    #[error("invalid authorization header format")]
    InvalidFormat,
    #[error("unauthorized: invalid token")]
    InvalidToken,
}

/// Validate Bearer token from request headers.
pub fn validate_bearer(
    headers: &HeaderMap,
    expected_token: &Option<String>,
) -> Result<(), AuthError> {
    // If no token is configured, allow all requests.
    let Some(expected) = expected_token else {
        return Ok(());
    };

    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingHeader)?;

    let bearer = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidFormat)?;

    if bearer != expected.as_str() {
        return Err(AuthError::InvalidToken);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_valid() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", "Bearer secret123".parse().unwrap());
        let result = validate_bearer(&headers, &Some("secret123".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn bearer_invalid_token() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", "Bearer wrong".parse().unwrap());
        let result = validate_bearer(&headers, &Some("secret123".to_string()));
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[test]
    fn bearer_missing() {
        let headers = HeaderMap::new();
        let result = validate_bearer(&headers, &Some("secret123".to_string()));
        assert!(matches!(result, Err(AuthError::MissingHeader)));
    }

    #[test]
    fn no_token_configured_allows_all() {
        let headers = HeaderMap::new();
        let result = validate_bearer(&headers, &None);
        assert!(result.is_ok());
    }
}
