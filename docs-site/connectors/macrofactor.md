---
title: MacroFactor Connector (Aspirational)
description: Integrate nutrition tracking for health-conscious focus rules.
---

# MacroFactor Connector

*This connector is aspirational. Status: Planned for Q4 2026.*

The MacroFactor connector brings your meal tracking and nutrition goals into FocalPoint, enabling health-conscious focus rules and coaching.

## Planned Features

- **Meal logging**: Sync meals to audit chain
- **Macro goals**: React to progress toward protein/carb/fat targets
- **Water intake**: Remind to hydrate during focus sessions
- **Nutritional coaching**: Pair focus sessions with meal planning

## Example Rule

```yaml
name: "Study fuel: Protein break"
trigger:
  event_type: "macrofactor.daily_nutrition.low_protein"
  threshold_grams: 50
action:
  - coach_message: "Low protein today ({{current}}g). Consider a protein-rich snack!"
  - suggest_break: "15 minutes"
    suggestion: "Eat a snack, then return to focus"
```

## Implementation Status

- [ ] MacroFactor API key authentication
- [ ] Daily nutrition sync
- [ ] Macro threshold configuration
- [ ] Coaching message templates

## Roadmap

Part of the broader **"Wellness & Health Integration"** initiative.
