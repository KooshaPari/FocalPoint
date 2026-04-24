# Time-Travel Debugging with focus replay

The `focus replay` subcommand lets you evaluate an alternate ruleset against historical audit events **without mutating state**. Test new rules, debug why rules didn't fire, and understand rule interactions in post-mortem analysis.

## Use Cases

### Testing a new rule before enabling

Draft a new rule in `alternate.fpl`, then replay the last 7 days of events to see:
- How many times the new rule would fire
- What actions would be triggered
- How credit/streak deltas would differ from the baseline

### Debugging why a rule didn't fire

Modify a rule's conditions and replay a specific window to confirm:
- Whether looser conditions would have caught more events
- If timing or cooldown prevented fires
- How stricter conditions impact behavior

### Post-mortem analysis

After a rule misbehaves in production, load a snapshot of that ruleset and replay:
- Compare baseline (original) vs. alternate (suspected-buggy) ruleset
- Identify which actions caused unexpected credit deltas
- Quantify the behavioral impact

## CLI Interface

```bash
focus replay window \
  --since 2026-04-20T00:00:00Z \
  --until 2026-04-21T00:00:00Z \
  --rules alternate.fpl \
  --format markdown
```

### Flags

- `--since` (required): Window start (ISO 8601 format)
- `--until` (required): Window end (ISO 8601 format)
- `--rules` (required): Path to alternate ruleset file (FPL or TOML)
- `--format`: Output format (`markdown` or `json`, default: `markdown`)
- `--db` (global): Path to core.db (or `FOCALPOINT_DB` env var)

## Ruleset Formats

### TOML Format

```toml
[[rules]]
id = "550e8400-e29b-41d4-a716-446655440000"
name = "Daily bonus"
priority = 10
enabled = true
trigger = "Event:daily_checkin"
actions = [
  { GrantCredit = 50 }
]
```

### FPL Format (Focus Policy Language)

```
rule "Daily bonus" {
  trigger Event:daily_checkin
  action grant_credit(50)
  priority 10
}
```

## Output: Markdown Report

```markdown
# Time-Travel Replay Report

**Window:** 2026-04-20T00:00:00Z — 2026-04-21T00:00:00Z

## Baseline Ruleset

- Events Seen: 100
- Decisions Evaluated: 500
- Rules Fired: 42

## Alternate Ruleset

- Events Seen: 100
- Decisions Evaluated: 500
- Rules Fired: 45

## Differences

### Diff 1

**Rule Fire Count Changed:** 42 → 45

### Diff 2

**Action Delta (credit_delta): 1200 → 1350**
```

## Output: JSON Format

```json
{
  "window_start": "2026-04-20T00:00:00Z",
  "window_end": "2026-04-21T00:00:00Z",
  "baseline_report": {
    "events_seen": 100,
    "decisions": 500,
    "fired_decisions": 42,
    "action_deltas": {
      "credit_delta": 1200
    },
    "streak_changes": {}
  },
  "alternate_report": {
    "events_seen": 100,
    "decisions": 500,
    "fired_decisions": 45,
    "action_deltas": {
      "credit_delta": 1350
    },
    "streak_changes": {}
  },
  "diffs": [
    {
      "FiredDecisionDelta": {
        "baseline": 42,
        "alternate": 45
      }
    },
    {
      "ActionDelta": {
        "key": "credit_delta",
        "baseline": 1200,
        "alternate": 1350
      }
    }
  ]
}
```

## Example Workflow

1. **Draft a new rule:**
   ```bash
   cat > new_rule.fpl << 'EOF'
   rule "Weekend bonus" {
     trigger Event:app_open
     condition day_of_week(Sat, Sun)
     action grant_credit(10)
     priority 15
   }
   EOF
   ```

2. **Replay the last 7 days with the new rule:**
   ```bash
   focus replay window \
     --since "$(date -u -d '7 days ago' +%Y-%m-%dT%H:%M:%SZ)" \
     --until "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
     --rules new_rule.fpl \
     --format markdown
   ```

3. **Review the report:**
   - Would the rule fire? (check "Rules Fired" count)
   - How much credit would it grant? (check "action_deltas")
   - Is the behavior as expected?

4. **If satisfied, enable the rule:**
   ```bash
   focus rules upsert --from-file new_rule.fpl
   ```

## Implementation Notes

### No State Mutation

The replay engine:
- Loads events from the store (read-only)
- Instantiates a fresh `RuleEngine` per ruleset evaluation
- Does not write decisions, mutations, or audit records
- Does not update cursor positions

### Immutable Evaluation

Each event is evaluated against a rule independently:
- Rule cooldown is NOT carried forward between events
- State snapshots are not used for condition evaluation
- Streak/credit state is accumulated for the report only

### Performance

Typical replays (1K–10K events, 10–50 rules) complete in <500 ms. For larger windows, consider narrowing the `--since` / `--until` range.

## Traces

- **FR-REPLAY-001:** Identical rulesets produce empty diff
- **FR-REPLAY-002:** Added rules show new fires in diff
- **FR-REPLAY-003:** Modified actions show delta in diff
- **FR-REPLAY-004:** Zero-event windows are handled

---

See also: `focus rules list`, `focus rules upsert`, `focus audit tail`
