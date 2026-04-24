# Bulk Authoring Guide

This guide covers CSV and YAML bulk import/export for FocalPoint rules and tasks, enabling rapid authoring and migration from other platforms.

## Quick Start

### Import Rules from CSV

```bash
focus rules import examples/bulk/sample-rules.csv
```

### Import Tasks from YAML

```bash
focus tasks import examples/bulk/sample-tasks.yaml
```

### Export All Rules

```bash
focus rules export --format=csv --output=my-rules.csv
focus rules export --format=yaml --output=my-rules.yaml
```

### Export All Tasks

```bash
focus tasks export --format=csv --output=my-tasks.csv
focus tasks export --format=yaml --output=my-tasks.yaml
```

### Dry-Run Preview

Preview what will be imported without persisting:

```bash
focus rules import sample-rules.csv --dry-run
focus tasks import sample-tasks.yaml --dry-run
```

## CSV Schema: Rules

| Column | Type | Required | Description | Example |
|--------|------|----------|-------------|---------|
| `name` | String | Yes | Rule name (unique identifier) | `"App Launch Bonus"` |
| `trigger_kind` | Enum | Yes | Event, Schedule, or StateChange | `Event` |
| `event_type` | String | No | Event/schedule/state name (depends on trigger_kind) | `app_launch` |
| `action_kind` | Enum | Yes | GrantCredit, DeductCredit, Block, Unblock, StreakIncrement, StreakReset, Notify, etc. | `GrantCredit` |
| `amount` | Integer | No | For GrantCredit/DeductCredit | `50` |
| `cooldown` | String | No | Cooldown duration (e.g., "5m", "1h") | `5m` |
| `priority` | Integer | Yes | Priority order (0=lowest, 3=highest) | `1` |
| `enabled` | Boolean | Yes | Whether rule is active | `true` |

### CSV Rules Example

```csv
name,trigger_kind,event_type,action_kind,amount,cooldown,priority,enabled
Focus Session Completed,Event,focus_session_end,GrantCredit,50,5m,2,true
App Launch Detected,Event,app_launch,GrantCredit,25,,1,true
Social Media Blocked,Event,sns_access_attempt,Block,,,3,true
Break Time Scheduled,Schedule,daily_10am,Notify,,30m,0,true
```

## CSV Schema: Tasks

| Column | Type | Required | Description | Example |
|--------|------|----------|-------------|---------|
| `title` | String | Yes | Task name | `"Write Q2 proposal"` |
| `priority` | Float | No | Priority in [0.0, 1.0] (0=lowest, 1.0=highest) | `0.8` |
| `deadline` | String (ISO 8601) | No | Deadline date/time | `2026-04-30T15:00:00Z` |
| `duration_min` | Integer | No | Estimated minutes to complete | `60` |
| `tags` | String (comma-separated) | No | Tags for categorization (comma-separated, spaces trimmed) | `work,urgent` |

### CSV Tasks Example

```csv
title,priority,deadline,duration_min,tags
Write proposal,0.9,2026-04-30T15:00:00Z,120,work,proposal
Review feedback,0.7,,45,review
Personal project,0.3,,90,hobby
```

## YAML Schema: Rules

YAML supports richer nesting and optional fields. Structure:

```yaml
- name: "Rule Name"
  trigger_kind: Event|Schedule|StateChange
  event_type: "event_name"  # Optional; required for Event/Schedule
  action_kind: "GrantCredit"  # Or other action types
  amount: 50                  # Optional; used by GrantCredit/DeductCredit
  cooldown: "5m"              # Optional; parseable durations
  priority: 1                 # Optional; defaults to 1
  enabled: true               # Optional; defaults to true
  version: "1.0"              # Optional; for schema versioning
```

### YAML Rules Example

```yaml
- name: "Morning focus bonus"
  trigger_kind: "Schedule"
  event_type: "daily_6am"
  action_kind: "GrantCredit"
  amount: 75
  cooldown: "24h"
  priority: 1
  enabled: true
  version: "1.0"

- name: "Afternoon slump recovery"
  trigger_kind: "StateChange"
  event_type: "low_focus_detected"
  action_kind: "Intervention"
  priority: 2
  enabled: true
```

## YAML Schema: Tasks

```yaml
- title: "Task Title"
  priority: 0.8              # Optional; [0.0, 1.0]
  deadline: "2026-04-30T..."  # Optional; ISO 8601
  duration_min: 120          # Optional; minutes
  tags:                       # Optional; list or null
    - tag1
    - tag2
  version: "1.0"             # Optional
```

### YAML Tasks Example

```yaml
- title: "Quarterly planning review"
  priority: 0.95
  deadline: "2026-05-15T17:00:00Z"
  duration_min: 180
  tags: ["planning", "quarterly", "strategic"]
  version: "1.0"

- title: "Refactor database queries"
  priority: 0.5
  duration_min: 240
  tags: ["refactor", "database"]
```

## Validation & Error Handling

### Validation Rules

- **Rules:** `name` cannot be empty; `trigger_kind` must be one of [Event, Schedule, StateChange]; `action_kind` must be a known action type.
- **Tasks:** `title` cannot be empty; `priority` must be in [0.0, 1.0] if provided.
- **Schema drift:** Extra fields are silently ignored (forward compatibility). Missing optional fields are acceptable.

### Error Reporting

When importing, the CLI reports:

```
Parsed 10 rules
Skipped 2 rows with errors:
  Row 5: trigger_kind - unknown trigger: InvalidTrigger
  Row 8: Record - Malformed CSV row: missing field
```

All valid rows are imported regardless of errors in other rows. Use `--dry-run` to preview and validate before committing.

## Importing from Competitor Apps

### From Apple Screen Time / iOS Focus modes

Export to CSV format using a third-party tool, then reformat to match FocalPoint schema:

```csv
name,trigger_kind,event_type,action_kind,amount,cooldown,priority,enabled
Apps Limit,Event,app_open,Block,,,2,true
Bedtime Mode,Schedule,bedtime_10pm,Block,,,3,true
```

### From Freedom / Cold Turkey

Export to JSON, convert to YAML or CSV:

```bash
# Convert Freedom JSON export to CSV
cat freedom-export.json | jq -r '.rules[] | [.name, "Event", .event_type, .action, .amount] | @csv' > rules.csv
```

### From Toggl / Timing

Time-tracking data can populate task duration and tags:

```bash
# Export tasks from time tracker
cat timing-export.json | jq -r '.entries[] | [.title, .priority, .deadline, .duration_minutes, .tags | join(",")] | @csv' > tasks.csv
```

## Migration Workflow

1. **Export from source:** Use the tool's export function to CSV or JSON.
2. **Transform:** If necessary, map columns to FocalPoint schema.
3. **Validate:** Test with `--dry-run`:
   ```bash
   focus rules import converted-rules.csv --dry-run
   ```
4. **Import:** If validation passes, import without `--dry-run`:
   ```bash
   focus rules import converted-rules.csv
   ```
5. **Verify:** List and spot-check:
   ```bash
   focus rules list
   focus tasks list
   ```

## Troubleshooting

### "Unknown trigger: InvalidTrigger"

**Problem:** CSV/YAML contains a trigger type not recognized.

**Solution:** Use one of: `Event`, `Schedule`, `StateChange`. Check spelling and capitalization.

### "Priority must be in range [0.0, 1.0]"

**Problem:** Task priority outside valid range.

**Solution:** Clamp priority to [0.0, 1.0]. Example: priority 0.8 is valid; 1.5 is not.

### "Name cannot be empty"

**Problem:** A rule or task title is blank or missing.

**Solution:** Ensure every row has a non-empty `name` (rules) or `title` (tasks).

### Dry-run shows errors but no rules imported

**Problem:** Validation failed for all rows.

**Solution:**
1. Check file format (is it valid CSV/YAML?).
2. Verify column headers match schema.
3. Inspect the first few rows manually.
4. Try `focus rules export --format=csv --output=template.csv` to see valid output format.

### Rows silently skipped

**Problem:** Some CSV rows are ignored with no error message.

**Solution:** CSV parsing is lenient; check the validation report. Use `--json` for machine-readable error details:

```bash
focus rules import rules.csv --json 2>&1 | jq '.validation_report.errors'
```

## Examples

See:
- `examples/bulk/sample-rules.csv` — 10 example rules
- `examples/bulk/sample-tasks.yaml` — 10 example tasks
- `examples/bulk/everything.yaml` — 20 mixed entries

Run:

```bash
focus rules import examples/bulk/sample-rules.csv --dry-run
focus tasks import examples/bulk/sample-tasks.yaml --dry-run
```

## Limits & Performance

- **Import limits:** No hard limit; tested with 1000+ rules/tasks.
- **Export:** Memory-efficient streaming; suitable for large exports.
- **Validation:** All rows validated before any import (atomic semantics per row, not transactional).

## Related

- `focus rules list` — View imported rules.
- `focus tasks list` — View imported tasks.
- `focus rules upsert --file <rule.json>` — Import single rule from JSON.
