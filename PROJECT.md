# FocalPoint — Project Overview

**FocalPoint** is a connector-first, audit-chained screen-time platform for iOS, Android, and CLI. It enables rule-based time management driven by external data sources (LMS, calendar, health, finance, task systems).

## What is FocalPoint?

FocalPoint combines rule-based policy enforcement with an audit trail to help users implement screen-time guardrails. Unlike blocking-first platforms, FocalPoint is:

- **Connector-first:** Integrates with external systems (Canvas, Todoist, Apple Health, YNAB, Google Calendar) to drive smart rules.
- **Audit-chained:** Every decision (reward, penalty, policy change) is cryptographically logged and tamper-evident.
- **Rule-driven:** Flexible rule DSL (TOML) lets users encode custom behavior ("if sleep-debt > 2h, activate strict mode").
- **Cross-platform:** Shared Rust core consumed by native iOS/Android via UniFFI/JNI, with CLI for power users.
- **Local-first:** SQLite is the source of truth; services are optional.

### Example Use Case

A student uses Canvas (LMS), Todoist (tasks), and Apple Health (sleep). They define a rule:

```toml
[rule.exam-focus]
name = "Exam Mode"
condition = "exam_pending & sleep_debt > 2h"
action = "strict_mode"  # blocks socials during exam period + if sleep-deprived
duration = "2h"
```

When an exam is detected in Canvas AND the student has accumulated >2h of sleep debt, FocalPoint activates strict mode. Every penalty or reward is logged in the audit chain (tamper-evident).

## Who Maintains It

**Primary maintainer:** [@kooshapari](https://github.com/KooshaPari)

**Maintainer group:** See [MAINTAINERS.md](./MAINTAINERS.md) for active contributors and scope ownership.

- **Core architecture:** Trait surfaces, cross-crate APIs, foundational patterns
- **Connectors:** Connector trait implementations, OAuth flows, event schemas
- **Rules DSL:** Parser, evaluator, rule composition
- **Crypto & audit:** ED25519 signatures, tamper-evident chains, secret storage
- **Docs:** Website, API references, governance, contributor guides

## How to Contribute

1. **Read the docs** → Start with [CHARTER.md](./CHARTER.md) (scope & principles) and [ADR.md](./ADR.md) (decisions).
2. **Check for existing issues** → Search [open issues](https://github.com/KooshaPari/FocalPoint/issues) to avoid duplicates.
3. **Open a proposal** → File a feature request, connector request, or bug report using templates (see `.github/ISSUE_TEMPLATE/`).
4. **For non-trivial work** → Coordinate via AgilePlus before coding (see [CONTRIBUTING.md](./CONTRIBUTING.md)).
5. **Follow code style** → Run `task verify` locally; commits must be signed and DCO'd (see [CONTRIBUTING.md](./CONTRIBUTING.md)).

### Ways to Contribute

| Type | Effort | Path |
|------|--------|------|
| **Connectors** | Medium–Large | Implement `Connector` trait for a new external system. See [docs-site/connector-sdk/spec.md](./docs-site/connector-sdk/spec.md). |
| **Rule templates** | Small–Medium | Author a sample rule pack for common use case. See [docs-site/rules/](./docs-site/rules/). |
| **Bug reports** | Small | Report reproducible issues with rule DSL + event stream. Use [bug report template](./github/ISSUE_TEMPLATE/bug_report.yml). |
| **Documentation** | Small–Medium | Improve docs-site pages (edit-this-page links on every page). |
| **Mascot (Coachy)** | Small | Write copy, lines, and personality traits. See [docs-site/mascot/](./docs-site/mascot/). |

## Documentation Map

| Document | Purpose | Audience |
|----------|---------|----------|
| [README.md](./README.md) | User-facing overview | End users, stakeholders |
| [CHARTER.md](./CHARTER.md) | Project scope & principles | Contributors, maintainers |
| [ADR.md](./ADR.md) | Architectural decisions | Engineers, reviewers |
| [FUNCTIONAL_REQUIREMENTS.md](./FUNCTIONAL_REQUIREMENTS.md) | Feature specifications | QA, testers, feature trackers |
| [CONTRIBUTING.md](./CONTRIBUTING.md) | Contributor guidelines | New contributors, code reviewers |
| [MAINTAINERS.md](./MAINTAINERS.md) | Maintainer roles & decision rights | Maintainers, governance |
| [SECURITY.md](./SECURITY.md) | Vulnerability reporting | Security researchers |
| [docs-site/](./docs-site/) | User & developer guides | End users, developers |

### Directory Structure

```
FocalPoint/
├── crates/                      # Rust workspace
│   ├── focus-core/              # Core types: Rule, Event, Wallet, Penalty
│   ├── focus-rules/             # Rule DSL: parser, evaluator
│   ├── focus-connectors/        # Connector trait + shared SDKs
│   ├── focus-audit/             # Audit chain: ED25519, tamper detection
│   ├── focus-storage/           # SQLite adapter
│   ├── focus-ffi/               # UniFFI bindings (Swift/Kotlin)
│   ├── connector-canvas/        # Example: Canvas LMS connector
│   ├── connector-*/             # Other connectors
│   └── ...
├── apps/                        # Platform apps
│   ├── ios/                     # Swift/SwiftUI iOS app
│   ├── android/                 # Kotlin/Jetpack Android app
│   └── builder/                 # Web UI for rule builder
├── services/                    # Optional services
│   ├── storekit-verifier/       # Apple StoreKit verification (cloud)
│   └── ...
├── docs-site/                   # VitePress documentation site
├── scripts/                     # Tooling glue (minimal shell)
├── docs/                        # Internal documentation
│   ├── governance/              # DCO, contributor conduct, release process
│   ├── research/                # Connector research, open questions
│   └── ...
├── examples/                    # Sample rule packs, connector fixtures
├── CHARTER.md                   # Project charter (scope, principles)
├── ADR.md                       # Architecture decision record
├── FUNCTIONAL_REQUIREMENTS.md   # FRs (feature specs)
├── CONTRIBUTING.md              # Contributor guide
├── MAINTAINERS.md               # Maintainer roles & ownership
├── SECURITY.md                  # Vulnerability reporting
├── CODE_OF_CONDUCT.md           # Community conduct policy
├── CLAUDE.md                    # AI agent instructions
└── README.md                    # User-facing README
```

## Development Setup

### Prerequisites

- **Rust** 1.82+ via `rustup`
- **Xcode** 15+ (iOS builds)
- **Bun** 1.1+ (docs-site)
- **lefthook** (git hooks) — install: `brew install lefthook && lefthook install`
- **Task** (task runner) — install: `brew install go-task/tap/go-task`
- **trufflehog** (secret scan) — install: `brew install trufflesecurity/trufflehog/trufflehog`

### Build & Test

```bash
# Clone
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint

# Check syntax
cargo check --workspace

# Run tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --check

# Full verification (mirrors CI)
task verify
```

### Docs Site

```bash
task docs-dev     # local dev server on http://localhost:5173
task docs-build   # static build into docs-site/.vitepress/dist
```

## Key Technologies

| Layer | Technologies |
|-------|--------------|
| **Rust core** | Trait-based architecture, `serde`, `tokio`, `anyhow` |
| **Rule DSL** | Custom TOML parser, declarative evaluation |
| **Crypto** | ED25519 (audit signatures), SHA-256 (chain hashing) |
| **Storage** | SQLite (local), optional cloud sync |
| **iOS** | SwiftUI, FamilyControls, UsageStats, UniFFI |
| **Android** | Jetpack, Accessibility Service, JNI |
| **Docs** | VitePress, Markdown + Mermaid diagrams |

## Governance & Policies

- **Language:** Rust (primary); Python/TS only for embedded runtimes; minimal shell (see [CLAUDE.md](./CLAUDE.md)).
- **Testing:** All FRs must have corresponding tests; tests reference FRs.
- **Commits:** Signed (GPG/SSH) + DCO sign-off (`Signed-off-by` trailer); conventional commits (`feat(scope):`, `fix(scope):`, etc.).
- **DCO:** All contributions require Developer Certificate of Origin (lightweight, not a CLA).
- **Audit:** Every state mutation produces an `AuditRecord`; chain is tamper-evident.
- **Releases:** Semantic versioning; coordinated via release manager.

## Blockers & Open Questions

Tracked in [docs/research/open_questions.md](./docs/research/open_questions.md):

- **Q1:** Final product name ("FocalPoint" or "Coachy" or other?).
- **Q5:** Foqos/Reef integration URLs.
- **Q8:** iOS Family Controls entitlement application status (blocks iOS testing).

Other questions are deferred or in active research.

## Quick Links

- **Issues:** [github.com/KooshaPari/FocalPoint/issues](https://github.com/KooshaPari/FocalPoint/issues)
- **Discussions:** [github.com/KooshaPari/FocalPoint/discussions](https://github.com/KooshaPari/FocalPoint/discussions)
- **Discord:** [discord.gg/focalpoint](https://discord.gg/focalpoint) (community chat)
- **Security:** [security@focalpoint.app](mailto:security@focalpoint.app) (confidential)
- **Governance:** [MAINTAINERS.md](./MAINTAINERS.md), [CHARTER.md](./CHARTER.md)

## Getting Help

- **Want to contribute?** Start with [CONTRIBUTING.md](./CONTRIBUTING.md).
- **Found a bug?** Use the [bug report template](./github/ISSUE_TEMPLATE/bug_report.yml).
- **Have a feature idea?** Use the [feature request template](./github/ISSUE_TEMPLATE/feature_request.yml).
- **Want to propose a connector?** Use the [connector request template](./github/ISSUE_TEMPLATE/connector_request.yml).
- **Security issue?** Email [security@focalpoint.app](mailto:security@focalpoint.app).
- **General questions?** Ask in [Discussions](https://github.com/KooshaPari/FocalPoint/discussions) or [Discord](https://discord.gg/focalpoint).
