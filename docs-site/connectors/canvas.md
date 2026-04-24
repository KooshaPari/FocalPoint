---
title: Canvas LMS Connector
description: Sync assignment deadlines, course schedules, and grades from your Canvas instance.
---

# Canvas LMS Connector

The Canvas connector brings your course deadlines, assignment schedules, and grade notifications directly into FocalPoint. When an assignment is due soon, Canvas can trigger focus rules automatically.

## Quick Start

1. **Install & enable**: FocalPoint Settings → Connectors → Canvas LMS → Enable
2. **Authorize**: Sign in with your Canvas account (institution SSO supported)
3. **Select courses**: Choose which Canvas courses to sync
4. **Done**: Deadlines now feed your rules

Setup time: **~5 minutes**

## Features

| Feature | Status | Description |
|---------|--------|-------------|
| Assignment deadlines | ✓ Shipping | Triggered when due in 24h, 1h, overdue |
| Course schedule | ✓ Shipping | Class meeting times sync to calendar |
| Grade notifications | ✓ Shipping | New grades trigger optional alert rules |
| Submission tracking | ✓ Shipping | Submitted status propagated to audit log |
| Attendance | ⏳ Planned | Absence tracking for make-up study rules |
| Syllabus scraping | ⏳ Planned | Extract learning objectives for coaching |

## Events Emitted

```json
{
  "event_type": "canvas.assignment.created",
  "assignment_id": "123",
  "course_id": "456",
  "title": "Midterm Project",
  "due_at": "2026-05-15T23:59:59Z",
  "points_possible": 100
}
```

```json
{
  "event_type": "canvas.assignment.due_soon",
  "hours_until_due": 24,
  "assignment_id": "123"
}
```

## Example Rule

```yaml
name: "Study Mode: Canvas assignment due in 24h"
enabled: true

trigger:
  event_type: "canvas.assignment.due_soon"
  hours_until_due: 24

condition:
  - not_blocked_by: "social_media"

action:
  - block_app:
      - "com.tiktok.main"
      - "com.instagram.android"
  - show_focus_view: "study"
  - set_deadline_reminder:
      hours: 1
      message: "Assignment due {{assignment.title}} - {{hours}} hours left"
```

## Troubleshooting

**Q: My courses aren't syncing**  
A: Check that your Canvas instance allows third-party integrations. Some institutions restrict API access. Contact your Canvas admin.

**Q: I'm seeing old assignments**  
A: Sync lookback is 90 days by default. You can adjust in Settings → Connectors → Canvas LMS → History.

**Q: Can I sync multiple Canvas instances?**  
A: Not yet—you can only link one Canvas organization per FocalPoint account. This is planned for v1.1.

## Architecture

The Canvas connector:

1. Uses Canvas OAuth 2.0 to authenticate with your institution
2. Polls the Canvas REST API every 30 minutes for new assignments and grade updates
3. Emits structured events (`canvas.assignment.*`) to the event store
4. Rules subscribe to those events and react accordingly

All credentials are stored in your device's secure enclave (Keychain on iOS, Keystore on Android).
