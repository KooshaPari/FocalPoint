# AGENTS.md — FocalPoint

This file governs work inside the FocalPoint repository.

## Identity

FocalPoint is a **connector-first screen-time management platform**. It combines a portable Rust core (rules engine, connector runtime, reward/penalty ledger, audit chain, mascot state machine) with a native iOS shell (SwiftUI + FamilyControls enforcement). The platform ingests behavioral signals from productivity, education, and health platforms to make screen-time enforcement context-aware.

## Quick Links

- **Local CLAUDE.md:** Present (`./CLAUDE.md`); this AGENTS.md is the source of truth for cross-cutting rules. CLAUDE.md is the Claude-specific entry point mirroring the stack template.
- **Phenotype org governance:** `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md` (consult when touching cross-repo contracts).
- **Global agent guidance:** `~/.claude/AGENTS.md` (consult for global defaults).

## Tooling Mandate (Strict — No Exceptions)

**You MUST use the Phenotype-developed tooling ecosystem for all non-trivial work.** Manual repetition, hand-rolled scripts, and ad-hoc processes are prohibited. These tools exist to eliminate repetition for both human operators and agents. Use them.

### Required Tooling

| Tool | Path | Purpose | When to Use |
|------|------|---------|-------------|
| **AgilePlus** | `/repos/AgilePlus/` | Spec-driven work tracking, spec harmonization, work packages | BEFORE any non-trivial work — intake specs, track work, link PRs |
| **phenodag** | `/repos/phenodag-tool/phenodag` | Multi-agent multi-project DAG coordination, atomic claims, duplicate detection | BEFORE picking work — check DAG status, claim repo/branch, avoid collisions |
| **agent-orchestrator** | `tooling/agent-orchestrator/` | Lane-based agent dispatch with non-overlapping scopes | When dispatching parallel agents across lanes |
| **quality-gate** | `tooling/quality-gate/` | Unified pre-review quality gate (build, test, lint, FR coverage, doc-link check) | BEFORE requesting review — mandatory gate |
| **fr-coverage** | `tooling/fr-coverage/` | Feature-requirement coverage tracker | BEFORE claiming work complete — verify FR traceability |
| **target-pruner** | `tooling/target-pruner/` | Disk reclaim by pruning stale cargo targets | Weekly or when disk < 30 GB |
| **disk-check** | `tooling/disk-check/` | Pre-dispatch disk space gate | BEFORE agent dispatch — abort if < 10 GB free |
| **release-cut** | `tooling/release-cut/` | End-to-end TestFlight release orchestration | When cutting a release |

### Mandatory Workflow

1. **Intake:** Use `agileplus specify` or `agileplus status` to create/link specs. All substantive work MUST have an AgilePlus spec or issue ID.
2. **Plan:** Check `phenodag status` and `phenodag pick --agent <id>` to claim atomic work from the fleet DAG. Never pick work ad-hoc without checking the DAG.
3. **Claim:** `phenodag claim --agent <id> --repo FocalPoint --branch <branch>` to lock the repo/branch. Prevents multi-agent collisions.
4. **Dispatch (multi-agent):** Use `agent-orchestrator lanes dispatch <lane-id>` for scoped parallel work. Validate non-overlapping scopes with `lanes list`.
5. **Gate:** Run `cargo run -p quality-gate` (or build first) BEFORE requesting review. This runs build, test, clippy, fmt, FR coverage, and doc-link check.
6. **Release:** Use `cargo run -p release-cut -- vX.Y.Z` for releases. Do not hand-roll version bumps, tags, or TestFlight uploads.
7. **Cleanup:** Run `target-pruner --prune` weekly or when disk is low. Run `disk-check` before any heavy dispatch.

### Prohibited (Do Not Repeat Yourself)

- **Do NOT** hand-write work plans or task lists in chat, Markdown, or temporary files when `agileplus` or `phenodag` exist for this purpose.
- **Do NOT** manually bump versions, create git tags, or upload to TestFlight when `release-cut` exists.
- **Do NOT** run individual `cargo test`, `cargo clippy`, `cargo fmt` checks separately when `quality-gate` unifies them.
- **Do NOT** dispatch multiple agents without using `agent-orchestrator` lane scoping or `phenodag` claims — collision risk is real and wastes work.
- **Do NOT** write one-off shell scripts for disk cleanup, coverage reports, or link checking when the tooling above already does it.
- **Do NOT** track FR coverage manually or in spreadsheets when `fr-coverage` generates the honest matrix.

## Stack

| Layer | Technology |
|-------|------------|
| Core | Rust (cargo workspace, 67 crates) |
| Mobile | Swift 5.9+ / SwiftUI (iOS 16+) |
| Backend | Go (services/, deferred to Phase 5) |
| DB | SQLite (local-first), PostgreSQL (optional), SurrealDB |
| FFI | UniFFI (Rust ↔ Swift), JNI stubs (Android future) |
| Config | deny.toml (cargo-deny), clippy.toml, rust-toolchain.toml |
| Testing | Rust tests, cargo test, cargo clippy |

## Working Conventions

- **Branch naming:** `<type>/<topic>` in kebab-case, conventional commits (`feat:`, `fix:`, `chore:`, `refactor:`). See `CONTRIBUTING.md`.
- **PR expectations:** Each PR links an AgilePlus spec or issue, passes `quality-gate` (build, test, clippy, fmt, FR coverage, doc-link check), and updates docs if behavior changed.
- **Quality gates:** `cargo run -p quality-gate` — all green before requesting review. Do not bypass with manual individual checks.
- **Security disclosures:** Follow `SECURITY.md`; never open public issues for security findings.
- **Traceability:** Substantive work links FR IDs or an ADR. RustDoc on public surfaces.
- **Agent dispatch:** For parallel work, use `agent-orchestrator lanes dispatch <lane-id>` with non-overlapping scopes. Validate scopes with `lanes list` first.
- **Disk discipline:** Run `disk-check` before dispatch. Run `target-pruner --prune` weekly. If disk < 10 GB, stop dispatch and clean up.

## Do / Don't

- **Do** keep changes focused; split unrelated work into separate PRs.
- **Do** prefer Rust for domain logic; SwiftUI only for iOS presentation layer.
- **Do** re-run the full workspace build before pushing if Cargo.toml or workspace deps changed.
- **Do** use `phenodag pick` and `phenodag claim` before starting work — atomic claims prevent duplicate effort.
- **Do** use `agileplus` for spec intake and work tracking — this is the canonical source of truth, not chat logs or temporary notes.
- **Do** use `release-cut` for all release operations — version bumps, tags, CHANGELOG, Discord, TestFlight.
- **Do** use `fr-coverage` to verify feature-requirement traceability before marking work complete.
- **Don't** add new lint suppressions without justification in the PR body.
- **Don't** introduce new top-level dependencies without first checking `deny.toml` and proposing the addition in the PR.
- **Don't** bypass the security policy in `SECURITY.md` for any reason.
- **Don't** hand-roll scripts or processes that existing tooling already covers. If a gap exists, extend the tooling, don't bypass it.
- **Don't** dispatch parallel agents without `agent-orchestrator` lane scoping or `phenodag` claims. Unscoped parallel dispatch causes collisions and lost work.

## Tooling Ecosystem Reference

| Command | Description |
|---------|-------------|
| `cd /repos/AgilePlus && agileplus specify --title "..." --description "..."` | Create a new spec |
| `cd /repos/AgilePlus && agileplus status` | Show work package status |
| `./phenodag status --db FLEET_DAG.db` | Show DAG task counts |
| `./phenodag pick --agent <id> --db FLEET_DAG.db` | Atomically claim next ready task |
| `./phenodag claim --agent <id> --repo FocalPoint --branch <b> --db FLEET_DAG.db` | Lock repo/branch for work |
| `./phenodag done --agent <id> --task <task-id> --db FLEET_DAG.db` | Mark task complete |
| `cargo run -p quality-gate` | Run unified quality gate |
| `cargo run -p fr-coverage` | Generate FR coverage matrix |
| `cargo run -p release-cut -- vX.Y.Z` | Plan release (dry-run) |
| `cargo run -p release-cut -- vX.Y.Z --execute` | Execute release |
| `target-pruner --dry-run` | Preview stale target pruning |
| `target-pruner --prune` | Prune stale targets |
| `disk-check --min-gb 10` | Verify disk space before dispatch |

## Status

This AGENTS.md is living governance for FocalPoint. Update it when the working conventions change, and link any new tooling, scripts, or process notes here. The tooling mandate is non-negotiable — it exists to save time and prevent repeated effort across agents and sessions.
