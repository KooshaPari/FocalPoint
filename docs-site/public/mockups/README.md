# Mockups Directory

This directory contains **device mockups** — proposed UI designs for the FocalPoint app used during the proposal, planning, and development phases.

## Purpose

These mockups serve as **reference visuals** to communicate proposed UI designs to:
- Stakeholders during feature proposals
- Design teams during planning sessions
- Developers as implementation guides

## Structure

```
mockups/
├── journeys/
│   ├── student-canvas/      # Student Canvas journey UI mockups
│   ├── developer-github/    # GitHub integration UI mockups
│   ├── connector-sdk/       # Connector SDK UI mockups
│   ├── rituals/             # Morning/Evening ritual UI mockups
│   └── focus-session/       # Focus session UI mockups
└── features/               # Feature-specific mockups
    └── [feature-name]/
```

## Status

These are **proposed designs** and do not represent actual implemented functionality. Implementation may differ significantly from these mockups.

## Relationship to Real Recordings

| Asset Type | Location | Purpose |
|------------|----------|---------|
| **Mockups** | `/mockups/` | Proposed designs (planning stage) |
| **Supporting Graphics** | `/images/` | Conceptual illustrations (documentation polish) |
| **Real Recordings** | `/recordings/` | Actual app screenshots (VLM-verified) |

## Creating Mockups

1. Use the iPhone 14 Pro template (390x844 viewport)
2. Include "MOCKUP" watermark
3. Add descriptive title and description
4. Place in appropriate subdirectory
5. Reference in design docs with appropriate caveats

## Example Usage

```markdown
::: info Proposed UI Design
![Onboarding Flow](/mockups/journeys/student-canvas/onboarding-permissions.svg)
*Subject to change based on design feedback and user research.*
:::
```

---

**Note**: Once features are implemented, real screenshots should replace these mockups using the [Journey Recording Harness](../research/13-user-journey-harness-2026.md).
