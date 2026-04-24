//! Sandboxed Starlark environment for FPL evaluation.
//!
//! Provides FPL builtins (rule, triggers, conditions, actions) without
//! filesystem/network/sys access.

use crate::{
    register_rule, ActionData, ConditionData, RuleData, TriggerData,
};
use serde_json::Value;
use starlark::environment::Globals;
use starlark::values::Value as StarValue;

/// Create the sandboxed Starlark environment for FPL.
///
/// Provides these globals:
/// - `rule(id, name, trigger, conditions, actions, priority, cooldown_seconds, duration_seconds, explanation_template, enabled)`
/// - `on_event(name)`, `on_schedule(cron, timezone)`, `on_state_change(path)`
/// - Condition helpers: `confidence_gte`, `payload_eq`, `payload_in`, `payload_exists`, `payload_matches`, `source_eq`, `occurred_within`, `all_of`, `any_of`, `not_`
/// - Action helpers: `grant_credit`, `deduct_credit`, `block`, `unblock`, `streak_increment`, `streak_reset`, `notify`
pub fn create_fpl_globals() -> Globals {
    // Use the standard Starlark globals as base.
    // We'll inject custom builtins via starlark's native_function mechanism.
    let globals = Globals::new();

    // For the first implementation, we keep globals minimal.
    // Actual builtins are injected via Module::add methods at evaluation time.
    globals
}
