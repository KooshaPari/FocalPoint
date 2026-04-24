# Siri Coachy Conversation Guide

## Overview

Coachy is FocalPoint's conversational Siri intent that provides personalized coaching feedback on demand. Say "Hey Siri, Coachy" or "Ask Coachy how I'm doing" to get a spoken status update tailored to your current tasks, credits, rituals, and focus streak.

## Setup

### iOS 18+ (Apple Intelligence)
- Coachy automatically uses the native Siri voice for natural, context-aware responses.
- Responses are synthesized on-device with full privacy.

### iOS 17 and Earlier
- Falls back to `AVSpeechSynthesizer` with emotion-tuned voice parameters.
- Pitch and rate adjust dynamically based on response sentiment (see below).

## Voice Phrases

Add any of these phrases to Siri:

| Phrase | Example |
|--------|---------|
| **Direct address** | "Hey Coachy" |
| **Question form** | "Ask Coachy how I'm doing" |
| **Status check** | "Coachy status" |
| **Alternative** | "What's my focus status" |

## Example Exchanges

### Fresh Morning (No Tasks Completed)

**You:** "Hey Siri, Coachy"

**Coachy (morning voice, slightly concerned):**
> "Good morning, Koosha. You have 5 tasks left. You've got 300 credits in the bank. You haven't done your Morning Brief yet."

---

### Mid-Day (All Tasks Done)

**You:** "Ask Coachy how I'm doing"

**Coachy (excited, higher pitch):**
> "You've crushed all your tasks today! You're sitting pretty with 450 credits. You're on a 7-day streak—keep it up!"

---

### Late Evening (Low Credits, Overdue Work)

**You:** "Coachy status"

**Coachy (concerned, lower pitch, slower):**
> "You're running on 25 credits. You have 7 tasks left. You haven't done your Evening Ritual yet."

---

### Zero State (No Tasks, No Credits)

**You:** "What's my focus status"

**Coachy (neutral, encouraging):**
> "You haven't added any tasks yet. You're out of credits. Time to earn some!"

---

## Response Variants

Coachy rotates through 8 different response phrasings based on the day of the week, so responses feel fresh even when checking status multiple times per week:

1. **Greeting + Status** (Monday) — Opens with time-aware greeting
2. **Dashboard style** (Tuesday) — Organized status report format
3. **Conversational detail** (Wednesday) — More narrative, task-first
4. **Question + Status** (Thursday) — Ends with engagement question
5. **Credit-first** (Friday) — Emphasizes wallet state
6. **Task completion focus** (Saturday) — Celebrates progress
7. **Day-aware summary** (Sunday) — Time-of-day aware, streak emphasis
8. **Quick check format** (Monday+1) — Rapid-fire facts

## Voice Tuning

Coachy adapts pitch and speech rate based on emotional context:

| Emotion | Pitch | Rate | Use Case |
|---------|-------|------|----------|
| **Happy / Excited** | +20% | +10% | All tasks done, high credits, long streak |
| **Proud / Encouraging** | +5% | -5% | Good progress, motivational |
| **Supportive / Neutral** | Baseline | Baseline | General status, mixed progress |
| **Focused** | -5% | -10% | Needs concentration, measured tone |
| **Concerned / Tired** | -15% | -15% | Low credits, overdue work, fatigue |

## Privacy & On-Device Processing

- **No network calls:** Coachy uses local state only (tasks, credits, rituals, audit records).
- **No tracking:** No telemetry, no analytics, no external logging.
- **All processing is local:** Voice synthesis happens entirely on your device.
- **Optional ritual data:** If you haven't set up rituals, Coachy gracefully notes "rituals all caught up" and continues.

## Advanced: Multi-Turn Dialog (iOS 18+)

On iOS 18 with Apple Intelligence, Coachy supports follow-up questions:

```
You: "Hey Coachy, what's my balance?"
Coachy: "You've got 450 credits."
You: "Can I do another focus session?"
Coachy: "You have 2 tasks remaining. Start a focus session?"
```

Multi-turn history is **not persisted**; each Siri invocation is stateless.

## Troubleshooting

### Coachy Doesn't Respond

- Ensure FocalPoint app has been opened at least once (to initialize the Rust core).
- Check that you have a valid app group entitlement (or dev fallback to app-local storage).
- Verify Siri is enabled in Settings → Siri & Search.

### Voice Sounds Robotic

- This is normal on iOS 17; upgrade to iOS 18 for Apple Intelligence voices.
- Alternatively, adjust system speech rate in Settings → Accessibility → Speech.

### Response Doesn't Match My Actual State

- Coachy reads from the last sync tick; if you just completed a task, wait 10–20 seconds for the sync engine to pick it up.
- Tap the "Sync Now" button in Settings to force an immediate update.

### Ritual Status Not Showing

- Rituals are optional; if you haven't created any, Coachy defaults to "rituals all caught up".
- Add a ritual in FocalPoint settings to see personalized ritual status in Coachy responses.

## Implementation Details

Coachy is built on three core components:

1. **`CoachyConversationIntent`** — AppIntent that gathers state and delegates response synthesis.
2. **`CoachyResponseSynth`** — Pure function mapping (tasks, credits, streaks, time) → natural language + emotion.
3. **`CoachyVoiceProfile`** — Voice provider that picks system Siri (iOS 18+) or `AVSpeechSynthesizer` with emotion tuning.

See the source code in `Sources/FocalPointApp/Intents/` for implementation details.

---

**Happy coaching!** ✨
