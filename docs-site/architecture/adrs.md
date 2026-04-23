# Architecture Decision Records

Accepted ADRs live in [`ADR.md`](https://github.com/KooshaPari/FocalPoint/blob/main/ADR.md) at the repo root.

## Index

| ID | Title | Status |
|----|-------|--------|
| ADR-001 | Native iOS + Rust core; no cross-platform UI bridge | Accepted |
| ADR-002 | Connector trait as public contract | Accepted |
| ADR-003 | SQLite as local source of truth | Accepted |
| ADR-004 | SHA-256 hash-chained audit log | Accepted |
| ADR-005 | Dual ledger (rewards + penalties) over single scoring system | Accepted |
| ADR-006 | Rule DSL in TOML, not Lua / JavaScript | Accepted |
| ADR-007 | UniFFI over hand-written bindgen | Accepted |
| ADR-008 | Fail-loudly error policy (no silent fallbacks) | Accepted |

The full text of each ADR lives in [`ADR.md`](https://github.com/KooshaPari/FocalPoint/blob/main/ADR.md) and in the source-of-truth directory `docs/adr/` (as it's scaffolded in Phase 1).

## Proposing an ADR

1. Copy the template from `docs/adr/0000-template.md` (to be added).
2. Number it sequentially.
3. Open a PR titled `adr: <short title>`.
4. Request review from the maintainer. ADRs require explicit acceptance; `rejected` and `superseded` are also valid terminal states.
5. Once accepted, add the row to this index and to `ADR.md`.

## Status definitions

- **Proposed** — PR open, discussion ongoing.
- **Accepted** — merged, binding.
- **Deprecated** — still in effect, but a newer ADR supersedes part of it; migration ongoing.
- **Superseded by ADR-XXX** — no longer authoritative; cross-link to replacement.
- **Rejected** — closed without merging; kept in history for the reasoning.
