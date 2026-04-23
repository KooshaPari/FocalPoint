//! Canonical domain entities, IDs, aggregate roots, invariants.
//!
//! No persistence, no I/O. Pure types.

#![cfg_attr(test, allow(clippy::disallowed_methods))]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub type Result<T, E = DomainError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invariant violation: {0}")]
    Invariant(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub created_at: DateTime<Utc>,
    pub display_name: String,
    pub primary_device_id: Option<DeviceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: DeviceId,
    pub user_id: UserId,
    pub platform: Platform,
    pub os_version: String,
    pub enrolled_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Ios,
    Android,
    Macos,
    Unknown,
}

// -----------------------------------------------------------------------------
// Rigidity spectrum (Task #29)
// -----------------------------------------------------------------------------

/// The cost an actor must pay to bypass a `Semi`-rigid constraint.
///
/// Traces to: FR-RIGIDITY-001.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RigidityCost {
    /// Spend this many credits from the wallet.
    CreditCost(i64),
    /// Advance the escalation tier by one step.
    TierBump,
    /// Resets an active streak on bypass.
    StreakRisk,
    /// Wait this duration before the bypass becomes effective.
    FrictionDelay(std::time::Duration),
    /// Ping a configured accountability partner.
    AccountabilityPing,
}

/// How rigid an enforcement or constraint is.
///
/// * `Hard` — cannot be bypassed by the user at all.
/// * `Semi(cost)` — bypassable, but only by paying `cost`.
/// * `Soft` — purely advisory; user may override freely.
///
/// Traces to: FR-RIGIDITY-001.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rigidity {
    #[default]
    Hard,
    Semi(RigidityCost),
    Soft,
}

impl Rigidity {
    pub fn is_hard(&self) -> bool {
        matches!(self, Rigidity::Hard)
    }

    pub fn is_soft(&self) -> bool {
        matches!(self, Rigidity::Soft)
    }

    pub fn semi_cost(&self) -> Option<&RigidityCost> {
        match self {
            Rigidity::Semi(c) => Some(c),
            _ => None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod rigidity_tests {
    use super::*;
    use std::time::Duration;

    // Traces to: FR-RIGIDITY-001
    #[test]
    fn hard_is_hard() {
        let r = Rigidity::Hard;
        assert!(r.is_hard());
        assert!(!r.is_soft());
        assert!(r.semi_cost().is_none());
    }

    // Traces to: FR-RIGIDITY-001
    #[test]
    fn soft_is_soft() {
        let r = Rigidity::Soft;
        assert!(r.is_soft());
        assert!(!r.is_hard());
        assert!(r.semi_cost().is_none());
    }

    // Traces to: FR-RIGIDITY-001
    #[test]
    fn semi_credit_cost_extraction() {
        let r = Rigidity::Semi(RigidityCost::CreditCost(25));
        assert!(!r.is_hard());
        assert!(!r.is_soft());
        match r.semi_cost() {
            Some(RigidityCost::CreditCost(n)) => assert_eq!(*n, 25),
            _ => panic!("expected CreditCost"),
        }
    }

    // Traces to: FR-RIGIDITY-001
    #[test]
    fn semi_tier_bump_and_streak_risk() {
        let tb = Rigidity::Semi(RigidityCost::TierBump);
        let sr = Rigidity::Semi(RigidityCost::StreakRisk);
        assert!(matches!(tb.semi_cost(), Some(RigidityCost::TierBump)));
        assert!(matches!(sr.semi_cost(), Some(RigidityCost::StreakRisk)));
    }

    // Traces to: FR-RIGIDITY-001
    #[test]
    fn semi_friction_delay_and_ping() {
        let fd = Rigidity::Semi(RigidityCost::FrictionDelay(Duration::from_secs(30)));
        match fd.semi_cost() {
            Some(RigidityCost::FrictionDelay(d)) => assert_eq!(*d, Duration::from_secs(30)),
            _ => panic!("expected FrictionDelay"),
        }
        let ap = Rigidity::Semi(RigidityCost::AccountabilityPing);
        assert!(matches!(ap.semi_cost(), Some(RigidityCost::AccountabilityPing)));
    }

    // Traces to: FR-RIGIDITY-001
    #[test]
    fn default_is_hard() {
        assert!(Rigidity::default().is_hard());
    }

    // Traces to: FR-RIGIDITY-001
    #[test]
    fn rigidity_roundtrips_serde() {
        let r = Rigidity::Semi(RigidityCost::CreditCost(7));
        let json = serde_json::to_string(&r).expect("serialize Rigidity");
        let back: Rigidity = serde_json::from_str(&json).expect("deserialize Rigidity");
        assert_eq!(r, back);
    }
}
