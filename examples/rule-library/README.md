# Rule Library

10 ready-to-copy rules authored across all 3 surfaces (CLI, FPL, Rule Builder graph JSON).

Each rule is indexed below with its trigger, action, and difficulty. All rules are verified to compile to identical IR via the parity test (`tests/parity_test.rs`).

## Index

| ID | Name | Trigger | Action | Difficulty | Status |
|---|---|---|---|---|---|
| `gh-pr-merged` | GitHub PR Merged | Event: `github:pr_merged` | +10 credits | Beginner | ✅ |
| `canvas-submit` | Canvas Assignment Submitted | Event: `canvas:assignment_submitted` | +20 credits + streak | Beginner | ✅ |
| `gcal-deep-work-end` | Calendar Deep Work Session End | Event: `gcal:event_ended` (filter: title="Deep Work") | Emit `session-completed` | Intermediate | ✅ |
| `fitbit-workout` | Fitbit Workout Logged | Event: `fitbit:workout_logged` | +30 credits + streak | Intermediate | ✅ |
| `morning-brief-nudge` | Morning Brief Incomplete by Noon | Schedule: `0 12 * * *` | Soft nudge intervention | Intermediate | ✅ |
| `3-session-streak` | 3-Session Streak Celebration | Condition: streak count ≥3 | Gentle intervention + 10 bonus credits | Intermediate | ✅ |
| `missed-focus-2x` | Missed Scheduled Focus 2 Days in a Row | Condition: `focus:session_missed` 2x consecutive | Concerned intervention | Advanced | ⚠️ |
| `canvas-due-24h` | Canvas Assignment Due in 24h | Event: `canvas:assignment_due` | Notify + add task if absent | Intermediate | ✅ |
| `strava-pr` | Strava Personal Record | Event: `strava:segment_pr` | +50 credits + celebrate intervention | Intermediate | ✅ |
| `weekday-after-10pm` | Strict Mode After 10pm on Weekday | Schedule: `0 22 * * 1-5` | Enforce strict block until 7am | Advanced | ⚠️ |

**Legend:**
- ✅ Fully expressible in all 3 surfaces with no gaps
- ⚠️ Gap identified (documented below)

---

## Surface Gaps & Notes

### Rule `missed-focus-2x`: Stateful Condition Across Events

**Gap**: Detecting "2 consecutive missed sessions" requires temporal state (did the user miss yesterday AND today?). 

**Current limitation**: FPL and Rule Builder cannot natively track cross-event state. Workaround: use a condition that fires on every missed event, then delegate state tracking to the host application (record a counter in `wallet.streaks` or a temporary flag).

**Status**: Can express the rule action, but condition requires external coordination.

### Rule `weekday-after-10pm`: Cron with Weekday Guards

**Gap**: Standard cron syntax (`0 22 * * 1-5`) is supported in CLI/FPL, but Rule Builder graph JSON lacks a native weekday-picker widget. 

**Current limitation**: Graph JSON can encode the cron string, but the UI must render it as a text field rather than a calendar picker.

**Status**: Compiles identically across all 3; UI UX is the limitation.

---

## Folder Structure

```
rule-library/
  README.md                    # This file
  gh-pr-merged/
    cli.sh                     # focus rules add --json ... with justification comment
    rule.fpl                   # Starlark source
    graph.json                 # ReactFlow graph (compatible with Rule Builder persistence)
  canvas-submit/
    cli.sh
    rule.fpl
    graph.json
  [... 8 more rules ...]
  tests/
    Cargo.toml                 # Standalone test binary
    src/
      main.rs                  # IR-hash parity test for all 10 rules
```

---

## How to Use

### Option 1: CLI (focus-cli)

```bash
#!/bin/bash
# Copy-paste the entire content of rule-library/<rule-id>/cli.sh
bash rule-library/gh-pr-merged/cli.sh
```

### Option 2: FPL (Starlark)

```bash
# Use the CLI upsert command:
focus rules upsert rule-library/gh-pr-merged/rule.fpl
```

### Option 3: Rule Builder (UI)

1. Open the Rule Builder in the iOS app or web dashboard.
2. Copy the JSON from `rule-library/<rule-id>/graph.json`.
3. Paste into the Rule Builder's "Import from JSON" dialog.
4. Review trigger, conditions, actions, and save.

---

## Parity Testing

All rules are verified to compile to identical IR across all 3 surfaces via the test binary:

```bash
cargo test --manifest-path examples/rule-library/tests/Cargo.toml
```

**Output example:**
```
test result: ok. 10 passed; 0 failed.
  - gh-pr-merged: ir_hash match ✓
  - canvas-submit: ir_hash match ✓
  [... 8 more ...]
```

If any rule shows `ir_hash mismatch`, the test will fail with a detailed diff of the IR.

---

## Implementation Notes

### CLI Format

The `.sh` files are ≤5-line wrappers calling `focus rules upsert`. Bash is acceptable here because:
1. They are copy-paste examples, not part of the build system.
2. They serve as human-readable documentation of the command-line API.
3. A real CLI user would naturally use shell to invoke the tool.

### FPL (Starlark)

FPL uses a Starlark-like syntax (similar to Bazel BUILD files). The transpiler converts FPL → IR.

Built-in functions:
- `rule(id=..., name=..., priority=..., trigger=..., conditions=..., actions=..., ...)`
- `on_event(name)` — Trigger helper
- `on_schedule(cron)` — Trigger helper
- `on_state_change(key)` — Trigger helper
- `block(profile, duration_seconds, rigidity)` — Action helper
- `grant_credit(amount)` — Action helper
- `notify(message)` — Action helper
- `intervention(message, severity)` — Action helper
- `streak_increment(name)` — Action helper
- etc.

### Rule Builder Graph JSON

ReactFlow-compatible node/edge graph. Each primitive (trigger, condition, action) is a node. Edges connect them. The graph is serialized to JSON and stored in the database or imported via the Rule Builder UI.

**Node types:**
- `trigger` — Defines when the rule fires
- `condition` — Optional guards
- `action` — Effects (credits, notifications, blocks, etc.)
- `metadata` — Rule name, priority, cooldown, etc.

---

## Next Steps

1. Copy any rule's `cli.sh`, `rule.fpl`, or `graph.json` to your project.
2. Customize trigger, conditions, and actions as needed.
3. Run the parity test to verify roundtrip equality if you modify the FPL or graph.
4. Upsert to the database and enable via CLI or app.
