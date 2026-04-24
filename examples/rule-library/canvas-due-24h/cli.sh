#!/bin/bash
# Trigger: Canvas assignment due in 24 hours event
# Action: Notify and add task to planner if absent
# Bash OK as copy-paste documentation of CLI invocation
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
