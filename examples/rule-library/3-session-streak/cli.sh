#!/bin/bash
# Trigger: Condition when session streak count >= 3
# Action: Gentle intervention celebration + 10 bonus credits
# Bash OK as copy-paste documentation of CLI invocation
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "3-session-streak",
  "name": "3-Session Streak Celebration",
  "body": {
    "kind": "Rule",
    "id": "3-session-streak",
    "name": "3-Session Streak Celebration",
    "trigger": {"type": "ConditionMet", "value": {"condition": {"op": "user_attribute", "key": "session_streak", "value": "3"}}},
    "conditions": [],
    "actions": [
      {"type": "show_notification", "notification_id": "streak-celebration", "text": "Congratulations on 3 consecutive sessions!", "duration_ms": 5000},
      {"type": "apply_mutation", "mutation_id": "grant_credits", "params": {"amount": 10}}
    ],
    "priority": 70,
    "cooldown_seconds": null,
    "duration_seconds": null,
    "explanation_template": "3-session streak: +10 bonus credits",
    "enabled": true
  }
}'
