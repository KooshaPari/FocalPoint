---
title: Google Calendar Connector (Aspirational)
description: Sync meetings, focus blocks, and availability from Google Calendar.
---

# Google Calendar Connector

*This connector is aspirational. Status: Planned for Q2 2026.*

The Google Calendar connector will sync your meeting schedule, focus time blocks, and availability status into FocalPoint rules.

## Planned Features

- **Meeting sync**: Detect meetings and trigger pre-meeting focus blocks
- **Focus blocks**: Honor "Focus Time" calendar entries (skip rule enforcement)
- **Availability**: Don't send coaching when you're busy or in meetings
- **Timezone handling**: Correctly interpret all-day events across timezones

## Events (Planned)

```json
{
  "event_type": "gcal.meeting.starting",
  "meeting_id": "xyz",
  "title": "Team Standup",
  "duration_minutes": 30,
  "start_at": "2026-04-23T10:00:00Z"
}
```

```json
{
  "event_type": "gcal.focus_time.active",
  "end_at": "2026-04-23T12:00:00Z"
}
```

## Proposed Rule

```yaml
name: "No coaching during meetings"
trigger:
  event_type: "gcal.meeting.starting"
action:
  - disable_coaching_for: "30 minutes"
```

## Implementation Status

- [ ] OAuth 2.0 integration
- [ ] Event subscription (via Google Pub/Sub)
- [ ] Timezone handling
- [ ] Conflict detection (overlapping events)
- [ ] Security audit
- [ ] Integration testing

See [Open Questions](/research/open-questions) for design discussions.
