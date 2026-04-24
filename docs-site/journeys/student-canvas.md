---
title: "User Journey: Student on Canvas"
description: How students use FocalPoint to manage Canvas assignments and study workflows.
---

# User Journey: Student on Canvas

## Persona

**Name**: Alice  
**Context**: CS major, 4 courses, Canvas + Apple Calendar  
**Goal**: Never miss a deadline; maintain consistent study habits  
**Pain point**: Procrastination on assignments; inconsistent focus time  

## Week 1: Onboarding

### Day 1: Installation & Setup (10 min)

Alice downloads FocalPoint on her iPhone 14. She:

1. Grants FamilyControls entitlement (system prompt)
2. Skips optional connectors (just local rules for now)
3. Creates her first rule in-app:
   - **Trigger**: "Daily at 8 AM"
   - **Action**: "Show morning brief"
4. Sees the morning brief: empty (no Canvas yet)

### Day 2–3: Canvas Connection (5 min)

Alice links her Canvas account:

1. Settings → Connectors → Canvas LMS → Enable
2. Signs in with her university SSO
3. Selects all 4 courses
4. Grants Canvas API permissions

FocalPoint now syncs her assignments every 30 minutes.

Morning brief now shows:

```
📚 Canvas assignments (next 2 weeks):
  • CS 101: Homework 3 (due Wed 11:59 PM)
  • CS 101: Midterm project (due in 10 days)
  • MATH 201: Problem set 5 (due Fri)
```

### Day 4–5: Rule Creation (15 min)

Alice creates her first context-specific rule:

```yaml
name: "Canvas assignment due in 24h - Study Focus"
trigger:
  - event_type: "canvas.assignment.due_soon"
    hours_until: 24
condition:
  - time_window: { start: "08:00", end: "23:59" }
  - not_in_focus: false
action:
  - show_focus_view: "study"
  - block_app: ["com.tiktok.main", "com.instagram.android"]
  - coach_message: "{{event.title}} due in {{event.hours_until_due}} hours! Let's focus."
```

She tests it by manually triggering the rule. TikTok gets blocked. ✓

## Week 2–3: Habit Formation

### Morning Ritual (2 min daily)

Alice opens FocalPoint each morning at 7:30 AM:

```
📊 Sleep: 7h 15m ✓
📅 Today: CS 101 lecture at 9 AM, MATH problem set due 11:59 PM
🎯 Focus opportunities:
   1. 8–9 AM: Review CS notes (60 min)
   2. 10 AM–12 PM: MATH problem set work (120 min)
🔥 Streak: 5 days
```

She taps "Start focus session" and TikTok/Instagram are blocked until 2 PM.

### During the Day

Alice is in Slack. A notification appears:

```
🎯 Canvas Assignment Alert
MATH 201 Problem set due in 4 hours!
👉 [Start focus] [Later] [Dismiss]
```

She taps "Start focus" → 90-min focus timer starts → social apps blocked.

At 1 PM, she finishes the problem set and submits it in Canvas. Coachy reacts:

```
💪 Submitted! You're ahead of schedule on MATH.
Keep the momentum going!
```

### Evening Reflection (5 min daily)

At 10 PM, Evening Shutdown ritual appears:

```
📊 TIME AUDIT
Focus time: 4h 30m (goal: 4h) ✓ +30m bonus!
Distraction time: 1h 15m (goal: <2h) ✓

📚 Canvas Progress
Submitted: 2/8 assignments this week
Due soon: 2 (both >=2 days away)

Q: "What blocked your focus?"
A: [Procrastination on problem set]

Q: "Tomorrow's priority?"
A: "Start CS Midterm project"

💭 Coachy: "4.5h focus is great! 
Procrastination hit you once—
consider tackling hard tasks first thing in the morning."

[Export audit] [Done]
```

## Week 4: Streak Achievement

After 14 consecutive days of focus (matching her goal), Coachy celebrates:

```
🔥 14-day streak!
You've built a real habit. Keep it up! 🎉
```

Alice shares her achievement on Instagram (voluntarily).

## Month 2: Advanced Usage

### Rule Packs

Alice imports the **"Student Focus Pack"** from the marketplace:

```
Includes 8 rules:
  • Canvas assignment deadline blocks
  • Morning study prep ritual
  • Exam week intense focus mode
  • Sleep wellness checks
  • Procrastination detection
```

She customizes the "Exam week intense focus mode" rule:

```yaml
# During exam week, block ALL social apps + calls
trigger:
  - schedule: "Mon–Fri, Apr 15–19, 8 AM–11 PM"
action:
  - block_category: ["Games", "Social Media", "Communication"]
  - mute_notifications: true
  - coach_message: "Exam week focus mode ON. You've got this!"
```

### Study Streaks

Over the semester, Alice's focus streak reaches 67 days. She earned:

```
⭐ 5,000 wallet points
🏆 "Consistent Studier" achievement
📜 Audit chain: 67 consecutive days
```

She can optionally export her audit chain to share with parents or advisors.

### Coaching Personalization

Coachy learns Alice's patterns:

- She procrastinates most on coding assignments → Suggest Pomodoro timer
- She studies best 8–11 AM → Suggest focus blocks in that window
- Her energy dips after 3 PM → Less aggressive coaching in evening

## Pain Point Resolution

### Procrastination on Midterm

In week 5, Alice delays starting the CS Midterm. Coachy detects:

```
💭 "I noticed you haven't started the Midterm project yet, 
and it's due in 5 days. That's usually when procrastination hits. 
Want to break it down into smaller steps?"

[Yes, help!] [I'll start tomorrow] [Let me think]
```

Alice taps "Yes, help!" and Coachy suggests:

```
1. Read project spec (30 min) — Today, 3 PM
2. Set up Git repo (15 min) — Tomorrow morning
3. Implement part 1 (2 hours) — Tomorrow, after lecture
```

Alice marks each subtask complete. When she finishes part 1 on schedule, Coachy celebrates—reinforcing momentum.

## Semester End

Alice's final stats:

```
Total focus time: 187 hours
Assignments on time: 32/32 ✓
Highest streak: 67 consecutive days
GPA benefit: +0.3 (attributed to consistent study)
```

She exports her audit chain as a PDF summary to share with her parents.

## Key Moments

| Moment | Emotion | Outcome |
|--------|---------|---------|
| Rule blocking TikTok successfully | Pride | Keeps using FocalPoint |
| 7-day streak celebration | Excitement | Shares with friends |
| Procrastination detection nudge | Relief | Breaks down big task |
| 67-day streak achievement | Joy | Becomes advocate; recommends to classmates |

## Success Criteria

✓ Alice never missed a Canvas deadline after week 1  
✓ Consistent 3–4 hour daily focus (vs. 1–2 hour baseline)  
✓ Procrastination incidents dropped from 3/week to 1/month  
✓ Actively uses FocalPoint 6 days/week  
✓ Imported 1 rule pack, created 2 custom rules  

See also: [Canvas Connector Guide](../connectors/canvas), [Rules Reference](../rules/dsl), [Rituals](../rituals/)
