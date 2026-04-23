//! Reward wallet aggregate + mutations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub last_incremented_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Credit {
    pub amount: i64,
    pub source_rule_id: Option<uuid::Uuid>,
    pub granted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiplierState {
    pub current: f32,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletMutation {
    GrantCredit(Credit),
    SpendCredit { amount: i64, purpose: String },
    StreakIncrement(String),
    StreakReset(String),
    SetMultiplier(MultiplierState),
}
