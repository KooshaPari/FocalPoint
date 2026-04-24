# FocalPoint Accessibility Guide

FocalPoint is built with accessibility-first design principles. This guide summarizes our a11y commitments and implementation patterns.

## VoiceOver Support

All interactive elements and images include descriptive `accessibilityLabel` modifiers so VoiceOver users can navigate the app without visual cues.

### Implementation Pattern

```swift
Image(systemName: "flame.fill")
    .accessibilityLabel(String(localized: "Active streak", defaultValue: "Active streak"))
```

### Key Labeled Elements

- **Buttons:** Every button has a label describing its action (e.g., "Dismiss nudge", "Add task").
- **Icons:** Icon-only buttons and informational images carry labels (e.g., "Timer icon", "Checkmark").
- **Images:** Mascot images (Coachy) are labeled with character name and context (e.g., "Coachy mascot").
- **Complex elements:** Multi-view containers use `.accessibilityElement(children: .combine)` to group related content for single VoiceOver announcement.

## Dynamic Type Support

All text respects user's Dynamic Type preference via system font styles. Fixed-size fonts are replaced with dynamic alternatives:

### Bad (Fixed Size)
```swift
.font(.system(size: 28, weight: .bold))  // Ignores Dynamic Type
```

### Good (Dynamic)
```swift
.font(.title.weight(.bold))  // Respects system scaling
```

**Status:** All Views fixed to use dynamic font sizes; `.title`, `.title2`, `.headline`, `.body`, `.caption` styles used throughout.

## Reduce Motion Respect

Animations respect `AccessibilityManager.isReduceMotionEnabled`:

```swift
@Environment(\.accessibilityReduceMotion) var reduceMotion

withAnimation(reduceMotion ? nil : .easeInOut(duration: 0.3)) {
    // Animated state change
}
```

**Areas with motion:**
- Mascot breathing animations (disable when reduce motion is on)
- Tab transitions and navigation effects
- Nudge banner slide-ins

## High-Contrast Mode

All color-coded information includes shape and text redundancy; color is never the sole indicator:

- **Status states:** "✓ Done" (green checkmark + text), not just green.
- **Conflicts:** Hard conflicts show red border + "Hard" label text.
- **Priority:** "🔥 Priority 1" (icon + text), not just accent color.

## Haptics Opt-Out

Users can disable haptic feedback via Settings > Mascot > Haptics:

```swift
@AppStorage("app.hapticEnabled") var hapticEnabled: Bool = true

if hapticEnabled {
    UIImpactFeedbackGenerator(style: .medium).impactOccurred()
}
```

## Implementation Checklist

- [x] All images have `.accessibilityLabel`
- [x] Icon-only buttons have labels
- [x] Complex interactive views use `.accessibilityElement(children:)`
- [x] All text uses dynamic font sizes (no fixed `size:` param)
- [x] Animations respect `.accessibilityReduceMotion`
- [x] Color-coded info has text/shape redundancy
- [x] Haptics are toggleable in Settings

## Testing

### Automated Checks
Run manual accessibility audit via:
1. Enable VoiceOver (Settings > Accessibility > Vision > VoiceOver)
2. Navigate each tab and verify labels are clear
3. Test Dynamic Type at +200% size (Settings > Accessibility > Display & Text Size)
4. Enable Reduce Motion (Settings > Accessibility > Motion > Reduce Motion) and verify animations disable
5. Toggle Dark Mode and verify contrast is maintained

### Manual Routes
- **HomeView:** Verify stat chips are labeled (Streak, Credits, Bypass)
- **StatsView:** Focus card should announce "Focus time this week: {value}"
- **TasksView:** Empty state Coachy should be identified
- **WalletView:** Balance card uses `.accessibilityValue` for credit count
- **RitualsView:** Conflict badges announce hard/soft type; priority rows are grouped
- **Settings:** Info button labeled; sync buttons announce their purpose

## Related Documentation

- [Accessibility API Reference](https://developer.apple.com/documentation/swiftui/accessibility)
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [iOS Accessibility Design Practices](https://developer.apple.com/design/human-interface-guidelines/accessibility)
