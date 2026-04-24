---
title: Apple Health Connector (Aspirational)
description: Integrate sleep, activity, and mindfulness data into wellness-focused rules.
---

# Apple Health Connector

*This connector is aspirational. Status: Planned for Q2 2026.*

The Apple Health connector brings your sleep data, activity rings, and mindfulness sessions into FocalPoint, enabling wellness-focused rules and coaching.

## Planned Features

- **Sleep tracking**: Detect sleep debt and trigger evening lockdowns
- **Activity rings**: Reward focus sessions with activity goals
- **Mindfulness minutes**: Track meditation and breathing sessions
- **Workouts**: Sync exercise sessions to rule conditions

## Example Rule

```yaml
name: "Sleep debt recovery"
trigger:
  event_type: "health.sleep_debt.high"
  threshold_hours: 5

action:
  - show_lockdown: "evening_focus"
  - coach_message: "Your sleep debt is {{hours}}h. Let's wind down early tonight."
  - schedule_ritual: "morning_brief"
    at: "next_morning"
```

## Requires

- **HealthKit framework** (iOS only)
- User authorization to read sleep, activity, mindfulness data
- iOS 17+

## Architecture Notes

Apple Health data is sensitive. The connector will:

1. Request only the minimum required permissions (sleep, activity, mindfulness)
2. Process all data locally; never send to cloud
3. Store a local cache (SQLite) for rule evaluation
4. Allow users to audit what's being read via Settings → Privacy → HealthKit

## Implementation Status

- [ ] HealthKit permission request
- [ ] Sleep data sync
- [ ] Activity ring integration
- [ ] Local caching strategy
- [ ] Privacy audit
- [ ] iOS testing

See [Architecture: FFI Topology](../architecture/ffi-topology) for how platform-specific permissions are bridged.
