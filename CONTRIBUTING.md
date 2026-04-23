# Contributing to FocalPoint

FocalPoint is an agent-friendly, connector-first screen-time platform. We welcome contributions, especially new connectors, rule templates, and rigorous bug reports.

## Before you start

1. Read [`CHARTER.md`](./CHARTER.md) for scope and principles.
2. Read [`ADR.md`](./ADR.md) and [`FUNCTIONAL_REQUIREMENTS.md`](./FUNCTIONAL_REQUIREMENTS.md) for the technical contract.
3. Search [open issues](https://github.com/KooshaPari/FocalPoint/issues) to avoid duplicate work.
4. For anything non-trivial, open a proposal issue first (see templates).

All work is tracked in AgilePlus:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus specify --title "FocalPoint: <feature>" --description "..."
```

## Ways to contribute

- **Connectors.** Implement the `Connector` trait for a new external system (Todoist, YNAB, MacroFactor, Google Calendar, Apple Health, etc.). See [`docs-site/connector-sdk/`](./docs-site/connector-sdk/).
- **Rule templates.** Author a sample rule pack for a common use case. See [`docs-site/rules/`](./docs-site/rules/).
- **Bug reports.** Reproducible, with rule DSL + connector state snapshot.
- **Docs.** Improve the docs-site. Every page is edit-this-page linked.
- **Mascot personality.** Coachy copy, lines, states. See [`docs-site/mascot/`](./docs-site/mascot/).

## Development setup

### Prerequisites

- **Rust** 1.82+ via `rustup`.
- **Xcode** 15+ (iOS builds).
- **Bun** 1.1+ (docs-site). Install: `brew install oven-sh/bun/bun` or `curl -fsSL https://bun.sh/install | bash`.
- **lefthook** (git hooks). Install: `brew install lefthook`, then `lefthook install`.
- **Task** (task runner). Install: `brew install go-task/tap/go-task`.
- **trufflehog** (secret scan). Install: `brew install trufflesecurity/trufflehog/trufflehog`.
- **vale** + **markdownlint-cli2** (optional, docs lint). Install: `brew install vale && bun add -g markdownlint-cli2`.

### Clone and build

```bash
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint
cargo check --workspace
cargo test --workspace
```

### Verify locally (mirrors CI)

```bash
task verify
```

This runs `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`, and (if Xcode is on PATH) a Swift build of the iOS app.

### Docs-site

```bash
task docs-dev     # local dev server on http://localhost:5173
task docs-build   # static build into docs-site/.vitepress/dist
```

## Adding a connector

Each connector is a crate named `connector-<source>`. Minimum viable connector:

1. Implement the `Connector` trait from `focus-connectors`.
2. Emit `Event`s into the event store.
3. Supply a `manifest.json` with metadata, scopes, OAuth config, and event schema.
4. Provide `examples/` fixtures for offline testing.
5. Add a page under `docs-site/connectors/<source>.md` covering auth flow, rate limits, event shape, and known gotchas.
6. Mark the verification tier (community / verified / phenotype-verified) in the manifest. See [`docs-site/ecosystem/verification-tiers.md`](./docs-site/ecosystem/verification-tiers.md).

Reference implementation: [`crates/connector-canvas`](./crates/connector-canvas). Full SDK spec: [`docs-site/connector-sdk/spec.md`](./docs-site/connector-sdk/spec.md).

## Adding a rule template

Rule templates are authored in the rule DSL (see [`docs-site/rules/dsl.md`](./docs-site/rules/dsl.md)) and shipped as TOML under `examples/rules/`. Each template needs:

- A title, description, and "why this helps" narrative.
- Required connectors listed explicitly.
- Example event stream that fires the rule.
- Expected reward/penalty outcome.
- Tests in `focus-rules/tests/` against the fixture stream.

## Code style

- **Rust:** `cargo fmt` with workspace `rustfmt.toml`. Clippy is strict (`-D warnings`). No `.unwrap()` in library code — use `expect("invariant: ...")` with a reason or propagate with `?`.
- **Swift:** follow the `.swift-format` config in `apps/ios/` (added in Phase 1).
- **Markdown:** `markdownlint-cli2` + Vale's `proselint` / `write-good` styles.
- **Commits:** conventional commits. Examples:
  - `feat(connector-canvas): handle 429 with Retry-After backoff`
  - `fix(focus-rules): cooldown timer off-by-one at DST boundary`
  - `docs(rules): add sleep-debt sample rule`
  - `chore(ci): pin bun to 1.1.x in docs.yml`

## Pull requests

Use the [PR template](./.github/pull_request_template.md). Every PR must:

- Reference one or more FRs (`Traces to: FR-CONN-003, FR-RULE-012`).
- Reference an AgilePlus work package or spec (`AgilePlus: <feature-id>/<wp-id>`).
- Pass `task verify` locally before opening.
- Include tests for new behavior. New public API surface requires both unit tests and an integration test.
- Keep the audit chain invariant if you touch `focus-audit`, `focus-storage`, or any mutation path: chain verifies from genesis; tampering is detected.

**Note on CI:** GitHub Actions billing is constrained (see [`~/.claude/CLAUDE.md`](https://github.com/KooshaPari/FocalPoint) GH Actions billing section). PRs should pass local `task verify` regardless of CI status. Reviewers merge on green-local + human review; CI is advisory.

## Security

Vulnerability reports: see [`SECURITY.md`](./SECURITY.md). Do not open public issues for security problems.

## Conduct

See [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md). TL;DR: be kind, be specific, be useful.

## License

By contributing, you agree that your contributions will be dual-licensed under MIT OR Apache-2.0.
