# FocalPoint CLI JSON Output Schema

The `focus` CLI supports a global `--json` / `-j` flag to emit structured JSON instead of human-readable text. This document specifies the schema for each subcommand's JSON output.

## Global Options

```
focus [--db PATH] [--json] <COMMAND> <SUBCOMMAND>
```

- `--db PATH`: Path to SQLite database (default: `~/Library/Application Support/focalpoint/core.db`)
- `--json`, `-j`: Emit JSON instead of human text

## Error Response (all commands)

When `--json` is set and an error occurs, output goes to stderr as:

```json
{
  "error": {
    "code": "string",
    "message": "string",
    "details": "optional string"
  }
}
```

Exit code is non-zero. Examples:
- `code: "PARSE_ERROR"`, `message: "invalid UUID format"`
- `code: "NOT_FOUND"`, `message: "task not found: 123e4567-e89b-12d3-a456-426614174000"`
- `code: "DB_ERROR"`, `message: "database locked"`

---

## Audit Commands

### `focus audit verify`

**Human output:** `"chain verified"` or error message.

**JSON output:**
```json
{
  "verified": true,
  "chain_length": 42,
  "root_hash": "sha256:abcd1234..."
}
```

### `focus audit tail [--limit N]`

**Human output:** One audit record per line (JSON lines format).

**JSON output:**
```json
[
  {
    "id": "uuid",
    "ts": "2026-04-23T10:30:00Z",
    "kind": "wallet_credit_granted",
    "user_id": "uuid",
    "payload": { /* event data */ }
  },
  ...
]
```

### `focus audit head`

**Human output:** Hex hash string or `"(empty)"`.

**JSON output:**
```json
{
  "hash": "sha256:abcd1234..." | null
}
```

---

## Tasks Commands

### `focus tasks list [--user_id UUID]`

**Human output:**
```
<id>  <status>  <title> (priority=0.500)
```

**JSON output:**
```json
[
  {
    "id": "uuid",
    "title": "Review PRs",
    "priority": 0.8,
    "status": "Active",
    "deadline": "2026-04-24T17:00:00Z" | null,
    "created_at": "2026-04-23T09:00:00Z",
    "updated_at": "2026-04-23T10:15:00Z"
  },
  ...
]
```

### `focus tasks add --title <TITLE> --minutes <N> [--priority h|m|l] [--deadline ISO8601]`

**Human output:** `"task created: <id>"`

**JSON output:**
```json
{
  "id": "uuid",
  "title": "Review PRs",
  "priority": 0.8,
  "status": "Active",
  "deadline": "2026-04-24T17:00:00Z" | null,
  "created_at": "2026-04-23T09:00:00Z",
  "updated_at": "2026-04-23T09:00:00Z"
}
```

### `focus tasks done <ID>`

**Human output:** `"task marked complete: <id>"`

**JSON output:** (task object after completion)
```json
{
  "id": "uuid",
  "title": "Review PRs",
  "priority": 0.8,
  "status": "Completed",
  "deadline": "2026-04-24T17:00:00Z" | null,
  "created_at": "2026-04-23T09:00:00Z",
  "updated_at": "2026-04-23T10:30:00Z"
}
```

### `focus tasks remove <ID>`

**Human output:** `"task removed: <id>"` or `"task not found: <id>"`

**JSON output:**
```json
{
  "id": "uuid",
  "removed": true | false
}
```

---

## Templates Commands

### `focus templates list`

**Human output:**
```
<id>  v<ver>  <name>  (<N> rules)  — <desc>
```

**JSON output:**
```json
[
  {
    "id": "daily-rhythm",
    "version": "1.0.0",
    "name": "Daily Rhythm Template",
    "rules": 5,
    "description": "Auto-adjust focus windows based on energy levels"
  },
  ...
]
```

### `focus templates install <PACK_ID> [--manifest PATH] [--require_signature]`

**Human output:** `"installed template pack: <id> v<ver> (<N> rules)"`

**JSON output:**
```json
{
  "pack_id": "daily-rhythm",
  "rules_installed": 5,
  "tasks_installed": 0,
  "signed_by": "phenotype-root-key" | null,
  "sha256": "abcd1234..."
}
```

---

## Rules Commands

### `focus rules list`

**Human output:**
```
<id>  <name>  priority=<N>  enabled=<bool>  trigger=<type>:<value>
```

**JSON output:**
```json
[
  {
    "id": "uuid",
    "name": "Morning Focus Block",
    "priority": 10,
    "enabled": true,
    "trigger": "schedule:0 6 * * *"
  },
  {
    "id": "uuid",
    "name": "Distraction Suppression",
    "priority": 5,
    "enabled": false,
    "trigger": "event:app_launch"
  },
  ...
]
```

### `focus rules enable <ID>`

**Human output:** `"rule enabled: <id>"`

**JSON output:**
```json
{
  "id": "uuid",
  "enabled": true
}
```

### `focus rules disable <ID>`

**Human output:** `"rule disabled: <id>"`

**JSON output:**
```json
{
  "id": "uuid",
  "enabled": false
}
```

### `focus rules upsert --file <PATH>`

**Human output (TOML):** `"upserted <N> rules from template pack"`

**JSON output (TOML):**
```json
{
  "rules_upserted": 5,
  "source": "template_pack",
  "pack_id": "daily-rhythm"
}
```

**Human output (JSON):** `"upserted rule: <id>"`

**JSON output (JSON):**
```json
{
  "id": "uuid",
  "upserted": true
}
```

---

## Wallet Commands

### `focus wallet balance [--user_id UUID]`

**Human output:**
```
user_id: <uuid>
earned_credits: 1000
spent_credits: 250
balance: 750
multiplier: 1.2 (expires: Some(...))
streaks: [...]
```

**JSON output:**
```json
{
  "user_id": "uuid",
  "earned_credits": 1000,
  "spent_credits": 250,
  "balance": 750,
  "multiplier": 1.2,
  "multiplier_expires_at": "2026-05-23T10:30:00Z" | null
}
```

### `focus wallet grant <AMOUNT> --purpose <REASON> [--user_id UUID]`

**Human output:** `"granted <N> credits (purpose: <reason>)"`

**JSON output:**
```json
{
  "balance_before": 750,
  "balance_after": 850,
  "delta": 100,
  "reason": "bonus_completion"
}
```

### `focus wallet spend <AMOUNT> --purpose <REASON> [--user_id UUID]`

**Human output:** `"spent <N> credits (purpose: <reason>)"`

**JSON output:**
```json
{
  "balance_before": 850,
  "balance_after": 650,
  "delta": -200,
  "reason": "premium_feature_unlock"
}
```

---

## Penalty Commands

### `focus penalty show [--user_id UUID]`

**Human output:**
```
user_id: <uuid>
escalation_tier: Tier1
bypass_budget: 10
debt_balance: 5
strict_mode_until: Some(...)
lockout_windows:
  <start> — <end> (<reason>) [rigidity: <type>]
```

**JSON output:**
```json
{
  "user_id": "uuid",
  "escalation_tier": "Tier1",
  "bypass_budget": 10,
  "debt_balance": 5,
  "strict_mode_until": "2026-04-30T10:30:00Z" | null,
  "lockout_windows": [
    {
      "starts_at": "2026-04-23T14:00:00Z",
      "ends_at": "2026-04-23T16:00:00Z",
      "reason": "focus_violation",
      "rigidity": "Hard"
    },
    ...
  ]
}
```

---

## Connectors Commands

### `focus connectors list`

**Human output:** (not yet implemented)

**JSON output:**
```json
{
  "message": "connector registry not yet built into CLI"
}
```

### `focus connectors sync <ID>`

**Human output:** (not yet implemented)

**JSON output:**
```json
{
  "error": "connector sync not implemented",
  "id": "github"
}
```

---

## Sync Commands

### `focus sync tick`

**Human output:** (not yet implemented)

**JSON output:**
```json
{
  "error": "sync orchestrator not yet built into CLI"
}
```

---

## Eval Commands

### `focus eval tick`

**Human output:** (not yet implemented)

**JSON output:**
```json
{
  "error": "eval pipeline not yet built into CLI"
}
```

---

## Focus Commands

### `focus focus start <MINUTES>`

**Human output:** `"focus:session_started (minutes=<N>) [test event emitted]"`

**JSON output:**
```json
{
  "event_type": "focus:session_started",
  "minutes": 45,
  "timestamp": "2026-04-23T10:30:00Z"
}
```

### `focus focus complete <MINUTES>`

**Human output:** `"focus:session_completed (minutes=<N>) [test event emitted]"`

**JSON output:**
```json
{
  "event_type": "focus:session_completed",
  "minutes": 45,
  "timestamp": "2026-04-23T11:15:00Z"
}
```

---

## Release Notes Commands

### `focus release-notes generate [--since TAG] [--format md|discord|testflight]`

**Human output (format=md):**
```markdown
### Added
- feature 1 (abcd123)
- feature 2 (def4567)

### Fixed
- bug fix 1 (1234abc)
```

**JSON output:**
```json
{
  "sections": [
    {
      "category": "Added",
      "items": [
        "feature 1",
        "feature 2"
      ]
    },
    {
      "category": "Fixed",
      "items": [
        "bug fix 1"
      ]
    },
    ...
  ]
}
```

---

## Usage Examples

```bash
# List tasks as JSON
focus --json tasks list

# Get wallet balance as JSON
focus --json wallet balance

# Verify audit chain as JSON
focus --json audit verify

# Install template with JSON output
focus --json templates install daily-rhythm

# Parse JSON output in a script
focus --json tasks list | jq '.[] | select(.status == "Active")'
```

---

## Notes

1. All timestamps are ISO 8601 format with UTC timezone.
2. UUIDs are string representation (e.g., `"123e4567-e89b-12d3-a456-426614174000"`).
3. Currency/amounts are always integers (cents or smallest currency unit).
4. Floating-point values (e.g., `priority`) are decimal numbers.
5. Null values are explicitly represented (not omitted).
6. Arrays are empty `[]` when no items exist; never null.
7. Error JSON always goes to stderr, success JSON goes to stdout.
