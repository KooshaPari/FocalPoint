---
title: Morning Brief Ritual
description: Start your day with intention—see your schedule, sleep quality, and focus goals.
---

# Morning Brief Ritual

The **Morning Brief** is a 2–3 minute daily ceremony that starts your day with intention.

## What You See

### 1. Sleep Quality (from Apple Health)

```
😴 Sleep: 7h 15m (Good)
   Debt: -30 min (recovering!)
```

Shows sleep duration and how you're tracking against your sleep goal.

### 2. Today's Calendar & Deadlines

```
📅 Schedule:
   • 9:00 AM – CS 101 Lecture
   • 2:00 PM – Project deadline (Canvas)
   • 4:30 PM – Team standup
```

Automatically pulls from Google Calendar and Canvas.

### 3. Suggested Focus Sessions

```
🎯 Focus opportunities:
   1. Before 9 AM – Review lecture slides (30 min)
   2. 10 AM – Start project work (90 min)
   3. Lunch break – Recharge (45 min)
```

Based on your calendar + assignments + past focus patterns.

### 4. Streak Status

```
🔥 Current streaks:
   • 7-day focus streak (great!)
   • 3-day early submission streak
```

Celebrate momentum and inspire consistency.

## Customization

### Timing

- **Default**: 6:30 AM (configurable)
- **Weekday-only**: Skip weekends
- **Adaptive**: Move based on first calendar event

### Content

Disable/enable specific sections:

- Sleep quality (requires Apple Health)
- Calendar sync
- Focus suggestions
- Streak display

### Presentation

- **Text-only**: Plain summary
- **Cards**: Visual layout (default)
- **Voice**: Audio brief (via Siri, if enabled)

## Ritual Flow

1. **Wakeup**: Brief appears in notification or app
2. **Review**: Read key info (30 seconds)
3. **Select**: Pick a focus goal or task (1 minute)
4. **Commit**: Start a focus session or review your calendar (1 minute)
5. **Dismiss**: Brief exits; you start your day

## Example: Student Morning

```
6:30 AM: Morning Brief appears

Sleep: 6h 45m (slight debt)
Today:
  • 9 AM – CS 101
  • 2 PM – Midterm project due ⚠️
  • 4 PM – Office hours

Focus suggestions:
  1. 7–8 AM: Review lecture notes (30 min)
  2. 9 AM–12 PM: Midterm work sprint (180 min)
  3. 1 PM: Lunch break (45 min)

Streaks: 5 days (keep going!)

👉 "Start focus session?"
   [Yes] [Maybe later] [Dismiss]
```

## Integration with Rules

The Morning Brief can trigger rules:

```yaml
trigger:
  - schedule: "every morning at 06:30"

action:
  - show_morning_brief
  - coach_message: "Let's make today great!"
```

## Privacy

- All data is local (Apple Health, Calendar stored on device)
- No cloud sync by default
- Users can disable specific sources
- Audit log tracks when brief was viewed

## Ritual Archetype: Productivity Kickoff

For general productivity:

```
Schedule + Focus opportunities + Streak status
(No sleep data unless you have Apple Health enabled)
```

## Ritual Archetype: Sleep Wellness

For sleep-focused users:

```
Sleep debt + Suggested recovery time + Energy levels
(No calendar unless busy day detected)
```

## Schedule Configuration

Ritual runs at configured time via system scheduler. No cloud dependency.

See **[Rituals Overview](./index)** for how to create custom rituals.
