---
title: Evening Shutdown Ritual
description: Reflect on your day, plan tomorrow, and wind down intentionally.
---

# Evening Shutdown Ritual

The **Evening Shutdown** is a 5–10 minute reflection ceremony before bed.

## What You Answer

### 1. Time Audit

```
📊 How you spent your time today:
   
   Focus time: 3h 45m (goal: 4h) ✓
   Distraction time: 1h 30m (goal: <2h) ✓
   Deep work: 2h 15m (goal: 2h) ✓
   Breaks: 45m (goal: 45m) ✓
```

Shows your actual screen-time breakdown vs. your goals.

### 2. Assignment Progress

```
📚 Canvas assignments:
   ✓ Submitted: 3
   ⏳ In progress: 2
   ⚠️ Overdue: 0
```

If Canvas is connected, shows submission status.

### 3. Reflection Questions

```
Q1: "Did you hit your focus goals today?"
    [Yes] [Mostly] [No] [N/A]

Q2: "What blocked your focus most?"
    - Social media
    - Procrastination
    - Other tasks
    - Distractions
    (Multiple select)

Q3: "What's your priority for tomorrow?"
    [Open text field, 30 chars]

Q4: "How's your energy level?"
    [Very low] [Low] [Medium] [High] [Very high]
```

### 4. Coaching Reflection

Based on answers, Coachy offers:

```
You're averaging 3h 45m focus per day (great!).
Procrastination blocked you 3 times this week.
Consider:
  • Smaller task chunks
  • Pomodoro timer
  • Pre-work ritual (e.g., coffee + music)

Tomorrow's focus goal: {{user_input}}
```

## Ritual Flow

1. **Prompt**: Evening Shutdown notification (default 10 PM)
2. **Audit**: Review your time breakdown (1 min)
3. **Reflect**: Answer 3–4 questions (3–4 min)
4. **Plan**: Set tomorrow's intention (1 min)
5. **Lockdown**: Optional: Enable evening focus mode (blocks social media, dimms screen)
6. **Log**: All responses saved to audit chain

## Customization

### Timing

- **Default**: 10 PM (configurable)
- **Flexible**: Appear in notification; user taps when ready
- **Strict**: Mandatory before logging out (can override)

### Questions

Choose which questions to include:

```
[ ] Time audit
[✓] Canvas progress
[✓] Focus reflection
[✓] Tomorrow's priority
[✓] Energy level
[ ] Sleep forecast
```

### Actions After Shutdown

Optionally trigger:

- Evening lockdown (block social apps)
- Do-not-disturb until morning
- Scheduled morning brief
- Export audit chain

## Example: Developer Evening

```
10 PM: Evening Shutdown notification appears

📊 TIME AUDIT
Deep work: 4h 30m (goal: 4h) ✓ +30m bonus
Meetings: 1h (goal: 1.5h) ✓
Email/admin: 45m (goal: <1h) ✓

Q: "Hit your focus goals?"
A: [Mostly]

Q: "What blocked focus?"
A: [Slack notifications, context switching]

Q: "Tomorrow's priority?"
A: "Deploy backend feature"

Q: "Energy level?"
A: [Medium]

💭 Coaching:
"Strong day! You avoided context switches pretty well 
in the afternoon. Watch Slack notifications tomorrow—
consider 'Focus hours' from 10–12 AM."

[Export audit] [Start evening lockdown] [Done]
```

## Integration with Rules

Evening shutdown can trigger rules:

```yaml
trigger:
  - schedule: "every day at 22:00"

action:
  - show_evening_shutdown
  - log_audit: "Evening reflection completed"
  - optional:
      - enable_evening_lockdown: true
      - mute_notifications: true
```

## Data Privacy

- Responses stay local (no cloud upload unless user exports)
- Responses feed audit chain (tamper-evident)
- Users can delete old shutdowns from Settings → Privacy
- Export data in JSON format for personal analysis

## Ritual Evolution

Over time, Evening Shutdown learns:

- Your typical time spent on focus vs. distraction
- Your most common blockers
- Your energy patterns (when you tend to have low energy)
- Your priority themes (academic, work, wellness, etc.)

Coachy uses this to personalize suggestions.

See **[Rituals Overview](./index)** for custom ritual design.
