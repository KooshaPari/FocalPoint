---
title: Connectors
description: Integrate FocalPoint with external services like Canvas LMS, Google Calendar, and GitHub.
---

# Connectors

Connectors are the bridge between FocalPoint and external productivity platforms. They feed events (assignment deadlines, calendar blocks, GitHub notifications, sleep data) into the rule engine.

## Shipping Connectors

### Canvas LMS

The Canvas connector syncs assignment deadlines, course schedules, and submission milestones directly from your Canvas instance.

- **Status**: Shipping (v1.0)
- **Setup time**: ~5 minutes
- **Event types**: Assignment created, deadline approaching, grade posted, submission due
- **More**: [Canvas Connector Guide](./canvas)

## Aspirational Connectors (Planned)

| Connector | Purpose | Est. Priority |
|-----------|---------|---------------|
| **Google Calendar** | Sync calendar blocks and meeting prep time | Q2 2026 |
| **Apple Health** | Track sleep, activity, mindfulness for wellness rules | Q2 2026 |
| **Todoist** | Sync task deadlines and project milestones | Q3 2026 |
| **GitHub** | PR reviews, issue assignments, commit deadlines | Q3 2026 |
| **MacroFactor** | Sync meal tracking for nutrition-focused focus rules | Q4 2026 |
| **YNAB** | Budget deadlines and financial goal tracking | Q4 2026 |

## Building Your Own Connector

Use the [Connector SDK](../connector-sdk/) to build a custom integration. You'll define:

1. **Manifest** — Metadata (name, version, OAuth scopes)
2. **Event schema** — What events your connector emits
3. **Auth flow** — OAuth 2.0 or API token handling
4. **Tests** — Unit + integration test suite

See [Connector SDK Spec](../connector-sdk/) for the full guide.

## Verification & Marketplace

Once you've built a connector, you can submit it for verification. FocalPoint maintains a marketplace of community-built connectors with three tiers:

- **Trusted** — Reviewed by FocalPoint core, pinned in app
- **Verified** — Passed security audit and functional tests
- **Community** — Listed as-is; user installs at own risk

See [Verification Tiers](../ecosystem/verification-tiers) for requirements.
