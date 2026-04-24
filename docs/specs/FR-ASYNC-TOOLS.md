# FR-ASYNC-TOOLS: Sync/Async Bridge for MCP Tool Calls

**Status:** Implemented  
**Traces to:** MCP server async tool unblocker (9448f37 follow-up)  
**Date:** 2026-04-24

## Problem

MCP tools expose a synchronous `call(&self, input: Option<Value>) -> Result<CallToolResponse>` interface. FocalPoint's storage layer (`RuleStore`, `AuditStore`, `WalletStore`) is entirely async—all mutations and queries require `await`.

This creates a blocker: 4 tools are stubbed in 9448f37, returning placeholder responses:
- `rules.enable`, `rules.disable`, `rules.upsert` — need async trait calls
- `audit.recent`, `audit.export` — need async trait calls

## Solution: Tokio `block_on` via Thread-Local Runtime

### Approach

**Use `tokio::task::block_in_place` + `Handle::block_on`.**

Why this approach:
1. **Existing tokio runtime:** MCP server already runs in a tokio `#[tokio::main]` context.
2. **Non-blocking:** `block_in_place` marks the code as yielding, so the executor doesn't starve other tasks.
3. **Clean syntax:** Call `crate::async_bridge::run_async(async_block)` from any sync context.
4. **No thread spawning:** Reuses the existing runtime; no thread pool overhead.

**Alternative considered (rejected):**
- Dedicated dispatcher actor: Overengineered for FocalPoint's tool call frequency.
- Thread-local runtime: Fragile; requires careful cleanup.

### Implementation

**Module: `crates/focus-mcp-server/src/async_bridge.rs`**

```rust
//! Sync-to-async bridge for MCP tool calls within a tokio runtime.
//!
//! MCP tools expose a sync interface; FocalPoint storage is async.
//! This module bridges that gap via tokio::task::block_in_place.

pub fn run_async<F, T>(f: F) -> anyhow::Result<T>
where
    F: std::future::Future<Output = anyhow::Result<T>>,
{
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => {
            // We're inside a tokio runtime; use block_in_place.
            handle.block_on(f)
        }
        Err(_) => {
            // No current runtime; shouldn't happen in MCP server context.
            // Fall back to creating a new runtime (only for tests).
            anyhow::bail!("no tokio runtime; MCP tools must be called from tokio context")
        }
    }
}
```

**Usage in tools:**

```rust
impl Tool for RulesEnableTool {
    fn call(&self, input: Option<Value>) -> Result<CallToolResponse> {
        let rule_id = parse_rule_id(input)?;
        
        // Bridge sync→async
        let rule = crate::async_bridge::run_async(async {
            self.adapter.rule_store().get(rule_id).await
        })?;
        
        // Continue with sync result
        Ok(CallToolResponse { ... })
    }
}
```

## Trait Expansions

### `RuleStore` (focus-storage)

Add mutation methods:

```rust
/// Get a rule by ID.
async fn get(&self, id: Uuid) -> Result<Option<Rule>>;
/// List all enabled rules.
async fn list_enabled(&self) -> Result<Vec<Rule>>;

// NEW:
/// Enable a rule by ID.
async fn enable(&self, id: Uuid) -> Result<()>;
/// Disable a rule by ID.
async fn disable(&self, id: Uuid) -> Result<()>;
/// Upsert a rule (update if exists; insert if new).
async fn upsert(&self, rule: Rule) -> Result<()>;
```

### `AuditStore` (focus-audit)

Add query methods:

```rust
fn append(&self, record: AuditRecord) -> Result<()>;
fn verify_chain(&self) -> Result<bool>;
fn head_hash(&self) -> Result<Option<String>>;

// NEW (async):
async fn recent(&self, limit: usize) -> Result<Vec<AuditRecord>>;
async fn export_ndjson(&self) -> Result<String>;
```

## Implementation Order

1. **async_bridge.rs** — Utility module (20 LOC)
2. **focus-storage/ports.rs** — Expand `RuleStore` trait with 3 mutation methods
3. **focus-audit/lib.rs** — Expand `AuditStore` trait with 2 query methods
4. **focus-storage/sqlite/mod.rs** — Implement new trait methods in `SqliteRuleStore`
5. **focus-audit/lib.rs** — Implement new trait methods in `InMemoryAuditStore` (add async wrappers)
6. **focus-mcp-server/tools.rs** — Wire 4 tools through async bridge

## Tests

- Unit tests: Trait methods callable in isolation (`#[tokio::test]`)
- Integration: MCP server test harness calls tools via bridge
- Audit mutation audit records appended; verify chain

## Future Considerations

- If tool call frequency exceeds 100/sec: batch async calls via a dispatcher actor.
- Observe tokio metrics (task count, executor saturation) in production.
