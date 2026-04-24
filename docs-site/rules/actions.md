---
title: Action Catalogue
description: All available actions for rules.
---

# Action Catalogue

This document lists every action type available in FocalPoint rules.

## App Control

### block_app
Block one or more apps from launching.

```yaml
action:
  - block_app:
      - "com.tiktok.main"
      - "com.instagram.android"
  - block_message: "This app is blocked. Focus Mode is active."
```

### block_category
Block all apps in a category (Social Media, Games, etc.).

```yaml
action:
  - block_category: ["Games", "Social Media"]
```

### whitelist_app
Explicitly allow an app when others are blocked.

```yaml
action:
  - whitelist_app:
      - "com.slack"
      - "com.gmail"
```

## Focus Modes

### show_focus_view
Activate a focus mode view.

```yaml
action:
  - show_focus_view: "study"  # study, deep-work, wellness, custom
```

### set_focus_duration
Set duration for current focus session.

```yaml
action:
  - set_focus_duration: "90 minutes"
```

### schedule_break
Schedule a break and set duration.

```yaml
action:
  - schedule_break: "15 minutes"
```

## Notifications & Coaching

### send_notification
Send a user notification.

```yaml
action:
  - send_notification:
      title: "Focus Mode Active"
      body: "Keep going! You're on a 5-day streak!"
      sound: true
      vibrate: true
```

### coach_message
Emit a coaching message.

```yaml
action:
  - coach_message:
      message: "You're crushing it! 2 more hours?"
      emotion: "encouraging"  # encouraging, warning, neutral
```

### show_coaching
Show coaching UI with user choices.

```yaml
action:
  - show_coaching:
      message: "Ready to focus?"
      options:
        - "Start 90-min session"
        - "Take a break first"
        - "Dismiss"
```

### mute_notifications
Silence all notifications.

```yaml
action:
  - mute_notifications: true
```

## Logging & Audit

### log_audit
Add an entry to the audit log.

```yaml
action:
  - log_audit: "Focus rule triggered by Canvas deadline"
```

### record_event
Emit a custom event.

```yaml
action:
  - record_event:
      type: "focus_session_start"
      duration_minutes: 90
      trigger: "canvas_deadline"
```

## Wallet & Rewards

### add_reward
Award points to user wallet.

```yaml
action:
  - add_reward: 25  # Points
```

### deduct_penalty
Deduct points from wallet.

```yaml
action:
  - deduct_penalty: 10
```

### reset_streak
Reset or maintain streak.

```yaml
action:
  - reset_streak: false  # false = keep streak
```

## Scheduling

### schedule_ritual
Schedule a future ritual.

```yaml
action:
  - schedule_ritual:
      ritual_id: "morning_brief"
      at: "next_morning_06:00"
```

### schedule_check_in
Schedule a user check-in.

```yaml
action:
  - schedule_check_in:
      delay: "2 hours"
      message: "How was your focus session?"
```

## Custom Actions

### execute_script
Run a custom action (advanced).

```yaml
action:
  - execute_script:
      lang: "lua"  # Lua, Wasm, or native
      code: "wallet.add_reward(25)"
```

## Action Ordering

Actions execute in declaration order. Example:

```yaml
action:
  - block_app: "com.tiktok.main"  # First: block the app
  - show_focus_view: "study"      # Second: show focus UI
  - log_audit: "Focus started"     # Third: log to audit
```
