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

## Code Style

- **Rust:** `cargo fmt` with workspace `rustfmt.toml`. Clippy is strict (`-D warnings`). No `.unwrap()` in library code — use `expect("invariant: ...")` with a reason or propagate with `?`.
- **Swift:** follow the `.swift-format` config in `apps/ios/`.
- **Markdown:** `markdownlint-cli2` + Vale's `proselint` / `write-good` styles.

## Commits

All commits must:

1. **Use conventional commits.** Format: `<type>(<scope>): <description>`.
   - Examples:
     - `feat(connector-canvas): handle 429 with Retry-After backoff`
     - `fix(focus-rules): cooldown timer off-by-one at DST boundary`
     - `docs(rules): add sleep-debt sample rule`
     - `chore(ci): pin bun to 1.1.x in docs.yml`
   - Types: `feat`, `fix`, `docs`, `chore`, `test`, `refactor`, `perf`, `ci`.

2. **Be signed.** Commits must be signed with GPG or SSH:
   ```bash
   git config user.signingkey <your-key-id>
   git config commit.gpgSign true
   git commit -m "message"
   ```
   GitHub will show a "Verified" badge on signed commits.

3. **Include DCO sign-off.** Every commit must include a Developer Certificate of Origin (DCO) sign-off:
   ```bash
   git commit -m "feat: add cool feature

   This implements the cool feature described in RFC-0123.

   Signed-off-by: Your Name <your.email@example.com>"
   ```
   Or use the `-s` flag:
   ```bash
   git commit -s -m "feat: add cool feature"
   ```

## DCO Sign-Off

The DCO certifies that:
- You authored the code (or obtained it under a compatible license).
- You have the legal right to contribute it under FocalPoint's MIT OR Apache-2.0 license.
- You understand the implications of your contribution.

It is **not** a Contributor License Agreement (CLA); it is a lightweight legal commitment to honesty.

For local setup and validation, see [`docs/governance/dco_setup.md`](./docs/governance/dco_setup.md). You can optionally install a local validator that runs before each commit to catch DCO violations early.

## Pull Requests

Use the [PR template](./.github/pull_request_template.md). Every PR must:

- **Reference an issue or RFC.** Open a GitHub issue or RFC first. Link it in the PR description.
- **Reference FRs.** Link one or more Functional Requirements (`Traces to: FR-CONN-003, FR-RULE-012`).
- **Trace to AgilePlus.** If this is tracked in AgilePlus, reference the work package (`AgilePlus: <feature-id>/<wp-id>`).
- **Pass local verification.** Run `task verify` before opening the PR. It must pass.
- **Include tests.** New behavior requires unit tests. New public API surfaces require both unit tests and an integration test.
- **Preserve audit integrity.** If you touch `focus-audit`, `focus-storage`, or any state-mutation path: the audit chain must remain tamper-evident and verify from genesis.
- **Use conventional commits.** Commits must follow the format above; each commit is squashed or rebased into a single conventional commit.
- **Be signed and DCO'd.** All commits must be signed (GPG/SSH) and include `Signed-off-by` trailers.

### Template Checklist

The PR template includes a checklist. Ensure all items are marked:

- [ ] Tests pass locally (`task verify`).
- [ ] Clippy and fmt pass.
- [ ] CHANGELOG.md updated (if user-facing change).
- [ ] Docs updated (if public API changed).
- [ ] Commits are signed and DCO'd.
- [ ] Dual-surface parity checked (Rust core matches Swift/Kotlin intent, if applicable).

### Example PR

```markdown
## Description

Implements RFC-0023 (improved rule DSL error messages) to address issue #456.

Traces to: FR-RULE-008 (better error feedback)
AgilePlus: feature-xyz/wp-03

## Changes

- Added `DiagnosticBuilder` in `focus-lang` for rich error messages.
- Tests cover 8 new error patterns.
- CHANGELOG updated.

## Verification

- [x] `task verify` passes (8m 42s).
- [x] Commits signed + DCO'd.
- [x] Docs updated in `docs-site/dsl/errors.md`.
```

### CI Note

GitHub Actions billing is constrained. PRs should pass local `task verify` regardless of CI status. Reviewers merge on green-local + human review; CI is advisory.

## Security

Vulnerability reports: see [`SECURITY.md`](./SECURITY.md). Do not open public issues for security problems.

## Conduct

See [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md). TL;DR: be kind, be specific, be useful.

## License

By contributing, you agree that your contributions will be dual-licensed under MIT OR Apache-2.0.
