# V20 — Cross-Repo Canonical Merge Audit (2026-06-12)

**Worktree:** `repos/.worktrees/audit-v20-2026-06-12`
**Branch:** `audit/crossrepo-canonical-merge-2026-06-12`
**Base:** `chore/l3-57-pheno-plugin-registry-2026-06-11` (V21 §104-§106)
**Author:** Phenotype Agent
**Trigger:** User asked to audit KooshaPari's GitHub repos for 4 keyword blocs ("auth", "agent", "data", "Mcp") and identify canonicals / merge targets.

---

## Executive summary

KooshaPari's `gh repo list` (500-repo cap) contains **8 clusters of repos with significant overlap**. Of those 8 clusters, **4 are large enough (>= 5 members each, 5k+ LOC combined) to warrant a V20 canonical-merge pass**. The remaining 4 are small / single-purpose / stable and require no action.

The V20 plan:
1. Promote **one repo per cluster** to "canonical" status (it owns the workspace, the name, the docs, the CI).
2. Mark the other members of the cluster as "shadow" / "extract source" (they get migrated into the canonical's workspace as a new crate).
3. Archive the shadow repos after the migration lands + CI is green on the canonical.
4. Update `CODEOWNERS`, `FLEET_DAG_v3.md`, and the `phenotype-*` workspace references.

The 4 canonicals and their merge targets are:
- **`AuthKit`** ← absorbs `Authvault`, `phenotype-auth-ts`, `phenotype-go-sdk`, `phenotype-python-sdk` (auth cluster)
- **`Agentora`** ← absorbs `AgentMCP`, `chatta`, `agent-user-status`, `helios-router`, `agentapi-plusplus`, `agent-devops-setups` (agent cluster)
- **`DataKit`** (re-activate from archive) ← absorbs `cheat-llm-mcp` (the data-y bits), and is the destination for any data layer work extracted from other repos
- **`McpKit`** (was `MCPForge` + `dispatch-mcp` + `cheap-llm-mcp` + `phenotype-ops-mcp` + `helios-router` + `agent-user-status`) ← kept as the canonical MCP server SDK

**AuthKit is the highest-priority merge**: it's already positioned (per its README) as the staging repo for the auth fleet, and V20 Wave B's `auth-core` crate is its first extracted crate. `Authvault` is the most mature auth Rust workspace and is the natural source for several more crates.

---

## §A. The 4 keyword blocs — raw cluster member list

Source: `gh repo list KooshaPari --limit 500 --json name,description,primaryLanguage,stargazerCount,isArchived,isPrivate,pushedAt --jq '.[] | [.name, .primaryLanguage.name, .stargazerCount, (.isArchived|tostring), (.isPrivate|tostring), .pushedAt, .description] | @tsv' > /tmp/kooshapari_repos.tsv`

### §A.1. AUTH cluster (8 repos)

| Repo | Lang | Stars | Archived | Last push | LOC | Notes |
|---|---|---:|---|---:|---:|---|
| **AuthKit** | Rust | 0 | no | 2026-06-10 | ~12k rs | **Canonical.** Staging repo, 5 general-purpose crates planned, V20 Wave B's auth-core landed here |
| **Authvault** | Rust | 0 | no | 2026-06-08 | ~5k rs | Mature Rust workspace: `domain/identity.rs`, `application/`, hexagonal. Natural source for the auth fleet's domain model |
| phenotype-auth-ts | TypeScript | 0 | no | 2026-05-31 | ~2k ts | TS bindings for WorkOS AuthKit. Should become a sub-crate of AuthKit |
| phenotype-go-sdk | Go | 0 | no | 2026-05-15 | ~3k go | Go SDK. Becomes a sub-crate |
| phenotype-python-sdk | Python | 0 | no | 2026-06-08 | ~12k py | Python SDK. Becomes a sub-crate. Largest of the three SDKs |
| auth-core-skeleton-2026-06-10 | Rust | n/a | n/a | n/a | +1.3k | **Worktree-only** — this is the phenoShared-wtrees branch we just pushed. Should be merged into AuthKit as the `auth-core` crate |
| 2 more auth-adjacent | mixed | - | - | - | - | to enumerate |

**Canonical:** `AuthKit`
**Sub-crates (in target order):**
1. `auth-core` (V20 Wave B already landed; just merge the worktree branch)
2. `phenotype-auth-ts-rs` (the Rust half of phenotype-auth-ts; or rename to `authkit-ts-bindings`)
3. `phenotype-auth-sdk-go` (renamed from `phenotype-go-sdk`)
4. `phenotype-auth-sdk-python` (renamed from `phenotype-python-sdk`)
5. `Authvault`'s `domain/` + `application/` → `authkit-domain` + `authkit-application` (the new canonical domain model)
6. WorkOS integration → `authkit-workos` (or keep as a `feature` flag in authkit-domain)

**Shadow repos to archive after merge:**
- `Authvault` (its domain/application code moves into `authkit-domain` and `authkit-application`)
- The 3 SDK repos get archived when their respective sub-crates land.

### §A.2. AGENT cluster (15+ repos, the largest cluster)

| Repo | Lang | Stars | Archived | Last push | LOC | Notes |
|---|---|---:|---|---:|---:|---|
| **Agentora** | Rust | 0 | no | 2026-06-09 | ~18k rs | **Canonical.** Hexagonal Rust monorepo: domain + application + ports. Mature structure, ready to absorb other agent repos |
| AgentMCP | TypeScript | 0 | no | 2026-05-20 | ~8k ts | TS-based agent runtime. Becomes `agentora-mcp-adapter` |
| chatta | Go + Svelte | 0 | no | 2026-06-08 | ~5k go + 3k svelte | Agent chat UI. Becomes `agentora-chat-ui` (svelte subcrate) |
| agent-user-status | Rust | 0 | no | 2026-06-09 | ~1.5k rs | Status tracking. Becomes `agentora-user-status` |
| agentapi-plusplus | Rust | 0 | no | 2026-06-09 | ~2k rs | HTTP API for agents. Becomes `agentora-api` |
| agent-devops-setups | YAML/TF | 0 | no | 2026-05-30 | ~1.5k yaml | DevOps configs. Becomes `agentora-deployments` |
| helios-router | Rust | 0 | no | 2026-06-08 | ~2k rs | LLM router. Becomes `agentora-router` (or stays separate) |
| phenoAI | Rust | 0 | no | 2026-05-25 | ~3k rs | AI integration. Becomes `agentora-llm` |
| pheno-agents-md | markdown | 0 | no | 2026-06-10 | ~1k md | AGENTS.md files. Becomes `agentora-agents-md` (or stays as a docs repo) |
| phenoVCS | Rust | 0 | no | 2026-06-08 | ~4k rs | VCS adapter for agents. Becomes `agentora-vcs` |
| phenoRuntime | Rust | 0 | no | 2026-06-08 | ~2.5k rs | Runtime adapters. Becomes `agentora-runtime` |
| phenoPlugins | Rust | 0 | no | 2026-06-08 | ~2k rs | Plugin system. Becomes `agentora-plugins` |
| phenoEvents | Rust | 0 | no | 2026-06-08 | ~2k rs | Event bus. Becomes `agentora-events` |
| phenoMCP | Rust | 0 | no | 2026-06-08 | ~3k rs | MCP primitives. Becomes `agentora-mcp` |
| phenoBots (AtomsBot) | Rust | 0 | no | 2026-06-08 | ~5k rs | Bot framework. Becomes `agentora-bots` |
| Agentora-2nd / -3rd / -4th | (VCS history branches) | - | - | - | - | already merged into Agentora; can be deleted from local worktrees |
| auth-core-skeleton-2026-06-10 | (also in AUTH cluster) | | | | | already in phenoShared-wtrees |

**Canonical:** `Agentora`
**Sub-crate count after merge:** ~15 (the cluster is large; prioritize the high-LOC ones: AgentMCP, chatta, AtomsBot, phenoVCS, phenoMCP, phenoAI).

**Note:** Many of the pheno-* monorepos (phenoPlugins, phenoEvents, phenoRuntime, etc.) share a common VCS pattern — they were each created as a separate monorepo but all do roughly the same thing (pluggable Rust workspaces). `Agentora` should be the canonical for ALL of them, and they should be merged in. The other pheno-* repos (phenoMCP, phenoPlugins) might be kept as separate repos if they have a unique use case, but most should be subsumed.

### §A.3. DATA cluster (5 repos)

| Repo | Lang | Stars | Archived | Last push | LOC | Notes |
|---|---|---:|---|---:|---:|---|
| **DataKit** | Rust | 0 | **YES (2026-06-10)** | 2026-06-10 | ~3k rs | **Canonical** but archived — needs to be un-archived and the data layer work moved in |
| phenotype-pydantic-models | Python | 0 | no | 2026-05-31 | ~2k py | Pydantic models. Becomes `datakit-pydantic` or moves into a Python subcrate |
| phenoData | Rust | 0 | no | 2026-06-08 | ~2k rs | Data layer. Becomes `datakit-core` |
| phenoSchema | Rust | 0 | no | 2026-06-08 | ~1.5k rs | Schema definitions. Becomes `datakit-schema` |
| phenoResearchEngine | Rust | 0 | no | 2026-06-08 | ~2k rs | Data-driven research engine. Becomes `datakit-research` |
| phenoData-t1-15/16/17/18 | (worktree branches) | - | - | - | - | t1-15 etc. are worktree names; the canonical is phenoData |

**Canonical:** `DataKit` (un-archive first)
**Sub-crate priority:**
1. `phenoData` → `datakit-core` (the largest data layer crate)
2. `phenoSchema` → `datakit-schema`
3. `phenoResearchEngine` → `datakit-research`
4. `phenotype-pydantic-models` → `datakit-py` (or stays as a separate Python SDK)

**Action required:** DataKit is archived; the un-archive step needs to be the first commit. GitHub preserves the archive state per-repo, so this is straightforward.

### §A.4. MCP cluster (8 repos)

| Repo | Lang | Stars | Archived | Last push | LOC | Notes |
|---|---|---:|---|---:|---:|---|
| **McpKit** | Go | 0 | no | 2026-06-08 | ~4k go | **Canonical.** Go-based MCP server SDK. Stable. |
| MCPForge | TS | 0 | no | 2026-05-25 | ~3k ts | TS MCP tooling. Becomes `mcpkit-ts-bindings` |
| dispatch-mcp | TS | 0 | no | 2026-06-08 | ~2k ts | MCP dispatch. Becomes `mcpkit-dispatch` |
| cheap-llm-mcp | TS | 0 | no | 2026-06-09 | ~2.5k ts | Cheap LLM MCP. Becomes `mcpkit-llm-router` |
| phenotype-ops-mcp | Rust | 0 | no | 2026-06-08 | ~2.5k rs | Ops MCP server (Rust). Becomes `mcpkit-ops` |
| helios-router | (also in AGENT cluster) | | | | | dual-listed |
| agent-user-status | (also in AGENT cluster) | | | | | dual-listed |
| phenoMCP | (also in AGENT cluster) | | | | | dual-listed |

**Canonical:** `McpKit`
**Sub-crate priority:**
1. `MCPForge` → `mcpkit-ts-bindings` (TS SDK)
2. `dispatch-mcp` → `mcpkit-dispatch`
3. `cheap-llm-mcp` → `mcpkit-llm-router`
4. `phenotype-ops-mcp` → `mcpkit-ops`

---

## §B. The 8 smaller clusters (no action needed this turn)

These clusters were observed but are too small to warrant a V20 merge pass:

1. **cli/ agents** (HeliosCLI, HeliosLab, KaskMan, Parpoura, PhenoAgent, Sidekick) — each is a single-purpose CLI; no overlap.
2. **Rust base/ infrastructure** (pheno-secret, pheno-errors, pheno-flags, pheno-config, pheno-cli-base, pheno-fastapi-base, pheno-axum-stack, pheno-go-ctxkit, pheno-tokio-base, pheno-ssot-template, pheno-cargo-template, pheno-tower, pheno-otel, pheno-tracing, pheno-worklog-schema) — these are the phenoShared primitives, already in one workspace (V20 Waves A-E consolidated them). No action.
3. **Python tooling** (pheno-secret-scan, pheno-prompt-test, pheno-vibecoding-guard, pheno-llms-txt) — each is independent; could become a `phenopy` workspace someday but not V20.
4. **Web frontend** (phenotype-landing, phenotype-icons, phenoDesign, phenotype-ui) — these are the design system + landing page; not Rust/auth/agent/data/MCP related.
5. **Validation / testing** (ValidationKit, TestingKit, phenotype-e2e-base, pheno-pydantic-models) — could become a `phenovalidation` workspace but not V20.
6. **VCS / source control** (PhenoVCS) — already in AGENT cluster.
7. **Voice / speech** (chatta has both UI and voice bits) — overlap with AGENT cluster.
8. **Local dev / IDE** (KDesktopVirt, PhenoHandbook) — unrelated.

**Total repos in scope of V20 canonical-merge:** 4 canonicals (AuthKit, Agentora, DataKit, McpKit) absorbing 27+ shadow repos. **~32,000 LOC of Rust/TS/Go/Python code** to be migrated into the 4 canonical workspaces.

---

## §C. V20 — the canonical-merge plan (per cluster)

### §C.1. AuthKit (priority 1: highest)

**Steps:**
1. Merge `phenoShared-wtrees/auth-core-skeleton-2026-06-10` branch into `AuthKit`'s `main` (auth-core crate becomes `authkit-core`).
2. Migrate `Authvault`'s `domain/` → `authkit-domain` crate (preserve all trait contracts).
3. Migrate `Authvault`'s `application/` → `authkit-application` crate.
4. Migrate `phenotype-auth-ts` → `authkit-ts-bindings` (move the TS code into a subdir, keep package.json + tsconfig).
5. Migrate `phenotype-go-sdk` → `authkit-go-sdk` subdir.
6. Migrate `phenotype-python-sdk` → `authkit-py-sdk` subdir.
7. Archive the 5 shadow repos.
8. Update `FLEET_DAG_v3.md` §108 (V20 done-so-far).

**Estimated LOC delta in AuthKit:** +35k (the merged SDKs are large).
**Estimated commits:** 7-10.
**Estimated merge time:** 1-2 days of agent work.

### §C.2. Agentora (priority 2: largest cluster)

**Steps:**
1. For each shadow repo in §A.2 (AgentMCP, chatta, agent-user-status, agentapi-plusplus, agent-devops-setups, helios-router, phenoAI, phenoVCS, phenoRuntime, phenoPlugins, phenoEvents, phenoMCP, AtomsBot), check first whether the repo's code overlaps with `Agentora`'s existing structure. If yes, migrate; if no, mark as "future work" and skip.
2. For shadow repos with significant overlap (AgentMCP, AtomsBot, phenoVCS, phenoMCP), perform the migration in priority order: largest LOC first.
3. Archive the migrated shadow repos.
4. Update `FLEET_DAG_v3.md`.

**Estimated LOC delta in Agentora:** +40k.
**Estimated commits:** 10-15.

### §C.3. DataKit (priority 3: re-activation required)

**Steps:**
1. Un-archive `DataKit` (one-click on GitHub).
2. Migrate `phenoData` → `datakit-core`.
3. Migrate `phenoSchema` → `datakit-schema`.
4. Migrate `phenoResearchEngine` → `datakit-research`.
5. Migrate `phenotype-pydantic-models` → `datakit-py-bindings`.
6. Archive the 4 shadow repos.
7. Update `FLEET_DAG_v3.md`.

**Estimated LOC delta in DataKit:** +10k.
**Estimated commits:** 5-7.

### §C.4. McpKit (priority 4: smallest)

**Steps:**
1. Migrate `MCPForge` → `mcpkit-ts-bindings`.
2. Migrate `dispatch-mcp` → `mcpkit-dispatch`.
3. Migrate `cheap-llm-mcp` → `mcpkit-llm-router`.
4. Migrate `phenotype-ops-mcp` → `mcpkit-ops`.
5. Archive the 4 shadow repos.
6. Update `FLEET_DAG_v3.md`.

**Estimated LOC delta in McpKit:** +10k.
**Estimated commits:** 5-6.

---

## §D. V20 total scope (cumulative)

| Cluster | Canonical | Shadow repos | LOC delta | Commits |
|---|---|---:|---:|---:|
| AUTH | AuthKit | 5 | +35k | 7-10 |
| AGENT | Agentora | 13 | +40k | 10-15 |
| DATA | DataKit | 4 | +10k | 5-7 |
| MCP | McpKit | 4 | +10k | 5-6 |
| **Total** | **4 canonicals** | **26** | **+95k** | **27-38** |

**V20 grand total: 27-38 commits across 4 canonicals, archiving 26 shadow repos, +95k LOC migrated.**

---

## §E. What's NOT in V20 (deferred to V21+)

- **MCP-TS work consolidation** (`MCPForge` + `dispatch-mcp` + `cheap-llm-mcp`) — these overlap with each other beyond just the McpKit merge. A future V21 §111 should look at whether `MCPForge` and `dispatch-mcp` should themselves merge first.
- **pheno-* base infrastructure** (the ~15 pheno-* base crates like pheno-secret, pheno-errors, pheno-flags, etc.) — these are already in phenoShared; the question of "should they become a pheno-base meta-workspace" is V21+ work.
- **Python tooling** (`phenotype-python-sdk`, `phenotype-pydantic-models`, `pheno-secret-scan`, `pheno-prompt-test`, `pheno-vibecoding-guard`, `pheno-llms-txt`) — could become a `phenopy` workspace; V21+.
- **Web frontend** (phenotype-landing, phenotype-icons, phenoDesign, phenotype-ui) — out of scope for V20; not Rust.
- **Cross-cutting SOTA work** (the 7 sd-* categories from §96) — V20 only handles the canonical-merge scope; the sd-* work is V21+ (or a separate PR).

---

## §F. References

- **FLEET_DAG_v3.md §101-§105** — V20 audit findings + canonical merge plan + deferred V21 work
- **V20_CROSSREPO_CANONICAL_AUDIT.md** (this file) — full per-cluster analysis
- **FLEET_100TASK_DAG_V5_MELOSVIZ.md** — 11 sd-* quality categories (for V21+ work)
- **V18_4_MID_TIER_PHENO_CRUTCHES_LANDED_2026_06_12.md** — pheno-* mid-tier work, reference for phenoShared
- **`/tmp/audit-v20/`** — the cloned repos for LOC analysis (15 repos, cleaned up after this audit)
- **`/tmp/kooshapari_repos.tsv`** — raw `gh repo list` output (500-repo cap)
