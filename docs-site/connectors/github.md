---
title: GitHub Connector (Aspirational)
description: Trigger focus sessions based on PR activity, issue assignments, and commit deadlines.
---

# GitHub Connector

*This connector is aspirational. Status: Planned for Q3 2026.*

The GitHub connector brings your pull request reviews, issue assignments, and project milestones into FocalPoint, enabling developer-focused focus rules.

## Planned Features

- **PR notifications**: Trigger focus blocks for code review sessions
- **Issue assignments**: Alert when assigned a new issue
- **Milestone deadlines**: Block distractions near release dates
- **Commit streaks**: Gamify daily commits with FocalPoint streaks

## Example Rules

```yaml
name: "Code review focus"
trigger:
  event_type: "github.pr.ready_for_review"
action:
  - show_focus_view: "code_review"
  - block_app:
      - "com.slack"
      - "com.discord"
  - schedule_break: "25 minutes"
---
name: "Milestone approaching"
trigger:
  event_type: "github.milestone.due_soon"
  days_until_due: 3
action:
  - show_daily_reminder: "{{milestone.title}} due in {{days}} days"
```

## Events (Planned)

```json
{
  "event_type": "github.pr.ready_for_review",
  "pr_number": 1234,
  "author": "alice",
  "title": "Add async support",
  "repository": "myorg/myrepo"
}
```

## Implementation Status

- [ ] OAuth app registration
- [ ] GitHub webhook subscription
- [ ] Rate limiting strategy
- [ ] Webhook secret validation
- [ ] Testing harness

See [Connector SDK](../connector-sdk/) for integration details.
