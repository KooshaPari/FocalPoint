use serde::{Deserialize, Serialize};

use crate::rule::Rule;

/// A policy containing a set of rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Policy {
    pub name: String,
    pub description: Option<String>,
    pub rules: Vec<Rule>,
    pub enabled: bool,
}

impl Policy {
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
