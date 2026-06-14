use serde_json::Value;
use std::collections::HashMap;

/// Evaluation context containing facts for policy evaluation.
#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    pub facts: HashMap<String, Value>,
}

impl EvaluationContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fact(mut self, key: impl Into<String>, value: Value) -> Self {
        self.facts.insert(key.into(), value);
        self
    }

    pub fn get_fact(&self, key: &str) -> Option<&Value> {
        self.facts.get(key)
    }
}
