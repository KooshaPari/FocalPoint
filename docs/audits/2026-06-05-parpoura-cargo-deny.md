# Parpoura — cargo-deny Hygiene Sweep 2026-06-05

## Summary

Ran `cargo deny check` against the Parpoura workspace from the
`feat/cargo-deny` worktree on 2026-06-05. Tool: `cargo-deny 0.19.0`.
Result: **clean** — no advisories, no bans, no license violations, no
source violations. The only diagnostics were 18 cosmetic
`license-not-encountered` warnings (license allow-list entries that are
permissive but no current crate in the graph is published under that
license) and 1 `no-license-field` warning for the in-tree `agileplus =
0.1.0` member crate whose manifest does not yet carry a `license`
field.

Full log: `/tmp/parpoura-deny-20260605.log` (112 lines).

## Final cargo-deny status line

```
advisories ok, bans ok, licenses ok, sources ok
```

## Configuration baseline

`deny.toml` (in repo root of the cargo-deny worktree) declares:

- `[advisories]` — no `ignore` block (zero advisories currently
  active in the dep graph, so no entries are required).
- `[licenses]` — version 2 allow-list of 19 SPDX expressions covering
  MIT/Apache-2.0/BSD family, MPL-2.0, CC0/CC-BY-SA, BlueOak,
  CDLA-Permissive-2.0, Unlicense, WTFPL, Unicode, 0BSD, GPL-3.0-only.
- `[bans]` — `multiple-versions = warn`, `wildcards = warn`.
- `[sources]` — `unknown-git = deny`, `unknown-registry = warn`,
  only `crates.io` allow-listed.

## Counts (before -> after)

| Check | Before | After | Delta |
|-------|-------:|------:|------:|
| Advisories (RUSTSEC) | 0 | 0 | 0 |
| Bans (multi-version / wildcards) | 0 | 0 | 0 |
| License violations | 0 | 0 | 0 |
| Source violations | 0 | 0 | 0 |
| License-not-encountered warnings | 18 | 18 | 0 (cosmetic) |
| No-license-field warnings | 1 | 1 | 0 (cosmetic) |

No advisories were fixed (none present). No advisories were added to
the `ignore` list (none required).

## What was found

- **Advisories:** none. The advisory database lookup against the
  current dep graph produced no matches. Nothing to bump, nothing to
  ignore.
- **Bans:** none. No crate is duplicated across the workspace at
  conflicting versions, and no wildcard version requirements
  (`*`, `>=*`) are present.
- **Licenses:** every crate in the graph resolves to an SPDX
  expression that is in the allow-list.
- **Sources:** every crate resolves through `crates.io` (the only
  allow-listed registry). No git-only or unknown-registry deps.

## What was fixed

Nothing — the run was clean on first invocation. No `Cargo.toml`
edits, no `deny.toml` edits, no dependency bumps were required.

## What was ignored

Nothing — no advisory, ban, license, or source rule produced a
non-cosmetic diagnostic. The `ignore` block in `[advisories]`
remains intentionally absent.

## Cosmetic warnings (out of scope for this sweep)

These are warnings, not failures, and are not blockers. Tracked here
for future work-package planning only:

1. **18 `license-not-encountered`** — entries in the allow-list
   (e.g. `GPL-3.0-only`, `ISC`, `MPL-2.0`, `Unicode-3.0`,
   `Unicode-DFS-2016`, `Zlib`, `0BSD`, `BlueOak-1.0.0`,
   `CDLA-Permissive-2.0`, `Unlicense`, `WTFPL`, `CC-BY-SA-4.0`) that
   no current crate uses. The list is intentionally permissive for
   future ecosystem pulls; trimming would create a churn cost without
   a security benefit.
2. **1 `no-license-field`** — the in-tree `agileplus = 0.1.0` member
   crate's `Cargo.toml` is missing a top-level `license` key. This is
   a documentation/manifest hygiene issue, not a supply-chain
   advisory, and is already covered by the repo-wide manifest
   work-package (see `manifest-fix` worktree).

## How to reproduce

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/Parpoura-wtrees/cargo-deny
cargo deny --version   # expect cargo-deny 0.19.0
cargo deny check 2>&1 | tee /tmp/parpoura-deny-$(date +%Y%m%d).log
```

Expected final line: `advisories ok, bans ok, licenses ok, sources ok`.

## Commit

- Branch: `feat/cargo-deny`
- Commit: see `git log -1 --oneline` after the sweep commit lands.
- Message: `chore(Parpoura): cargo-deny sweep 2026-06-05`
- Not pushed (per task spec).

## Notes for the next sweep

- If any new RUSTSEC id appears between sweeps, prefer a non-major
  dependency bump over adding it to `deny.toml` `[advisories]
  ignore`. Only ignore when the fix requires a major bump that would
  land in a separate breaking-change work package.
- If a crate is added to the `ignore` list, the entry **must**
  include `reason = "..."` and `date = "YYYY-MM-DD"` per repo
  governance (expiry tracked as a follow-up work item).
