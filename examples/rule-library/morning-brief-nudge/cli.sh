#!/bin/bash
# Trigger: Cron schedule daily at noon (0 12 * * *)
# Action: Soft nudge intervention for incomplete morning brief
# Bash OK as copy-paste documentation of CLI invocation
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
