//! Penalty state, escalation tiers, bypass budget.
//!
//! Traces to FR-STATE-002.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PenaltyError {
    #[error("invariant violation: {0}")]
    Invariant(String),
    #[error("insufficient bypass budget: {balance} < {requested}")]
    InsufficientBypass { balance: i64, requested: i64 },
    #[error("negative amount: {0}")]
    NegativeAmount(i64),
}

pub type Result<T> = std::result::Result<T, PenaltyError>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PenaltyState {
    pub user_id: uuid::Uuid,
    pub escalation_tier: EscalationTier,
    pub bypass_budget: i64,
    pub lockout_windows: Vec<LockoutWindow>,
    pub debt_balance: i64,
    pub strict_mode_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EscalationTier {
    #[default]
    Clear,
    Warning,
    Restricted,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockoutWindow {
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PenaltyMutation {
    Escalate(EscalationTier),
    SpendBypass(i64),
    GrantBypass(i64),
    AddLockout(LockoutWindow),
    ClearLockouts,
    SetStrictMode { until: DateTime<Utc> },
    Clear,
}

impl PenaltyState {
    /// True iff strict-mode window covers `now`.
    /// Traces to: FR-STATE-002.
    pub fn is_strict(&self, now: DateTime<Utc>) -> bool {
        match self.strict_mode_until {
            Some(exp) => exp > now,
            None => false,
        }
    }

    /// Apply a mutation at `now`, enforcing invariants.
    /// Traces to: FR-STATE-002.
    pub fn apply(&mut self, mutation: PenaltyMutation, now: DateTime<Utc>) -> Result<()> {
        // Auto-clear expired strict mode.
        if let Some(exp) = self.strict_mode_until {
            if exp <= now {
                self.strict_mode_until = None;
            }
        }
        // Drop fully-expired lockouts.
        self.lockout_windows.retain(|w| w.ends_at > now);

        match mutation {
            PenaltyMutation::Escalate(tier) => {
                if tier < self.escalation_tier {
                    return Err(PenaltyError::Invariant(
                        "escalation can only move up; use Clear to reset".into(),
                    ));
                }
                self.escalation_tier = tier;
            }
            PenaltyMutation::SpendBypass(n) => {
                if n < 0 {
                    return Err(PenaltyError::NegativeAmount(n));
                }
                if self.bypass_budget < n {
                    return Err(PenaltyError::InsufficientBypass {
                        balance: self.bypass_budget,
                        requested: n,
                    });
                }
                self.bypass_budget -= n;
                if self.bypass_budget < 0 {
                    return Err(PenaltyError::Invariant("bypass_budget < 0".into()));
                }
            }
            PenaltyMutation::GrantBypass(n) => {
                if n < 0 {
                    return Err(PenaltyError::NegativeAmount(n));
                }
                self.bypass_budget = self
                    .bypass_budget
                    .checked_add(n)
                    .ok_or_else(|| PenaltyError::Invariant("bypass overflow".into()))?;
            }
            PenaltyMutation::AddLockout(w) => {
                if w.ends_at <= w.starts_at {
                    return Err(PenaltyError::Invariant("lockout ends <= starts".into()));
                }
                self.lockout_windows.push(w);
            }
            PenaltyMutation::ClearLockouts => {
                self.lockout_windows.clear();
            }
            PenaltyMutation::SetStrictMode { until } => {
                if until <= now {
                    return Err(PenaltyError::Invariant("strict_mode_until in past".into()));
                }
                self.strict_mode_until = Some(until);
            }
            PenaltyMutation::Clear => {
                self.escalation_tier = EscalationTier::Clear;
                self.strict_mode_until = None;
                self.lockout_windows.clear();
            }
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn t(y: i32, m: u32, d: u32, h: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, 0, 0).unwrap()
    }

    // Traces to: FR-STATE-002
    #[test]
    fn escalation_only_moves_up() {
        let mut s = PenaltyState::default();
        s.apply(PenaltyMutation::Escalate(EscalationTier::Warning), t(2026, 1, 1, 0)).unwrap();
        s.apply(PenaltyMutation::Escalate(EscalationTier::Restricted), t(2026, 1, 1, 1)).unwrap();
        assert_eq!(s.escalation_tier, EscalationTier::Restricted);
        let err = s
            .apply(PenaltyMutation::Escalate(EscalationTier::Warning), t(2026, 1, 1, 2))
            .unwrap_err();
        assert!(matches!(err, PenaltyError::Invariant(_)));
    }

    // Traces to: FR-STATE-002
    #[test]
    fn clear_resets_tier() {
        let mut s = PenaltyState::default();
        s.apply(PenaltyMutation::Escalate(EscalationTier::Strict), t(2026, 1, 1, 0)).unwrap();
        s.apply(PenaltyMutation::Clear, t(2026, 1, 1, 1)).unwrap();
        assert_eq!(s.escalation_tier, EscalationTier::Clear);
    }

    // Traces to: FR-STATE-002
    #[test]
    fn bypass_budget_nonnegative() {
        let mut s = PenaltyState::default();
        s.apply(PenaltyMutation::GrantBypass(10), t(2026, 1, 1, 0)).unwrap();
        s.apply(PenaltyMutation::SpendBypass(7), t(2026, 1, 1, 1)).unwrap();
        assert_eq!(s.bypass_budget, 3);
        let err = s.apply(PenaltyMutation::SpendBypass(10), t(2026, 1, 1, 2)).unwrap_err();
        assert!(matches!(err, PenaltyError::InsufficientBypass { .. }));
    }

    // Traces to: FR-STATE-002
    #[test]
    fn strict_mode_auto_clears_after_expiry() {
        let mut s = PenaltyState::default();
        s.apply(PenaltyMutation::SetStrictMode { until: t(2026, 1, 1, 10) }, t(2026, 1, 1, 9))
            .unwrap();
        assert!(s.is_strict(t(2026, 1, 1, 9)));
        s.apply(PenaltyMutation::ClearLockouts, t(2026, 1, 1, 11)).unwrap();
        assert!(!s.is_strict(t(2026, 1, 1, 11)));
        assert!(s.strict_mode_until.is_none());
    }

    // Traces to: FR-STATE-002
    #[test]
    fn expired_lockouts_pruned_on_apply() {
        let mut s = PenaltyState::default();
        s.lockout_windows.push(LockoutWindow {
            starts_at: t(2026, 1, 1, 0),
            ends_at: t(2026, 1, 1, 1),
            reason: "x".into(),
        });
        s.apply(PenaltyMutation::GrantBypass(0), t(2026, 1, 1, 5)).unwrap();
        assert!(s.lockout_windows.is_empty());
    }

    // Traces to: FR-STATE-002
    #[test]
    fn add_lockout_rejects_bad_window() {
        let mut s = PenaltyState::default();
        let err = s
            .apply(
                PenaltyMutation::AddLockout(LockoutWindow {
                    starts_at: t(2026, 1, 1, 5),
                    ends_at: t(2026, 1, 1, 5),
                    reason: "x".into(),
                }),
                t(2026, 1, 1, 0),
            )
            .unwrap_err();
        assert!(matches!(err, PenaltyError::Invariant(_)));
    }
}
