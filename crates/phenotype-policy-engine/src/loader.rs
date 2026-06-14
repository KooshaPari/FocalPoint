use phenotype_error_core::{PhenotypeError, Result};
use std::collections::HashMap;

use crate::policy::Policy;

/// Load policies from a TOML string.
pub fn load_policies(toml_str: &str) -> Result<HashMap<String, Policy>> {
    let policies: Vec<Policy> = toml::from_str(toml_str)
        .map_err(|e| PhenotypeError::config(format!("Failed to parse policies: {e}")))?;

    let mut map = HashMap::new();
    for policy in policies {
        map.insert(policy.name.clone(), policy);
    }
    Ok(map)
}
