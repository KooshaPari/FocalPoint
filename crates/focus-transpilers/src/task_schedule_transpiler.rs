//! Task and Schedule transpilers: round-trip converters for planning primitives.
//!
//! Traces to: FR-PLAN-001 (Task model), FR-SCHEDULE-001 (Schedule temporal trigger).
//!
//! Handles conversions:
//! - focus_planning::Task ↔ TaskIr (via focus-ir)
//! - ScheduleIr → cron string (display)
//! - Graph JSON nodes (TaskNode, ScheduleNode) ↔ IR

use anyhow::{anyhow, Result};
use focus_ir::{ConstraintIr, DurationSpecIr, EstimateIr, ScheduleIr, TaskIr, TaskStatusIr};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use uuid::Uuid;

/// Convert a focus_planning::Task to IR and encode as JSON for storage/transport.
pub fn task_to_json(
    task: &focus_planning::Task,
    user_id: &str,
) -> Result<Value> {
    // Build TaskIr
    let ir = task_to_ir(task, user_id);

    // Serialize to JSON value
    let json = serde_json::to_value(&ir)?;
    Ok(json)
}

/// Convert JSON back to focus_planning::Task.
pub fn json_to_task(json: &Value, _user_id: &str) -> Result<focus_planning::Task> {
    let ir: TaskIr = serde_json::from_value(json.clone())?;
    ir_to_task(&ir)
}

/// Build a ScheduleIr from cron and description.
pub fn cron_to_schedule(cron_spec: &str, description: &str, rule_ids: Vec<String>) -> ScheduleIr {
    ScheduleIr {
        id: Uuid::new_v4().to_string(),
        cron_spec: cron_spec.to_string(),
        enabled: true,
        description: description.to_string(),
        attached_rule_ids: rule_ids,
    }
}

/// Extract cron spec from ScheduleIr for external use.
pub fn schedule_to_cron(schedule: &ScheduleIr) -> &str {
    &schedule.cron_spec
}

// ============================================================================
// Graph JSON Nodes (ReactFlow format)
// ============================================================================

/// TaskNode type for ReactFlow canvas.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskNode {
    pub id: String,
    pub position: (f64, f64),
    pub data: TaskNodeData,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskNodeData {
    pub title: String,
    pub duration_minutes: i64,
    pub priority: f32,
    pub status: String,
}

/// ScheduleNode type for ReactFlow canvas.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduleNode {
    pub id: String,
    pub position: (f64, f64),
    pub data: ScheduleNodeData,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduleNodeData {
    pub cron_spec: String,
    pub description: String,
}

/// Convert TaskIr to a ReactFlow node.
pub fn task_ir_to_node(ir: &TaskIr, x: f64, y: f64) -> TaskNode {
    let duration = ir
        .duration_spec
        .fixed_minutes
        .or_else(|| ir.duration_spec.estimate.as_ref().map(|e| e.p90_minutes))
        .unwrap_or(0);

    TaskNode {
        id: ir.id.clone(),
        position: (x, y),
        data: TaskNodeData {
            title: ir.title.clone(),
            duration_minutes: duration,
            priority: ir.priority_weight,
            status: format!("{:?}", ir.status),
        },
    }
}

/// Convert ScheduleIr to a ReactFlow node.
pub fn schedule_ir_to_node(ir: &ScheduleIr, x: f64, y: f64) -> ScheduleNode {
    ScheduleNode {
        id: ir.id.clone(),
        position: (x, y),
        data: ScheduleNodeData {
            cron_spec: ir.cron_spec.clone(),
            description: ir.description.clone(),
        },
    }
}

/// Convert a ReactFlow TaskNode back to TaskIr.
pub fn node_to_task_ir(node: &TaskNode, user_id: &str) -> TaskIr {
    TaskIr {
        id: node.id.clone(),
        user_id: user_id.to_string(),
        title: node.data.title.clone(),
        duration_spec: DurationSpecIr {
            fixed_minutes: Some(node.data.duration_minutes),
            estimate: None,
        },
        priority_weight: node.data.priority,
        deadline: None,
        chunking: default_chunking(),
        constraints: vec![],
        status: TaskStatusIr::Pending,
    }
}

/// Convert a ReactFlow ScheduleNode back to ScheduleIr.
pub fn node_to_schedule_ir(node: &ScheduleNode) -> ScheduleIr {
    ScheduleIr {
        id: node.id.clone(),
        cron_spec: node.data.cron_spec.clone(),
        enabled: true,
        description: node.data.description.clone(),
        attached_rule_ids: vec![],
    }
}

// ============================================================================
// Internal Helpers
// ============================================================================

fn task_to_ir(task: &focus_planning::Task, user_id: &str) -> TaskIr {
    TaskIr {
        id: task.id.to_string(),
        user_id: user_id.to_string(),
        title: task.title.clone(),
        duration_spec: duration_spec_to_ir(&task.duration),
        priority_weight: task.priority.weight,
        deadline: None, // Simplified for now
        chunking: chunking_to_ir(&task.chunking),
        constraints: task.constraints.iter().map(constraint_to_ir).collect(),
        status: status_to_ir(&task.status),
    }
}

fn ir_to_task(ir: &TaskIr) -> Result<focus_planning::Task> {
    Ok(focus_planning::Task {
        id: Uuid::parse_str(&ir.id)
            .map_err(|_| anyhow!("Invalid task ID UUID"))?,
        title: ir.title.clone(),
        duration: ir_to_duration_spec(&ir.duration_spec)?,
        priority: focus_planning::Priority::clamped(ir.priority_weight),
        deadline: focus_planning::Deadline::none(), // Simplified for now
        chunking: ir_to_chunking(&ir.chunking)?,
        constraints: ir
            .constraints
            .iter()
            .map(ir_to_constraint)
            .collect::<Result<_, _>>()?,
        status: ir_to_status(&ir.status)?,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}

fn duration_spec_to_ir(ds: &focus_planning::DurationSpec) -> DurationSpecIr {
    DurationSpecIr {
        fixed_minutes: ds.fixed.map(|d| d.num_minutes()),
        estimate: ds.estimate.as_ref().map(|e| EstimateIr {
            p50_minutes: e.p50.num_minutes(),
            p90_minutes: e.p90.num_minutes(),
        }),
    }
}

fn ir_to_duration_spec(ir: &DurationSpecIr) -> Result<focus_planning::DurationSpec> {
    Ok(focus_planning::DurationSpec {
        fixed: ir.fixed_minutes.map(chrono::Duration::minutes),
        estimate: ir.estimate.as_ref().map(|e| focus_planning::Estimate {
            p50: chrono::Duration::minutes(e.p50_minutes),
            p90: chrono::Duration::minutes(e.p90_minutes),
        }),
    })
}

fn chunking_to_ir(cp: &focus_planning::ChunkingPolicy) -> focus_ir::ChunkingPolicyIr {
    focus_ir::ChunkingPolicyIr {
        allow_split: cp.allow_split,
        min_chunk_minutes: cp.min_chunk.num_minutes(),
        max_chunk_minutes: cp.max_chunk.num_minutes(),
        ideal_chunk_minutes: cp.ideal_chunk.num_minutes(),
    }
}

fn ir_to_chunking(ir: &focus_ir::ChunkingPolicyIr) -> Result<focus_planning::ChunkingPolicy> {
    Ok(focus_planning::ChunkingPolicy {
        allow_split: ir.allow_split,
        min_chunk: chrono::Duration::minutes(ir.min_chunk_minutes),
        max_chunk: chrono::Duration::minutes(ir.max_chunk_minutes),
        ideal_chunk: chrono::Duration::minutes(ir.ideal_chunk_minutes),
    })
}

fn constraint_to_ir(c: &focus_planning::Constraint) -> ConstraintIr {
    match c {
        focus_planning::Constraint::WorkingHours { start, end, days } => {
            ConstraintIr::WorkingHours {
                start_hour: start.hour(),
                end_hour: end.hour(),
                days: days.iter().map(|d| format!("{:?}", d)).collect(),
            }
        }
        focus_planning::Constraint::NoEarlierThan(dt) => ConstraintIr::NoEarlierThan {
            when_iso8601: dt.to_rfc3339(),
        },
        focus_planning::Constraint::NoLaterThan(dt) => ConstraintIr::NoLaterThan {
            when_iso8601: dt.to_rfc3339(),
        },
        focus_planning::Constraint::Buffer(d) => ConstraintIr::Buffer {
            duration_minutes: d.num_minutes(),
        },
        focus_planning::Constraint::EnergyTier(et) => ConstraintIr::EnergyTier {
            tier: format!("{:?}", et),
        },
    }
}

fn ir_to_constraint(ir: &ConstraintIr) -> Result<focus_planning::Constraint> {
    match ir {
        ConstraintIr::WorkingHours {
            start_hour,
            end_hour,
            days,
        } => {
            let start = chrono::NaiveTime::from_hms_opt(*start_hour, 0, 0)
                .ok_or_else(|| anyhow!("Invalid start hour"))?;
            let end = chrono::NaiveTime::from_hms_opt(*end_hour, 0, 0)
                .ok_or_else(|| anyhow!("Invalid end hour"))?;
            let days_parsed = days
                .iter()
                .filter_map(|s| match s.as_str() {
                    "Mon" => Some(chrono::Weekday::Mon),
                    "Tue" => Some(chrono::Weekday::Tue),
                    "Wed" => Some(chrono::Weekday::Wed),
                    "Thu" => Some(chrono::Weekday::Thu),
                    "Fri" => Some(chrono::Weekday::Fri),
                    "Sat" => Some(chrono::Weekday::Sat),
                    "Sun" => Some(chrono::Weekday::Sun),
                    _ => None,
                })
                .collect();
            Ok(focus_planning::Constraint::WorkingHours {
                start,
                end,
                days: days_parsed,
            })
        }
        ConstraintIr::NoEarlierThan { when_iso8601 } => {
            let dt = chrono::DateTime::parse_from_rfc3339(when_iso8601)
                .map_err(|_| anyhow!("Invalid ISO8601 datetime"))?
                .with_timezone(&chrono::Utc);
            Ok(focus_planning::Constraint::NoEarlierThan(dt))
        }
        ConstraintIr::NoLaterThan { when_iso8601 } => {
            let dt = chrono::DateTime::parse_from_rfc3339(when_iso8601)
                .map_err(|_| anyhow!("Invalid ISO8601 datetime"))?
                .with_timezone(&chrono::Utc);
            Ok(focus_planning::Constraint::NoLaterThan(dt))
        }
        ConstraintIr::Buffer { duration_minutes } => {
            Ok(focus_planning::Constraint::Buffer(
                chrono::Duration::minutes(*duration_minutes),
            ))
        }
        ConstraintIr::EnergyTier { tier } => {
            let energy = match tier.as_str() {
                "DeepFocus" => focus_planning::EnergyTier::DeepFocus,
                "Light" => focus_planning::EnergyTier::Light,
                "Admin" => focus_planning::EnergyTier::Admin,
                _ => focus_planning::EnergyTier::Light,
            };
            Ok(focus_planning::Constraint::EnergyTier(energy))
        }
    }
}

fn status_to_ir(s: &focus_planning::TaskStatus) -> TaskStatusIr {
    match s {
        focus_planning::TaskStatus::Pending => TaskStatusIr::Pending,
        focus_planning::TaskStatus::Scheduled { chunks } => TaskStatusIr::Scheduled {
            chunks: chunks
                .iter()
                .map(|tb| focus_ir::TimeBlockIr {
                    task_id: tb.task_id.to_string(),
                    starts_at_iso8601: tb.starts_at.to_rfc3339(),
                    ends_at_iso8601: tb.ends_at.to_rfc3339(),
                    rigidity: format!("{:?}", tb.rigidity),
                })
                .collect(),
        },
        focus_planning::TaskStatus::InProgress => TaskStatusIr::InProgress,
        focus_planning::TaskStatus::Completed => TaskStatusIr::Completed,
        focus_planning::TaskStatus::Cancelled => TaskStatusIr::Cancelled,
    }
}

fn ir_to_status(ir: &TaskStatusIr) -> Result<focus_planning::TaskStatus> {
    match ir {
        TaskStatusIr::Pending => Ok(focus_planning::TaskStatus::Pending),
        TaskStatusIr::Scheduled { chunks } => {
            let parsed = chunks
                .iter()
                .map(|tb| {
                    let task_id = Uuid::parse_str(&tb.task_id)
                        .map_err(|_| anyhow!("Invalid task ID in chunk"))?;
                    let starts_at = chrono::DateTime::parse_from_rfc3339(&tb.starts_at_iso8601)
                        .map_err(|_| anyhow!("Invalid start timestamp"))?
                        .with_timezone(&chrono::Utc);
                    let ends_at = chrono::DateTime::parse_from_rfc3339(&tb.ends_at_iso8601)
                        .map_err(|_| anyhow!("Invalid end timestamp"))?
                        .with_timezone(&chrono::Utc);
                    let rigidity = match tb.rigidity.as_str() {
                        "Hard" => focus_domain::Rigidity::Hard,
                        _ => focus_domain::Rigidity::Soft,
                    };
                    Ok(focus_planning::TimeBlock {
                        task_id,
                        starts_at,
                        ends_at,
                        rigidity,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(focus_planning::TaskStatus::Scheduled { chunks: parsed })
        }
        TaskStatusIr::InProgress => Ok(focus_planning::TaskStatus::InProgress),
        TaskStatusIr::Completed => Ok(focus_planning::TaskStatus::Completed),
        TaskStatusIr::Cancelled => Ok(focus_planning::TaskStatus::Cancelled),
    }
}

fn default_chunking() -> focus_ir::ChunkingPolicyIr {
    focus_ir::ChunkingPolicyIr {
        allow_split: true,
        min_chunk_minutes: 25,
        max_chunk_minutes: 120,
        ideal_chunk_minutes: 50,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_schedule_to_json_round_trip() {
        let schedule = ScheduleIr {
            id: "sched-test".to_string(),
            cron_spec: "0 9 * * 1-5".to_string(),
            enabled: true,
            description: "Weekday standup".to_string(),
            attached_rule_ids: vec!["rule-1".to_string()],
        };

        let json = serde_json::to_value(&schedule).expect("Serialize");
        let back: ScheduleIr = serde_json::from_value(json).expect("Deserialize");

        assert_eq!(back.id, schedule.id);
        assert_eq!(back.cron_spec, schedule.cron_spec);
    }

    #[test]
    fn test_task_node_round_trip() {
        let ir = TaskIr {
            id: "task-123".to_string(),
            user_id: "user-1".to_string(),
            title: "Test task".to_string(),
            duration_spec: DurationSpecIr {
                fixed_minutes: Some(60),
                estimate: None,
            },
            priority_weight: 0.7,
            deadline: None,
            chunking: default_chunking(),
            constraints: vec![],
            status: TaskStatusIr::Pending,
        };

        let node = task_ir_to_node(&ir, 100.0, 200.0);
        let back = node_to_task_ir(&node, "user-1");

        assert_eq!(back.id, ir.id);
        assert_eq!(back.title, ir.title);
        assert_eq!(back.priority_weight, ir.priority_weight);
    }

    #[test]
    fn test_schedule_node_round_trip() {
        let ir = ScheduleIr {
            id: "sched-456".to_string(),
            cron_spec: "*/15 * * * *".to_string(),
            enabled: true,
            description: "Frequent".to_string(),
            attached_rule_ids: vec![],
        };

        let node = schedule_ir_to_node(&ir, 50.0, 150.0);
        let back = node_to_schedule_ir(&node);

        assert_eq!(back.id, ir.id);
        assert_eq!(back.cron_spec, ir.cron_spec);
    }

    #[test]
    fn test_cron_to_schedule_builder() {
        let sched = cron_to_schedule("0 9 * * 1-5", "Morning standup", vec![]);
        assert_eq!(sched.cron_spec, "0 9 * * 1-5");
        assert_eq!(sched.description, "Morning standup");
        assert!(sched.enabled);
    }

    #[test]
    fn test_schedule_to_cron_extractor() {
        let sched = ScheduleIr {
            id: "test".to_string(),
            cron_spec: "0 17 * * *".to_string(),
            enabled: true,
            description: "EOD".to_string(),
            attached_rule_ids: vec![],
        };
        assert_eq!(schedule_to_cron(&sched), "0 17 * * *");
    }
}
