# CLAUDE.md — FocalPoint

## Project

Connector-first screen-time platform. Rust core + native iOS/Android.
Status: v0.0.1 scaffold (2026-04-22). See `README.md`.

## AgilePlus Mandate

All work MUST be tracked in AgilePlus:
```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus specify --title "FocalPoint: <feature>" --description "..."
```

## Scaffold Rules

- **No impls without a spec.** The scaffold is intentional stubs. Don't ship logic without corresponding FR + work package.
- **Trait surfaces are stable.** The `Connector`, `EventStore`, `RuleStore`, `WalletStore`, `PenaltyStore`, `ClockPort`, `SecureSecretStore` traits define the public contract. Break them only via ADR.
- **Audit append on mutation.** Any reward/penalty/policy state change must produce an `AuditRecord`. The chain is tamper-evident; verify on every start.
- **Local-first.** SQLite is the source of truth. Services (`services/*`) are optional.

## Cross-platform Core

- Rust core in `crates/` is the shared brain.
- iOS consumes via UniFFI (Swift bindings from `crates/focus-ffi`).
- Android consumes via JNI (Kotlin bindings from `crates/focus-ffi`).
- **Never** put domain/rule/state logic in Swift or Kotlin. Only platform adapters (FamilyControls, UsageStats, Accessibility, CoreNFC, Keychain).

## Scripting

Per Phenotype scripting policy: Rust default; no new shell; Python/TS only when embedded in an existing runtime. `scripts/` contains glue only (≤5-line wrappers with top-of-file justification).

## Build

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

## Entitlements / Permissions (BLOCKERS)

- **iOS:** `com.apple.developer.family-controls` application — submit Phase 0. Apple review ~1–4 weeks. Blocks any actual iOS enforcement test.
- **Android:** `PACKAGE_USAGE_STATS` + `BIND_ACCESSIBILITY_SERVICE` — user-grant; onboarding flow required.

## Open Questions

Tracked in `docs/research/open_questions.md`. Q1 (name), Q5 (Foqos/Reef URLs), and Q8 (entitlement app) block meaningful impl; others can be deferred.

## Governance

Inherits Phenotype-org policies from `~/.claude/CLAUDE.md`:
- Scripting hierarchy (Rust default)
- CI completeness policy (though GH Actions billing blocks CI runs)
- Worktree discipline (feature work in `repos/FocalPoint-wtrees/<topic>/`)
- No commits without explicit request
