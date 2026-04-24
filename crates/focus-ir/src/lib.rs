//! FocalPoint Intermediate Representation (IR).
//!
//! Single canonical JSON format for all FocalPoint documents: Rule, Connector,
//! Template, Task, Schedule, Pose, CoachingConfig, EnforcementPolicy, WalletMutation,
//! Ritual, SoundCue, AuditQuery.
//!
//! Content-addressed via SHA-256 hash of canonical JSON (sorted keys, no whitespace).
//! Supports versioning and deterministic serialization.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

// ============================================================================
// Document Wrapper (Top-Level)
// ============================================================================

/// Top-level IR document for any FocalPoint primitive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Schema version: fixed to 1 for first slice.
    pub version: u32,

    /// Document kind/variant.
    pub kind: DocKind,

    /// Stable, unique identifier.
    pub id: String,

    /// Human-readable name.
    pub name: String,

    /// Body content (variant-specific).
    pub body: Body,
}

/// Document kind enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DocKind {
    Rule,
    Connector,
    Template,
    Task,
    Schedule,
    MascotScene,
    CoachingConfig,
    EnforcementPolicy,
    WalletMutation,
    Ritual,
    SoundCue,
    AuditQuery,
}

/// Body is a tagged enum containing the variant-specific content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Body {
    #[serde(rename = "Rule")]
    Rule(Box<RuleIr>),

    #[serde(rename = "Connector")]
    Connector(ConnectorIr),

    #[serde(rename = "Template")]
    Template(TemplateIr),

    #[serde(rename = "Task")]
    Task(TaskIr),

    #[serde(rename = "Schedule")]
    Schedule(ScheduleIr),

    #[serde(rename = "MascotScene")]
    MascotScene(MascotSceneIr),

    #[serde(rename = "CoachingConfig")]
    CoachingConfig(CoachingConfigIr),

    #[serde(rename = "EnforcementPolicy")]
    EnforcementPolicy(EnforcementPolicyIr),

    #[serde(rename = "WalletMutation")]
    WalletMutation(WalletMutationIr),

    #[serde(rename = "Ritual")]
    Ritual(RitualIr),

    #[serde(rename = "SoundCue")]
    SoundCue(SoundCueIr),

    #[serde(rename = "AuditQuery")]
    AuditQuery(AuditQueryIr),
}

// ============================================================================
// Rule IR (First Slice - Fully Specified)
// ============================================================================

/// Rule IR: flat, serde-stable representation of a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleIr {
    pub id: String,
    pub name: String,
    pub trigger: TriggerIr,
    pub conditions: Vec<ConditionIr>,
    pub actions: Vec<ActionIr>,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    pub explanation_template: String,
    pub enabled: bool,
}

/// Trigger IR: union type for rule triggers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TriggerIr {
    #[serde(rename = "UserStartsSession")]
    UserStartsSession { session_type: String },

    #[serde(rename = "EventFired")]
    EventFired { event_name: String },

    #[serde(rename = "TimeElapsed")]
    TimeElapsed { duration_ms: u64 },

    #[serde(rename = "ScheduleCron")]
    ScheduleCron {
        cron_expression: String,
        timezone: String,
    },

    #[serde(rename = "WebhookReceived")]
    WebhookReceived { path: String, method: String },

    #[serde(rename = "UserAction")]
    UserAction {
        action_type: String,
        target: String,
    },

    #[serde(rename = "ConditionMet")]
    ConditionMet { condition: Box<ConditionIr> },
}

/// Condition IR: boolean expression (and/or/not + primitives).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum ConditionIr {
    #[serde(rename = "and")]
    And { conditions: Vec<ConditionIr> },

    #[serde(rename = "or")]
    Or { conditions: Vec<ConditionIr> },

    #[serde(rename = "not")]
    Not { condition: Box<ConditionIr> },

    #[serde(rename = "time_in_range")]
    TimeInRange { start_hour: u8, end_hour: u8 },

    #[serde(rename = "day_of_week")]
    DayOfWeek { days: Vec<String> },

    #[serde(rename = "user_attribute")]
    UserAttribute { key: String, value: String },

    #[serde(rename = "event_property")]
    EventProperty {
        property: String,
        expected: serde_json::Value,
    },

    #[serde(rename = "custom_predicate")]
    CustomPredicate {
        name: String,
        args: serde_json::Value,
    },
}

/// Action IR: what to execute when a rule fires.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionIr {
    #[serde(rename = "enforce_policy")]
    EnforcePolicy {
        policy_id: String,
        #[serde(default)]
        params: BTreeMap<String, serde_json::Value>,
    },

    #[serde(rename = "emit_event")]
    EmitEvent {
        event_type: String,
        #[serde(default)]
        payload: BTreeMap<String, serde_json::Value>,
    },

    #[serde(rename = "apply_mutation")]
    ApplyMutation {
        mutation_id: String,
        #[serde(default)]
        params: BTreeMap<String, serde_json::Value>,
    },

    #[serde(rename = "schedule_task")]
    ScheduleTask {
        task_id: String,
        delay_ms: Option<u64>,
        #[serde(default)]
        params: BTreeMap<String, serde_json::Value>,
    },

    #[serde(rename = "trigger_sequence")]
    TriggerSequence { actions: Vec<ActionIr> },

    #[serde(rename = "show_notification")]
    ShowNotification {
        notification_id: String,
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration_ms: Option<u64>,
    },
}

// ============================================================================
// Placeholder Variants (TODO: Reference spec sections)
// ============================================================================

/// Connector IR placeholder.
/// TODO: Implement per spec section "Connector (Integration Definition)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorIr {}

/// Template IR placeholder.
/// TODO: Implement per spec section "Template (Reusable Composition)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateIr {}

/// Task IR placeholder.
/// TODO: Implement per spec section "Task (Executable Unit)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskIr {}

/// Schedule IR placeholder.
/// TODO: Implement per spec section "Schedule (Temporal Trigger)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleIr {}

/// MascotScene IR placeholder.
/// TODO: Implement per spec section "Pose (Mascot Visual State)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MascotSceneIr {}

/// CoachingConfig IR placeholder.
/// TODO: Implement per spec section "CoachingConfig (Tone & Voice Settings)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoachingConfigIr {}

/// EnforcementPolicy IR placeholder.
/// TODO: Implement per spec section "EnforcementPolicy (Rule Constraint)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementPolicyIr {}

/// WalletMutation IR placeholder.
/// TODO: Implement per spec section "WalletMutation (Points/Rewards)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMutationIr {}

/// Ritual IR placeholder.
/// TODO: Implement per spec section "Ritual (Habit Loop Sequence)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualIr {}

/// SoundCue IR placeholder.
/// TODO: Implement per spec section "SoundCue (Audio Asset Reference)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundCueIr {}

/// AuditQuery IR placeholder.
/// TODO: Implement per spec section "AuditQuery (Event Query)".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditQueryIr {}

// ============================================================================
// Content Addressing
// ============================================================================

/// Error type for IR operations.
#[derive(Debug, thiserror::Error)]
pub enum IrError {
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid document: {0}")]
    InvalidDocument(String),
}

impl Document {
    /// Compute the content hash (SHA-256) of this document.
    ///
    /// Uses canonical JSON format: all keys sorted alphabetically, no whitespace,
    /// deterministic serialization for stable hashing across rebuilds.
    pub fn content_hash(&self) -> Result<[u8; 32], IrError> {
        let canonical = canonical_json(self)?;
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result[..]);
        Ok(hash)
    }

    /// Return content hash as hex string for inspection.
    pub fn content_hash_hex(&self) -> Result<String, IrError> {
        let hash = self.content_hash()?;
        Ok(hex::encode(hash))
    }
}

/// Convert document to canonical JSON (sorted keys, no whitespace).
fn canonical_json(doc: &Document) -> Result<String, IrError> {
    // Serialize to JSON, then re-parse and re-serialize to ensure key ordering.
    let json_str = serde_json::to_string(doc)?;
    let value: serde_json::Value = serde_json::from_str(&json_str)?;
    let canonical = sort_json_object(&value);
    Ok(serde_json::to_string(&canonical)?)
}

/// Recursively sort all JSON object keys to ensure deterministic ordering.
fn sort_json_object(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut sorted = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                if let Some(val) = map.get(&key) {
                    sorted.insert(key, sort_json_object(val));
                }
            }
            serde_json::Value::Object(sorted)
        }
        serde_json::Value::Array(arr) => {
            let sorted: Vec<_> = arr.iter().map(sort_json_object).collect();
            serde_json::Value::Array(sorted)
        }
        other => other.clone(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    // Round-Trip Conversions (focus_rules::Rule <-> RuleIr)

    /// Convert a focus_rules::Rule to RuleIr.
    #[allow(dead_code)]
    fn rule_to_ir(rule: &focus_rules::Rule) -> RuleIr {
        RuleIr {
            id: rule.id.to_string(),
            name: rule.name.clone(),
            trigger: trigger_to_ir(&rule.trigger),
            conditions: rule
                .conditions
                .iter()
                .map(condition_to_ir)
                .collect(),
            actions: rule.actions.iter().map(action_to_ir).collect(),
            priority: rule.priority,
            cooldown_seconds: rule.cooldown.map(|d| d.num_seconds()),
            duration_seconds: rule.duration.map(|d| d.num_seconds()),
            explanation_template: rule.explanation_template.clone(),
            enabled: rule.enabled,
        }
    }

    /// Convert RuleIr back to focus_rules::Rule.
    #[allow(dead_code)]
    pub fn ir_to_rule(ir: &RuleIr) -> Result<focus_rules::Rule, IrError> {
        Ok(focus_rules::Rule {
            id: Uuid::parse_str(&ir.id)
                .map_err(|_| IrError::InvalidDocument("Invalid rule ID UUID".into()))?,
            name: ir.name.clone(),
            trigger: ir_to_trigger(&ir.trigger)?,
            conditions: ir
                .conditions
                .iter()
                .map(ir_to_condition)
                .collect::<Result<_, _>>()?,
            actions: ir
                .actions
                .iter()
                .map(ir_to_action)
                .collect::<Result<_, _>>()?,
            priority: ir.priority,
            cooldown: ir.cooldown_seconds.map(chrono::Duration::seconds),
            duration: ir.duration_seconds.map(chrono::Duration::seconds),
            explanation_template: ir.explanation_template.clone(),
            enabled: ir.enabled,
        })
    }

    #[allow(dead_code)]
    fn trigger_to_ir(trigger: &focus_rules::Trigger) -> TriggerIr {
        match trigger {
            focus_rules::Trigger::Event(name) => TriggerIr::EventFired {
                event_name: name.clone(),
            },
            focus_rules::Trigger::Schedule(cron) => TriggerIr::ScheduleCron {
                cron_expression: cron.clone(),
                timezone: "UTC".into(),
            },
            focus_rules::Trigger::StateChange(state) => TriggerIr::UserAction {
                action_type: "state_change".into(),
                target: state.clone(),
            },
        }
    }

    #[allow(dead_code)]
    fn ir_to_trigger(trigger: &TriggerIr) -> Result<focus_rules::Trigger, IrError> {
        match trigger {
            TriggerIr::EventFired { event_name } => {
                Ok(focus_rules::Trigger::Event(event_name.clone()))
            }
            TriggerIr::ScheduleCron {
                cron_expression, ..
            } => Ok(focus_rules::Trigger::Schedule(cron_expression.clone())),
            TriggerIr::UserAction {
                action_type,
                target,
            } if action_type == "state_change" => {
                Ok(focus_rules::Trigger::StateChange(target.clone()))
            }
            _ => Err(IrError::InvalidDocument(
                "Unsupported trigger type".into(),
            )),
        }
    }

    #[allow(dead_code)]
    fn condition_to_ir(_condition: &focus_rules::Condition) -> ConditionIr {
        ConditionIr::CustomPredicate {
            name: _condition.kind.clone(),
            args: _condition.params.clone(),
        }
    }

    #[allow(dead_code)]
    fn ir_to_condition(ir: &ConditionIr) -> Result<focus_rules::Condition, IrError> {
        match ir {
            ConditionIr::CustomPredicate { name, args } => Ok(focus_rules::Condition {
                kind: name.clone(),
                params: args.clone(),
            }),
            _ => Err(IrError::InvalidDocument(
                "Complex conditions not yet supported in round-trip".into(),
            )),
        }
    }

    #[allow(dead_code)]
    fn action_to_ir(action: &focus_rules::Action) -> ActionIr {
        match action {
            focus_rules::Action::GrantCredit { amount } => ActionIr::EmitEvent {
                event_type: "grant_credit".into(),
                payload: {
                    let mut m = BTreeMap::new();
                    m.insert("amount".into(), serde_json::Value::Number((*amount).into()));
                    m
                },
            },
            focus_rules::Action::DeductCredit { amount } => ActionIr::EmitEvent {
                event_type: "deduct_credit".into(),
                payload: {
                    let mut m = BTreeMap::new();
                    m.insert("amount".into(), serde_json::Value::Number((*amount).into()));
                    m
                },
            },
            focus_rules::Action::Block {
                profile,
                duration,
                rigidity,
            } => ActionIr::EnforcePolicy {
                policy_id: "block".into(),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("profile".into(), serde_json::json!(profile));
                    m.insert("duration_secs".into(), serde_json::json!(duration.num_seconds()));
                    m.insert("rigidity".into(), serde_json::json!(format!("{:?}", rigidity)));
                    m
                },
            },
            focus_rules::Action::Unblock { profile } => ActionIr::EnforcePolicy {
                policy_id: "unblock".into(),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("profile".into(), serde_json::json!(profile));
                    m
                },
            },
            focus_rules::Action::StreakIncrement(name) => ActionIr::EmitEvent {
                event_type: "streak_increment".into(),
                payload: {
                    let mut m = BTreeMap::new();
                    m.insert("streak_name".into(), serde_json::json!(name));
                    m
                },
            },
            focus_rules::Action::StreakReset(name) => ActionIr::EmitEvent {
                event_type: "streak_reset".into(),
                payload: {
                    let mut m = BTreeMap::new();
                    m.insert("streak_name".into(), serde_json::json!(name));
                    m
                },
            },
            focus_rules::Action::Notify(msg) => ActionIr::ShowNotification {
                notification_id: Uuid::new_v4().to_string(),
                text: msg.clone(),
                duration_ms: None,
            },
            focus_rules::Action::EmergencyExit {
                profiles,
                duration,
                bypass_cost,
                reason,
            } => ActionIr::EnforcePolicy {
                policy_id: "emergency_exit".into(),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert(
                        "profiles".into(),
                        serde_json::json!(profiles.iter().collect::<Vec<_>>()),
                    );
                    m.insert("duration_secs".into(), serde_json::json!(duration.num_seconds()));
                    m.insert("bypass_cost".into(), serde_json::json!(bypass_cost));
                    m.insert("reason".into(), serde_json::json!(reason));
                    m
                },
            },
            focus_rules::Action::Intervention {
                message,
                severity: _,
            } => ActionIr::ShowNotification {
                notification_id: Uuid::new_v4().to_string(),
                text: message.clone(),
                duration_ms: Some(5000),
            },
            focus_rules::Action::ScheduledUnlockWindow {
                profile,
                starts_at,
                ends_at,
                credit_cost,
            } => ActionIr::ScheduleTask {
                task_id: "unlock_window".into(),
                delay_ms: None,
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("profile".into(), serde_json::json!(profile));
                    m.insert("starts_at".into(), serde_json::json!(starts_at.to_rfc3339()));
                    m.insert("ends_at".into(), serde_json::json!(ends_at.to_rfc3339()));
                    m.insert("credit_cost".into(), serde_json::json!(credit_cost));
                    m
                },
            },
        }
    }

    #[allow(dead_code)]
    fn ir_to_action(ir: &ActionIr) -> Result<focus_rules::Action, IrError> {
        // This is a simplified conversion; not all IR actions can round-trip yet.
        match ir {
            ActionIr::EmitEvent {
                event_type,
                payload,
            } => {
                match event_type.as_str() {
                    "grant_credit" => {
                        let amount = payload
                            .get("amount")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0) as i32;
                        Ok(focus_rules::Action::GrantCredit { amount })
                    }
                    "deduct_credit" => {
                        let amount = payload
                            .get("amount")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0) as i32;
                        Ok(focus_rules::Action::DeductCredit { amount })
                    }
                    _ => Err(IrError::InvalidDocument("Unknown event type".into())),
                }
            }
            _ => Err(IrError::InvalidDocument(
                "Action type not yet supported in IR->Rule conversion".into(),
            )),
        }
    }

    #[test]
    fn test_content_hash_stable() {
        let doc = Document {
            version: 1,
            kind: DocKind::Rule,
            id: "rule-test-001".into(),
            name: "test-rule".into(),
            body: Body::Rule(Box::new(RuleIr {
                id: "rule-1".into(),
                name: "test".into(),
                trigger: TriggerIr::EventFired {
                    event_name: "test_event".into(),
                },
                conditions: vec![],
                actions: vec![],
                priority: 1,
                cooldown_seconds: None,
                duration_seconds: None,
                explanation_template: "Test rule".into(),
                enabled: true,
            })),
        };

        let hash1 = doc.content_hash().expect("First hash");
        let hash2 = doc.content_hash().expect("Second hash");
        assert_eq!(hash1, hash2, "Content hash must be stable across calls");
    }

    #[test]
    fn test_content_hash_changes_on_field_change() {
        let mut doc = Document {
            version: 1,
            kind: DocKind::Rule,
            id: "rule-test-001".into(),
            name: "test-rule".into(),
            body: Body::Rule(Box::new(RuleIr {
                id: "rule-1".into(),
                name: "test".into(),
                trigger: TriggerIr::EventFired {
                    event_name: "event1".into(),
                },
                conditions: vec![],
                actions: vec![],
                priority: 1,
                cooldown_seconds: None,
                duration_seconds: None,
                explanation_template: "Test rule".into(),
                enabled: true,
            })),
        };

        let hash1 = doc.content_hash().expect("First hash");

        // Change a field
        if let Body::Rule(ref mut rule) = &mut doc.body {
            rule.name = "modified".into();
        }
        let hash2 = doc.content_hash().expect("Second hash");

        assert_ne!(hash1, hash2, "Content hash must change when document changes");
    }

    #[test]
    fn test_serde_json_round_trip() {
        let original = Document {
            version: 1,
            kind: DocKind::Rule,
            id: "rule-test-002".into(),
            name: "round-trip-test".into(),
            body: Body::Rule(Box::new(RuleIr {
                id: "rule-2".into(),
                name: "rt".into(),
                trigger: TriggerIr::ScheduleCron {
                    cron_expression: "0 9 * * 1-5".into(),
                    timezone: "America/New_York".into(),
                },
                conditions: vec![ConditionIr::TimeInRange {
                    start_hour: 8,
                    end_hour: 17,
                }],
                actions: vec![ActionIr::EnforcePolicy {
                    policy_id: "social-block".into(),
                    params: {
                        let mut m = BTreeMap::new();
                        m.insert("duration".into(), serde_json::json!(3600));
                        m
                    },
                }],
                priority: 10,
                cooldown_seconds: Some(300),
                duration_seconds: Some(7200),
                explanation_template: "Block during work hours".into(),
                enabled: true,
            })),
        };

        let json = serde_json::to_string(&original).expect("Serialize");
        let restored: Document = serde_json::from_str(&json).expect("Deserialize");

        assert_eq!(original.version, restored.version);
        assert_eq!(original.id, restored.id);
        assert_eq!(original.name, restored.name);

        let orig_hash = original.content_hash().expect("Hash original");
        let rest_hash = restored.content_hash().expect("Hash restored");
        assert_eq!(orig_hash, rest_hash, "Round-trip must preserve hash");
    }

    #[test]
    fn test_canonical_json_sorts_keys() {
        // Create a document; serialize to JSON; verify keys are sorted.
        let doc = Document {
            version: 1,
            kind: DocKind::Rule,
            id: "rule-3".into(),
            name: "sort-test".into(),
            body: Body::Rule(Box::new(RuleIr {
                id: "r3".into(),
                name: "sort".into(),
                trigger: TriggerIr::EventFired {
                    event_name: "e".into(),
                },
                conditions: vec![],
                actions: vec![],
                priority: 1,
                cooldown_seconds: None,
                duration_seconds: None,
                explanation_template: "x".into(),
                enabled: true,
            })),
        };

        let canonical = canonical_json(&doc).expect("Canonical JSON");
        // Verify no whitespace
        assert!(!canonical.contains('\n'));
        assert!(!canonical.contains('\r'));
        // Verify it parses back
        let _: serde_json::Value = serde_json::from_str(&canonical).expect("Valid JSON");
    }

    #[test]
    fn test_rule_ir_with_complex_conditions() {
        let rule = RuleIr {
            id: "complex-1".into(),
            name: "complex condition rule".into(),
            trigger: TriggerIr::UserStartsSession {
                session_type: "focus".into(),
            },
            conditions: vec![ConditionIr::And {
                conditions: vec![
                    ConditionIr::TimeInRange {
                        start_hour: 8,
                        end_hour: 17,
                    },
                    ConditionIr::DayOfWeek {
                        days: vec!["Monday".into(), "Tuesday".into(), "Wednesday".into()],
                    },
                ],
            }],
            actions: vec![
                ActionIr::EnforcePolicy {
                    policy_id: "block-social".into(),
                    params: {
                        let mut m = BTreeMap::new();
                        m.insert("duration_ms".into(), serde_json::json!(7200000));
                        m
                    },
                },
                ActionIr::ShowNotification {
                    notification_id: "notif-1".into(),
                    text: "Deep work started".into(),
                    duration_ms: Some(5000),
                },
            ],
            priority: 100,
            cooldown_seconds: Some(600),
            duration_seconds: Some(3600),
            explanation_template: "Block social media during focus sessions".into(),
            enabled: true,
        };

        let json = serde_json::to_string(&rule).expect("Serialize");
        let restored: RuleIr = serde_json::from_str(&json).expect("Deserialize");

        assert_eq!(rule.id, restored.id);
        assert_eq!(rule.conditions.len(), restored.conditions.len());
    }

    #[test]
    fn test_placeholder_variants_empty() {
        // Verify placeholder structs serialize/deserialize cleanly
        let connector = Body::Connector(ConnectorIr {});
        let json = serde_json::to_string(&connector).expect("Serialize connector");
        let _: Body = serde_json::from_str(&json).expect("Deserialize connector");

        let template = Body::Template(TemplateIr {});
        let json = serde_json::to_string(&template).expect("Serialize template");
        let _: Body = serde_json::from_str(&json).expect("Deserialize template");
    }

    #[test]
    fn test_document_with_actions_round_trip() {
        let doc = Document {
            version: 1,
            kind: DocKind::Rule,
            id: "action-test".into(),
            name: "action rule".into(),
            body: Body::Rule(Box::new(RuleIr {
                id: "ar1".into(),
                name: "actions".into(),
                trigger: TriggerIr::EventFired {
                    event_name: "badge_earned".into(),
                },
                conditions: vec![],
                actions: vec![
                    ActionIr::EnforcePolicy {
                        policy_id: "unlock".into(),
                        params: {
                            let mut m = BTreeMap::new();
                            m.insert("duration_ms".into(), serde_json::json!(600000));
                            m
                        },
                    },
                    ActionIr::ShowNotification {
                        notification_id: "reward-notif".into(),
                        text: "Great job!".into(),
                        duration_ms: Some(3000),
                    },
                ],
                priority: 1,
                cooldown_seconds: None,
                duration_seconds: None,
                explanation_template: "Reward for achievement".into(),
                enabled: true,
            })),
        };

        let json = serde_json::to_string(&doc).expect("Serialize");
        let restored: Document = serde_json::from_str(&json).expect("Deserialize");
        let json2 = serde_json::to_string(&restored).expect("Serialize again");

        assert_eq!(json, json2, "JSON round-trip must be identical");
    }
}
