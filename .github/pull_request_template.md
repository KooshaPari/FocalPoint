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
- [ ] Docs-site page added / updated (for user-visible changes)
- [ ] FR references included in test bodies (`// Traces to: FR-...`)
- [ ] No `.unwrap()` introduced in library code
- [ ] No new `.sh` scripts (Rust/Go/Zig default — see Phenotype scripting policy)
- [ ] No secrets committed (pre-commit hook + trufflehog clean)
- [ ] **Commits are signed** (GPG/SSH; GitHub shows "Verified" badge)
- [ ] **Commits include DCO sign-off** (`Signed-off-by: Your Name <email>` trailer)
- [ ] **Conventional-commit title** (e.g., `feat(scope):`, `fix(scope):`)
- [ ] CHANGELOG entry (if user-visible)

## Notes for reviewers

<Anything tricky, known-unresolved, or deliberately out of scope for this PR.>
