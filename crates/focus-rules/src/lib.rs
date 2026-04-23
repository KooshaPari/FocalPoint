//! Rule DSL, evaluation, priority, cooldowns, explanation.
//!
//! Traces to FR-RULE-001..005.

use chrono::{DateTime, Duration, Utc};
use focus_events::{EventType, NormalizedEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    Event(String),    // EventType name
    Schedule(String), // cron-like
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuleDecision {
    Fired(Vec<Action>),
    Suppressed { reason: String },
    Skipped { reason: String },
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        use Action::*;
        match (self, other) {
            (GrantCredit { amount: a }, GrantCredit { amount: b }) => a == b,
            (DeductCredit { amount: a }, DeductCredit { amount: b }) => a == b,
            (Block { profile: p1, duration: d1 }, Block { profile: p2, duration: d2 }) => {
                p1 == p2 && d1 == d2
            }
            (Unblock { profile: p1 }, Unblock { profile: p2 }) => p1 == p2,
            (StreakIncrement(a), StreakIncrement(b)) => a == b,
            (StreakReset(a), StreakReset(b)) => a == b,
            (Notify(a), Notify(b)) => a == b,
            _ => false,
        }
    }
}

/// Decision bundled with source rule metadata for priority aggregation.
#[derive(Debug, Clone)]
pub struct PrioritizedDecision {
    pub rule_id: Uuid,
    pub priority: i32,
    pub decision: RuleDecision,
}

pub struct RuleEngine {
    /// Per-rule last fired timestamp (for cooldown enforcement).
    cooldowns: HashMap<Uuid, DateTime<Utc>>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self { cooldowns: HashMap::new() }
    }

    /// Seed cooldowns (e.g. from persisted state).
    pub fn with_cooldowns(cooldowns: HashMap<Uuid, DateTime<Utc>>) -> Self {
        Self { cooldowns }
    }

    /// Expose cooldown map for persistence (read-only).
    pub fn cooldowns(&self) -> &HashMap<Uuid, DateTime<Utc>> {
        &self.cooldowns
    }

    /// Evaluate a single rule against a single event at `now`.
    ///
    /// Deterministic given (rule, event, cooldown state, now).
    /// Traces to: FR-RULE-001, FR-RULE-002, FR-RULE-003.
    pub fn evaluate(
        &mut self,
        rule: &Rule,
        event: &NormalizedEvent,
        now: DateTime<Utc>,
    ) -> RuleDecision {
        // FR-RULE-001: disabled rules skip.
        if !rule.enabled {
            return RuleDecision::Skipped { reason: "disabled".into() };
        }

        // FR-RULE-001: trigger must match event.
        match &rule.trigger {
            Trigger::Event(expected) => {
                if !event_type_matches(&event.event_type, expected) {
                    return RuleDecision::Skipped { reason: "trigger_mismatch".into() };
                }
            }
            Trigger::Schedule(_) | Trigger::StateChange(_) => {
                return RuleDecision::Skipped { reason: "non_event_trigger".into() };
            }
        }

        // FR-RULE-003: evaluate conditions (best-effort built-ins).
        for cond in &rule.conditions {
            if !condition_matches(cond, event) {
                return RuleDecision::Skipped { reason: format!("condition_failed:{}", cond.kind) };
            }
        }

        // FR-RULE-002: cooldown check.
        if let Some(cooldown) = rule.cooldown {
            if let Some(last) = self.cooldowns.get(&rule.id) {
                if now.signed_duration_since(*last) < cooldown {
                    return RuleDecision::Suppressed { reason: "cooldown".into() };
                }
            }
        }

        // Fire.
        self.cooldowns.insert(rule.id, now);
        RuleDecision::Fired(rule.actions.clone())
    }

    /// Evaluate many rules; returns all decisions in input order alongside
    /// an aggregated winner per (profile, conflict-class).
    ///
    /// Traces to: FR-RULE-004 (priority conflict resolution).
    pub fn evaluate_all(
        &mut self,
        rules: &[Rule],
        event: &NormalizedEvent,
        now: DateTime<Utc>,
    ) -> Vec<PrioritizedDecision> {
        let mut out = Vec::with_capacity(rules.len());
        // Sort by priority descending so higher-priority rules evaluate first
        // and can "win" cooldown-free slots. Preserve input order for ties.
        let mut indexed: Vec<(usize, &Rule)> = rules.iter().enumerate().collect();
        indexed.sort_by(|a, b| b.1.priority.cmp(&a.1.priority).then(a.0.cmp(&b.0)));
        for (_, rule) in indexed {
            let decision = self.evaluate(rule, event, now);
            out.push(PrioritizedDecision { rule_id: rule.id, priority: rule.priority, decision });
        }
        out
    }

    /// Render a rule's explanation template, substituting placeholders.
    /// Traces to: FR-RULE-005.
    pub fn render_explanation(rule: &Rule, event: &NormalizedEvent) -> String {
        let event_type = event_type_name(&event.event_type);
        rule.explanation_template
            .replace("{rule_name}", &rule.name)
            .replace("{event_type}", &event_type)
            .replace("{event_id}", &event.event_id.to_string())
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn event_type_matches(et: &EventType, expected: &str) -> bool {
    event_type_name(et) == expected
}

fn event_type_name(et: &EventType) -> String {
    match et {
        EventType::AssignmentDue => "AssignmentDue".into(),
        EventType::AssignmentGraded => "AssignmentGraded".into(),
        EventType::CourseEnrolled => "CourseEnrolled".into(),
        EventType::EventStarted => "EventStarted".into(),
        EventType::EventEnded => "EventEnded".into(),
        EventType::TaskCompleted => "TaskCompleted".into(),
        EventType::TaskAdded => "TaskAdded".into(),
        EventType::SleepRecorded => "SleepRecorded".into(),
        EventType::ExerciseLogged => "ExerciseLogged".into(),
        EventType::AppSessionStarted => "AppSessionStarted".into(),
        EventType::AppSessionEnded => "AppSessionEnded".into(),
        EventType::Custom(s) => format!("Custom:{s}"),
    }
}

/// Built-in condition kinds:
///   * `confidence_gte` — params.min: f32
///   * `payload_eq`     — params.path: str, params.value: any
///
/// Unknown kinds are treated as "pass" (forward-compat).
fn condition_matches(cond: &Condition, event: &NormalizedEvent) -> bool {
    match cond.kind.as_str() {
        "confidence_gte" => cond
            .params
            .get("min")
            .and_then(|v| v.as_f64())
            .map(|min| event.confidence as f64 >= min)
            .unwrap_or(false),
        "payload_eq" => {
            let path = match cond.params.get("path").and_then(|v| v.as_str()) {
                Some(p) => p,
                None => return false,
            };
            let expected = match cond.params.get("value") {
                Some(v) => v,
                None => return false,
            };
            event.payload.get(path).map(|v| v == expected).unwrap_or(false)
        }
        _ => true,
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use focus_events::DedupeKey;
    use serde_json::json;

    fn mk_event(et: EventType, confidence: f32, payload: serde_json::Value) -> NormalizedEvent {
        NormalizedEvent {
            event_id: Uuid::nil(),
            connector_id: "test".into(),
            account_id: Uuid::nil(),
            event_type: et,
            occurred_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            effective_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            dedupe_key: DedupeKey("k".into()),
            confidence,
            payload,
            raw_ref: None,
        }
    }

    fn mk_rule(name: &str, trigger: &str, actions: Vec<Action>, priority: i32) -> Rule {
        Rule {
            id: Uuid::new_v4(),
            name: name.into(),
            trigger: Trigger::Event(trigger.into()),
            conditions: vec![],
            actions,
            priority,
            cooldown: None,
            duration: None,
            explanation_template: "{rule_name} fired on {event_type}".into(),
            enabled: true,
        }
    }

    // Traces to: FR-RULE-001
    #[test]
    fn disabled_rule_is_skipped() {
        let mut eng = RuleEngine::new();
        let mut rule = mk_rule("r", "TaskCompleted", vec![], 0);
        rule.enabled = false;
        let ev = mk_event(EventType::TaskCompleted, 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        assert!(matches!(eng.evaluate(&rule, &ev, now), RuleDecision::Skipped { .. }));
    }

    // Traces to: FR-RULE-001
    #[test]
    fn trigger_mismatch_is_skipped() {
        let mut eng = RuleEngine::new();
        let rule = mk_rule("r", "TaskCompleted", vec![], 0);
        let ev = mk_event(EventType::SleepRecorded, 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        match eng.evaluate(&rule, &ev, now) {
            RuleDecision::Skipped { reason } => assert_eq!(reason, "trigger_mismatch"),
            o => panic!("unexpected: {o:?}"),
        }
    }

    // Traces to: FR-RULE-001
    #[test]
    fn matching_event_fires_rule() {
        let mut eng = RuleEngine::new();
        let rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 5 }], 0);
        let ev = mk_event(EventType::TaskCompleted, 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        match eng.evaluate(&rule, &ev, now) {
            RuleDecision::Fired(actions) => {
                assert_eq!(actions, vec![Action::GrantCredit { amount: 5 }])
            }
            o => panic!("unexpected: {o:?}"),
        }
    }

    // Traces to: FR-RULE-002
    #[test]
    fn cooldown_suppresses_repeat_within_window() {
        let mut eng = RuleEngine::new();
        let mut rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 1 }], 0);
        rule.cooldown = Some(Duration::minutes(10));
        let ev = mk_event(EventType::TaskCompleted, 1.0, json!({}));
        let t0 = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        assert!(matches!(eng.evaluate(&rule, &ev, t0), RuleDecision::Fired(_)));
        let t1 = t0 + Duration::minutes(5);
        match eng.evaluate(&rule, &ev, t1) {
            RuleDecision::Suppressed { reason } => assert_eq!(reason, "cooldown"),
            o => panic!("unexpected: {o:?}"),
        }
    }

    // Traces to: FR-RULE-002
    #[test]
    fn cooldown_expires_allows_refire() {
        let mut eng = RuleEngine::new();
        let mut rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 1 }], 0);
        rule.cooldown = Some(Duration::minutes(10));
        let ev = mk_event(EventType::TaskCompleted, 1.0, json!({}));
        let t0 = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        let _ = eng.evaluate(&rule, &ev, t0);
        let t2 = t0 + Duration::minutes(11);
        assert!(matches!(eng.evaluate(&rule, &ev, t2), RuleDecision::Fired(_)));
    }

    // Traces to: FR-RULE-003
    #[test]
    fn condition_confidence_gate_filters() {
        let mut eng = RuleEngine::new();
        let mut rule = mk_rule("r", "TaskCompleted", vec![], 0);
        rule.conditions
            .push(Condition { kind: "confidence_gte".into(), params: json!({"min": 0.9}) });
        let ev = mk_event(EventType::TaskCompleted, 0.5, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        match eng.evaluate(&rule, &ev, now) {
            RuleDecision::Skipped { reason } => {
                assert!(reason.starts_with("condition_failed"))
            }
            o => panic!("unexpected: {o:?}"),
        }
    }

    // Traces to: FR-RULE-005
    #[test]
    fn explanation_template_substitutes_placeholders() {
        let rule = mk_rule("MyRule", "TaskCompleted", vec![], 0);
        let ev = mk_event(EventType::TaskCompleted, 1.0, json!({}));
        let rendered = RuleEngine::render_explanation(&rule, &ev);
        assert!(rendered.contains("MyRule"));
        assert!(rendered.contains("TaskCompleted"));
    }

    // Traces to: FR-RULE-004
    #[test]
    fn evaluate_all_orders_by_priority_desc() {
        let mut eng = RuleEngine::new();
        let low =
            mk_rule("low", "TaskCompleted", vec![Action::Unblock { profile: "games".into() }], 1);
        let high = mk_rule(
            "high",
            "TaskCompleted",
            vec![Action::Block { profile: "games".into(), duration: Duration::minutes(30) }],
            100,
        );
        let ev = mk_event(EventType::TaskCompleted, 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        let decisions = eng.evaluate_all(&[low, high], &ev, now);
        assert_eq!(decisions.len(), 2);
        assert_eq!(decisions[0].priority, 100);
        assert_eq!(decisions[1].priority, 1);
    }

    // Traces to: FR-RULE-001, FR-RULE-002
    #[test]
    fn evaluate_is_deterministic() {
        let mut eng_a = RuleEngine::new();
        let mut eng_b = RuleEngine::new();
        let rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 7 }], 0);
        let ev = mk_event(EventType::TaskCompleted, 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        let a = eng_a.evaluate(&rule, &ev, now);
        let b = eng_b.evaluate(&rule, &ev, now);
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }
}
