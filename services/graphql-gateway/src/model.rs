//! GraphQL types and resolvers.
//!
//! Queries:
//!   - tasks(status: TaskStatus) -> [Task]
//!   - rules(enabled: Boolean) -> [Rule]
//!   - wallet() -> WalletSnapshot
//!   - audit(since: DateTime, limit: Int) -> [AuditRecord]
//!   - connectors() -> [Connector]
//!   - focusSessions(since: DateTime) -> [FocusSession]
//!
//! Mutations:
//!   - markTaskDone(id: UUID) -> Task
//!   - enableRule(id: UUID) -> Rule
//!   - triggerSync(connectorId: UUID) -> SyncResult

use async_graphql::{Object, SimpleObject};
use chrono::{DateTime, Utc};
use focus_domain::Rigidity;
use serde::{Deserialize, Serialize};

// ============================================================================
// Types
// ============================================================================

/// Task status enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, async_graphql::Enum)]
pub enum TaskStatus {
    /// Task is active.
    Active,
    /// Task is completed.
    Done,
    /// Task is archived.
    Archived,
}

/// Rigidity level for a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, async_graphql::Enum)]
pub enum RigidityLevel {
    /// Hard constraint; cannot be bypassed.
    Hard,
    /// Semi-rigid; can be bypassed by paying a cost.
    Semi,
    /// Soft constraint; purely advisory.
    Soft,
}

impl From<Rigidity> for RigidityLevel {
    fn from(r: Rigidity) -> Self {
        match r {
            Rigidity::Hard => RigidityLevel::Hard,
            Rigidity::Semi(_) => RigidityLevel::Semi,
            Rigidity::Soft => RigidityLevel::Soft,
        }
    }
}

/// A task representing user work.
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Task {
    /// Unique task ID.
    pub id: String,
    /// Task title.
    pub title: String,
    /// Task status.
    pub status: TaskStatus,
    /// When the task was created.
    pub created_at: DateTime<Utc>,
    /// When the task is due.
    pub due_at: Option<DateTime<Utc>>,
    /// Task description.
    pub description: Option<String>,
}

/// A behavior rule.
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Rule {
    /// Unique rule ID.
    pub id: String,
    /// Rule name.
    pub name: String,
    /// Is this rule currently enabled?
    pub enabled: bool,
    /// How rigid this enforcement is.
    pub rigidity: String, // "Hard", "Semi", or "Soft"
    /// When the rule was created.
    pub created_at: DateTime<Utc>,
    /// When the rule was last modified.
    pub updated_at: DateTime<Utc>,
}

/// A connector integration.
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Connector {
    /// Unique connector ID.
    pub id: String,
    /// Connector type (e.g., "github", "gcal", "canvas").
    pub connector_type: String,
    /// Is the connector currently connected?
    pub is_connected: bool,
    /// Last time data was synced from this connector.
    pub last_synced_at: Option<DateTime<Utc>>,
}

/// Wallet snapshot: aggregate credit/penalty state.
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct WalletSnapshot {
    /// Current credit balance.
    pub credits: i64,
    /// Total penalties applied.
    pub total_penalties: i64,
    /// Total rewards earned.
    pub total_rewards: i64,
    /// Snapshot timestamp.
    pub snapshot_at: DateTime<Utc>,
}

/// Audit record in the tamper-evident chain.
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct AuditRecord {
    /// Audit record ID.
    pub id: String,
    /// Type of event (e.g., "task_completed", "rule_enabled").
    pub record_type: String,
    /// Subject being audited (e.g., task ID, rule ID).
    pub subject_ref: String,
    /// When the event occurred.
    pub occurred_at: DateTime<Utc>,
    /// SHA-256 hash of previous record (or "genesis").
    pub prev_hash: String,
    /// SHA-256 hash of this record.
    pub hash: String,
    /// Event payload (arbitrary JSON).
    pub payload: serde_json::Value,
}

/// A focus session (e.g., locked-in time on a task).
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct FocusSession {
    /// Unique session ID.
    pub id: String,
    /// Task being worked on.
    pub task_id: String,
    /// When the session started.
    pub started_at: DateTime<Utc>,
    /// When the session ended (if completed).
    pub ended_at: Option<DateTime<Utc>>,
    /// Duration of the session in seconds.
    pub duration_secs: Option<i64>,
}

/// Result of a sync trigger.
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct SyncResult {
    /// Connector ID that was synced.
    pub connector_id: String,
    /// Sync status ("started", "success", "error").
    pub status: String,
    /// Error message if status is "error".
    pub error: Option<String>,
}

// ============================================================================
// Query Root
// ============================================================================

/// Root query type for the GraphQL schema.
pub struct Query;

#[Object]
impl Query {
    /// Fetch tasks by optional status filter.
    /// Traces to: FR-GRAPHQL-QUERIES-001
    async fn tasks(&self, _status: Option<TaskStatus>) -> Vec<Task> {
        // Placeholder: in production, load from storage.
        vec![
            Task {
                id: "task-1".to_string(),
                title: "Complete report".to_string(),
                status: TaskStatus::Active,
                created_at: Utc::now(),
                due_at: Some(Utc::now()),
                description: Some("Finish Q2 report".to_string()),
            },
        ]
    }

    /// Fetch rules by optional enabled filter.
    /// Traces to: FR-GRAPHQL-QUERIES-002
    async fn rules(&self, _enabled: Option<bool>) -> Vec<Rule> {
        // Placeholder: in production, load from storage.
        vec![
            Rule {
                id: "rule-1".to_string(),
                name: "No social media before 9am".to_string(),
                enabled: true,
                rigidity: "Hard".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ]
    }

    /// Fetch wallet snapshot (credits, penalties, rewards).
    /// Traces to: FR-GRAPHQL-QUERIES-003
    async fn wallet(&self) -> WalletSnapshot {
        WalletSnapshot {
            credits: 1000,
            total_penalties: 50,
            total_rewards: 250,
            snapshot_at: Utc::now(),
        }
    }

    /// Fetch audit records (since optional timestamp, limit default 100).
    /// Traces to: FR-GRAPHQL-QUERIES-004
    async fn audit(&self, _since: Option<DateTime<Utc>>, _limit: Option<i32>) -> Vec<AuditRecord> {
        // Placeholder: in production, load from audit chain.
        vec![
            AuditRecord {
                id: "audit-1".to_string(),
                record_type: "task_completed".to_string(),
                subject_ref: "task-1".to_string(),
                occurred_at: Utc::now(),
                prev_hash: "genesis".to_string(),
                hash: "abc123def456".to_string(),
                payload: serde_json::json!({"status": "done"}),
            },
        ]
    }

    /// Fetch connectors (github, gcal, canvas, etc).
    /// Traces to: FR-GRAPHQL-QUERIES-005
    async fn connectors(&self) -> Vec<Connector> {
        // Placeholder: in production, load from connector registry.
        vec![
            Connector {
                id: "conn-github".to_string(),
                connector_type: "github".to_string(),
                is_connected: true,
                last_synced_at: Some(Utc::now()),
            },
        ]
    }

    /// Fetch focus sessions (since optional timestamp).
    /// Traces to: FR-GRAPHQL-QUERIES-006
    async fn focus_sessions(&self, _since: Option<DateTime<Utc>>) -> Vec<FocusSession> {
        // Placeholder: in production, load from session store.
        vec![
            FocusSession {
                id: "session-1".to_string(),
                task_id: "task-1".to_string(),
                started_at: Utc::now(),
                ended_at: None,
                duration_secs: None,
            },
        ]
    }
}

// Mutation stubs for future implementation
pub struct Mutation;

#[Object]
impl Mutation {
    /// Mark a task as done (requires auth).
    /// Traces to: FR-GRAPHQL-MUTATIONS-001
    async fn mark_task_done(&self, id: String) -> Task {
        Task {
            id,
            title: "Completed task".to_string(),
            status: TaskStatus::Done,
            created_at: Utc::now(),
            due_at: None,
            description: None,
        }
    }

    /// Enable a rule (requires auth).
    /// Traces to: FR-GRAPHQL-MUTATIONS-002
    async fn enable_rule(&self, id: String) -> Rule {
        Rule {
            id,
            name: "Enabled rule".to_string(),
            enabled: true,
            rigidity: "Hard".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Trigger a connector sync (requires auth).
    /// Traces to: FR-GRAPHQL-MUTATIONS-003
    async fn trigger_sync(&self, connector_id: String) -> SyncResult {
        SyncResult {
            connector_id,
            status: "started".to_string(),
            error: None,
        }
    }
}

/// Subscription root (placeholder for future implementation).
/// Note: subscriptions require full streaming support; for now using EmptySubscription
/// in the schema. See subscription.rs for the infrastructure.
pub struct Subscription;
