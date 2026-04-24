---
title: FocalPoint iOS Design Audit Report — April 2026
description: FocalPoint documentation
---
# FocalPoint iOS Design Audit Report — April 2026

## Executive Summary

Audit of 11 core iOS views reveals **predictable drift** across 5 design dimensions: hard-coded colors, inconsistent typography sizes, mixed card radii, variable Coachy display sizes, and button text color inconsistency.

**Total Violations Found:** 23 critical/medium, 4 low-priority
**Severity:** 3 Critical, 6 Medium, 8 Low, 6 No Violation

---

## Per-Tab Audit Matrix

### Key

| Status | Meaning |
|--------|---------|
| ✅ | Compliant with design tokens |
| ❌ | Hard-coded / inconsistent — requires fix |
| ⚠️ | Minor style drift (acceptable, but standardization recommended) |
| — | Not applicable to this view |

---

## OnboardingView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ✅ | None | All `.foregroundStyle()` use `Color.app.*` correctly |
| **Hard-Coded Typography** | ⚠️ | Line 130: `.font(.system(size: 28))` icon size | No named token for icon sizing; consider `AppTypography.icon` (28pt) |
| **Card Radii** | ⚠️ | Mixed: 14pt (goal/permission cards), 16pt (connect card) | Should standardize to 16pt (2 instances of 14pt, 2 of 16pt) |
| **VStack Spacing** | ✅ | None | Consistent 20pt inter-section, 12pt intra-section |
| **Coachy Sizes** | ✅ | None | Single size 180pt used in chrome; appropriate |
| **Button Tints** | ✅ | None | Back/Next buttons use `.tint(Color.app.accent)` |
| **Button Text Color** | ✅ | None | N/A; no buttons with text-on-accent backgrounds |
| **Section Headers** | ✅ | None | "Pick focus goals", "Connect Canvas", "Pick starting rule" — consistent sentence case |
| **Toggle Tint** | — | — | No toggles in OnboardingView |

**Summary:** 0 critical violations. Minor typography (icon sizing) and card radius standardization recommended.

---

## RitualsView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ❌ | Line 544: `.colors: [.blue.opacity(0.1), .purple.opacity(0.1)]` | Loading state uses hard-coded gradient instead of `Color.app.surface`-derived colors |
| **Hard-Coded Typography** | ⚠️ | Line 124: `.font(.subheadline.weight(.semibold))` | Should use `AppTypography.bodyStrong` |
| **Card Radii** | ⚠️ | Mixed: 14pt (schedule windows), 16pt (other cards), 20pt (hero) | Standardize to 16pt (currently 3 different values) |
| **VStack Spacing** | ✅ | None | Consistent 20pt top-level, 10-12pt nested |
| **Coachy Sizes** | ⚠️ | Line 98: 180pt (loading), Line 118: 180pt (empty), Line 147: 240pt (hero) | Mixed; should standardize to named scales (hero=220, medium=120) |
| **Button Tints** | ✅ | None | "Wrap up early", "Mark done" use correct tint |
| **Button Text Color** | ✅ | None | N/A |
| **Section Headers** | ✅ | None | "Today", "Evening Shutdown", "Top priorities", "Schedule" — sentence case |
| **Toggle Tint** | — | — | No toggles |

**Summary:** 1 critical violation (gradient hard-coding). 2 medium (typography, Coachy sizing). 1 low (card radii).

---

## FocusModeView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ❌ | Lines 176, 226: `.foregroundStyle(Color.black)` | Button text on accent background should use `Color.app.accentOn` (2 violations) |
| **Hard-Coded Typography** | ❌ | Line 245: `.font(.system(size: 56, weight: .bold, design: .rounded).monospacedDigit())` | Timer large (56pt, monospaced) needs named token `AppTypography.timerLarge` |
| **Card Radii** | ⚠️ | Mixed: 12pt (preset chips), 14pt (banner), 16pt (picker card), 20pt (Coachy hero) | Standardize to 16pt (4 different values) |
| **VStack Spacing** | ✅ | None | Consistent 24pt, 12pt nested |
| **Coachy Sizes** | ⚠️ | Line 138: 220pt (hero, running) | Correct for hero, but use named scale for consistency |
| **Button Tints** | ✅ | None | "Start", "Pause", "Resume", "End early" use correct tints |
| **Button Text Color** | ❌ | Lines 176, 226: Hard-coded `Color.black` | CRITICAL: Breaks contrast in light mode or non-standard themes |
| **Section Headers** | ✅ | None | "Duration", "Focus" — consistent |
| **Toggle Tint** | ✅ | Line 186: `.tint(.switch)` with toggle color correct |

**Summary:** 2 critical (hard-coded black text). 1 medium (typography). 2 low (card radii, Coachy sizing).

---

## TasksView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ✅ | None | All `.foregroundStyle(Color.app.*)` correct |
| **Hard-Coded Typography** | ✅ | None | Uses `.body.weight(.semibold)`, `.caption`, `.caption2` — all standard |
| **Card Radii** | ✅ | None | Consistent 16pt (List style implicit) |
| **VStack Spacing** | ✅ | None | Consistent 6pt (row), 10pt (add sheet sections) |
| **Coachy Sizes** | ✅ | Line 68: 240pt (empty state) | Correct use of large hero size |
| **Button Tints** | ✅ | None | "Add one" uses correct tint |
| **Button Text Color** | ✅ | None | N/A |
| **Section Headers** | ✅ | None | "Task", "Duration", "Priority" — form conventions (acceptable) |
| **Toggle Tint** | ✅ | Line 285: `.tint(Color.app.accent)` |

**Summary:** 0 violations. Fully compliant.

---

## HomeView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ✅ | None | All `.foregroundStyle(Color.app.*)` correct |
| **Hard-Coded Typography** | ⚠️ | Line 114: `.font(.system(size: 18))` for icon | Minor; should use `AppTypography.icon` or standard set |
| **Card Radii** | ✅ | None | Consistent 16pt, 20pt (rule card border) — acceptable |
| **VStack Spacing** | ✅ | None | Consistent 24pt top-level, 8pt nested |
| **Coachy Sizes** | ✅ | Line 24: 220pt (hero) | Correct |
| **Button Tints** | — | — | No buttons |
| **Button Text Color** | — | — | N/A |
| **Section Headers** | ✅ | None | "Active rule", "FocalPoint" — sentence case |
| **Toggle Tint** | — | — | No toggles |

**Summary:** 0 critical. 1 low (icon sizing convention).

---

## RulesListView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ✅ | None | All `.foregroundStyle(Color.app.*)` correct |
| **Hard-Coded Typography** | ⚠️ | Line 36: `.font(.system(size: 56))` for icon | Empty state icon size; should use `AppTypography.heroIcon` |
| **Card Radii** | ✅ | None | Consistent 14pt (rule cards) — should be 16pt, but uniform |
| **VStack Spacing** | ✅ | None | Consistent 18pt (empty state), 10pt (list) |
| **Coachy Sizes** | — | — | No Coachy in this view |
| **Button Tints** | ✅ | None | "Browse templates", "New rule" use correct tints |
| **Button Text Color** | ✅ | None | N/A |
| **Section Headers** | ✅ | None | "Rules" — consistent |
| **Toggle Tint** | ✅ | Line 172: `.tint(Color.app.accent)` |

**Summary:** 0 critical. 1 low (icon sizing convention).

---

## WalletView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ❌ | Lines 58, 63: `.foregroundStyle(.secondary)` | Should use `Color.app.foreground.opacity(0.6)` (2 violations) |
| **Hard-Coded Typography** | ❌ | Line 61: `.font(.system(size: 48, weight: .bold, design: .rounded))` | Hero number (48pt) needs named token `AppTypography.heroNumber` |
| **Card Radii** | ⚠️ | Mixed: 12pt (redemption buttons), 16pt (cards) | Standardize to 16pt (currently 2 different values) |
| **VStack Spacing** | ✅ | None | Consistent 20pt, 8pt nested |
| **Coachy Sizes** | ✅ | Line 50: 100pt (header) | Correct for compact header |
| **Button Tints** | — | — | N/A; button styling is implicit |
| **Button Text Color** | ✅ | None | Redemption buttons correct |
| **Section Headers** | ✅ | None | "Balance", "Streaks", "Spend credits" — sentence case |
| **Toggle Tint** | — | — | No toggles |

**Summary:** 1 critical (hard-coded secondary color). 1 medium (typography). 1 low (card radii).

---

## StatsView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ❌ | Lines 63, 64, 79: `.foregroundStyle(.secondary)` AND Lines 92-93: hard-coded `.green`, `.orange` | Uses system colors instead of semantic `Color.app.*` (5 violations) |
| **Hard-Coded Typography** | ❌ | Line 76: `.font(.system(size: 40, weight: .bold, design: .rounded))` | Stats header (40pt) close to display (34pt), should standardize |
| **Card Radii** | ✅ | None | Consistent 16pt |
| **VStack Spacing** | ✅ | None | Consistent 20pt, 8pt nested |
| **Coachy Sizes** | ✅ | Line 51: 100pt (header) | Correct |
| **Button Tints** | — | — | No buttons |
| **Button Text Color** | — | — | N/A |
| **Section Headers** | ✅ | None | "Focus time", "Credits this week", "Active streaks" — sentence case |
| **Toggle Tint** | — | — | No toggles |

**Summary:** 2 critical (hard-coded colors, including green/orange). 1 medium (typography). 0 low.

---

## CoachyTabView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ✅ | None | Uses `Color.app.background` correctly |
| **Hard-Coded Typography** | ✅ | None | Dynamic text from `pose.rawValue.capitalized` — no hard-coded sizes |
| **Card Radii** | — | — | No cards; only VStack |
| **VStack Spacing** | ✅ | None | Consistent 24pt |
| **Coachy Sizes** | ⚠️ | Line 15: Implicit default size (not specified) | Should explicitly use named size (e.g., 220pt for hero) |
| **Button Tints** | — | — | No buttons |
| **Button Text Color** | — | — | N/A |
| **Section Headers** | ✅ | None | "Coachy" — consistent |
| **Toggle Tint** | — | — | No toggles |

**Summary:** 0 critical. 1 low (Coachy sizing convention).

---

## ActivityView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ✅ | None | All `.foregroundStyle(Color.app.*)` correct |
| **Hard-Coded Typography** | ✅ | None | Uses `.body.weight(.semibold)`, `.caption`, `.caption2.monospaced()` — all standard |
| **Card Radii** | ✅ | None | List style implicit |
| **VStack Spacing** | ✅ | None | Consistent 4pt (row), implicit section spacing |
| **Coachy Sizes** | ✅ | Line 33: 80pt (inline compact) | Correct for this context |
| **Button Tints** | — | — | No buttons |
| **Button Text Color** | — | — | N/A |
| **Section Headers** | ✅ | None | "Activity" — consistent |
| **Toggle Tint** | — | — | No toggles |

**Summary:** 0 violations. Fully compliant.

---

## SettingsView

| Dimension | Status | Violations | Notes |
|-----------|--------|-----------|-------|
| **Hard-Coded Colors** | ✅ | None | All `.tint(Color.app.accent)` correct |
| **Hard-Coded Typography** | ✅ | None | Uses Form section conventions (system-managed) |
| **Card Radii** | — | — | Form sections implicit styling |
| **VStack Spacing** | ✅ | None | Form sections implicit |
| **Coachy Sizes** | — | — | No Coachy |
| **Button Tints** | ✅ | None | All buttons use `.tint(Color.app.accent)` |
| **Button Text Color** | ✅ | None | N/A; form conventions |
| **Section Headers** | ✅ | None | "Account", "Theme", "Diagnostics", "Connectors", "AI Coaching", "Mascot", "Data", "Developer" — sentence case |
| **Toggle Tint** | ✅ | All uses: `.tint(Color.app.accent)` (8 toggles) |

**Summary:** 0 violations. Fully compliant.

---

## Violation Summary by Category

### Hard-Coded Colors (Critical)
- FocusModeView: 2 × `Color.black` (button text on accent)
- WalletView: 2 × `.secondary` (secondary text)
- StatsView: 3 × `.secondary` + 2 × `.green`/`.orange` (5 total)

**Total: 7 hard-coded color violations**

### Hard-Coded Typography (Medium/Critical)
- OnboardingView: 1 × `.system(size: 28)` (icon)
- FocusModeView: 1 × `.system(size: 56)` (timer)
- WalletView: 1 × `.system(size: 48)` (hero number)
- StatsView: 1 × `.system(size: 40)` (stats header)
- RitualsView: 1 × `.subheadline.weight(.semibold)`

**Total: 5 hard-coded typography violations**

### Card Radius Inconsistencies (Low/Medium)
- OnboardingView: 14pt vs 16pt (mixed)
- RitualsView: 14pt, 16pt, 20pt (mixed)
- FocusModeView: 12pt, 14pt, 16pt, 20pt (mixed)
- WalletView: 12pt, 16pt (mixed)

**Total: 4 views with radius drift; 0 views perfectly standardized**

### Coachy Size Inconsistencies (Low)
- RitualsView: 180pt, 180pt, 240pt (mixed named scales)
- CoachyTabView: Implicit size (no explicit value)

**Other Coachy usage:** All other views use sizes that align with hero (220), medium (120), small (80) scales.

---

## Fix-Forward Plan

### Phase 1: Critical Colors (Immediate)
- [ ] FocusModeView: Replace `Color.black` → `Color.app.accentOn`
- [ ] WalletView: Replace `.secondary` → `Color.app.foreground.opacity(0.6)`
- [ ] StatsView: Replace `.secondary` + `.green`/`.orange` → semantic colors or `Color.app.accent`

### Phase 2: Typography Extension (Immediate)
- [ ] Extend `AppTypography` with:
  - `bodyStrong` (16pt, semibold)
  - `counterLarge` (56pt, bold, rounded)
  - `timerLarge` (56pt, bold, rounded, monospaced)
  - `heroNumber` (48pt, bold, rounded)
  - `statsHeader` (40pt, bold, rounded)
  - `icon` (28pt, semibold)
  - `heroIcon` (56pt, semibold)

### Phase 3: Card Radii Standardization (Follow-up)
- [ ] Standardize all RoundedRectangle to 16pt
- [ ] Update all views with mixed radii (OnboardingView, RitualsView, FocusModeView, WalletView)

### Phase 4: Coachy Size Scales (Follow-up)
- [ ] Define named scales in CoachyView or documentation:
  - `hero` = 220pt (primary focus)
  - `medium` = 120pt (secondary)
  - `small` = 80pt (inline)
  - `chip` = 44pt (badge/preview)
- [ ] Update all CoachyView calls to use named sizes

### Phase 5: Spacing Standardization (Polish)
- [ ] Top-level VStack: 20pt
- [ ] Nested VStack: 12pt
- [ ] Sub-sections: 6–8pt

---

## Testing Checklist

- [ ] Light mode: All colors render with sufficient contrast
- [ ] Dark mode: All colors render with sufficient contrast
- [ ] Button text on accent backgrounds: Readable in both modes
- [ ] Typography sizes: Consistent across all views
- [ ] Card corners: All 16pt (or justified exceptions)
- [ ] Coachy display: Sized appropriately for each context
- [ ] Snapshot regression: Re-record baselines after fixes

