---
title: Rules
description: Create conditions and actions to automate FocalPoint's screen-time management.
---

# Rules

A **rule** is a condition-action pair that drives FocalPoint's automation. When a condition is true, the rule executes actions like blocking apps, showing focus views, or triggering coaching.

## Rule Anatomy

```yaml
name: "Study Mode: Block TikTok during class"
enabled: true

condition:
  - event_type: "canvas.assignment.due_soon"
    hours_until: 24
  - calendar_event_type: "class"
  time_window:
    start: "08:00"
    end: "17:00"

action:
  - block_app: "com.tiktok.main"
  - show_focus_view: "study"
  - send_notification: "Focus: Canvas deadline in 2 hours"
  - log_audit: "Rule triggered by assignment deadline"
```

## Creating Rules

### In-App Wizard

1. Open FocalPoint Settings → Rules → New Rule
2. Give your rule a name and choose a trigger (calendar event, assignment, time of day, etc.)
3. Select conditions and actions from the menu
4. Preview the rule and save

### YAML Format (Advanced)

Rules can also be written as YAML files and imported via CLI:

```bash
focalpoint rule import my-rules.yaml
```

## Core References

- **[DSL Reference](./dsl)** — Complete rule syntax
- **[Condition Built-ins](./conditions)** — All available condition types
- **[Action Catalogue](./actions)** — All available action types
- **[Sample Rule Packs](./samples)** — Pre-built rule templates

## Rule Packs

A **rule pack** is a curated collection of rules for a specific use case (e.g., "Student Study Focus", "Developer Deep Work", "Sleep Wellness").

Rule packs can be:
- **Shipped**: Bundled with FocalPoint
- **Community**: Shared in the marketplace
- **Personal**: Created by users for their own workflows

See [Rule Template Format](../ecosystem/rule-template-format) for pack specs.
