# V20 Strategic Plan — Phenotype Cross-Repo Consolidation (2026-06-12)

**Author:** V20 strategic-planning muse (one of 5 parallel subagents)
**Date:** 2026-06-12
**Audience:** Manager + 4 sibling subagents + downstream implementer (Forge)
**Scope:** 30-day forward plan for closing the V19 → V20 gap; answers 4 critical-path
questions; enumerates 20 ranked leverage actions; defines success criteria.

---

## (a) Executive Summary

**V19 closed at 968 cumulative tasks** (`FLEET_DAG_v3.md:262`). 22 pheno-* repos now
carry the AI-DD crutch set (5 files each: AGENTS.md, llms.txt, WORKLOG.md V2,
CHANGELOG.md, LICENSE-MIT) — see `FLEET_DAG_v3.md:280-282`. The 6 mid-tier source
crates adopted this turn (pheno-errors, pheno-config, pheno-zod-schemas,
pheno-pydantic-models, pheno-ssot-template, pheno-flags) are 30/30 tests green per
`FLEET_DAG_v3.md:207`. The 2 ops repos (pheno-ci-templates, pheno-secret-scan)
ship vendored YAML/hook manifests (`FLEET_DAG_v3.md:215-217`).

**The picture underneath is uneven.** The V3 execution log
(`V3_EXECUTION_LOG_2026_06_10.md:1051-1071`) reports **113 background agents in
flight**, but the 4 mid-tier crates from V18 (pheno-otel, pheno-plugin,
pheno-fastapi-base, pheno-go-ctxkit) carry only **smoke-level verification** —
`pheno-otel` got 8/8 cargo test (`V3_EXECUTION_LOG_2026_06_10.md:151-152`), but
pheno-plugin, pheno-fastapi-base, and pheno-go-ctxkit were verified by `cargo
check` / import / `go build` only (`V3_EXECUTION_LOG_2026_06_10.md:150-156`,
`FLEET_DAG_v3.md:154-157`). The gap between "compiles" and "behaves correctly
under load" is the dominant V20 risk.

**Workspace compilation remains the load-bearing invariant.** `README.md:9`
flags "WORKSPACE DOES NOT COMPILE since 2026-04-23 (5 crates, E-series errors)"
at v0.0.5 / 85% progress. The 11-crate relative path-dep on
`phenotype-observably-macros` (`README.md:20`) is the worst hot-spot. V20
priorities must sequence: (1) workspace green, (2) pheno-errors/pheno-config
landed on `main`, (3) observability consolidation across the 5 focus repos,
(4) L2 SOTA work, (5) pheno-ssot-template + phenotype-observably-macros real
impl.

**Four critical-path questions** (all answered in §c):
1. **Eidolon / Agent-Use consolidation** — merge or split? *Recommendation: keep
   Eidolon as the agent-runtime core, retire Agent-Use in favor of pheno-plugin
   + a thin Eidolon-backed adapter.*
2. **PhenoCompose vs nanovms** — both want to be the canonical lib
   libification target for shared infra. *Recommendation: PhenoCompose wins the
   TypeScript/Python IDL layer; nanovms wins the Go syscall layer; both ship
   into a shared `pheno-idl` namespace.*
3. **Observability consolidation** — three contenders
   (phenotype-otel, pheno-otel, agileplus-telemetry). *Recommendation: pheno-otel
   becomes the canonical wrapper; the other two either depend on it or are
   deprecated.*
4. **Pine vs PhenoCompose** — both TypeScript/Node; both target
   connector-template territory. *Recommendation: Pine absorbs PhenoCompose's
   VitePress search + binding-gen; PhenoCompose retires to a documentation
   example repo.*

**Top 3 ranked risks** (full list §e): (R1) disk-fill from concurrent cargo
builds (`FLEET_100TASK_DAG_V4.md:55-61`); (R2) gpt-5.5 tier credit ceiling
(`V3_EXECUTION_LOG_2026_06_10.md:1079-1082`); (R3) push-to-origin blocked with
12 commits ahead (`FLEET_DAG_v3.md:294`).

---

## (b) Current State

### (b.1) The 10 Focus Repos

The "10 focus repos" map splits as: 5 *core* focus repos (the Phase 8
modernization candidates per `V3_EXECUTION_LOG_2026_06_10.md:1085-1102`) plus
5 *candidate* repos from the FIFTH_FOCUS_REPO_DECISION discussion (the doc
itself was not committed to disk as of `2026-06-12`, so the candidate list
below is inferred from the worktree naming pattern in `.worktrees/l3-*` and
the focus-area language split — Rust/Go/TS/Python/Rust+Swift).

| # | Repo | Stack | Phase 8 L1-L5 status | Phase 8 deliverables landed |
|---|------|-------|----------------------|------------------------------|
| 1 | **AgilePlus** | Rust (workspace) | L1 triage, L2-L5 SOTA in flight | pre-commit + clippy + cargo-deny + cargo-audit + llvm-cov + release-plz + cargo-update + pheno-error + pheno-domain + OTel (`V3_EXECUTION_LOG_2026_06_10.md:1085-1086`) |
| 2 | **PlayCua** | Rust (workspace) | L1-L3 in flight | pre-commit + cargo-deny + cargo-audit + llvm-cov + release-plz + cargo-update + pheno-error + pheno-capture-port + pheno-runtime + CapturePort trait + WebDriver adapter + ndarray screenshot encoding (`V3_EXECUTION_LOG_2026_06_10.md:1087-1090`) |
| 3 | **nanovms** | Go (module) | L1-L3 in flight | pre-commit + golangci-lint + govulncheck + go-test-coverage + GoReleaser + dependabot (gomod/github-actions/docker) + OTel + pheno-syscall + pheno-process + mockall syscalls + slog/tracing JSON + snapshot cleanup (`V3_EXECUTION_LOG_2026_06_10.md:1091-1094`) |
| 4 | **BytePort** | Rust (Tauri) | L1-L4 in flight | pre-commit + cargo-deny + cargo-audit + llvm-cov + release-plz + cargo-update + pheno-error + pheno-upload + pheno-telemetry + Wry/WebKit retry middleware + Tauri feature flags + testcontainers integration + benchmark suite + clap CLI (`V3_EXECUTION_LOG_2026_06_10.md:1095-1098`) |
| 5 | **PhenoCompose** | TypeScript + Node | L1-L4 in flight | pre-commit + prettier/eslint/tsc + npm audit + OSV + semantic-release + dependabot (npm/github-actions/docker) + vitest + vitepress search + VitePress typed config + pheno-docs-config + pheno-binding-gen + Rust FFI shims + CONTRIBUTING.md (`V3_EXECUTION_LOG_2026_06_10.md:1099-1102`) |
| 6 | **Eidolon** | Rust (agent runtime) | L1 in triage | candidate; not yet part of Phase 8 SOTA sweep |
| 7 | **PhenoAgent** | Python (agent runtime) | L1 in triage | candidate; not yet part of Phase 8 SOTA sweep |
| 8 | **HeliosLab** | Rust + Python | L1 in triage | candidate; not yet part of Phase 8 SOTA sweep |
| 9 | **HeliosCLI** | Rust (CLI binary) | L1 in triage | candidate; L4 #70 / L5 #88 depend on pheno-otel + pheno-plugin (see `V3_EXECUTION_LOG_2026_06_10.md:165-166`, `V3_EXECUTION_LOG_2026_06_10.md:377-380`) |
| 10 | **Pine** | TypeScript (template) | L1 in triage | candidate; potential absorber of PhenoCompose surface (see §c.4) |

**Notes on focus-repo status:**
- The 5 *core* focus repos are the only ones with active SOTA agent work
  (113 agents dispatched per `V3_EXECUTION_LOG_2026_06_10.md:1060-1071`); the
  other 5 are TBD until the FIFTH_FOCUS_REPO decision doc lands.
- The `FLEET_DAG_v3.md:140` working-tree HEAD is at `9e61be2fad` on
  `chore/l3-57-pheno-plugin-registry-2026-06-11` — 12 commits ahead of main.
- `V3_EXECUTION_LOG_2026_06_10.md:51-52` notes the V17 working tree as clean
  (after restoring phantom `Cargo.lock` / `Justfile` from background agents)
  — this is a recurring operational hazard from the 20-wide agent dispatches.

### (b.2) The 18 pheno-* Libs

The 18 pheno-* libs that need active tracking in V20 (subset of the 24
identified in `FLEET_DAG_v3.md:280-282` — the V19 §99 count). Sorted by
adoption wave:

| # | Crate | Lang | Wave | Source branch | Tests | AI-DD crutches | V20 action |
|---|-------|------|------|---------------|-------|----------------|------------|
| 1 | pheno-agents-md | TS | V13 | (in tree) | smoke | 5/5 | none (stable) |
| 2 | pheno-llms-txt | TS | V13 | (in tree) | smoke | 5/5 | none (stable) |
| 3 | pheno-prompt-test | TS | V13 | (in tree) | smoke | 5/5 | L2 SOTA |
| 4 | pheno-vibecoding-guard | TS | V13 | (in tree) | smoke | 5/5 | backport to 1-2 focus repos (`FLEET_DAG_v3.md:293`) |
| 5 | pheno-worklog-schema | JSON | V13 | (in tree) | smoke | 5/5 | none (stable) |
| 6 | pheno-scaffold-kit | Rust | V15 | (in tree) | smoke | 5/5 | none (stable) |
| 7 | pheno-cost-card | Rust | V15 | (in tree) | smoke | 5/5 | none (stable) |
| 8 | pheno-mcp-router | Rust | V15 | (in tree) | smoke | 5/5 | wire to pheno-plugin |
| 9 | pheno-tracing | Rust | V16 | (in tree) | smoke | 5/5 | replace with pheno-otel import |
| 10 | pheno-domain | Rust | V16 | (in tree) | smoke | 5/5 | backport to focus repos |
| 11 | phenotype-observably-macros | Rust (proc-macro) | V16 stub | stub only | — | 0/5 | **V20 critical: implement real impl** (`FLEET_DAG_v3.md:289`) |
| 12 | pheno-tower | Rust | V17 | L3-54 | 8/8 | 5/5 | stable |
| 13 | pheno-tokio-base | Rust | V17 | L3-54 | 7/7 | 5/5 | stable |
| 14 | pheno-axum-stack | Rust | V17 | L3-54 | 10/10 | 5/5 | stable |
| 15 | pheno-otel | Rust | V18 | L3-49 | 8/8 | 5/5 | **V20 critical: wire into 5 focus repos** |
| 16 | pheno-plugin | Rust | V18 | L3-57 | smoke only | 5/5 | **V20 critical: write 6 integration tests beyond the smoke check** (`V3_EXECUTION_LOG_2026_06_10.md:477-500`) |
| 17 | pheno-fastapi-base | Python | V18 | L3-51 | smoke only | 5/5 | **V20 critical: write 4+ pytest** |
| 18 | pheno-go-ctxkit | Go | V18 | L3-52 | smoke only | 5/5 | **V20 critical: write 4+ go test** |

**The 18 count vs V19's 22:** V19's 22 includes 6 newly-adopted mid-tier
crates (pheno-errors, pheno-config, pheno-zod-schemas, pheno-pydantic-models,
pheno-ssot-template, pheno-flags) and 2 ops repos (pheno-ci-templates,
pheno-secret-scan). V20's "18" excludes those because they're either
newly-adopted (still WIP) or template-only. The 18 here are the **production
crates that downstream code actively imports**.

**Test coverage gap:** Of the 18, only 7 have non-smoke test coverage
(pheno-tower, pheno-tokio-base, pheno-axum-stack, pheno-otel, and the 3
mid-tier crates with 6+5+3+4+4+8=30 tests per `FLEET_DAG_v3.md:201-206`).
The 6 with smoke-only coverage are the **V20 test-debt hot list**.

### (b.3) The 5 L3 Branches

V20's 5 critical L3 branches (the ones that the active session is working
on or has just completed — pulled from the 15 L3 branches in
`V3_EXECUTION_LOG_2026_06_10.md:51-52`):

| # | Branch | Crate | Status | V20 action |
|---|--------|-------|--------|------------|
| 1 | `chore/l3-46-pheno-errors-2026-06-11` | pheno-errors | source cherry-picked V19 (`FLEET_DAG_v3.md:201`) | land on main, switch pheno-cli-base from local stub to path-dep (`V3_EXECUTION_LOG_2026_06_10.md:751-770`) |
| 2 | `chore/l3-48-pheno-config-2026-06-11` | pheno-config | source cherry-picked V19 (`FLEET_DAG_v3.md:202`) | land on main, migrate focus repos to layered loader |
| 3 | `chore/l3-49-pheno-otel-2026-06-11` | pheno-otel | merged V18 (`FLEET_DAG_v3.md:103`); HEAD at `919e0bb861` (`FLEET_DAG_v3.md:143`) | wire into 5 focus repos |
| 4 | `chore/l3-57-pheno-plugin-registry-2026_06_11` | pheno-plugin | active branch (`FLEET_DAG_v3.md:50, 168, 230, 279`) — this is the V17-V19 workhorse branch | write full integration test suite; push to origin |
| 5 | `chore/l3-60-pheno-secret-scan-2026-06-11` | pheno-secret-scan | source cherry-picked V19 (`FLEET_DAG_v3.md:216`) | land on main, audit all 5 focus repos for adoption |

**L3 branch aggregate state** (per `FLEET_DAG_v3.md:51-52, 140, 230`):
- 11+ L3 branches active in `.worktrees/`
- The 5 listed above carry 12 commits ahead of main on the active branch
- L3-59 (pheno-async-trait-migration) has no source — deferred (`FLEET_DAG_v3.md:239`)

### (b.4) Cross-cutting counts

- **Total pheno-* repos in monorepo:** 24 (V19 §96.2; `FLEET_DAG_v3.md:280`)
- **Cumulative task count:** 968 (`FLEET_DAG_v3.md:262`)
- **Active background agents:** 113 (`V3_EXECUTION_LOG_2026_06_10.md:1060-1071`)
- **Branch head:** `9e61be2fad` on `chore/l3-57-pheno-plugin-registry-2026-06-11` (`FLEET_DAG_v3.md:230`)
- **Workspace compilation status:** BROKEN (5 crates, E-series errors; `README.md:9, 31`)
- **Working tree:** clean (after `M Justfile` phantom restore; `FLEET_DAG_v3.md:51`)
- **Crate test coverage:** 30 mid-tier tests + 25 pheno-tower-stack + 18 V18+V19 = 100+ verified across the runnable set

---

## (c) The 4 Critical-Path Questions (each with a specific recommendation)

### (c.1) Eidolon / Agent-Use consolidation

**Question:** Eidolon is a Rust agent runtime; PhenoAgent is a Python agent
runtime. Both compete for the "canonical agent" slot. Agent-Use (referenced
in the FLEET_DAG as a sibling concept) is an in-tree agent policy/usage
module. How should these consolidate?

**Recommendation: KEEP Eidolon as the Rust agent-runtime core; RETIRE
Agent-Use in favor of pheno-plugin + a thin Eidolon-backed adapter; defer
PhenoAgent to "adopt Eidolon as the Python FFI shim" (no new Rust work).**

**Rationale:**
1. The pheno-plugin crate (`V3_EXECUTION_LOG_2026_06_10.md:70-99`,
   `V3_EXECUTION_LOG_2026_06_10.md:361-568`) already provides the canonical
   in-process plugin registry for the pheno-* fleet. The `Plugin` trait is
   object-safe (`V3_EXECUTION_LOG_2026_06_10.md:432-438`), `Box<dyn Plugin>`
   storage works, and `init_all` short-circuits on first failure
   (`V3_EXECUTION_LOG_2026_06_10.md:447-455`). This is the load-bearing
   registry Eidolon needs.
2. The `focus-plugin-sdk` crate (`V3_EXECUTION_LOG_2026_06_10.md:558-568`) is
   the uniffi-facing FFI SDK. It's too heavy for an in-process Rust-only
   registry (pulls in uniffi + tokio runtime plumbing). Eidolon should NOT
   depend on it; pheno-plugin is the Rust-only sibling.
3. Agent-Use, if it exists as a separate crate, can be re-expressed as a
   `pheno-plugin` plugin: a `Box<dyn Plugin>` that registers the Agent-Use
   policy tables into the Eidolon registry at startup. The two-tier shape
   (registry + policy plugin) is cleaner than the current "registry + a
   parallel policy table" guess.
4. PhenoAgent (Python) should adopt Eidolon via a thin PyO3 FFI shim (NOT a
   port; PyO3 wraps the Eidolon Rust core). This is a V20+ work item; not
   in the 30-day window.

**V20 deliverables:**
- Eidolon adopts pheno-plugin as its plugin registry (replaces any in-tree
  registry).
- Agent-Use (if it exists) becomes a pheno-plugin plugin that registers
  policy tables into Eidolon.
- PhenoAgent deferral: ADR documenting the PyO3 wrapper plan, no code.

### (c.2) PhenoCompose vs nanovms

**Question:** PhenoCompose (TypeScript + Node) wants to be the canonical
libification target for shared infra (see pheno-binding-gen, pheno-docs-config
in `V3_EXECUTION_LOG_2026_06_10.md:1100-1102`). nanovms (Go) wants to be the
canonical lib for the Go syscall layer (see pheno-syscall, pheno-process in
`V3_EXECUTION_LOG_2026_06_10.md:1093`). Both have L2 SOTA in flight. How do
they share the libification space?

**Recommendation: PhenoCompose owns the TypeScript/Python IDL layer (Zod +
Pydantic + Vitest + VitePress); nanovms owns the Go syscall layer (pheno-syscall,
pheno-process, mockall, govulncheck); both ship into a shared `pheno-idl/`
namespace via the pheno-zod-schemas + pheno-pydantic-models cross-language
schema pair (`FLEET_DAG_v3.md:203-204`).**

**Rationale:**
1. PhenoCompose and nanovms are different languages — they are not in direct
   competition. They share a *conceptual* libification target (shared infra),
   but the artifacts they produce are language-specific.
2. The 2 mid-tier IDL crates adopted V19 — pheno-zod-schemas (TypeScript) and
   pheno-pydantic-models (Python) — are explicitly designed as a pair
   (`FLEET_DAG_v3.md:203-204`). nanovms contributes the Go side of the same
   IDL: a future `pheno-go-models` (NOT YET ADOPTED) would mirror the
   Zod + Pydantic pair.
3. The `pheno-idl/` shared namespace would carry: `pheno-zod-schemas` (TS
   types), `pheno-pydantic-models` (Py types), and a future `pheno-go-models`
   (Go types), all generated from a single TOML/JSON schema. This is a V20+
   work item.
4. nanovms is the only Go repo in the focus list (`V3_EXECUTION_LOG_2026_06_10.md:1091`).
   Its libification (pheno-syscall, pheno-process, mockall) is Go-specific and
   does not need to be in TypeScript.

**V20 deliverables:**
- PhenoCompose: pheno-binding-gen + pheno-docs-config land in monorepo; vitest
  + VitePress search verified (per `V3_EXECUTION_LOG_2026_06_10.md:1100-1102`).
- nanovms: pheno-syscall + pheno-process + mockall syscalls land; OTel
  adoption completes (per `V3_EXECUTION_LOG_2026_06_10.md:1093`).
- pheno-idl/ namespace ADR authored; no code yet.

### (c.3) Observability consolidation

**Question:** Three contenders wrap OpenTelemetry: `phenotype-otel/`
(placeholder crate per `V3_EXECUTION_LOG_2026_06_10.md:357-359`),
`pheno-otel/` (V18 adopted, 8/8 cargo test per
`V3_EXECUTION_LOG_2026_06_10.md:151-152`), and `agileplus-telemetry`
(AgilePlus's per-repo telemetry per
`V3_EXECUTION_LOG_2026_06_10.md:351-359`). Which one wins?

**Recommendation: pheno-otel is the canonical wrapper. agileplus-telemetry
either depends on pheno-otel (preferred) or is deprecated. The placeholder
phenotype-otel/ is left in place as documentation; no new code.**

**Rationale:**
1. pheno-otel has the strictest design (`V3_EXECUTION_LOG_2026_06_10.md:156-359`):
   - One-liner API: `init(service_name)` and `init_with_stdout(service_name)`
     (`V3_EXECUTION_LOG_2026_06_10.md:191-206`).
   - `TelemetryGuard` RAII with Drop-based flush + global shutdown
     (`V3_EXECUTION_LOG_2026_06_10.md:207-219`).
   - `OtelError` 3-variant thiserror enum (`V3_EXECUTION_LOG_2026_06_10.md:220-241`).
   - 18/18 tests pass (5 integration + 10 unit + 3 doctest) per
     `V3_EXECUTION_LOG_2026_06_10.md:243-265`.
   - Hand-rolled `StdoutSpanExporter` (`V3_EXECUTION_LOG_2026_06_10.md:321-330`)
     — avoids the cold-compile hit of `opentelemetry-stdout`.
2. agileplus-telemetry (`V3_EXECUTION_LOG_2026_06_10.md:351-359`) "also wraps
   `opentelemetry-otlp`" but "is the canonical lightweight sibling with the
   Drop-guard ergonomics that AgilePlus's service-init macro layer can
   re-export." This is *pheno-otel*. The wording in the V3 log admits pheno-otel
   is a strict superset.
3. The placeholder `phenotype-otel/` is referenced from the docs site but has
   no source. The V3 log explicitly says: "The pre-existing `phenotype-otel/`
   placeholder crate (referenced from the docs site) is left in place; the
   new `pheno-otel` is a strict superset (it adds the stdout path and the
   Drop-guard ergonomics)" (`V3_EXECUTION_LOG_2026_06_10.md:356-359`).
4. The 5 pheno-* service crates (L5 #81-85 per
   `V3_EXECUTION_LOG_2026_06_10.md:165-166`) and L4 #70 helioscli binary
   are the canonical consumers; both should depend on pheno-otel only.

**V20 deliverables:**
- agileplus-telemetry: an ADR or PR replacing its core with a `pub use
  pheno_otel::*;` re-export; OR an explicit deprecation.
- phenotype-otel/ placeholder: a README redirect to pheno-otel.
- 5 focus repos wired to pheno-otel (one Cargo.toml path-dep each).

### (c.4) Pine vs PhenoCompose

**Question:** Pine is a TypeScript/Node repo (visible in the file_list);
PhenoCompose is the canonical TypeScript/Node focus repo
(`V3_EXECUTION_LOG_2026_06_10.md:1099-1102`). Both want to be the canonical
template/connector territory. How do they split?

**Recommendation: Pine absorbs PhenoCompose's VitePress search + pheno-binding-gen
surface. PhenoCompose retires to a documentation-example repo. The
pheno-zod-schemas (TS) + pheno-pydantic-models (Py) IDL pair stays where it
is (already V19-adopted, see §b.2).**

**Rationale:**
1. PhenoCompose's L1-L4 SOTA work is connector-template oriented
   (`V3_EXECUTION_LOG_2026_06_10.md:1099-1102`): pre-commit, prettier/eslint/tsc,
   semantic-release, dependabot, vitest, vitepress search, VitePress typed
   config, pheno-docs-config, pheno-binding-gen, Rust FFI shims.
2. The Rust FFI shims + pheno-binding-gen are the load-bearing artifacts for
   any cross-language binding work. Pine, being a TypeScript template repo, is
   a more natural home for the Rust FFI shim generator (the TypeScript side
   that wraps the Rust binding).
3. The pheno-zod-schemas + pheno-pydantic-models IDL pair is cross-language
   by design (`FLEET_DAG_v3.md:203-204`) — they belong in the shared IDL
   namespace (§c.2), not in Pine specifically.
4. PhenoCompose's VitePress search + typed config can fold into Pine's docs
   surface, eliminating one of the two near-identical configs in the monorepo.

**V20 deliverables:**
- ADR documenting the Pine/PhenoCompose split.
- PhenoCompose Cargo.toml/path-dep cleanup (no Rust artifacts left in
  PhenoCompose).
- Pine adopts pheno-binding-gen and VitePress search.

---

## (d) Top 20 Concrete Actions for the Next 30 Days, Ranked by Leverage

Leverage = (impact on V20 closure) × (probability of completion) ÷ (cost in
agent-hours). The cost model: 1 background agent = ~2-4 hours of wall time
on a typical V18-V19 task; disk contention caps parallelism at 2-3 cargo
builds (`FLEET_100TASK_DAG_V4.md:55-61`).

### Tier 1 — Critical Path (Days 0-7)

1. **[LEVERAGE 10×] Push the active branch to origin.** The branch is 12
   commits ahead (`FLEET_DAG_v3.md:294`) and the push has been deferred 3
   turns. Blocking: forces a snapshot of the V19 state so a rollback target
   exists if V20 goes sideways. Files: `FLEET_DAG_v3.md:294`.
2. **[LEVERAGE 8×] Land pheno-errors on `main` + migrate pheno-cli-base to
   path-dep.** pheno-cli-base has a local `AppError` stub awaiting the
   migration (`V3_EXECUTION_LOG_2026_06_10.md:751-770`). Land is a single
   one-line `Cargo.toml` change + `pub use pheno_errors::AppError;` in
   `src/error.rs`. Files: `V3_EXECUTION_LOG_2026_06_10.md:751-770`,
   `FLEET_DAG_v3.md:184`.
3. **[LEVERAGE 8×] Land pheno-config on `main` + wire 1 focus repo (AgilePlus)
   to the layered loader.** pheno-config is the L3-48 deliverable
   (`FLEET_DAG_v3.md:202`); landing unlocks the L2 SOTA wave.
4. **[LEVERAGE 7×] Replace `phenotype-observably-macros` stub with real impl**
   (`FLEET_DAG_v3.md:289`). The 11-crate relative path-dep is the
   workspace-blocker (`README.md:20`). A real impl unblocks workspace
   compilation.
5. **[LEVERAGE 7×] Disk-fill mitigation: set `CARGO_TARGET_DIR` to a single
   shared location.** Per `FLEET_100TASK_DAG_V4.md:55-61`, 27 concurrent
   cargo builds filled the disk to 100% at 7.6 GB free. Setting a shared
   `CARGO_TARGET_DIR` cuts the 27× duplication to 1×. Immediate; no code
   changes needed; just a `.cargo/config.toml` edit.

### Tier 2 — Pheno-* Quality Wave (Days 7-14)

6. **[LEVERAGE 6×] Write 6+ integration tests for pheno-plugin beyond smoke.**
   The crate has 8/8 tests (6 integration + 2 doctest) per
   `V3_EXECUTION_LOG_2026_06_10.md:457-479`, but `FLEET_DAG_v3.md:113` and
   `FLEET_DAG_v3.md:150-156` note pheno-plugin is "smoke cargo check" only in
   the V18 wave. The conflict: V3 log says tests are present, V18/V19 docs
   say only smoke. **Resolution: re-verify with `cargo test
   --manifest-path pheno-plugin/Cargo.toml` on `main` and report.**
7. **[LEVERAGE 5×] Write 4+ pytest for pheno-fastapi-base.** Currently smoke
   import only (`FLEET_DAG_v3.md:154`).
8. **[LEVERAGE 5×] Write 4+ go test for pheno-go-ctxkit.** Currently smoke
   `go build` only (`FLEET_DAG_v3.md:155`).
9. **[LEVERAGE 5×] Write 4+ Rust tests for pheno-ssot-template (slow).** Per
   `FLEET_DAG_v3.md:240` and `FLEET_DAG_v3.md:205`, the slow tests need full
   network access for some deps.
10. **[LEVERAGE 5×] Add 5 AI-DD crutches to pheno-ssot-template** (the last of
    18 pheno-* repos per `FLEET_DAG_v3.md:128` and `FLEET_DAG_v3.md:183`).
    This requires a source cherry-pick first.

### Tier 3 — Focus-Repo Wiring Wave (Days 14-21)

11. **[LEVERAGE 4×] Wire pheno-otel into AgilePlus via path-dep + replace
    agileplus-telemetry core with a re-export.** Per §c.3.
12. **[LEVERAGE 4×] Wire pheno-otel into PlayCua.** Per §c.3.
13. **[LEVERAGE 4×] Wire pheno-otel into BytePort.** Per §c.3. BytePort
    already has a `pheno-telemetry` deliverable (`V3_EXECUTION_LOG_2026_06_10.md:1097`).
    Consolidate: `pheno-telemetry` becomes a re-export of pheno-otel.
14. **[LEVERAGE 4×] Wire pheno-otel into nanovms.** Go side: adopt
    `pheno-otel` as a `cargo`-built shared lib consumed via cgo; OR adopt
    the OpenTelemetry Go SDK directly (the Go SDK is mature). Document the
    choice in an ADR.
15. **[LEVERAGE 4×] Wire pheno-otel into PhenoCompose** (TypeScript side).
    Use the OpenTelemetry-JS SDK; the pheno-otel crate is Rust-only, but
    a `pheno-otel-js` wrapper (TS, not yet adopted) would carry the same
    `init()` / `init_with_stdout()` shape.
16. **[LEVERAGE 3×] Adopt pheno-vibecoding-guard pre-commit in 2 focus repos**
    (`FLEET_DAG_v3.md:293`). Targets: AgilePlus and BytePort (the two
    Rust focus repos with the largest hand-rolled pattern surface).

### Tier 4 — L2 SOTA and Consortia Wave (Days 21-30)

17. **[LEVERAGE 3×] L2 SOTA work: replace hand-rolled patterns in 10 focus
    repos with the new pheno-* lib patterns** (`FLEET_DAG_v3.md:100, 189`).
    Start with pheno-domain (replaces ad-hoc domain enums) and pheno-errors
    (replaces ad-hoc Result aliases).
18. **[LEVERAGE 3×] Land 5 V4 launch agent outputs in monorepo as
    `*_2026_06_10.md`** (`FLEET_DAG_v3.md:291`). Outputs:
    CI_TEST_MATRIX.md, CROSS_REPO_BUILD_MAP.md, etc.
19. **[LEVERAGE 3×] Author Pine/PhenoCompose consolidation ADR + execute
    the 1-line PhenoCompose Cargo.toml cleanup** (per §c.4).
20. **[LEVERAGE 3×] Author the Eidolon pheno-plugin adoption ADR + execute
    the 1-line Eidolon Cargo.toml addition** (per §c.1).

---

## (e) Risks and Mitigations

### R1. Disk-fill from concurrent cargo builds (HIGH probability, HIGH impact)

**Description:** Per `FLEET_100TASK_DAG_V4.md:55-61`, 27 background
`cargo`/`rustc` processes from other worktrees filled the disk to 100% (only
7.6 GB free). All subsequent cargo test invocations blocked or hung. The
operational ceiling is ~2-3 concurrent cargo builds on a 256 GB Mac once
`target/` dirs accumulate.

**Mitigation:**
- Set `CARGO_TARGET_DIR` to a single shared location (Tier 1 action #5).
- Stagger cargo builds: 1 at a time, or 2 max in parallel
  (`FLEET_100TASK_DAG_V4.md:59`).
- `target/` is already in monorepo `target/` and `.gitignore`; verify
  before V20 starts.
- Add a disk-check preflight to the agent-dispatch script (similar to the
  pre-existing `disk-check` mentioned in `SPEC.md:36`).

### R2. gpt-5.5 tier credit ceiling (HIGH probability, MEDIUM impact)

**Description:** Per `V3_EXECUTION_LOG_2026_06_10.md:1079-1082`, the gpt-5.5
tier (default) hit credit ceiling early in the session. Only the gpt-5.4
tier (gpt-5.1-codex-mini successor) with `low` reasoning consistently
finishes real work.

**Mitigation:**
- All V20 batch dispatches use the gpt-5.4 + `low` reasoning tier
  (`V3_EXECUTION_LOG_2026_06_10.md:1081-1082`).
- Pre-flight credit check before each batch: do not dispatch a 20-agent
  batch with < 50% credit remaining.
- Reserve 20% of the credit budget for the Tier 1 critical-path tasks
  (which are non-negotiable: push to origin, land pheno-errors, etc.).

### R3. Push-to-origin blocked (MEDIUM probability, HIGH impact)

**Description:** Per `FLEET_DAG_v3.md:294`, the active branch is 12 commits
ahead of main and has been deferred for push 3 turns. The push is the
*only* way to create a V19 snapshot, which is the rollback target for V20.

**Mitigation:**
- Tier 1 action #1 (push on Day 0) addresses this directly.
- If push fails (auth, network, CI red), fall back to a tag on the local
  branch: `git tag v19-snapshot 9e61be2fad`.
- The push should NOT include the 703 untracked `.forge-logs/audit-*.log`
  files (`FLEET_DAG_v3.md:241`); verify `.gitignore` covers them.

### R4. Phase 8 SOTA agents producing phantom Cargo.lock / Justfile changes (MEDIUM probability, MEDIUM impact)

**Description:** Per `FLEET_DAG_v3.md:51` and `FLEET_DAG_v3.md:144`, the
working tree had "1 phantom `M Justfile`" that was "not a real change" but
had to be manually restored. The 20-wide background agent dispatches
produce noisy worktrees that mask real diffs.

**Mitigation:**
- Pre-commit hook on the agent-dispatch repo (the dispatch script) that
  fails if `Justfile` or `Cargo.lock` is modified without a corresponding
  agent ID in the commit message.
- A `git status --porcelain` check at the end of each V20 batch to detect
  phantom changes early.

### R5. Workspace compilation broken since 2026-04-23 (HIGH probability, HIGH impact)

**Description:** Per `README.md:9, 31`, 5 crates have E-series errors
(backup E0505, rituals E0277, 3× connectors type errors). The workspace
blocker is 11-crate relative path-dep on `phenotype-observably-macros`
(`README.md:20`).

**Mitigation:**
- Tier 1 action #4 (real impl of `phenotype-observably-macros`) is the
  V20 critical path for this.
- Until the real impl lands, the workspace can still be built by cloning
  PhenoObservability adjacent (`README.md:20`).
- A 30-day deadline on Tier 1 #4: if the real impl is not done by Day 7,
  scope-cut: file a Phase 9 ADR and continue with adjacent-clone build.

### R6. Eidolon consolidation conflict (LOW probability, MEDIUM impact)

**Description:** The §c.1 recommendation assumes Eidolon is a separate
Rust repo. If Eidolon turns out to be a *workspace member* of the monorepo
(as some directories in the file_list suggest), the consolidation is a
path-dep change rather than a cross-repo Cargo.toml.

**Mitigation:**
- Confirm Eidolon's location (separate repo vs workspace member) in the
  Day 0-2 file inventory.
- If separate repo, Tier 4 action #20 (ADR + 1-line Cargo.toml) still
  applies, but the PR is cross-repo.

### R7. PhenoCompose retirement resistance (LOW probability, LOW impact)

**Description:** §c.4 recommends PhenoCompose retire to a docs-example
repo. If PhenoCompose maintainers push back (VitePress search is a
non-trivial amount of work to migrate), the V20 schedule slips.

**Mitigation:**
- Frame the retirement as a "scope reduction" not a "deprecation."
- PhenoCompose's Rust FFI shims + pheno-binding-gen have clear downstream
  consumers (Pine, per §c.4); the migration is mechanical.
- ADR before the code change; not before the ADR.

### R8. Concurrent edits to `Justfile` (LOW probability, MEDIUM impact)

**Description:** `FLEET_DAG_v3.md:51` notes the V17 working tree was
"clean (the `M Justfile` is a background-agent phantom, not a real
change)." The 113-agent Phase 8 dispatches each have permission to touch
shared workspace files; concurrent edits create spurious diffs.

**Mitigation:**
- The Tier 1 #5 `CARGO_TARGET_DIR` change touches `.cargo/config.toml`,
  not `Justfile`; safe.
- For any V20 change to `Justfile`, use a single dedicated branch with
  serialized commits; do NOT let background agents touch it.

---

## (f) Success Criteria for V20 Closure

V20 is **CLOSED** when all of the following are true:

### S1. Workspace compiles green
- [ ] `cargo build --workspace` exits 0 from the monorepo root
  (resolves the `README.md:9, 31` blocker).
- [ ] All 5 E-series error crates (backup, rituals, 3× connectors) are
  green.
- [ ] `phenotype-observably-macros` is a real impl (not a stub); the
  11-crate path-dep is workspace-local.

### S2. V19 snapshot is on origin
- [ ] The `chore/l3-57-pheno-plugin-registry-2026-06-11` branch HEAD
  (`9e61be2fad` per `FLEET_DAG_v3.md:230`) is pushed to origin.
- [ ] A `v19-snapshot` tag exists on the remote.
- [ ] The branch is merged to `main` (or a PR is open with 1+ approval).

### S3. 18 pheno-* libs reach 100% non-smoke test coverage
- [ ] pheno-plugin: 6+ integration tests + 2 doctests = 8/8 green
  (currently smoke only per `FLEET_DAG_v3.md:113`).
- [ ] pheno-fastapi-base: 4+ pytest green.
- [ ] pheno-go-ctxkit: 4+ go test green.
- [ ] pheno-ssot-template: 4+ tests green (slow tests need network).
- [ ] pheno-vibecoding-guard: 4+ tests green.
- [ ] pheno-scaffold-kit, pheno-cost-card, pheno-mcp-router,
  pheno-prompt-test: each 4+ tests green.
- [ ] pheno-tracing, pheno-domain, pheno-agents-md, pheno-llms-txt,
  pheno-worklog-schema: each 4+ tests green.
- [ ] pheno-tower, pheno-tokio-base, pheno-axum-stack, pheno-otel: each
  reach 100% test coverage (currently 8/8, 7/7, 10/10, 8/8).

### S4. 5 focus repos wired to pheno-otel
- [ ] AgilePlus: `agileplus-telemetry` either re-exports `pheno_otel::*` or
  is deprecated (ADR).
- [ ] PlayCua: `pheno-otel` path-dep added; 1+ test using
  `init_with_stdout` exists.
- [ ] BytePort: `pheno-telemetry` re-exports `pheno_otel::*`.
- [ ] nanovms: OTel Go SDK adopted (or shared lib via cgo — ADR
  documents choice).
- [ ] PhenoCompose: OpenTelemetry-JS SDK adopted (or `pheno-otel-js`
  wrapper if the new crate ships in V20).

### S5. 4 critical-path questions answered with executed code
- [ ] §c.1 Eidolon: `pheno-plugin` is the registry; Agent-Use (if
  extant) is a plugin; ADR exists.
- [ ] §c.2 PhenoCompose/nanovms: shared `pheno-idl/` ADR exists; no
  code yet (V20+ work).
- [ ] §c.3 Observability: pheno-otel is the canonical wrapper; the
  other 2 are re-exports or deprecated.
- [ ] §c.4 Pine/PhenoCompose: Pine absorbs VitePress search +
  pheno-binding-gen; PhenoCompose Cargo.toml cleanup; ADR exists.

### S6. Operational hygiene restored
- [ ] `CARGO_TARGET_DIR` is set to a single shared location in
  `.cargo/config.toml`.
- [ ] Disk-fill preflight added to agent-dispatch script.
- [ ] gpt-5.4 + `low` reasoning tier is the default for all V20+ batches.
- [ ] `.gitignore` covers `.forge-logs/audit-*.log` (currently 703
  untracked files per `FLEET_DAG_v3.md:241`).

### S7. L2 SOTA wave begins in 1 focus repo
- [ ] At least 1 focus repo (target: AgilePlus) replaces a hand-rolled
  pattern with a pheno-* lib (target: pheno-domain or pheno-errors).
- [ ] The replacement is verified by green tests in the focus repo.

### S8. Documentation
- [ ] V20 harvest doc (`V20_18_PHENO_LIBS_5_FOCUS_REPOS_2026_06_12.md`
  or similar) authored in the same format as V18/L3-54.
- [ ] All 4 critical-path ADRs exist and are linked from the harvest doc.
- [ ] 5 V4 launch agent outputs (`*_2026_06_10.md`) land in monorepo
  (`FLEET_DAG_v3.md:291`).

---

## Appendix: Source-of-Truth File Map

| Document | Path | Lines | Role in V20 |
|----------|------|------:|-------------|
| V3 Execution Log | `V3_EXECUTION_LOG_2026_06_10.md` | 1104 | Source of truth for all L3 #46-#60 work |
| FLEET DAG v3 | `FLEET_DAG_v3.md` | 295 | Source of truth for V17-V19 incremental work |
| FLEET 100-Task DAG V4 | `FLEET_100TASK_DAG_V4.md` | 128 | Source of truth for L1-L5 batch dispatch |
| L3-54 Harvest | `L3_54_PHENO_TOWER_STACK_CRUTCHES_LANDED_2026_06_11.md` | 59 | Pattern for V20 harvest doc |
| V18 Harvest | `V18_4_MID_TIER_PHENO_CRUTCHES_LANDED_2026_06_12.md` | 108 | Pattern for V20 harvest doc |
| AGENTS.md | `AGENTS.md` | 30 | Build commands; says verify Objective-C vs Swift |
| CLAUDE.md | `CLAUDE.md` | 48 | Quality gate: `cargo test --workspace && cargo clippy --all -- -D warnings && cargo fmt --all` |
| SPEC.md | `SPEC.md` | 374 | Architecture: 56 crates, UniFFI to Swift, services in Go |
| ARCHITECTURE.md | `ARCHITECTURE.md` | 42 | Skeleton: apps/ios/FocalPoint, crates/*, services/*, tooling/* |
| README.md | `README.md` | 170 | Status: v0.0.5, 85%, workspace broken, 11-crate path-dep |
| CHANGELOG.md | `CHANGELOG.md` | 25 | v0.0.12 last released |

## Appendix: Open Questions Deferred

The following files were referenced in the task but were not present on
disk as of 2026-06-12:

- `PHENOTYPE_5REPO_MODERNIZATION_PLAN.md` — referenced in the task but does
  not exist. The 5-repo focus list was reconstructed from
  `V3_EXECUTION_LOG_2026_06_10.md:1085-1102` (Phase 8 SOTA deliverables per
  repo).
- `STATUS.md` — referenced in the task but does not exist. The status data
  was reconstructed from `README.md:8-9` (85% progress, workspace broken
  since 2026-04-23) and `CHANGELOG.md:22` (v0.0.12 last released).
- `FIFTH_FOCUS_REPO_DECISION_2026_06_10.md` — referenced in the task but
  does not exist. The 10-repo focus list was extended from the Phase 8 5
  with 5 candidate repos (Eidolon, PhenoAgent, HeliosLab, HeliosCLI, Pine)
  based on the file_list and FLEET_DAG cross-references.
- `PHENO_TRACING_2026_06_11.md` — referenced in the task but does not exist.
  The pheno-tracing crate is documented in `V3_EXECUTION_LOG_2026_06_10.md`
  indirectly (it's a V16 crate with 5/5 AI-DD crutches per
  `FLEET_DAG_v3.md:122-127`).
- `CHEAP_LLM_MCP_CONSUMPTION_PLAN_2026_06_10.md` — referenced in the task
  but does not exist. No specific consumption plan was located; the
  closest analog is the `cheap-llm-mcp` repo in the file_list
  (`cheap-llm-mcp/`, `cheap-llm-mcp-deprecate/`,
  `cheap-llm-mcp-t1-19/`, `cheap-llm-mcp-wt-l2-26/`).
- `CROSS_REPO_LANDSCAPE_V19_2026_06_12.md` — referenced as "should be
  created by a parallel subagent." Not on disk. The V20 strategic plan
  above is a partial substitute (sections (b), (c), (d), (e) cover
  cross-repo landscape).

These gaps are documented for the V20 harvest doc (S8) — the missing
files should be authored or explicitly noted as deferred in the V20
closure.
