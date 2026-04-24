---
title: Rituals
description: Design daily and weekly ceremonies to anchor your screen-time habits.
---

# Rituals

**Rituals** are scheduled check-ins and ceremonies that help you reflect on your screen-time patterns and set intentions for the day ahead.

## Core Rituals

### Morning Brief

Every morning, FocalPoint presents a brief dashboard showing:

- **Sleep quality** (from Apple Health integration)
- **Today's calendar & deadlines** (Canvas, Google Calendar)
- **Suggested focus sessions** (based on schedule and past patterns)
- **Streak status** (days without distraction blocks)

Estimated read time: **2–3 minutes**

See **[Morning Brief Guide](./morning-brief)** for customization options.

### Evening Shutdown

Before bed, FocalPoint prompts a 5-minute reflection:

- **Time audit**: How much time did you spend on focused work vs. distraction?
- **Goals reflection**: Did you hit your study blocks or wellness targets?
- **Tomorrow's intention**: What's one priority for tomorrow?

The shutdown is logged to your audit chain. Over time, patterns emerge.

See **[Evening Shutdown Guide](./evening-shutdown)** for ritual design.

## Custom Rituals

Advanced users can design their own ceremonies:

```yaml
ritual:
  name: "Weekly Retrospective"
  schedule: "every Sunday at 19:00"
  prompts:
    - "What went well this week?"
    - "What blocked my focus most?"
    - "What's one habit to improve?"
  actions:
    - export_audit_chain: "weekly-report.json"
    - send_email: "coaching@focalpoint.local"
```

## Ritual Architecture

Rituals are:

- **Time-triggered** via the system scheduler (cron-like rules)
- **Logged** to the audit chain for transparency
- **Customizable** by users (frequency, prompts, actions)
- **Privacy-respecting** (exported data stays local unless user opts to share)

Rituals empower reflection without being invasive.
