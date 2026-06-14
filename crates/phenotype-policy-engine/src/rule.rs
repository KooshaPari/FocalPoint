use serde::{Deserialize, Serialize};

/// Type of a policy rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RuleType {
    Allow,
    Deny,
    Require,
    Validate,
}

/// Severity of a rule violation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// A single rule within a policy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rule {
    pub name: String,
    pub rule_type: RuleType,
    pub condition: String,
    pub action: String,
    pub severity: Severity,
    pub message: Option<String>,
}
