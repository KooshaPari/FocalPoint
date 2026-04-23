//! Reward wallet aggregate + mutations.
//!
//! Traces to FR-STATE-001.

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("invariant violation: {0}")]
    Invariant(String),
    #[error("insufficient credit: balance {balance}, requested {requested}")]
    InsufficientCredit { balance: i64, requested: i64 },
    #[error("negative amount not allowed: {0}")]
    NegativeAmount(i64),
}

pub type Result<T> = std::result::Result<T, WalletError>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RewardWallet {
    pub user_id: uuid::Uuid,
    pub earned_credits: i64,
    pub spent_credits: i64,
    pub streaks: HashMap<String, Streak>,
    pub unlock_balances: HashMap<String, i64>,
    pub multiplier_state: MultiplierState,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Streak {
    pub name: String,
    pub count: u32,
    pub last_incremented_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Credit {
    pub amount: i64,
    pub source_rule_id: Option<uuid::Uuid>,
    pub granted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiplierState {
    pub current: f32,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletMutation {
    GrantCredit(Credit),
    SpendCredit { amount: i64, purpose: String },
    StreakIncrement(String),
    StreakReset(String),
    SetMultiplier(MultiplierState),
}

impl RewardWallet {
    /// Current credit balance (non-negative invariant: earned >= spent).
    pub fn balance(&self) -> i64 {
        self.earned_credits - self.spent_credits
    }

    /// Multiplier in effect at `now`, or 1.0 if none / expired.
    /// Traces to: FR-STATE-001.
    pub fn effective_multiplier(&self, now: DateTime<Utc>) -> f32 {
        match self.multiplier_state.expires_at {
            Some(exp) if exp <= now => 1.0,
            _ => {
                if self.multiplier_state.current > 0.0 {
                    self.multiplier_state.current
                } else {
                    1.0
                }
            }
        }
    }

    /// Apply a mutation at `now`, enforcing invariants.
    /// Traces to: FR-STATE-001.
    pub fn apply(&mut self, mutation: WalletMutation, now: DateTime<Utc>) -> Result<()> {
        // Prune expired multiplier deterministically on every apply.
        if let Some(exp) = self.multiplier_state.expires_at {
            if exp <= now {
                self.multiplier_state = MultiplierState::default();
            }
        }

        match mutation {
            WalletMutation::GrantCredit(c) => {
                if c.amount < 0 {
                    return Err(WalletError::NegativeAmount(c.amount));
                }
                self.earned_credits = self
                    .earned_credits
                    .checked_add(c.amount)
                    .ok_or_else(|| WalletError::Invariant("earned overflow".into()))?;
            }
            WalletMutation::SpendCredit { amount, .. } => {
                if amount < 0 {
                    return Err(WalletError::NegativeAmount(amount));
                }
                if self.balance() < amount {
                    return Err(WalletError::InsufficientCredit {
                        balance: self.balance(),
                        requested: amount,
                    });
                }
                self.spent_credits = self
                    .spent_credits
                    .checked_add(amount)
                    .ok_or_else(|| WalletError::Invariant("spent overflow".into()))?;
                if self.spent_credits > self.earned_credits {
                    return Err(WalletError::Invariant("spent > earned".into()));
                }
            }
            WalletMutation::StreakIncrement(name) => {
                let entry = self.streaks.entry(name.clone()).or_insert_with(|| Streak {
                    name: name.clone(),
                    count: 0,
                    last_incremented_at: None,
                });
                // One increment per UTC day per streak.
                if let Some(last) = entry.last_incremented_at {
                    if same_utc_day(last, now) {
                        return Ok(()); // idempotent no-op
                    }
                }
                entry.count = entry.count.saturating_add(1);
                entry.last_incremented_at = Some(now);
            }
            WalletMutation::StreakReset(name) => {
                if let Some(s) = self.streaks.get_mut(&name) {
                    s.count = 0;
                    s.last_incremented_at = None;
                }
            }
            WalletMutation::SetMultiplier(m) => {
                if m.current.is_nan() || m.current < 0.0 {
                    return Err(WalletError::Invariant("invalid multiplier".into()));
                }
                if let Some(exp) = m.expires_at {
                    if exp <= now {
                        return Ok(());
                    }
                }
                self.multiplier_state = m;
            }
        }
        Ok(())
    }
}

fn same_utc_day(a: DateTime<Utc>, b: DateTime<Utc>) -> bool {
    a.year() == b.year() && a.ordinal() == b.ordinal()
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

    // Traces to: FR-STATE-001
    #[test]
    fn grant_and_spend_balance() {
        let mut w = RewardWallet::default();
        w.apply(
            WalletMutation::GrantCredit(Credit {
                amount: 100,
                source_rule_id: None,
                granted_at: t(2026, 1, 1, 0),
            }),
            t(2026, 1, 1, 0),
        )
        .unwrap();
        w.apply(
            WalletMutation::SpendCredit { amount: 40, purpose: "unlock".into() },
            t(2026, 1, 1, 1),
        )
        .unwrap();
        assert_eq!(w.balance(), 60);
    }

    // Traces to: FR-STATE-001
    #[test]
    fn spend_more_than_balance_errors() {
        let mut w = RewardWallet::default();
        let err = w
            .apply(WalletMutation::SpendCredit { amount: 5, purpose: "x".into() }, t(2026, 1, 1, 0))
            .unwrap_err();
        assert!(matches!(err, WalletError::InsufficientCredit { .. }));
    }

    // Traces to: FR-STATE-001
    #[test]
    fn streak_increments_only_once_per_utc_day() {
        let mut w = RewardWallet::default();
        w.apply(WalletMutation::StreakIncrement("daily".into()), t(2026, 1, 1, 8)).unwrap();
        w.apply(WalletMutation::StreakIncrement("daily".into()), t(2026, 1, 1, 23)).unwrap();
        assert_eq!(w.streaks["daily"].count, 1);
        w.apply(WalletMutation::StreakIncrement("daily".into()), t(2026, 1, 2, 0)).unwrap();
        assert_eq!(w.streaks["daily"].count, 2);
    }

    // Traces to: FR-STATE-001
    #[test]
    fn multiplier_expires_and_effective_is_one() {
        let mut w = RewardWallet::default();
        w.apply(
            WalletMutation::SetMultiplier(MultiplierState {
                current: 2.0,
                expires_at: Some(t(2026, 1, 1, 10)),
            }),
            t(2026, 1, 1, 9),
        )
        .unwrap();
        assert_eq!(w.effective_multiplier(t(2026, 1, 1, 9)), 2.0);
        w.apply(WalletMutation::StreakReset("noop".into()), t(2026, 1, 1, 11)).unwrap();
        assert_eq!(w.effective_multiplier(t(2026, 1, 1, 11)), 1.0);
        assert!(w.multiplier_state.expires_at.is_none());
    }

    // Traces to: FR-STATE-001
    #[test]
    fn negative_grant_rejected() {
        let mut w = RewardWallet::default();
        let err = w
            .apply(
                WalletMutation::GrantCredit(Credit {
                    amount: -1,
                    source_rule_id: None,
                    granted_at: t(2026, 1, 1, 0),
                }),
                t(2026, 1, 1, 0),
            )
            .unwrap_err();
        assert!(matches!(err, WalletError::NegativeAmount(_)));
    }

    // Traces to: FR-STATE-001
    #[test]
    fn streak_reset_clears_count() {
        let mut w = RewardWallet::default();
        w.apply(WalletMutation::StreakIncrement("s".into()), t(2026, 1, 1, 0)).unwrap();
        w.apply(WalletMutation::StreakReset("s".into()), t(2026, 1, 1, 1)).unwrap();
        assert_eq!(w.streaks["s"].count, 0);
    }
}
