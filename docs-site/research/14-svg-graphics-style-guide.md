# SVG Graphics Style Guide for FocalPoint Documentation

## Overview

This guide codifies the process for creating SVG graphics that make documentation look professional, human-made, and polished. It establishes clear **delineation** between three types of visual assets:

| Type | Purpose | Location | Stage |
|------|--------|----------|-------|
| **Mockups** | Reference visuals for proposals/planning | `public/mockups/` | Planning, Development |
| **Supporting Graphics** | Professional polish for final docs | `public/images/` | Documentation |
| **Real Recordings** | Actual app screenshots via harness | `recordings/` | Verification |

---

## Type 1: Mockups

### Purpose
Mockups are **reference visuals** used during the proposal, planning, and early development stages. They show *what something might look like* without implying it's real functionality.

### When to Use
- Feature proposals to stakeholders
- Planning sessions with design teams
- Technical specifications before implementation
- "Here's what this might look like" scenarios

### Characteristics
- Device frames (iPhone, iPad, macOS)
- UI elements with realistic proportions
- Placeholder content ("Lorem ipsum")
- Blue "MOCKUP" watermark
- Cannot be used as if they represent actual features

### Directory Structure
```
docs-site/public/mockups/
├── features/
│   ├── morning-brief-v1.svg      # Early concept
│   └── morning-brief-v2.svg      # Revised concept
├── connectors/
│   └── canvas-oauth-proposal.svg
└── journeys/
    └── student-journey-wireframe.svg
```

### Example
```markdown
::: info Mockup - Subject to Change
This image represents a proposed UI design.
Actual implementation may differ significantly.
:::
```

---

## Type 2: Supporting Graphics

### Purpose
Supporting graphics are **illustrations and decorative elements** that make documentation feel professional, polished, and human-crafted. They are NOT app screenshots—they are conceptual artwork that enhances documentation.

### When to Use
- Hero banners and page headers
- Persona illustrations (Coachy mascot)
- Connector logos (Canvas, GitHub, Todoist)
- Process diagrams and flowcharts
- Section dividers and visual hierarchy
- Empty state illustrations
- Celebration/achievement graphics

### Characteristics
- Custom illustrations (not device frames)
- Consistent design system (see below)
- Conceptual, not realistic
- Emoji/icons for visual interest
- No "MOCKUP" labels needed

### Directory Structure
```
docs-site/public/images/
├── coachy/                       # Mascot illustrations
│   ├── coachy-happy.svg
│   ├── coachy-celebrating.svg
│   └── ...
├── connectors/                   # Connector logos
│   ├── github.svg
│   ├── canvas-lms.svg
│   └── ...
├── journeys/                     # Journey-specific artwork
│   ├── rituals/
│   │   ├── morning-overview.svg
│   │   └── evening-overview.svg
│   ├── focus/
│   │   └── break-suggestion.svg
│   └── connector-sdk/
│       └── sdk-overview.svg
├── app-icon/
│   └── app-icon.svg
├── journeys-hero.svg             # Hero banners
└── process-diagram.svg          # Flowcharts
```

---

## Type 3: Real Recordings

### Purpose
Real recordings are **actual app screenshots and videos** captured via the recording harness. These represent *true* functionality and are VLM-verified.

### When to Use
- Journey keyframes (documented user flows)
- Feature demonstrations
- Bug reports with visual evidence
- VLM verification of actual behavior

### Directory Structure
```
docs-site/recordings/
├── student-canvas/
│   ├── manifest.json
│   ├── frames/
│   │   ├── frame-001.png
│   │   └── ...
│   ├── video.mp4
│   └── verification.json
└── developer-github/
    └── ...
```

### Capture Process
1. Use XCTest + ScreenCaptureKit for iOS
2. Use VHS for CLI/TUI components
3. Extract keyframes (1 frame / 3 seconds)
4. Run VLM verification
5. Store in `recordings/` with manifest

---

## Supporting Graphics Design System

### Color Palette

| Purpose | Color | Hex |
|---------|-------|-----|
| Background | Dark slate | `#0f172a` |
| Background gradient | Slate to blue-gray | `#1e293b` |
| Card fill | White 10% opacity | `rgba(255,255,255,0.10)` |
| Card stroke | White 15% opacity | `rgba(255,255,255,0.15)` |
| Primary accent | Purple | `#8b5cf6` |
| Success | Green | `#10b981` |
| Info | Blue | `#3b82f6` |
| Warning | Orange | `#f59e0b` |
| Streak/celebration | Gold | `#ffd700` |

### Typography

```xml
<text font-family="SF Pro Display, -apple-system, sans-serif" font-size="24" font-weight="600" fill="#fff">
  Headers - Bold
</text>

<text font-family="SF Pro Display, -apple-system, sans-serif" font-size="14" font-weight="400" fill="rgba(255,255,255,0.6)">
  Body text - Regular
</text>

<text font-family="SF Pro Text, sans-serif" font-size="11" fill="rgba(255,255,255,0.4)">
  Captions - Small, muted
</text>
```

### Dimensions

| Type | Width | Height | Aspect Ratio |
|------|-------|--------|--------------|
| Hero banner | 800 | 400 | 2:1 |
| Section illustration | 700 | 400 | ~1.75:1 |
| Connector logo | 64 | 64 | 1:1 |
| Mascot expression | 120 | 120 | 1:1 |
| Process diagram | 800 | 500 | 1.6:1 |

### Glass Card Pattern

```xml
<defs>
  <linearGradient id="cardGrad" x1="0%" y1="0%" x2="0%" y2="100%">
    <stop offset="0%" style="stop-color:rgba(255,255,255,0.12)"/>
    <stop offset="100%" style="stop-color:rgba(255,255,255,0.04)"/>
  </linearGradient>
</defs>

<rect width="200" height="120" rx="16" fill="url(#cardGrad)" stroke="rgba(255,255,255,0.1)"/>
```

### Glow Filter

```xml
<defs>
  <filter id="glow">
    <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
    <feMerge>
      <feMergeNode in="coloredBlur"/>
      <feMergeNode in="SourceGraphic"/>
    </feMerge>
  </filter>
</defs>
```

---

## SVG Creation Process

### Step 1: Determine Type

Before creating, ask:

1. **Is this an actual app screenshot?** → Use recording harness
2. **Is this a reference UI for planning?** → Mockup → `public/mockups/`
3. **Is this a conceptual illustration?** → Supporting graphic → `public/images/`

### Step 2: Choose Template

| Template | Use When |
|----------|----------|
| `hero-template.svg` | Page headers, journey overviews |
| `card-template.svg` | Feature cards, step indicators |
| `process-template.svg` | Flowcharts, timelines |
| `mascot-template.svg` | Coachy expressions |
| `logo-template.svg` | Connector logos |

### Step 3: Create with Design System

```bash
# Copy template
cp docs-site/.templates/hero-template.svg \
   docs-site/public/images/journeys/rituals/my-new-illustration.svg

# Edit with:
# 1. Update viewBox dimensions
# 2. Change content/colors per design system
# 3. Add appropriate emoji/icons
# 4. Verify accessibility (title, desc)
```

### Step 4: Accessibility

Every SVG must include:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 400" role="img">
  <title>Journey Overview</title>
  <desc>Illustration showing the four phases of student onboarding journey</desc>
  <!-- content -->
</svg>
```

### Step 5: Test

1. Open in browser: `/images/...svg`
2. Check contrast ratios
3. Verify text is readable
4. Ensure responsive scaling

---

## Delineation Examples

### ❌ Wrong
```markdown
![App Screenshot](/images/journeys/student-canvas/onboarding-permissions.svg)
```
*This implies the SVG is a real screenshot when it's a mockup.*

### ✅ Correct

**For Mockups:**
```markdown
::: info Mockup
![Proposed UI](/mockups/features/onboarding-v1.svg)
*Subject to change based on design feedback.*
:::
```

**For Supporting Graphics:**
```markdown
![Morning Ritual Overview](/images/journeys/rituals/morning-overview.svg)
```
*Clearly a conceptual illustration, not a screenshot.*

**For Real Recordings:**
```markdown
<JourneyViewer manifest="/recordings/student-canvas/manifest.json" />
```
*Explicitly loads actual recorded keyframes.*

---

## Quick Reference

| Question | Answer |
|----------|--------|
| Need actual app screenshot? | Use `recordings/` + JourneyViewer |
| Showing UI concept to stakeholders? | Use `mockups/` |
| Making docs look professional? | Use `images/` |
| Coachy mascot? | Always `images/coachy/` |
| Connector logos? | Always `images/connectors/` |
| Process diagrams? | Always `images/` |
| Hero banners? | Always `images/` |

---

**Document created**: April 30, 2026
**Last updated**: April 30, 2026
**Owner**: Documentation team
