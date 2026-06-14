use phenotype_error_core::{PhenotypeError, Result};
use std::collections::HashMap;
use std::sync::Arc;

use crate::context::EvaluationContext;
use crate::policy::Policy;
use crate::result::PolicyResult;
use crate::rule::RuleType;

/// Policy evaluation engine.
#[derive(Debug, Clone)]
pub struct PolicyEngine {
    policies: Arc<HashMap<String, Policy>>,
}

impl PolicyEngine {
    pub fn new(policies: HashMap<String, Policy>) -> Self {
        Self {
            policies: Arc::new(policies),
        }
    }

    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }

    pub fn evaluate(&self, policy_name: &str, context: &EvaluationContext) -> Result<PolicyResult> {
        let policy = self
            .policies
            .get(policy_name)
            .ok_or_else(|| PhenotypeError::policy(format!("Policy '{}' not found", policy_name)))?;

        if !policy.enabled {
            return Ok(PolicyResult::Indeterminate {
                reason: format!("Policy '{}' is disabled", policy_name),
            });
        }

        for rule in &policy.rules {
            match rule.rule_type {
                RuleType::Deny => {
                    if self.evaluate_condition(&rule.condition, context) {
                        return Ok(PolicyResult::Deny {
                            reason: rule.message.clone().unwrap_or_else(|| rule.name.clone()),
                        });
                    }
                }
                RuleType::Require => {
                    if !self.evaluate_condition(&rule.condition, context) {
                        return Ok(PolicyResult::Deny {
                            reason: rule.message.clone().unwrap_or_else(|| rule.name.clone()),
                        });
                    }
                }
                RuleType::Allow => {
                    if self.evaluate_condition(&rule.condition, context) {
                        return Ok(PolicyResult::Allow);
                    }
                }
                RuleType::Validate => {
                    if !self.evaluate_condition(&rule.condition, context) {
                        return Ok(PolicyResult::Deny {
                            reason: rule.message.clone().unwrap_or_else(|| rule.name.clone()),
                        });
                    }
                }
            }
        }

        Ok(PolicyResult::Allow)
    }

    pub fn evaluate_all(
        &self,
        context: &EvaluationContext,
    ) -> Result<HashMap<String, PolicyResult>> {
        let mut results = HashMap::new();
        for (name, policy) in self.policies.iter() {
            if policy.enabled {
                results.insert(name.clone(), self.evaluate(name, context)?);
            }
        }
        Ok(results)
    }

    fn evaluate_condition(&self, condition: &str, context: &EvaluationContext) -> bool {
        // Placeholder: in production, use a proper expression engine (CEL, JSONata, etc.)
        // For now, simple string matching against facts.
        if let Some(fact) = context.get_fact("authenticated") {
            if condition.contains("authenticated == true") {
                return fact.as_bool().unwrap_or(false);
            }
        }
        if let Some(fact) = context.get_fact("requests") {
            if condition.contains("requests <") {
                let threshold = condition
                    .split("<")
                    .nth(1)
                    .and_then(|s| s.trim().parse::<i64>().ok())
                    .unwrap_or(100);
                return fact.as_i64().unwrap_or(0) < threshold;
            }
        }
        // Default to true for unhandled conditions.
        true
    }
}
