# Worktree Collapse Plan

> **Task:** L5 #89 — Audit and plan removal of stale worktrees.  
> **Date:** 2026-06-12  
> **Branch:** `chore/l5-89-worktree-collapse-2026-06-11`  
> **Status:** PLAN ONLY — no worktrees have been removed.

---

## (a) Current Worktrees and Their Branches

| # | Worktree Path | Branch | HEAD | Uncommitted Changes |
|---|---------------|--------|------|---------------------|
| 1 | `/Users/kooshapari/CodeProjects/Phenotype/repos` | `chore/l5-89-worktree-collapse-2026-06-11` | `52dfc7aa` | `justfile` (M) |
| 2 | `/private/tmp/v20-publisher-wt-l3-57` | `chore/l3-57-pheno-plugin-registry-2026-06-11` | `5728451b` | `justfile` (M) |
| 3 | `/Users/kooshapari/CodeProjects/Phenotype/.claude/worktrees/FocalPoint-housekeeping` | *(detached HEAD)* | `cd564049` | none |
| 4 | `/Users/kooshapari/CodeProjects/Phenotype/.claude/worktrees/phenoPlugins-pr82` | `hygiene/preserve-changes` | `f331d967` | none |
| 5 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/audit-v20-2026-06-12` | `audit/crossrepo-canonical-merge-2026-06-12` | `26918c77` | `justfile` (M) |
| 6 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l2-37-branch-protection-2026-06-11` | `chore/l2-37-branch-protection-2026-06-11` | `974beb0a` | `Justfile` (M) |
| 7 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-44-byteport-cov-2026-06-11` | `chore/l3-44-byteport-cov-2026-06-11` | `1c06d4b7a6` | none |
| 8 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-48-pheno-config-2026-06-11` | `chore/l3-48-pheno-config-2026-06-11` | `cf68d394` | `pheno-config/src/lib.rs` (M) |
| 9 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-49-pheno-otel-2026-06-11` | `chore/l3-49-pheno-otel-2026-06-11` | `198c9feb5a` | none |
| 10 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-50-pheno-cli-base-2026-06-11` | `chore/l3-50-pheno-cli-base-2026-06-11` | `36bf0ee192` | none |
| 11 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-51-pheno-fastapi-base-2026-06-11` | `chore/l3-51-pheno-fastapi-base-2026-06-11` | `81b6d3e8d0` | none |
| 12 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-52-pheno-go-ctxkit-2026-06-11` | `chore/l3-52-pheno-go-ctxkit-2026-06-11` | `bee7acaa` | none |
| 13 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-53-pheno-zod-pydantic-2026-06-11` | `chore/l3-53-pheno-zod-pydantic-2026-06-11` | `66384583` | none |
| 14 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-55-pheno-ssot-template-2026-06-11` | `chore/l3-55-pheno-ssot-template-2026-06-11` | `39a2edc074` | none |
| 15 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-56-pheno-flags-2026-06-11` | `chore/l3-56-pheno-feature-flags-2026-06-11` | `1493b8c86c` | none |
| 16 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-58-pheno-ci-templates-2026-06-11` | `chore/l3-58-pheno-ci-templates-2026-06-11` | `a50a1d747e` | none |
| 17 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-60-pheno-secret-scan-2026-06-11` | `chore/l3-60-pheno-secret-scan-2026-06-11` | `dfce392add` | none |
| 18 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l4-66-pheno-port-adapter-2026-06-11` | `chore/l4-66-pheno-port-adapter-2026-06-11` | `08a68f16` | `Justfile` (M), `pheno-port-adapter/src/lib.rs` (M) |
| 19 | `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l4-68-pheno-context-2026-06-11` | `chore/l4-68-pheno-context-2026-06-11` | `afacf891` | `justfile` (M) |
| 20 | `/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint-wtrees/security-workflows` | `main` | `52dfc7aa` | `Justfile` (M) |

*Note:* `chore/l3-59-pheno-async-trait-migration-2026-06-11` is merged into `main` but its worktree directory is already absent from the filesystem and from the worktree registry (`git worktree list` no longer shows it). It is therefore not listed above.

---

## (b) Safe to Remove (Merged / Stale Branches)

| Worktree Path | Reason | Risk |
|---------------|--------|------|
| `/Users/kooshapari/CodeProjects/Phenotype/.claude/worktrees/FocalPoint-housekeeping` | Detached HEAD with no associated branch; no uncommitted changes; stale housekeeping worktree. | **None** — no changes to lose. |

---

## (c) Must Be Preserved (Active L3–L5 Branches and Other Live Work)

| Worktree Path | Branch | Reason for Preservation |
|---------------|--------|------------------------|
| `/Users/kooshapari/CodeProjects/Phenotype/repos` | `chore/l5-89-worktree-collapse-2026-06-11` | **Current task branch** (this plan). |
| `/private/tmp/v20-publisher-wt-l3-57` | `chore/l3-57-pheno-plugin-registry-2026-06-11` | **Active L3** — unmerged, ahead of main, has uncommitted changes. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-44-byteport-cov-2026-06-11` | `chore/l3-44-byteport-cov-2026-06-11` | **Active L3** — unmerged, ahead of main. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-48-pheno-config-2026-06-11` | `chore/l3-48-pheno-config-2026-06-11` | **Active L3** — unmerged, ahead of main, has uncommitted changes (`pheno-config/src/lib.rs`). |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-49-pheno-otel-2026-06-11` | `chore/l3-49-pheno-otel-2026-06-11` | **Active L3** — unmerged, ahead of main. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-50-pheno-cli-base-2026-06-11` | `chore/l3-50-pheno-cli-base-2026-06-11` | **Active L3** — unmerged, ahead of main. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-51-pheno-fastapi-base-2026-06-11` | `chore/l3-51-pheno-fastapi-base-2026-06-11` | **Active L3** — unmerged, ahead of main. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-52-pheno-go-ctxkit-2026-06-11` | `chore/l3-52-pheno-go-ctxkit-2026-06-11` | **Active L3** — unmerged, ahead of main, tracked on origin. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-53-pheno-zod-pydantic-2026-06-11` | `chore/l3-53-pheno-zod-pydantic-2026-06-11` | **Active L3** — unmerged, ahead of main, tracked on origin. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-55-pheno-ssot-template-2026-06-11` | `chore/l3-55-pheno-ssot-template-2026-06-11` | **Active L3** — unmerged, ahead of main. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-56-pheno-flags-2026-06-11` | `chore/l3-56-pheno-feature-flags-2026-06-11` | **Active L3** — unmerged, ahead of main. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-58-pheno-ci-templates-2026-06-11` | `chore/l3-58-pheno-ci-templates-2026-06-11` | **Active L3** — unmerged, ahead of main. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l3-60-pheno-secret-scan-2026-06-11` | `chore/l3-60-pheno-secret-scan-2026-06-11` | **Active L3** — unmerged, ahead of main. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l4-66-pheno-port-adapter-2026-06-11` | `chore/l4-66-pheno-port-adapter-2026-06-11` | **Active L4** — unmerged, ahead of main, has uncommitted changes (`pheno-port-adapter/src/lib.rs`). |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l4-68-pheno-context-2026-06-11` | `chore/l4-68-pheno-context-2026-06-11` | **Active L4** — unmerged, ahead of main, has uncommitted changes. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/audit-v20-2026-06-12` | `audit/crossrepo-canonical-merge-2026-06-12` | **Active audit** — unmerged, tracked on origin, has uncommitted changes. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/l2-37-branch-protection-2026-06-11` | `chore/l2-37-branch-protection-2026-06-11` | **Active L2** — unmerged, ahead of main, has uncommitted changes. |
| `/Users/kooshapari/CodeProjects/Phenotype/.claude/worktrees/phenoPlugins-pr82` | `hygiene/preserve-changes` | **Live branch** — unmerged, tracked on origin (`origin/hygiene/preserve-changes`), serves as a preservation worktree for PR82. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint-wtrees/security-workflows` | `main` | **Has uncommitted changes** (`Justfile` modified); while not an L3–L5 branch, removing it would discard active work. |

---

## (d) `git worktree remove` Commands (Safe-to-Remove Only)

The following commands are ready to execute **only after** confirming the worktree is truly stale and no downstream automation depends on its path.

```bash
# 1. FocalPoint housekeeping — detached HEAD, no changes, stale
#    (Path: /Users/kooshapari/CodeProjects/Phenotype/.claude/worktrees/FocalPoint-housekeeping)
git worktree remove /Users/kooshapari/CodeProjects/Phenotype/.claude/worktrees/FocalPoint-housekeeping

# Optional: prune any orphaned worktree entries
# git worktree prune
```

> **DO NOT EXECUTE.** This plan is documentation-only. The actual removal of worktrees is out of scope for L5 #89 and must be performed in a separate, explicitly authorized operation.

---

## Summary

- **Total worktrees audited:** 20 (plus 1 already-absent merged worktree `l3-59`).
- **Safe to remove:** 1 (`FocalPoint-housekeeping` — detached HEAD, stale, no changes).
- **Must be preserved:** 19 (active L2–L5 tasks, active audits, live hygiene branches, and any worktree with uncommitted changes).
- **Merged branches already collapsed:** `chore/l3-59-pheno-async-trait-migration-2026-06-11` (worktree directory missing, no longer in registry).
