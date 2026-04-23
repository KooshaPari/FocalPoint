# AGENTS.md — FocalPoint

Instructions for AI agents working on FocalPoint.

## Scope boundaries

- **Don't cross the FFI line in a single PR.** Rust core changes and
  iOS/Android adapter changes ship separately unless the public trait surface
  changes intentionally.
- **Don't add business logic in Swift/Kotlin.** Only platform adapters.

## Reading order for a cold agent

1. `README.md` → `00_START_HERE.md` → `PRD.md`
2. `ADR.md` + `docs/adr/*` for locked decisions
3. `FUNCTIONAL_REQUIREMENTS.md` — FR-* IDs are canonical
4. `docs/research/open_questions.md` — know what's still TBD before touching it
5. `PLAN.md` — phase ordering

## Workflow

1. Pick a crate or WP with a stub/placeholder.
2. Check `docs/research/open_questions.md` — is anything upstream still TBD?
3. Write failing test first (FR-trace in doc comment: `// Traces to: FR-XXX-NNN`).
4. Impl.
5. `cargo test --workspace && cargo clippy --workspace -- -D warnings && cargo fmt --check`.
6. Update CHANGELOG.
7. No commit without user approval.

## Test traceability

Every test MUST reference a Functional Requirement:
```rust
/// Traces to: FR-RULE-002
#[test]
fn rule_eval_is_deterministic() { ... }
```

## Connector authoring

- Implement `focus_connectors::Connector` trait.
- Ship manifest + auth strategy + sync implementation.
- Use `connector-testkit` for fixture-replay tests.
- Canvas (`crates/connector-canvas`) is the reference.

## Platform specifics

**iOS:** FamilyControls entitlement is gated. Don't attempt real enforcement
testing before entitlement is approved. Use simulator-safe code paths behind
`#if !targetEnvironment(simulator)`.

**Android:** AccessibilityService requires user consent + detailed rationale in
onboarding. Don't deep-link to settings without explanation.

## When to ask the human

- Anything touching **Q1 project name** before rename
- Anything assuming **Foqos/Reef** code is available for copy
- Anything crossing the **trait boundary** (breaking changes)
- Deploying / submitting to App Store or Play Store
