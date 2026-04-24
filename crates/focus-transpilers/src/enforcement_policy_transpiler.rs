//! Transpiler: focus_policy::EnforcementPolicy ↔ focus_ir::EnforcementPolicyIr

use anyhow::{anyhow, Result};
use focus_ir::{
    ActionIr, Body, ConditionIr, Document, DocKind, EnforcementPolicyIr, ThresholdIr,
};
use focus_policy::EnforcementPolicy;

/// Convert focus_policy::EnforcementPolicy to focus_ir::EnforcementPolicyIr.
pub fn policy_to_ir(policy: &EnforcementPolicy) -> EnforcementPolicyIr {
    EnforcementPolicyIr {
        id: policy.id.to_string(),
        name: policy.block_profile.name.clone(),
        description: None, // EnforcementPolicy doesn't have description field
        targets: policy
            .app_targets
            .iter()
            .map(|t| format!("{:?}", t))
            .collect(),
        threshold: None, // EnforcementPolicy doesn't have explicit threshold field
        action_on_violation: ActionIr::EnforcePolicy {
            policy_id: policy.id.to_string(),
            params: serde_json::json!(policy.profile_states).as_object().cloned().unwrap_or_default(),
        },
        grace_period_ms: None, // EnforcementPolicy doesn't have grace period field
    }
}

/// Convert focus_ir::EnforcementPolicyIr back to a minimal EnforcementPolicy representation.
/// Note: Some fields cannot be reconstructed without additional context, so we return
/// a partial policy that captures the essential information.
pub fn ir_to_policy_minimal(ir: &EnforcementPolicyIr) -> Result<EnforcementPolicy> {
    use chrono::Utc;
    use focus_policy::BlockProfile;
    use uuid::Uuid;

    let block_profile = BlockProfile {
        name: ir.name.clone(),
        categories: vec![],
        exceptions: vec![],
    };

    Ok(EnforcementPolicy {
        id: Uuid::parse_str(&ir.id).unwrap_or_else(|_| Uuid::new_v4()),
        user_id: Uuid::new_v4(), // Cannot reconstruct; use default
        block_profile,
        app_targets: vec![],
        scheduled_windows: vec![],
        active: true,
        profile_states: std::collections::HashMap::new(),
        generated_at: Utc::now(),
    })
}

/// Convert EnforcementPolicy to an IR Document.
pub fn policy_to_document(policy: &EnforcementPolicy) -> Document {
    let ir = policy_to_ir(policy);
    Document {
        version: 1,
        kind: DocKind::EnforcementPolicy,
        id: policy.id.to_string(),
        name: policy.block_profile.name.clone(),
        body: Body::EnforcementPolicy(ir),
    }
}

/// Convert an IR Document back to a minimal EnforcementPolicy.
pub fn document_to_policy_minimal(doc: &Document) -> Result<EnforcementPolicy> {
    match &doc.body {
        Body::EnforcementPolicy(ir) => ir_to_policy_minimal(ir),
        _ => Err(anyhow!("Expected EnforcementPolicy body, got {:?}", doc.kind)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_to_ir_round_trip() {
        use chrono::Utc;
        use focus_policy::BlockProfile;
        use uuid::Uuid;

        let policy = EnforcementPolicy {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            block_profile: BlockProfile {
                name: "social-media".into(),
                categories: vec!["social".into()],
                exceptions: vec![],
            },
            app_targets: vec![],
            scheduled_windows: vec![],
            active: true,
            profile_states: std::collections::HashMap::new(),
            generated_at: Utc::now(),
        };

        let ir = policy_to_ir(&policy);
        let restored = ir_to_policy_minimal(&ir).expect("Round-trip");

        assert_eq!(policy.id, restored.id);
        assert_eq!(policy.block_profile.name, restored.block_profile.name);
    }

    #[test]
    fn test_document_round_trip() {
        use chrono::Utc;
        use focus_policy::BlockProfile;
        use uuid::Uuid;

        let policy = EnforcementPolicy {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            block_profile: BlockProfile {
                name: "test-policy".into(),
                categories: vec![],
                exceptions: vec![],
            },
            app_targets: vec![],
            scheduled_windows: vec![],
            active: true,
            profile_states: std::collections::HashMap::new(),
            generated_at: Utc::now(),
        };

        let doc = policy_to_document(&policy);
        let restored = document_to_policy_minimal(&doc).expect("Document round-trip");

        assert_eq!(policy.id, restored.id);
        assert_eq!(policy.block_profile.name, restored.block_profile.name);
    }
}
