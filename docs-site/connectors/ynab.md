---
title: YNAB Connector (Aspirational)
description: Integrate budget goals and financial deadlines into FocalPoint.
---

# YNAB (You Need A Budget) Connector

*This connector is aspirational. Status: Planned for Q4 2026.*

The YNAB connector brings your budget goals and financial deadlines into FocalPoint, enabling financially-conscious focus rules.

## Planned Features

- **Budget deadlines**: Alert when budget review dates approach
- **Overspend warnings**: Trigger focus sessions before overspending
- **Savings goals**: Celebrate hitting savings milestones
- **Monthly reviews**: Schedule budget reflection rituals

## Example Rule

```yaml
name: "Budget review ritual"
trigger:
  event_type: "ynab.budget.monthly_close"
action:
  - schedule_ritual: "monthly_financial_review"
  - coach_message: "Time to review this month's spending!"
```

## Implementation Status

- [ ] YNAB OAuth integration
- [ ] Budget event subscription
- [ ] Threshold configuration

## Roadmap

Part of the broader **"Financial Wellness Integration"** initiative.
