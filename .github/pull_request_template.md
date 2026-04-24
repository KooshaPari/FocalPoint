<!--
Title: use conventional commits. Examples:
  feat(connector-canvas): back off on 429 with Retry-After
  fix(focus-rules): cooldown off-by-one at DST boundary
  docs(rules): add sleep-debt sample rule
  chore(ci): pin bun to 1.1.x
-->

## Summary

<One paragraph: what this PR does and why.>

## Traceability

- **FRs:** `FR-...`, `FR-...`
- **AgilePlus:** `feature-id` / `wp-id`
- **Issues / ADRs:** Closes #__, ref ADR-__

## Changes

- ...
- ...
- ...

## Connector / rule impact (if applicable)

- Connector(s) touched: `connector-...`
- Rules that change behavior: `...`
- Event schema changes: [ ] none  [ ] additive  [ ] breaking (explain)
- Audit chain invariants preserved: [ ] yes  [ ] n/a

## User-facing changes

If this PR introduces user-visible changes (new features, breaking changes, behavior changes):

- [ ] **CLI:** Command signature / output format change
- [ ] **Rule DSL:** New condition, action, or syntax
- [ ] **Event schema:** New event type or field
- [ ] **Connector manifest:** New scopes, OAuth flow, or metadata
- [ ] **iOS/Android enforcement:** New behavior or permission requirements
- [ ] CHANGELOG.md updated with user-facing summary
- [ ] Docs-site page added / updated

## Dual-surface coverage (if new primitive added)

If this PR adds a new rule condition, action, connector, or other primitive:

- [ ] **Rust core:** Implemented in `crates/focus-*` or `crates/connector-*`
- [ ] **Rule DSL:** Parser + evaluator handle the primitive
- [ ] **CLI:** Command / output displays the primitive correctly
- [ ] **iOS/Android UI:** Displays or interacts with the primitive (if applicable)
- [ ] **Builder (web UI):** If applicable, builder can compose rules with the primitive
- [ ] **IR (intermediate representation):** Primitive is serialized / deserialized correctly
- [ ] Tests verify primitive across all surfaces

## How to verify

```bash
task verify
```

Plus any manual checks:

1. ...
2. ...

## Checklist

- [ ] `task verify` passes locally (fmt + clippy -D warnings + test + iOS build if Xcode present)
- [ ] Unit tests added / updated for new behavior
- [ ] Integration tests added for new public API surface
- [ ] Tests reference FRs (`// Traces to: FR-...`)
- [ ] No `.unwrap()` in library code (use `expect("invariant: ...")` with reason or `?`)
- [ ] No new `.sh` scripts (Rust/Go/Zig default — see CLAUDE.md scripting policy)
- [ ] No secrets committed (pre-commit hook + trufflehog clean)
- [ ] **Commits are signed** (GPG/SSH; GitHub shows "Verified" badge)
- [ ] **Commits include DCO sign-off** (`Signed-off-by: Your Name <email>` trailer)
- [ ] **Conventional-commit title** (e.g., `feat(scope):`, `fix(scope):`)
- [ ] Audit chain invariants preserved (if touching state mutation)

## Notes for reviewers

<Anything tricky, known-unresolved, or deliberately out of scope for this PR.>
