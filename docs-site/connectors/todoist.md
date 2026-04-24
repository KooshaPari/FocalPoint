---
title: Todoist Connector (Aspirational)
description: Sync task deadlines and project milestones from Todoist.
---

# Todoist Connector

*This connector is aspirational. Status: Planned for Q3 2026.*

The Todoist connector brings your task deadlines and project milestones into FocalPoint, triggering focus rules as due dates approach.

## Planned Features

- **Task due dates**: Emit events when tasks are due
- **Project deadlines**: Sync project completion dates
- **Priority labels**: React to high-priority task markers
- **Recurring tasks**: Recognize daily/weekly patterns

## Example Rule

```yaml
name: "High-priority task focus"
trigger:
  event_type: "todoist.task.due_soon"
  priority: "high"
  hours_until_due: 4
action:
  - show_focus_view: "productivity"
  - coach_message: "High-priority task '{{task.title}}' due in {{hours}} hours"
```

## Implementation Status

- [ ] Todoist API authentication
- [ ] Webhook setup
- [ ] Priority level parsing
- [ ] Recurring task handling

## Roadmap

Part of the broader **"Multi-Workspace Productivity Integration"** initiative. See GitHub Project Board.
