//! Rule DSL, evaluation, priority, cooldowns, explanation.
//!
//! Traces to FR-RULE-001..005.

use chrono::{DateTime, Duration, Utc};
use focus_coaching::{complete_guarded, prompts, CoachingProvider};
use focus_domain::Rigidity;
use focus_events::{EventType, NormalizedEvent, WellKnownEventType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::warn;
use uuid::Uuid;

fn default_rigidity_hard() -> Rigidity {
    Rigidity::Hard
}

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
    GrantCredit {
        amount: i32,
    },
    DeductCredit {
        amount: i32,
    },
    Block {
        profile: String,
        duration: Duration,
        /// Traces to: FR-RIGIDITY-001. Defaulted to `Hard` for backward
        /// compatibility so existing serialized rules still deserialize.
        #[serde(default = "default_rigidity_hard")]
        rigidity: Rigidity,
    },
    Unblock {
        profile: String,
    },
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
            (
                Block { profile: p1, duration: d1, rigidity: r1 },
                Block { profile: p2, duration: d2, rigidity: r2 },
            ) => p1 == p2 && d1 == d2 && r1 == r2,
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

/// Ask the LLM to convert a natural-language rule spec into a [`Rule`].
pub async fn propose_rule_from_nl(
    nl_spec: &str,
    coaching: &dyn CoachingProvider,
) -> anyhow::Result<Rule> {
    let system = prompts::rule_authoring_prompt();
    let out = complete_guarded(coaching, nl_spec, Some(&system), 800)
        .await?
        .ok_or_else(|| anyhow::anyhow!("coaching provider returned no content"))?;
    let trimmed = out
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    serde_json::from_str::<Rule>(trimmed)
        .map_err(|e| anyhow::anyhow!("LLM returned invalid Rule JSON: {e}; raw output: {out}"))
}

/// Rewrite a rule's explanation template via the LLM, grounded in the event
/// payload. Falls back to the static [`RuleEngine::render_explanation`] on
/// any failure (kill switch, empty response, transport error).
pub async fn render_llm_explanation(
    rule: &Rule,
    event: &NormalizedEvent,
    coaching: &dyn CoachingProvider,
) -> anyhow::Result<String> {
    let fallback = RuleEngine::render_explanation(rule, event);
    let payload = serde_json::to_string(&event.payload).unwrap_or_else(|_| "{}".into());
    let user = format!(
        "Rule name: {}\nStatic template: {}\nEvent type: {}\nEvent payload JSON: {}",
        rule.name,
        rule.explanation_template,
        event_type_name(&event.event_type),
        payload,
    );
    match complete_guarded(coaching, &user, Some(prompts::RULE_EXPLANATION_SYSTEM_PROMPT), 220)
        .await
    {
        Ok(Some(text)) => Ok(text),
        Ok(None) => Ok(fallback),
        Err(e) => {
            warn!(target: "coaching.fallback", error = %e, "explanation LLM error");
            Ok(fallback)
        }
    }
}

/// Match an event's type against a rule trigger pattern.
///
/// Supports:
/// * exact match on `EventType::to_string()` (e.g. `"AssignmentDue"`,
///   `"canvas:quiz_posted"`)
/// * trailing glob, where `"canvas:*"` matches any `Custom("canvas:...")`.
///
/// Traces to: FR-EVT-VOCAB-001, FR-RULE-001.
fn event_type_matches(et: &EventType, expected: &str) -> bool {
    let name = event_type_name(et);
    if let Some(prefix) = expected.strip_suffix('*') {
        name.starts_with(prefix)
    } else {
        name == expected
    }
}

fn event_type_name(et: &EventType) -> String {
    et.to_string()
}

// Keep the WellKnownEventType import referenced so the re-export in other
// modules doesn't dead-code out via the pub-use path.
#[allow(dead_code)]
type _KeepWellKnown = WellKnownEventType;

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
#[allow(clippy::disallowed_methods)]
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
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        assert!(matches!(eng.evaluate(&rule, &ev, now), RuleDecision::Skipped { .. }));
    }

    // Traces to: FR-RULE-001
    #[test]
    fn trigger_mismatch_is_skipped() {
        let mut eng = RuleEngine::new();
        let rule = mk_rule("r", "TaskCompleted", vec![], 0);
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::SleepRecorded), 1.0, json!({}));
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
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
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
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
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
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
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
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 0.5, json!({}));
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
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
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
            vec![Action::Block {
                profile: "games".into(),
                duration: Duration::minutes(30),
                rigidity: Rigidity::Hard,
            }],
            100,
        );
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        let decisions = eng.evaluate_all(&[low, high], &ev, now);
        assert_eq!(decisions.len(), 2);
        assert_eq!(decisions[0].priority, 100);
        assert_eq!(decisions[1].priority, 1);
    }

    // Traces to: FR-EVT-VOCAB-001, FR-RULE-001
    #[test]
    fn trigger_exact_match_against_custom_event_type() {
        let mut eng = RuleEngine::new();
        let rule = mk_rule("r", "canvas:quiz_posted", vec![Action::GrantCredit { amount: 1 }], 0);
        let ev = mk_event(EventType::Custom("canvas:quiz_posted".into()), 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        assert!(matches!(eng.evaluate(&rule, &ev, now), RuleDecision::Fired(_)));
    }

    // Traces to: FR-EVT-VOCAB-001, FR-RULE-001
    #[test]
    fn trigger_prefix_glob_matches_custom_namespace() {
        let mut eng = RuleEngine::new();
        let rule = mk_rule("r", "canvas:*", vec![Action::GrantCredit { amount: 1 }], 0);
        let ev = mk_event(EventType::Custom("canvas:quiz_posted".into()), 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        assert!(matches!(eng.evaluate(&rule, &ev, now), RuleDecision::Fired(_)));
    }

    // Traces to: FR-EVT-VOCAB-001, FR-RULE-001
    #[test]
    fn trigger_prefix_glob_rejects_out_of_namespace_custom() {
        let mut eng = RuleEngine::new();
        let rule = mk_rule("r", "canvas:*", vec![Action::GrantCredit { amount: 1 }], 0);
        let ev = mk_event(EventType::Custom("slack:message".into()), 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        match eng.evaluate(&rule, &ev, now) {
            RuleDecision::Skipped { reason } => assert_eq!(reason, "trigger_mismatch"),
            o => panic!("unexpected: {o:?}"),
        }
    }

    // Traces to: FR-RULE-001, FR-RULE-002
    #[test]
    fn evaluate_is_deterministic() {
        let mut eng_a = RuleEngine::new();
        let mut eng_b = RuleEngine::new();
        let rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 7 }], 0);
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        let a = eng_a.evaluate(&rule, &ev, now);
        let b = eng_b.evaluate(&rule, &ev, now);
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }

    // -----------------------------------------------------------------------
    // LLM coaching integration tests (Stub provider)
    // -----------------------------------------------------------------------

    use focus_coaching::{NoopCoachingProvider, StubCoachingProvider};

    fn sample_rule_json() -> String {
        let id = Uuid::new_v4();
        serde_json::json!({
            "id": id.to_string(),
            "name": "Reward homework",
            "trigger": {"Event": "TaskCompleted"},
            "conditions": [],
            "actions": [{"GrantCredit": {"amount": 5}}],
            "priority": 10,
            "cooldown": null,
            "duration": null,
            "explanation_template": "{rule_name} fired on {event_type}",
            "enabled": true
        })
        .to_string()
    }

    #[tokio::test]
    async fn propose_rule_parses_valid_llm_json() {
        let provider = StubCoachingProvider::single(sample_rule_json());
        let rule = propose_rule_from_nl("give me 5 credits per task completion", &provider)
            .await
            .expect("parse");
        assert_eq!(rule.name, "Reward homework");
        assert_eq!(rule.priority, 10);
    }

    #[tokio::test]
    async fn propose_rule_strips_markdown_fences() {
        let fenced = format!("```json\n{}\n```", sample_rule_json());
        let provider = StubCoachingProvider::single(fenced);
        let rule = propose_rule_from_nl("x", &provider).await.expect("parse");
        assert_eq!(rule.name, "Reward homework");
    }

    #[tokio::test]
    async fn propose_rule_errors_on_garbage() {
        let provider = StubCoachingProvider::single("not even close to json");
        let err = propose_rule_from_nl("whatever", &provider).await.unwrap_err();
        assert!(err.to_string().contains("invalid Rule JSON"));
    }

    #[tokio::test]
    async fn propose_rule_errors_on_noop() {
        let provider = NoopCoachingProvider;
        let err = propose_rule_from_nl("x", &provider).await.unwrap_err();
        assert!(err.to_string().contains("no content"));
    }

    #[tokio::test]
    async fn render_llm_explanation_uses_provider_text() {
        let provider = StubCoachingProvider::single(
            "You finished the assignment — +5 credits banked.".to_string(),
        );
        let rule = mk_rule("Reward", "TaskCompleted", vec![], 0);
        let ev = mk_event(
            EventType::WellKnown(WellKnownEventType::TaskCompleted),
            1.0,
            json!({"title": "Essay"}),
        );
        let out = render_llm_explanation(&rule, &ev, &provider).await.expect("explain");
        assert!(out.contains("+5 credits"));
    }

    #[tokio::test]
    async fn render_llm_explanation_falls_back_when_noop() {
        let rule = mk_rule("Reward", "TaskCompleted", vec![], 0);
        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
        let provider = NoopCoachingProvider;
        let out = render_llm_explanation(&rule, &ev, &provider).await.expect("explain");
        assert!(out.contains("Reward"));
    }
}
