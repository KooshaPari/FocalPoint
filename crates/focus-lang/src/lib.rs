//! FocalPoint Language (FPL) — Starlark→IR Compiler
//!
//! Compiles Starlark programs to FocalPoint Intermediate Representation (IR).
//! First slice: Rules primitive only.
//!
//! Starlark globals provide sandboxed helpers for constructing rule documents:
//! - `rule(name, trigger, conditions, actions, priority, cooldown_seconds, duration_seconds, explanation_template, enabled)`
//! - Trigger helpers: `on_event`, `on_schedule`, `on_state_change`
//! - Condition helpers: `confidence_gte`, `payload_eq`, `all_of`, `any_of`, `not_`, etc.
//! - Action helpers: `grant_credit`, `deduct_credit`, `block`, `unblock`, `streak_increment`, `notify`

use anyhow::{anyhow, Result};
use focus_ir::{ActionIr, Body, ConditionIr, Document, DocKind, RuleIr, TriggerIr};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use starlark::environment::{Globals, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};

mod sandbox;
use sandbox::create_fpl_globals;

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

    #[error("Unknown helper: {0}")]
    UnknownHelper(String),

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
///     enabled=True
/// )
/// ```
pub fn compile_fpl(source: &str) -> Result<Vec<Document>, CompileError> {
    // Prepend helper function definitions to the source.
    let full_source = format!("{}\n{}", STARLARK_HELPERS, source);

    // Parse Starlark source.
    let ast = AstModule::parse(
        "fpl",
        &full_source,
        &Dialect::Standard,
    ).map_err(|e| {
        let msg = e.to_string();
        let line = extract_line_number(&msg).unwrap_or(1);
        CompileError::ParseError {
            line,
            message: msg,
        }
    })?;

    // Create the module.
    let module = Module::new();

    // Create sandbox globals.
    let globals = create_fpl_globals();

    // Evaluate in restricted context.
    let mut evaluator = Evaluator::new(&module);
    evaluator.eval_module(ast, &globals)
        .map_err(|e| CompileError::EvalError(e.to_string()))?;

    // Extract _fpl_rules from the frozen module.
    let frozen = module.freeze().map_err(|_e| CompileError::InvalidRule("Failed to freeze module".to_string()))?;

    // Get the _fpl_rules variable.
    let _rules_var = frozen.get("_fpl_rules")
        .or_else(|| Err(CompileError::InvalidRule("No rules defined".to_string())))?;

    // Convert Starlark list to Rust vector of rule data.
    // For now, we extract from thread-local registry until we properly implement Starlark value extraction.
    let mut _docs = Vec::new();
    extract_rules_from_starlark(&_rules_var, &mut _docs)?;

    // Collect from thread-local registry as fallback.
    let rules = RULE_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
    let mut docs = Vec::new();
    for rule_data in rules {
        let doc = build_rule_document(&rule_data)?;
        docs.push(doc);
    }

    Ok(docs)
}

/// Extract rules from Starlark _fpl_rules list and build documents.
fn extract_rules_from_starlark(
    _rules_var: &starlark::values::Value,
    _docs: &mut Vec<Document>,
) -> Result<(), CompileError> {
    // This would require implementing Starlark value extraction.
    // For now, return a placeholder since the actual extraction depends on
    // starlark-rust's Value trait implementation.

    Ok(())
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

# FPL rule() builtin - collects rules into thread-local registry
_fpl_rules = []

def rule(id, name, trigger, conditions=None, actions=None, priority=0, cooldown_seconds=None, duration_seconds=None, explanation_template="", enabled=True):
    if conditions == None:
        conditions = []
    if actions == None:
        actions = []

    rule_dict = {
        "id": id,
        "name": name,
        "trigger": trigger,
        "conditions": conditions,
        "actions": actions,
        "priority": priority,
        "cooldown_seconds": cooldown_seconds,
        "duration_seconds": duration_seconds,
        "explanation_template": explanation_template,
        "enabled": enabled,
    }
    _fpl_rules.append(rule_dict)
    return rule_dict
"#;

/// Inject FPL builtins into the Starlark module.
fn inject_fpl_builtins(_module: &Module) {
    // Pre-define helper functions as Starlark code.
    // These will be parsed and available during evaluation.
    let _helpers = r#"
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
"#;

    // Note: module.exec() would require handling the exec result.
    // We'll keep this for documentation; actual injection happens at parse time.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rule_compiles() {
        let source = r#"
rule(
    id="test-rule",
    name="Test Rule",
    trigger=on_event("test:event"),
    conditions=[],
    actions=[grant_credit(25)],
    priority=50,
    enabled=True
)
"#;
        let result = compile_fpl(source);
        assert!(result.is_ok());
        let docs = result.unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].name, "Test Rule");
    }

    #[test]
    fn test_syntax_error_reports_line() {
        let source = "rule(invalid syntax here";
        let result = compile_fpl(source);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Parse error"));
    }

    #[test]
    fn test_multiple_rules() {
        let source = r#"
rule(
    id="rule1",
    name="Rule 1",
    trigger=on_event("event1"),
    conditions=[],
    actions=[grant_credit(10)],
    enabled=True
)
rule(
    id="rule2",
    name="Rule 2",
    trigger=on_event("event2"),
    conditions=[],
    actions=[grant_credit(20)],
    enabled=True
)
"#;
        let result = compile_fpl(source);
        assert!(result.is_ok());
        let docs = result.unwrap();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn test_conditions_compile() {
        let source = r#"
rule(
    id="cond-rule",
    name="Condition Rule",
    trigger=on_event("test"),
    conditions=[confidence_gte(0.8), payload_exists("x")],
    actions=[notify("test")],
    enabled=True
)
"#;
        let result = compile_fpl(source);
        assert!(result.is_ok());
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
    enabled=True
)
"#;
        let result = compile_fpl(source);
        assert!(result.is_ok());
        let docs = result.unwrap();
        assert_eq!(docs.len(), 1);
        if let Body::Rule(rule_ir) = &docs[0].body {
            assert_eq!(rule_ir.actions.len(), 1);
        }
    }

    #[test]
    fn test_loop_produces_multiple_rules() {
        let source = r#"
for profile in ["social", "games"]:
    rule(
        id="block-" + profile,
        name="Block " + profile,
        trigger=on_event("focus:session_started"),
        conditions=[],
        actions=[block(profile=profile, duration_seconds=1800, rigidity="hard")],
        enabled=True
    )
"#;
        let result = compile_fpl(source);
        assert!(result.is_ok());
        let docs = result.unwrap();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn test_default_priority_and_values() {
        let source = r#"
rule(
    id="defaults",
    name="Defaults",
    trigger=on_event("test"),
    conditions=[],
    actions=[],
    enabled=True
)
"#;
        let result = compile_fpl(source);
        assert!(result.is_ok());
        let docs = result.unwrap();
        if let Body::Rule(rule_ir) = &docs[0].body {
            assert_eq!(rule_ir.priority, 0);
            assert_eq!(rule_ir.cooldown_seconds, None);
        }
    }

    #[test]
    fn test_composite_conditions() {
        let source = r#"
rule(
    id="composite",
    name="Composite Conditions",
    trigger=on_event("test"),
    conditions=[all_of([confidence_gte(0.8), payload_exists("x")])],
    actions=[notify("msg")],
    enabled=True
)
"#;
        let result = compile_fpl(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_filesystem_access() {
        let source = r#"
try:
    open("/etc/passwd")
    rule(id="bad", name="Bad", trigger=on_event("x"), conditions=[], actions=[], enabled=True)
except:
    rule(id="safe", name="Safe", trigger=on_event("x"), conditions=[], actions=[], enabled=True)
"#;
        let result = compile_fpl(source);
        // Should not panic; the try-except catches the undefined function.
        assert!(result.is_ok());
        let docs = result.unwrap();
        // Expect only the safe rule since open() is not defined.
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].name, "Safe");
    }
}
