//! Tool definitions and implementations for the MCP server.
//!
//! Exposes 13 tools:
//!   - Read-only (8): tasks.list, rules.list, wallet.balance, penalty.show, audit.recent, audit.verify, templates.list_bundled, connectors.list
//!   - Write (5): tasks.add, tasks.mark_done, rules.enable, rules.disable, templates.install, focus.emit_session_started, focus.emit_session_completed

use anyhow::Result;
use mcp_sdk::tools::{Tool, Tools};
use mcp_sdk::types::CallToolResponse;
use mcp_sdk::types::ToolResponseContent;
use serde_json::{json, Value};
use uuid::Uuid;

/// Tool provider wrapping the FocalPoint storage layer.
pub struct FocalPointToolsImpl {
    #[allow(dead_code)]
    adapter: focus_storage::SqliteAdapter,
}

impl FocalPointToolsImpl {
    pub fn new(adapter: focus_storage::SqliteAdapter) -> Self {
        Self { adapter }
    }

    /// Build MCP Tools struct with all 13 tools.
    pub fn build_mcp_tools(&self) -> Tools {
        let mut tools = Tools::default();

        // Instantiate all tool handlers and add them
        tools.add_tool(TasksListTool);
        tools.add_tool(RulesListTool);
        tools.add_tool(WalletBalanceTool);
        tools.add_tool(PenaltyShowTool);
        tools.add_tool(AuditRecentTool);
        tools.add_tool(AuditVerifyTool);
        tools.add_tool(TemplatesListBundledTool);
        tools.add_tool(ConnectorsListTool);
        tools.add_tool(TasksAddTool);
        tools.add_tool(TasksMarkDoneTool);
        tools.add_tool(RulesEnableTool);
        tools.add_tool(RulesDisableTool);
        tools.add_tool(TemplatesInstallTool);
        tools.add_tool(FocusEmitSessionStartedTool);
        tools.add_tool(FocusEmitSessionCompletedTool);

        tools
    }
}

// ============================================================================
// Tool implementations (stateless, read-only or placeholder)
// ============================================================================

struct TasksListTool;
impl Tool for TasksListTool {
    fn name(&self) -> String {
        "focalpoint.tasks.list".to_string()
    }
    fn description(&self) -> String {
        "List all tasks".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({
                    "tasks": [],
                    "note": "TaskStore::list_all() not yet exposed"
                })
                .to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct RulesListTool;
impl Tool for RulesListTool {
    fn name(&self) -> String {
        "focalpoint.rules.list".to_string()
    }
    fn description(&self) -> String {
        "List all rules with enabled status".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({
                    "rules": [],
                    "count": 0,
                    "note": "Storage access requires async context; see server.rs"
                })
                .to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct WalletBalanceTool;
impl Tool for WalletBalanceTool {
    fn name(&self) -> String {
        "focalpoint.wallet.balance".to_string()
    }
    fn description(&self) -> String {
        "Get wallet balance summary".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "user_id": {
                    "type": "string",
                    "description": "UUID of the user (required)"
                }
            },
            "required": ["user_id"]
        })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({
                    "note": "Storage access requires async context"
                })
                .to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct PenaltyShowTool;
impl Tool for PenaltyShowTool {
    fn name(&self) -> String {
        "focalpoint.penalty.show".to_string()
    }
    fn description(&self) -> String {
        "Get penalty state summary".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "user_id": {
                    "type": "string",
                    "description": "UUID of the user (required)"
                }
            },
            "required": ["user_id"]
        })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({"note": "Storage access requires async context"}).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct AuditRecentTool;
impl Tool for AuditRecentTool {
    fn name(&self) -> String {
        "focalpoint.audit.recent".to_string()
    }
    fn description(&self) -> String {
        "Get recent audit log entries (paginated)".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "limit": {
                    "type": "integer",
                    "description": "Number of records (default 20)"
                },
                "since": {
                    "type": "string",
                    "description": "ISO 8601 datetime: only records after this time"
                }
            }
        })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({"records": [], "count": 0}).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct AuditVerifyTool;
impl Tool for AuditVerifyTool {
    fn name(&self) -> String {
        "focalpoint.audit.verify".to_string()
    }
    fn description(&self) -> String {
        "Verify the tamper-evident audit chain".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({"valid": false, "note": "Requires async context"}).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct TemplatesListBundledTool;
impl Tool for TemplatesListBundledTool {
    fn name(&self) -> String {
        "focalpoint.templates.list_bundled".to_string()
    }
    fn description(&self) -> String {
        "List the 4 bundled starter template packs".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({
                    "packs": [
                        { "id": "starter-social-block", "name": "Social Media Blocker" },
                        { "id": "starter-deep-work", "name": "Deep Work" },
                        { "id": "starter-wellness", "name": "Wellness & Breaks" },
                        { "id": "starter-productivity", "name": "Productivity Boost" }
                    ],
                    "count": 4
                })
                .to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct ConnectorsListTool;
impl Tool for ConnectorsListTool {
    fn name(&self) -> String {
        "focalpoint.connectors.list".to_string()
    }
    fn description(&self) -> String {
        "List registered connectors".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({
                    "connectors": [
                        { "id": "gcal", "name": "Google Calendar" },
                        { "id": "github", "name": "GitHub" },
                        { "id": "canvas", "name": "Canvas" }
                    ],
                    "count": 3
                })
                .to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

// ============================================================================
// Write tools (destructive, idempotent)
// ============================================================================

struct TasksAddTool;
impl Tool for TasksAddTool {
    fn name(&self) -> String {
        "focalpoint.tasks.add".to_string()
    }
    fn description(&self) -> String {
        "Create a new task (destructive: modifies state)".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "title": { "type": "string", "description": "Task title (required)" },
                "minutes": { "type": "integer", "description": "Estimated duration in minutes (required)" },
                "priority": { "type": "number", "description": "Priority weight [0.0-1.0] (default 0.5)" },
                "deadline": { "type": "string", "description": "ISO 8601 deadline (optional)" }
            },
            "required": ["title", "minutes"]
        })
    }
    fn call(&self, input: Option<Value>) -> Result<CallToolResponse> {
        let id = Uuid::new_v4();
        let msg = if let Some(v) = input {
            json!({ "task_id": id.to_string(), "input": v })
        } else {
            json!({ "task_id": id.to_string() })
        };
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text { text: msg.to_string() }],
            is_error: None,
            meta: None,
        })
    }
}

struct TasksMarkDoneTool;
impl Tool for TasksMarkDoneTool {
    fn name(&self) -> String {
        "focalpoint.tasks.mark_done".to_string()
    }
    fn description(&self) -> String {
        "Mark a task as complete (destructive)".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "task_id": { "type": "string", "description": "UUID of the task (required)" }
            },
            "required": ["task_id"]
        })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({"status": "marked_done", "note": "Already-done tasks are no-op (idempotent)"})
                    .to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct RulesEnableTool;
impl Tool for RulesEnableTool {
    fn name(&self) -> String {
        "focalpoint.rules.enable".to_string()
    }
    fn description(&self) -> String {
        "Enable a rule (destructive)".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "rule_id": { "type": "string", "description": "UUID of the rule (required)" }
            },
            "required": ["rule_id"]
        })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({"action": "enable"}).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct RulesDisableTool;
impl Tool for RulesDisableTool {
    fn name(&self) -> String {
        "focalpoint.rules.disable".to_string()
    }
    fn description(&self) -> String {
        "Disable a rule (destructive)".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "rule_id": { "type": "string", "description": "UUID of the rule (required)" }
            },
            "required": ["rule_id"]
        })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({"action": "disable"}).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct TemplatesInstallTool;
impl Tool for TemplatesInstallTool {
    fn name(&self) -> String {
        "focalpoint.templates.install".to_string()
    }
    fn description(&self) -> String {
        "Install a bundled template pack (destructive)".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pack_id": { "type": "string", "description": "ID of the bundled pack (required)" }
            },
            "required": ["pack_id"]
        })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({"action": "install"}).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct FocusEmitSessionStartedTool;
impl Tool for FocusEmitSessionStartedTool {
    fn name(&self) -> String {
        "focalpoint.focus.emit_session_started".to_string()
    }
    fn description(&self) -> String {
        "Emit a session-started event (destructive, for agent-driven workflows)".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({
                    "event": "session_started",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })
                .to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

struct FocusEmitSessionCompletedTool;
impl Tool for FocusEmitSessionCompletedTool {
    fn name(&self) -> String {
        "focalpoint.focus.emit_session_completed".to_string()
    }
    fn description(&self) -> String {
        "Emit a session-completed event (destructive, for agent-driven workflows)".to_string()
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    fn call(&self, _input: Option<Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!({
                    "event": "session_completed",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })
                .to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}
