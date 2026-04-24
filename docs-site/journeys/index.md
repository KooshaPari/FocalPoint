---
title: User Journeys
description: Real-world workflows showing how different personas use FocalPoint.
---

# User Journeys

User journeys show real-world workflows from onboarding through long-term usage. These are **personas + scenarios**, not marketing copy.

## Why Journeys?

Journeys help:

- **New users** see how FocalPoint fits their workflow
- **Designers** understand user pain points and solutions
- **Developers** identify feature gaps and opportunities
- **Connector authors** see how their connectors unlock workflows

## Featured Journeys

### Student on Canvas

Alice uses FocalPoint to manage Canvas assignments and build study habits.

- **Duration**: 2-month semester
- **Setup time**: 20 minutes
- **Key features**: Canvas connector, study focus rules, morning brief, evening shutdown
- **Outcome**: Never missed a deadline; 67-day focus streak

**[Read journey →](./student-canvas)**

### Developer with GitHub

Bob uses FocalPoint for deep code review work and feature development.

- **Duration**: 2 months
- **Setup time**: 15 minutes
- **Key features**: GitHub connector, PR review focus, context-switch detection, commit streaks
- **Outcome**: Better PR feedback; 45-day commit streak

**[Read journey →](./developer-github)**

### Connector SDK Developer

Carol builds a Todoist connector and publishes it to the marketplace.

- **Duration**: 2–3 weeks
- **Setup time**: 30 minutes (SDK setup)
- **Key features**: OAuth, event emission, testing framework, marketplace publishing
- **Outcome**: First connector published; adopted by 23 users

**[Read journey →](./connector-sdk-author)**

## Common Themes

Across all journeys, users follow a pattern:

1. **Onboarding** (15–30 min): Install, grant permissions, link first connector
2. **First rule** (10–15 min): Create a custom rule or import a pack
3. **Daily ritual** (5–10 min): Morning brief, work, evening shutdown
4. **Habit formation** (Week 2–3): Streaks, Coachy coaching, rule customization
5. **Advanced usage** (Month 2+): Custom connectors, complex rules, data export

## Personas Not Yet Documented

We plan to add journeys for:

- **Sleep/Wellness User** — Apple Health integration, evening lockdowns, sleep coaching
- **Parent (Family Sharing)** — Setting screen time rules for kids, monitoring compliance
- **Productivity Enthusiast** — Multi-connector, calendar-driven, complex rule packs
- **Athlete** — Health metrics, training blocks, recovery coaching

Contributions welcome! See [Contribution Guide](../governance/contributing).

## Using Journeys in Design

When proposing a new feature:

1. **Pick a journey** (or create one)
2. **Add the feature** to the workflow
3. **Trace user impact**: How does this change the outcome?
4. **Identify gaps**: What blocks the workflow?

Example:

> **Proposed feature**: "Export focus stats as PDF"
> 
> **Relevant journey**: Student on Canvas
> 
> **Impact**: At end of semester, Alice can share her audit chain + focus stats with parents or advisors.
> 
> **User value**: Demonstrates consistent study habits; useful for applications/scholarships.

## Contributing a Journey

To document a new user journey:

1. Choose a persona (name, context, goal, pain point)
2. Map the journey from onboarding to long-term usage
3. Include: setup time, key moments, pain point resolution, success metrics
4. Write 1500–2500 words
5. Submit as a PR to `/journeys/`

See **[Contributing Guide](../governance/contributing)** for details.

## Feedback

Did a journey miss your use case? Open an [issue](https://github.com/KooshaPari/FocalPoint/issues) or [discussion](https://github.com/KooshaPari/FocalPoint/discussions).
