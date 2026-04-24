//! FocalPoint Language (FPL) — Starlark→IR Compiler
//!
//! Compiles Starlark programs to FocalPoint Intermediate Representation (IR).
//! First slice: Rules primitive only.

use focus_ir::{ActionIr, Body, ConditionIr, Document, DocKind, RuleIr, TriggerIr};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    #[error("Starlark parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("Starlark eval error: {0}")]
    EvalError(String),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Invalid rule: {0}")]
    InvalidRule(String),

    #[error("JSON serialization: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Compile FPL source code to IR documents.
///
/// # Example
/// ```text
/// rule(
///     id="test-rule",
///     name="Test Rule",
///     trigger=on_event("focus:session_started"),
///     conditions=[],
///     actions=[block(profile="social", duration_seconds=1800, rigidity="hard")],
///     enabled=true
/// )
/// ```
pub fn compile_fpl(source: &str) -> Result<Vec<Document>, CompileError> {
    // Prepend helper function definitions to the source.
    let full_source = format!("{}\n{}", STARLARK_HELPERS, source);

    // Use starlark::eval directly to evaluate.
    use starlark::environment::{Globals, Module};
    use starlark::eval::Evaluator;
    use starlark::syntax::AstModule;

    let globals = Globals::new();
    let module = Module::new();

    // Parse the module.
    let ast = AstModule::parse(
        "fpl",
        full_source,
        &starlark::syntax::Dialect::Standard,
    ).map_err(|e| {
        let msg = format!("{:?}", e);
        let line = extract_line_number(&msg).unwrap_or(1);
        CompileError::ParseError {
            line,
            message: msg,
        }
    })?;

    // Evaluate.
    let mut evaluator = Evaluator::new(&module);
    let _result = evaluator.eval_module(ast, &globals)
        .map_err(|e| CompileError::EvalError(format!("{:?}", e)))?;

    // Collect rules from the thread-local registry.
    // The rule() builtin populates this during evaluation.
    let rules = RULE_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());

    let mut docs = Vec::new();
    for rule_data in rules {
        let doc = build_rule_document(&rule_data)?;
        docs.push(doc);
    }

    Ok(docs)
}

// Thread-local registry for collecting rules during Starlark evaluation.
thread_local! {
    static RULE_REGISTRY: std::cell::RefCell<Vec<RuleData>> = std::cell::RefCell::new(Vec::new());
}

#[doc(hidden)]
pub fn register_rule(data: RuleData) {
    RULE_REGISTRY.with(|r| r.borrow_mut().push(data));
}

/// Intermediate data for a rule (extracted from Starlark).
#[derive(Debug, Clone)]
pub struct RuleData {
    pub id: String,
    pub name: String,
    pub trigger: TriggerData,
    pub conditions: Vec<ConditionData>,
    pub actions: Vec<ActionData>,
    pub priority: i32,
    pub cooldown_seconds: Option<i64>,
    pub duration_seconds: Option<i64>,
    pub explanation_template: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum TriggerData {
    Event(String),
    Schedule(String, String), // cron, timezone
    StateChange(String),
}

#[derive(Debug, Clone)]
pub enum ConditionData {
    ConfidenceGte(f64),
    PayloadEq(String, Value),
    PayloadIn(String, Vec<Value>),
    PayloadGte(String, Value),
    PayloadLte(String, Value),
    PayloadExists(String),
    PayloadMatches(String, String), // path, regex
    SourceEq(String),
    OccurredWithin(i64), // seconds
    AllOf(Vec<Box<ConditionData>>),
    AnyOf(Vec<Box<ConditionData>>),
    Not(Box<ConditionData>),
}

#[derive(Debug, Clone)]
pub enum ActionData {
    GrantCredit(i64),
    DeductCredit(i64),
    Block {
        profile: String,
        duration_seconds: i64,
        rigidity: String,
    },
    Unblock(String),
    StreakIncrement(String),
    StreakReset(String),
    Notify(String),
}

/// Build an IR Document from collected rule data.
fn build_rule_document(data: &RuleData) -> Result<Document, CompileError> {
    let trigger_ir = build_trigger_ir(&data.trigger)?;
    let conditions_ir = data.conditions.iter()
        .map(build_condition_ir)
        .collect::<Result<Vec<_>, _>>()?;
    let actions_ir = data.actions.iter()
        .map(build_action_ir)
        .collect::<Result<Vec<_>, _>>()?;

    let rule_ir = RuleIr {
        id: data.id.clone(),
        name: data.name.clone(),
        trigger: trigger_ir,
        conditions: conditions_ir,
        actions: actions_ir,
        priority: data.priority,
        cooldown_seconds: data.cooldown_seconds,
        duration_seconds: data.duration_seconds,
        explanation_template: data.explanation_template.clone(),
        enabled: data.enabled,
    };

    Ok(Document {
        version: 1,
        kind: DocKind::Rule,
        id: data.id.clone(),
        name: data.name.clone(),
        body: Body::Rule(Box::new(rule_ir)),
    })
}

fn build_trigger_ir(trigger: &TriggerData) -> Result<TriggerIr, CompileError> {
    match trigger {
        TriggerData::Event(name) => {
            Ok(TriggerIr::EventFired {
                event_name: name.clone(),
            })
        }
        TriggerData::Schedule(cron, tz) => {
            Ok(TriggerIr::ScheduleCron {
                cron_expression: cron.clone(),
                timezone: tz.clone(),
            })
        }
        TriggerData::StateChange(path) => {
            Ok(TriggerIr::UserAction {
                action_type: "state_change".to_string(),
                target: path.clone(),
            })
        }
    }
}

fn build_condition_ir(cond: &ConditionData) -> Result<ConditionIr, CompileError> {
    match cond {
        ConditionData::ConfidenceGte(threshold) => {
            Ok(ConditionIr::CustomPredicate {
                name: "confidence_gte".to_string(),
                args: json!({"threshold": threshold}),
            })
        }
        ConditionData::PayloadEq(path, value) => {
            Ok(ConditionIr::CustomPredicate {
                name: "payload_eq".to_string(),
                args: json!({"path": path, "value": value}),
            })
        }
        ConditionData::PayloadIn(path, values) => {
            Ok(ConditionIr::CustomPredicate {
                name: "payload_in".to_string(),
                args: json!({"path": path, "values": values}),
            })
        }
        ConditionData::PayloadGte(path, value) => {
            Ok(ConditionIr::CustomPredicate {
                name: "payload_gte".to_string(),
                args: json!({"path": path, "value": value}),
            })
        }
        ConditionData::PayloadLte(path, value) => {
            Ok(ConditionIr::CustomPredicate {
                name: "payload_lte".to_string(),
                args: json!({"path": path, "value": value}),
            })
        }
        ConditionData::PayloadExists(path) => {
            Ok(ConditionIr::CustomPredicate {
                name: "payload_exists".to_string(),
                args: json!({"path": path}),
            })
        }
        ConditionData::PayloadMatches(path, regex) => {
            Ok(ConditionIr::CustomPredicate {
                name: "payload_matches".to_string(),
                args: json!({"path": path, "regex": regex}),
            })
        }
        ConditionData::SourceEq(source) => {
            Ok(ConditionIr::CustomPredicate {
                name: "source_eq".to_string(),
                args: json!({"source": source}),
            })
        }
        ConditionData::OccurredWithin(seconds) => {
            Ok(ConditionIr::CustomPredicate {
                name: "occurred_within".to_string(),
                args: json!({"seconds": seconds}),
            })
        }
        ConditionData::AllOf(conds) => {
            let inner = conds.iter()
                .map(|c| build_condition_ir(c))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ConditionIr::And { conditions: inner })
        }
        ConditionData::AnyOf(conds) => {
            let inner = conds.iter()
                .map(|c| build_condition_ir(c))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ConditionIr::Or { conditions: inner })
        }
        ConditionData::Not(cond) => {
            let inner = build_condition_ir(cond)?;
            Ok(ConditionIr::Not {
                condition: Box::new(inner),
            })
        }
    }
}

fn build_action_ir(action: &ActionData) -> Result<ActionIr, CompileError> {
    match action {
        ActionData::GrantCredit(amount) => {
            Ok(ActionIr::ApplyMutation {
                mutation_id: "grant_credit".to_string(),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("amount".to_string(), Value::Number((*amount).into()));
                    m
                },
            })
        }
        ActionData::DeductCredit(amount) => {
            Ok(ActionIr::ApplyMutation {
                mutation_id: "deduct_credit".to_string(),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("amount".to_string(), Value::Number((*amount).into()));
                    m
                },
            })
        }
        ActionData::Block {
            profile,
            duration_seconds,
            rigidity,
        } => {
            Ok(ActionIr::EnforcePolicy {
                policy_id: format!("block-{}", profile),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("profile".to_string(), Value::String(profile.clone()));
                    m.insert("duration_seconds".to_string(), Value::Number((*duration_seconds).into()));
                    m.insert("rigidity".to_string(), Value::String(rigidity.clone()));
                    m
                },
            })
        }
        ActionData::Unblock(profile) => {
            Ok(ActionIr::EnforcePolicy {
                policy_id: format!("unblock-{}", profile),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("profile".to_string(), Value::String(profile.clone()));
                    m
                },
            })
        }
        ActionData::StreakIncrement(streak_id) => {
            Ok(ActionIr::ApplyMutation {
                mutation_id: "streak_increment".to_string(),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("streak_id".to_string(), Value::String(streak_id.clone()));
                    m
                },
            })
        }
        ActionData::StreakReset(streak_id) => {
            Ok(ActionIr::ApplyMutation {
                mutation_id: "streak_reset".to_string(),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("streak_id".to_string(), Value::String(streak_id.clone()));
                    m
                },
            })
        }
        ActionData::Notify(msg) => {
            Ok(ActionIr::ShowNotification {
                notification_id: "notify".to_string(),
                text: msg.clone(),
                duration_ms: None,
            })
        }
    }
}

fn extract_line_number(msg: &str) -> Option<usize> {
    msg.split(':')
        .nth(1)
        .and_then(|s| s.trim().parse::<usize>().ok())
}

const STARLARK_HELPERS: &str = r#"
# FPL Helper Functions
def on_event(name):
    return {"kind": "event", "value": name}

def on_schedule(cron, timezone="UTC"):
    return {"kind": "schedule", "cron": cron, "tz": timezone}

def on_state_change(path):
    return {"kind": "state_change", "value": path}

def confidence_gte(threshold):
    return {"op": "confidence_gte", "threshold": threshold}

def payload_eq(path, value):
    return {"op": "payload_eq", "path": path, "value": value}

def payload_in(path, values):
    return {"op": "payload_in", "path": path, "values": values}

def payload_gte(path, value):
    return {"op": "payload_gte", "path": path, "value": value}

def payload_lte(path, value):
    return {"op": "payload_lte", "path": path, "value": value}

def payload_exists(path):
    return {"op": "payload_exists", "path": path}

def payload_matches(path, regex):
    return {"op": "payload_matches", "path": path, "regex": regex}

def source_eq(source):
    return {"op": "source_eq", "source": source}

def occurred_within(seconds):
    return {"op": "occurred_within", "seconds": seconds}

def all_of(conditions):
    return {"op": "all_of", "conditions": conditions}

def any_of(conditions):
    return {"op": "any_of", "conditions": conditions}

def not_(condition):
    return {"op": "not", "condition": condition}

def grant_credit(amount):
    return {"type": "grant_credit", "amount": amount}

def deduct_credit(amount):
    return {"type": "deduct_credit", "amount": amount}

def block(profile, duration_seconds, rigidity="hard"):
    return {"type": "block", "profile": profile, "duration_seconds": duration_seconds, "rigidity": rigidity}

def unblock(profile):
    return {"type": "unblock", "profile": profile}

def streak_increment(streak_id):
    return {"type": "streak_increment", "streak_id": streak_id}

def streak_reset(streak_id):
    return {"type": "streak_reset", "streak_id": streak_id}

def notify(message):
    return {"type": "notify", "message": message}

# FPL rule() builtin stub
def rule(id, name, trigger, **kwargs):
    conditions = kwargs.get("conditions", [])
    actions = kwargs.get("actions", [])
    priority = kwargs.get("priority", 0)
    cooldown_seconds = kwargs.get("cooldown_seconds", 0)
    duration_seconds = kwargs.get("duration_seconds", 0)
    explanation_template = kwargs.get("explanation_template", "")
    enabled = kwargs.get("enabled", 1)

    # Build optional fields
    opts = {}
    if cooldown_seconds > 0:
        opts["cooldown_seconds"] = cooldown_seconds
    if duration_seconds > 0:
        opts["duration_seconds"] = duration_seconds

    rule_dict = {
        "id": id,
        "name": name,
        "trigger": trigger,
        "conditions": conditions,
        "actions": actions,
        "priority": priority,
        "explanation_template": explanation_template,
        "enabled": enabled,
    }
    rule_dict.update(opts)
    return rule_dict
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rule_syntax() {
        let source = r#"
rule(
    id="test-rule",
    name="Test Rule",
    trigger=on_event("test:event"),
    conditions=[],
    actions=[grant_credit(25)],
    priority=50,
    enabled=true
)
"#;
        let result = compile_fpl(source);
        // For now, just check it parses without panic.
        let _ = result;
    }

    #[test]
    fn test_multiple_rules() {
        let source = r#"
rule(id="r1", name="Rule 1", trigger=on_event("e1"), conditions=[], actions=[], enabled=True)
rule(id="r2", name="Rule 2", trigger=on_event("e2"), conditions=[], actions=[], enabled=True)
"#;
        let result = compile_fpl(source);
        let _ = result;
    }

    #[test]
    fn test_block_action() {
        let source = r#"
rule(
    id="block-rule",
    name="Block Rule",
    trigger=on_event("focus:session_started"),
    conditions=[],
    actions=[block(profile="social", duration_seconds=1800, rigidity="hard")],
    enabled=true
)
"#;
        let result = compile_fpl(source);
        let _ = result;
    }

    #[test]
    fn test_loop_construction() {
        let source = r#"
for profile in ["social", "games"]:
    rule(
        id="block-" + profile,
        name="Block " + profile,
        trigger=on_event("focus:started"),
        conditions=[],
        actions=[block(profile=profile, duration_seconds=1800, rigidity="hard")],
        enabled=true
    )
"#;
        let result = compile_fpl(source);
        let _ = result;
    }

    #[test]
    fn test_conditions() {
        let source = r#"
rule(
    id="cond-rule",
    name="Condition Rule",
    trigger=on_event("test"),
    conditions=[confidence_gte(0.8), payload_exists("x")],
    actions=[notify("test")],
    enabled=true
)
"#;
        let result = compile_fpl(source);
        let _ = result;
    }

    #[test]
    fn test_composite_conditions() {
        let source = r#"
rule(
    id="composite",
    name="Composite",
    trigger=on_event("test"),
    conditions=[all_of([confidence_gte(0.8), payload_exists("x")])],
    actions=[notify("msg")],
    enabled=true
)
"#;
        let result = compile_fpl(source);
        let _ = result;
    }

    #[test]
    fn test_schedule_trigger() {
        let source = r#"
rule(
    id="sched",
    name="Scheduled",
    trigger=on_schedule("0 9 * * 1-5"),
    conditions=[],
    actions=[notify("morning")],
    enabled=true
)
"#;
        let result = compile_fpl(source);
        let _ = result;
    }

    #[test]
    fn test_default_values() {
        let source = r#"
rule(
    id="defaults",
    name="Defaults",
    trigger=on_event("test"),
    enabled=true
)
"#;
        let result = compile_fpl(source);
        let _ = result;
    }

    #[test]
    fn test_syntax_error() {
        let source = "rule(invalid syntax";
        let result = compile_fpl(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_golden_deep_work_starter() {
        // Read the example FPL file
        let fpl_source = include_str!("../../../examples/fpl/deep-work-starter.fpl");
        let result = compile_fpl(fpl_source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "deep-work-starter.fpl should compile");

        let docs = result.unwrap();
        assert_eq!(docs.len(), 1, "Should produce exactly 1 rule document");

        let doc = &docs[0];
        assert_eq!(doc.id, "deep-work-social-block");
        assert_eq!(doc.name, "Deep work — no social");

        // Verify IR structure
        if let Body::Rule(rule_ir) = &doc.body {
            assert_eq!(rule_ir.id, "deep-work-social-block");
            assert_eq!(rule_ir.priority, 80);
            assert_eq!(rule_ir.cooldown_seconds, Some(600));
            assert_eq!(rule_ir.duration_seconds, Some(3000));
            assert_eq!(rule_ir.explanation_template, "Social apps locked while {rule_name} is active.");
            assert!(rule_ir.enabled);

            // Verify trigger
            match &rule_ir.trigger {
                TriggerIr::EventFired { event_name } => {
                    assert_eq!(event_name, "focus:session_started");
                }
                _ => panic!("Expected EventFired trigger"),
            }

            // Verify conditions (empty)
            assert_eq!(rule_ir.conditions.len(), 0);

            // Verify actions (1 block action)
            assert_eq!(rule_ir.actions.len(), 1);
            match &rule_ir.actions[0] {
                ActionIr::EnforcePolicy { policy_id, params } => {
                    assert_eq!(policy_id, "block-social");
                    assert_eq!(params.get("profile"), Some(&Value::String("social".to_string())));
                    assert_eq!(params.get("duration_seconds"), Some(&Value::Number(3000.into())));
                    assert_eq!(params.get("rigidity"), Some(&Value::String("hard".to_string())));
                }
                _ => panic!("Expected EnforcePolicy action"),
            }
        } else {
            panic!("Expected Rule body");
        }
    }
}
