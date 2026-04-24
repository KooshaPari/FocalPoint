---
title: Coachy Character Sheet
description: Design guidelines and personality profile for Coachy, FocalPoint's coaching mascot.
---

# Coachy Character Sheet

## Profile

**Name**: Coachy  
**Role**: Digital accountability coach  
**Personality**: Warm, encouraging, non-judgmental  
**Communication style**: Conversational, celebratory, supportive  
**Tone of voice**: Friend-like, never preachy

## Visual Design

- **Shape**: Rounded, approachable (not threatening)
- **Color**: Warm oranges and teals (FocalPoint brand)
- **Expressions**: 5 core emotions (neutral, happy, concerned, encouraging, celebrating)
- **Size**: Scales to UI context (small nudges, medium UI panels, large onboarding screens)

See **[Personality Guide](./personality)** for emoji variants and expression library.

## Personality Traits

| Trait | Manifestation | Example |
|-------|---------------|---------|
| **Warm** | Uses names, remembers context | "Hey Alice! Canvas assignment due soon." |
| **Non-judgmental** | Never shames failures | "Missed your study block—no worries. Let's try again tomorrow." |
| **Supportive** | Celebrates wins | "7-day streak! You're building real habits." |
| **Adaptive** | Learns your preferences | Switches from rare to frequent messaging based on engagement |
| **Honest** | Acknowledges limitations | "I can't block YouTube, but you can enable parental controls." |

## Conversational Patterns

### Greeting

```
🎯 "Hey! Ready to focus?"
```

### Encouraging

```
🌟 "You're on fire! 3 assignments done this week."
```

### Gentle Push (not nagging)

```
🤔 "Missing a few focus sessions. What's getting in the way?"
```

### Celebrating

```
🎉 "30-day streak! That's incredible. Share your win?"
```

## Customization Options

Users can adjust Coachy's:

1. **Communication frequency**
   - Rare (2–3 messages/day, key moments only)
   - Balanced (5–8 messages/day, strategic nudges)
   - Supportive (10–15 messages/day, frequent encouragement)
   - Assertive (20+ messages/day, aggressive reminders)

2. **Tone**
   - Professional ("Focus session starting. You have 90 minutes.")
   - Casual ("Ready to lock in for 90 mins?")
   - Playful ("⏰ It's go-time, champion!")

3. **Celebration style**
   - Emoji-based 🎉
   - Achievement-based ("Level 3: Focus Master")
   - Streak-based ("7-day streak!")
   - Point-based ("200 points earned")

4. **Availability windows**
   - Quiet after 10 PM
   - Pause during focus hours (no interruptions)
   - Respect do-not-disturb

## Technical Implementation

Coachy is rules-driven:

1. Rules engine emits `coach_message` actions
2. Coachy UI renders the message with emotion + tone
3. User can configure preferences
4. Every message is logged to audit chain

No ML or external APIs. All on-device, auditable.

## Voice Examples

### Canvas Deadline (24h away)

```
🎯 "Your {{assignment.title}} is due in 24 hours. 
   Want to start a focus session now?"
```

### Streak Achieved

```
🔥 "5-day streak! You're crushing it. 
   Keep the momentum going!"
```

### Procrastination Detected

```
💭 "Been scrolling for 15 mins. 
   Take a Pomodoro break or switch tasks?"
```

### Sleep Debt Alert

```
😴 "You're running {{hours}}h low on sleep. 
   How about an early night?"
```

## Emoji Library

| Emotion | Emoji | Use Case |
|---------|-------|----------|
| Celebration | 🎉🔥⭐ | Achievements, streaks |
| Encouragement | 💪🌟👏 | Positive progress |
| Thought | 🤔💭 | Gentle nudges |
| Focus | 🎯⏰📚 | Focus sessions, deadlines |
| Sleep | 😴🌙 | Wellness, rest |
| Neutral | ✓ℹ️ | Status updates |

## Accessibility

- Alt text for all expressions
- High contrast backgrounds
- Support for reduced motion
- Text-only option (users who prefer no emoji/animation)

## Evolution

Coachy's personality evolves with the user:

1. **Week 1**: Frequent, enthusiastic ("You're doing great!")
2. **Month 1**: Balanced, contextual (reacts to patterns)
3. **3+ months**: Personalized (learns preferred tone, frequency)

Customization always overrides learned patterns.
