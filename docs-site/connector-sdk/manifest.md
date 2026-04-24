---
title: Connector Manifest Format
description: Define your connector's metadata, capabilities, and OAuth scopes.
---

# Connector Manifest

Every connector must include a **manifest file** that describes its metadata, event types, and required permissions.

## Example Manifest (Canvas LMS)

```toml
[connector]
id = "canvas-lms"
name = "Canvas LMS"
version = "1.0.0"
description = "Sync assignment deadlines and grades from Canvas"
author = "FocalPoint contributors"
license = "MIT"

[auth]
type = "oauth2"
provider = "canvas"
scopes = [
  "url:GET|/api/v1/users/self/assignments",
  "url:GET|/api/v1/courses",
]

[events]
"canvas.assignment.created" = "Assignment created in Canvas"
"canvas.assignment.due_soon" = "Assignment due approaching"
"canvas.grade.posted" = "Grade received for assignment"

[capabilities]
permissions = [
  "read:assignments",
  "read:courses",
  "read:grades",
]
```

## Fields

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `connector.id` | ✓ | string | Unique identifier (slug format) |
| `connector.name` | ✓ | string | User-friendly name |
| `connector.version` | ✓ | semver | Connector version |
| `connector.description` | ✓ | string | 1-2 sentence description |
| `connector.author` | ✓ | string | Author name or org |
| `auth.type` | ✓ | enum | `oauth2`, `api_key`, or `none` |
| `auth.provider` | ✓ | string | OAuth provider slug (e.g., `canvas`, `github`) |
| `auth.scopes` | ✓ | array | Required OAuth scopes |
| `events` | ✓ | map | Event type → description |
| `capabilities.permissions` | ✓ | array | Required permissions from rule engine |

## Event Type Convention

Event types follow the pattern: `{service}.{entity}.{action}`

Examples:
- `canvas.assignment.due_soon`
- `github.pr.ready_for_review`
- `health.sleep_debt.high`

## OAuth Providers (Supported)

- `canvas` — Canvas LMS OAuth 2.0
- `github` — GitHub OAuth 2.0
- `google` — Google OAuth 2.0
- `todoist` — Todoist OAuth 2.0
- `ynab` — YNAB OAuth 2.0
- `custom` — Your own OAuth endpoint

## Capabilities & Permissions

Connectors declare what they need access to:

```toml
[capabilities]
permissions = [
  "read:assignments",
  "write:audit_log",
  "access:device_clock",
]
```

Common permissions:

- `read:{entity}` — Query stored data
- `write:{entity}` — Mutate stored data
- `access:device_clock` — Read device time
- `access:device_location` — Read location (if applicable)
- `emit:notifications` — Send user notifications

## Validation

Validate your manifest before publishing:

```bash
focalpoint connector validate-manifest connector.toml
```
