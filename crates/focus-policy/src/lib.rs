//! Enforcement policy generation from rule decisions.
//!
//! Traces to FR-ENF-001.

use chrono::{DateTime, Duration, Utc};
use focus_audit::AuditSink;
use focus_rules::{Action, PrioritizedDecision, RuleDecision};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementPolicy {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub block_profile: BlockProfile,
    pub app_targets: Vec<AppTarget>,
    pub scheduled_windows: Vec<Window>,
    pub active: bool,
    /// Per-profile computed effective state (Block or Unblock).
    pub profile_states: HashMap<String, ProfileState>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockProfile {
    pub name: String,
    pub categories: Vec<String>,
    pub exceptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppTarget {
    Category(String),
    BundleId(String),
    PackageName(String),
    Domain(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileState {
    /// Profile is actively blocked; window describes duration.
    Blocked { ends_at: DateTime<Utc> },
    /// Profile explicitly unblocked by a higher-priority rule.
    Unblocked,
}

pub struct PolicyBuilder;

impl PolicyBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Build an `EnforcementPolicy` from an ordered list of prioritized rule
    /// decisions.
    ///
    /// Conflict rules (FR-ENF-001):
    /// * Within a single rule's action list, `Unblock{profile=X}` beats
    ///   `Block{profile=X}`.
    /// * Across rule decisions, the highest-priority rule wins. Callers pass
    ///   decisions in any order; we sort by `priority` descending, stable.
    pub fn from_rule_decisions(
        decisions: &[PrioritizedDecision],
        now: DateTime<Utc>,
        audit: &dyn AuditSink,
    ) -> EnforcementPolicy {
        // Sort a copy by descending priority, stable on input order.
        let mut sorted: Vec<&PrioritizedDecision> = decisions.iter().collect();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut profile_states: HashMap<String, ProfileState> = HashMap::new();
        let mut scheduled_windows: Vec<Window> = Vec::new();

        for pd in sorted {
            let actions = match &pd.decision {
                RuleDecision::Fired(a) => a,
                _ => continue,
            };
            // Within a single decision, Unblock beats Block for same profile.
            let mut local: HashMap<String, ProfileState> = HashMap::new();
            for action in actions {
                match action {
                    Action::Block { profile, duration } => {
                        local.entry(profile.clone()).or_insert_with(|| ProfileState::Blocked {
                            ends_at: now + clamp_duration(*duration),
                        });
                    }
                    Action::Unblock { profile } => {
                        // Force-overwrite within the same decision.
                        local.insert(profile.clone(), ProfileState::Unblocked);
                    }
                    _ => {}
                }
            }
            // Merge: only set profile state if not already set by a
            // higher-priority decision.
            for (profile, state) in local {
                profile_states.entry(profile).or_insert(state);
            }
            // Accumulate scheduled windows for any Block action (informational).
            for action in actions {
                if let Action::Block { duration, .. } = action {
                    scheduled_windows
                        .push(Window { starts_at: now, ends_at: now + clamp_duration(*duration) });
                }
            }
        }

        let any_blocked =
            profile_states.values().any(|s| matches!(s, ProfileState::Blocked { .. }));

        let policy = EnforcementPolicy {
            id: uuid::Uuid::new_v4(),
            user_id: uuid::Uuid::nil(),
            block_profile: BlockProfile::default(),
            app_targets: vec![],
            scheduled_windows,
            active: any_blocked,
            profile_states,
            generated_at: now,
        };

        // Best-effort audit: a failed append shouldn't corrupt policy
        // construction (the function is infallible by type). Serialize a
        // compact summary: decision ids + profile states + active flag.
        let decision_ids: Vec<String> =
            decisions.iter().map(|d| d.rule_id.to_string()).collect();
        let states_json: HashMap<String, serde_json::Value> = policy
            .profile_states
            .iter()
            .map(|(k, v)| {
                let payload = match v {
                    ProfileState::Blocked { ends_at } => json!({"state": "Blocked", "ends_at": ends_at}),
                    ProfileState::Unblocked => json!({"state": "Unblocked"}),
                };
                (k.clone(), payload)
            })
            .collect();
        let payload = json!({
            "policy_id": policy.id,
            "active": policy.active,
            "decision_ids": decision_ids,
            "profile_states": states_json,
        });
        // Swallow audit errors; construction is infallible. Callers that need
        // hard failure should wrap with their own sink impl that panics or
        // bubbles up via a thread-local.
        let _ = audit.record_mutation(
            "policy.built",
            &policy.id.to_string(),
            payload,
            now,
        );

        policy
    }
}

impl Default for PolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn clamp_duration(d: Duration) -> Duration {
    if d < Duration::zero() {
        Duration::zero()
    } else {
        d
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use focus_audit::{CapturingAuditSink, NoopAuditSink};
    use focus_rules::{Action, PrioritizedDecision, RuleDecision};
    use uuid::Uuid;

    fn t() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap()
    }

    fn fired(priority: i32, actions: Vec<Action>) -> PrioritizedDecision {
        PrioritizedDecision {
            rule_id: Uuid::new_v4(),
            priority,
            decision: RuleDecision::Fired(actions),
        }
    }

    // Traces to: FR-ENF-001
    #[test]
    fn block_produces_active_policy() {
        let d = fired(
            10,
            vec![Action::Block { profile: "games".into(), duration: Duration::minutes(30) }],
        );
        let p = PolicyBuilder::from_rule_decisions(&[d], t(), &NoopAuditSink);
        assert!(p.active);
        assert!(matches!(p.profile_states.get("games"), Some(ProfileState::Blocked { .. })));
    }

    // Traces to: FR-ENF-001
    #[test]
    fn unblock_within_same_decision_beats_block() {
        let d = fired(
            10,
            vec![
                Action::Block { profile: "games".into(), duration: Duration::minutes(30) },
                Action::Unblock { profile: "games".into() },
            ],
        );
        let p = PolicyBuilder::from_rule_decisions(&[d], t(), &NoopAuditSink);
        assert_eq!(p.profile_states.get("games"), Some(&ProfileState::Unblocked));
    }

    // Traces to: FR-ENF-001
    #[test]
    fn higher_priority_rule_wins_across_decisions() {
        let low = fired(1, vec![Action::Unblock { profile: "social".into() }]);
        let high = fired(
            100,
            vec![Action::Block { profile: "social".into(), duration: Duration::minutes(60) }],
        );
        // Input order intentionally low-first to prove sort.
        let p = PolicyBuilder::from_rule_decisions(&[low, high], t(), &NoopAuditSink);
        assert!(matches!(p.profile_states.get("social"), Some(ProfileState::Blocked { .. })));
    }

    // Traces to: FR-ENF-001
    #[test]
    fn no_fired_decisions_yields_inactive_policy() {
        let skipped = PrioritizedDecision {
            rule_id: Uuid::new_v4(),
            priority: 5,
            decision: RuleDecision::Skipped { reason: "x".into() },
        };
        let p = PolicyBuilder::from_rule_decisions(&[skipped], t(), &NoopAuditSink);
        assert!(!p.active);
        assert!(p.profile_states.is_empty());
    }

    // Traces to: FR-ENF-001
    #[test]
    fn multiple_profiles_are_independent() {
        let d = fired(
            10,
            vec![
                Action::Block { profile: "games".into(), duration: Duration::minutes(30) },
                Action::Unblock { profile: "education".into() },
            ],
        );
        let p = PolicyBuilder::from_rule_decisions(&[d], t(), &NoopAuditSink);
        assert!(matches!(p.profile_states.get("games"), Some(ProfileState::Blocked { .. })));
        assert_eq!(p.profile_states.get("education"), Some(&ProfileState::Unblocked));
    }

    // Traces to: FR-STATE-004
    #[test]
    fn policy_build_records_audit() {
        let d = fired(
            10,
            vec![Action::Block { profile: "games".into(), duration: Duration::minutes(30) }],
        );
        let sink = CapturingAuditSink::new();
        let p = PolicyBuilder::from_rule_decisions(&[d], t(), &sink);
        let snap = sink.snapshot();
        assert_eq!(snap.len(), 1);
        assert_eq!(snap[0].0, "policy.built");
        assert_eq!(snap[0].1, p.id.to_string());
        assert_eq!(snap[0].2["active"], true);
    }

    // Traces to: FR-STATE-004
    #[test]
    fn policy_audit_payload_includes_decision_ids() {
        let d1 = fired(10, vec![Action::Unblock { profile: "x".into() }]);
        let d2 = fired(5, vec![Action::Unblock { profile: "y".into() }]);
        let ids: Vec<String> = vec![d1.rule_id.to_string(), d2.rule_id.to_string()];
        let sink = CapturingAuditSink::new();
        let _ = PolicyBuilder::from_rule_decisions(&[d1, d2], t(), &sink);
        let snap = sink.snapshot();
        let decisions = snap[0].2["decision_ids"].as_array().expect("decision_ids array");
        let got: Vec<String> =
            decisions.iter().map(|v| v.as_str().unwrap().to_string()).collect();
        // Order in payload matches input order (we preserve input order).
        assert_eq!(got, ids);
    }

    // Traces to: FR-STATE-004
    #[test]
    fn policy_audit_payload_has_profile_states() {
        let d = fired(
            10,
            vec![
                Action::Block { profile: "games".into(), duration: Duration::minutes(30) },
                Action::Unblock { profile: "education".into() },
            ],
        );
        let sink = CapturingAuditSink::new();
        let _ = PolicyBuilder::from_rule_decisions(&[d], t(), &sink);
        let snap = sink.snapshot();
        let states = &snap[0].2["profile_states"];
        assert_eq!(states["education"]["state"], "Unblocked");
        assert_eq!(states["games"]["state"], "Blocked");
    }
}
