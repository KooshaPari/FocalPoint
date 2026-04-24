//! FocalPoint Entitlements & Feature Gates
//!
//! Subscription tier management with feature gates for Free, Plus, Pro, and Family tiers.
//! Enforces tier-based limits (max rules, max tasks, connector cadence, voice synthesis, etc.)
//! across all rule evaluation, storage, and coaching surfaces.
//!
//! This crate is the single source of truth for subscription semantics. Every tier gate
//! is evaluated here, ensuring iOS app + Rust backend agree on entitlements.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[cfg(test)]
mod tests;

/// Subscription tier.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Free,
    Plus,
    Pro,
    Family,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tier::Free => write!(f, "free"),
            Tier::Plus => write!(f, "plus"),
            Tier::Pro => write!(f, "pro"),
            Tier::Family => write!(f, "family"),
        }
    }
}

/// Entitlement state: tier, expiry, and receipt signature for verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entitlement {
    pub tier: Tier,
    /// When the subscription expires (None for Free tier, which never expires)
    pub expires_at: Option<DateTime<Utc>>,
    /// Base64-encoded signed receipt from App Store (for server-side verification)
    pub receipt_signature: Option<String>,
}

impl Entitlement {
    /// Create a default Free tier entitlement.
    pub fn free() -> Self {
        Self {
            tier: Tier::Free,
            expires_at: None,
            receipt_signature: None,
        }
    }

    /// Create a Plus/Pro/Family entitlement with expiry.
    pub fn with_tier(tier: Tier, expires_at: DateTime<Utc>, receipt_signature: String) -> Self {
        Self {
            tier,
            expires_at: Some(expires_at),
            receipt_signature: Some(receipt_signature),
        }
    }

    /// Check if subscription is active (not expired).
    pub fn is_active(&self, now: DateTime<Utc>) -> bool {
        match self.expires_at {
            None => true, // Free tier never expires
            Some(exp) => now < exp,
        }
    }

    /// Days until expiry; returns None for Free tier.
    pub fn days_until_expiry(&self, now: DateTime<Utc>) -> Option<i64> {
        self.expires_at.map(|exp| {
            let dur = exp.signed_duration_since(now);
            dur.num_days()
        })
    }
}

/// Feature gate errors.
#[derive(Debug, Error)]
pub enum GateError {
    #[error("Tier limit exceeded for {feature}: {current}/{max}")]
    LimitExceeded {
        feature: String,
        current: u32,
        max: u32,
    },

    #[error("Subscription expired on {expiry}")]
    SubscriptionExpired { expiry: DateTime<Utc> },

    #[error("Invalid entitlement state: {0}")]
    InvalidState(String),

    #[error("Receipt verification failed: {0}")]
    ReceiptVerificationFailed(String),
}

/// Result type for entitlement operations.
pub type GateResult<T> = Result<T, GateError>;

// ============================================================================
// Feature Gates
// ============================================================================

/// Check if user can add another rule.
/// Traces to: FR-ENTITLEMENTS-001
pub fn can_add_rule(current_count: u32, entitlement: &Entitlement) -> GateResult<bool> {
    match entitlement.tier {
        Tier::Free => {
            if current_count >= 3 {
                Err(GateError::LimitExceeded {
                    feature: "rules".to_string(),
                    current: current_count,
                    max: 3,
                })
            } else {
                Ok(true)
            }
        }
        Tier::Plus | Tier::Pro | Tier::Family => Ok(true),
    }
}

/// Check if user can add another task/goal.
/// Traces to: FR-ENTITLEMENTS-002
pub fn can_add_task(current_count: u32, entitlement: &Entitlement) -> GateResult<bool> {
    match entitlement.tier {
        Tier::Free => {
            if current_count >= 3 {
                Err(GateError::LimitExceeded {
                    feature: "tasks".to_string(),
                    current: current_count,
                    max: 3,
                })
            } else {
                Ok(true)
            }
        }
        Tier::Plus | Tier::Pro | Tier::Family => Ok(true),
    }
}

/// Get the connector refresh cadence in minutes.
/// Free: 4 hours (240 min), Plus+: 15 minutes
/// Traces to: FR-ENTITLEMENTS-003
pub fn connector_refresh_cadence_minutes(entitlement: &Entitlement) -> u32 {
    match entitlement.tier {
        Tier::Free => 240,
        Tier::Plus | Tier::Pro | Tier::Family => 15,
    }
}

/// Check if multiple connectors can be active simultaneously.
/// Free: 1, Plus+: 4
/// Traces to: FR-ENTITLEMENTS-004
pub fn max_active_connectors(entitlement: &Entitlement) -> u32 {
    match entitlement.tier {
        Tier::Free => 1,
        Tier::Plus | Tier::Pro | Tier::Family => 4,
    }
}

/// Validate focus session duration (in minutes).
/// Free: fixed 25 min
/// Plus+: 5–180 min (user customizable)
/// Traces to: FR-ENTITLEMENTS-005
pub fn validate_focus_duration(minutes: u32, entitlement: &Entitlement) -> GateResult<()> {
    match entitlement.tier {
        Tier::Free => {
            if minutes == 25 {
                Ok(())
            } else {
                Err(GateError::InvalidState(
                    "Free tier allows only 25-minute focus sessions".to_string(),
                ))
            }
        }
        Tier::Plus | Tier::Pro | Tier::Family => {
            if minutes >= 5 && minutes <= 180 {
                Ok(())
            } else {
                Err(GateError::InvalidState(
                    "Focus duration must be 5–180 minutes".to_string(),
                ))
            }
        }
    }
}

/// Validate break duration (in minutes).
/// Free: fixed 45 min
/// Plus+: 1–60 min (user customizable)
/// Traces to: FR-ENTITLEMENTS-006
pub fn validate_break_duration(minutes: u32, entitlement: &Entitlement) -> GateResult<()> {
    match entitlement.tier {
        Tier::Free => {
            if minutes == 45 {
                Ok(())
            } else {
                Err(GateError::InvalidState(
                    "Free tier allows only 45-minute breaks".to_string(),
                ))
            }
        }
        Tier::Plus | Tier::Pro | Tier::Family => {
            if minutes >= 1 && minutes <= 60 {
                Ok(())
            } else {
                Err(GateError::InvalidState(
                    "Break duration must be 1–60 minutes".to_string(),
                ))
            }
        }
    }
}

/// Check if Coachy voice synthesis is available.
/// Free: false (silent only)
/// Plus: native AVSpeechSynthesizer (basic)
/// Pro/Family: ElevenLabs (premium)
/// Traces to: FR-ENTITLEMENTS-007
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceProvider {
    Silent,
    Native,     // AVSpeechSynthesizer
    ElevenLabs, // Premium
}

pub fn voice_provider(entitlement: &Entitlement) -> VoiceProvider {
    match entitlement.tier {
        Tier::Free => VoiceProvider::Silent,
        Tier::Plus => VoiceProvider::Native,
        Tier::Pro | Tier::Family => VoiceProvider::ElevenLabs,
    }
}

/// Check if Live Activity (lock screen widget) is available.
/// Free: false, Plus+: true
/// Traces to: FR-ENTITLEMENTS-008
pub fn can_use_live_activity(entitlement: &Entitlement) -> bool {
    matches!(
        entitlement.tier,
        Tier::Plus | Tier::Pro | Tier::Family
    )
}

/// Check if HomeKit widget is available.
/// Free: false, Plus+: true
/// Traces to: FR-ENTITLEMENTS-009
pub fn can_use_homekit_widget(entitlement: &Entitlement) -> bool {
    matches!(
        entitlement.tier,
        Tier::Plus | Tier::Pro | Tier::Family
    )
}

/// Get audit retention in days.
/// Free: 7, Plus: 90, Pro: 180, Family: 365
/// Traces to: FR-ENTITLEMENTS-010
pub fn audit_retention_days(entitlement: &Entitlement) -> u32 {
    match entitlement.tier {
        Tier::Free => 7,
        Tier::Plus => 90,
        Tier::Pro => 180,
        Tier::Family => 365,
    }
}

/// Check if CloudKit sync is available.
/// Free: false, Plus+: true
/// Traces to: FR-ENTITLEMENTS-011
pub fn can_use_cloudkit_sync(entitlement: &Entitlement) -> bool {
    matches!(
        entitlement.tier,
        Tier::Plus | Tier::Pro | Tier::Family
    )
}

/// Get daily nudge limit.
/// Free: 0, Plus: 3, Pro/Family: unlimited (represented as u32::MAX)
/// Traces to: FR-ENTITLEMENTS-012
pub fn nudge_limit_per_day(entitlement: &Entitlement) -> u32 {
    match entitlement.tier {
        Tier::Free => 0,
        Tier::Plus => 3,
        Tier::Pro | Tier::Family => u32::MAX,
    }
}

/// Check if proactive 24-hour-ahead nudges are available.
/// Free/Plus: false, Pro/Family: true
/// Traces to: FR-ENTITLEMENTS-013
pub fn has_proactive_nudges(entitlement: &Entitlement) -> bool {
    matches!(entitlement.tier, Tier::Pro | Tier::Family)
}

/// Check if custom Coachy cosmetics (poses, outfits, accessories) are available.
/// Free/Plus: false, Pro/Family: true
/// Traces to: FR-ENTITLEMENTS-014
pub fn can_customize_coachy(entitlement: &Entitlement) -> bool {
    matches!(entitlement.tier, Tier::Pro | Tier::Family)
}

/// Check if template marketplace access is available.
/// Free/Plus: false, Pro/Family: true
/// Traces to: FR-ENTITLEMENTS-015
pub fn has_template_marketplace(entitlement: &Entitlement) -> bool {
    matches!(entitlement.tier, Tier::Pro | Tier::Family)
}

/// Check if advanced analytics dashboard is available.
/// Free: false, Plus: basic only, Pro/Family: advanced
/// Traces to: FR-ENTITLEMENTS-016
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalyticsTier {
    None,
    Basic,
    Advanced,
}

pub fn analytics_tier(entitlement: &Entitlement) -> AnalyticsTier {
    match entitlement.tier {
        Tier::Free => AnalyticsTier::None,
        Tier::Plus => AnalyticsTier::Basic,
        Tier::Pro | Tier::Family => AnalyticsTier::Advanced,
    }
}

/// Check if family dashboard is available.
/// Only Family tier
/// Traces to: FR-ENTITLEMENTS-017
pub fn has_family_dashboard(entitlement: &Entitlement) -> bool {
    entitlement.tier == Tier::Family
}

/// Get support priority level.
/// Free: community, Plus: 48h, Pro: 24h, Family: 24h
/// Traces to: FR-ENTITLEMENTS-018
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportPriority {
    Community,
    Standard,  // 48h
    Priority,  // 24h
}

pub fn support_priority(entitlement: &Entitlement) -> SupportPriority {
    match entitlement.tier {
        Tier::Free => SupportPriority::Community,
        Tier::Plus => SupportPriority::Standard,
        Tier::Pro | Tier::Family => SupportPriority::Priority,
    }
}

// ============================================================================
// Entitlement Store Trait (for platform abstraction)
// ============================================================================

/// Trait for storing and retrieving entitlements.
/// Implementations can use SQLite (on-device), CloudKit, or test mocks.
#[async_trait::async_trait]
pub trait EntitlementStore: Send + Sync {
    /// Get current entitlement.
    async fn get_current(&self) -> GateResult<Entitlement>;

    /// Update entitlement (e.g., after receipt verification).
    async fn set(&self, entitlement: Entitlement) -> GateResult<()>;

    /// Verify receipt and update entitlement in one atomic operation.
    /// Returns the new entitlement if verification succeeds.
    async fn verify_receipt(&self, receipt_signature: &str) -> GateResult<Entitlement>;
}

/// In-memory entitlement store for tests and default initialization.
pub struct InMemoryEntitlementStore {
    state: tokio::sync::Mutex<Entitlement>,
}

impl InMemoryEntitlementStore {
    pub fn new(entitlement: Entitlement) -> Self {
        Self {
            state: tokio::sync::Mutex::new(entitlement),
        }
    }

    pub fn free() -> Self {
        Self::new(Entitlement::free())
    }
}

#[async_trait::async_trait]
impl EntitlementStore for InMemoryEntitlementStore {
    async fn get_current(&self) -> GateResult<Entitlement> {
        Ok(self.state.lock().await.clone())
    }

    async fn set(&self, entitlement: Entitlement) -> GateResult<()> {
        *self.state.lock().await = entitlement;
        Ok(())
    }

    async fn verify_receipt(&self, _receipt_signature: &str) -> GateResult<Entitlement> {
        // In tests, assume receipt verification succeeds and grants Plus tier
        let entitlement = Entitlement::with_tier(
            Tier::Plus,
            Utc::now() + chrono::Duration::days(30),
            _receipt_signature.to_string(),
        );
        self.set(entitlement.clone()).await?;
        Ok(entitlement)
    }
}

// ============================================================================
// FFI Surface (for Swift bindings via UniFFI)
// ============================================================================

/// DTO for Tier (serializable to Swift)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierDto {
    pub name: String,
}

impl From<Tier> for TierDto {
    fn from(tier: Tier) -> Self {
        Self {
            name: tier.to_string(),
        }
    }
}

/// DTO for current entitlement state (exposed to iOS)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementDto {
    pub tier: String,
    pub expires_at: Option<i64>, // Unix timestamp
    pub days_until_expiry: Option<i64>,
}

impl EntitlementDto {
    pub fn from_entitlement(ent: &Entitlement, now: DateTime<Utc>) -> Self {
        Self {
            tier: ent.tier.to_string(),
            expires_at: ent.expires_at.map(|dt| dt.timestamp()),
            days_until_expiry: ent.days_until_expiry(now),
        }
    }
}

/// Public API for iOS (via UniFFI)
pub struct EntitlementsApi;

impl EntitlementsApi {
    /// Get current tier as string (e.g., "plus", "free")
    pub async fn current_tier(store: &dyn EntitlementStore) -> String {
        store
            .get_current()
            .await
            .map(|e| e.tier.to_string())
            .unwrap_or_else(|_| "free".to_string())
    }

    /// Set tier from receipt (called after StoreKit 2 purchase on iOS)
    pub async fn set_tier_from_receipt(
        store: &dyn EntitlementStore,
        receipt_signature: String,
    ) -> Result<String, String> {
        store
            .verify_receipt(&receipt_signature)
            .await
            .map(|e| e.tier.to_string())
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-ENTITLEMENTS-001
    #[test]
    fn test_can_add_rule_free_tier_limit() {
        let ent = Entitlement::free();
        assert!(can_add_rule(0, &ent).is_ok());
        assert!(can_add_rule(1, &ent).is_ok());
        assert!(can_add_rule(2, &ent).is_ok());
        assert!(can_add_rule(3, &ent).is_err());
        assert!(can_add_rule(10, &ent).is_err());
    }

    // Traces to: FR-ENTITLEMENTS-001
    #[test]
    fn test_can_add_rule_paid_tiers() {
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        let pro = Entitlement::with_tier(Tier::Pro, exp, "sig".to_string());
        let family = Entitlement::with_tier(Tier::Family, exp, "sig".to_string());

        assert!(can_add_rule(100, &plus).is_ok());
        assert!(can_add_rule(100, &pro).is_ok());
        assert!(can_add_rule(100, &family).is_ok());
    }

    // Traces to: FR-ENTITLEMENTS-002
    #[test]
    fn test_can_add_task_free_tier_limit() {
        let ent = Entitlement::free();
        assert!(can_add_task(0, &ent).is_ok());
        assert!(can_add_task(2, &ent).is_ok());
        assert!(can_add_task(3, &ent).is_err());
    }

    // Traces to: FR-ENTITLEMENTS-002
    #[test]
    fn test_can_add_task_paid_tiers() {
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        assert!(can_add_task(1000, &plus).is_ok());
    }

    // Traces to: FR-ENTITLEMENTS-003
    #[test]
    fn test_connector_refresh_cadence() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());

        assert_eq!(connector_refresh_cadence_minutes(&free), 240); // 4 hours
        assert_eq!(connector_refresh_cadence_minutes(&plus), 15); // 15 minutes
    }

    // Traces to: FR-ENTITLEMENTS-004
    #[test]
    fn test_max_active_connectors() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let pro = Entitlement::with_tier(Tier::Pro, exp, "sig".to_string());

        assert_eq!(max_active_connectors(&free), 1);
        assert_eq!(max_active_connectors(&pro), 4);
    }

    // Traces to: FR-ENTITLEMENTS-005
    #[test]
    fn test_validate_focus_duration_free_tier() {
        let ent = Entitlement::free();
        assert!(validate_focus_duration(25, &ent).is_ok());
        assert!(validate_focus_duration(30, &ent).is_err());
        assert!(validate_focus_duration(20, &ent).is_err());
    }

    // Traces to: FR-ENTITLEMENTS-005
    #[test]
    fn test_validate_focus_duration_plus_tier() {
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        assert!(validate_focus_duration(5, &plus).is_ok());
        assert!(validate_focus_duration(25, &plus).is_ok());
        assert!(validate_focus_duration(180, &plus).is_ok());
        assert!(validate_focus_duration(181, &plus).is_err());
        assert!(validate_focus_duration(4, &plus).is_err());
    }

    // Traces to: FR-ENTITLEMENTS-006
    #[test]
    fn test_validate_break_duration_free_tier() {
        let ent = Entitlement::free();
        assert!(validate_break_duration(45, &ent).is_ok());
        assert!(validate_break_duration(30, &ent).is_err());
        assert!(validate_break_duration(60, &ent).is_err());
    }

    // Traces to: FR-ENTITLEMENTS-006
    #[test]
    fn test_validate_break_duration_plus_tier() {
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        assert!(validate_break_duration(1, &plus).is_ok());
        assert!(validate_break_duration(30, &plus).is_ok());
        assert!(validate_break_duration(60, &plus).is_ok());
        assert!(validate_break_duration(61, &plus).is_err());
        assert!(validate_break_duration(0, &plus).is_err());
    }

    // Traces to: FR-ENTITLEMENTS-007
    #[test]
    fn test_voice_provider() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        let pro = Entitlement::with_tier(Tier::Pro, exp, "sig".to_string());

        assert_eq!(voice_provider(&free), VoiceProvider::Silent);
        assert_eq!(voice_provider(&plus), VoiceProvider::Native);
        assert_eq!(voice_provider(&pro), VoiceProvider::ElevenLabs);
    }

    // Traces to: FR-ENTITLEMENTS-008
    #[test]
    fn test_can_use_live_activity() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());

        assert!(!can_use_live_activity(&free));
        assert!(can_use_live_activity(&plus));
    }

    // Traces to: FR-ENTITLEMENTS-009
    #[test]
    fn test_can_use_homekit_widget() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let family = Entitlement::with_tier(Tier::Family, exp, "sig".to_string());

        assert!(!can_use_homekit_widget(&free));
        assert!(can_use_homekit_widget(&family));
    }

    // Traces to: FR-ENTITLEMENTS-010
    #[test]
    fn test_audit_retention_days() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        let pro = Entitlement::with_tier(Tier::Pro, exp, "sig".to_string());
        let family = Entitlement::with_tier(Tier::Family, exp, "sig".to_string());

        assert_eq!(audit_retention_days(&free), 7);
        assert_eq!(audit_retention_days(&plus), 90);
        assert_eq!(audit_retention_days(&pro), 180);
        assert_eq!(audit_retention_days(&family), 365);
    }

    // Traces to: FR-ENTITLEMENTS-011
    #[test]
    fn test_can_use_cloudkit_sync() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let pro = Entitlement::with_tier(Tier::Pro, exp, "sig".to_string());

        assert!(!can_use_cloudkit_sync(&free));
        assert!(can_use_cloudkit_sync(&pro));
    }

    // Traces to: FR-ENTITLEMENTS-012
    #[test]
    fn test_nudge_limit_per_day() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        let pro = Entitlement::with_tier(Tier::Pro, exp, "sig".to_string());

        assert_eq!(nudge_limit_per_day(&free), 0);
        assert_eq!(nudge_limit_per_day(&plus), 3);
        assert_eq!(nudge_limit_per_day(&pro), u32::MAX);
    }

    // Traces to: FR-ENTITLEMENTS-013
    #[test]
    fn test_has_proactive_nudges() {
        let plus = Entitlement::with_tier(
            Tier::Plus,
            Utc::now() + chrono::Duration::days(30),
            "sig".to_string(),
        );
        let pro = Entitlement::with_tier(
            Tier::Pro,
            Utc::now() + chrono::Duration::days(30),
            "sig".to_string(),
        );
        let free = Entitlement::free();

        assert!(!has_proactive_nudges(&free));
        assert!(!has_proactive_nudges(&plus));
        assert!(has_proactive_nudges(&pro));
    }

    // Traces to: FR-ENTITLEMENTS-014
    #[test]
    fn test_can_customize_coachy() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        let family = Entitlement::with_tier(Tier::Family, exp, "sig".to_string());

        assert!(!can_customize_coachy(&free));
        assert!(!can_customize_coachy(&plus));
        assert!(can_customize_coachy(&family));
    }

    // Traces to: FR-ENTITLEMENTS-015
    #[test]
    fn test_has_template_marketplace() {
        let plus = Entitlement::with_tier(
            Tier::Plus,
            Utc::now() + chrono::Duration::days(30),
            "sig".to_string(),
        );
        let pro = Entitlement::with_tier(
            Tier::Pro,
            Utc::now() + chrono::Duration::days(30),
            "sig".to_string(),
        );

        assert!(!has_template_marketplace(&plus));
        assert!(has_template_marketplace(&pro));
    }

    // Traces to: FR-ENTITLEMENTS-016
    #[test]
    fn test_analytics_tier() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        let pro = Entitlement::with_tier(Tier::Pro, exp, "sig".to_string());

        assert_eq!(analytics_tier(&free), AnalyticsTier::None);
        assert_eq!(analytics_tier(&plus), AnalyticsTier::Basic);
        assert_eq!(analytics_tier(&pro), AnalyticsTier::Advanced);
    }

    // Traces to: FR-ENTITLEMENTS-017
    #[test]
    fn test_has_family_dashboard() {
        let plus = Entitlement::with_tier(
            Tier::Plus,
            Utc::now() + chrono::Duration::days(30),
            "sig".to_string(),
        );
        let family = Entitlement::with_tier(
            Tier::Family,
            Utc::now() + chrono::Duration::days(30),
            "sig".to_string(),
        );

        assert!(!has_family_dashboard(&plus));
        assert!(has_family_dashboard(&family));
    }

    // Traces to: FR-ENTITLEMENTS-018
    #[test]
    fn test_support_priority() {
        let free = Entitlement::free();
        let exp = Utc::now() + chrono::Duration::days(30);
        let plus = Entitlement::with_tier(Tier::Plus, exp, "sig".to_string());
        let pro = Entitlement::with_tier(Tier::Pro, exp, "sig".to_string());

        assert_eq!(support_priority(&free), SupportPriority::Community);
        assert_eq!(support_priority(&plus), SupportPriority::Standard);
        assert_eq!(support_priority(&pro), SupportPriority::Priority);
    }

    // Traces to: FR-ENTITLEMENTS-001 through FR-ENTITLEMENTS-018 (comprehensive combo test)
    #[test]
    fn test_entitlement_lifecycle() {
        let now = Utc::now();
        let mut ent = Entitlement::free();

        // Start with Free tier
        assert_eq!(ent.tier, Tier::Free);
        assert!(ent.is_active(now));
        assert_eq!(ent.days_until_expiry(now), None);

        // Upgrade to Plus
        ent = Entitlement::with_tier(
            Tier::Plus,
            now + chrono::Duration::days(30),
            "sig1".to_string(),
        );
        assert_eq!(ent.tier, Tier::Plus);
        assert!(ent.is_active(now));
        assert_eq!(ent.days_until_expiry(now), Some(30));

        // Check Plus features
        assert!(can_add_rule(10, &ent).is_ok());
        assert_eq!(voice_provider(&ent), VoiceProvider::Native);
        assert!(can_use_live_activity(&ent));
        assert_eq!(audit_retention_days(&ent), 90);

        // Simulate expiry
        let expired_time = now + chrono::Duration::days(31);
        assert!(!ent.is_active(expired_time));
        assert_eq!(ent.days_until_expiry(expired_time), Some(-1));
    }

    // Traces to: FR-ENTITLEMENTS-001 (in-memory store test)
    #[tokio::test]
    async fn test_in_memory_store() {
        let store = InMemoryEntitlementStore::free();
        let ent = store.get_current().await.unwrap();
        assert_eq!(ent.tier, Tier::Free);

        // Simulate receipt verification
        let updated = store.verify_receipt("sig1").await.unwrap();
        assert_eq!(updated.tier, Tier::Plus);

        // Verify state persists
        let ent2 = store.get_current().await.unwrap();
        assert_eq!(ent2.tier, Tier::Plus);
    }

    // Traces to: FR-ENTITLEMENTS-001 (FFI API test)
    #[tokio::test]
    async fn test_entitlements_api_current_tier() {
        let store = InMemoryEntitlementStore::free();
        let tier = EntitlementsApi::current_tier(&store).await;
        assert_eq!(tier, "free");
    }

    // Traces to: FR-ENTITLEMENTS-001 (FFI API receipt test)
    #[tokio::test]
    async fn test_entitlements_api_set_tier_from_receipt() {
        let store = InMemoryEntitlementStore::free();
        let result = EntitlementsApi::set_tier_from_receipt(&store, "sig1".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "plus");
    }
}
