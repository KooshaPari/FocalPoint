# Rule Suggestions: Smart Pattern Detection

## Overview

FocalPoint's rule suggestion engine analyzes your audit trail and connector events over the past 30 days to detect patterns and recommend custom rules. Suggestions are powered by **four heuristics** that identify missed opportunities and consistent behaviors.

**Privacy Model**: All analysis happens locally on your device. No data is sent to external services.

## How It Works

Every week, the engine runs in the background and scans:
- **Audit chain**: Your wallet grants, penalty escalations, streak changes, rule firings
- **Connector events**: Task completions (Canvas), app usage patterns (local), GitHub actions, calendar events
- **Time windows**: Activities grouped into focused patterns (e.g., "always focus at 9 AM on Tuesdays")

Suggestions appear in **Settings → Coachy Suggests** with a confidence score and human-readable explanation. You can:
- **Apply**: Create the rule immediately (persisted to your rule store)
- **Dismiss**: Hide it for this cycle (dismissed suggestions don't reappear)

## The Four Heuristics

### H1: Scheduled Focus Sessions

**Detects**: You start focus sessions at the same time consistently.

**Example**: "You start focus sessions on Tue/Thu at 9:00 AM (4 times). Consider automating this schedule."

**Suggested Rule**: `schedule:0 9 ? ? tue → FocusSessionStart { duration_minutes: 90 }`

**Confidence**: Based on repetition count; 4+ occurrences at the same (weekday, hour) = 0.95 confidence.

### H2: Missing Celebrations

**Detects**: You complete tasks frequently but rarely celebrate wins.

**Example**: "You completed 7 tasks but celebrated in only 1 of them. Celebrating wins boosts morale and retention."

**Suggested Rule**: `event:TaskCompleted → Notify + StreakIncrement { name: 'daily_wins' }`

**Confidence**: If celebration ratio < 30%, suggests a celebration rule. Confidence: 0.7.

### H3: Missed Check-ins

**Detects**: Gaps of 24+ hours between your daily check-ins.

**Example**: "You've missed 2 daily check-ins in the past 30 days. An earlier reminder (8:00 AM) might help you stay consistent."

**Suggested Rule**: `schedule:0 8 ? ? * → Notify { message: 'Time for your daily check-in!' }`

**Confidence**: Based on gap frequency. Confidence: 0.65.

### H4: Unlinked Actions

**Detects**: You close GitHub PRs without granting yourself credits.

**Example**: "You've closed 5 GitHub PRs but only granted credits for 2 of them. Consider automating credit grants for merged code."

**Suggested Rule**: `event:github_pr_merged → GrantCredit { amount: 10 }`

**Confidence**: If < 50% of PRs get grants, suggests automation. Confidence: 0.6.

## Privacy & Data

- **Local-only**: No data leaves your device. Suggestions are computed using on-device audit records and events.
- **No ML**: This is rule-based heuristics (v0), not machine learning. No training data, no external inference calls.
- **Dismissed suggestions**: Persisted locally; never uploaded.
- **Weekly refresh**: Background task runs at a configurable time (default: Sunday 8 AM, no wakeup guarantee).

## Disabling Suggestions

To disable suggestion generation:
1. **Settings → Coachy Suggests → Turn Off Weekly Suggestions**

This stops the background task but does not delete previously dismissed suggestions.

To clear all dismissed suggestions:
1. **Settings → Coachy Suggests → Reset Suggestions**

This forces a fresh analysis on the next weekly run.

## Example Workflow

1. **Sunday 8 AM**: Background task runs `suggester.fetch(window_days: 30)`.
2. **Settings opens**: You see "Coachy suggests..." card with 2 suggestions.
   - Scheduled Focus Sessions (0.95 confidence, rationale: 4 occurrences)
   - Unlinked Actions (0.6 confidence, rationale: 3 PRs without grants)
3. **You tap "Apply"** on Scheduled Focus Sessions → rule created and enabled.
4. **You tap "Dismiss"** on Unlinked Actions → hidden for this cycle.
5. **Next week**: Unlinked Actions reappears if the pattern persists.

## Implementation Notes

### iOS Settings Integration

The "Coachy suggests..." entry point is a card in the Settings tab:

```swift
// Pseudo-Swift
import FocalPoint

struct SettingsView: View {
    @StateObject var suggester = SuggesterViewController()
    
    var body: some View {
        List {
            // ... other settings ...
            
            if !suggester.suggestions.isEmpty {
                Section("Coachy Suggests") {
                    SuggestionCardView(suggestions: suggester.suggestions)
                        .onTapGesture { suggester.applyOrDismiss($0) }
                }
            }
        }
    }
}
```

### Background Task Registration

iOS background task (in `AppDelegate`):

```swift
func registerSuggesterTask() {
    let request = BGProcessingTaskRequest(identifier: "com.focalpoint.suggest")
    request.requiresNetworkConnectivity = false
    request.requiresExternalPower = false
    
    try? BGTaskScheduler.shared.submit(request)
}
```

### FFI Methods

```rust
// From focus_ffi:
pub fn fetch(window_days: u32) -> Vec<RuleSuggestionDto>;
pub fn apply(suggestion_id: String) -> Result<()>;
pub fn dismiss(suggestion_id: String) -> Result<()>;
```

## Future Enhancements

- **H5**: Penalty patterns — detect escalation triggers and suggest preventive rules.
- **H6**: Ritual compliance — detect skipped morning briefs and suggest earlier reminders.
- **H7**: Credit economy — detect spending patterns and suggest multiplier adjustments.
- **ML-based**: Transition to supervised learning once 30+ days of user data is available.
