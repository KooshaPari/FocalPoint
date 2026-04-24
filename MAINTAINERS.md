# Maintainers

FocalPoint is maintained by a rotating group of trusted contributors. This document lists active maintainers, their scopes, and how to contact them.

## Active Maintainers

| Name | GitHub | Scope | Status | Contact |
|------|--------|-------|--------|---------|
| <maintainer-name-1> | @<handle-1> | Core architecture, crypto, audit chain | Active | Maintainer on most PRs |
| <maintainer-name-2> | @<handle-2> | Connectors, platform SDK | Active | First reviewer for connector PRs |
| <maintainer-name-3> | @<handle-3> | Rules DSL, templates, docs | Active | Rules and docs maintainer |

## Scope Descriptions

- **Core architecture:** Decisions affecting trait surfaces, trait implementations, cross-crate APIs, and foundational patterns.
- **Crypto & audit chain:** Changes to `focus-crypto`, `focus-audit`, and secret storage. These are security-critical.
- **Connectors:** New `Connector` trait implementations and connector SDKs. Reviewers ensure manifest schemas, OAuth flows, and event shape are sound.
- **Rules DSL:** Changes to the rule language, parser, and evaluator. Reviews ensure backward compatibility and usability.
- **Templates & examples:** Sample rule packs, connector fixtures, and example code.
- **Docs:** Website, API references, developer guides, and governance documentation.

## Decision Rights

- **Approvals:** Maintainers with relevant scope can approve PRs. One approval is usually sufficient for routine changes.
- **RFCs:** Any maintainer can propose an RFC; the maintainer group votes on acceptance.
- **Breaking changes:** Require consensus among scope owners.
- **Release coordination:** Delegated to the project lead or a designated release manager.

## Meetings & Sync

- No mandatory meetings. Asynchronous GitHub discussions are the norm.
- Major decision discussions may happen in a private channel (Discord, Slack) to keep noise down; outcomes are always public in the repo.

## Adding / Removing Maintainers

See `GOVERNANCE.md` → "Becoming a Maintainer" and "Rotating Out".

## Contact

- **Public:** Open an issue or discussion on GitHub.
- **Direct:** Check the "Contact" column above for public channels (GitHub handles, email, etc.).
- **Security or conduct:** security@focalpoint.app (confidential).
- **General governance:** See `GOVERNANCE.md`.
