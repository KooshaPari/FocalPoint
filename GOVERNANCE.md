# FocalPoint Governance

FocalPoint is a community-driven, open-source project. This document describes our decision-making structures, how to propose breaking changes, and how to become a project maintainer.

## Core Principles

1. **Transparency.** All major decisions are documented and visible to contributors.
2. **Inclusive leadership.** We rotate maintainer responsibilities and mentor new contributors into leadership.
3. **Stable traits.** The `Connector`, `EventStore`, `RuleStore`, `WalletStore`, `PenaltyStore`, `ClockPort`, and `SecureSecretStore` traits are the public contract. Breaking changes to these require an RFC and a major semver bump.
4. **Audit chain integrity.** The append-only event store and tamper-evident audit trail are non-negotiable security properties. Changes to `focus-audit` or `focus-crypto` are high-risk and require rigorous review.

## Decision Making

### Routine Decisions

Decisions about bug fixes, documentation, new connectors, rule templates, and non-breaking enhancements follow a lightweight process:

1. **Issue or PR.** Open an issue or PR with a clear description.
2. **Discussion.** Respond to review comments within 7 days.
3. **Approval.** A maintainer approves and merges.
4. **Timeline.** Routine decisions usually resolve in 3–7 days.

### Breaking Changes & RFCs

Breaking changes to the following **require an RFC** (see [`RFC process`](#rfc-process)):

- Public trait surfaces (`Connector`, rule DSL, event schema).
- Storage format changes (breaking migration required).
- Security properties (crypto, audit chain, secret storage).
- The rule DSL syntax or semantics.
- The connector manifest schema.

**Examples that require RFCs:**
- Adding a required field to `RuleDraft` that existing rules cannot provide.
- Changing the audit chain hash function from SHA-256 to another.
- Removing a connector trait method.

**Examples that do NOT require RFCs:**
- Adding an optional field to a public struct (backward-compatible).
- Adding a new connector trait method with a default impl.
- Fixing a bug in rule evaluation.
- Adding a new connector.

### RFC Process

1. **Proposal:** Author an RFC in `docs/rfcs/NNNN-title.md`. Use the template at `docs/rfcs/0000-template.md`.
2. **Title and number:** Find the next unused number in `docs/rfcs/`. Title should be short and descriptive (e.g., `0001-plugin-sdk`, `0002-encrypted-storage`).
3. **Discussion period:** Open a GitHub issue or discussion linking to the RFC. Discussion lasts **14 days**. Anyone can weigh in.
4. **Maintainer decision:** After 14 days, a maintainer (usually the one who will implement it) decides: accept, request changes, or decline. Decisions are documented in the RFC's "Decision" section.
5. **Implementation:** Once accepted, the RFC is moved to `docs/rfcs/accepted/`. Implementation is tracked in a GitHub issue referencing the RFC number.
6. **Timeline:** Typical RFC lifecycle is 3–4 weeks (14 days discussion + 1–2 weeks implementation decision + implementation).

### Consensus and Escalation

- **Maintainers make decisions**, but disagreements are resolved by discussion. If consensus is hard, the maintainer group votes.
- **Tie votes or deadlock:** escalate to the project lead (currently `<maintainer-name>`; see `MAINTAINERS.md`).
- **Public decisions:** All RFCs and their outcomes are documented in the repo; no private decisions that affect the project's direction.

## Maintainers

Maintainers are trusted contributors who review PRs, triage issues, and guide the project direction.

**Current maintainers:** see `MAINTAINERS.md`.

### Becoming a Maintainer

1. **Contribute regularly** for at least 3 months (this can be bug fixes, new connectors, documentation, or review help).
2. **Demonstrate judgment:** Your reviews should be thoughtful, kind, and technically sound.
3. **Propose yourself** (or be nominated). Open an issue titled "Proposal: <name> as maintainer" and link to your contributions.
4. **Vote:** Existing maintainers vote on consensus. If unanimous or near-unanimous yes, congratulations!
5. **Onboard:** Receive write access to the repo and a profile in `MAINTAINERS.md`.

### Rotating Out

Maintainers who are inactive for 6+ months are politely invited to step down. This is not a punishment — life gets busy. Former maintainers are always welcome back and are credited in the repo history.

## Code of Conduct

All participants in FocalPoint spaces are governed by our [Code of Conduct](./CODE_OF_CONDUCT.md). Violations are reported to the project lead via security@focalpoint.app.

## Scope of Governance

Governance applies to:

- The FocalPoint GitHub repository.
- Pull requests and issues.
- Discussions and comment threads.
- Code contributions and reviews.
- Any public communication representing FocalPoint (social media, conferences, etc.).

It **does not** govern:

- Private research or forks (as long as they respect the license).
- External projects that use FocalPoint (unless they claim to be "official").
- Discussions outside the repo (Discord, Twitter, etc.) — though the Code of Conduct spirit applies.

## License

All contributions are dual-licensed under MIT OR Apache-2.0 (see `CONTRIBUTING.md`).

## Contact

- **Governance questions:** Open a GitHub issue or discussion.
- **Security concerns:** security@focalpoint.app (confidential).
- **Code of Conduct violations:** security@focalpoint.app (confidential).
- **Public contact:** see `MAINTAINERS.md`.
