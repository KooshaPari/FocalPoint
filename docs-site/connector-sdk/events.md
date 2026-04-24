---
title: Event Schema
description: Define the event types and payload structures your connector emits.
---

# Event Schema

Connectors emit **structured events** that feed into FocalPoint's rule engine. This document describes the schema and validation rules.

## Event Structure

All events follow this structure:

```json
{
  "event_type": "canvas.assignment.due_soon",
  "timestamp": "2026-04-23T10:30:00Z",
  "connector_id": "canvas-lms",
  "payload": {
    "assignment_id": "123",
    "course_id": "456",
    "hours_until_due": 24
  }
}
```

## Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `event_type` | string | Fully qualified type from manifest |
| `timestamp` | ISO 8601 | UTC event time |
| `connector_id` | string | Connector ID from manifest |
| `payload` | object | Event-specific data |

## Payload Validation

Each event type in your manifest must have a corresponding **schema definition**. Use JSON Schema:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "assignment_id": { "type": "string" },
    "course_id": { "type": "string" },
    "hours_until_due": { "type": "integer", "minimum": 0 }
  },
  "required": ["assignment_id", "course_id"]
}
```

## Emitting Events

From your connector WASM code:

```rust
use focalpoint_sdk::event::{Event, emit_event};

emit_event(Event {
    event_type: "canvas.assignment.due_soon".to_string(),
    timestamp: chrono::Utc::now(),
    connector_id: "canvas-lms".to_string(),
    payload: serde_json::json!({
        "assignment_id": assignment.id,
        "course_id": course.id,
        "hours_until_due": (due_at - now()).num_hours(),
    }),
})?;
```

## Event Ordering

Events are appended to the event store in order. The rule engine respects:

1. **Temporal ordering**: Events processed in timestamp order
2. **Idempotency**: Duplicate event_type + payload_hash are de-duplicated
3. **Causality**: Rules can reference event history

## Best Practices

1. **Use consistent payloads**: Same event type → same payload schema
2. **Include IDs**: Always include external service IDs (e.g., `assignment_id`)
3. **Timestamp accurately**: Use service time when possible; fall back to `now()`
4. **Test your schema**: Validate payloads before emitting
5. **Document rare fields**: Add clarifying comments for optional or rare fields

## Examples

### Canvas: Assignment Due Soon

```json
{
  "event_type": "canvas.assignment.due_soon",
  "payload": {
    "assignment_id": "789",
    "course_id": "456",
    "course_name": "CS 101",
    "assignment_title": "Midterm Project",
    "hours_until_due": 24,
    "points_possible": 100
  }
}
```

### Health: Sleep Debt

```json
{
  "event_type": "health.sleep_debt.high",
  "payload": {
    "total_sleep_hours": 4.5,
    "target_sleep_hours": 8,
    "sleep_debt_hours": 3.5,
    "nights_tracked": 5
  }
}
```
