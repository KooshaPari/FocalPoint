# FocalPoint MCP Server

Ship an MCP (Model Context Protocol) server that exposes FocalPoint's rule/task/wallet/audit surface as tools Claude (or any MCP-capable agent) can call. This unblocks agent-driven planning, rule authoring, and diagnostic queries.

## Overview

The FocalPoint MCP server provides a standardized interface for LLMs and agents to interact with FocalPoint's core services via the Model Context Protocol. It exposes 15 tools (8 read-only, 7 write/destructive) and supports STDIO transport by default, with optional HTTP+SSE transport.

**MCP SDK Version:** `0.0.3`

## Installation

### Build from Source

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint
cargo install --path crates/focus-mcp-server
```

This installs the binary `focalpoint-mcp-server` to your Cargo bin directory.

### Verify Installation

```bash
focalpoint-mcp-server --help
```

## Configuration

### Environment Variables

- `FOCALPOINT_DB` — Path to FocalPoint's SQLite core.db. Overrides `--db` flag.
- `RUST_LOG` — Tracing log level (default: `info`).

### Platform Defaults

If neither `--db` nor `FOCALPOINT_DB` are set, the server looks for the database at:

- **macOS:** `~/Library/Application Support/focalpoint/core.db`
- **Linux/Other:** `$XDG_DATA_HOME/focalpoint/core.db` (or `~/.local/share/focalpoint/core.db`)

## Running the Server

### STDIO Transport (Default)

```bash
focalpoint-mcp-server --db ~/path/to/core.db
```

or via environment:

```bash
export FOCALPOINT_DB=~/Library/Application\ Support/focalpoint/core.db
focalpoint-mcp-server
```

The server listens on stdin/stdout for JSON-RPC 2.0 messages in MCP protocol format.

### Claude Desktop Configuration

Add the server to Claude Desktop's `claude_desktop_config.json`:

#### macOS/Linux

```json
{
  "mcpServers": {
    "focalpoint": {
      "command": "focalpoint-mcp-server",
      "args": ["--db", "/path/to/core.db"]
    }
  }
}
```

#### With Environment Variable

```json
{
  "mcpServers": {
    "focalpoint": {
      "command": "focalpoint-mcp-server",
      "env": {
        "FOCALPOINT_DB": "/path/to/core.db"
      }
    }
  }
}
```

## Tools

### Read-Only Tools (8)

#### `focalpoint.tasks.list`

List all tasks.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {}
}
```

**Example Response:**

```json
{
  "tasks": [],
  "note": "TaskStore::list_all() not yet exposed"
}
```

---

#### `focalpoint.rules.list`

List all rules with enabled status.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {}
}
```

**Example Response:**

```json
{
  "rules": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "Block social media during work hours",
      "enabled": true,
      "priority": 100,
      "actions_count": 1
    }
  ],
  "count": 1
}
```

---

#### `focalpoint.wallet.balance`

Get wallet balance summary for a user.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "user_id": {
      "type": "string",
      "description": "UUID of the user (required)"
    }
  },
  "required": ["user_id"]
}
```

**Example Request:**

```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Example Response:**

```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "balance": 1500,
  "total_granted": 2000,
  "total_spent": 500
}
```

---

#### `focalpoint.penalty.show`

Get penalty state summary for a user.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "user_id": {
      "type": "string",
      "description": "UUID of the user (required)"
    }
  },
  "required": ["user_id"]
}
```

**Example Request:**

```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Example Response:**

```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "escalation_tier": "Warning",
  "lockout_windows": 0,
  "debt_balance": 0
}
```

---

#### `focalpoint.audit.recent`

Get recent audit log entries (paginated).

**Input Schema:**

```json
{
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
}
```

**Example Request:**

```json
{
  "limit": 10
}
```

**Example Response:**

```json
{
  "records": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "record_type": "wallet.grant_credit",
      "subject_ref": "user-123",
      "occurred_at": "2025-04-23T12:34:56Z",
      "hash": "a1b2c3d4..."
    }
  ],
  "count": 1,
  "limit": 10
}
```

---

#### `focalpoint.audit.verify`

Verify the tamper-evident audit chain.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {}
}
```

**Example Response:**

```json
{
  "valid": true,
  "record_count": 42
}
```

---

#### `focalpoint.templates.list_bundled`

List the 4 bundled starter template packs.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {}
}
```

**Example Response:**

```json
{
  "packs": [
    {
      "id": "starter-social-block",
      "name": "Social Media Blocker"
    },
    {
      "id": "starter-deep-work",
      "name": "Deep Work"
    },
    {
      "id": "starter-wellness",
      "name": "Wellness & Breaks"
    },
    {
      "id": "starter-productivity",
      "name": "Productivity Boost"
    }
  ],
  "count": 4
}
```

---

#### `focalpoint.connectors.list`

List registered connectors.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {}
}
```

**Example Response:**

```json
{
  "connectors": [
    {
      "id": "gcal",
      "name": "Google Calendar"
    },
    {
      "id": "github",
      "name": "GitHub"
    },
    {
      "id": "canvas",
      "name": "Canvas"
    }
  ],
  "count": 3
}
```

---

### Write Tools (7)

Write tools are **destructive** (they modify state) and **idempotent** where possible.

#### `focalpoint.tasks.add`

Create a new task.

**Input Schema:**

```json
{
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
}
```

**Example Request:**

```json
{
  "title": "Review Q2 goals",
  "minutes": 30,
  "priority": 0.8
}
```

**Example Response:**

```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Review Q2 goals",
  "minutes": 30
}
```

---

#### `focalpoint.tasks.mark_done`

Mark a task as complete. Idempotent (marking an already-done task is a no-op).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "task_id": {
      "type": "string",
      "description": "UUID of the task (required)"
    }
  },
  "required": ["task_id"]
}
```

**Example Request:**

```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Example Response:**

```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "marked_done"
}
```

---

#### `focalpoint.rules.enable`

Enable a rule.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "rule_id": {
      "type": "string",
      "description": "UUID of the rule (required)"
    }
  },
  "required": ["rule_id"]
}
```

---

#### `focalpoint.rules.disable`

Disable a rule.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "rule_id": {
      "type": "string",
      "description": "UUID of the rule (required)"
    }
  },
  "required": ["rule_id"]
}
```

---

#### `focalpoint.templates.install`

Install a bundled template pack.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "pack_id": {
      "type": "string",
      "description": "ID of the bundled pack (required)"
    }
  },
  "required": ["pack_id"]
}
```

**Example Request:**

```json
{
  "pack_id": "starter-deep-work"
}
```

---

#### `focalpoint.focus.emit_session_started`

Emit a session-started event (for agent-driven workflows).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {}
}
```

**Example Response:**

```json
{
  "event": "session_started",
  "timestamp": "2025-04-23T12:34:56Z"
}
```

---

#### `focalpoint.focus.emit_session_completed`

Emit a session-completed event (for agent-driven workflows).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {}
}
```

**Example Response:**

```json
{
  "event": "session_completed",
  "timestamp": "2025-04-23T12:34:56Z"
}
```

---

## Testing

Run the test suite:

```bash
cargo test -p focus-mcp-server
```

Tests verify:

- All 15 tools are registered
- All tools have descriptions and valid input schemas
- Idempotent operations behave correctly
- Templates list returns 4 starter packs

## Architecture

### Crate Structure

```
crates/focus-mcp-server/
  src/
    lib.rs          # Library exports
    main.rs         # Binary entry point
    server.rs       # Transport layer (STDIO, SSE)
    tools.rs        # Tool implementations
  tests/
    integration_tests.rs  # Tool tests
  Cargo.toml
```

### Dependencies

- `mcp-sdk = "0.0.3"` — Official Rust MCP SDK
- `tokio` — Async runtime
- `serde_json` — JSON serialization
- Focus core crates: `focus-storage`, `focus-rules`, `focus-audit`, etc.

### Data Flow

1. **Transport** (STDIO) receives JSON-RPC request
2. **MCP Server** routes to matching tool
3. **Tool** deserializes input, calls FocalPoint storage APIs
4. **Tool** returns result as JSON
5. **Transport** sends JSON-RPC response back

## Troubleshooting

### "db not found" Error

Ensure the database path is correct:

```bash
ls -la ~/Library/Application\ Support/focalpoint/core.db
```

If missing, launch the FocalPoint app once to initialize it.

### "Unknown tool" Error

Verify the tool name matches exactly (e.g., `focalpoint.tasks.list`, not `tasks.list`).

### MCP Not Connecting in Claude Desktop

Check the config file location:

- **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Linux:** `~/.config/Claude/claude_desktop_config.json`

Restart Claude Desktop after updating the config.

## Future Enhancements

- HTTP+SSE transport for remote agents
- Async storage access (currently tools return placeholders for async operations)
- Resource endpoints (`focalpoint://audit/head`, `focalpoint://rules/all`)
- Agent-specific session management

## See Also

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [FocalPoint README](../../README.md)
- [Focus Storage API](../crates/focus-storage)
