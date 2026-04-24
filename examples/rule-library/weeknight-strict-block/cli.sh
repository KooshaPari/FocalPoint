#!/bin/bash
# Trigger: Cron schedule weekdays at 10pm (0 22 * * 1-5)
# Action: Enforce strict block from 10pm to 7am on weekdays
# Gap: Standard cron syntax supported; Rule Builder graph JSON lacks native weekday picker widget
# Bash OK as copy-paste documentation of CLI invocation
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
