# FocalPoint — Project Charter

## Mission

Turn screen-time management into a **behavioral rules platform**. The blocker is not the product — the product is an ecosystem of connectors (Canvas, calendars, tasks, health apps, financial trackers) that feed a rules engine which decides when and how to intervene. Your phone gets out of your way when you're doing the work; pushes back when you're not.

## Principles

1. **Connectors are the moat.** Every external signal (assignment due, meeting started, sleep debt, grocery budget blown) is a first-class input. The ecosystem compounds; each connector added makes every rule template more useful.
2. **Explainability over automation magic.** Every block, reward, or penalty cites the rule, the event, and the state that triggered it. No "the algorithm decided" black boxes.
3. **Local-first, tamper-evident.** SQLite is the source of truth. Every mutation appends to a hash-chained audit log. The chain is verified on startup; a broken chain fails loudly.
4. **Platform-native enforcement.** iOS FamilyControls / ManagedSettings and Android UsageStats / Accessibility are the actuators. No cross-platform UI bridges for blocking.
5. **Portable Rust core.** Domain logic, rules, ledger, and audit live in Rust. Swift and Kotlin are adapters. No business logic in platform code.
6. **Coachy, not Clippy.** The mascot is warm, terse, and never condescending. It coaches — it doesn't nag, shame, or gamify into oblivion.
7. **Rewards AND penalties.** Pure-reward apps drift into gamification slop; pure-penalty apps get uninstalled within a week. FocalPoint runs a dual ledger with escalation tiers, streaks, and a bypass budget.
8. **Fail loudly.** No silent fallbacks. If the Canvas token expired, the user sees "Canvas reauth needed" — not a rule that quietly stopped firing.
9. **Wrap, don't hand-roll.** OAuth2 via `oauth2` crate, storage via `rusqlite`, crypto via `ring`, bindings via `uniffi`. Reuse `phenotype-event-sourcing`, `phenotype-cache-adapter` where they fit.

## Scope

### In (v1)

- iOS enforcement (FamilyControls + ManagedSettings + DeviceActivity).
- Rules engine with explainable decisions, cooldowns, and state snapshots.
- Reward + penalty dual-ledger with escalation tiers.
- Connector runtime (Canvas LMS is the reference connector).
- Rule DSL + sample rule packs (assignment-driven focus, sleep penalty, calendar lock).
- Coachy mascot (Spline-rendered, state-machine-driven).
- Local SQLite persistence, hash-chained audit log.
- UniFFI Swift bindings from the Rust core.

### Out (deferred)

- Android (Kotlin via JNI) — Phase 2+.
- Desktop admin / Web admin — Phase 3+.
- Multi-device sync — Phase 3+.
- Parental controls for minors — Phase 5.
- Corporate / MDM deployment — not planned.
- Browser extension URL-level blocking — Phase 4.

## Stakeholders

- **Primary user:** self-regulating adult, student, or knowledge worker with fragmented focus surfaces.
- **Solo maintainer:** @kooshapari.
- **Phenotype org cross-project reuse:** connector runtime and rule engine are candidates for promotion to shared crates once the SDK stabilizes.
- **Future contributors:** MIT OR Apache-2.0 from day one; connector marketplace planned with verification tiers.

## Success criteria (v1)

1. Canvas connector authenticates, ingests assignments, drives a real rule on device without crashing for 30 consecutive days.
2. Assignment-due → lock social apps → submission → unlock cycle completes in <1.5 s p95 from event arrival.
3. Audit chain verifies from genesis on every launch; tampering is detected and reported.
4. At least 3 rule templates ship with the app; at least 1 community-authored connector proposal lands in GitHub before the marketplace opens.
5. D30 retention > 40 % in private beta (vs. pure-blocker baseline ~15 %).

## Non-goals / anti-patterns

- No Electron, no React Native, no Tauri for the enforcement app.
- No gamification-for-its-own-sake (XP bars that don't tie back to a rule are forbidden).
- No silent connector failures.
- No cloud dependency for core enforcement loop.
- No domain/rule/state logic in Swift or Kotlin.
- No bypass of workspace quality gates (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`).
