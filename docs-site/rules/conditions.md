---
title: Condition Built-ins
description: All available condition types for rules.
---

# Condition Built-ins

This document lists every built-in condition type available in FocalPoint rules.

## Time Conditions

### time_window
Evaluates to true if current time falls in the window.

```yaml
condition:
  - time_window:
      start: "08:00"
      end: "17:00"
      timezone: "America/New_York"  # Optional; default: device timezone
```

### weekday
Evaluates to true if current day matches any in the list.

```yaml
condition:
  - weekday: [Monday, Tuesday, Wednesday, Thursday, Friday]
```

### time_until_event
Evaluates to true if an event is approaching.

```yaml
condition:
  - time_until_event:
      event_type: "canvas.assignment.due_soon"
      within_hours: 24
```

## Device Conditions

### battery_above
True if battery level is above threshold.

```yaml
condition:
  - battery_above: 20  # Percentage
```

### screen_on
True if device screen is active.

```yaml
condition:
  - screen_on: true
```

### on_charger
True if device is plugged in.

```yaml
condition:
  - on_charger: false  # True if NOT on charger
```

### location
True if user is in a region.

```yaml
condition:
  - location:
      region: "home"  # Geofence name from settings
      within_meters: 100
```

## App & Focus Conditions

### app_in_foreground
True if specific app is active.

```yaml
condition:
  - app_in_foreground: "com.tiktok.main"
```

### in_focus_mode
True if user is in any focus mode.

```yaml
condition:
  - in_focus_mode: true
```

### focus_mode_type
True if user is in a specific focus mode.

```yaml
condition:
  - focus_mode_type: "study"
```

## Calendar & Scheduling

### not_in_event
True if no calendar event is active.

```yaml
condition:
  - not_in_event: true
```

### calendar_event_type
True if a specific event type is active.

```yaml
condition:
  - calendar_event_type: "meeting"
```

## Audit & Wallet

### streak_active
True if user has an active streak.

```yaml
condition:
  - streak_active: true
  - streak_days: 7  # Minimum streak length
```

### wallet_balance_above
True if wallet balance exceeds threshold.

```yaml
condition:
  - wallet_balance_above: 100
```

### audit_check
True if audit chain passes verification.

```yaml
condition:
  - audit_check:
      verify_signature: true
      max_age_hours: 24
```

## Logical Operators

### all_of
All sub-conditions must be true.

```yaml
condition:
  - all_of:
      - time_window: { start: "09:00", end: "17:00" }
      - weekday: [Monday, Tuesday, Wednesday]
```

### any_of
At least one sub-condition must be true.

```yaml
condition:
  - any_of:
      - app_in_foreground: "com.canvas.canvas"
      - event_type: "canvas.assignment.due_soon"
```

### not
Negates a condition.

```yaml
condition:
  - not:
      in_focus_mode: true
```

## Context Variables

Access event data:

```yaml
condition:
  - custom_check:
      expression: "event.hours_until_due < 12"
```

This is a Turing-complete expression language (limited to prevent abuse).
