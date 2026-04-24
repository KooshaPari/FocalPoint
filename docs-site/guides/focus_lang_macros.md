# FPL Macro Library Reference

High-level, 1-liner macros for common focus-lang patterns. Each macro lowers to standard IR without introducing new types.

## Overview

The 8 core macros cover reward/penalty systems, scheduling, enforcement, and streak tracking:

| Macro | Purpose | Returns |
|-------|---------|---------|
| `reward()` | Grant credits + streak | Rule |
| `penalize()` | Deduct credits | Rule |
| `remind()` | Cron-scheduled notification | Rule |
| `celebrate()` | Celebration with sound/notify | Rule |
| `block()` | Enforce app blocking | EnforcementPolicy |
| `unlock_after()` | Conditional unlock with timeout | Rule |
| `track_streak()` | Increment named streak | Rule |
| `if_pattern()` | Named condition patterns | Condition list |

---

## `reward(event, credits=10, streak=True)`

Grant credits and optionally track a streak on an event.

### Signature
```python
reward(
    event: str,          # Event name (e.g., "focus:session_completed")
    credits: int = 10,   # Credits to grant (default: 10)
    streak: bool = True  # Track streak (default: True)
) -> Rule
```

### Expands to
- `rule()` with `grant_credit(credits)` action
- If `streak=True`: adds `streak_increment(event_streak_id)` action
- Priority: 50

### Examples

**Reward 15 credits + 1 streak point:**
```python
# Before (verbose)
rule(
    id="reward_session",
    name="Session Reward",
    trigger=on_event("focus:session_completed"),
    conditions=[],
    actions=[grant_credit(15), streak_increment("session_streak")],
    priority=50,
    enabled=1
)

# After (macro)
reward("focus:session_completed", credits=15, streak=True)
```

**Reward only credits (no streak):**
```python
reward("milestone:reached", credits=25, streak=False)
```

---

## `penalize(event, credits=5)`

Deduct credits as a penalty on an event.

### Signature
```python
penalize(
    event: str,        # Event name
    credits: int = 5   # Credits to deduct (default: 5)
) -> Rule
```

### Expands to
- `rule()` with `deduct_credit(credits)` action
- Priority: 40
- Automatically generates unique rule ID

### Examples

**Penalize distraction:**
```python
# Verbose
rule(
    id="penalize_distraction",
    name="Distraction penalty",
    trigger=on_event("distraction:triggered"),
    conditions=[],
    actions=[deduct_credit(8)],
    priority=40,
    enabled=1
)

# Macro
penalize("distraction:triggered", credits=8)
```

---

## `remind(every, message, at="UTC")`

Schedule a cron-based reminder with notification.

### Signature
```python
remind(
    every: str,         # Cron expression (e.g., "0 9 * * MON")
    message: str,       # Notification text
    at: str = "UTC"     # Timezone (e.g., "America/New_York")
) -> Rule
```

### Expands to
- `rule()` with `on_schedule(cron)` trigger
- Action: `notify(message)`
- Priority: 30

### Cron Format
```
"0 9 * * *"       → Daily at 9am
"0 9 * * MON"     → Mondays at 9am
"0 */4 * * *"     → Every 4 hours
"0 0 1 * *"       → First day of month
```

### Examples

**Remind every Monday morning:**
```python
remind("0 9 * * MON", "Weekly planning time", at="America/New_York")
```

**Daily 3pm sync:**
```python
remind("0 15 * * *", "Team sync in 5 min", at="UTC")
```

---

## `celebrate(event, message, sound="confetti")`

Celebratory notification with optional mascot scene and sound cue.

### Signature
```python
celebrate(
    event: str,           # Event to celebrate
    message: str,         # Celebration message
    sound: str = "confetti"  # Sound cue name
) -> Rule
```

### Expands to
- `rule()` with `on_event(event)` trigger
- Action: `notify(message)`
- Priority: 60
- (Future: sound cue + mascot scene integration)

### Examples

**Celebrate weekly goal:**
```python
celebrate(
    "milestone:weekly_goal_met",
    "🎉 Weekly goal achieved!",
    sound="confetti"
)
```

**Quick win celebration:**
```python
celebrate("task:completed_early", "Nice work!", sound="ding")
```

---

## `block(app_list, during_schedule)`

Enforce app blocking policy during a named schedule.

### Signature
```python
block(
    app_list: str | list[str],  # App(s) to block
    during_schedule: str         # Schedule/profile name
) -> EnforcementPolicy
```

### Expands to
- `enforcement()` with `rigidity="hard"`
- Targets = `app_list`

### Examples

**Block social during work:**
```python
block("Instagram", "work_hours")

# or with list
block(["Instagram", "TikTok", "Twitter"], "work_hours")
```

**Evening entertainment block:**
```python
block(["YouTube", "Netflix"], "evening_6pm_to_midnight")
```

---

## `unlock_after(condition, duration_hours)`

Conditional unlock: restore access after a state change or timeout.

### Signature
```python
unlock_after(
    condition: str,         # Condition to wait for (state change path)
    duration_hours: float   # Max wait time (hours)
) -> Rule
```

### Expands to
- `rule()` with `on_state_change(condition)` trigger
- Action: `unblock("social")` (or relevant profile)
- `duration_seconds = duration_hours * 3600`
- Priority: 70

### Examples

**Unlock after task completion (2-hour timeout):**
```python
unlock_after("daily:task_list_cleared", 2)
```

**Unlock after workout (1-hour):**
```python
unlock_after("workout:session_completed", 1)
```

---

## `track_streak(event, name)`

Track a named streak: increment on event + optional notification.

### Signature
```python
track_streak(
    event: str,  # Event triggering streak increment
    name: str    # Human-readable streak name
) -> Rule
```

### Expands to
- `rule()` with `on_event(event)` trigger
- Actions:
  - `notify(f"Streak continued: {name}")`
  - `streak_increment(name_normalized)`
- Priority: 55

### Examples

**Daily focus streak:**
```python
track_streak("focus:session_ended", "Daily Focus")
```

**Fitness consistency:**
```python
track_streak("workout:completed", "Fitness Chain")
```

---

## `if_pattern(pattern_name, conditions_list=None)`

Named pattern matcher for common temporal/contextual patterns.

### Signature
```python
if_pattern(
    pattern_name: str,        # Pattern: "weekday", "evening", "work_hours"
    conditions_list: list = None  # Override with custom conditions
) -> list[Condition]
```

### Built-in Patterns
- `"weekday"` — Monday to Friday
- `"evening"` — After 5pm (context-dependent)
- `"work_hours"` — 9am-5pm (context-dependent)

### Expands to
- Returns a condition list (does not emit a rule directly)
- Use with other rules for advanced filtering

### Examples

**Use weekday pattern:**
```python
weekday_conds = if_pattern("weekday")

rule(
    id="weekday_focus",
    name="Weekday Focus Session",
    trigger=on_event("focus:started"),
    conditions=weekday_conds,
    actions=[notify("Let's focus!")],
    enabled=1
)
```

**Custom pattern:**
```python
custom_conds = if_pattern("business_hours", [
    payload_gte("hour_of_day", 8),
    payload_lte("hour_of_day", 18)
])
```

---

## Complete Example

Here's a 40-line policy using all 8 macros:

```python
# Rewards & Penalties
reward("focus:session_completed", credits=15, streak=True)
penalize("distraction:triggered", credits=8)

# Scheduled Reminders
remind("0 9 * * MON", "Weekly planning", at="America/New_York")
remind("0 18 * * *", "Evening shutdown", at="UTC")

# Celebrations
celebrate("milestone:weekly_goal_met", "🎉 Weekly goal!", sound="confetti")

# Enforcement
block(["Instagram", "TikTok"], "work_hours")

# Unlock Conditional
unlock_after("daily:tasks_cleared", 1)

# Streak Tracking
track_streak("workout:completed", "Fitness Chain")

# Pattern Conditions
work_hours = if_pattern("work_hours")
weekdays = if_pattern("weekday")
```

---

## Implementation Notes

- **No new IR types**: Macros expand using existing primitives (`rule()`, `enforcement()`, etc.)
- **Stable IDs**: Generated rule IDs are deterministic and collision-free
- **Default priorities**: reward=50, penalize=40, remind=30, celebrate=60, unlock=70, track=55
- **JSON-safe**: All expansions produce valid, serializable IR

---

## Testing

All macros are covered by 15+ unit tests in `crates/focus-lang/src/macros/tests.rs`:
- Expansion correctness
- Parameter validation
- IR stability across compiles
- JSON round-tripping
- Multi-macro compilation
