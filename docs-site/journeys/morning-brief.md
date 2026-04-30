---
title: "User Journey: Morning Brief Ritual"
description: Daily morning routine with sleep stats, connector data, and AI-suggested focus blocks.
verification:
  manifest: "/recordings/morning-brief/manifest.json"
  status: "planned"
  last_verified: null
---

# User Journey: Morning Brief Ritual

## Persona

**Names**: Alice (Student), Bob (Developer)
**Context**: Daily morning users with connected apps
**Goal**: Start the day with clear priorities and focus opportunities
**Pain point**: Morning confusion about what to tackle first

## What You'll See

Recording of the morning brief ritual: sleep stats → calendar → focus opportunities → start first block.

<JourneyViewer manifest="/recordings/morning-brief/manifest.json" />

![Morning Brief Overview](/images/journeys/rituals/morning-overview.svg)

## Phase 1: Launch & Sleep Stats

### Open FocalPoint Morning

Bob opens FocalPoint at 7:30 AM:

```
📊 MORNING BRIEF
⏰ 7:30 AM, Tuesday

Good morning, Bob! Let's make today count.
```

Sleep data from Apple Health appears:

```
😴 Sleep: 7h 15m ✓ (goal: 7h)
  ████████████░░░░░░ 85% of goal
  Trend: ↑ +15 min vs. last week

💡 Tip: Great sleep! Your focus should be sharp today.
```

![Sleep Stats Detail](/images/journeys/rituals/sleep-stats.svg)

## Phase 2: Today's Schedule

### Calendar Integration

Apple Calendar shows:

```
📅 TODAY
  • 9:00 AM — Team standup (1h)
  • 12:00 PM — Lunch
  • 3:00 PM — 1:1 with manager

⚠️ 2 items need your attention:
  • CS 101: Midterm project due in 3 days
  • Review @alice's PR (waiting 2 days)
```

## Phase 3: Focus Opportunities

### AI-Suggested Blocks

FocalPoint analyzes your schedule and suggests:

```
🎯 FOCUS OPPORTUNITIES

1. 8:00–9:00 AM (60 min) — Review @alice's PR
   Impact: Unblock teammate; keep PR streak alive
   👉 [Start Focus]

2. 10:00 AM–12:00 PM (2h) — Midterm project work
   Impact: 3 hours until deadline; start early beats cramming
   👉 [Schedule Block]

3. 1:00–2:00 PM (1h) — Prep for 1:1
   Impact: Clear talking points; make the most of 1:1
   👉 [Schedule Block]
```

![Focus Opportunities](/images/journeys/rituals/focus-opportunities.svg)

### Start First Focus

Bob taps "Start Focus" on the PR review opportunity:

```
🎯 FOCUS MODE ACTIVE
Duration: 60 min (timer started)

Blocked apps: Slack, Discord, Twitter
Whitelisted: GitHub, VS Code

💭 Coachy: "Review @alice's PR now.
         You have 9 min before standup."
```

## Verification

::: info Planned
This journey is planned for recording. The JourneyViewer above shows the expected manifest structure.

Track progress: [GitHub Issue #journey-morning-brief](https://github.com/KooshaPari/FocalPoint/issues?q=label:journey-morning-brief)
:::

## Related Guides

- [Morning Brief Ritual](../rituals/morning-brief.md)
- [Core Loop](../guides/core-loop.md)
- [Focus Mode](../guides/focus-mode.md)
