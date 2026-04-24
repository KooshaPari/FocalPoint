# FocalPoint MCP Server

Ship an MCP (Model Context Protocol) server that exposes FocalPoint's rule/task/wallet/audit surface as tools Claude (or any MCP-capable agent) can call. This unblocks agent-driven planning, rule authoring, and diagnostic queries.

## Overview

The FocalPoint MCP server provides a standardized interface for LLMs and agents to interact with FocalPoint's core services via the Model Context Protocol. It exposes **27 tools** (15 read-only, 12 write/destructive) and 2 resources for agent-driven planning, rule authoring, and diagnostics.

**MCP SDK Version:** `0.0.3`
**Tools:** 27 | **Resources:** 2 (RFC docs + weekly stats)

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

### Via `focus` CLI

The `focus` CLI provides a convenient wrapper around the MCP server:

```bash
focus mcp serve --mode stdio
```

or with custom database:

```bash
focus mcp serve --mode stdio --db ~/path/to/core.db
```

### Direct Binary

#### STDIO Transport (Default)

```bash
focalpoint-mcp-server --db ~/path/to/core.db
```

or via environment:

```bash
export FOCALPOINT_DB=~/Library/Application\ Support/focalpoint/core.db
focalpoint-mcp-server
```

The server listens on stdin/stdout for JSON-RPC 2.0 messages in MCP protocol format.

#### Socket Transport (Experimental)

```bash
focalpoint-mcp-server --mode socket --bind ~/Library/Containers/com.koosha.focalpoint/Data/Library/mcp.sock
```

**Note:** Socket mode is experimental and intended for iOS in-process bridge scenarios. Use STDIO for production.

### Claude Desktop Configuration

Add the server to Claude Desktop's `claude_desktop_config.json`:

#### macOS/Linux

```json
{
  "mcpServers": {
    "focalpoint": {
      "command": "focus",
      "args": ["mcp", "serve", "--mode", "stdio"]
    }
  }
}
```

#### Or with direct binary and environment variable

```json
{
  "mcpServers": {
    "focalpoint": {
      "command": "focalpoint-mcp-server",
      "args": ["--mode", "stdio"],
      "env": {
        "FOCALPOINT_DB": "/path/to/core.db"
      }
    }
  }
}
```

### Cursor Configuration

For Cursor, add to your project's `.cursor/config.json` or global Cursor settings:

```json
{
  "mcpServers": {
    "focalpoint": {
      "command": "focus",
      "args": ["mcp", "serve", "--mode", "stdio"],
      "env": {
        "FOCALPOINT_DB": "~/Library/Application Support/focalpoint/core.db"
      }
    }
  }
}
```

## Tools

### Read-Only Tools (15)

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

#### `focalpoint.audit.export`

Export audit records as JSONL for agent reasoning.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "last_n": {
      "type": "integer",
      "description": "Number of records to export (default 100)"
    },
    "since": {
      "type": "string",
      "description": "ISO 8601 datetime: only records after this time"
    }
  }
}
```

**Example Response:**

```json
{
  "records": [],
  "count": 0,
  "format": "jsonl"
}
```

---

#### `focalpoint.templates.catalog`

Expose the full ConnectorRegistry catalog for agent discovery.

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
  "registry": {
    "total": 4,
    "packs": [
      { "id": "starter-social-block", "name": "Social Media Blocker", "tags": ["productivity"] },
      { "id": "starter-deep-work", "name": "Deep Work", "tags": ["focus"] },
      { "id": "starter-wellness", "name": "Wellness & Breaks", "tags": ["health"] },
      { "id": "starter-productivity", "name": "Productivity Boost", "tags": ["productivity"] }
    ]
  }
}
```

---

#### `focalpoint.connectors.registry`

Expose the full connector registry for agent discovery.

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
  "registry": {
    "total": 3,
    "connectors": [
      { "id": "gcal", "name": "Google Calendar", "auth": "oauth2", "scope": "calendar.readonly" },
      { "id": "github", "name": "GitHub", "auth": "pat", "scope": "repos,user" },
      { "id": "canvas", "name": "Canvas", "auth": "instance_url+code", "scope": "canvas.user" }
    ]
  }
}
```

---

#### `focalpoint.focus.status`

Get current focus session status.

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
  "session": null,
  "active": false
}
```

---

#### `focalpoint.always_on.tick`

Get current NudgeProposalDto list from always-on evaluator.

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
  "nudges": [],
  "count": 0
}
```

---

#### `focalpoint.eval.tick_status`

Get status of the rule-eval pipeline tick.

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
  "status": "idle",
  "last_tick": null
}
```

---

#### `focalpoint.sync.tick_status`

Get status of the sync orchestrator tick.

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
  "status": "idle",
  "last_tick": null
}
```

---

### Write Tools (12)

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

#### `focalpoint.rules.upsert`

Upsert a rule from JSON RuleDraft (destructive, idempotent per rule_id).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "rule_id": { "type": "string", "description": "UUID (optional; generated if omitted)" },
    "name": { "type": "string", "description": "Rule name (required)" },
    "trigger": { "type": "object", "description": "Trigger condition (JSON, required)" },
    "action": { "type": "object", "description": "Action to execute (JSON, required)" },
    "enabled": { "type": "boolean", "description": "Enable on creation (default true)" }
  },
  "required": ["name", "trigger", "action"]
}
```

**Example Request:**

```json
{
  "name": "Block Instagram 9am-5pm",
  "trigger": { "time_range": "09:00-17:00" },
  "action": { "block_app": "com.instagram.android" },
  "enabled": true
}
```

**Example Response:**

```json
{
  "rule_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "upserted"
}
```

---

#### `focalpoint.rules.upsert_from_fpl`

Upsert rules from raw Starlark FPL code (destructive).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "fpl_code": {
      "type": "string",
      "description": "Raw Starlark FPL code (required)"
    }
  },
  "required": ["fpl_code"]
}
```

**Example Request:**

```json
{
  "fpl_code": "rule('block-instagram', trigger=time_range('09:00', '17:00'), action=block('com.instagram.android'))"
}
```

**Example Response:**

```json
{
  "status": "parse_stub",
  "note": "TODO: binding to focus-lang for Starlark compilation and rule installation"
}
```

---

#### `focalpoint.focus.cancel`

Cancel any in-progress focus session cleanly (destructive).

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
  "action": "cancel",
  "session": null
}
```

---

#### `focalpoint.wallet.spend`

Spend wallet credits for a purpose (destructive, idempotent).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "user_id": { "type": "string", "description": "UUID of the user (required)" },
    "amount": { "type": "integer", "description": "Amount to spend (required)" },
    "purpose": { "type": "string", "description": "Reason for spend (required)" }
  },
  "required": ["user_id", "amount", "purpose"]
}
```

**Example Request:**

```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "amount": 50,
  "purpose": "bypass social media block"
}
```

**Example Response:**

```json
{
  "status": "spent",
  "new_balance": 1450
}
```

---

#### `focalpoint.wallet.grant`

Grant wallet credits (destructive, testing utility, idempotent).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "user_id": { "type": "string", "description": "UUID of the user (required)" },
    "amount": { "type": "integer", "description": "Amount to grant (required)" },
    "purpose": { "type": "string", "description": "Reason for grant (required)" }
  },
  "required": ["user_id", "amount", "purpose"]
}
```

**Example Response:**

```json
{
  "status": "granted",
  "new_balance": 2000
}
```

---

#### `focalpoint.penalty.apply`

Apply a penalty mutation (destructive, idempotent).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "user_id": { "type": "string", "description": "UUID of the user (required)" },
    "mutation": { "type": "object", "description": "PenaltyMutation variant (required)" }
  },
  "required": ["user_id", "mutation"]
}
```

**Example Response:**

```json
{
  "status": "applied"
}
```

---

#### `focalpoint.connectors.connect_canvas`

Authenticate Canvas connector (destructive, idempotent).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "instance_url": { "type": "string", "description": "Canvas instance URL (required)" },
    "code": { "type": "string", "description": "OAuth authorization code (required)" }
  },
  "required": ["instance_url", "code"]
}
```

**Example Response:**

```json
{
  "status": "connected",
  "connector": "canvas"
}
```

---

#### `focalpoint.connectors.connect_gcal`

Authenticate Google Calendar connector (destructive, idempotent).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "code": { "type": "string", "description": "OAuth authorization code (required)" }
  },
  "required": ["code"]
}
```

**Example Response:**

```json
{
  "status": "connected",
  "connector": "gcal"
}
```

---

#### `focalpoint.connectors.connect_github`

Authenticate GitHub connector with PAT (destructive, idempotent).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "pat": { "type": "string", "description": "GitHub personal access token (required)" }
  },
  "required": ["pat"]
}
```

**Example Response:**

```json
{
  "status": "connected",
  "connector": "github"
}
```

---

## Resources

The MCP server exposes 2 resources for agent knowledge discovery:

### `focalpoint://docs/rfcs/*`

RFCs and governance documentation from the FocalPoint project. Agents can read:
- Architecture decision records (ADRs)
- Feature RFC specs
- Rules and policies
- Governance docs

**Example resource paths:**
- `focalpoint://docs/rfcs/rigidity-spectrum.md`
- `focalpoint://docs/rfcs/wallet-design.md`
- `focalpoint://docs/rfcs/always-on-evaluator.md`

### `focalpoint://stats/week`

7-day aggregated summary of user focus metrics:
- Total focus minutes
- Credits earned
- Rules fired (top 5)
- Connector sync status

**Example:**

```json
{
  "period": "2025-04-16T00:00:00Z to 2025-04-22T23:59:59Z",
  "focus_minutes": 840,
  "credits_earned": 250,
  "top_rules_fired": [
    { "name": "Deep Work Block", "count": 28 },
    { "name": "Social Media Blocker", "count": 15 }
  ],
  "syncs": {
    "gcal": "ok",
    "github": "ok",
    "canvas": "ok"
  }
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
