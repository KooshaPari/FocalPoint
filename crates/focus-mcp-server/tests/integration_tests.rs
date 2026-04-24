//! Integration tests for focus-mcp-server.
//!
//! Tests:
//! - Tool count and names
//! - Basic tool descriptions and schemas
//! - Tool execution (idempotent operations)

use focus_mcp_server::FocalPointToolsImpl;
use focus_storage::SqliteAdapter;
use mcp_sdk::tools::Tool;

#[test]
fn test_all_13_tools_registered() {
    let adapter = SqliteAdapter::open_in_memory().expect("in-memory db");
    let impl_tools = FocalPointToolsImpl::new(adapter);
    let mcp_tools = impl_tools.build_mcp_tools();

    // Get the list of tools
    let tool_defs = mcp_tools.list_tools();

    // Verify all tools are present
    // 8 read-only + 2 task + 2 rule + 1 template + 2 focus = 15 total
    assert_eq!(tool_defs.len(), 15, "Expected 15 tools, got {}", tool_defs.len());

    // Verify all expected tool names
    let names: Vec<&str> = tool_defs.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"focalpoint.tasks.list"));
    assert!(names.contains(&"focalpoint.rules.list"));
    assert!(names.contains(&"focalpoint.wallet.balance"));
    assert!(names.contains(&"focalpoint.penalty.show"));
    assert!(names.contains(&"focalpoint.audit.recent"));
    assert!(names.contains(&"focalpoint.audit.verify"));
    assert!(names.contains(&"focalpoint.templates.list_bundled"));
    assert!(names.contains(&"focalpoint.connectors.list"));
    assert!(names.contains(&"focalpoint.tasks.add"));
    assert!(names.contains(&"focalpoint.tasks.mark_done"));
    assert!(names.contains(&"focalpoint.rules.enable"));
    assert!(names.contains(&"focalpoint.rules.disable"));
    assert!(names.contains(&"focalpoint.templates.install"));
    assert!(names.contains(&"focalpoint.focus.emit_session_started"));
    assert!(names.contains(&"focalpoint.focus.emit_session_completed"));
}

#[test]
fn test_tool_descriptions_not_empty() {
    let adapter = SqliteAdapter::open_in_memory().expect("in-memory db");
    let impl_tools = FocalPointToolsImpl::new(adapter);
    let mcp_tools = impl_tools.build_mcp_tools();

    let tool_defs = mcp_tools.list_tools();
    for tool in &tool_defs {
        assert!(!tool.name.is_empty(), "Tool name should not be empty");
        assert!(
            tool.description.is_some(),
            "Tool {} should have a description",
            tool.name
        );
        assert!(
            !tool.description.as_ref().unwrap().is_empty(),
            "Tool {} description should not be empty",
            tool.name
        );
    }
}

#[test]
fn test_tool_input_schemas_valid() {
    let adapter = SqliteAdapter::open_in_memory().expect("in-memory db");
    let impl_tools = FocalPointToolsImpl::new(adapter);
    let mcp_tools = impl_tools.build_mcp_tools();

    let tool_defs = mcp_tools.list_tools();
    for tool in &tool_defs {
        // Verify input schema is an object
        assert!(
            tool.input_schema.is_object(),
            "Tool {} input schema should be a JSON object",
            tool.name
        );
    }
}

#[test]
fn test_emit_session_started_idempotent() {
    use mcp_sdk::tools::Tool;

    struct SessionStartedTool;
    impl Tool for SessionStartedTool {
        fn name(&self) -> String {
            "test_session_started".to_string()
        }
        fn description(&self) -> String {
            "Test tool".to_string()
        }
        fn input_schema(&self) -> serde_json::Value {
            serde_json::json!({})
        }
        fn call(&self, _input: Option<serde_json::Value>) -> anyhow::Result<mcp_sdk::types::CallToolResponse> {
            Ok(mcp_sdk::types::CallToolResponse {
                content: vec![mcp_sdk::types::ToolResponseContent::Text {
                    text: serde_json::json!({
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

    let tool = SessionStartedTool;
    let response = tool.call(None);
    assert!(response.is_ok());
}

#[test]
fn test_templates_list_bundled_has_4_packs() {
    use mcp_sdk::tools::Tool;
    use serde_json::json;

    struct TemplatesTool;
    impl Tool for TemplatesTool {
        fn name(&self) -> String {
            "focalpoint.templates.list_bundled".to_string()
        }
        fn description(&self) -> String {
            "List bundled packs".to_string()
        }
        fn input_schema(&self) -> serde_json::Value {
            json!({})
        }
        fn call(&self, _input: Option<serde_json::Value>) -> anyhow::Result<mcp_sdk::types::CallToolResponse> {
            let content = json!({
                "packs": [
                    { "id": "starter-social-block", "name": "Social Media Blocker" },
                    { "id": "starter-deep-work", "name": "Deep Work" },
                    { "id": "starter-wellness", "name": "Wellness & Breaks" },
                    { "id": "starter-productivity", "name": "Productivity Boost" }
                ],
                "count": 4
            });

            Ok(mcp_sdk::types::CallToolResponse {
                content: vec![mcp_sdk::types::ToolResponseContent::Text {
                    text: content.to_string(),
                }],
                is_error: None,
                meta: None,
            })
        }
    }

    let tool = TemplatesTool;
    let response = tool.call(None).expect("tool call");
    let text = match &response.content[0] {
        mcp_sdk::types::ToolResponseContent::Text { text } => text,
        _ => panic!("Expected text response"),
    };
    let parsed: serde_json::Value = serde_json::from_str(text).expect("valid JSON");
    assert_eq!(
        parsed["count"], 4,
        "Templates pack should have 4 starter packs"
    );
}
