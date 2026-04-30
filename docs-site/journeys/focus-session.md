---
title: "User Journey: Focus Session — PR Review"
description: Deep focus session for code review with Coachy break suggestions and session completion.
verification:
  manifest: "/recordings/focus-session/manifest.json"
  status: "planned"
  last_verified: null
---

# User Journey: Focus Session — PR Review

## Persona

**Name**: Bob (Developer)
**Context**: Incoming PR notification during work
**Goal**: Provide thoughtful code review without context switching
**Pain point**: Notifications interrupt deep work; reviews get rushed

## What You'll See

Recording of a focus session: PR notification → focus activation → 90-min work → break suggestion → completion.

![Focus Session Overview](/images/journeys/focus/session-overview.svg)

<JourneyViewer manifest="/recordings/focus-session/manifest.json" />

## Phase 1: Notification

### PR Ready for Review

Bob is in VS Code working on a feature. A notification appears:

```
👀 PR READY FOR REVIEW

@alice: "Add async database pooling"
Repository: myorg/backend
Comments: 2 | Changes: 450 lines (+120, -80)

⏱️ Waiting for 2 days — getting stale

[Start Review]  [Later]  [Dismiss]
```

Bob taps "Start Review":

```
🎯 STARTING FOCUS SESSION

Duration: 60 min (or until PR submitted)
Goal: Provide thoughtful review, not a rushed skim
```

## Phase 2: Focus Activation

### Apps Blocked

```
🎯 FOCUS MODE ACTIVE

⏱️ Timer: 60:00 (started)

🔇 Blocked:
  • Slack (1 unread)
  • Discord (2 mentions)
  • Twitter (notifications paused)

🔓 Whitelisted:
  • GitHub (review in progress)
  • VS Code
  • Terminal

💭 Coachy: "You're reviewing @alice's PR.
         Take your time — quality over speed."
```

## Phase 3: Deep Work

### Review in Progress

Bob reviews the PR with full focus. After 25 minutes:

```
⏱️ 35:00 remaining

📝 Review draft:
  • Line 42: Consider adding retry logic
  • Line 78: The connection pool size should be configurable
  • Overall: Clean abstraction, good tests

💭 Coachy: "You're doing great. 35 minutes left.
         No need to rush."
```

After 50 minutes, Bob submits the review:

```
✅ REVIEW SUBMITTED

@alice will be notified.

⏱️ 10:00 saved (finished 10 min early)
📝 Quality: Thoughtful feedback, 4 comments, 2 suggestions
🎯 Streak: +1 PR reviewed today
```

## Phase 4: Coachy's Celebration

### Session Complete

```
🎉 FOCUS SESSION COMPLETE

⏱️ 50 minutes | Quality focus

What you accomplished:
  • Reviewed @alice's PR (450 lines)
  • Left 4 thoughtful comments
  • Helped unblock teammate

🔥 23-day streak maintained!

💭 Coachy: "Excellent review! Your feedback will
         help @alice ship better code. Great work
         taking time to do it right."
```

## Phase 5: Break Suggestion

### 90-Minute Deep Work

Bob starts another focus session for feature work. After 90 minutes:

```
⏰ BREAK SUGGESTION

![Break Suggestion](/images/journeys/focus/break-suggestion.svg)

You've been in deep focus for 90 minutes.
Research shows: breaks improve retention and creativity.

Suggested: 15-minute break
  • Stand up, stretch
  • Get water
  • Look out the window

[Take Break]  [Keep Going]  [End Session]
```

Bob taps "Take Break":

```
☕ BREAK TIME

Timer: 15:00

Stretch, hydrate, reset.
Your brain will thank you.

💭 Coachy: "Great discipline! Breaks are part
         of sustainable productivity."
```

## Verification

::: info Planned
This journey is planned for recording. The JourneyViewer above shows the expected manifest structure.

Track progress: [GitHub Issue #journey-focus-session](https://github.com/KooshaPari/FocalPoint/issues?q=label:journey-focus-session)
:::

## Related Guides

- [Focus Mode](../guides/focus-mode.md)
- [Coachy Debug](../guides/coachy_debug.md)
- [GitHub Connector](../connectors/github.md)
