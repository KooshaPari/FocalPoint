#!/bin/bash
# Trigger: focus:session_missed event fired
# Action: Concerned intervention (requires host-side state tracking for 2x consecutive)
# Gap: Detecting 2 consecutive misses requires external state coordination
# Bash OK as copy-paste documentation of CLI invocation
focus rules upsert --json '{
  "version": 1,
  "kind": "Rule",
  "id": "missed-focus-2x",
  "name": "Missed Scheduled Focus 2 Days in a Row",
  "body": {
    "kind": "Rule",
    "id": "missed-focus-2x",
    "name": "Missed Scheduled Focus 2 Days in a Row",
    "trigger": {"type": "EventFired", "value": {"event_name": "focus:session_missed"}},
    "conditions": [{"op": "user_attribute", "key": "consecutive_missed_sessions", "value": "2"}],
    "actions": [{"type": "show_notification", "notification_id": "concerned-intervention", "text": "You have missed focus sessions for 2 consecutive days. Let's talk about barriers.", "duration_ms": 8000}],
    "priority": 80,
    "cooldown_seconds": 86400,
    "duration_seconds": null,
    "explanation_template": "Concerned intervention: 2 consecutive missed sessions",
    "enabled": true
  }
}'
