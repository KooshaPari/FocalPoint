---
name: Connector proposal
about: Propose a new connector (Canvas, calendars, health apps, task systems...).
title: "connector: <source>"
labels: ["connector", "proposal"]
assignees: []
---

## Source

**System name:** <e.g. Todoist, Google Calendar, MacroFactor, Apple Health, YNAB>
**Category:** [ ] LMS  [ ] Calendar  [ ] Tasks  [ ] Health  [ ] Finance  [ ] Other: ___
**Upstream docs:** <link to the public API docs>
**Pricing / access:** [ ] free  [ ] paid  [ ] requires-approval (explain)

## Why this connector

<What rules does this unlock? Who benefits? Be specific — "sleep debt → strict mode tomorrow" beats "health data is useful".>

## Events it would produce

| Event type | Shape (sketch) | Fires when |
|-----------|----------------|-----------|
| `...` | `{ ... }` | ... |
| `...` | `{ ... }` | ... |

## Auth model

- [ ] OAuth 2.0 (authorization code + PKCE)
- [ ] OAuth 2.0 (client credentials)
- [ ] API token
- [ ] Session cookie / reverse-engineered
- [ ] Other: ___

Required scopes: `...`

## Rate limits & quotas

<Document the upstream rate limit. How often do we need to poll? Is there a webhook / push mechanism?>

## Data sensitivity

- [ ] Non-sensitive (public metadata)
- [ ] Personally identifiable
- [ ] Health / financial (regulated — HIPAA / PCI / GDPR special category)

## Verification tier

Proposed verification tier (see [ecosystem/verification-tiers](../../docs-site/ecosystem/verification-tiers.md)):
- [ ] Community
- [ ] Verified
- [ ] Phenotype-verified

## Open questions

1. ...
2. ...

## I can help

- [ ] I will implement it
- [ ] I want to help test / use it
- [ ] Proposing only
