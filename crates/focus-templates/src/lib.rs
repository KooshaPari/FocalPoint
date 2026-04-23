//! Template-pack format: a distributable bundle of focus [`Rule`] drafts plus
//! recommended connectors and mascot-copy overrides. Serialized as TOML and
//! shipped as a `.fptpl` archive (`tar.gz` + detached ed25519 signature).
//!
//! Traces to: FR-TEMPLATE-PACK-001, FR-TEMPLATE-SIGN-001.
//!
//! # Lifecycle
//!
//! 1. Author writes TOML → [`TemplatePack::from_toml_str`].
//! 2. (Optional) operator signs the pack → [`signing::sign_pack`].
//! 3. App loads pack and calls [`TemplatePack::apply`] to install the rules
//!    through a [`RuleUpsert`] store.
//!
//! # Format stability
//!
//! The TOML surface is additive-only. Unknown fields are tolerated via
//! `#[serde(default)]`; removing a field is a breaking change that bumps
//! pack format version.

use chrono::Duration;
use focus_domain::Rigidity;
use focus_rules::{Action, Condition, Rule, Trigger};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub mod signing;

/// Error surface for template-pack operations.
#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("toml parse: {0}")]
    TomlParse(String),
    #[error("toml serialize: {0}")]
    TomlSerialize(String),
    #[error("apply: {0}")]
    Apply(String),
    #[error("signature: {0}")]
    Signature(String),
}

pub type Result<T> = std::result::Result<T, TemplateError>;

/// Target for [`TemplatePack::apply`]. Implementors persist every rule the
/// template pack carries; id collisions replace the existing rule.
pub trait RuleUpsert {
    fn upsert_rule(&mut self, rule: Rule) -> std::result::Result<(), String>;
}

/// A template pack — one TOML document distributed as `.fptpl`.
///
/// Traces to: FR-TEMPLATE-PACK-001.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplatePack {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    #[serde(default)]
    pub description: String,
    /// Rules the pack installs. Empty is allowed: a pack may exist solely to
    /// ship mascot copy or connector recommendations.
    #[serde(default)]
    pub rules: Vec<RuleDraft>,
    /// Connector ids the pack recommends the user enable (e.g.
    /// `"gcal"`, `"github"`, `"canvas"`). Informational — the host app
    /// surfaces these in the install UI.
    #[serde(default)]
    pub recommended_connectors: Vec<String>,
    /// Mascot copy overrides keyed by event id. Applied by the mascot layer
    /// if present; otherwise ignored.
    #[serde(default)]
    pub mascot_copy: std::collections::BTreeMap<String, String>,
}

/// A TOML-friendly projection of [`focus_rules::Rule`]. Trigger is split into
/// a kind + value pair so authors don't need to know serde's internal tagging.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleDraft {
    /// Stable string id — a template-scoped name like
    /// `"deep-work-social-block"`. We hash this into a deterministic UUID on
    /// [`TemplatePack::apply`] so re-applying the same pack upserts rather
    /// than duplicating.
    pub id: String,
    pub name: String,
    pub trigger: TriggerDraft,
    #[serde(default)]
    pub conditions: Vec<ConditionDraft>,
    pub actions: Vec<ActionDraft>,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub cooldown_seconds: Option<i64>,
    #[serde(default)]
    pub duration_seconds: Option<i64>,
    #[serde(default)]
    pub explanation_template: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum TriggerDraft {
    Event(String),
    Schedule(String),
    StateChange(String),
}

impl From<TriggerDraft> for Trigger {
    fn from(t: TriggerDraft) -> Self {
        match t {
            TriggerDraft::Event(e) => Trigger::Event(e),
            TriggerDraft::Schedule(s) => Trigger::Schedule(s),
            TriggerDraft::StateChange(k) => Trigger::StateChange(k),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionDraft {
    pub kind: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

impl From<ConditionDraft> for Condition {
    fn from(c: ConditionDraft) -> Self {
        Condition { kind: c.kind, params: c.params }
    }
}

/// Action variants available in template packs. Mirrors [`focus_rules::Action`]
/// but only exposes variants safe to ship in a pack — e.g. `EmergencyExit` is
/// deliberately omitted; it must be configured per-device, not installed
/// wholesale from a template.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionDraft {
    GrantCredit { amount: i32 },
    DeductCredit { amount: i32 },
    Block {
        profile: String,
        duration_seconds: i64,
        #[serde(default = "default_rigidity")]
        rigidity: RigidityDraft,
    },
    Unblock { profile: String },
    StreakIncrement { name: String },
    StreakReset { name: String },
    Notify { message: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RigidityDraft {
    #[default]
    Hard,
    Soft,
}

fn default_rigidity() -> RigidityDraft {
    RigidityDraft::Hard
}

impl From<RigidityDraft> for Rigidity {
    fn from(r: RigidityDraft) -> Self {
        match r {
            RigidityDraft::Hard => Rigidity::Hard,
            RigidityDraft::Soft => Rigidity::Soft,
        }
    }
}

impl From<ActionDraft> for Action {
    fn from(a: ActionDraft) -> Self {
        match a {
            ActionDraft::GrantCredit { amount } => Action::GrantCredit { amount },
            ActionDraft::DeductCredit { amount } => Action::DeductCredit { amount },
            ActionDraft::Block { profile, duration_seconds, rigidity } => Action::Block {
                profile,
                duration: Duration::seconds(duration_seconds),
                rigidity: rigidity.into(),
            },
            ActionDraft::Unblock { profile } => Action::Unblock { profile },
            ActionDraft::StreakIncrement { name } => Action::StreakIncrement(name),
            ActionDraft::StreakReset { name } => Action::StreakReset(name),
            ActionDraft::Notify { message } => Action::Notify(message),
        }
    }
}

impl RuleDraft {
    /// Materialize into a [`Rule`]. The UUID is derived deterministically from
    /// `(pack_id, rule.id)` so re-applying the same pack upserts the same row.
    pub fn into_rule(self, pack_id: &str) -> Rule {
        let rule_uuid = derive_uuid(pack_id, &self.id);
        Rule {
            id: rule_uuid,
            name: self.name,
            trigger: self.trigger.into(),
            conditions: self.conditions.into_iter().map(Into::into).collect(),
            actions: self.actions.into_iter().map(Into::into).collect(),
            priority: self.priority,
            cooldown: self.cooldown_seconds.map(Duration::seconds),
            duration: self.duration_seconds.map(Duration::seconds),
            explanation_template: self.explanation_template,
            enabled: self.enabled,
        }
    }
}

/// Derive a stable UUID from the pack+rule id pair. Produces a UUIDv4-style
/// layout (variant=RFC4122, version=4) from the SHA-256 of the seed so
/// re-applying the same pack upserts rather than duplicating. Not a true
/// v5 (we don't depend on uuid's `v5` feature) but the determinism
/// property is equivalent for our purposes.
fn derive_uuid(pack_id: &str, rule_id: &str) -> Uuid {
    // FNV-1a 128-bit is plenty for a 16-byte deterministic id. Keeping the
    // hash stdlib-only avoids pulling sha2 just for id derivation.
    let seed = format!("focalpoint:template:{pack_id}:{rule_id}");
    let mut h1: u64 = 0xcbf2_9ce4_8422_2325;
    let mut h2: u64 = 0x1000_0000_01b3_1000;
    for b in seed.as_bytes() {
        h1 ^= *b as u64;
        h1 = h1.wrapping_mul(0x0000_0100_0000_01b3);
        h2 ^= (*b as u64).rotate_left(13);
        h2 = h2.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&h1.to_be_bytes());
    bytes[8..].copy_from_slice(&h2.to_be_bytes());
    // Stamp version=4, variant=RFC4122 so downstream UUID validators accept it.
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

impl TemplatePack {
    /// Parse a TOML document into a [`TemplatePack`].
    pub fn from_toml_str(input: &str) -> Result<Self> {
        toml::from_str::<Self>(input).map_err(|e| TemplateError::TomlParse(e.to_string()))
    }

    /// Serialize a pack as TOML.
    pub fn to_toml_string(&self) -> Result<String> {
        toml::to_string_pretty(self).map_err(|e| TemplateError::TomlSerialize(e.to_string()))
    }

    /// Wire every rule through `store.upsert_rule`. Errors surface the first
    /// store failure; remaining rules are not applied. Empty rule sets are
    /// allowed — the call returns `Ok(0)` in that case.
    pub fn apply(&self, store: &mut dyn RuleUpsert) -> Result<usize> {
        let mut n = 0;
        for draft in self.rules.iter().cloned() {
            let rule = draft.into_rule(&self.id);
            store
                .upsert_rule(rule)
                .map_err(|e| TemplateError::Apply(format!("rule #{n}: {e}")))?;
            n += 1;
        }
        Ok(n)
    }
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TOML: &str = r#"
id = "deep-work-starter"
name = "Deep Work Starter"
version = "0.1.0"
author = "focalpoint-team"
description = "A tiny starter pack."
recommended_connectors = ["gcal", "github"]

[mascot_copy]
session_start = "Let's go."

[[rules]]
id = "deep-work-social-block"
name = "Deep work — no social"
priority = 80
cooldown_seconds = 600
duration_seconds = 3000
explanation_template = "Social apps locked while {rule_name} is active."
enabled = true
trigger = { kind = "event", value = "focus:session_started" }
actions = [
  { type = "block", profile = "social", duration_seconds = 3000, rigidity = "hard" },
]
"#;

    #[derive(Default)]
    struct MemStore {
        rules: Vec<Rule>,
        fail_at: Option<usize>,
    }

    impl RuleUpsert for MemStore {
        fn upsert_rule(&mut self, rule: Rule) -> std::result::Result<(), String> {
            if let Some(at) = self.fail_at {
                if self.rules.len() == at {
                    return Err("boom".into());
                }
            }
            if let Some(slot) = self.rules.iter_mut().find(|r| r.id == rule.id) {
                *slot = rule;
            } else {
                self.rules.push(rule);
            }
            Ok(())
        }
    }

    #[test]
    fn toml_roundtrip_preserves_fields() {
        let pack = TemplatePack::from_toml_str(SAMPLE_TOML).expect("parse");
        assert_eq!(pack.id, "deep-work-starter");
        assert_eq!(pack.rules.len(), 1);
        assert_eq!(pack.recommended_connectors, vec!["gcal", "github"]);
        let s = pack.to_toml_string().expect("serialize");
        let back = TemplatePack::from_toml_str(&s).expect("parse back");
        assert_eq!(pack, back);
    }

    #[test]
    fn apply_populates_store_and_is_idempotent() {
        let pack = TemplatePack::from_toml_str(SAMPLE_TOML).expect("parse");
        let mut store = MemStore::default();
        let n = pack.apply(&mut store).expect("apply");
        assert_eq!(n, 1);
        assert_eq!(store.rules.len(), 1);
        assert_eq!(store.rules[0].name, "Deep work — no social");
        // Re-apply → upsert, not dup.
        let _ = pack.apply(&mut store).expect("apply 2");
        assert_eq!(store.rules.len(), 1);
    }

    #[test]
    fn apply_propagates_store_error() {
        let pack = TemplatePack::from_toml_str(SAMPLE_TOML).expect("parse");
        let mut store = MemStore { fail_at: Some(0), ..Default::default() };
        let err = pack.apply(&mut store).unwrap_err();
        match err {
            TemplateError::Apply(msg) => assert!(msg.contains("boom")),
            o => panic!("unexpected: {o:?}"),
        }
    }

    #[test]
    fn empty_rules_pack_is_allowed() {
        let toml_text = r#"
id = "copy-only"
name = "Mascot Copy Only"
version = "0.0.1"
author = "x"
"#;
        let pack = TemplatePack::from_toml_str(toml_text).expect("parse");
        assert!(pack.rules.is_empty());
        let mut store = MemStore::default();
        assert_eq!(pack.apply(&mut store).unwrap(), 0);
    }

    #[test]
    fn malformed_toml_surfaces_clear_error() {
        let bad = "this is not = = = toml";
        let err = TemplatePack::from_toml_str(bad).unwrap_err();
        assert!(matches!(err, TemplateError::TomlParse(_)));
    }

    #[test]
    fn derived_uuid_is_stable_across_calls() {
        let a = derive_uuid("p", "r");
        let b = derive_uuid("p", "r");
        assert_eq!(a, b);
        assert_ne!(a, derive_uuid("p", "r2"));
    }
}
