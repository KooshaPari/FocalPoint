---
layout: home

hero:
  name: FocalPoint
  text: Coachy has your back.
  tagline: Connector-first screen-time management. External signals (Canvas, calendars, health, tasks, finance) drive a rules engine that blocks, rewards, and coaches — explainably.
  actions:
    - theme: brand
      text: Get started on iOS
      link: /getting-started/install-ios
    - theme: alt
      text: Write your first rule
      link: /getting-started/first-rule
    - theme: alt
      text: Browse connectors
      link: /connectors/
    - theme: alt
      text: Architecture
      link: /architecture/

features:
  - title: Connector-first
    details: Canvas assignments, calendar events, sleep debt, todo completion, grocery budget — all first-class inputs to your focus rules. One ecosystem, not a drawer of disconnected blockers.

  - title: Explainable blocks
    details: Every lock shows you which rule fired, which event triggered it, and when it lifts. No "the algorithm decided" — the decision trace is a tap away.

  - title: Dual ledger
    details: Rewards AND penalties. Streaks earn bypass budget. Skipped sleep tightens tomorrow's block windows. Escalation tiers for chronic violations, never punitive by default.

  - title: Tamper-evident audit
    details: Every state change appends to a SHA-256 hash-chained audit log. Verified from genesis on every launch. A broken chain fails loudly.

  - title: Platform-native enforcement
    details: iOS FamilyControls + ManagedSettings + DeviceActivity. No cross-platform UI bridges. Android via UsageStats + Accessibility (Phase 2+).

  - title: Coachy, not Clippy
    details: Your mascot is warm, terse, and specific. Never nags, never shames. Reacts to real state — assignment due, streak earned, token expired — with lines that feel like a friend.
---

## What problem this solves

Existing screen-time tools fall into two camps:

1. **Pure blockers** (Opal, Freedom, iOS Screen Time's default) — brittle, no context, ignore the *why*. You block Instagram during "work hours", Instagram wins at 2 pm when the assignment isn't due.
2. **Gamified habit trackers** (Forest, Streaks) — reward without teeth. Pretty trees, no behavioral change after week three.

FocalPoint is a **rules platform**. External systems emit events. Rules combine events, state, and schedules into decisions. Decisions produce blocks, rewards, penalties — every one of them explainable and tied to a connector signal you authorized.

## Architecture at a glance

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

## Quick start

```bash
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint
task verify
task docs-dev
```

For iOS, see [Install on iOS](/getting-started/install-ios). You will need an Apple Developer account with the `com.apple.developer.family-controls` entitlement approved.

## Roadmap snapshot

| Phase | Status |
|------|-------|
| P0 Scaffold | complete |
| P1 Core crates + UniFFI | in progress |
| P2 Canvas connector + first rule on device | planned |
| P3 Rewards / penalties ledger | planned |
| P4 Connector SDK + marketplace | planned |
| P5 Android | deferred |
| P6 Multi-device sync | deferred |

Full roadmap: [`PLAN.md`](https://github.com/KooshaPari/FocalPoint/blob/main/PLAN.md).

## Acknowledgments

- **Foqos** — donor codebase for the FamilyControls / ManagedSettings harness pattern.
- **MiniMax M2.7** — cheap-LLM routing for docs summarization and rule-template extraction.
- **Kimi K2.5** — secondary routing model for long-context rule DSL generation.
- **Phenotype org shared crates** — `phenotype-event-sourcing`, `phenotype-cache-adapter` patterns echoed in `focus-events` and connector caches.

## License

MIT OR Apache-2.0 at your option. See [LICENSE](https://github.com/KooshaPari/FocalPoint/blob/main/LICENSE).
