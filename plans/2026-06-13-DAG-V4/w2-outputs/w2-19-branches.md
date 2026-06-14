# W2-19: FocalPoint Branch Cleanup Summary

**Date:** 2026-06-13
**Repo:** FocalPoint
**Task:** Identify and delete merged/stale branches, preserving `main` and branches with `chore/l5*` or `wt-*` prefixes.

---

## W1-11 Input

The W1-11 output identified three FocalPoint branches:

- `chore/l4-68-pheno-context-2026-06-11` (97623acb)
- `chore/l5-88-readme-agents-2026-06-11` (7173f974)
- `chore/l5-92-pr-rebase-2026-06-11` (893b8deb)

All three were present in the repo at the time of this cleanup. The `chore/l5*` branches are protected by prefix policy and were not deleted.

---

## Pre-Cleanup Branch Inventory

`git branch -a` showed **51 local branches** and **~78 remote-tracking branches** (excluding HEAD and origin duplicates).

### Merged Branches

`git branch -a --merged main` returned only `main`. No other branches were detected as fully merged via fast-forward or merge commit.

### Stale Branches (no commits in 30+ days, i.e., before 2026-05-14)

| Branch | Last Commit | Protected? |
|--------|-------------|------------|
| `ci/cargo-deny-add-workflow-dispatch` | 2026-04-27 | No |
| `docs/security-md-policy` | 2026-05-07 | No |
| `feat/journey-impl` | 2026-05-01 | No |
| `fix/focalpoint-changelog-hygiene` | 2026-05-06 | No |
| `json-macro-and-msrv` | 2026-05-07 | No |
| `scratch-clean` | 2026-05-07 | No |
| `origin/ci/cargo-deny-add-workflow-dispatch` | 2026-04-27 | No |
| `origin/docs/security-md-policy` | 2026-05-07 | No |
| `origin/feat/journey-impl` | 2026-05-01 | No |
| `origin/fix/focalpoint-changelog-hygiene` | 2026-05-06 | No |
| `origin/gh-pages` | 2026-04-26 | No (kept -- GitHub Pages deployment branch) |
| `origin/json-macro-and-msrv` | 2026-05-07 | No |
| `origin/scratch-clean` | 2026-05-07 | No |

---

## Actions Taken

### Deleted Branches

The following **6 local branches** and **6 remote branches** were deleted:

- `ci/cargo-deny-add-workflow-dispatch`
- `docs/security-md-policy`
- `feat/journey-impl`
- `fix/focalpoint-changelog-hygiene`
- `json-macro-and-msrv`
- `scratch-clean`

### Preserved Branches

The following branches were explicitly preserved per policy:

- `main`
- `chore/l5-88-readme-agents-2026-06-11` (`chore/l5*` prefix)
- `chore/l5-92-pr-rebase-2026-06-11` (`chore/l5*` prefix)
- `origin/chore/l5-89-worktree-collapse-2026-06-11` (`chore/l5*` prefix)
- `origin/chore/l5-90-branch-cleanup-2026-06-11` (`chore/l5*` prefix)
- `origin/chore/l5-91-stash-cleanup-2026-06-11` (`chore/l5*` prefix)
- `origin/gh-pages` (standard GitHub Pages deployment branch)

No `wt-*` prefixed branches exist in the FocalPoint repo.

---

## Post-Cleanup State

After deletion:
- `git branch -a --merged main` still shows only `main`.
- Stale check (`< 2026-05-14`) returns only `origin/gh-pages` (preserved by convention).
- All `chore/l5*` branches remain intact.
- All `wt-*` branches remain intact (none exist).

---

## Branches Remaining

`git branch -a` post-cleanup lists **45 local branches** and **~72 remote branches** (excluding HEAD). The remaining branches are active (commits within the last 30 days) or are protected by policy.
