# FocalPoint — Product Requirements

> Source: `ChatGPT-App Architecture for Screen Time.md` (lines 17–245)

## Vision

Connector-first screen-time management. Not another blocker — a **behavioral
rules platform** that treats external systems (LMS, calendars, tasks, health
apps) as first-class inputs to focus policy. Ecosystem of connectors is the
compounding moat.

## Primary value prop

- Blocking that **understands context** (you finished your Canvas assignment → unlock; you skipped sleep → tighter block windows)
- **Explainable** decisions (why was I blocked? which rule fired? what state triggered it?)
- Reward + penalty dual-ledger for sustained behavior change (not just willpower)
- **Platform-native** enforcement (iOS Screen Time / Android UsageStats+Accessibility) over bridged UI

## Primary personas

- **Self-regulators** — adults using connectors to enforce own rules
- **Students** — Canvas/Notion/Todoist-driven focus sessions
- **Parents (v2)** — tier for child devices with ruleset sharing

## Use cases (see USER_JOURNEYS.md for walkthroughs)

1. **Assignment-driven focus** — Canvas assignment due → lock social apps until submitted
2. **Sleep debt penalty** — health app reports <6h → strict-mode tomorrow morning
3. **Streak reward** — 7-day study streak → bypass budget restored + 2× multiplier on next credit
4. **Calendar window** — meeting event starts → lock distractor apps until event ends

## Non-goals (v1)

- Corporate/MDM deployment
- Browser extension (URL-level blocking) — Phase 4
- Multi-device sync — Phase 3+
- Parental controls for minors — Phase 5

## Principles

- Local-first, offline-capable. Sync is optional.
- Audit chain for every state mutation (tamper-evident).
- Explainability over automation magic.
- Connector contract is narrow and stable — community authoring comes later.
- Platform-native enforcement. No cross-platform UI bridge for blocking.

## Success metrics (draft — to be finalized per PLAN Phase 0)

- D30 retention > 40% (vs. Foqos-style blockers ~15%)
- ≥ 3 rules active per user by D7
- ≥ 1 connector authenticated by D3
- Penalty events evaluated < 1.5s p95 from event arrival
