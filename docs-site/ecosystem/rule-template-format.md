---
title: Rule Template Format
description: Specification for rule templates in the FocalPoint ecosystem.
---

# Rule Template Format

Rule templates are pre-built rule configurations that users can install and customize for their workflow.

## Template Structure

```yaml
name: "Focus on Canvas Assignments"
description: "Block distracting apps when Canvas assignments are due"
version: "1.0.0"
author: "FocalPoint team"

trigger:
  - event_type: "canvas.assignment.due_soon"
    hours_until: 24

condition:
  - time_window:
      start: "08:00"
      end: "17:00"
  - weekday: [Monday, Tuesday, Wednesday, Thursday, Friday]

action:
  - block_app: ["com.tiktok.main", "com.instagram.android"]
  - show_focus_view: "study"
```

## Customization Points

Users can override:
- Trigger thresholds (hours_until)
- App lists
- Focus view type
- Time windows

See the [DSL Reference](/rules/dsl) for complete syntax.
