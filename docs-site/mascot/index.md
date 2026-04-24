---
title: Coachy
description: Meet FocalPoint's AI-powered coaching companion.
---

# Coachy: Your Digital Coach

**Coachy** is FocalPoint's personable, supportive coaching interface. Think of Coachy as your digital accountability buddy—not a nag, but someone who gets your goals and helps you stay on track.

## Coachy's Personality

Coachy is:

- **Warm**: Uses your name, remembers your goals, celebrates wins
- **Non-judgmental**: Never shames you for breaks or missed targets
- **Adaptive**: Learns your patterns and adjusts coaching to your style
- **Encouraging**: Offers specific, actionable suggestions based on your habits

Coachy is **not**:

- Preachy or guilt-tripping
- Patronizing or overly cute
- Intrusive (respects do-not-disturb times)
- Suspicious of your choices

## Coaching Flows

| Trigger | Coachy Response | Customizable? |
|---------|-----------------|---------------|
| Blocked app attempt | Offers a 5-min focus timer or redirection | Yes |
| Focus goal met | Celebrates milestone (streak, time total) | Yes |
| Sleep debt detected | Morning nudge with sleep quality data | Yes |
| Assignment deadline approaching | Calendar reminder + study plan template | Yes |
| Procrastination detected | Gentle nudge + Pomodoro suggestion | Yes |

## Customizing Coachy

Users can configure:

- **Communication frequency**: Rare, balanced, supportive (default), assertive
- **Tone**: Professional, casual, playful
- **Availability**: Do-not-disturb times (e.g., after 10 PM)
- **Celebration style**: Emoji, achievements, streaks, points

See **[Character Sheet](./character-sheet)** and **[Personality Guide](./personality)** for full customization details.

## Coachy's Technical Role

Under the hood, Coachy is:

- **Rules-driven**: Coaching messages are emitted by the rule engine (not ML)
- **Local**: Coachy logic runs on-device; no cloud coaching model needed
- **Auditable**: Every coaching message is logged in the audit chain

This ensures coaching is transparent and respects your privacy.
