---
title: FocalPoint
description: Connector-first screen-time management. Rules engine + dual ledger + iOS enforcement.
layout: home

hero:
  name: FocalPoint
  text: Coachy has your back.
  tagline: Context-aware screen-time management. External signals (Canvas, calendars, health, tasks, finance) drive a rules engine that blocks, rewards, and coaches—with full transparency.
  actions:
    - theme: brand
      text: Get started on iOS
      link: /getting-started/install-ios
    - theme: alt
      text: Quick Start
      link: /getting-started/
    - theme: alt
      text: Plugin SDK
      link: /connector-sdk/
    - theme: alt
      text: Architecture
      link: /architecture/

features:
  - title: Rules, not timers
    details: Canvas assignments, calendar events, sleep debt, todo completion, grocery budget — all first-class inputs to your focus rules. One ecosystem, not a drawer of disconnected blockers.

  - title: Explainable decisions
    details: Every lock shows which rule fired, which event triggered it, and when it lifts. No "the algorithm decided"—the decision trace is a tap away.

  - title: Dual ledger
    details: Rewards AND penalties. Streaks earn bypass budget. Skipped sleep tightens tomorrow's block windows. Escalation tiers for chronic violations, never punitive by default.

  - title: Tamper-evident audit
    details: Every state change appends to a SHA-256 hash-chained audit log. Verified from genesis on every launch. A broken chain fails loudly.

  - title: Platform-native iOS enforcement
    details: FamilyControls + ManagedSettings + DeviceActivity. No cross-platform UI bridges. Android via UsageStats + Accessibility (Phase 2+).

  - title: Coachy, your coach
    details: Warm, terse, specific. Never nags, never shames. Reacts to real state—assignment due, streak earned, token expired—with lines that feel like a friend.
---

## Problem

Existing screen-time tools fall into two camps:

- **Pure blockers** (Opal, Freedom) — brittle, no context. You block Instagram during "work hours," but Instagram wins at 2 pm when the assignment isn't due.
- **Gamified trackers** (Forest, Streaks) — reward without teeth. Pretty trees, no behavioral change.

FocalPoint is a **rules platform**. External systems emit events. Rules combine events, state, and schedules into decisions. Decisions produce blocks, rewards, penalties—every one explainable and tied to a connector signal you authorized.

## For users

Start here if you're looking to manage your screen time with rules tied to your calendar, assignments, and health.

- **[Quick Start](/getting-started/)** — Install, connect Canvas/Google Calendar/GitHub, write your first rule
- **[User Guides](/guides/)** — Focus modes, rewards, backup, feedback
- **[Journeys](/journeys/)** — Real workflows: student on Canvas, developer with GitHub, sleep wellness

## For developers

Build connectors, rule templates, and theme packs. Extend FocalPoint for your ecosystem.

- **[Plugin SDK](/connector-sdk/)** — Manifest format, event schema, auth flows, testing, verification
- **[Connector Framework](/architecture/connector-framework)** — How connectors integrate
- **[Rule DSL Reference](/rules/dsl)** — Write conditional logic in FPL

## Status

**v0.0.4** — Pre-release. TestFlight pending Apple entitlement review.

| Phase | Status |
|------|-------|
| P0 Scaffold | ✅ complete |
| P1 Core + UniFFI | 🔄 in progress |
| P2 Canvas + first rule on device | ⏸ blocked on entitlement |
| P3 Rewards/penalties ledger | 📋 planned |
| P4 Connector SDK + marketplace | 📋 planned |
| P5 Android | 📋 deferred |

Full roadmap: [PLAN.md](https://github.com/KooshaPari/FocalPoint/blob/main/PLAN.md)

## Architecture

```mermaid
flowchart LR
  subgraph External["External systems"]
    Canvas[Canvas LMS]
    GCal[Google Calendar]
    Health[Apple Health]
    YNAB[YNAB]
    Todoist[Todoist]
  end

  subgraph Core["Rust core (crates/)"]
    Connectors[focus-connectors<br/>runtime]
    Events[focus-events<br/>store]
    Rules[focus-rules<br/>engine]
    Ledger[focus-rewards<br/>focus-penalties]
    Audit[focus-audit<br/>hash chain]
    Policy[focus-policy<br/>decisions]
  end

  subgraph iOS["iOS app"]
    Swift[SwiftUI + UniFFI]
    FC[FamilyControls /<br/>ManagedSettings /<br/>DeviceActivity]
    Mascot[Coachy mascot<br/>Spline runtime]
  end

  Canvas --> Connectors
  GCal --> Connectors
  Health --> Connectors
  YNAB --> Connectors
  Todoist --> Connectors

  Connectors --> Events
  Events --> Rules
  Rules --> Policy
  Policy --> Ledger
  Policy --> Audit
  Policy --> Swift
  Swift --> FC
  Swift --> Mascot
```

## Community

- **GitHub**: [KooshaPari/FocalPoint](https://github.com/KooshaPari/FocalPoint)
- **Discord**: <DISCORD_URL>
- **Email**: kooshapari@gmail.com

## License

MIT OR Apache-2.0. See [LICENSE](https://github.com/KooshaPari/FocalPoint/blob/main/LICENSE).
