use serde::{Deserialize, Serialize};

/// Result of a policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyResult {
    Allow,
    Deny { reason: String },
    Indeterminate { reason: String },
}

impl PolicyResult {
    pub fn is_allowed(&self) -> bool {
        matches!(self, PolicyResult::Allow)
    }

    pub fn is_denied(&self) -> bool {
        matches!(self, PolicyResult::Deny { .. })
    }
}
