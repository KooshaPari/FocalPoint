---
title: "User Journey: Developer with GitHub"
description: How developers use FocalPoint to manage PR reviews, issue assignments, and coding streaks.
---

# User Journey: Developer with GitHub

## Persona

**Name**: Bob  
**Context**: Full-stack engineer, 3 active repos, GitHub organization owner  
**Goal**: Deep focus for code review and feature work; ship quality code  
**Pain point**: Notifications interrupt flow; context switching between repos  

## Week 1: Onboarding

### Day 1: Installation (8 min)

Bob downloads FocalPoint. He:

1. Grants FamilyControls entitlement
2. Creates a basic rule: "Block Slack during coding sessions"
3. Skips connectors (explores later)

### Day 2–3: GitHub Connection (5 min)

Bob links his GitHub account:

1. Settings → Connectors → GitHub → Enable
2. OAuth redirects to GitHub
3. Selects repos to monitor: `myorg/backend`, `myorg/frontend`, `myorg/cli`
4. Grants API permissions

FocalPoint now tracks:

- PR ready for review notifications
- Issue assignments
- Milestone deadlines
- Commit activity

### Day 4–5: Code Review Rule (10 min)

Bob creates a custom rule for PR reviews:

```yaml
name: "PR Review Focus Session"
trigger:
  - event_type: "github.pr.ready_for_review"
    repository: "myorg/backend"
condition:
  - time_window: { start: "10:00", end: "12:00" }
  - not_in_focus: false
action:
  - show_focus_view: "code_review"
  - block_app:
      - "com.slack"
      - "com.discord"
      - "com.figma.mac"
  - set_focus_duration: "60 minutes"
  - coach_message: "PR from {{event.author}}: {{event.title}}"
  - log_audit: "Started PR review session"
```

He tests it. Slack gets muted. ✓

## Week 2–3: Deep Work Establishment

### Morning Check-in (3 min)

Bob opens FocalPoint at 9 AM:

```
📊 GitHub Activity:
  • 3 PRs waiting for review
  • 1 new issue assigned
  • Milestone "v1.2" due in 5 days

📅 Schedule:
  • 10 AM: Team standup
  • 1 PM: Architecture discussion

🎯 Focus opportunities:
  1. 9–10 AM: Quick PR review (60 min)
  2. 11 AM–1 PM: Feature work (120 min)
  3. 2 PM: Async code review (90 min)

🔥 Streak: 3 days
```

### PR Review Session (10 AM)

A notification appears:

```
👀 PR Ready for Review
Backend team: "Add async database pooling"
Comments: 2 | Changes: 450 lines
👉 [Start review] [Later] [Dismiss]
```

Bob taps "Start review":

- Slack is muted
- Focus timer starts (60 min)
- GitHub opens in focus mode (other apps dimmed)
- Coachy watches for context switching

Bob spends 55 minutes reviewing the PR. He provides detailed feedback. Once done, Coachy celebrates:

```
💪 Thorough review! 6 comments, 2 suggestions.
Quality work! 
```

### Afternoon Feature Work (2 PM)

Bob starts implementing a new feature. He manually enables a longer focus session:

```
🎯 Feature: "Add webhook support"
Duration: 180 minutes
Status: In progress
```

After 90 minutes, Coachy suggests:

```
⏰ You've been focused for 90 minutes. 
Time for a break?
[Take 15 min break] [Keep going] [End session]
```

Bob taps "Take 15 min break". He steps away, comes back, and resumes focus. After 180 minutes, the session ends and Coachy celebrates:

```
🔥 3-hour focus session completed!
You're in the zone. 
Take a real break now—you've earned it!
```

### Evening Reflection (5 min)

At 7 PM, Bob completes the Evening Shutdown ritual:

```
📊 CODE ACTIVITY
Focus time: 4h 30m (goal: 4h) ✓ +30m bonus
PR reviews: 2 completed
Commits: 3 feature work
Code quality: 95% (linted, tested)

Q: "What blocked your focus?"
A: [Brief Slack distraction at 11 AM]

Q: "Tomorrow's priority?"
A: "Finish webhook feature + write tests"

💭 Coachy: "Strong day! 
You nailed those PRs and got deep into the webhook work.
Tomorrow, consider disabling Slack entirely during feature work."

[Export audit] [Done]
```

## Week 4: Milestone Tracking

### Milestone Deadline Approaching

Coachy detects that the "v1.2" milestone is due in 2 days:

```
⚠️ Milestone "v1.2" due in 2 days!
Remaining items: 3 PRs to review, 1 feature to finish

Want to schedule intensive focus blocks for the next 2 days?
[Yes, schedule] [I have it under control] [Snooze]
```

Bob taps "Yes, schedule". FocalPoint adds:

```yaml
trigger:
  - schedule: "tomorrow 9 AM–6 PM"
action:
  - show_focus_view: "deadline_sprint"
  - block_category: ["Games", "Social Media"]
  - limit_notifications: "urgent_only"
  - coach_message: "Milestone sprint day 1. Let's ship!"
```

### Deadline Day

The next day, Bob focuses intensively:

- 9–12 PM: Review 2 PRs
- 1–3 PM: Lunch + refocus
- 3–6 PM: Finish webhook feature + tests

By end of day, all v1.2 PRs are merged. Coachy celebrates:

```
🎉 MILESTONE SHIPPED!
v1.2 is live. Great work, team!
Your contribution: 4 PRs reviewed, 1 major feature.
```

## Month 2: Advanced Usage

### Custom Coaching Rules

Bob creates a personalized coaching rule:

```yaml
name: "Pomodoro Pairing"
trigger:
  - event_type: "github.pr.review.long_session"
    duration_minutes: 120
action:
  - coach_message: "You've been reviewing for 2 hours.
    Time for a Pomodoro break?
    [25-min timer] [Keep going] [End session]"
  - schedule_break: "25 minutes"
```

### Commit Streaks

Over 45 days, Bob maintains a commit streak. His audit chain shows:

```
45-day commit streak
Commits: 187
PR reviews: 52
Avg code review time: 18 min
Code quality score: 96%
```

He shares this summary with his team during sprint retro.

### GitHub Actions Integration (Aspirational)

In v1.1, Bob will be able to link FocalPoint to his GitHub Actions CI/CD:

```
Trigger: "CI pipeline failed"
Action: "Urgent focus block for debugging"
```

For now, he manually enables focus when debugging fails.

## Pain Point: Context Switching

### Situation

Bob works on 3 repos simultaneously. He's in:

- Reviewing backend PR (repo A)
- Implementing feature (repo B)
- Debugging CLI test (repo C)

After each repo switch, FocalPoint notices:

```
⚠️ Context switched 6 times in 45 minutes.
This often breaks flow. 

Consider:
  1. Batch repo tasks (all backend PRs, then feature work)
  2. Set a "single-repo focus" rule for feature work
  3. Schedule reviews in a dedicated time block (10 AM)

[Create focus rule] [Later]
```

Bob taps "Create focus rule". FocalPoint drafts:

```yaml
name: "Single-Repo Focus: Feature Work"
trigger:
  - user_initiated: true
action:
  - show_focus_view: "feature_work"
  - whitelist_app: ["GitHub", "VSCode", "Terminal"]
  - block_category: ["Games", "Social Media"]
  - coach_message: "Feature focus: repo B only.
    Batch reviews for 2 PM."
```

Bob customizes and saves it. Over the next week, context switches drop from 6 to 2 per focus session.

## Success Metrics (Month 2)

```
Focus sessions: 24
Avg focus duration: 87 minutes
PR review quality: +12% (measured by feedback scores)
Time-to-merge: -15% (faster reviews)
Commit streak: 45 days
Code quality: 96%
Context switches per session: 2 (was 6)
```

Bob tells his team: "FocalPoint made code review better. I'm actually thoughtful instead of skimming."

## Key Moments

| Moment | Emotion | Outcome |
|--------|---------|---------|
| First PR review in focus mode | Calm/focused | Provides better feedback |
| Coachy detects context switching | Surprised | Changes workflow for better focus |
| Milestone shipped on time | Pride | Recommends to teammates |
| 45-day commit streak | Motivated | Continues using FocalPoint |

See also: [GitHub Connector Guide](../connectors/github), [Rules Reference](../rules/dsl), [Focus Modes](../getting-started/first-rule)
