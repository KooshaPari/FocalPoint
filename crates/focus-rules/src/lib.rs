//! Rule DSL, evaluation, priority, cooldowns, explanation.

use chrono::{DateTime, Duration, Utc};
use focus_events::NormalizedEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: Uuid,
    pub name: String,
    pub trigger: Trigger,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub priority: i32,
    pub cooldown: Option<Duration>,
    pub duration: Option<Duration>,
    pub explanation_template: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trigger {
    Event(String),     // EventType name
    Schedule(String),  // cron-like
    StateChange(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub kind: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    GrantCredit { amount: i32 },
    DeductCredit { amount: i32 },
    Block { profile: String, duration: Duration },
    Unblock { profile: String },
    StreakIncrement(String),
    StreakReset(String),
    Notify(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEvaluation {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub event_ids: Vec<Uuid>,
    pub evaluated_at: DateTime<Utc>,
    pub decision: RuleDecision,
    pub state_snapshot_ref: Option<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleDecision {
    Fired(Vec<Action>),
    Suppressed { reason: String },
    Skipped { reason: String },
}

pub struct RuleEngine {
    // Stub: actual state + indexes TBD
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, _rule: &Rule, _event: &NormalizedEvent) -> RuleDecision {
        RuleDecision::Skipped { reason: "stub".into() }
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}
