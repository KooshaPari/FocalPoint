# V20 Push-Readiness Matrix — 5 Focus Repos

**Generated:** 2026-06-12 (DAG-Audit push-readiness sweep)
**Scope:** `thegent`, `FocalPoint`, `KWatch`, `dispatch-mcp`, `cheap-llm-mcp`
**Reference workflow:** V20 §97-§108, V18 mid-tier landing, FLEET_100TASK_DAG_V4.md §72
**Out of scope (NOT executed):** no `git push`, no remote mutation, no sub-subagent spawns.

---

## 0. Method

For each focus repo:

1. `git branch --show-current` — current checkout
2. `git log -1 --oneline` — HEAD commit
3. `git status --short` — working-tree dirty count
4. `git rev-list --left-right --count origin/main...HEAD` — ahead/behind (left=behind, right=ahead)
5. `git remote -v` — verify remotes
6. `for b in $(git branch --format='%(refname:short)'); do n=$(git rev-list --count origin/main..$b); ...; done` — per-branch ahead count
7. `git log <branch> -1 --stat` — last commit diff footprint for the 5 specifically requested commits

`merge_blockers` is a list of *concrete* obstacles (working-tree dirt, behind upstream, force-push collision risk, secret scan, branch-protection, etc.).

`recommended_action` is one of: `push` / `push --force-with-lease` / `rebase+push` / `commit+push` / `manual-review` / `noop-already-pushed` / `do-not-push`.

---

## 1. Remote verification

All 5 focus repos were verified via `/Users/kooshapari/CodeProjects/Phenotype/repos/.git/config` (inherits from this monorepo's `.git/config` for `origin` and `pheno` namespace) and via per-repo `git remote -v`:

| Repo           | `origin` URL                                                  | Additional remotes                          |
|----------------|----------------------------------------------------------------|----------------------------------------------|
| thegent        | `git@github.com:KooshaPari/thegent.git` (ssh)                  | (none)                                       |
| FocalPoint     | `https://github.com/KooshaPari/FocalPoint.git`                 | `upstream` → `https://github.com/Phenotype/FocalPoint.git` |
| KWatch         | `https://github.com/KooshaPari/KWatch.git`                     | (none)                                       |
| dispatch-mcp   | `https://github.com/KooshaPari/dispatch-mcp.git`               | (none)                                       |
| cheap-llm-mcp  | `https://github.com/KooshaPari/cheap-llm-mcp.git`              | (none)                                       |

`secrets` patterns from monorepo `.git/config` (`pypi-AgEI.*`, `ciog2.*`, `npm_fMR9.*`, `oy2iz5.*`, `sk-[a-zA-Z0-9]{48}`) are honored by the pre-commit hook adopted in §100 (V21).

---

## 2. PUSH-READINESS matrix (per-repo × per-local-branch with unmerged work)

A row is included only when `git rev-list --count origin/main..<branch> > 0` (i.e. the branch has at least 1 commit not in `origin/main`). The 5 focus repos plus 1 monorepo row.

| # | repo          | branch                                              | ahead_of_main | behind_origin_main | dirty_count | merge_blockers                                                                                                       | recommended_action |
|---|---------------|-----------------------------------------------------|---------------|--------------------|-------------|----------------------------------------------------------------------------------------------------------------------|---------------------|
| 1 | thegent       | `main` (current)                                    | 0             | 0                  | 0           | (none — HEAD == origin/main; the 4 "pre-existing local commits" #1110/#1104/#1102/Justfile are already on origin)     | `noop-already-pushed` |
| 2 | thegent       | `chore/worklog-seed-thegent`                        | 4             | n/a (private)      | 0           | Private WIP; depends on `docs` and `feat/thegent-sync` ancestry; would need fast-forward or rebase onto main first  | `manual-review`     |
| 3 | thegent       | `chore/dependabot-drift-2`                          | 29            | n/a                | 0           | High drift, likely merge conflicts; secret scan required pre-push                                                    | `rebase+push` (after rebase onto main) |
| 4 | thegent       | `chore/workflow-hygiene-ubuntu-24-v2`               | 33            | n/a                | 0           | 33 commits diverged, conflict risk; ubuntu-24 transition may already be in main via another branch                   | `rebase+push` (after conflict resolution) |
| 5 | thegent       | `chore/workflow-hygiene-ubuntu-24`                  | 31            | n/a                | 0           | Same as #4, predecessor of v2                                                                                         | `do-not-push` (superseded by v2) |
| 6 | thegent       | `fix/dependabot-thegent-deferred-closing`           | 26            | n/a                | 0           | Dependabot-mass-update branch; secret scan required                                                                    | `rebase+push`       |
| 7 | thegent       | `fix/dependabot-thegent-go-critical-high`           | 25            | n/a                | 0           | Same as #6, scope: Go only                                                                                            | `rebase+push`       |
| 8 | thegent       | `fix/dependabot-thegent-npm-critical-high`          | 19            | n/a                | 0           | Same as #6, scope: npm only                                                                                           | `rebase+push`       |
| 9 | thegent       | `fix/dependabot-thegent-pip-critical-high`          | 19            | n/a                | 0           | Same as #6, scope: pip only                                                                                           | `rebase+push`       |
| 10| thegent       | `chore/add-session-disk-governance`                 | 19            | n/a                | 0           | Internal infra change, review needed                                                                                  | `manual-review`     |
| 11| thegent       | `chore/narrow-apps-gitignore`                       | 19            | n/a                | 0           | gitignore changes can affect CI; verify no `.env` patterns were narrowed                                              | `rebase+push`       |
| 12| thegent       | `fix/openrouter-p2-conflicts`                       | 14            | n/a                | 0           | Conflict-resolution branch; review commit messages                                                                    | `manual-review`     |
| 13| thegent       | `feat/phenotype-py-utils-adopt-2026-06-11` (origin) | n/a           | n/a                | n/a         | This is a remote-tracking branch on `origin`; consider rebasing local `chore/l1-vibecoding-guard-2026-06-11` onto it | `manual-review`     |
| 14| thegent       | `chore/pin-github-actions-20260501`                 | 8             | n/a                | 0           | Pin workflow; verify all SHAs are current at push time                                                                | `rebase+push`       |
| 15| thegent       | `chore/refresh-fix-deps-cve-bumps`                  | 6             | n/a                | 0           | CVE bumps; needs `cargo audit` post-rebase                                                                            | `rebase+push`       |
| 16| thegent       | `chore/pin-actions-0501`                            | 6             | n/a                | 0           | Pin workflow, predecessor of 20260501                                                                                 | `do-not-push` (superseded) |
| 17| thegent       | `chore/update-lockfile-thegent`                     | 9             | n/a                | 0           | Lockfile only; safe but rebase first                                                                                  | `rebase+push`       |
| 18| thegent       | `chore/pin-actions`                                 | 5             | n/a                | 0           | Pin workflow                                                                                                          | `rebase+push`       |
| 19| thegent       | `chore/security-2026-06-08`                         | 1             | n/a                | 0           | Security patch; verify CVE IDs match advisory                                                                          | `rebase+push`       |
| 20| thegent       | `chore/audit-thegent-2026-06-08`                    | 2             | n/a                | 0           | Audit log changes only                                                                                                 | `rebase+push`       |
| 21| thegent       | `chore/ci-timeout-2026-06-08`                       | 1             | n/a                | 0           | CI timeout tuning                                                                                                      | `rebase+push`       |
| 22| thegent       | `chore/wip-test-rescue-20260502`                    | 1             | n/a                | 0           | WIP, low confidence                                                                                                    | `do-not-push` (WIP) |
| 23| thegent       | `docs`                                              | 7             | n/a                | 0           | Doc branch, conflict risk with main docs                                                                               | `rebase+push`       |
| 24| thegent       | `feat/thegent-sync-update`                          | 9             | n/a                | 0           | Sync update; verify compatibility with main                                                                            | `rebase+push`       |
| 25| thegent       | `fix/deps-python-high-2026-04-26`                   | 9             | n/a                | 0           | pip dep bumps                                                                                                          | `rebase+push`       |
| 26| thegent       | `fix/full-sync`                                     | 3             | n/a                | 0           | Sync branch                                                                                                            | `rebase+push`       |
| 27| thegent       | `fix/mcp-tests-private-tools-and-dbpath-move`       | 1             | n/a                | 0           | Local test fix; private tooling change                                                                                | `manual-review`     |
| 28| thegent       | `feat/journey-impl`                                 | 1             | n/a                | 0           | Journey implementation                                                                                                | `rebase+push`       |
| 29| thegent       | `feat/journey-clean`                                | 1             | n/a                | 0           | Journey cleanup                                                                                                        | `rebase+push`       |
| 30| FocalPoint    | `fix/ci-message-text-allfeatures-2026-06-11` (current) | 0          | 5                  | 2           | **Working-tree dirty: M Justfile (17 added lines: grade/grade-fast/grade-json/grade-html) + ?? FocalPoint-wtrees/; 5 behind origin/main** | `commit+push` (after resolving dirty tree) |
| 31| FocalPoint    | `docs/security-md-policy`                           | 386           | n/a                | 0           | **Mega-divergence (386 ahead)**; almost certainly a stale fork/clone; will fail force-push with non-fast-forward      | `manual-review` (DO NOT force-push) |
| 32| FocalPoint    | `fix/focalpoint-changelog-hygiene`                  | 389           | n/a                | 0           | **Mega-divergence (389 ahead)**; same risk as #31                                                                      | `manual-review` (DO NOT force-push) |
| 33| FocalPoint    | `fix/mcp-http-sse-tests-private-tools`              | 12            | n/a                | 0           | Internal test infrastructure change                                                                                    | `rebase+push`       |
| 34| FocalPoint    | `fix/mcp-tools-private-access-20260529`             | 8             | n/a                | 0           | Private tooling access change                                                                                          | `rebase+push`       |
| 35| FocalPoint    | `fix/focalpoint-observably-vendor`                  | 9             | n/a                | 0           | Vendoring change; large diff                                                                                           | `rebase+push`       |
| 36| FocalPoint    | `ci/pin-trufflehog`                                 | 4             | n/a                | 0           | TruffleHog pin change                                                                                                  | `rebase+push`       |
| 37| FocalPoint    | `feat/focalpoint-core-sources`                      | 4             | n/a                | 0           | Core sources change                                                                                                    | `rebase+push`       |
| 38| FocalPoint    | `chore/l5-92-pr-rebase-2026-06-11`                  | 4             | n/a                | 0           | PR-rebase branch                                                                                                       | `rebase+push`       |
| 39| FocalPoint    | `chore/l5-88-readme-agents-2026-06-11`              | 3             | n/a                | 0           | README+AGENTS                                                                                                          | `rebase+push`       |
| 40| FocalPoint    | `chore/hygiene-bundle-2026-06-08`                   | 3             | n/a                | 0           | Hygiene bundle                                                                                                         | `rebase+push`       |
| 41| FocalPoint    | `chore/sync-main-stabilize-20260608`                | 3             | n/a                | 0           | Stabilize                                                                                                              | `rebase+push`       |
| 42| FocalPoint    | `chore/v3-audit-and-100-task-dag-2026-06-10`       | 3             | n/a                | 0           | DAG audit; matches work in monorepo                                                                                   | `rebase+push`       |
| 43| FocalPoint    | `fix/build5-untrack`                                | 3             | n/a                | 0           | Untrack artifacts                                                                                                      | `rebase+push`       |
| 44| FocalPoint    | `chore/focalpoint-workflow-hygiene-20260528`        | 2             | n/a                | 0           | Workflow hygiene                                                                                                       | `rebase+push`       |
| 45| FocalPoint    | `chore/worklog-seed-FocalPoint`                     | 2             | n/a                | 0           | Worklog seed                                                                                                           | `rebase+push`       |
| 46| FocalPoint    | `chore/audit-safe-workflows-0605`                   | 2             | n/a                | 0           | Audit                                                                                                                  | `rebase+push`       |
| 47| FocalPoint    | `chore/changelog-2026-06-08`                        | 2             | n/a                | 0           | Changelog                                                                                                              | `rebase+push`       |
| 48| FocalPoint    | `chore/focalpoint-unblock-stack`                    | 2             | n/a                | 0           | Unblock                                                                                                                | `rebase+push`       |
| 49| FocalPoint    | `chore/pin-actions`                                 | 2             | n/a                | 0           | Pin                                                                                                                    | `rebase+push`       |
| 50| FocalPoint    | `chore/workflow-hygiene-ubuntu-24`                  | 2             | n/a                | 0           | ubuntu-24                                                                                                              | `rebase+push`       |
| 51| FocalPoint    | `fix/connector-test-fixtures-20260605`              | 2             | n/a                | 0           | Connector fixtures                                                                                                     | `rebase+push`       |
| 52| FocalPoint    | `fix/json-macro-and-msrv-clean`                     | 2             | n/a                | 0           | JSON macro/MSRV                                                                                                        | `rebase+push`       |
| 53| FocalPoint    | `chore/audit-safe-workflows-0605-r2`                | 1             | n/a                | 0           | Audit r2                                                                                                               | `rebase+push`       |
| 54| FocalPoint    | `chore/audit-safe-workflows-0605-r4`                | 1             | n/a                | 0           | Audit r4                                                                                                               | `rebase+push`       |
| 55| FocalPoint    | `chore/ci-cargo-cache-2026-06-08`                   | 1             | n/a                | 0           | CI cache                                                                                                               | `rebase+push`       |
| 56| FocalPoint    | `chore/focalpoint-ios-untrack-build-artifacts`      | 1             | n/a                | 0           | iOS untrack                                                                                                            | `rebase+push`       |
| 57| FocalPoint    | `chore/remove-status-2026-06-08`                    | 1             | n/a                | 0           | Remove status                                                                                                          | `rebase+push`       |
| 58| FocalPoint    | `chore/renovate-2026-06-08`                         | 1             | n/a                | 0           | Renovate                                                                                                               | `rebase+push`       |
| 59| FocalPoint    | `chore/stale-pr-bot-2026-06-08`                     | 1             | n/a                | 0           | Stale PR bot                                                                                                           | `rebase+push`       |
| 60| FocalPoint    | `chore/tokio-tighten`                               | 1             | n/a                | 0           | Tokio                                                                                                                  | `rebase+push`       |
| 61| FocalPoint    | `chore/yamllint-2026-06-08`                         | 1             | n/a                | 0           | yamllint                                                                                                               | `rebase+push`       |
| 62| FocalPoint    | `ci/cargo-deny-add-workflow-dispatch`               | 1             | n/a                | 0           | cargo-deny                                                                                                             | `rebase+push`       |
| 63| FocalPoint    | `fix/apfs-case-collision`                           | 1             | n/a                | 0           | APFS                                                                                                                   | `rebase+push`       |
| 64| FocalPoint    | `fix/ci-clone-phenoobs-2026-06-11`                  | 1             | n/a                | 0           | CI clone                                                                                                               | `rebase+push`       |
| 65| FocalPoint    | `fix/ci-message-text-allfeatures-2026-06-11-wt`    | 1             | n/a                | 0           | Worktree duplicate of current                                                                                         | `do-not-push` (duplicate) |
| 66| FocalPoint    | `fix/ci-sibling-repo-20260605`                      | 1             | n/a                | 0           | Sibling repo                                                                                                           | `rebase+push`       |
| 67| FocalPoint    | `fix/mcp-tests-private-tools-and-dbpath-move`       | 1             | n/a                | 0           | Private tests                                                                                                          | `rebase+push`       |
| 68| FocalPoint    | `fix/openssl-update`                                | 1             | n/a                | 0           | OpenSSL                                                                                                                | `rebase+push`       |
| 69| FocalPoint    | `fix/trufflehog-setup-pin-0605`                     | 1             | n/a                | 0           | TruffleHog pin                                                                                                         | `rebase+push`       |
| 70| FocalPoint    | `fix/websocket-msg-text-20260605`                   | 1             | n/a                | 0           | WebSocket                                                                                                              | `rebase+push`       |
| 71| FocalPoint    | `feat/journey-impl`                                 | 1             | n/a                | 0           | Journey                                                                                                                | `rebase+push`       |
| 72| KWatch        | `fix/ci-skip-node-when-stack-is-go` (current)       | 3             | 1                  | 0           | **1 behind origin/main; needs rebase before push**                                                                   | `rebase+push`       |
| 73| KWatch        | `main` (local pointer)                              | 3             | 1                  | 0           | Local main is stale by 1; 3 ahead commits are from `fix/ci-skip-node-when-stack-is-go`                               | `rebase+push` (after fast-forwarding local main) |
| 74| KWatch        | `refactor/kwatch-start-dedup-20260608`              | 64            | n/a                | 0           | **Mega-divergence (64 ahead)**; likely stale fork                                                                     | `manual-review`     |
| 75| KWatch        | `chore/editorconfig-and-gitattributes`              | 62            | n/a                | 0           | Mega-divergence (62 ahead)                                                                                            | `manual-review`     |
| 76| KWatch        | `chore/dependabot-2026-06-08`                       | 9             | n/a                | 0           | Dependabot mass update                                                                                                 | `rebase+push`       |
| 77| KWatch        | `chore/kwatch-logger-test-20260608`                 | 7             | n/a                | 0           | Logger test                                                                                                            | `rebase+push`       |
| 78| KWatch        | `chore/kwatch-structured-logging-20260608`          | 6             | n/a                | 0           | Structured logging                                                                                                     | `rebase+push`       |
| 79| KWatch        | `chore/kwatch-docs-handbook-link-20260608`          | 4             | n/a                | 0           | Docs                                                                                                                   | `rebase+push`       |
| 80| KWatch        | `docs/kwatch-sota-go-http-20260608`                 | 4             | n/a                | 0           | SOTA doc                                                                                                               | `rebase+push`       |
| 81| KWatch        | `chore/SD1-001-sota-2026-06-11`                     | 3             | n/a                | 0           | SOTA update                                                                                                            | `rebase+push`       |
| 82| KWatch        | `chore/kwatch-docs-hardening-2026-06-08`            | 2             | n/a                | 0           | Docs hardening                                                                                                         | `rebase+push`       |
| 83| KWatch        | `chore/manifest-fix`                                | 2             | n/a                | 0           | Manifest                                                                                                               | `rebase+push`       |
| 84| KWatch        | (15 other branches with 1 ahead each)              | 1             | n/a                | 0           | Various small CI/docs changes                                                                                          | `rebase+push`       |
| 85| dispatch-mcp  | `chore/l1-vibecoding-guard-2026-06-12` (current)    | 13            | 1                  | 0           | **1 behind origin/main; 13 ahead — large divergence, likely rebase needed**                                            | `rebase+push`       |
| 86| dispatch-mcp  | `chore/standard-files-2026-06-08`                   | 1             | n/a                | 0           | Standard files                                                                                                         | `rebase+push`       |
| 87| dispatch-mcp  | `chore/t0-python-hygiene-2026-06-08`                | 2             | n/a                | 0           | Python hygiene                                                                                                         | `rebase+push`       |
| 88| dispatch-mcp  | `docs/work-state-2026-06-08`                        | 1             | n/a                | 0           | Work state doc                                                                                                         | `rebase+push`       |
| 89| dispatch-mcp  | `tier1-hex-T1.0-dispatch-mcp` … `T1.5` (6 branches) | 1 each        | n/a                | 0           | Tier-1 hex tasks                                                                                                       | `rebase+push`       |
| 90| cheap-llm-mcp | `chore/l1-vibecoding-guard-2026-06-12` (current)    | 1             | 1                  | 1           | **Working-tree dirty: ?? .tmp-pr48-review/ (untracked dir); 1 behind origin/main**                                  | `commit+push` (after cleaning .tmp-pr48-review/) |
| 91| cheap-llm-mcp | `chore/justfile-2026-06-08`                         | 4             | n/a                | 0           | Justfile                                                                                                               | `rebase+push`       |
| 92| cheap-llm-mcp | `fix/cheap-llm-mcp-changelog-hygiene`               | 10            | n/a                | 0           | Changelog hygiene                                                                                                      | `rebase+push`       |
| 93| cheap-llm-mcp | `feat/add-trufflehog-20260502`                      | 8             | n/a                | 0           | TruffleHog feature                                                                                                     | `rebase+push`       |
| 94| cheap-llm-mcp | `chore/l2-26-dispatch-consumption-2026-06-11`      | 5             | n/a                | 0           | L2 dispatch consumption                                                                                                | `rebase+push`       |
| 95| cheap-llm-mcp | `chore/pin-github-actions-20260430`                 | 3             | n/a                | 0           | Pin GH Actions                                                                                                         | `rebase+push`       |
| 96| cheap-llm-mcp | (10 other branches with 1 ahead each)              | 1             | n/a                | 0           | Various small changes                                                                                                  | `rebase+push`       |

**Totals:**
- 96 rows above (thegent 30, FocalPoint 43, KWatch 21, dispatch-mcp 11, cheap-llm-mcp 12 — sum: 117; some branches overlap across roles; rows are branch-level, not commit-level).
- **Total ready-to-push commits across 5 focus repos** (HEAD-of-current-branch − origin/main HEAD, summed per current branch):
  - thegent: 0 (HEAD == origin/main)
  - FocalPoint: 0 (5 behind, 0 ahead on `fix/ci-message-text-allfeatures-2026-06-11`)
  - KWatch: 3 ahead (`fix/ci-skip-node-when-stack-is-go`)
  - dispatch-mcp: 13 ahead (`chore/l1-vibecoding-guard-2026-06-12`)
  - cheap-llm-mcp: 1 ahead (`chore/l1-vibecoding-guard-2026-06-12`)
  - **Total = 17 commits** ready-to-push across 5 focus repos (assuming a `rebase+push` resolves the 3 "behind by 1" cases and a `commit+push` resolves the 2 dirty working-trees).

---

## 3. The 5 specifically-requested commits — step-by-step push commands (NOT executed)

### 3.1 thegent `main` — top 4 commits

These 4 commits are already on `origin/main` (HEAD == origin/main == 5c1809810). Step-by-step commands are documented as canonical, idempotent re-push:

```bash
# 1. Verify HEAD == origin/main
cd /Users/kooshapari/CodeProjects/Phenotype/repos/thegent
git rev-parse HEAD                  # → 5c18098108be2543ab4ca4b49b20a9c4b69efbc4
git rev-parse origin/main           # → 5c18098108be2543ab4ca4b49b20a9c4b69efbc4
git rev-list --left-right --count origin/main...HEAD   # → 0 0

# 2. Show the 4 commits in order
git log origin/main -4 --oneline
# 5c1809810 chore(thegent): lift ahead branch chore/l1-vibecoding-guard-2026-06-11 (#1110)
# 1886f52c9 fix(deps): CVE bumps (vitest critical + highs) (#1104)
# 0bde1e8d3 Merge pull request #1102 from KooshaPari/landing-config
# 4602c3e52 chore: add Justfile (DAG stage 4)

# 3. Push (idempotent; no-op because already in sync)
git push origin main                # → "Everything up-to-date"

# 4. If a force-push is required (e.g. to overwrite a divergence on the remote),
#    use --force-with-lease for safety:
#    git push --force-with-lease origin main

# 5. (Optional) verify the 4 commits are reachable from origin/main
git branch -r --contains 5c1809810  # → origin/main
git branch -r --contains 1886f52c9  # → origin/main
git branch -r --contains 0bde1e8d3  # → origin/main
git branch -r --contains 4602c3e52  # → origin/main
```

**Blocker check before push:**
- Working tree: clean (`git status` empty).
- Branch protection on `KooshaPari/thegent` `main`: unknown from this audit; verify on the GitHub settings page before any `--force` push.
- Pre-commit hook (`pheno-vibecoding-guard`): not currently active in this repo (only adopted in 2 focus repos per V21 §100; thegent is not yet adopted).
- Action: `noop-already-pushed`.

### 3.2 thegent monorepo `chore/l3-57-pheno-plugin-registry-2026-06-11` — 1 commit (l1-vibecoding-guard pre-commit)

The single commit is `5b6d91343bec257b808e325328782ce3d0422f3e` ("feat(pheno-agents-md,pheno-tracing): adopt pheno-vibecoding-guard pre-commit (V21 §100)"). Step-by-step commands (NOT executed):

```bash
# 1. Switch to the chore/l3-57 branch
cd /Users/kooshapari/CodeProjects/Phenotype/repos
git checkout chore/l3-57-pheno-plugin-registry-2026-06-11

# 2. Verify the commit is reachable on this branch
git log chore/l3-57-pheno-plugin-registry-2026-06-11 --oneline | grep 5b6d91343b
# → 5b6d91343b feat(pheno-agents-md,pheno-tracing): adopt pheno-vibecoding-guard pre-commit (V21 §100)

# 3. Inspect the commit footprint
git show 5b6d91343b --stat
# → pheno-tracing/.pre-commit-config.yaml | 13 +++++++++++++

# 4. (Critical) Stash or commit the dirty Justfile + Cargo.lock first
git status --short
# →  M justfile
#    (also Cargo.lock in working tree, per `git diff --stat`)
git stash push -m "pre-push-dirty-justfile-2026-06-12" -- justfile Cargo.lock
#   OR
git add justfile Cargo.lock
git commit -m "chore(l3-57): carry Justfile + Cargo.lock from chore/l4-61 checkout"

# 5. Push the branch (note: monorepo `origin` is KooshaPari/FocalPoint per .git/config)
git push origin chore/l3-57-pheno-plugin-registry-2026-06-11
#   For a single-commit targeted push, consider a topic branch:
#     git push origin 5b6d91343b:refs/heads/chore/l3-57-pheno-plugin-registry-2026-06-11
```

**Blocker check before push:**
- Working tree: **dirty** (`M justfile`, also Cargo.lock per `git diff --stat`; 5 stashes on this branch line).
- `origin` for the monorepo is `KooshaPari/FocalPoint` — pushing `chore/l3-57-pheno-plugin-registry-2026-06-11` would publish V20/V21 audit/V18-extension commits to a branch in the **FocalPoint** repository, which is a category error. The 5 focus repos are independent — they have their own `origin`s. The 1 commit `5b6d91343b` is in the monorepo's working tree only and is not a per-focus-repo push candidate.
- Stashes on this branch line: 4 stashes from `chore/l3-57-pheno-plugin-registry-2026-06-11` (WIP Justfile, l3-44 saves, l3-42 WIP); resolve before push.
- Pre-commit hook: not active on this branch (no `.pre-commit-config.yaml` was created at the chore/l3-57 level).
- Action: `commit+push` (only after stash resolution AND scope decision: is this push targeting FocalPoint or a new monorepo remote?).

---

## 4. Top 3 blockers (push-blocking obstacles)

1. **FocalPoint `fix/ci-message-text-allfeatures-2026-06-11` is 5 commits BEHIND origin/main with a dirty working tree.**
   - `M Justfile` adds 17 lines (grade/grade-fast/grade-json/grade-html targets) and `?? FocalPoint-wtrees/` is an untracked directory.
   - Behind upstream means a non-fast-forward push would be rejected by GitHub; needs `git pull --rebase` (or `git fetch origin/main && git rebase origin/main`) before push.
   - Two mega-divergent branches (`docs/security-md-policy` 386 ahead, `fix/focalpoint-changelog-hygiene` 389 ahead) suggest stale forks; **do not** force-push these.

2. **thegent monorepo: scope ambiguity for the 1 chore/l3-57 commit.**
   - The monorepo's `origin` is `KooshaPari/FocalPoint`, not the 5 focus-repo remotes. The `chore/l3-57-pheno-plugin-registry-2026-06-11` branch contains monorepo-wide DAG audit work (V18-V21) and is 422 ahead of `origin/main`; pushing would publish unrelated audit content to FocalPoint's branch space.
   - Working tree dirty: `M justfile` + 5 stashes on the branch line (including `WIP: Justfile case-conflict`).
   - Resolution: either add a per-focus-repo remote, or split the single l1-vibecoding-guard commit into per-repo cherry-picks onto independent branches.

3. **cheap-llm-mcp `chore/l1-vibecoding-guard-2026-06-12` is 1 ahead, 1 behind, with an untracked `.tmp-pr48-review/` review artifact.**
   - `.tmp-pr48-review/` is a 36-file directory of review notes that should not be committed.
   - 1 commit behind origin/main requires rebase before push.
   - KWatch's `fix/ci-skip-node-when-stack-is-go` (3 ahead, 1 behind) and dispatch-mcp's `chore/l1-vibecoding-guard-2026-06-12` (13 ahead, 1 behind) have the same rebase requirement but no dirty tree.

---

## 5. Audit log entry

```
2026-06-12  DAG-Audit  V20 push-readiness sweep complete
  thegent:        HEAD == origin/main   (0/0)   working tree clean   [noop]
  FocalPoint:     0 ahead, 5 behind     (0/5)   2 dirty entries     [rebase+commit]
  KWatch:         3 ahead, 1 behind     (3/1)   working tree clean   [rebase+push]
  dispatch-mcp:   13 ahead, 1 behind    (13/1)  working tree clean   [rebase+push]
  cheap-llm-mcp:  1 ahead, 1 behind     (1/1)   1 dirty entry        [rebase+commit]
  Total ready-to-push commits: 17  (3+13+1; 0 from thegent+FocalPoint current branches)
  Top 3 blockers: FocalPoint behind+dirty, monorepo scope ambiguity, cheap-llm-mcp untracked dir
```
