---
title: Sample Rule Packs
description: Pre-built rule collections for common use cases.
---

# Sample Rule Packs

Pre-built rule collections for students, developers, and productivity enthusiasts. Copy and customize for your workflow.

## Student Focus Pack

For students managing Canvas courses and class schedules.

```yaml
---
name: "Canvas Assignment Focus"
trigger:
  - event_type: "canvas.assignment.due_soon"
    hours_until: 24
condition:
  - time_window: { start: "08:00", end: "23:59" }
  - weekday: [Monday, Tuesday, Wednesday, Thursday, Friday]
action:
  - block_app:
      - "com.tiktok.main"
      - "com.instagram.android"
      - "com.snapchat.android"
  - show_focus_view: "study"
  - coach_message: "Canvas assignment due in {{event.hours_until_due}} hours. Focus time!"

---
name: "Morning Study Block"
trigger:
  - schedule: "every weekday at 08:00"
condition:
  - not_in_event: true
action:
  - show_focus_view: "study"
  - whitelist_app:
      - "com.canvas.canvas"
      - "com.google.android.apps.docs"
  - set_focus_duration: "90 minutes"
  - add_reward: 10  # Bonus for early morning focus

---
name: "Afternoon Break Reminder"
trigger:
  - schedule: "every weekday at 15:00"
action:
  - schedule_break: "15 minutes"
  - coach_message: "You've been focused for 3+ hours. Take a break!"
```

## Developer Deep Work Pack

For developers working on GitHub and managing PRs.

```yaml
---
name: "PR Review Focus"
trigger:
  - event_type: "github.pr.ready_for_review"
condition:
  - not_in_focus: false
action:
  - show_focus_view: "code_review"
  - block_app:
      - "com.slack"
      - "com.discord"
  - set_focus_duration: "60 minutes"
  - log_audit: "Started PR review session"

---
name: "Daily Standup Prep"
trigger:
  - schedule: "weekdays at 09:55"
condition:
  - in_focus_mode: false
action:
  - schedule_break: "10 minutes"
  - coach_message: "Standup in 5 minutes. Wrap up your task."
```

## Sleep Wellness Pack

For users managing sleep health via Apple Health integration.

```yaml
---
name: "Evening Lockdown"
trigger:
  - schedule: "every day at 22:00"
condition:
  - not_in_event: true
action:
  - block_app:
      - "com.tiktok.main"
      - "com.instagram.android"
      - "com.netflix.mediaclient"
  - mute_notifications: true
  - show_focus_view: "wellness"
  - coach_message: "Time to wind down. Phone is locked for the night."

---
name: "Sleep Debt Recovery"
trigger:
  - event_type: "health.sleep_debt.high"
    threshold_hours: 5
action:
  - show_coaching:
      message: "You have {{event.sleep_debt_hours}}h sleep debt. Rest tonight?"
      options: ["Enable evening lockdown", "Snooze until tomorrow", "Ignore"]
```

## Productivity General Pack

For calendar-driven productivity.

```yaml
---
name: "Focus Before Meeting"
trigger:
  - event_type: "calendar.meeting.in_30_minutes"
condition:
  - not_in_focus: false
action:
  - schedule_break: "5 minutes"
  - coach_message: "Meeting in 30 min. Wrap up your current task."

---
name: "Post-Meeting Debrief"
trigger:
  - event_type: "calendar.meeting.ended"
action:
  - schedule_check_in:
      delay: "2 minutes"
      message: "Follow-up tasks from that meeting?"
```

## Parent Accountability Pack

For parents using FocalPoint with Family Sharing.

```yaml
---
name: "Homework Time Focus"
trigger:
  - schedule: "every weekday at 16:00"
action:
  - show_focus_view: "study"
  - block_category: ["Games", "Social Media"]
  - set_focus_duration: "120 minutes"
  - coach_message: "Homework time! Let's focus."
```

## Import a Pack

```bash
focalpoint rule pack import student-focus-pack.yaml
```

Or import in-app: Settings → Rules → Import Pack.
