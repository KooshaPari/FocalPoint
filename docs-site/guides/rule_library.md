# FocalPoint Rule Library

Ready-to-use rules for common focus management scenarios. Each rule is available in three formats: CLI command, FPL (Starlark), and Rule Builder JSON graph.

## Quick Start

### Option 1: CLI (Fastest)

Copy and paste any rule's `cli.sh` command:

```bash
bash examples/rule-library/gh-pr-merged/cli.sh
```

### Option 2: FPL (Starlark)

Compile and upsert a rule from FPL source:

```bash
focus rules upsert examples/rule-library/canvas-submit/rule.fpl
```

### Option 3: Rule Builder UI

1. Open the Rule Builder in the iOS app or web dashboard
2. Click "Import from JSON"
3. Copy the contents of `examples/rule-library/<rule-id>/graph.json`
4. Paste and review

---

## Rules

### 1. GitHub PR Merged

Reward commits and code reviews.

**Trigger:** GitHub PR merged event
**Action:** +10 credits
**Difficulty:** Beginner

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "gh-pr-merged",
  "name": "GitHub PR Merged",
  "body": {
    "kind": "Rule",
    "id": "gh-pr-merged",
    "name": "GitHub PR Merged",
    "trigger": {"type": "EventFired", "value": {"event_name": "github:pr_merged"}},
    "conditions": [],
    "actions": [{"type": "apply_mutation", "mutation_id": "grant_credits", "params": {"amount": 10}}],
    "priority": 50,
    "cooldown_seconds": null,
    "duration_seconds": null,
    "explanation_template": "GitHub PR merged: +10 credits",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="gh-pr-merged",
    name="GitHub PR Merged",
    trigger=on_event("github:pr_merged"),
    conditions=[],
    actions=[grant_credit(amount=10)],
    priority=50
)
```

---

### 2. Canvas Assignment Submitted

Celebrate coursework progress.

**Trigger:** Canvas assignment submitted event
**Action:** +20 credits + increment assignments streak
**Difficulty:** Beginner

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "canvas-submit",
  "name": "Canvas Assignment Submitted",
  "body": {
    "kind": "Rule",
    "id": "canvas-submit",
    "name": "Canvas Assignment Submitted",
    "trigger": {"type": "EventFired", "value": {"event_name": "canvas:assignment_submitted"}},
    "conditions": [],
    "actions": [
      {"type": "apply_mutation", "mutation_id": "grant_credits", "params": {"amount": 20}},
      {"type": "apply_mutation", "mutation_id": "increment_streak", "params": {"streak_name": "assignments"}}
    ],
    "priority": 50,
    "cooldown_seconds": null,
    "duration_seconds": null,
    "explanation_template": "Canvas assignment submitted: +20 credits + streak",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="canvas-submit",
    name="Canvas Assignment Submitted",
    trigger=on_event("canvas:assignment_submitted"),
    conditions=[],
    actions=[
        grant_credit(amount=20),
        streak_increment(streak_id="assignments")
    ],
    priority=50
)
```

---

### 3. Google Calendar Deep Work Session

Reward uninterrupted focus time.

**Trigger:** Google Calendar event ended with title "Deep Work"
**Action:** Emit session-completed event
**Difficulty:** Intermediate

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "gcal-deep-work-end",
  "name": "Calendar Deep Work Session End",
  "body": {
    "kind": "Rule",
    "id": "gcal-deep-work-end",
    "name": "Calendar Deep Work Session End",
    "trigger": {"type": "EventFired", "value": {"event_name": "gcal:event_ended"}},
    "conditions": [{"op": "event_property", "property": "event_title", "expected": "Deep Work"}],
    "actions": [{"type": "emit_event", "event_type": "session-completed", "payload": {}}],
    "priority": 60,
    "cooldown_seconds": null,
    "duration_seconds": null,
    "explanation_template": "Deep Work session completed",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="gcal-deep-work-end",
    name="Calendar Deep Work Session End",
    trigger=on_event("gcal:event_ended"),
    conditions=[payload_eq(path="event_title", value="Deep Work")],
    actions=[],
    priority=60
)
```

---

### 4. Fitbit Workout Logged

Motivate exercise streaks.

**Trigger:** Fitbit workout logged event
**Action:** +30 credits + increment workouts streak
**Difficulty:** Intermediate

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "fitbit-workout",
  "name": "Fitbit Workout Logged",
  "body": {
    "kind": "Rule",
    "id": "fitbit-workout",
    "name": "Fitbit Workout Logged",
    "trigger": {"type": "EventFired", "value": {"event_name": "fitbit:workout_logged"}},
    "conditions": [],
    "actions": [
      {"type": "apply_mutation", "mutation_id": "grant_credits", "params": {"amount": 30}},
      {"type": "apply_mutation", "mutation_id": "increment_streak", "params": {"streak_name": "workouts"}}
    ],
    "priority": 55,
    "cooldown_seconds": null,
    "duration_seconds": null,
    "explanation_template": "Workout logged: +30 credits + streak",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="fitbit-workout",
    name="Fitbit Workout Logged",
    trigger=on_event("fitbit:workout_logged"),
    conditions=[],
    actions=[
        grant_credit(amount=30),
        streak_increment(streak_id="workouts")
    ],
    priority=55
)
```

---

### 5. Morning Brief Incomplete by Noon

Soft nudge for morning ritual completion.

**Trigger:** Daily at noon (0 12 * * *)
**Condition:** morning_brief_complete attribute is false
**Action:** Soft nudge notification
**Difficulty:** Intermediate

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "morning-brief-nudge",
  "name": "Morning Brief Incomplete by Noon",
  "body": {
    "kind": "Rule",
    "id": "morning-brief-nudge",
    "name": "Morning Brief Incomplete by Noon",
    "trigger": {"type": "ScheduleCron", "value": {"cron_expression": "0 12 * * *", "timezone": "UTC"}},
    "conditions": [{"op": "user_attribute", "key": "morning_brief_complete", "value": "false"}],
    "actions": [{"type": "show_notification", "notification_id": "nudge-brief", "text": "Morning brief incomplete. Take a moment to review.", "duration_ms": 5000}],
    "priority": 40,
    "cooldown_seconds": 3600,
    "duration_seconds": null,
    "explanation_template": "Soft nudge: morning brief incomplete",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="morning-brief-nudge",
    name="Morning Brief Incomplete by Noon",
    trigger=on_schedule("0 12 * * *"),
    conditions=[payload_eq(path="morning_brief_complete", value=False)],
    actions=[notify(message="Morning brief incomplete. Take a moment to review.")],
    priority=40,
    cooldown_seconds=3600
)
```

---

### 6. 3-Session Streak Celebration

Celebrate consistency milestones.

**Trigger:** session_streak state changes to 3+
**Action:** Celebration notification + 10 bonus credits
**Difficulty:** Intermediate

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "3-session-streak",
  "name": "3-Session Streak Celebration",
  "body": {
    "kind": "Rule",
    "id": "3-session-streak",
    "name": "3-Session Streak Celebration",
    "trigger": {"type": "ConditionMet", "value": {"condition": {"op": "user_attribute", "key": "session_streak", "value": "3"}}},
    "conditions": [],
    "actions": [
      {"type": "show_notification", "notification_id": "streak-celebration", "text": "Congratulations on 3 consecutive sessions!", "duration_ms": 5000},
      {"type": "apply_mutation", "mutation_id": "grant_credits", "params": {"amount": 10}}
    ],
    "priority": 70,
    "cooldown_seconds": null,
    "duration_seconds": null,
    "explanation_template": "3-session streak: +10 bonus credits",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="3-session-streak",
    name="3-Session Streak Celebration",
    trigger=on_state_change("session_streak"),
    conditions=[payload_eq(path="session_streak", value=3)],
    actions=[
        notify(message="Congratulations on 3 consecutive sessions!"),
        grant_credit(amount=10)
    ],
    priority=70
)
```

---

### 7. Missed Focus 2 Days in a Row

Concerned intervention for patterns.

**Trigger:** focus:session_missed event
**Condition:** consecutive_missed_sessions count = 2
**Action:** Concerned intervention notification
**Difficulty:** Advanced
**Gap:** Detecting 2 consecutive misses requires host-side state tracking. This rule fires on the second miss; the app must update `consecutive_missed_sessions` in the event payload.

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "missed-focus-2x",
  "name": "Missed Scheduled Focus 2 Days in a Row",
  "body": {
    "kind": "Rule",
    "id": "missed-focus-2x",
    "name": "Missed Scheduled Focus 2 Days in a Row",
    "trigger": {"type": "EventFired", "value": {"event_name": "focus:session_missed"}},
    "conditions": [{"op": "user_attribute", "key": "consecutive_missed_sessions", "value": "2"}],
    "actions": [{"type": "show_notification", "notification_id": "concerned-intervention", "text": "You have missed focus sessions for 2 consecutive days. Let's talk about barriers.", "duration_ms": 8000}],
    "priority": 80,
    "cooldown_seconds": 86400,
    "duration_seconds": null,
    "explanation_template": "Concerned intervention: 2 consecutive missed sessions",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="missed-focus-2x",
    name="Missed Scheduled Focus 2 Days in a Row",
    trigger=on_event("focus:session_missed"),
    conditions=[payload_eq(path="consecutive_missed_count", value=2)],
    actions=[notify(message="You have missed focus sessions for 2 consecutive days. Let's talk about barriers.")],
    priority=80,
    cooldown_seconds=86400
)
```

---

### 8. Canvas Assignment Due in 24h

Proactive assignment reminders.

**Trigger:** Canvas assignment_due event (24 hours warning)
**Action:** Notification + optional task creation
**Difficulty:** Intermediate

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "canvas-due-24h",
  "name": "Canvas Assignment Due in 24h",
  "body": {
    "kind": "Rule",
    "id": "canvas-due-24h",
    "name": "Canvas Assignment Due in 24h",
    "trigger": {"type": "EventFired", "value": {"event_name": "canvas:assignment_due"}},
    "conditions": [{"op": "event_property", "property": "hours_until_due", "expected": 24}],
    "actions": [
      {"type": "show_notification", "notification_id": "canvas-due-24h", "text": "Canvas assignment due in 24 hours!", "duration_ms": 5000},
      {"type": "schedule_task", "task_id": "add_canvas_task", "delay_ms": null, "params": {}}
    ],
    "priority": 75,
    "cooldown_seconds": null,
    "duration_seconds": null,
    "explanation_template": "Canvas due soon: notify and task",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="canvas-due-24h",
    name="Canvas Assignment Due in 24h",
    trigger=on_event("canvas:assignment_due"),
    conditions=[payload_eq(path="hours_until_due", value=24)],
    actions=[notify(message="Canvas assignment due in 24 hours!")],
    priority=75
)
```

---

### 9. Strava Personal Record

Celebrate athletic achievements.

**Trigger:** Strava segment PR event
**Action:** +50 credits + celebrate notification
**Difficulty:** Intermediate

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "strava-pr",
  "name": "Strava Personal Record",
  "body": {
    "kind": "Rule",
    "id": "strava-pr",
    "name": "Strava Personal Record",
    "trigger": {"type": "EventFired", "value": {"event_name": "strava:segment_pr"}},
    "conditions": [],
    "actions": [
      {"type": "apply_mutation", "mutation_id": "grant_credits", "params": {"amount": 50}},
      {"type": "show_notification", "notification_id": "strava-celebrate", "text": "Amazing! You set a new personal record on Strava!", "duration_ms": 6000}
    ],
    "priority": 85,
    "cooldown_seconds": null,
    "duration_seconds": null,
    "explanation_template": "Strava PR: +50 credits + celebrate",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="strava-pr",
    name="Strava Personal Record",
    trigger=on_event("strava:segment_pr"),
    conditions=[],
    actions=[
        grant_credit(amount=50),
        notify(message="Amazing! You set a new personal record on Strava!")
    ],
    priority=85
)
```

---

### 10. Strict Mode After 10pm on Weekday

Enforce evening boundaries.

**Trigger:** Cron: 10pm weekdays (0 22 * * 1-5)
**Action:** Enforce strict block until 7am
**Difficulty:** Advanced
**Gap:** Standard cron syntax supported; Rule Builder UI lacks native weekday picker widget. Graph JSON encodes the cron string; UI renders as text field.

**CLI:**
```bash
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "weeknight-strict-block",
  "name": "Strict Mode After 10pm on Weekday",
  "body": {
    "kind": "Rule",
    "id": "weeknight-strict-block",
    "name": "Strict Mode After 10pm on Weekday",
    "trigger": {"type": "ScheduleCron", "value": {"cron_expression": "0 22 * * 1-5", "timezone": "UTC"}},
    "conditions": [{"op": "day_of_week", "days": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]}],
    "actions": [{"type": "enforce_policy", "policy_id": "strict_block", "params": {"until_hour": 7}}],
    "priority": 100,
    "cooldown_seconds": null,
    "duration_seconds": 32400,
    "explanation_template": "Strict block active: weekday 10pm-7am",
    "enabled": true
  }
}'
```

**FPL:**
```starlark
rule(
    id="weeknight-strict-block",
    name="Strict Mode After 10pm on Weekday",
    trigger=on_schedule("0 22 * * 1-5"),
    conditions=[],
    actions=[block(profile="strict", duration_seconds=32400, rigidity="hard")],
    priority=100
)
```

---

## Surface Gaps

### FPL (Starlark)

FPL currently lacks support for:
- Custom condition helpers like `user_attribute()` and `event_property()` (must use `payload_eq()`, `payload_gte()`, etc.)
- Emit/notify actions (expected in future helpers)
- Full weekday condition syntax (use cron in trigger instead)

**Workaround:** Use CLI or Rule Builder JSON for rules requiring these primitives.

### Rule Builder Graph JSON

The Rule Builder UI lacks native widgets for:
- Weekday picker (cron with dow fields). Graph encodes as text; UI renders as text input.
- Multi-option selectors for intervention severity levels

**Workaround:** Edit the JSON directly for complex cron expressions or use the CLI.

---

## Testing

All rules verify IR parity across CLI and Graph surfaces:

```bash
cd examples/rule-library/tests
cargo run --bin parity_test
```

Output:
```
Rule Library Parity Test
========================

  gh-pr-merged ✓ ir_hash match
  canvas-submit ✓ ir_hash match
  [... 8 more ...]

Result: PASS
```

---

## Customization

Each rule is a template. Common customizations:

### Change Credit Amounts

Edit the `"amount"` in the `apply_mutation` action:

```json
{"type": "apply_mutation", "mutation_id": "grant_credits", "params": {"amount": 25}}
```

### Change Notification Text

Edit the `"text"` field:

```json
{"type": "show_notification", "text": "Your custom message", "duration_ms": 5000}
```

### Adjust Priority

Rules with higher priority fire first. Valid range: 0-100.

```json
"priority": 80
```

### Add Cooldown

Prevent the rule from firing repeatedly within a window (seconds):

```json
"cooldown_seconds": 3600
```

---

## Next Steps

1. Choose a rule from the library
2. Copy its JSON, FPL, or CLI version
3. Customize trigger/conditions/actions as needed
4. Test via `focus rules validate` before deployment
5. Monitor outcomes and adjust credit amounts or cooldowns based on user behavior
