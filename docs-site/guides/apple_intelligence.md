# Apple Intelligence Writing Tools Integration

FocalPoint on iOS 18+ leverages **Apple Intelligence** to transform coaching messages and help users refine their focus journeys—all on-device, all private.

## What is Apple Intelligence?

Apple Intelligence is Apple's on-device AI system for iOS 18+. It provides powerful text transformation capabilities—rewriting, summarization, and tone shifting—without sending any data to cloud servers. All processing happens securely on your device.

## FocalPoint Features

### 1. Coaching Message Rewriting

When Coachy shares a coaching message, you can invoke Apple Intelligence's **Writing Tools** to:
- **Rewrite** the message in a different style
- **Proofread** for grammar and clarity
- **Expand** for more detail or **Condense** for brevity

**How it works:**
- iOS 18+ automatically adds a Writing Tools context menu to Coachy's message bubbles
- Tap and hold (or use the Edit menu) to see rewrite options
- Select a transformation; the on-device model applies it instantly
- No network call; no data leaves your device

### 2. Morning Brief Tone Shift

The Morning Brief summary includes tone transformation buttons:

| Tone | Style | Best For |
|------|-------|----------|
| **Friendly** | Warm and conversational | Casual, encouraging check-in |
| **Coach** | Direct and motivational | Action-oriented push |
| **Concise** | Short and punchy | Quick summary |
| **Motivational** | Inspiring and energetic | Big wins and goals |

Tap any button to rewrite the brief in that tone. The result stays on your device for review before sharing or copying.

### 3. Rule Explanation Simplification (ELI5)

Complex rule configurations can be confusing. Tap the **ELI5** button next to a rule explanation to:
- Get a simplified, child-friendly version
- Understand what the rule does in plain language
- Share easier explanations with family or teammates

**Example:**
- **Original:** "This rule fires on focus:session_completed when the session duration exceeds 25 minutes and cooldown has elapsed. It awards wallet credits based on the focus intensity multiplier."
- **Simplified:** "When you finish a focused work session that's longer than 25 minutes, the app gives you reward points."

## Privacy Model

### What Stays On-Device
- **All text transformation** (rewriting, summarization, tone shifts)
- **All processing** happens securely on the iOS device's neural engine
- **No network requests** to Apple, FocalPoint servers, or any third party

### What Never Happens
- Your coaching messages are never sent to cloud servers
- Your focus sessions, rules, or task details are never analyzed externally
- Your device identifiers or location data are never included
- No logs are stored on remote servers

### Control & Transparency
- **Settings > Mascot > Coaching Intelligence** lets you toggle Writing Tools on/off
- Disabling the toggle turns off all Apple Intelligence features instantly
- You can always tap and hold to see what transformation options are available

## System Requirements

- **iOS 18+** — Apple Intelligence features are not available on earlier iOS versions
- **On-device processing** — Requires Neural Engine (available on iPhone 15 Pro, iPhone 16, and later models)
- Optional feature — If your device doesn't support iOS 18, tone buttons and ELI5 options are hidden gracefully

## Settings

### Enable/Disable

Navigate to **Settings > Mascot > Coaching Intelligence** to toggle:
- **On (default):** Writing Tools and tone shifts are available
- **Off:** All Apple Intelligence features are disabled; original text only

### Privacy Explanation

The settings screen includes a clear privacy notice:

> iOS 18+: Rewrite coaching messages, shift tone for morning briefs, and simplify rule explanations using on-device Apple Intelligence. All processing stays on your device—no data is sent to Apple servers.

## User Experience

### Coaching Messages
1. Coachy sends a message
2. Tap the message and hold, or use the Edit menu
3. Select "Rewrite," "Proofread," or another Apple Intelligence option
4. The on-device model applies the transformation
5. Review and accept, or try another tone

### Morning Brief
1. Open the Morning Brief
2. Review the default summary
3. Optional: tap a tone button (Friendly, Coach, Concise, Motivational)
4. The brief rewritten in that tone appears instantly
5. Copy to clipboard or share

### Rule Explanation
1. View a rule's details
2. Look for the **ELI5** button in the explanation section
3. Tap to generate a simplified version
4. Review the child-friendly explanation
5. Restore the original if needed

## Performance

- **Latency:** 1-3 seconds for tone transformation (on-device processing time)
- **No network dependency** — works offline
- **Low battery impact** — neural engine is power-efficient
- **Memory:** Minimal; on-device models are optimized for mobile

## Future Enhancements

Potential expansions (on-device only):
- **Voice command rewriting** — transform coaching via Siri
- **Custom tone profiles** — save your favorite tone combinations
- **Batch transformation** — rewrite multiple briefs at once
- **Accessibility improvements** — reading level adjustment for dyslexia-friendly text

---

## Troubleshooting

### Writing Tools Don't Appear

**Cause:** iOS version < 18 or toggle is disabled  
**Fix:** Check Settings > Mascot > Coaching Intelligence is ON. iPhone 15 Pro/16+ required.

### Tone Buttons Don't Appear

**Cause:** Same as above, or device doesn't support Apple Intelligence  
**Fix:** Verify iOS 18+ and Coaching Intelligence is enabled.

### Transformation Takes Too Long

**Cause:** Neural Engine is busy with other tasks  
**Fix:** Try again in a few seconds. No network call is happening.

### ELI5 Looks Odd

**Cause:** Complex rule descriptions may produce unexpected simplifications  
**Fix:** Tap "Show Original" to restore the full explanation.

---

## Learn More

- [Apple Intelligence Overview](https://www.apple.com/intelligence/) — Apple's official site
- [iOS 18 Features](https://www.apple.com/ios/ios-18-preview/) — Release notes
- FocalPoint Settings > Mascot > Coaching Intelligence — On-device privacy details
