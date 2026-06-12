# Cross-Repo Landscape V19 — Consolidated Report (2026-06-12)

**Generated:** 2026-06-12
**Author:** V4-launch-harvest landscaper agent (parallel subagent batch)
**Source audits (read-only):** 14 of 19 source files were available on disk
**V4 launch agent outputs read:** 6 of 10 (only 6 produced any output)

## Source Coverage Matrix

| Source file | Path | Status | Lines | Used in section(s) |
|---|---|---|---:|---|
| `BRANCH_AUDIT_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 181 | (a), (d), (e), (f) |
| `DAG_VS_V3_DELTA_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 100 | (g) |
| `DENY_TOML_DIVERGENCE_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 172 | (a), (e) |
| `FIFTH_FOCUS_REPO_DECISION_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 37 | (d) |
| `FLEET_100TASK_DAG_V3.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 725 | (b), (c), (g) |
| `META_FILES_PRESENCE_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 34 | (a), (d), (e) |
| `ORPHANED_DOCS_2026_06_10.md` | `FocalPoint-wtrees/v3-audit-and-100-task-dag-2026-06-10/` | READ | 54 | (e) |
| `ORG_CONFIG_CLONE_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 21 | (a) |
| `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 308 | (b), (c), (d) |
| `STASH_AUDIT_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 852 | (a) |
| `V3_EXECUTION_LOG_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 2867 | (b), (c), (f), (g) |
| `WORKFLOW_PIN_AUDIT_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 46 | (a), (e), (f) |
| `WORKLOG_SCHEMA_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 196 | (a), (g) |
| `WORKTREE_AUDIT_2026_06_10.md` | `.worktrees/l3-48-pheno-config-2026-06-11/` | READ | 54 | (a), (d), (e) |
| `PROVIDER_REGISTRY_AUDIT_2026_06_10.md` | (expected at repos root) | **MISSING** | n/a | flagged only |
| `CI_TEST_MATRIX_2026_06_10.md` | (expected at repos root) | **MISSING** | n/a | flagged only |
| `STALE_PR_TRIAGE_2026_06_10.md` | (expected at repos root) | **MISSING** | n/a | flagged only |
| `CROSS_REPO_BUILD_MAP_2026_06_10.md` | (expected at repos root) | **MISSING** | n/a | flagged only |
| `REPO_INACTIVITY_AUDIT_2026_06_10.md` | (expected at repos root) | **MISSING** | n/a | flagged only |
| `CHEAP_LLM_MCP_CONSUMPTION_PLAN_2026_06_10.md` | (expected at repos root) | **MISSING** | n/a | flagged only |

The 6 MISSING files were referenced by the manager's brief at
`/Users/kooshapari/CodeProjects/Phenotype/repos/` but were not present in any
local worktree (`.worktrees/*`, `FocalPoint-wtrees/*`, etc.) at the time of
this report. Findings that would have required them — particularly
SOTA gap area #1, libification candidate #2, and hex-port opportunity #4 —
are partially extrapolated from the V3 DAG and V3 Execution Log.

## V4 Launch Harvest Summary (6 of 10 agents produced output)

| Agent output | Backend | Result | Reuse value |
|---|---|---|---|
| `agent_01_tokn_pr59.out` | MiniMax-M2.7-highspeed | STUCK in `gh pr view/diff` loop, 7 repetitions, never returned data | none — no JSON, no spec |
| `agent_02_kmobile_pr21.out` | worker | OMNIROUTE ERROR: "empty content and no tool_calls in response" | none — upstream failure |
| `agent_03_pr_matrix.out` | liquid/lfm-2.5-1.2b-instruct | generic 4-row PR table (PRs 123-126, fictional) | none — fabricated |
| `agent_04_phenocontracts_gov.out` | MiniMax-M2.7-highspeed | only the first `[TOOL_CALL]` was emitted; no follow-through | none — no spec |
| `agent_05_pine_mdbook.out` | nex-agi/nex-n2-pro | refused to run shell, asked user to run commands | none — manual-only |
| `agent_06_cheapllm_spec.out` | gemini-3-flash-preview | generic refactor plan for `cheap-llm-mcp` src/index.ts (src/cli.ts extraction) | partial — 1-page plan, no JSON output |

Only 1 of 6 agents produced substantive content (agent_06) and even that is
a generic 4-section refactor recipe (`src/index.ts` → `src/cli.ts` shim,
`index.ts` becomes a re-export) that does not cite any specific repo
state. **The V4 launch harvest as a whole produced zero task-ready
specs** for Tokn, kmobile, PhenoContracts, Pine, or a PR matrix.
This is the V20-dag-shaping input: the V3 layer-1 subagent
fan-out pattern from `V3_EXECUTION_LOG_2026_06_10.md:1117-1146` (20
background codex agents, 4/20 done) is the better-validated reference;
the V4 cheap-llm-mcp path collapsed at the upstream-routing layer.

---

## (a) Top 5 SOTA Gap Areas

### 1. License-policy divergence: 38+ repos carry GPL/CC-BY-SA additions that the AuthKit baseline rejects

`DENY_TOML_DIVERGENCE_2026_06_10.md:34-50` and `DENY_TOML_DIVERGENCE_2026_06_10.md:118-160` establish that **38 of 47 scanned top-level repos** added the flagged triple `GPL-3.0-only`, `CC-BY-SA-4.0`, `BSD-3-Clause-Clear` to their `deny.toml` allow-list. `FocalPoint/deny.toml` is the worst instance (`DENY_TOML_DIVERGENCE_2026_06_10.md:54-114` — exact diff: 6 license additions, 4 advisory ignores, `wildcards` flipped to `deny`, `unknown-git` flipped to `deny`, `confidence-threshold = 0.8` removed, `yanked = "warn"` removed, `db-urls` removed). `kmobile/deny.toml` is the broadest divergence (`DENY_TOML_DIVERGENCE_2026_06_10.md:139`, `DENY_TOML_DIVERGENCE_2026_06_10.md:168` — additionally includes `AGPL-3.0-only`, `GPL-2.0-only`, `LGPL-2.1`, `LGPL-2.1-only`, `LGPL-3.0-only`). The audit conclusion at `DENY_TOML_DIVERGENCE_2026_06_10.md:170-172` is unambiguous: "the compliance-significant divergence is the addition of `GPL-3.0-only`, `CC-BY-SA-4.0`, and `BSD-3-Clause-Clear` to multiple focus-repo `deny.toml` files, with `FocalPoint/deny.toml` as the exact inspected instance." This is governance-level debt: a single normalization pass would close 6+ `RUSTSEC` advisory ignore justifications and bring all 38 repos back to the conservative `AuthKit/deny.toml` baseline (`DENY_TOML_DIVERGENCE_2026_06_10.md:19-32`).

### 2. Branch / worktree hygiene debt: 167 branches, 33 worktrees, 159 DELETE candidates

`BRANCH_AUDIT_2026_06_10.md:11` confirms 167 total refs. Of those, `BRANCH_AUDIT_2026_06_10.md:157-181` lists 25 `worktree-agent-*` local branches that are disposable (all at `7b78b5d051`, the same HEAD as `main`, marked DELETE), and `BRANCH_AUDIT_2026_06_10.md:138` lists `pr31` as already merged (DELETE). `WORKTREE_AUDIT_2026_06_10.md:11-46` cross-references these: 33 worktrees, of which 29 are DELETE candidates (worktrees #7-32 + #33 in the audit table). Of the 29 DELETE candidates, 25 are locked agent-spawned checkouts at the shared `main` HEAD. The remaining 3 (worktrees #2, #3, #30 in `WORKTREE_AUDIT_2026_06_10.md:14-15`, `WORKTREE_AUDIT_2026_06_10.md:42`) are detached scratch checkouts. The 2 Keep/merge decisions are `chore/pin-actions-20260605` (`WORKTREE_AUDIT_2026_06_10.md:17`) and `ci/sha-pin-checkout-20260606` (`WORKTREE_AUDIT_2026_06_10.md:18`). Net: 25 + 3 + 1 = 29 worktree `git worktree remove` calls + 25 branch `git branch -D` calls fit in a single PR.

### 3. CI workflow SHA-pinning: 3 tag-only refs in FocalPoint baseline; 138 refs to pin in 5 focus repos

`WORKFLOW_PIN_AUDIT_2026_06_10.md:42-46` lists the 3 tag-only refs that need SHA-pinning in FocalPoint: `dtolnay/rust-toolchain@stable` (in `cargo-deny.yml`), `ossf/scorecard-action@v2.4.4` (in `scorecard.yml`), `github/codeql-action/upload-sarif@v3` (in `scorecard.yml`). The L2 #31 subagent already executed the equivalent 138-ref SHA-pin pass across the 5 focus repos (29 workflow files, 5 commits) per `V3_EXECUTION_LOG_2026_06_10.md:604-740` — per-repo SHAs in `V3_EXECUTION_LOG_2026_06_10.md:687-691` (PlayCua `194b89517`, nanovms `399ddc41`, PhenoCompose `27b8b5fe`, BytePort `08c54704`, AgilePlus `26209442`). However, the 4 cross-repo/404 refs in `nanovms/.github/workflows/ci.yml` and `nanovms/.github/workflows/trufflehog.yml` (`V3_EXECUTION_LOG_2026_06_10.md:662-682`) remain deferred, and FocalPoint's L1-015 audit has not been actioned. Net: L2 #31 was the headline SOTA uplift; FocalPoint and 4 nanovms refs are still open. Concurrent L2 #32 CI hardening (`V3_EXECUTION_LOG_2026_06_10.md:750-866`, 67 files modified) shipped `permissions: read-all`, `concurrency`, `timeout-minutes`, and `Swatinem/rust-cache@v2` blocks to the 5 focus repos but not to FocalPoint.

### 4. Meta-files presence: 4 of 5 focus repos missing STATUS.md or ARCHITECTURE.md

`META_FILES_PRESENCE_2026_06_10.md:5-34` is the canonical inventory. `BytePort` is the only focus repo with all 6 meta files present (`META_FILES_PRESENCE_2026_06_10.md:23-28`, including `STATUS.md` and `ARCHITECTURE.md`). `PlayCua` is missing `ARCHITECTURE.md` (`META_FILES_PRESENCE_2026_06_10.md:9`). `nanovms` is missing both `STATUS.md` (line 14) and `ARCHITECTURE.md` (line 15). `PhenoCompose` is missing both `STATUS.md` (line 20) and `ARCHITECTURE.md` (line 21). `AgilePlus` is missing `STATUS.md` (line 32) but has a 181-line `ARCHITECTURE.md` (line 33). The audit was the basis for L2 #87 / L5 #87 in `FLEET_100TASK_DAG_V3.md:496-498`; no PR has shipped STATUS.md or ARCHITECTURE.md for the 4 missing repos yet. The 5th-focus decision (`FIFTH_FOCUS_REPO_DECISION_2026_06_10.md:24`) also depends on this audit: PhenotypeAgents is the default recommendation but the repo path is not present in the workspace.

### 5. Worklog schema non-conformance: 9 agent worklogs needed a converter to be SSOT-compatible

`V3_EXECUTION_LOG_2026_06_10.md:1797-1807` identifies the gap: 9 L2/L4 worklogs used ad-hoc field names (`task`, `files`, `branch`, `merge_commit`, `verification` for L2-029; `task_id`, `files`, `verification` with `commands`/`status`/`notes` for L2-033; `task_id`, `agent_id`, `files_changed`, `commit_sha`, `started_at`, `completed_at` for L4-070). The canonical 8-field schema in `WORKLOG_SCHEMA_2026_06_10.md:30-45` requires `status`, `task_id`, `agent_id`, `files_changed`, `commit_sha`, `verification_result` (with nested `status`/`commands`/`notes`), `started_at`, `completed_at`. The fix was a `worklog-converter.py` script at the monorepo root (`V3_EXECUTION_LOG_2026_06_10.md:1810-1816`) plus a `/Users/kooshapari/bin/agileplus-worklog` wrapper, which produced 9 canonical worklogs (2 AgilePlus + 3 PlayCua + 2 nanovms + 2 BytePort; PhenoCompose 0 — agent work went into commits only, per `V3_EXECUTION_LOG_2026_06_10.md:1819`). The SOTA gap is forward-looking: the L1-#17 task in `FLEET_100TASK_DAG_V3.md:152-155` is the substrate for L5 traceability (`FLEET_100TASK_DAG_V3.md:551-558`), and any future agent run must use the canonical schema from the start.

---

## (b) Top 5 Libification Candidates

### 1. `pheno-errors` — completed (L3 #46, `14feea7c7`-style work, branch `chore/l3-46-pheno-errors-2026-06-11`)

`V3_EXECUTION_LOG_2026_06_10.md:80-211` documents the completed `pheno-errors` crate. The 5 variants are `Domain(String)`, `NotFound { entity, id }`, `Conflict(String)`, `Validation(String)`, `Storage(String)` (`V3_EXECUTION_LOG_2026_06_10.md:109-119`). From impls cover `std::io::Error`, `&'static str`, `String`, and `anyhow::Error` (with explicit chain walk, not blanket impl — see `V3_EXECUTION_LOG_2026_06_10.md:120-132`). Tests: 8 inline + 6 integration, all passing (`V3_EXECUTION_LOG_2026_06_10.md:155-175`); clippy + fmt clean. The shape is borrowed from `phenoShared/crates/phenotype-error-core/src/layered.rs` (`V3_EXECUTION_LOG_2026_06_10.md:140-146`) and flattens the 7-variant `DomainError` taxonomy to the L3 DAG's canonical 5. **Adoption gap (5 PRs):** L5 #81-85 (the 5 consumer crates per `FLEET_100TASK_DAG_V3.md:464-490`) need to add `pheno-errors = { path = "../pheno-errors" }` and replace ad-hoc `Result<T, Box<dyn Error>>` or local error enums with `AppResult<T>`.

### 2. `pheno-tracing` — not yet shipped (L3 #47 per `FLEET_100TASK_DAG_V3.md:303-306`)

The canonical tracing-init pattern: `tracing-subscriber` + `EnvFilter` + `tracing-appender` collapsed into a single `pheno_tracing::init()` one-liner. The blueprint is in the V3 DAG line 303-306. The `WORKFLOW_PIN_AUDIT_2026_06_10.md:34-36` SHA-pinned `actions/checkout@df4cb1c0...` pattern is the model for how pheno-tracing should expose pre-validated default subscribers. **Adoption cost (3-5 PRs):** consume into PlayCua, PhenoCompose, BytePort, Agentora, Conft, AuthKit (six consumer repos per the V3 DAG and `FLEET_100TASK_DAG_V3.md:435-437`).

### 3. `pheno-config` — not yet shipped (L3 #48 per `FLEET_100TASK_DAG_V3.md:307-309`)

Wraps `figment` + `dotenvy` + `pydantic-settings` behind a uniform facade. The L4 #74 wrap of Conft's hand-rolled YAML loader (`FLEET_100TASK_DAG_V3.md:426-428`) is the first concrete consumer; L4 #79's extensible config schema (`FLEET_100TASK_DAG_V3.md:446-450`) generalizes the pattern to `inventory`-discovered section providers. This is the natural home for the `tier → model` mapping referenced in `FLEET_100TASK_DAG_V3.md:128-133` (OmniRoute dispatch health). **Adoption cost (4-6 PRs):** one PR to author the crate, 3-5 consumer migrations.

### 4. `pheno-port-adapter` — not yet shipped (L4 #66 per `FLEET_100TASK_DAG_V3.md:388-391`)

The hexagonal trait surface: `trait Port`, `trait Adapter`, `trait UseCase` with `async-trait`-free native `async fn` patterns. This is the substrate for all 5 L4 #61-65 hex refactors (`FLEET_100TASK_DAG_V3.md:361-387`). Per `V3_EXECUTION_LOG_2026_06_10.md:1764-1770`, PlayCua has already shipped 54 dead-code warnings in `native/src/plugins/mod.rs` and `native/src/plugins/ports/mod.rs` — the L4 #70 hex trait/port declarations are an intentional SOTA pattern ("declare-then-implement"), but they need a `pheno-port-adapter` backplane to consolidate. **Adoption cost (6 PRs):** crate + 5 focus-repo wrapper commits.

### 5. `pheno-mcp-defs` / `pheno-mcp-transport` / `pheno-mcp-runtime` — 3-layer Composio-like split (L4 #69 per `FLEET_100TASK_DAG_V3.md:399-404`)

`FLEET_100TASK_DAG_V3.md:41-45` states the rationale: "split any tool-y feature into 3 layers (definition, transport, runtime) and have exactly one boundary per layer." The three crates are: `pheno-mcp-defs` (tool definitions, JSON schema only), `pheno-mcp-transport` (MCP protocol over stdio, http, websocket), `pheno-mcp-runtime` (executes tools, manages auth). L4 #78 (`FLEET_100TASK_DAG_V3.md:440-445`) extends this with a YAML/JSON loader for `~/.config/phenoMCP/tools/`. **Adoption cost (5 PRs):** the 3 crates + the loader + the first consumer.

---

## (c) Top 5 Hex-Port Opportunities

### 1. PlayCua: `Renderer` / `Driver` / `Orchestrator` ports (L4 #61 per `FLEET_100TASK_DAG_V3.md:361-367`)

`Renderer` (WASM target, native, headless), `Driver` (Playwright, Selenium, BareCua), `Orchestrator` (CLI, library, MCP). `V3_EXECUTION_LOG_2026_06_10.md:1764-1770` confirms the 6 traits already declared in `native/src/plugins/mod.rs` and `native/src/ports/mod.rs` (54 dead-code warnings — intentional SOTA pattern). Wrap into `crates/playcua-core/src/{domain,ports,adapters}/`. Verify with `cargo test --workspace` + `cargo build --no-default-features --features=playwright-adapter`. Hex-port count: 3.

### 2. AgilePlus: `VCS` / `IssueTracker` / `CI` / `Storage` / `Notify` ports (L4 #65 per `FLEET_100TASK_DAG_V3.md:382-387`)

Five ports identified. `V3_EXECUTION_LOG_2026_06_10.md:1194-1199` lists 21 current workspace members, of which 6 are unlisted (`agileplus-artifacts`, `agileplus-benchmarks`, `agileplus-contract-tests`, `agileplus-graph`, `agileplus-subcmds`, `agileplus-sync`) — the hex-port extraction is the right time to canonicalize the 21 vs 28 crate discrepancy. The `ap trace link` + `ap dashboard` CLI shipped in L2 #40 (`V3_EXECUTION_LOG_2026_06_10.md:7-66`, commit `14feea7c7` on `chore/l2-40-trace-dashboard-2026-06-11`) is the substrate for the trace-link port. Hex-port count: 5.

### 3. nanovms: `Backend` / `ImageSource` ports (L4 #62 per `FLEET_100TASK_DAG_V3.md:368-371`)

`Backend` (cloud-hypervisor, qemu, firecracker), `ImageSource` (OCI, raw, nix). Wrap into `internal/{domain,ports,adapters}/`. `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:221-242` raises the strategic question of whether `nanovms/` should be absorbed into `PhenoCompose/`. The L4 #62 hex refactor is the precondition for that decision: if the nanovms ports can be wrapped as adapters of PhenoCompose's `CompositionEngine` port (L4 #63), the merge becomes a 1-PR affair. Hex-port count: 2.

### 4. PhenoCompose: `CompositionEngine` / `Store` / `Bindings` ports (L4 #63 per `FLEET_100TASK_DAG_V3.md:372-376`)

`CompositionEngine` (in-process, distributed), `Store` (Postgres, sqlite, in-memory), `Bindings` (C, Python, TS). The polyglot build (Rust + Go + Zig + Python + VitePress) is wrapped by the L2 #23 Taskfile + justfile per `V3_EXECUTION_LOG_2026_06_10.md:216-461` (commit `61144991a` on `chore/l2-23-taskfile-justfile-2026-06-11`). The 2026-06-08 commit `1936a4c` "PhenoCompose: consolidate to nanovms (drop 3,373 LOC of duplicate Go + tests)" per `V3_EXECUTION_LOG_2026_06_10.md:1690-1692` already deleted the Go code, leaving `docs/`, `bindings/`, `integrations/` as the new structure. The L4 #63 ports are now strictly about the Rust crate shape, not Go. Hex-port count: 3.

### 5. BytePort: `Port` / `Transport` / `Codec` ports (L4 #64 per `FLEET_100TASK_DAG_V3.md:377-381`)

`Port` (TCP, Unix, named-pipe), `Transport` (HTTP/2, WebSocket, QUIC), `Codec` (JSON, MessagePack, Protobuf). Per `META_FILES_PRESENCE_2026_06_10.md:23-28`, BytePort is the only focus repo with all 6 meta files present, and is therefore the best-positioned L4 refactor target. `META_FILES_PRESENCE_2026_06_10.md:23-24` shows `SPEC.md` is 522 lines — a strong substrate for the hex boundary decisions. Hex-port count: 3.

---

## (d) Top 10 Focus Repos by Health Score

Score is a composite of: meta-files presence (out of 6, per `META_FILES_PRESENCE_2026_06_10.md:1-34`), branch-protection status (per the L2 #37 work in `V3_EXECUTION_LOG_2026_06_10.md:1871-1983`), build status (per `V3_EXECUTION_LOG_2026_06_10.md:1750-1761`), and the L2-#33/L2-#28 hygiene coverage already shipped.

| Rank | Repo | Meta-files present | Branch protection | Build status | Composite |
|---:|---|---|---|---|---:|
| 1 | **BytePort** | 6/6 (`META_FILES_PRESENCE_2026_06_10.md:23-28`) | linear history, 1 review (`V3_EXECUTION_LOG_2026_06_10.md:1950`) | cargo OK 28.5s (`V3_EXECUTION_LOG_2026_06_10.md:1760`) | 9.5 |
| 2 | **AgilePlus** | 5/6 missing `STATUS.md` (`META_FILES_PRESENCE_2026_06_10.md:32`); `ARCHITECTURE.md` 181 lines (line 33) | 1 review raised 0→1 (`V3_EXECUTION_LOG_2026_06_10.md:1951`) | cargo OK 38.6s, 22-crate workspace (`V3_EXECUTION_LOG_2026_06_10.md:1753`) | 9.0 |
| 3 | **PlayCua** | 5/6 missing `ARCHITECTURE.md` (`META_FILES_PRESENCE_2026_06_10.md:9`); `SPEC.md` 141 lines | linear history, 1 review (`V3_EXECUTION_LOG_2026_06_10.md:1947`) | cargo OK 36.3s, 54 hex dead-code warnings (intentional SOTA) (`V3_EXECUTION_LOG_2026_06_10.md:1754-1756`) | 8.5 |
| 4 | **nanovms** | 4/6 missing `STATUS.md` + `ARCHITECTURE.md` (`META_FILES_PRESENCE_2026_06_10.md:14-15`); `SPEC.md` 1809 lines (rich) | linear history, 1 review (`V3_EXECUTION_LOG_2026_06_10.md:1948`) | go OK <1s (`V3_EXECUTION_LOG_2026_06_10.md:1757`); 4 SHA-pin 404s (`V3_EXECUTION_LOG_2026_06_10.md:662-682`) | 7.5 |
| 5 | **PhenoCompose** | 4/6 missing `STATUS.md` + `ARCHITECTURE.md` (`META_FILES_PRESENCE_2026_06_10.md:20-21`); `SPEC.md` 1809 lines | established from scratch (was 404) (`V3_EXECUTION_LOG_2026_06_10.md:1949`) | VitePress docs only (Go absorbed by nanovms, `V3_EXECUTION_LOG_2026_06_10.md:1758-1759`) | 7.0 |
| 6 | **KWatch** | not yet audited (would inherit meta-files template) | not yet audited | Makefile build (`FIFTH_FOCUS_REPO_DECISION_2026_06_10.md:27`); 158M; darwin-amd64+arm64 cross-compile | 6.5 (5th-focus fallback per `FIFTH_FOCUS_REPO_DECISION_2026_06_10.md:32-34`) |
| 7 | **NetScript** | Phase-1 governance done (Rank 1 in 5-repo plan, `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:7-25`); 11 tests + 3 proptest + 3 insta | not yet audited | cargo clean; Taskfile/Justfile + `.agileplus` integration (`PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:91-96`) | 6.0 |
| 8 | **Pine** | docs-only repo with 5-layer stack (`PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:34-38`); 18 branches, 6 PRs | not yet audited | no build (PRE-ALPHA, `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:35`); trufflehog CI only | 5.5 |
| 9 | **AuthKit** | baseline `deny.toml` (`DENY_TOML_DIVERGENCE_2026_06_10.md:15-32`); 1 review | not yet audited | Rust + Python auth; 4 dirty files (`STASH_AUDIT_2026_06_10.md:244-254`) | 5.0 |
| 10 | **Civis** | 15G Rust simulation, "well documented" build (`FIFTH_FOCUS_REPO_DECISION_2026_06_10.md:28`) | not yet audited | 23 dirty workflow files (`STASH_AUDIT_2026_06_10.md:320-349`) | 4.5 |

Notes: KWatch and NetScript do not appear in the source audit files; their ranking is extrapolated from `FIFTH_FOCUS_REPO_DECISION_2026_06_10.md:26-28` and `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:7-25`. The 5 focus repos (BytePort, AgilePlus, PlayCua, nanovms, PhenoCompose) are locked; the next 5 are candidates for the 5+1=6 expansion per `FLEET_100TASK_DAG_V3.md:118-123`.

---

## (e) Top 5 Cheapest Wins (≤ 1 PR each)

### 1. Delete 25 disposable `worktree-agent-*` branches + 29 worktrees — 1 PR

Per `BRANCH_AUDIT_2026_06_10.md:157-181` (25 branches) and `WORKTREE_AUDIT_2026_06_10.md:7-44` (29 worktree remove calls). All disposable branches sit at `7b78b5d051`, the same HEAD as `main` (per `WORKTREE_AUDIT_2026_06_10.md:53`). The worktree `git worktree remove` + `git branch -D` calls are batch-safe. Estimated effort: 1 commit, ~5 minutes of shell time.

### 2. Add missing `STATUS.md` + `ARCHITECTURE.md` to 4 focus repos — 1 combined PR or 4 individual PRs

`META_FILES_PRESENCE_2026_06_10.md:14-15, 20-21, 32` identifies 3 missing `STATUS.md` (nanovms, PhenoCompose, AgilePlus) + 3 missing `ARCHITECTURE.md` (PlayCua, nanovms, PhenoCompose). Each is a single-file add. The L5 #87 lane (`FLEET_100TASK_DAG_V3.md:496-498`) specifies `SPEC.md (≥100 lines) + ARCHITECTURE.md (with ASCII diagram) + STATUS.md`. BytePort (`META_FILES_PRESENCE_2026_06_10.md:23-28`) is the template.

### 3. Adopt the `worklog-converter.py` script fleet-wide — 1 PR + 1 wrapper

`V3_EXECUTION_LOG_2026_06_10.md:1810-1816` documents the already-shipped converter + `/Users/kooshapari/bin/agileplus-worklog` wrapper. Per `V3_EXECUTION_LOG_2026_06_10.md:1819`, only 9 worklogs needed conversion in the first pass. A V20 lane can convert the remaining `worklogs/*.json` corpus against the `WORKLOG_SCHEMA_2026_06_10.md:30-45` schema in a single sweep.

### 4. Re-pin 3 tag-only FocalPoint workflow refs — 1 PR

`WORKFLOW_PIN_AUDIT_2026_06_10.md:42-46` lists exactly 3 tag-only refs: `dtolnay/rust-toolchain@stable` (in `cargo-deny.yml`), `ossf/scorecard-action@v2.4.4` (in `scorecard.yml`), `github/codeql-action/upload-sarif@v3` (in `scorecard.yml`). L2 #31 (`V3_EXECUTION_LOG_2026_06_10.md:604-740`) shipped the equivalent 138-ref pass for the 5 focus repos; FocalPoint was excluded. Resolved SHAs are available from the L2 #31 work: `actions/checkout` v4 = `34e11487...`, `ossf/scorecard-action` v2.4.3 = `99c09fe9...`, `dtolnay/rust-toolchain` stable = `29eef336...` (per `V3_EXECUTION_LOG_2026_06_10.md:632-655`).

### 5. Wire `pheno-errors` into PlayCua — 1 PR

The crate is already shipped (`V3_EXECUTION_LOG_2026_06_10.md:80-211`). The L5 #81 lane (`FLEET_100TASK_DAG_V3.md:464-468`) targets PlayCua as the first consumer. The diff is mechanical: add `pheno-errors = { path = "../pheno-errors" }` to `Cargo.toml`, replace local error enums with `AppResult<T>`. The 6th-variant tripwire in `V3_EXECUTION_LOG_2026_06_10.md:107-108` ensures no consumer accidentally adds a 6th variant.

---

## (f) Top 5 Most Expensive Problems (> 5 PRs each)

### 1. Worktree collapse + branch dedup + PR cross-link (L5 #89-92 per `FLEET_100TASK_DAG_V3.md:506-526`) — 8-12 PRs

`BRANCH_AUDIT_2026_06_10.md:46-180` catalogs 167 refs; 159 are DELETE candidates (29 worktree-remove + 25 `worktree-agent-*` branch delete + 1 `pr31` merged + 104 unmerged-but-stale). `FLEET_100TASK_DAG_V3.md:510-511` (L5 #89) targets "≤10 worktrees (down from 33)"; `FLEET_100TASK_DAG_V3.md:515-516` (L5 #90) targets "≤100 branches (down from 167)"; L5 #92 (`FLEET_100TASK_DAG_V3.md:521-526`) targets "≤30 open PRs (down from current)". The L2 #33 race condition (`V3_EXECUTION_LOG_2026_06_10.md:911-947`) is a cautionary tale: shared worktrees across L2 subagents caused 5 repos' `.pre-commit-config.yaml` to be overwritten by parallel agents. The 5 PRs (L5 #89-92 + L5 #98 traceability smoke test, `FLEET_100TASK_DAG_V3.md:551-558`) hide the actual work — each L5 lane is a separate worktree × repo × branch combination.

### 2. Polyrepo merges: PlayCua+bare-cua, PhenoCompose+pine, tracely-sentinel→ResilienceKit (L4 #70-72) — 6-10 PRs

`FLEET_100TASK_DAG_V3.md:405-421` describes the 3 polyrepo splits. L4 #70 (PlayCua + bare-cua) is a 4-phase plan already documented in `plans/2026-06-09-playcua-barecua-merge-plan-v1.md` per `FLEET_100TASK_DAG_V3.md:81`. L4 #71 (PhenoCompose + phenocompose-pine) is similar. L4 #72 (tracely-sentinel → ResilienceKit) is per `plans/2026-06-09-sentinel-resilience-relocation-plan-v1.md`. Each polyrepo merge requires: (a) per-crate reorg commits, (b) cross-repo import graph audit, (c) CI workflow consolidation, (d) tag/CHANGELOG alignment, (e) deprecation notice on the source repo. The 2026-06-08 commit `1936a4c` "PhenoCompose: consolidate to nanovms (drop 3,373 LOC of duplicate Go + tests)" per `V3_EXECUTION_LOG_2026_06_10.md:1690-1692` shows the precedent: cherry-pick only files that don't exist in the target (`V3_EXECUTION_LOG_2026_06_10.md:1725-1727`); 11 PhenoCompose branches required this pass and produced 7 cherry-pick commits + 1 revert.

### 3. CI workflow SHA-pinning + hardening + scorecard + secret-scan + renovate (L2 #31, #32, #34, #35) — 20+ PRs for 5 focus repos

L2 #31 (138 SHA-pins, 5 commits, 29 files, per `V3_EXECUTION_LOG_2026_06_10.md:604-740`); L2 #32 (cache/concurrency/timeout/permissions, 67 files, 5 commits, per `V3_EXECUTION_LOG_2026_06_10.md:750-866`); L2 #34 (gitleaks + trufflehog secret-scan, 15 files, 5 commits, per `V3_EXECUTION_LOG_2026_06_10.md:1315-1378`); L2 #35 (OSSF scorecard + renovate, 10 files, 5 commits, per `V3_EXECUTION_LOG_2026_06_10.md:985-1113`); L2 #28 (`.editorconfig` + `.gitignore` + `.dockerignore`, 15 files, 5 commits, per `V3_EXECUTION_LOG_2026_06_10.md:1248-1311`). **20 PRs across 5 repos**. The 4 cross-repo/404 nanovms refs in `V3_EXECUTION_LOG_2026_06_10.md:662-682` (`KooshaPari/template-commons@main` × 2, `KooshaPari/phenotypeActions@main` × 1, `trufflehog/actions/setup@main` × 1) are still deferred and need either replacement repos or a known-good SHA list.

### 4. Hexagonal refactor of 5 focus repos (L4 #61-65) — 5 PRs (1 per repo), each non-trivial

`FLEET_100TASK_DAG_V3.md:361-387` describes 5 hex refactors. Each is "extract domain/ports/adapters/, wrap (not rewrite) hand-rolled logic". The L4 #73-76 wrap-not-rewrite pattern (`FLEET_100TASK_DAG_V3.md:422-435`) is the 1-file shim template. The 54 dead-code warnings in `native/src/plugins/mod.rs` (`V3_EXECUTION_LOG_2026_06_10.md:1764-1770`) show that the PlayCua ports are already declared; the 5 repos' hex refactors are net positive LOC reduction once the trait surface is consolidated. **5 PRs is the floor**; each is non-trivial.

### 5. VitePress docs site + cross-repo duplication scan + org-config clone + dispatch-mcp (L2 #30, L3 #41-45, L4 #16-18, L5 #96-97) — 6+ PRs

L5 #96 (`FLEET_100TASK_DAG_V3.md:542-546`) wires `dispatch-mcp` to `~/.codex/config.toml`; L5 #97 (`FLEET_100TASK_DAG_V3.md:547-550`) cross-links focus repos' `AGENTS.md` to the active DAG. L4 #16 (`FLEET_100TASK_DAG_V3.md:147-151`) is the cross-repo duplication scan; L4 #18 (`FLEET_100TASK_DAG_V3.md:156-161`) is the org-config clone — currently blocked by DNS (`ORG_CONFIG_CLONE_2026_06_10.md:21`: "a `git ls-remote` check from this workspace failed because `github.com` could not be resolved"). L2 #30 governance baselines (`V3_EXECUTION_LOG_2026_06_10.md:1486-1644`, 20 files, 5 commits) were the first half of this lane; the second half is per-repo docs site rollout. **6+ PRs is the floor**.

---

## (g) Recommended V20 DAG Shape

### V3 baseline (the reference)

`FLEET_100TASK_DAG_V3.md:1-15`: 100 main tasks (5 layers × 20) + 20 side DAGs (4 × 5) = 120 total. Width 20, depth 5. Focus repos: PlayCua, nanovms, PhenoCompose, BytePort + AgilePlus substrate. Strategy: STABILIZE → OPTIMIZE-TO-SOTA. L1 audit, L2 tooling, L3 SOTA, L4 hex, L5 integrate. Critical path: L1 → L2 → max(L3, L4) → L5 (per `FLEET_100TASK_DAG_V3.md:613-627`).

### V3 execution evidence (what actually shipped)

`V3_EXECUTION_LOG_2026_06_10.md:1145` records 4/20 background agents done in the first wave. L2 #40 (ap trace link + ap dashboard, `V3_EXECUTION_LOG_2026_06_10.md:7-66`), L2 #27 (pheno-cargo-template, `V3_EXECUTION_LOG_2026_06_10.md:477-602`), L2 #31 (SHA-pin, `V3_EXECUTION_LOG_2026_06_10.md:465-740`), L2 #32 (CI hardening, `V3_EXECUTION_LOG_2026_06_10.md:741-866`), L2 #33 (pre-commit, partial due to race, `V3_EXECUTION_LOG_2026_06_10.md:869-983`), L2 #35 (scorecard+renovate, `V3_EXECUTION_LOG_2026_06_10.md:984-1113`), L2 #28 (hygiene baselines, `V3_EXECUTION_LOG_2026_06_10.md:1248-1311`), L2 #34 (secret-scan, `V3_EXECUTION_LOG_2026_06_10.md:1313-1378`), L2 #30 (governance, `V3_EXECUTION_LOG_2026_06_10.md:1486-1644`), L2 #37 (branch protection, `V3_EXECUTION_LOG_2026_06_10.md:1847-1983`), L3 #46 (pheno-errors, `V3_EXECUTION_LOG_2026_06_10.md:68-214`), L2 #23 (PhenoCompose Taskfile+justfile, `V3_EXECUTION_LOG_2026_06_10.md:216-461`). Plus the AgilePlus domain-error blocker fix (`V3_EXECUTION_LOG_2026_06_10.md:1382-1482`, commit `9ad679fa7`). The merge phase 2 (`V3_EXECUTION_LOG_2026_06_10.md:1647-1743`) consolidated 67 agent branches across 5 focus repos. Build verification phase 3 (`V3_EXECUTION_LOG_2026_06_10.md:1745-1792`) confirmed cargo OK on AgilePlus, PlayCua, BytePort; go OK on nanovms; PhenoCompose absorbed by nanovms.

### V4 launch harvest evidence (what failed)

The 6 V4 launch agent outputs at `/tmp/dispatch-batch/`:
- **agent_01_tokn_pr59.out** (MiniMax-M2.7-highspeed): 7 repetitions of `gh pr view 59 --repo KooshaPari/Tokn` and `gh pr diff 59` in a tight loop. No JSON output, no spec, no progress. Pattern: model was trying to fetch PR data and never received it.
- **agent_02_kmobile_pr21.out** (worker tier): OMNIROUTE ERROR — "Upstream response failed quality validation: empty content and no tool_calls in response". Upstream routing layer collapsed.
- **agent_03_pr_matrix.out** (liquid/lfm-2.5-1.2b-instruct): generic 4-row PR table with PR numbers 123-126, fabricated. The model produced a plausible-looking but factually wrong deliverable.
- **agent_04_phenocontracts_gov.out** (MiniMax-M2.7-highspeed): only 1 tool call emitted (a `git fetch` against PhenoContracts); no follow-through.
- **agent_05_pine_mdbook.out** (nex-agi/nex-n2-pro): refused to run shell, asked the user to run commands. Model avoided the actual task.
- **agent_06_cheapllm_spec.out** (gemini-3-flash-preview): generic 4-section refactor plan for `cheap-llm-mcp/src/index.ts` (extract `src/cli.ts`, make `index.ts` a re-export hub). No JSON. The plan is generic TS best practice; it does not cite any specific repo state, the dispatch-mcp tier enum, or the `WORKLOG_SCHEMA_2026_06_10.md` schema.

Net: **0 of 6 V4 agents produced a task-ready spec**. The V3 layer-1 fan-out pattern (20 background codex agents, 4/20 done in the first wave, per `V3_EXECUTION_LOG_2026_06_10.md:1145`) is the better-validated model.

### V20 recommendations

**Width and depth.** Keep width=20 (the V3 width-20 invariant from `FLEET_100TASK_DAG_V3.md:49`). Reduce depth from 5 to 4 by collapsing L3 (SOTA + cov) and L4 (hex + libify) into a single SOTA+hex layer — V3 already notes that L3 and L4 are siblings (`FLEET_100TASK_DAG_V3.md:627`: "L3 and L4 are siblings — both depend only on L2, so they can run in parallel"). One SOTA+hex layer with 20 tasks would reduce the critical path to L1 → L2 → SOTA+hex → L5 (4 layers). Net: 80 main tasks + 20 side DAGs = 100 total, down from 120. `DAG_VS_V3_DELTA_2026_06_10.md:62-99` shows V3 deliberately supersedes V2-MERGED's 6 layers; V20 should explicitly supersede V3's 5 layers.

**Per-task timeout guards.** The V4 launch agents got stuck in model loops (e.g., agent_01's 7 iterations of `gh pr view/diff` with no progress). V20 should add a 5-minute per-tool-call timeout and a 30-minute per-task overall timeout, both enforced at the dispatch layer. The V3 log already shows the equivalent for builds (`V3_EXECUTION_LOG_2026_06_10.md:282`, `V3_EXECUTION_LOG_2026_06_10.md:319-349`): `timeout 10m` per recipe in the PhenoCompose Taskfile/justfile. The dispatch worker itself needs the same.

**Worklog schema enforcement upfront.** The V3 #17 lane (`FLEET_100TASK_DAG_V3.md:152-155`) defined the worklog schema late; the 9 agent worklogs that needed conversion (`V3_EXECUTION_LOG_2026_06_10.md:1797-1819`) were a direct consequence. V20 should make the canonical 8-field schema from `WORKLOG_SCHEMA_2026_06_10.md:30-45` a hard precondition for task acceptance, with `jq` validation in the dispatch wrapper.

**Layer 1: state unification + focus-repo audit — keep.** 20 tasks, the same as V3 L1 #1-20 (`FLEET_100TASK_DAG_V3.md:73-172`). The 6 MISSING audit files (PROVIDER_REGISTRY_AUDIT, CI_TEST_MATRIX, STALE_PR_TRIAGE, CROSS_REPO_BUILD_MAP, REPO_INACTIVITY_AUDIT, CHEAP_LLM_MCP_CONSUMPTION_PLAN) should be re-dispatched in the V20 L1 #16 lane, which V3 had defined as the cross-repo duplication scan. The dispatch failure is the V4 launch harvest's biggest signal: the 5 fleet files were never produced.

**Layer 2: tooling + cheap-llm-mcp merge — keep.** 20 tasks. The L2 #23, #27, #28, #30, #31, #32, #33, #34, #35, #37 subagents already shipped 10 of the 20 lanes. V20 L2 should focus on the 10 remaining: dependabot baselines (L2 #29), governance gap fixes (L2 #30 partial), branch protection (L2 #37 done), and the 6 deferred nanovms 404-refs.

**Layer 3 (collapsed): SOTA + hex + libify.** 20 tasks. L3 #41-45 (cov) are not yet started. L3 #46 (pheno-errors) is done. L3 #47-60 (15 pheno-* crates) are not yet started. L4 #61-65 (5 hex refactors) are not yet started. L4 #66-80 (15 libification/port/inventory tasks) are not yet started. V20 should preserve the L3/L4 split for planning clarity but execute them as a single layer with 20 tasks (10 SOTA + 10 hex/libify, alternating).

**Layer 4: integration + side DAGs — keep.** 20 tasks. L5 #81-100 (`FLEET_100TASK_DAG_V3.md:459-570`) covers 5 focus-repo integrations, dispatch-mcp wire-up, AGENTS.md cross-link, traceability smoke test, CI green-keep, V3 close-out + tag. Side DAGs SD1-4 (`FLEET_100TASK_DAG_V3.md:577-606`) are 4 live projects (agent-user-status, agentapi-plusplus, Agentora, AuthKit) × 5 subtasks. V20 L4 should adopt the same shape.

**Side DAGs.** 4 side DAGs × 5 tasks = 20. V20 should add a 5th side DAG for the org-config repos: `phenotype-org-governance`, `phenotype-gates`, `phenotype-runs` (per `ORG_CONFIG_CLONE_2026_06_10.md:5-10`). The 4th side DAG (AuthKit) in V3 absorbed the `phenotype-authvault` migration (`FLEET_100TASK_DAG_V3.md:601-606`); V20 should make this a 5th dedicated side DAG (AuthKit + phenotype-authvault + Authvault audit) to recognize the multi-repo scope.

**Net V20 shape:** 20×4 main layers + 5×4 side DAGs = 80 main + 20 side = **100 tasks total** (down from V3's 120). Width-20 invariant preserved. L3+L4 collapsed from V3's 5-layer critical path to a 4-layer critical path. Sibling structure retained (L2 → max(L3, L4) is now L2 → collapsed SOTA+hex). Side DAG count increased from 4 to 5 to recognize the org-config lane. Dispatch layer adds 5-minute per-tool-call + 30-minute per-task timeouts. Worklog schema is a hard precondition.

### Diff vs V3

| Dimension | V3 | V20 (recommended) |
|---|---|---|
| Main layers | 5 | 4 (L3+L4 collapsed) |
| Side DAGs | 4 | 5 (add org-config DAG) |
| Total tasks | 120 | 100 |
| Width | 20 | 20 (unchanged) |
| Per-task timeout | none at dispatch | 5 min per tool call, 30 min per task |
| Worklog schema | documented in L1 #17 | hard precondition for task acceptance |
| 5th focus repo | KWatch fallback (per `FIFTH_FOCUS_REPO_DECISION_2026_06_10.md:34`) | KWatch (same) + 4 near-fallbacks (NetScript, Pine, AuthKit, Civis) |
| Org-config lane | L1 #18 (deferred, DNS-blocked) | dedicated side DAG SD5 |

---

## Appendix A: Source files NOT read (6 of 19)

These files were referenced in the manager's brief at
`/Users/kooshapari/CodeProjects/Phenotype/repos/` but were not present in
any local worktree (`.worktrees/*`, `FocalPoint-wtrees/*`) at the time
of this report. The findings in sections (a)-(g) that would have used
these files are partially extrapolated from the V3 DAG and V3 Execution
Log:

- `PROVIDER_REGISTRY_AUDIT_2026_06_10.md` — would have informed hex-port opportunity #1 (PlayCua Driver port) and SOTA gap area #5
- `CI_TEST_MATRIX_2026_06_10.md` — would have informed SOTA gap area #3
- `STALE_PR_TRIAGE_2026_06_10.md` — would have informed cheap-win #1 and expensive problem #1
- `CROSS_REPO_BUILD_MAP_2026_06_10.md` — would have informed SOTA gap area #3 and hex-port opportunity #4
- `REPO_INACTIVITY_AUDIT_2026_06_10.md` — would have informed health-score ranking #6-10
- `CHEAP_LLM_MCP_CONSUMPTION_PLAN_2026_06_10.md` — would have informed L2 #26 (the headline V3 task per `FLEET_100TASK_DAG_V3.md:200-207`)

## Appendix B: V4 launch agent output quality

The 6 of 10 V4 agents that produced output were evaluated as follows:

- `agent_01_tokn_pr59.out` (47 lines): STUCK in shell loop. Quality: 0/10.
- `agent_02_kmobile_pr21.out` (2 lines): OMNIROUTE ERROR. Quality: 0/10.
- `agent_03_pr_matrix.out` (16 lines): fabricated PR numbers. Quality: 1/10 (output exists but is false).
- `agent_04_phenocontracts_gov.out` (6 lines): first tool call only. Quality: 0/10.
- `agent_05_pine_mdbook.out` (26 lines): refused to run shell, asked user. Quality: 1/10 (output exists, but is "please do my job").
- `agent_06_cheapllm_spec.out` (79 lines): generic refactor plan, no JSON, no repo state. Quality: 3/10 (best of the 6, but still not task-ready).

Average: 0.83/10. Net V4 harvest value: **0 task-ready specs delivered**. The V20 dispatch layer should add: (a) per-tool-call timeouts, (b) per-task timeouts, (c) a "model produced output that is non-empty AND non-refusal AND non-fabricated" gate before accepting a task, (d) the canonical worklog schema as a hard precondition.

## Appendix C: V20 Wave-by-Wave Task Breakdown (proposed)

### V20 Wave 1 (Layer 1: state unification, 20 tasks)

L1 mirrors V3 L1 #1-20 (`FLEET_100TASK_DAG_V3.md:73-172`) with three concrete deltas:
- **L1 #16** (cross-repo duplication scan, V3 #16): re-dispatch the 6 MISSING audit files (PROVIDER_REGISTRY_AUDIT, CI_TEST_MATRIX, STALE_PR_TRIAGE, CROSS_REPO_BUILD_MAP, REPO_INACTIVITY_AUDIT, CHEAP_LLM_MCP_CONSUMPTION_PLAN) using the V3 worklog-converter pattern (`V3_EXECUTION_LOG_2026_06_10.md:1810-1816`) as a validation gate.
- **L1 #18** (org-config clone, V3 #18): re-attempt now that DNS may be unblocked; if still blocked, swap the lane for a `phenotype-org-governance` skeleton PR per `ORG_CONFIG_CLONE_2026_06_10.md:5-21`.
- **L1 #19-20** (5th-focus-repo decision, V3 #19-20): carry forward the `FIFTH_FOCUS_REPO_DECISION_2026_06_10.md:32-34` KWatch fallback + add the 4 near-fallbacks (NetScript, Pine, AuthKit, Civis) as a 6-way ranked list. The `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md:7-25` (NetScript as Phase-1 governance Rank 1) is the strongest signal that NetScript should jump above KWatch.

### V20 Wave 2 (Layer 2: tooling + dispatch, 20 tasks)

L2 carries forward the 10 done V3 L2 lanes as read-only artifacts and targets 10 new lanes. Of the 10 new lanes, the highest-priority 5 are:
- **L2 #26** (cheap-llm-mcp consumption): V3 #26 was blocked on the missing CHEAP_LLM_MCP_CONSUMPTION_PLAN; V20 should re-scope this lane to the agent_06 generic refactor (extract `cheap-llm-mcp/src/cli.ts`, make `index.ts` a re-export hub) plus add the dispatch-mcp tier enum (`WORKFLOW_PIN_AUDIT_2026_06_10.md:34-36` SHA-pinning model).
- **L2 #29** (dependabot baseline): per V3 L2 #29, not yet shipped. Required because the `WORKFLOW_PIN_AUDIT_2026_06_10.md` SHA-pinning pass created 138 new SHA-pinned refs that need automated tracking.
- **L2 #33-rev** (pre-commit race fix): V3 #33 shipped a race-condition fix (`V3_EXECUTION_LOG_2026_06_10.md:869-983`) that left 5 repos' `.pre-commit-config.yaml` overwritten. V20 should redo this lane as a single-agent pass with file-level lock to avoid the race.
- **L2 #38** (FocalPoint workflow SHA-pin, 3 refs per `WORKFLOW_PIN_AUDIT_2026_06_10.md:42-46`).
- **L2 #39** (nanovms 4-ref 404 fix per `V3_EXECUTION_LOG_2026_06_10.md:662-682`).

### V20 Wave 3 (Layer 3 collapsed: SOTA + hex + libify, 20 tasks)

This is the 1-layer collapse. 10 SOTA tasks (V3 L3 #41-60) and 10 hex/libify tasks (V3 L4 #61-80), alternating in the dispatch queue so that no agent holds a port declaration longer than its SOTA consumer. Critical-path is `L2 → Wave 3 → Wave 4` (4 layers total). The 20 tasks split as:
- 10 SOTA: pheno-errors (done, adoption), pheno-tracing, pheno-config, pheno-cargo-template (done, propagate), pheno-tokio-base, pheno-fastapi-base, pheno-go-ctxkit, pheno-flags, pheno-secret-scan, pheno-vibecoding-guard.
- 10 hex/libify: pheno-port-adapter, pheno-mcp-defs, pheno-mcp-transport, pheno-mcp-runtime, PlayCua hex refactor, AgilePlus hex refactor, nanovms hex refactor, PhenoCompose hex refactor, BytePort hex refactor, L4 #73-76 wrap-not-rewrite pattern.

### V20 Wave 4 (Layer 4: integration + side DAGs, 20 tasks)

L5 #81-100 mirrors V3 (`FLEET_100TASK_DAG_V3.md:459-570`) with one delta: add L5 #100.5 (a single V3→V20 close-out lane that produces `CROSS_REPO_LANDSCAPE_V21_2026_XX_XX.md` from this report's template). The 5 side DAGs:
- **SD1**: agent-user-status (5 tasks, V3 SD1 unchanged, `FLEET_100TASK_DAG_V3.md:577-586`)
- **SD2**: agentapi-plusplus (5 tasks, V3 SD2 unchanged, `FLEET_100TASK_DAG_V3.md:587-592`)
- **SD3**: Agentora (5 tasks, V3 SD3 unchanged, `FLEET_100TASK_DAG_V3.md:593-598`)
- **SD4**: AuthKit (5 tasks, V3 SD4 absorbed `phenotype-authvault`, `FLEET_100TASK_DAG_V3.md:599-606`)
- **SD5 (new)**: org-config (5 tasks: `phenotype-org-governance` init, `phenotype-gates` init, `phenotype-runs` init, `ORG_CONFIG_CLONE_2026_06_10.md` re-attempt, `dispatch-mcp` config sync per `FLEET_100TASK_DAG_V3.md:542-550`)

### V20 totals

- Main: 20 × 4 = **80 tasks** (down from V3's 100 main)
- Side DAGs: 5 × 4 = **20 tasks** (up from V3's 4 × 5 = 20)
- **Grand total: 100 tasks** (down from V3's 120, a 17% reduction)
- Critical path: L1 → L2 → Wave 3 → Wave 4 (4 layers, down from V3's 5)
- Width: 20 (V3 invariant preserved per `FLEET_100TASK_DAG_V3.md:49`)

## Appendix D: Per-Source-Audit Citation Density

A tally of how many times each source audit was cited across sections (a)-(g):

| Source audit | Citations |
|---|---:|
| `V3_EXECUTION_LOG_2026_06_10.md` | 47 (sections b, c, d, e, f, g) |
| `FLEET_100TASK_DAG_V3.md` | 38 (sections b, c, d, e, f, g) |
| `META_FILES_PRESENCE_2026_06_10.md` | 8 (sections a, d) |
| `BRANCH_AUDIT_2026_06_10.md` | 6 (sections a, e, f) |
| `DENY_TOML_DIVERGENCE_2026_06_10.md` | 5 (sections a, d, e) |
| `WORKFLOW_PIN_AUDIT_2026_06_10.md` | 5 (sections a, e, f) |
| `WORKTREE_AUDIT_2026_06_10.md` | 4 (sections a, e) |
| `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md` | 4 (sections b, c, d) |
| `WORKLOG_SCHEMA_2026_06_10.md` | 3 (sections a, e, g) |
| `FIFTH_FOCUS_REPO_DECISION_2026_06_10.md` | 3 (sections d, f) |
| `STASH_AUDIT_2026_06_10.md` | 2 (sections a, d) |
| `DAG_VS_V3_DELTA_2026_06_10.md` | 1 (section g) |
| `ORPHANED_DOCS_2026_06_10.md` | 1 (section e) |
| `ORG_CONFIG_CLONE_2026_06_10.md` | 2 (sections a, f) |

The 6 MISSING files would have added 10-15 additional citations per Appendix A; the extrapolated coverage above (sections a, c, d, e) is the floor.

## Appendix E: Failure-Mode Summary for V4 Launch Agents

Three distinct failure modes were observed in the 6 V4 launch agent outputs, and V20 dispatch hardening should address all three:

1. **Model stuck in shell loop** (agent_01): 7 repetitions of identical `gh pr view/diff` invocations with no progress. Hardening: per-tool-call timeout (5 min) + per-tool-call result-hash dedup (if same tool+args+result, kill the agent).
2. **Upstream routing failure** (agent_02): empty content + no tool_calls. Hardening: add a "minimum content length" gate (≥ 200 chars) before accepting any agent response.
3. **Model refusal / fabrication** (agents 03, 05): one fabricated a PR matrix, one refused to run shell. Hardening: add a "non-fabrication" check (no PR numbers in the response that aren't in the agent's prior context) and a "non-refusal" check (no "please run this yourself" or "I cannot" phrases in the first 200 chars).

The V3 layer-1 fan-out (`V3_EXECUTION_LOG_2026_06_10.md:1145`, 20 background codex agents, 4/20 done) is the better-validated model because it (a) uses the local codex CLI directly (no OmniRoute routing), (b) writes worklogs in the canonical schema from the start, and (c) has a human-in-the-loop merge phase (V3 #2, `V3_EXECUTION_LOG_2026_06_10.md:1647-1743`). V20 should retain the codex-CLI dispatch path and drop the cheap-llm-mcp routing for the V20 fleet.
