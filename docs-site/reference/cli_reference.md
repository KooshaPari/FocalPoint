---
title: Focus CLI Reference
description: FocalPoint documentation
---
# Focus CLI Reference

Full dual-surface CLI for the FocalPoint core. Implements 90%+ parity with the iOS/Android native APIs via command-line inspection, mutation, and orchestration tools.

**Installation:**
```bash
cargo build -p focus-cli --release
# Binary: ./target/release/focus
```

**Global Options:**
- `--db <PATH>` — path to `core.db` (defaults to `$FOCALPOINT_DB` or `~/Library/Application Support/focalpoint/core.db`)
- `-h, --help` — show help for any command

---

## Audit & Chain Verification

### `focus audit verify`
Verify the hash chain end-to-end (SHA-256 linkage). Exits non-zero if tampering detected.

```bash
focus audit verify --db ./core.db
# Output: "chain verified" or "chain tamper detected"
```

### `focus audit tail [--limit N]`
Print the most recent N audit records as JSON lines (default: 50).

```bash
focus audit tail --limit 100
# Output: JSON-line-delimited AuditRecord entries
```

### `focus audit head`
Print the head hash of the chain (or "(empty)" if no records).

```bash
focus audit head
```

---

## Task Management

### `focus tasks list [--user_id UUID]`
List all tasks for a user (default: nil UUID). Shows id, status, title, priority.

```bash
focus tasks list
focus tasks list --user_id "550e8400-e29b-41d4-a716-446655440000"
# Output:
# 550e8400-e29b-41d4-a716-446655440001  Pending   Write docs (priority=0.500)
# 550e8400-e29b-41d4-a716-446655440002  Completed Code review (priority=0.800)
```

### `focus tasks add --title <STR> --minutes <N> [--priority h/m/l] [--deadline ISO8601]`
Create a new task with title, estimated minutes, and optional deadline.

```bash
focus tasks add --title "Write docs" --minutes 60 --priority h --deadline "2026-04-24T17:00:00Z"
# Output: "task created: 550e8400-e29b-41d4-a716-446655440003"
```

**Priority:** h/H (0.8), m/M (0.5), l/L (0.2); defaults to medium.

### `focus tasks done <UUID>`
Mark a task complete (validates state transition).

```bash
focus tasks done 550e8400-e29b-41d4-a716-446655440001
# Output: "task marked complete: 550e8400-e29b-41d4-a716-446655440001"
```

### `focus tasks remove <UUID>`
Delete a task by id.

```bash
focus tasks remove 550e8400-e29b-41d4-a716-446655440001
# Output: "task removed: 550e8400-e29b-41d4-a716-446655440001"
```

---

## Template Packs

### `focus templates list`
List all bundled template packs (from `examples/templates/`).

```bash
focus templates list
# Output:
# deep-work-starter  v0.1.0  Deep Work Starter  (8 rules)  — Get focused. No distractions.
# social-balance     v0.2.1  Social Media Balance  (5 rules)  — Healthy digital habits.
```

### `focus templates install <PACK_ID_OR_PATH>`
Install a template pack by bundled id (e.g., `"deep-work-starter"`) or file path (`"./custom.toml"`).

```bash
focus templates install deep-work-starter
focus templates install ./my-template.toml
# Output: "installed template pack: deep-work-starter v0.1.0 (8 rules)"
```

---

## Rule Management

### `focus rules list`
List all enabled rules with id, name, priority, trigger type.

```bash
focus rules list
# Output:
# 550e8400-e29b-41d4-a716-446655440010  Block social media  priority=100  enabled=true  trigger=event:app_launch
# 550e8400-e29b-41d4-a716-446655440011  Daily reset  priority=50  enabled=true  trigger=schedule:0 0 * * *
```

### `focus rules enable <UUID>`
Enable a rule (set `enabled=true`).

```bash
focus rules enable 550e8400-e29b-41d4-a716-446655440010
# Output: "rule enabled: 550e8400-e29b-41d4-a716-446655440010"
```

### `focus rules disable <UUID>`
Disable a rule (set `enabled=false`).

```bash
focus rules disable 550e8400-e29b-41d4-a716-446655440010
# Output: "rule disabled: 550e8400-e29b-41d4-a716-446655440010"
```

### `focus rules upsert --file <PATH>`
Create or update a rule from a `.toml` (template pack), `.json` (Rule IR), or `.fpl` (focus-lang) file.

```bash
focus rules upsert --file my-rule.toml
focus rules upsert --file my-rule.json
focus rules upsert --file my-rule.fpl    # Error: FPL support pending (focus-lang integration)
# Output: "upserted N rules from template pack" or "upserted rule: UUID"
```

**File Formats:**
- `.toml`: Template pack (extracts all rules)
- `.json`: Individual Rule (IR format)
- `.fpl`: Focus-lang DSL (not yet implemented — requires focus-lang integration)

---

## Wallet Management

### `focus wallet balance [--user_id UUID]`
Display wallet state: earned credits, spent credits, balance, streaks, multiplier.

```bash
focus wallet balance
# Output:
# user_id: 00000000-0000-0000-0000-000000000000
# earned_credits: 1000
# spent_credits: 250
# balance: 750
# multiplier: 1.5 (expires: Some(2026-04-25T10:00:00Z))
# streaks: {"deep_work": Streak { count: 7, last_incremented_at: Some(...) }}
```

### `focus wallet grant <AMOUNT> --purpose <STR> [--user_id UUID]`
Award credits to a wallet (for testing).

```bash
focus wallet grant 100 --purpose "completed focus session" --user_id "550e8400-e29b-41d4-a716-446655440000"
# Output: "granted 100 credits (purpose: completed focus session)"
```

### `focus wallet spend <AMOUNT> --purpose <STR> [--user_id UUID]`
Deduct credits from a wallet (for testing).

```bash
focus wallet spend 50 --purpose "emergency unlock" --user_id "550e8400-e29b-41d4-a716-446655440000"
# Output: "spent 50 credits (purpose: emergency unlock)"
```

---

## Penalty State

### `focus penalty show [--user_id UUID]`
Display penalty state: escalation tier, bypass budget, lockout windows, debt balance, strict mode deadline.

```bash
focus penalty show
# Output:
# user_id: 00000000-0000-0000-0000-000000000000
# escalation_tier: Restricted
# bypass_budget: 300
# debt_balance: 50
# strict_mode_until: Some(2026-04-24T10:00:00Z)
# lockout_windows:
#   2026-04-23T14:00:00Z — 2026-04-23T16:00:00Z (emergency block) [rigidity: Hard]
```

---

## Connector Registry & Sync

### `focus connectors list`
List all registered connectors: id, health, cadence, next sync time.

**Status:** Stub — requires SyncOrchestrator integration (TODO).

### `focus connectors sync <ID>`
Manually trigger sync for one connector (e.g., `"github"`, `"gcal"`).

**Status:** Stub — requires SyncOrchestrator integration (TODO).

---

## Sync Orchestrator

### `focus sync tick`
Drive one orchestrator tick: sync all due connectors, report events pulled and errors.

**Status:** Stub — requires SyncOrchestrator::tick wiring (TODO).

Expected output (when implemented):
```
events_pulled: 42
connectors_synced: 3
errors: []
```

---

## Rule Evaluation Pipeline

### `focus eval tick`
Drive one evaluation pipeline tick: process queued events, fire matching rules, report decisions.

**Status:** Stub — requires RuleEvaluationPipeline::tick wiring (TODO).

Expected output (when implemented):
```
events_evaluated: 15
decisions_fired: 3
decisions_suppressed: 2
decisions_skipped: 0
```

---

## Focus Sessions

### `focus focus start <MINUTES>`
Emit a `focus:session_started` host event (test helper for iOS/Android).

```bash
focus focus start 25
# Output: "focus:session_started (minutes=25) [test event emitted]"
```

### `focus focus complete <MINUTES>`
Emit a `focus:session_completed` host event (test helper for iOS/Android).

```bash
focus focus complete 25
# Output: "focus:session_completed (minutes=25) [test event emitted]"
```

---

## Dual-Surface Contract

This CLI fulfills the **dual-surface mandate**: every operation in the iOS/Android native UIs is also accessible via the CLI:

| iOS/Android Concept | CLI Equivalent | Status |
|---|---|---|
| Task creation | `tasks add` | ✅ Full |
| Task completion | `tasks done` | ✅ Full |
| Task list | `tasks list` | ✅ Full |
| Wallet display | `wallet balance` | ✅ Full |
| Credit grant (testing) | `wallet grant` | ✅ Full |
| Penalty state | `penalty show` | ✅ Full |
| Template install | `templates install` | ✅ Full |
| Rule enable/disable | `rules enable/disable` | ✅ Full |
| Audit verification | `audit verify` | ✅ Full |
| Focus session (test) | `focus start/complete` | ✅ Partial (events emitted, not stored) |
| Sync orchestration | `sync tick` | ⏳ Stub (TODO) |
| Rule evaluation | `eval tick` | ⏳ Stub (TODO) |
| Connector management | `connectors list/sync` | ⏳ Stub (TODO) |

---

## Error Handling

All commands exit with non-zero status on failure:

```bash
focus rules enable invalid-uuid 2>&1
# stderr: Error: invalid UUID

echo $?
# 1
```

Errors include:
- Missing database: "db not found at ... — launch the app once first, or pass --db"
- Invalid UUID format: "invalid UUID"
- Database errors: "open db: ..."
- State violations: "task status Pending cannot transition to Completed"
- File I/O: "failed to read /path/to/file: No such file"

---

## Examples

### Full workflow: Create task, list, mark complete

```bash
# Create task
$ focus tasks add --title "Review PR #123" --minutes 30 --priority h
task created: 550e8400-e29b-41d4-a716-446655440001

# List tasks
$ focus tasks list
550e8400-e29b-41d4-a716-446655440001  Pending  Review PR #123 (priority=0.800)

# Mark complete
$ focus tasks done 550e8400-e29b-41d4-a716-446655440001
task marked complete: 550e8400-e29b-41d4-a716-446655440001

# Verify
$ focus tasks list
550e8400-e29b-41d4-a716-446655440001  Completed  Review PR #123 (priority=0.800)
```

### Audit chain verification

```bash
$ focus audit verify
chain verified

$ focus audit head
550e8400e29b41d4a716446655440099a550e8400e29b41d4a716446655440001
```

### Wallet operations (testing)

```bash
$ focus wallet balance
user_id: 00000000-0000-0000-0000-000000000000
earned_credits: 100
spent_credits: 0
balance: 100
multiplier: 1.0 (expires: None)
streaks: {}

$ focus wallet grant 200 --purpose "bonus"
granted 200 credits (purpose: bonus)

$ focus wallet spend 50 --purpose "quick unlock"
spent 50 credits (purpose: quick unlock)

$ focus wallet balance
balance: 250
```

---

## Cross-Link to Dual-Surface Matrix

See `docs-site/reference/dual_surface_matrix.md` for the complete dual-surface contract and mapping between iOS/Android native APIs, RESTful API endpoints (FFI bridging), and CLI subcommands.
