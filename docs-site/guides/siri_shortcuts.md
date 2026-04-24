# FocalPoint Siri Shortcuts

FocalPoint includes 6 Siri Shortcuts that let you manage focus tasks, sessions, and wallet balance entirely through voice commands.

## Setup

1. Open FocalPoint on your iOS device
2. Enable App Intents in Settings > FocalPoint
3. Ask Siri: "Open Shortcuts and search for FocalPoint"
4. The 6 shortcuts will appear in Siri Suggestions

## Available Shortcuts

### 1. Add Focus Task

**Voice Command:** "Hey Siri, add focus task [title]"

Add a new task to your FocalPoint inbox with optional priority and duration.

**Parameters:**
- `title` (required): Task description (e.g., "buy milk", "review PR")
- `priority` (optional): Task priority level; defaults to "Normal"
- `duration` (optional): Estimated minutes to complete

**Example:** "Hey Siri, add focus task buy milk"

### 2. Start Focus Session

**Voice Command:** "Hey Siri, start focus session"

Begin a focus session with optional duration and rule activation.

**Parameters:**
- `duration` (optional): Session length in minutes; defaults to 25 min (Pomodoro)
- `rule` (optional): Specific rule name to activate (e.g., "Deep Work", "No Slack")

**Examples:**
- "Hey Siri, start focus session"
- "Hey Siri, start focus for 45 minutes"
- "Hey Siri, start focus with Deep Work rule"

### 3. Check FocalPoint Balance

**Voice Command:** "Hey Siri, check FocalPoint balance"

Get your current wallet balance: available credits + pending credits.

**Parameters:** None

**Response:** "Balance: 240 credits, Pending: 15"

### 4. Sync FocalPoint

**Voice Command:** "Hey Siri, sync FocalPoint"

Trigger an immediate sync with all connected services.

**Parameters:** None

**Response:** "Sync completed: 42 events processed"

### 5. When's My Next Focus

**Voice Command:** "Hey Siri, when's my next focus"

Query your next scheduled focus window or rule trigger.

**Parameters:** None

**Examples:**
- "When's my next focus"
- "What's my next focus session"
- "Show next focus time"

**Response:** "Next focus: deep_work at 2025-04-25T14:30:00Z"

### 6. Log Focus Note

**Voice Command:** "Hey Siri, log focus note [content]"

Append a note to your audit chain for tracking focus observations.

**Parameters:**
- `note` (required): Note content (e.g., "I got distracted by Slack", "Good flow state")

**Examples:**
- "Hey Siri, log focus note I stayed focused for 45 minutes"
- "Hey Siri, add note Got interrupted by a meeting"

## Tips

- **Combine with Automation:** Create iOS Shortcuts automations to run FocalPoint intents on schedule (e.g., "9 AM: Start focus session")
- **Voice Feedback:** Use "Speak" action in Shortcuts to read responses aloud
- **Background Execution:** All intents run in the background without opening the app
- **Siri Suggestions:** FocalPoint learns which shortcuts you use most and offers them proactively

## Troubleshooting

**Shortcuts not appearing in Siri?**
1. Open Settings > FocalPoint
2. Toggle App Intents off, then back on
3. Wait 5-10 seconds for Siri index to update
4. Try asking again

**"Permission Denied" error?**
- Grant FocalPoint necessary permissions in Settings > Privacy

**Shortcut fails silently?**
- Check that FocalPoint app is not forcefully closed
- Verify you have internet connectivity (some intents require sync)
- Check app logs in Settings > FocalPoint > Diagnostics
