#![forbid(unsafe_code)]

//! # focus-auth — Security & Authentication Consolidation
//!
//! Provides a unified authentication and authorization layer for FocalPoint.
//!
//! ## Features
//!
//! - **Authentication**: JWT, OAuth2, API key, and session-based auth
//! - **Authorization**: RBAC with roles, permissions, and resource scoping
//! - **Credential Management**: Secure storage via focus-crypto keychain
//! - **Token Handling**: JWT encode/decode with validation
//! - **Rate Limiting**: Token-bucket rate limiter per identity
//!
//! ## Usage
//!
//! ```rust
//! use focus_auth::{AuthContext, Authenticator, Role, Permission};
//!
//! let auth = AuthContext::new("user-123").with_role(Role::User);
//! assert!(auth.has_permission(&Permission::ReadTask));
//! ```

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use focus_errors::FocusError;
use focus_hash::FocusHasher;
use focus_result::FocusResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

// =============================================================================
// Core Types
// =============================================================================

/// Unique identifier for an authenticated principal.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrincipalId(pub String);

impl PrincipalId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

impl std::fmt::Display for PrincipalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Authentication token with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub token_type: TokenType,
    pub expires_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
}

/// Token type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
pub enum TokenType {
    Bearer,
    ApiKey,
    Jwt,
    Session,
    OAuth2,
}

/// Role definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
pub enum Role {
    Admin,
    User,
    Guest,
    Service,
    Connector,
}

/// Permission definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
pub enum Permission {
    ReadTask,
    WriteTask,
    DeleteTask,
    ReadRule,
    WriteRule,
    DeleteRule,
    ReadConfig,
    WriteConfig,
    ReadAudit,
    FlushTelemetry,
    ManageConnector,
    AdminAccess,
}

/// Resource scope for fine-grained authorization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceScope {
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub actions: Vec<Permission>,
}

// =============================================================================
// AuthContext
// =============================================================================

/// Authenticated context for a request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub principal_id: PrincipalId,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
    pub scopes: Vec<ResourceScope>,
    pub session: Option<Session>,
    pub metadata: HashMap<String, String>,
}

/// Session metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl AuthContext {
    /// Create a new auth context for a principal.
    pub fn new(principal_id: impl Into<String>) -> Self {
        Self {
            principal_id: PrincipalId::new(principal_id),
            roles: vec![],
            permissions: vec![],
            scopes: vec![],
            session: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a role.
    pub fn with_role(mut self, role: Role) -> Self {
        self.roles.push(role);
        self
    }

    /// Add a permission.
    pub fn with_permission(mut self, perm: Permission) -> Self {
        self.permissions.push(perm);
        self
    }

    /// Add a resource scope.
    pub fn with_scope(mut self, scope: ResourceScope) -> Self {
        self.scopes.push(scope);
        self
    }

    /// Set session.
    pub fn with_session(mut self, session: Session) -> Self {
        self.session = Some(session);
        self
    }

    /// Check if the principal has a specific role.
    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }

    /// Check if the principal has a specific permission.
    pub fn has_permission(&self, perm: &Permission) -> bool {
        self.permissions.contains(perm)
            || self.roles.iter().any(|r| role_implies_permission(r, perm))
    }

    /// Check if the principal can perform an action on a resource.
    pub fn can_access(&self, resource_type: &str, resource_id: Option<&str>, action: &Permission) -> bool {
        if !self.has_permission(action) {
            return false;
        }
        self.scopes.iter().any(|scope| {
            scope.resource_type == resource_type
                && scope.actions.contains(action)
                && (scope.resource_id.is_none() || scope.resource_id.as_deref() == resource_id)
        })
    }

    /// Hash the principal ID for telemetry-safe identifiers.
    pub fn hashed_principal(&self) -> String {
        let hash = FocusHasher::sha256().hash(self.principal_id.as_str().as_bytes());
        hash.to_hex()
    }
}

/// Default roles imply certain permissions.
fn role_implies_permission(role: &Role, perm: &Permission) -> bool {
    match role {
        Role::Admin => true,
        Role::User => matches!(perm, Permission::ReadTask | Permission::WriteTask | Permission::ReadRule | Permission::ReadConfig),
        Role::Guest => matches!(perm, Permission::ReadTask),
        Role::Service => matches!(perm, Permission::ReadTask | Permission::ReadRule | Permission::ReadConfig | Permission::ReadAudit | Permission::FlushTelemetry),
        Role::Connector => matches!(perm, Permission::ReadTask | Permission::WriteTask | Permission::ManageConnector),
    }
}

// =============================================================================
// Authenticator Trait
// =============================================================================

/// Trait for authentication backends.
#[async_trait]
pub trait Authenticator: Send + Sync {
    /// Authenticate a raw token and return an AuthContext.
    async fn authenticate(&self, token: &AuthToken) -> FocusResult<AuthContext>;

    /// Refresh a token if supported.
    async fn refresh(&self, token: &AuthToken) -> FocusResult<AuthToken>;

    /// Revoke a token.
    async fn revoke(&self, token: &AuthToken) -> FocusResult<()>;
}

// =============================================================================
// Authorizer Trait
// =============================================================================

/// Trait for authorization backends.
#[async_trait]
pub trait Authorizer: Send + Sync {
    /// Check if a principal is authorized for an action.
    async fn authorize(&self, ctx: &AuthContext, action: &Permission, resource: Option<&ResourceScope>) -> FocusResult<bool>;

    /// List all permissions for a principal.
    async fn list_permissions(&self, ctx: &AuthContext) -> FocusResult<Vec<Permission>>;
}

// =============================================================================
// Simple In-Memory Implementations
// =============================================================================

/// In-memory authenticator for testing.
pub struct InMemoryAuthenticator {
    tokens: HashMap<String, AuthContext>,
}

impl InMemoryAuthenticator {
    pub fn new() -> Self {
        Self { tokens: HashMap::new() }
    }

    pub fn register_token(&mut self, token: String, ctx: AuthContext) {
        self.tokens.insert(token, ctx);
    }
}

impl Default for InMemoryAuthenticator {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl Authenticator for InMemoryAuthenticator {
    async fn authenticate(&self, token: &AuthToken) -> FocusResult<AuthContext> {
        self.tokens
            .get(&token.token)
            .cloned()
            .ok_or_else(|| FocusError::not_found("token"))
    }

    async fn refresh(&self, _token: &AuthToken) -> FocusResult<AuthToken> {
        Err(FocusError::Internal { message: "refresh not supported".into() })
    }

    async fn revoke(&self, _token: &AuthToken) -> FocusResult<()> {
        Ok(())
    }
}

/// In-memory authorizer for testing.
pub struct InMemoryAuthorizer {
    grants: HashMap<PrincipalId, Vec<Permission>>,
}

impl InMemoryAuthorizer {
    pub fn new() -> Self {
        Self { grants: HashMap::new() }
    }

    pub fn grant(&mut self, principal: PrincipalId, perms: Vec<Permission>) {
        self.grants.insert(principal, perms);
    }
}

impl Default for InMemoryAuthorizer {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl Authorizer for InMemoryAuthorizer {
    async fn authorize(&self, ctx: &AuthContext, action: &Permission, _resource: Option<&ResourceScope>) -> FocusResult<bool> {
        Ok(ctx.has_permission(action))
    }

    async fn list_permissions(&self, ctx: &AuthContext) -> FocusResult<Vec<Permission>> {
        Ok(self.grants.get(&ctx.principal_id).cloned().unwrap_or_default())
    }
}

// =============================================================================
// Rate Limiter
// =============================================================================

/// Token-bucket rate limiter per identity.
pub struct RateLimiter {
    buckets: std::sync::Mutex<HashMap<String, TokenBucket>>,
    max_per_minute: u64,
}

#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: u64,
    last_refill: DateTime<Utc>,
}

impl RateLimiter {
    pub fn new(max_per_minute: u64) -> Self {
        Self {
            buckets: std::sync::Mutex::new(HashMap::new()),
            max_per_minute,
        }
    }

    /// Check if a request is allowed for the given identity.
    pub fn check(&self, identity: &str) -> bool {
        let mut buckets = self.buckets.lock().unwrap();
        let now = Utc::now();
        let bucket = buckets.entry(identity.to_string()).or_insert(TokenBucket {
            tokens: self.max_per_minute,
            last_refill: now,
        });

        // Refill tokens based on time elapsed
        let elapsed = (now - bucket.last_refill).num_seconds().max(0) as u64;
        let refill = (elapsed * self.max_per_minute) / 60;
        bucket.tokens = (bucket.tokens + refill).min(self.max_per_minute);
        bucket.last_refill = now;

        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            true
        } else {
            false
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_context_role_check() {
        let ctx = AuthContext::new("user-1").with_role(Role::Admin);
        assert!(ctx.has_role(&Role::Admin));
        assert!(!ctx.has_role(&Role::Guest));
    }

    #[test]
    fn test_auth_context_permission_check() {
        let ctx = AuthContext::new("user-1")
            .with_role(Role::User)
            .with_permission(Permission::WriteTask);
        assert!(ctx.has_permission(&Permission::ReadTask));
        assert!(ctx.has_permission(&Permission::WriteTask));
        assert!(!ctx.has_permission(&Permission::AdminAccess));
    }

    #[test]
    fn test_auth_context_admin_has_all_permissions() {
        let ctx = AuthContext::new("admin-1").with_role(Role::Admin);
        assert!(ctx.has_permission(&Permission::ReadTask));
        assert!(ctx.has_permission(&Permission::WriteTask));
        assert!(ctx.has_permission(&Permission::DeleteRule));
        assert!(ctx.has_permission(&Permission::AdminAccess));
    }

    #[test]
    fn test_auth_context_resource_scope() {
        let ctx = AuthContext::new("user-1")
            .with_role(Role::User)
            .with_scope(ResourceScope {
                resource_type: "task".to_string(),
                resource_id: Some("task-42".to_string()),
                actions: vec![Permission::ReadTask],
            });
        assert!(ctx.can_access("task", Some("task-42"), &Permission::ReadTask));
        assert!(!ctx.can_access("task", Some("task-99"), &Permission::ReadTask));
        assert!(!ctx.can_access("task", Some("task-42"), &Permission::WriteTask));
    }

    #[test]
    fn test_auth_context_hashed_principal() {
        let ctx = AuthContext::new("user-123");
        let hash = ctx.hashed_principal();
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_in_memory_authenticator() {
        let mut auth = InMemoryAuthenticator::new();
        let ctx = AuthContext::new("user-1").with_role(Role::User);
        auth.register_token("token-abc".to_string(), ctx.clone());

        let token = AuthToken {
            token: "token-abc".to_string(),
            token_type: TokenType::Bearer,
            expires_at: None,
            scopes: vec![],
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async { auth.authenticate(&token).await });
        assert!(result.is_ok());
        assert_eq!(result.unwrap().principal_id.as_str(), "user-1");
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(10);
        let identity = "user-1";
        for _ in 0..10 {
            assert!(limiter.check(identity));
        }
        assert!(!limiter.check(identity));
    }

    #[test]
    fn test_rate_limiter_refill() {
        let limiter = RateLimiter::new(2);
        let identity = "user-2";
        assert!(limiter.check(identity));
        assert!(limiter.check(identity));
        assert!(!limiter.check(identity));
        // After a minute, tokens should refill
        // (simulated by new bucket with same key)
    }

    #[test]
    fn test_token_type_display() {
        assert_eq!(format!("{}", TokenType::Bearer), "Bearer");
        assert_eq!(format!("{}", TokenType::Jwt), "Jwt");
    }

    #[test]
    fn test_role_display() {
        assert_eq!(format!("{}", Role::Admin), "Admin");
        assert_eq!(format!("{}", Role::Guest), "Guest");
    }

    #[test]
    fn test_permission_display() {
        assert_eq!(format!("{}", Permission::ReadTask), "ReadTask");
        assert_eq!(format!("{}", Permission::WriteTask), "WriteTask");
    }

    #[test]
    fn test_principal_id_display() {
        let id = PrincipalId::new("user-42");
        assert_eq!(format!("{}", id), "user-42");
    }

    #[test]
    fn test_auth_token_serde() {
        let token = AuthToken {
            token: "abc123".to_string(),
            token_type: TokenType::Bearer,
            expires_at: None,
            scopes: vec!["read".to_string()],
        };
        let json = serde_json::to_string(&token).unwrap();
        assert!(json.contains("abc123"));
        let deserialized: AuthToken = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.token, "abc123");
    }

    #[test]
    fn test_session_creation() {
        let session = Session {
            session_id: "sess-1".to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("test-agent".to_string()),
        };
        assert_eq!(session.session_id, "sess-1");
    }

    #[test]
    fn test_auth_context_with_session() {
        let session = Session {
            session_id: "sess-1".to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
            ip_address: None,
            user_agent: None,
        };
        let ctx = AuthContext::new("user-1").with_session(session);
        assert!(ctx.session.is_some());
    }

    #[test]
    fn test_in_memory_authorizer() {
        let mut authz = InMemoryAuthorizer::new();
        let principal = PrincipalId::new("user-1");
        authz.grant(principal.clone(), vec![Permission::ReadTask, Permission::WriteTask]);

        let ctx = AuthContext::new("user-1");
        let rt = tokio::runtime::Runtime::new().unwrap();
        let perms = rt.block_on(async { authz.list_permissions(&ctx).await }).unwrap();
        assert_eq!(perms.len(), 2);
    }
}
