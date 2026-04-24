---
title: Ecosystem & Marketplace
description: Discover and publish rule templates, connectors, and coaching templates.
---

# FocalPoint Ecosystem

The FocalPoint ecosystem is a marketplace for **rule templates**, **connectors**, and **coaching coaches**—created by the community or by FocalPoint core.

## Marketplace Categories

### Rule Template Packs

Pre-built rule collections for common use cases:

- **Student Focus Pack** — Block distractions during class time & assignments
- **Developer Deep Work Pack** — GitHub-triggered focus sessions, PR review blocks
- **Sleep Wellness Pack** — Evening lockdowns, sleep debt coaching
- **Parent's Accountability Pack** — Family sharing with healthy boundaries

See [Rule Template Format](./rule-template-format) for spec details.

### Connectors

Integrations with external platforms:

- **Shipping**: Canvas LMS
- **Verified**: (Community submissions reviewed & signed)
- **Community**: (Published as-is; user assumes risk)

See [Connector SDK](../connector-sdk/) to build your own.

### Coaching Modules

Advanced coaching strategies:

- **Streak gamification** — Visual achievement system, badges
- **Pomodoro integration** — Auto-triggered focus sessions with timers
- **Motivation loops** — Personalized encouragement based on mood & energy
- **Accountability partners** — Share progress summaries (optional)

## Publishing

### For Rule Pack Authors

```bash
focalpoint rule pack publish my-pack.yaml   --name "My Pack"   --description "..."   --version 1.0.0
```

### For Connector Authors

```bash
focalpoint connector publish connector.toml   --manifest-url "..."   --wasm-url "..."
```

### Verification Tiers

| Tier | Criteria | Installation |
|------|----------|--------------|
| **Trusted** | Created by FocalPoint core, pinned in app UI | One tap |
| **Verified** | Community submission that passed security & functional audits | App marketplace |
| **Community** | Published without review; user accepts risk | Manual URL import |

See [Verification Tiers](./verification-tiers) for full audit criteria.

## Support & Feedback

- **Discuss**: [GitHub Discussions](https://github.com/KooshaPari/FocalPoint/discussions)
- **Report issues**: [GitHub Issues](https://github.com/KooshaPari/FocalPoint/issues)
- **Showcase your work**: Tweet with `#FocalPointEcosystem`
