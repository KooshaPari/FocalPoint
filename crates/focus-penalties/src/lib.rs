//! Penalty state, escalation tiers, bypass budget.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PenaltyState {
    pub user_id: uuid::Uuid,
    pub escalation_tier: EscalationTier,
    pub bypass_budget: i64,
    pub lockout_windows: Vec<LockoutWindow>,
    pub debt_balance: i64,
    pub strict_mode_until: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscalationTier {
    #[default]
    Clear,
    Warning,
    Restricted,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockoutWindow {
    pub starts_at: chrono::DateTime<chrono::Utc>,
    pub ends_at: chrono::DateTime<chrono::Utc>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PenaltyMutation {
    Escalate(EscalationTier),
    SpendBypass(i64),
    AddLockout(LockoutWindow),
    ClearLockouts,
    SetStrictMode { until: chrono::DateTime<chrono::Utc> },
}
