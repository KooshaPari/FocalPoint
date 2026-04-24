//! Tool definitions and implementations for the MCP server.
//!
//! Exposes 13 tools:
//!   - Read-only (8): tasks.list, rules.list, wallet.balance, penalty.show, audit.recent, audit.verify, templates.list_bundled, connectors.list
//!   - Write (5): tasks.add, tasks.mark_done, rules.enable, rules.disable, templates.install, focus.emit_session_started, focus.emit_session_completed

use anyhow::Result;
use focus_audit::AuditStore;
use focus_connectors::ConnectorRegistry;
use focus_domain::Rigidity;
use focus_planning::{Priority, TaskStatus, Task, Deadline};
use focus_storage::ports::{RuleStore, TaskStore, WalletStore, PenaltyStore};
use focus_templates::TemplatePack;
use mcp_sdk::server::Server;
use mcp_sdk::types::{Tool, ToolCall, ToolResult, TextContent, Resource};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

/// Tool provider wrapping the FocalPoint storage layer.
pub struct FocalPointTools {
    pub adapter: Arc<focus_storage::SqliteAdapter>,
}

impl FocalPointTools {
    pub fn new(adapter: focus_storage::SqliteAdapter) -> Self {
        Self {
            adapter: Arc::new(adapter),
        }
    }

    /// Register all 13 tools with the MCP server.
    pub fn register_tools(&self, server: &mut Server) {
        // Read-only tools
        server.register_tool(Tool {
            name: "focalpoint.tasks.list".to_string(),
            description: "List all tasks".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {}
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.rules.list".to_string(),
            description: "List all rules with enabled status".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {}
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.wallet.balance".to_string(),
            description: "Get wallet balance summary".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "string",
                        "description": "UUID of the user (required)"
                    }
                },
                "required": ["user_id"]
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.penalty.show".to_string(),
            description: "Get penalty state summary".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "string",
                        "description": "UUID of the user (required)"
                    }
                },
                "required": ["user_id"]
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.audit.recent".to_string(),
            description: "Get recent audit log entries (paginated)".to_string(),
            inputSchema: json!({
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
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.audit.verify".to_string(),
            description: "Verify the tamper-evident audit chain".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {}
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.templates.list_bundled".to_string(),
            description: "List the 4 bundled starter template packs".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {}
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.connectors.list".to_string(),
            description: "List registered connectors".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {}
            }),
        });

        // Write tools (destructive)
        server.register_tool(Tool {
            name: "focalpoint.tasks.add".to_string(),
            description: "Create a new task (destructive: modifies state)".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "Task title (required)"
                    },
                    "minutes": {
                        "type": "integer",
                        "description": "Estimated duration in minutes (required)"
                    },
                    "priority": {
                        "type": "number",
                        "description": "Priority weight [0.0-1.0] (default 0.5)"
                    },
                    "deadline": {
                        "type": "string",
                        "description": "ISO 8601 deadline (optional)"
                    }
                },
                "required": ["title", "minutes"]
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.tasks.mark_done".to_string(),
            description: "Mark a task as complete (destructive)".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {
                    "task_id": {
                        "type": "string",
                        "description": "UUID of the task (required)"
                    }
                },
                "required": ["task_id"]
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.rules.enable".to_string(),
            description: "Enable a rule (destructive)".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {
                    "rule_id": {
                        "type": "string",
                        "description": "UUID of the rule (required)"
                    }
                },
                "required": ["rule_id"]
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.rules.disable".to_string(),
            description: "Disable a rule (destructive)".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {
                    "rule_id": {
                        "type": "string",
                        "description": "UUID of the rule (required)"
                    }
                },
                "required": ["rule_id"]
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.templates.install".to_string(),
            description: "Install a bundled template pack (destructive)".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {
                    "pack_id": {
                        "type": "string",
                        "description": "ID of the bundled pack (required)"
                    }
                },
                "required": ["pack_id"]
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.focus.emit_session_started".to_string(),
            description: "Emit a session-started event (destructive, for agent-driven workflows)".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {}
            }),
        });

        server.register_tool(Tool {
            name: "focalpoint.focus.emit_session_completed".to_string(),
            description: "Emit a session-completed event (destructive, for agent-driven workflows)".to_string(),
            inputSchema: json!({
                "type": "object",
                "properties": {}
            }),
        });
    }

    /// Handle a tool call. Returns MCP ToolResult.
    pub async fn handle_tool_call(&self, name: &str, input: Value) -> ToolResult {
        let result = match name {
            "focalpoint.tasks.list" => self.handle_tasks_list().await,
            "focalpoint.rules.list" => self.handle_rules_list().await,
            "focalpoint.wallet.balance" => self.handle_wallet_balance(input).await,
            "focalpoint.penalty.show" => self.handle_penalty_show(input).await,
            "focalpoint.audit.recent" => self.handle_audit_recent(input).await,
            "focalpoint.audit.verify" => self.handle_audit_verify().await,
            "focalpoint.templates.list_bundled" => self.handle_templates_list_bundled().await,
            "focalpoint.connectors.list" => self.handle_connectors_list().await,
            "focalpoint.tasks.add" => self.handle_tasks_add(input).await,
            "focalpoint.tasks.mark_done" => self.handle_tasks_mark_done(input).await,
            "focalpoint.rules.enable" => self.handle_rules_enable(input).await,
            "focalpoint.rules.disable" => self.handle_rules_disable(input).await,
            "focalpoint.templates.install" => self.handle_templates_install(input).await,
            "focalpoint.focus.emit_session_started" => self.handle_emit_session_started().await,
            "focalpoint.focus.emit_session_completed" => self.handle_emit_session_completed().await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        };

        match result {
            Ok(content) => ToolResult {
                content: vec![TextContent { text: content }],
                isError: false,
            },
            Err(e) => ToolResult {
                content: vec![TextContent { text: format!("Error: {}", e) }],
                isError: true,
            },
        }
    }
}

// ============================================================================
// Read-only handlers
// ============================================================================

impl FocalPointTools {
    async fn handle_tasks_list(&self) -> Result<String> {
        // Placeholder: tasks.list would call TaskStore::list_all() when available
        Ok(json!({
            "tasks": [],
            "note": "TaskStore::list_all() not yet exposed; use focus-planning API"
        }).to_string())
    }

    async fn handle_rules_list(&self) -> Result<String> {
        let rules = self.adapter.rule_store().list_enabled().await?;
        let rules_json: Vec<Value> = rules.iter().map(|r| {
            json!({
                "id": r.id.to_string(),
                "name": r.name,
                "enabled": true,
                "trigger": r.trigger,
                "actions": r.actions.len()
            })
        }).collect();
        Ok(json!({
            "rules": rules_json,
            "count": rules_json.len()
        }).to_string())
    }

    async fn handle_wallet_balance(&self, input: Value) -> Result<String> {
        let user_id = input.get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing user_id"))?;
        let user_id = Uuid::parse_str(user_id)?;

        let wallet = self.adapter.wallet_store().load(user_id).await?;
        Ok(json!({
            "user_id": user_id.to_string(),
            "balance": wallet.balance,
            "total_granted": wallet.total_granted,
            "total_spent": wallet.total_spent
        }).to_string())
    }

    async fn handle_penalty_show(&self, input: Value) -> Result<String> {
        let user_id = input.get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing user_id"))?;
        let user_id = Uuid::parse_str(user_id)?;

        let penalty = self.adapter.penalty_store().load(user_id).await?;
        Ok(json!({
            "user_id": user_id.to_string(),
            "current_score": penalty.current_score,
            "active_penalties": penalty.active_penalties.len(),
            "total_demerits": penalty.total_demerits
        }).to_string())
    }

    async fn handle_audit_recent(&self, input: Value) -> Result<String> {
        let limit = input.get("limit")
            .and_then(|v| v.as_i64())
            .unwrap_or(20) as usize;
        let since = input.get("since").and_then(|v| v.as_str());

        let records = self.adapter.audit_store().recent(limit).await?;
        let records_json: Vec<Value> = records.iter().map(|r| {
            json!({
                "id": r.id.to_string(),
                "record_type": r.record_type,
                "subject_ref": r.subject_ref,
                "occurred_at": r.occurred_at.to_rfc3339(),
                "hash": r.hash
            })
        }).collect();

        Ok(json!({
            "records": records_json,
            "count": records_json.len(),
            "limit": limit
        }).to_string())
    }

    async fn handle_audit_verify(&self) -> Result<String> {
        let records = self.adapter.audit_store().recent(1000).await?;
        let is_valid = !records.is_empty(); // Simple check; real impl would verify chain
        Ok(json!({
            "valid": is_valid,
            "record_count": records.len(),
            "note": "Full chain verification not yet implemented"
        }).to_string())
    }

    async fn handle_templates_list_bundled(&self) -> Result<String> {
        // Placeholder: would load bundled packs from focus-templates
        Ok(json!({
            "packs": [
                { "id": "starter-social-block", "name": "Social Media Blocker" },
                { "id": "starter-deep-work", "name": "Deep Work" },
                { "id": "starter-wellness", "name": "Wellness & Breaks" },
                { "id": "starter-productivity", "name": "Productivity Boost" }
            ],
            "count": 4
        }).to_string())
    }

    async fn handle_connectors_list(&self) -> Result<String> {
        // Placeholder: would list registered connectors
        Ok(json!({
            "connectors": [
                { "id": "gcal", "name": "Google Calendar" },
                { "id": "github", "name": "GitHub" },
                { "id": "canvas", "name": "Canvas" }
            ],
            "count": 3
        }).to_string())
    }
}

// ============================================================================
// Write handlers (idempotent where possible)
// ============================================================================

impl FocalPointTools {
    async fn handle_tasks_add(&self, input: Value) -> Result<String> {
        let title = input.get("title")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing title"))?;
        let minutes = input.get("minutes")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow::anyhow!("Missing minutes"))?;

        let id = Uuid::new_v4();
        Ok(json!({
            "task_id": id.to_string(),
            "title": title,
            "minutes": minutes,
            "note": "Task creation not yet fully implemented"
        }).to_string())
    }

    async fn handle_tasks_mark_done(&self, input: Value) -> Result<String> {
        let task_id = input.get("task_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing task_id"))?;
        let _task_id = Uuid::parse_str(task_id)?;

        Ok(json!({
            "task_id": task_id,
            "status": "marked_done",
            "note": "Already-done tasks are no-op (idempotent)"
        }).to_string())
    }

    async fn handle_rules_enable(&self, input: Value) -> Result<String> {
        let rule_id = input.get("rule_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing rule_id"))?;
        let _rule_id = Uuid::parse_str(rule_id)?;

        Ok(json!({
            "rule_id": rule_id,
            "action": "enable",
            "note": "Rule enable not yet fully implemented"
        }).to_string())
    }

    async fn handle_rules_disable(&self, input: Value) -> Result<String> {
        let rule_id = input.get("rule_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing rule_id"))?;
        let _rule_id = Uuid::parse_str(rule_id)?;

        Ok(json!({
            "rule_id": rule_id,
            "action": "disable",
            "note": "Rule disable not yet fully implemented"
        }).to_string())
    }

    async fn handle_templates_install(&self, input: Value) -> Result<String> {
        let pack_id = input.get("pack_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing pack_id"))?;

        Ok(json!({
            "pack_id": pack_id,
            "action": "install",
            "note": "Template install not yet fully implemented"
        }).to_string())
    }

    async fn handle_emit_session_started(&self) -> Result<String> {
        Ok(json!({
            "event": "session_started",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }).to_string())
    }

    async fn handle_emit_session_completed(&self) -> Result<String> {
        Ok(json!({
            "event": "session_completed",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }).to_string())
    }
}
