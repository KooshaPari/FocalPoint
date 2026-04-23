# Ecosystem Strategy

> Source: arch doc lines 1293–1836. Condensed; see source for full prose.

## Why ecosystem > blocker

Blocking is commoditized (Apple Screen Time, Foqos, Reef, Opal all ship it).
Winning is about **context**: what you use to decide what to block.

Every connector is a new context source, which unlocks new rule templates,
which unlocks new user segments. Network effects accrue to the platform with
the most connectors, not the best blocker.

## Connector classes (4)

1. **First-party first-party** — we ship (Canvas, Apple Health, Google Calendar, Notion, Todoist).
2. **First-party community-reviewed** — community authors; we review before marketplace publish.
3. **Self-hosted** — power user runs a connector on their own machine/server.
4. **MCP-bridged** — generic MCP servers as connectors via `focus-connectors::Connector` adapter (future).

## Three-layer ecosystem

1. **Connectors** — data sources (this SDK)
2. **Rule templates** — pre-built rule bundles ("pomodoro", "thesis mode", "doomscroll detox")
3. **Skins / surfaces** — UI personality layers (mascot, color schemes, explanation styles)

Launch rule templates first (easier for community); connectors second (higher bar); skins last.

## Verification tiers

- **Official** — first-party; Apple-signed; in-app store
- **Verified** — community-authored; we signed; in-app store
- **Side-loaded** — self-hosted URL; user pastes manifest; big scary warning

## Packaging

- First-party connectors: compiled into the app (Cargo dep)
- Verified connectors: WASM modules? (TBD) loaded at runtime
- Side-loaded: HTTP manifest + polling via local proxy (runs as a service on user's machine)

## Monetization (draft)

- Free tier: 1 connector, 3 rules, core enforcement
- Pro: unlimited connectors, rule templates, mascot
- Team (future): shared rulesets for parents/spouses/study groups

## Community growth loops

- Rule templates are shareable links → TikTok-able
- Explanation screens show attribution ("powered by your Canvas + Sleep connectors") → implicit marketing
- Creator program for popular rule-template authors → revenue share

## Governance

- Connector review criteria (security, privacy, dedupe correctness)
- Kill-switch for abusive connectors
- Disclosure requirements for third-party data sharing

All of the above is Phase 3+. Don't build ecosystem machinery before Phase 1
core ships.
