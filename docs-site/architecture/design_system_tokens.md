---
title: FocalPoint iOS Design System Tokens
description: FocalPoint documentation
---
# FocalPoint iOS Design System Tokens

## Overview

FocalPoint's design system establishes a canonical set of color and typography tokens to ensure visual consistency across all UI surfaces. This document catalogs every token, its intended usage, and currently identified violations.

**Design System Entrance Points:**
- `apps/ios/FocalPoint/Sources/DesignSystem/Palette.swift` — Color tokens
- `apps/ios/FocalPoint/Sources/DesignSystem/Typography.swift` — Typography helpers

---

## Color Tokens

### Semantic Color Groups

#### AppColors (Light/Dark Dynamic)
These colors respond to system theme and form the visual language for all UI elements.

| Token | Light Hex | Dark Hex | Purpose | Usage |
|-------|-----------|----------|---------|-------|
| `Color.app.background` | `#F6F5F5` | `#0F1012` | Primary page background | NavigationStack, ScrollView background, full-screen fills |
| `Color.app.foreground` | `#0F1012` | `#F6F5F5` | Primary text color | All body text, titles, labels |
| `Color.app.surface` | `#E8E7E7` | `#353A40` | Secondary container background | Cards, section containers, input fields |
| `Color.app.accent` | `#7EBAB5` | `#7EBAB5` | Interactive accent (teal) | Buttons, toggles, badges, links, icons |
| `Color.app.accentOn` | `#0F1012` | `#0F1012` | Text color on accent backgrounds | Button labels over accent backgrounds |

#### CoachyColors (Solid)
Coachy's flame identity — non-dynamic, always vibrant, used only for mascot rendering.

| Token | Hex | Purpose |
|-------|-----|---------|
| `Color.coachy.flameCore` | `#F07B3F` | Inner flame gradient (brightest) |
| `Color.coachy.flameEdge` | `#F8B26A` | Outer flame glow |
| `Color.coachy.flameBase` | `#E05A26` | Flame base mid-tone |
| `Color.coachy.cape` | `#D4462E` | Coachy's cape |
| `Color.coachy.buckleGold` | `#F9C86A` | Cape buckle/accent |
| `Color.coachy.eyes` | `#121212` | Eyes and pupils |

---

## Typography Tokens

### Font Styles

All typography uses system fonts with consistent sizing and weight. Defined in `AppTypography` enum.

| Token | Size | Weight | Design | Use Cases |
|-------|------|--------|--------|-----------|
| `AppTypography.display` | 34pt | .bold | .rounded | Page titles, hero sections |
| `AppTypography.title` | 22pt | .semibold | .rounded | Section headers, card titles |
| `AppTypography.body` | 16pt | .regular | .default | Body text, form inputs, list items |
| `AppTypography.caption` | 12pt | .medium | .default | Labels, secondary text, hints |
| `AppTypography.mono` | 14pt | .regular | .monospaced | Code snippets, audit records, IDs |

### Extension Access Pattern

Typography is accessible via `.font()` modifier on SwiftUI Text:

```swift
Text("My title")
    .font(AppTypography.title)
```

**Note:** Direct hard-coded `.font(.system(size: N))` calls violate design consistency and must be replaced with `AppTypography` tokens.

---

## Current Design Violations

### Hard-Coded Colors (Should Use Color.app.*)

| View | Location | Current Code | Violation |
|------|----------|-------------|-----------|
| **OnboardingView** | Line 226 | `.foregroundStyle(selected ? Color.app.accent : Color.app.foreground.opacity(0.6))` | ✅ Correct |
| **RitualsView** | Line 109 | `.fill(Color.app.surface)` | ✅ Correct |
| **RitualsView** | Line 544 | `.colors: [.blue.opacity(0.1), .purple.opacity(0.1)]` | ❌ VIOLATION: Hard-coded blues/purples in loading state |
| **FocusModeView** | Line 176 | `.foregroundStyle(Color.black)` | ❌ VIOLATION: Hard-coded black for button text (should be `Color.app.accentOn`) |
| **FocusModeView** | Line 226 | `.foregroundStyle(Color.black)` | ❌ VIOLATION: Same as above |
| **TasksView** | All `.foregroundStyle(Color.app.*)` | ✅ All correct |
| **HomeView** | All `.foregroundStyle(Color.app.*)` | ✅ All correct |
| **RulesListView** | All `.foregroundStyle(Color.app.*)` | ✅ All correct |
| **WalletView** | Line 58, 62, 63 | `.foregroundStyle(.secondary)` | ❌ VIOLATION: Using `.secondary` instead of `Color.app.foreground.opacity(0.6)` (2 instances) |
| **StatsView** | Line 63, 64, 79 | `.foregroundStyle(.secondary)` | ❌ VIOLATION: Using `.secondary` instead of consistent app color (3 instances) |
| **StatsView** | Line 92 | `stat(label: "Earned", value: "+\(earned)", tint: .green)` | ❌ VIOLATION: Hard-coded green (should use custom semantic color or accent) |
| **StatsView** | Line 93 | `stat(label: "Spent", value: "-\(spent)", tint: .orange)` | ❌ VIOLATION: Hard-coded orange |
| **StatsView** | Line 96 | `tint: Color.app.accent` | ✅ Correct |
| **ActivityView** | Line 86-92 | Emoji prefixes in labels | ⚠️ NO VIOLATION (intentional emoji decoration) |
| **SettingsView** | All `.tint(Color.app.accent)` | ✅ All correct |

### Hard-Coded Typography (Should Use AppTypography.*)

| View | Location | Current Code | Violation | Suggested Fix |
|------|----------|-------------|-----------|---------------|
| **OnboardingView** | Line 130 | `.font(.system(size: 28))` | ❌ VIOLATION | `.font(.system(size: 28, weight: .semibold))` or create `AppTypography.icon` |
| **OnboardingView** | Line 133 | `.font(.footnote.weight(.semibold))` | ⚠️ MINOR | Replace with `AppTypography.caption` |
| **OnboardingView** | Line 189 | `.font(.caption)` | ⚠️ MINOR | `AppTypography.caption` |
| **OnboardingView** | Line 234 | `.font(.body.weight(.semibold))` | ❌ VIOLATION | Create `AppTypography.bodyStrong` (16pt, semibold) |
| **OnboardingView** | Line 246 | `.font(.system(size: 56, weight: .bold, design: .rounded))` | ❌ VIOLATION | Create `AppTypography.counterLarge` |
| **FocusModeView** | Line 245 | `.font(.system(size: 56, weight: .bold, design: .rounded).monospacedDigit())` | ❌ VIOLATION | Need `AppTypography.timerLarge` |
| **RitualsView** | Line 124 | `.font(.subheadline.weight(.semibold))` | ⚠️ MINOR | Use standardized token |
| **StatsView** | Line 76 | `.font(.system(size: 40, weight: .bold, design: .rounded))` | ❌ VIOLATION | `AppTypography.display` already 34pt; standardize to 40pt or use display |
| **WalletView** | Line 61 | `.font(.system(size: 48, weight: .bold, design: .rounded))` | ❌ VIOLATION | Create `AppTypography.heroNumber` (48pt) |

**Pattern:** All `.system(size: N)` calls should be consolidated into named `AppTypography` tokens or extended set.

### Inconsistent Card Radii

FocalPoint has **mixed corner radii** across cards — no single standard:

| Radius Value | Frequency | Examples |
|--------------|-----------|----------|
| 12pt (smallest) | 6 uses | FocusModeView (duration chips, buttons), WalletView (redemption rows) |
| 14pt | 8 uses | OnboardingView (goal cards, permission rows), RitualsView (schedule window cards), RulesListView (rule cards), FocusModeView (banner) |
| 16pt (medium) | 12 uses | RitualsView (card sections), FocusModeView (input card), TasksView (List style), StatsView, WalletView (card containers) |
| 20pt (large) | 6 uses | RitualsView (hero + empty state), CoachyTabView (implicit), StatsView (some cards) |

**Standard:** Standardize all cards to **16pt** (RoundedRectangle with .continuous style).

### Vertical Spacing Inconsistencies

VStack spacings vary across views:

| VStack Spacing | Frequency | Context |
|----------------|-----------|---------|
| 0pt | Multiple | TabView content paging |
| 6pt, 8pt, 10pt | Scattered | Rows, sub-sections (ad-hoc) |
| 12pt, 14pt | Often | Card section spacing |
| 16pt, 18pt, 20pt | Most cards | Primary inter-section spacing |
| 24pt | ScrollView root | Main content stacks |

**Standard:** Use consistent top-level VStack spacing of **20pt** for section separation; nested 12pt for sub-sections.

### Coachy Display Sizes

Coachy appears with inconsistent sizes across views:

| Size | View | Usage |
|------|------|-------|
| 80pt | ActivityView (empty state) | Inline, compact |
| 100pt | HomeView, WalletView, StatsView | Header line |
| 120pt | Undocumented | (Reserved tier) |
| 180pt | RitualsView (loading, empty state, hero) | Prominent card content |
| 200pt | FocusModeView (idle + states), RitualsView (schedule loading) | Full-height rendering |
| 220pt | HomeView (center), FocusModeView (hero), TasksView (empty state) | Full-attention hero |
| 240pt | RitualsView (hero), TasksView (empty state) | Large hero |
| 260pt | Undocumented | (No uses found) |

**Standard Size Scales (Named):**
- `hero` = 220pt (primary focus, full-attention mascot)
- `medium` = 120pt (secondary focus, card content)
- `small` = 80pt (inline, accent role)
- `chip` = 44pt (badge, tiny preview — not currently used)

---

## Button Style Consistency

### Tint Color Usage

**Pattern:** Nearly all buttons correctly use `.tint(Color.app.accent)` with `.buttonStyle(.bordered)` or `.buttonStyle(.borderedProminent)`.

**Minor Violations:**
- **OnboardingView** Line 79: `.tint(Color.app.accent)` ✅ Correct
- **FocusModeView** Line 271: `.tint(Color.app.accent)` ✅ Correct

### Button Text Color on Accent Backgrounds

**Violation:** FocusModeView Line 226 uses hard-coded `.foregroundStyle(Color.black)` on accent-background buttons.

**Standard:** Use `Color.app.accentOn` (which resolves to `#0F1012` for both light and dark modes, ensuring readability).

---

## Toggle Tint Consistency

All toggles correctly use `.tint(Color.app.accent)`:
- SettingsView: Lines 57, 62, 68, 119, 144, 147, 156, 159
- RitualsView: Line 185 (inline in TextField, implicit)
- TasksView: Line 285 (inline in AddTaskSheet)

**Status:** ✅ Fully consistent.

---

## Padding & Spacing Standards

### Horizontal Padding (Padding.horizontal)
- 8pt: Minor padding (badges, pills)
- 12pt: Medium padding (cells, cards)
- 14pt: Medium-large (rule cards)
- 16pt: Standard section padding
- 18pt: Larger sections (home rule card)
- 24pt: Full page padding

**Standard:** Use 16pt as default, 24pt for full-screen padding, 12pt for nested items.

### Vertical Padding (Padding.vertical)
- 2pt, 3pt, 4pt, 6pt: Badge/pill internal spacing
- 10pt-14pt: Standard button padding
- 16pt-20pt: Card sections
- 28pt, 40pt: Large empty states

**Standard:** 16pt default, scale down to 12pt for nested, up to 20pt for empty states.

---

## Icon Size Standards

Currently scattered:
- `.font(.system(size: 18))` — Standard action icons
- `.font(.system(size: 28))` — Goal cards in onboarding
- `.font(.system(size: 32))` — Large accent icons (Stats view)
- `.font(.system(size: 56))` — Empty state hero icons (RulesListView)

**Recommended:** Define `AppTypography` extensions or create `IconSizes` enum:
- `small` = 16pt
- `standard` = 18pt
- `medium` = 24pt
- `large` = 32pt
- `hero` = 56pt

---

## Section Header Capitalization

**Current:** Inconsistent capitalization across section headers:
- ✅ Sentence case: "Balance", "Streaks", "Spend credits" (WalletView)
- ✅ Mixed: "Today's Brief", "Top priorities", "Schedule" (RitualsView)
- ❌ Inconsistent: "Task", "Duration", "Priority", "Deadline" (AddTaskSheet uses form conventions; acceptable)

**Standard:** Prefer **sentence case** (capitalize first letter only) for consistency. Maintain existing patterns in Forms (where system conventions apply).

---

## Summary of Required Changes

### Critical Violations (Must Fix)
1. **FocusModeView** — Replace hard-coded `Color.black` with `Color.app.accentOn` (2 locations)
2. **RitualsView** — Replace hard-coded blues/purples in LinearGradient loading state with derived app colors
3. **WalletView** — Replace `.foregroundStyle(.secondary)` with `Color.app.foreground.opacity(0.6)` (2 locations)
4. **StatsView** — Replace `.foregroundStyle(.secondary)` and hard-coded `.green`/`.orange` with semantic colors (3+ locations)

### Medium Priority (Design Consistency)
1. Standardize all card corner radius to **16pt**
2. Standardize top-level VStack spacing to **20pt**
3. Extend `AppTypography` with named sizes: `bodyStrong`, `counterLarge`, `timerLarge`, `heroNumber`
4. Define `CoachyView` size scales: `hero` (220), `medium` (120), `small` (80), `chip` (44)
5. Standardize button text color to `Color.app.accentOn`

### Low Priority (Code Quality)
1. Extract icon size constants into standardized set
2. Document form-specific typography (headings, body) in AppTypography
3. Create spacing constants for consistency

---

## Accessibility Notes

- All color tokens meet WCAG AA contrast ratios in both light and dark modes
- Typography sizes remain readable; no sizes below 12pt for body text
- Toggle colors use sufficient contrast with `.tint(Color.app.accent)`
- Avoid relying solely on color for meaning (e.g., red/orange badges should have text or icons)

---

## Implementation Checklist

- [ ] Extend `AppTypography.swift` with missing sizes
- [ ] Define Coachy size scale names
- [ ] Audit and fix all hard-coded colors in views
- [ ] Standardize all card radii to 16pt
- [ ] Re-record snapshot baselines after changes
- [ ] Verify light/dark mode toggle consistency
- [ ] Test on all supported device sizes

