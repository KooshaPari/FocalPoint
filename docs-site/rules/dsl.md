---
title: Rules DSL Reference
description: Complete syntax for FocalPoint's rule definition language.
---

# Rules DSL Reference

This is the complete specification for FocalPoint's rule definition language (YAML-based).

## Rule Structure

```yaml
# Metadata
name: "Rule name"
description: "What does this rule do?"
enabled: true
version: "1.0.0"

# Triggering condition
trigger:
  - event_type: "canvas.assignment.due_soon"
    hours_until: 24

# Additional conditions (all must be true)
condition:
  - time_window:
      start: "08:00"
      end: "17:00"
  - not_focused: false  # Only if user is not in focus mode
  - weekday: [Monday, Tuesday, Wednesday, Thursday, Friday]

# Actions to execute
action:
  - block_app: "com.tiktok.main"
  - show_focus_view: "study"
  - log_audit: "Focus rule triggered"

# Optional: custom metadata
meta:
  tags: ["productivity", "student"]
  author: "alice"
```

## Triggers

Triggers are the starting event:

```yaml
trigger:
  # Connector event
  - event_type: "canvas.assignment.due_soon"
    # Optional event payload filters
    hours_until: 24

  # Time-based
  - schedule: "every weekday at 08:00"

  # Manual
  - user_initiated: true
```

## Conditions

Conditions refine the trigger. All must be true:

```yaml
condition:
  # Time windows
  - time_window:
      start: "08:00"
      end: "23:59"

  # Day of week
  - weekday: [Monday, Tuesday]

  # Calendar blocking
  - not_in_event: true

  # Device state
  - battery_above: 20
  - not_on_charger: false  # True if NOT on charger
  - screen_on: true

  # App state
  - app_in_foreground: "com.canvas.canvas"
  - not_in_focus: false  # True if NOT in focus mode

  # Audit state
  - streak_active: true
  - wallet_balance_above: 100
```

## Actions

Actions execute when all conditions match:

```yaml
action:
  # App control
  - block_app:
      - "com.tiktok.main"
      - "com.instagram.android"
  - block_category: ["Games", "Social Media"]
  - whitelist_app: "com.slack"

  # Focus modes
  - show_focus_view: "study"  # study, deep-work, wellness, custom
  - set_focus_duration: "90 minutes"
  - schedule_break: "15 minutes"

  # Notifications
  - send_notification:
      title: "Focus Mode Active"
      body: "Canvas assignment due in {{hours}} hours"
  - mute_notifications: true

  # Coaching
  - show_coaching:
      message: "Let's get focused!"
      options: ["Start timer", "Cancel"]
  - coach_message: "Nice work!"

  # Logging
  - log_audit: "User started focus session"
  - record_event:
      type: "focus_start"
      duration_minutes: 90

  # Wallet
  - add_reward: 25
  - deduct_penalty: 10
  - reset_streak: false
```

## Variable Interpolation

Rules support event payload interpolation:

```yaml
action:
  - show_notification:
      title: "{{event.assignment_title}} due soon"
      body: "Due in {{event.hours_until_due}} hours"
```

Available variables:

- `{{event.*}}` — Event payload fields
- `{{now}}` — Current time
- `{{user.name}}` — User profile name
- `{{wallet.balance}}` — Current wallet balance
- `{{streak.days}}` — Current streak count

## Comments

```yaml
# This is a comment
name: "My rule"  # Inline comment
```

## YAML Validation

Validate your rule:

```bash
focalpoint rule validate my-rule.yaml
```

## DSL Limits

For performance:

- Max 10 conditions per rule
- Max 15 actions per rule
- Max 100 rules per user (default)

Contact support for limit increases.
