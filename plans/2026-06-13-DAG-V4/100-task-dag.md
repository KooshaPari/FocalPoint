# Unbounded DAG V4 (20×N) — Phenotype Monorepo Consolidation

## Design Rules
1. **20-wide rectangle** — every wave must have exactly 20 slots.
2. **No shrinking** — if consolidation tasks are fewer than 20, fill gaps with complementary tasks (hygiene, audits, SOTA, optimization, guardrails, test coverage, or other repo work).
3. **N grows as needed** — there is no fixed 5-wave limit. The DAG continues indefinitely until all work is done.
4. **Parallelism max 20** — at most 20 subagents spawn in parallel per wave.

## Wave Diagram

```
Wave 1 ─► Wave 2 ─► Wave 3 ─► Wave 4 ─► Wave 5 ─► ... ─► Wave N
  20        20        20        20        20              20
```

Total slots: N × 20 — unbounded.

---

## Wave 1 — Foundation Audits (20 tasks)

| Slot | Task | Description | Output |
|------|------|-------------|--------|
| W1-01 | `cargo check --workspace` | Full compilation check | `w1-01-cargo-check.log` |
| W1-02 | `cargo clippy --workspace` | Lint pass | `w1-02-clippy.log` |
| W1-03 | `cargo test --workspace --no-run` | Test compilation only | `w1-03-test-build.log` |
| W1-04 | Re-export audit | `pub use` cross-crate chains | `w1-04-reexports.{md,json}` |
| W1-05 | Error enum mapping | `pub enum Error` and `type Result` | `w1-05-errors.md` |
| W1-06 | Derive macro inventory | `#[derive(...)]` usage | `w1-06-derives.md` |
| W1-07 | Unsafe block inventory | `unsafe { ... }` occurrences | `w1-07-unsafe.md` |
| W1-08 | Dependency snapshot | `cargo tree` + semver check | `w1-08-deps.md` |
| W1-09 | CI run capture | Recent GitHub Actions runs | `w1-09-ci-runs.md` |
| W1-10 | Per-repo `cargo test --no-run` | AgilePlus, PhenoCompose, BytePort, nanovms, PlayCua | `w1-10-{repo}.log` |
| W1-11 | Branch log | `chore/l5*` and all local branches | `w1-11-branches.md` |
| W1-12 | Worktree state | `.worktrees/` audit | `w1-12-worktrees.md` |
| W1-13 | PhenoShared sync | Git subtree status | `w1-13-phenoShared.md` |
| W1-14 | phenoscripts inventory | `phenoscripts/` contents | `w1-14-scripts.md` |
| W1-15 | SOTA research — 2026 | Current tech landscape | `w1-15-sota-2026.md` |
| W1-16 | SOTA — macOS focus management | Latest APIs & tools | `w1-16-sota-focus.md` |
| W1-17 | CI workflows inventory | `.github/workflows/*.yml` | `w1-17-ci.md` |
| W1-18 | `cargo deny` | License & security audit | `w1-18-deny.log` |
| W1-19 | Coverage analysis | Test coverage metrics | `w1-19-coverage.md` |
| W1-20 | `cargo bloat` | Binary size analysis | `w1-20-bloat.log` |

---

## Wave 2 — Phases 1-5 Consolidation (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W2-01 | Fix `focus-icon-gen` anyhow | Add `anyhow` dep | W1-01 |
| W2-02 | Fix `focus-serde` Debug | `dyn Serializer` Debug impl | W1-01 |
| W2-03 | Fix `phenotype-derive` clippy | Prefix unused vars | W1-02 |
| W2-04 | Fix `phenotype-config` profile warning | Move profile to root | W1-01 |
| W2-05 | Consolidate `pub enum Error` | Unify error enums across crates | W1-05 |
| W2-06 | Consolidate `type Result` | Single `Result<T, E>` alias | W1-05 |
| W2-07 | Remove orphaned re-exports | Delete unused `pub use` | W1-04 |
| W2-08 | Fix broken re-export chains | Repair broken `pub use` | W1-04 |
| W2-09 | `cargo test --workspace` | Run tests after fixes | W1-03, W2-01..W2-08 |
| W2-10 | `cargo clippy --workspace` | Re-lint after fixes | W1-02, W2-01..W2-08 |
| W2-11 | Fix `cargo deny` issues | Resolve license/security warnings | W1-18 |
| W2-12 | Fix `cargo bloat` regressions | Reduce binary size | W1-20 |
| W2-13 | Add missing tests | Fill coverage gaps from W1-19 | W1-19 |
| W2-14 | CI workflow fix | Fix broken workflows from W1-17 | W1-17 |
| W2-15 | SOTA integration — focus API | Apply macOS focus API findings | W1-16 |
| W2-16 | SOTA integration — 2026 tech | Apply new tech findings | W1-15 |
| W2-17 | `phenotype-observably-macros` path fix | Fix broken path in FocalPoint | W1-08 |
| W2-18 | Worktree cleanup | Remove stale worktrees | W1-12 |
| W2-19 | Branch cleanup | Remove merged/stale branches | W1-11 |
| W2-20 | PhenoShared subtree update | Pull latest subtree | W1-13 |

---

## Wave 3 — Phases 6-10 Consolidation (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W3-01 | Transpiler trait unification | Merge `FocusTranspiler` + `Transpiler` | W2-09 |
| W3-02 | Builder pattern migration | Convert to type-state builders | W2-09 |
| W3-03 | Serde roundtrip tests | Add `serde_test` for all serializable types | W2-09 |
| W3-04 | Crypto crate integration | Wire `phenotype-crypto` into focus | W2-09 |
| W3-05 | FFI safety audit | Verify all `unsafe` from W1-07 is sound | W1-07 |
| W3-06 | Event system consolidation | Merge `focus-events` + `focus-events-core` | W2-09 |
| W3-07 | Domain model refactor | Apply `focus-domain` findings | W2-09 |
| W3-08 | Connector abstraction | Unify connector traits | W2-09 |
| W3-09 | CLI argument consolidation | Merge CLI args from multiple crates | W2-09 |
| W3-10 | Plugin system hardening | Add guardrails for plugin loading | W2-09 |
| W3-11 | Storage backend unification | Merge storage abstractions | W2-09 |
| W3-12 | Config system refactor | Apply `phenotype-config` findings | W2-09 |
| W3-13 | Telemetry integration | Wire OTel into all crates | W2-09 |
| W3-14 | Security audit fixes | Apply findings from W1-18 | W2-11 |
| W3-15 | Workspace hygiene | Fix `Cargo.toml` formatting, sort deps | W2-09 |
| W3-16 | Test infrastructure | Add `insta` + `pretty_assertions` | W2-09 |
| W3-17 | Documentation pass | Add rustdoc to all public items | W2-09 |
| W3-18 | Benchmark suite | Add `criterion` benchmarks for hot paths | W2-09 |
| W3-19 | Cross-compilation check | `cargo check` for all targets | W2-09 |
| W3-20 | Dependency deduplication | Remove duplicate transitive deps | W2-09 |

---

## Wave 4 — Phases 11-15 Consolidation (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W4-01 | IR system completion | Finish `focus-ir` migration | W3-01 |
| W4-02 | Language parser completion | Finish `focus-lang` grammar | W3-01 |
| W4-03 | Mascot system | Complete `focus-mascot` | W3-01 |
| W4-04 | Coaching system | Complete `focus-coaching` | W3-01 |
| W4-05 | Rituals system | Complete `focus-rituals` | W3-01 |
| W4-06 | Policy system | Complete `focus-policy` | W3-01 |
| W4-07 | Scheduler system | Complete `focus-scheduler` | W3-01 |
| W4-08 | Rewards system | Complete `focus-rewards` | W3-01 |
| W4-09 | Penalties system | Complete `focus-penalties` | W3-01 |
| W4-10 | Backup system | Complete `focus-backup` | W3-01 |
| W4-11 | Calendar integration | Complete `focus-calendar` | W3-01 |
| W4-12 | Asset fetcher | Complete `focus-asset-fetcher` | W3-01 |
| W4-13 | Icon generator | Complete `focus-icon-gen` | W2-01 |
| W4-14 | FFI bindings | Complete `focus-ffi` bindings | W3-05 |
| W4-15 | MCP server | Complete `focus-mcp-server` | W3-01 |
| W4-16 | Observability | Complete `focus-observability` | W3-13 |
| W4-17 | CI watcher | Complete `focus-ci-watcher` | W3-14 |
| W4-18 | Demo seed | Complete `focus-demo-seed` | W3-01 |
| W4-19 | Always-on | Complete `focus-always-on` | W3-01 |
| W4-20 | Eval system | Complete `focus-eval` | W3-01 |

---

## Wave 5 — Side DAGs & Verification (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W5-01 | `.worktrees/` cleanup | Remove all stale worktrees | W2-18 |
| W5-02 | `.agileplus/` cleanup | Clean up DB files | W2-18 |
| W5-03 | `target/` pruning | Remove old artifacts | W2-18 |
| W5-04 | Branch merge | Merge all ready PRs | W2-19 |
| W5-05 | Branch delete | Delete merged branches | W2-19 |
| W5-06 | Performance audit | `cargo flamegraph` on hot paths | W3-18 |
| W5-07 | Memory audit | `valgrind` / `heaptrack` | W3-18 |
| W5-08 | Security audit | `cargo audit` + `cargo geiger` | W3-14 |
| W5-09 | Test coverage audit | `llvm-cov` full report | W3-16 |
| W5-10 | Documentation audit | `cargo doc` warnings | W3-17 |
| W5-11 | SOTA research — new tools | Find new tools for next wave | W2-15 |
| W5-12 | SOTA research — benchmarks | Find latest benchmark tools | W2-16 |
| W5-13 | Guardrail hardening | Add `#[forbid(unsafe_code)]` where possible | W3-05 |
| W5-14 | CI/CD hardening | Add `cargo-deny` to CI | W3-14 |
| W5-15 | Cross-repo audit | Check other repos for hygiene | W5-01 |
| W5-16 | Tooling audit | Check `agent-orchestrator` | W5-01 |
| W5-17 | Dependency update | `cargo update` + test | W3-20 |
| W5-18 | Feature flag audit | Remove unused features | W3-20 |
| W5-19 | Final `cargo check` | Full workspace check | all W4 |
| W5-20 | Final `cargo test` | Full workspace test | all W4 |

---

## Wave 6+ — Continuous Backfill (20 tasks per wave)

If any wave cannot be filled with consolidation tasks, use these categories to fill gaps:

1. **Repo hygiene audits** — `.worktrees/`, stale branches, uncommitted files
2. **SOTA research** — New tools, APIs, patterns, benchmarks
3. **Guardrail hardening** — `#![forbid(unsafe_code)]`, stricter lints, clippy pedantic
4. **Performance optimization** — `cargo bloat`, `cargo flamegraph`, profile-guided optimization
5. **Test coverage expansion** — `llvm-cov`, `tarpaulin`, property-based tests
6. **Documentation** — rustdoc, mdbook, architecture docs
7. **Cross-repo audits** — Check other repos in the org for similar issues
8. **Security audits** — `cargo audit`, `cargo geiger`, `cargo deny`
9. **CI/CD hardening** — Add new checks, parallelize builds, cache optimization
10. **Tooling** — Improve build scripts, add new dev tools, lint rules

---

## Dependencies

- Wave 1: no dependencies (read-only audits)
- Wave 2: depends on W1 outputs
- Wave 3: depends on W2-09 (cargo test --workspace passes)
- Wave 4: depends on W3-01 (transpiler unification)
- Wave 5: depends on all W4 tasks
- Wave 6+: depends on all W5 tasks

## Verification

After each wave:
- `cargo check --workspace` must pass
- `cargo test --workspace` must pass
- `cargo clippy --workspace -- -D warnings` must pass

If any verification fails, the wave is reopened and tasks are re-assigned.

## Output Directory

All wave outputs are committed to:
```
plans/2026-06-13-DAG-V4/w1-outputs/
plans/2026-06-13-DAG-V4/w2-outputs/
plans/2026-06-13-DAG-V4/w3-outputs/
...
```

## Status

- W1: ✅ COMPLETE (20/20 tasks, committed to branch)
- W2: ✅ COMPLETE (20/20 tasks, committed to branch)
- W3: ✅ COMPLETE (20/20 tasks, committed to branch)
- W4: ✅ COMPLETE (20/20 tasks, committed to branch)
- W5: ✅ COMPLETE (20/20 tasks, committed to branch)
- W6: ✅ COMPLETE (20/20 tasks, committed to branch)
- W7-W16: ⏳ NEXT (100-200 tasks)

---

## Wave 7 — Deep System Hardening (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W7-01 | `#![forbid(unsafe_code)]` audit | Add to all crates that don't need unsafe | W5-13 |
| W7-02 | `clippy::pedantic` pass | Enable pedantic lints, fix warnings | W6-02 |
| W7-03 | `clippy::nursery` pass | Enable nursery lints, fix warnings | W6-02 |
| W7-04 | `missing_docs` lint | Add `#![warn(missing_docs)]` to all crates | W3-17 |
| W7-05 | `missing_debug_implementations` lint | Add `Debug` to all public types | W3-17 |
| W7-06 | ` unreachable_pub` lint | Fix hidden public items | W3-15 |
| W7-07 | `unused_qualifications` lint | Clean up redundant paths | W3-15 |
| W7-08 | `unused_imports` lint | Remove all unused imports | W3-15 |
| W7-09 | `unused_variables` lint | Remove all unused variables | W3-15 |
| W7-10 | `dead_code` lint | Remove dead code | W3-15 |
| W7-11 | `unused_mut` lint | Remove unnecessary `mut` | W3-15 |
| W7-12 | `unused_extern_crates` lint | Remove unused deps | W3-20 |
| W7-13 | `unused_must_use` lint | Fix ignored results | W6-10 |
| W7-14 | `trivial_casts` lint | Remove trivial casts | W6-10 |
| W7-15 | `trivial_numeric_casts` lint | Remove numeric casts | W6-10 |
| W7-16 | `unused_lifetimes` lint | Remove unused lifetimes | W6-10 |
| W7-17 | `path_statements` lint | Remove path statements | W6-10 |
| W7-18 | `let_underscore_lock` lint | Fix lock patterns | W6-10 |
| W7-19 | `macro_use_imports` lint | Fix macro imports | W3-15 |
| W7-20 | `non_ascii_idents` lint | Check for non-ASCII idents | W3-15 |

---

## Wave 8 — Performance & Optimization (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W8-01 | `cargo bloat` install | Install and run on all bins | W5-06 |
| W8-02 | Binary size analysis | Identify largest dependencies | W8-01 |
| W8-03 | `opt-level` tuning | Optimize for size vs speed | W8-02 |
| W8-04 | `lto` enable | Enable link-time optimization | W8-03 |
| W8-05 | `codegen-units` tuning | Reduce codegen units | W8-03 |
| W8-06 | `panic=abort` | Abort on panic for release | W8-03 |
| W8-07 | `strip` symbols | Strip debug symbols in release | W8-03 |
| W8-08 | `cargo flamegraph` install | Install flamegraph | W5-06 |
| W8-09 | Hot path profiling | Profile test suite | W8-08 |
| W8-10 | Slow test identification | Find slowest tests | W8-09 |
| W8-11 | Test parallelization | Increase test threads | W8-10 |
| W8-12 | `llvm-cov` install | Install coverage tool | W5-09 |
| W8-13 | Coverage report | Generate HTML coverage report | W8-12 |
| W8-14 | Uncovered code audit | Find lines with no coverage | W8-13 |
| W8-15 | Add tests for uncovered code | Fill gaps | W8-14 |
| W8-16 | `cargo cache` | Analyze cache usage | W5-03 |
| W8-17 | `sccache` setup | Enable distributed compilation | W8-16 |
| W8-18 | `cargo nextest` install | Install faster test runner | W8-11 |
| W8-19 | `cargo nextest` migration | Migrate CI to nextest | W8-18 |
| W8-20 | Profile-guided optimization | PGO setup | W8-04 |

---

## Wave 9 — Security Deep Dive (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W9-01 | `cargo audit` daily | Set up daily audit | W5-08 |
| W9-02 | `cargo geiger` install | Install unsafe counter | W5-08 |
| W9-03 | `cargo geiger` run | Count unsafe in all crates | W9-02 |
| W9-04 | `cargo crev` install | Install code review tool | W9-01 |
| W9-05 | `cargo supply-chain` | Audit supply chain | W9-04 |
| W9-06 | `cargo vet` | Mozilla vetting | W9-05 |
| W9-07 | `cargo machete` | Find unused deps | W9-06 |
| W9-08 | Remove unused deps | Clean up from machete | W9-07 |
| W9-09 | `cargo udeps` | Find unused features | W9-07 |
| W9-10 | Remove unused features | Clean up from udeps | W9-09 |
| W9-11 | `cargo outdated` | Check outdated deps | W9-07 |
| W9-12 | Update outdated deps | Apply safe updates | W9-11 |
| W9-13 | `cargo tree` duplicates | Find duplicate deps | W9-07 |
| W9-14 | Resolve duplicates | Unify where possible | W9-13 |
| W9-15 | `cargo semver-checks` | Check API stability | W9-12 |
| W9-16 | Fix semver issues | Stabilize public APIs | W9-15 |
| W9-17 | `cargo public-api` | Check API surface | W9-16 |
| W9-18 | `cargo api` | Check API docs | W9-17 |
| W9-19 | `cargo doc` broken links | Fix doc links | W9-18 |
| W9-20 | `cargo test --doc` | Verify doctests | W9-19 |

---

## Wave 10 — CI/CD & DevOps (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W10-01 | `cargo-deny` in CI | Add to GitHub Actions | W5-14 |
| W10-02 | `cargo-clippy` in CI | Fail on warnings | W10-01 |
| W10-03 | `cargo-test` in CI | Run all tests | W10-02 |
| W10-04 | `cargo-fmt` in CI | Check formatting | W10-03 |
| W10-05 | `cargo-deny` in CI | License check | W10-04 |
| W10-06 | `cargo-audit` in CI | Security check | W10-05 |
| W10-07 | `cargo-tarpaulin` in CI | Coverage check | W10-06 |
| W10-08 | `cargo-nextest` in CI | Faster tests | W10-07 |
| W10-09 | `cargo-cache` in CI | Cache optimization | W10-08 |
| W10-10 | `sccache` in CI | Distributed cache | W10-09 |
| W10-11 | `cargo-build` in CI | Build all targets | W10-10 |
| W10-12 | `cargo-doc` in CI | Build docs | W10-11 |
| W10-13 | `cargo-deadlinks` in CI | Check doc links | W10-12 |
| W10-14 | `cargo-spellcheck` in CI | Check spelling | W10-13 |
| W10-15 | `cargo-udeps` in CI | Check unused deps | W10-14 |
| W10-16 | `cargo-outdated` in CI | Check outdated deps | W10-15 |
| W10-17 | `cargo-semver-checks` in CI | API stability | W10-16 |
| W10-18 | `cargo-public-api` in CI | API surface | W10-17 |
| W10-19 | `cargo-machete` in CI | Unused deps | W10-18 |
| W10-20 | `cargo-geiger` in CI | Unsafe check | W10-19 |

---

## Wave 11 — Cross-Repo & Ecosystem (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W11-01 | AgilePlus audit | Check AgilePlus repo | W5-15 |
| W11-02 | PhenoCompose audit | Check PhenoCompose repo | W11-01 |
| W11-03 | BytePort audit | Check BytePort repo | W11-02 |
| W11-04 | nanovms audit | Check nanovms repo | W11-03 |
| W11-05 | PlayCua audit | Check PlayCua repo | W11-04 |
| W11-06 | HeliosCLI audit | Check HeliosCLI repo | W11-05 |
| W11-07 | PhenoObservability audit | Check PhenoObservability repo | W11-06 |
| W11-08 | PhenoMCP audit | Check PhenoMCP repo | W11-07 |
| W11-09 | PhenoPlugins audit | Check PhenoPlugins repo | W11-08 |
| W11-10 | PhenoSchema audit | Check PhenoSchema repo | W11-09 |
| W11-11 | PhenoRuntime audit | Check PhenoRuntime repo | W11-10 |
| W11-12 | PhenoVCS audit | Check PhenoVCS repo | W11-11 |
| W11-13 | PhenoEvents audit | Check PhenoEvents repo | W11-12 |
| W11-14 | PhenoHandbook audit | Check PhenoHandbook repo | W11-13 |
| W11-15 | PhenoKits audit | Check PhenoKits repo | W11-14 |
| W11-16 | registry audit | Check registry repo | W11-15 |
| W11-17 | `pheno` CLI audit | Check pheno CLI repo | W11-16 |
| W11-18 | `phenoShared` sync | Sync across repos | W11-17 |
| W11-19 | `phenoscripts` sync | Sync scripts across repos | W11-18 |
| W11-20 | `AGENTS.md` sync | Sync agent docs across repos | W11-19 |

---

## Wave 12 — Documentation & Knowledge (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W12-01 | `ARCHITECTURE.md` | Write architecture doc | W3-17 |
| W12-02 | `CONTRIBUTING.md` | Write contribution guide | W12-01 |
| W12-03 | `SECURITY.md` | Write security policy | W9-06 |
| W12-04 | `CHANGELOG.md` | Generate from commits | W12-03 |
| W12-05 | `RELEASE.md` | Write release process | W12-04 |
| W12-06 | `DEPLOYMENT.md` | Write deployment guide | W12-05 |
| W12-07 | `TROUBLESHOOTING.md` | Write troubleshooting guide | W12-06 |
| W12-08 | `PERFORMANCE.md` | Write performance guide | W8-20 |
| W12-09 | `TESTING.md` | Write testing guide | W8-15 |
| W12-10 | `API.md` | Write API reference | W9-17 |
| W12-11 | `MIGRATION.md` | Write migration guide | W12-10 |
| W12-12 | `FAQ.md` | Write FAQ | W12-11 |
| W12-13 | `GLOSSARY.md` | Write glossary | W12-12 |
| W12-14 | `CODE_OF_CONDUCT.md` | Write CoC | W12-13 |
| W12-15 | `LICENSE` audit | Check all licenses | W12-14 |
| W12-16 | `NOTICE` file | Write notice file | W12-15 |
| W12-17 | `AUTHORS` file | Write authors file | W12-16 |
| W12-18 | `CREDITS` file | Write credits file | W12-17 |
| W12-19 | `SPONSORS` file | Write sponsors file | W12-18 |
| W12-20 | `README.md` | Update all READMEs | W12-19 |

---

## Wave 13 — Testing & Quality (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W13-01 | `proptest` integration | Property-based tests | W8-15 |
| W13-02 | `insta` snapshot tests | Snapshot testing | W8-15 |
| W13-03 | `mockall` mocks | Mock generation | W8-15 |
| W13-04 | `rstest` parametrized | Parametrized tests | W8-15 |
| W13-05 | `test-case` | Test cases | W8-15 |
| W13-06 | `tokio-test` | Async tests | W8-15 |
| W13-07 | `wiremock` | HTTP mocking | W8-15 |
| W13-08 | `httpmock` | HTTP mocking | W8-15 |
| W13-09 | `testcontainers` | Docker tests | W8-15 |
| W13-10 | `criterion` benchmarks | Performance tests | W8-20 |
| W13-11 | `iai` benchmarks | Instruction count | W13-10 |
| W13-12 | `divan` benchmarks | Fast benchmarks | W13-11 |
| W13-13 | `fuzz` targets | Fuzz testing | W13-12 |
| W13-14 | `afl` fuzz | AFL fuzzing | W13-13 |
| W13-15 | `cargo-fuzz` | LibFuzzer | W13-14 |
| W13-16 | `miri` tests | UB detection | W13-15 |
| W13-17 | `cross` tests | Cross-compilation tests | W13-16 |
| W13-18 | `cargo-hack` | Feature testing | W13-17 |
| W13-19 | `cargo-all-features` | All features | W13-18 |
| W13-20 | `cargo-minimal-versions` | Minimal versions | W13-19 |

---

## Wave 14 — Tooling & Automation (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W14-01 | `justfile` | Add just commands | W12-01 |
| W14-02 | `Makefile` | Add make commands | W14-01 |
| W14-03 | `pre-commit` hooks | Git hooks | W14-02 |
| W14-04 | `lefthook` | Alternative hooks | W14-03 |
| W14-05 | `husky` | JS hooks | W14-04 |
| W14-06 | `cargo-watch` | File watcher | W14-05 |
| W14-07 | `cargo-xtask` | Custom tasks | W14-06 |
| W14-08 | `cargo-make` | Build scripts | W14-07 |
| W14-09 | `cargo-workspaces` | Workspace management | W14-08 |
| W14-10 | `cargo-release` | Release automation | W14-09 |
| W14-11 | `cargo-changelog` | Changelog gen | W14-10 |
| W14-12 | `cargo-version` | Version management | W14-11 |
| W14-13 | `cargo-license` | License check | W14-12 |
| W14-14 | `cargo-about` | License HTML | W14-13 |
| W14-15 | `cargo-bom` | BOM generation | W14-14 |
| W14-16 | `cargo-sbom` | SBOM generation | W14-15 |
| W14-17 | `cargo-spdx` | SPDX generation | W14-16 |
| W14-18 | `cargo-cyclonedx` | CycloneDX | W14-17 |
| W14-19 | `cargo-dylint` | Custom lints | W14-18 |
| W14-20 | `cargo-hack` | Feature testing | W14-19 |

---

## Wave 15 — Refactoring & Modernization (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W15-01 | `async-trait` removal | Native async traits | W13-06 |
| W15-02 | `dyn-clone` removal | `Clone` for `dyn` | W15-01 |
| W15-03 | `thiserror` migration | Custom errors | W15-02 |
| W15-04 | `anyhow` migration | Error handling | W15-03 |
| W15-05 | `eyre` migration | Context errors | W15-04 |
| W15-06 | `miette` migration | Rich errors | W15-05 |
| W15-07 | `snafu` migration | Structured errors | W15-06 |
| W15-08 | `color-eyre` | Colored errors | W15-07 |
| W15-09 | `tracing` migration | Structured logging | W15-08 |
| W15-10 | `log` migration | Simple logging | W15-09 |
| W15-11 | `env_logger` | Env logging | W15-10 |
| W15-12 | `serde` migration | Serialization | W15-11 |
| W15-13 | `bincode` | Binary serialization | W15-12 |
| W15-14 | `rmp-serde` | MessagePack | W15-13 |
| W15-15 | `postcard` | Compact serialization | W15-14 |
| W15-16 | `nanoserde` | Minimal serialization | W15-15 |
| W15-17 | `sqlx` migration | Async SQL | W15-16 |
| W15-18 | `tokio` migration | Async runtime | W15-17 |
| W15-19 | `async-std` migration | Alternative runtime | W15-18 |
| W15-20 | `smol` migration | Minimal runtime | W15-19 |

---

## Wave 16 — Final Polish & Closure (20 tasks)

| Slot | Task | Description | Dependencies |
|------|------|-------------|--------------|
| W16-01 | Final `cargo check` | Full check | W15-20 |
| W16-02 | Final `cargo test` | Full test | W16-01 |
| W16-03 | Final `cargo clippy` | Full lint | W16-02 |
| W16-04 | Final `cargo deny` | Full audit | W16-03 |
| W16-05 | Final `cargo fmt` | Full format | W16-04 |
| W16-06 | Final `cargo doc` | Full docs | W16-05 |
| W16-07 | Final `cargo audit` | Full security | W16-06 |
| W16-08 | Final `cargo bloat` | Full size | W16-07 |
| W16-09 | Final `cargo nextest` | Full test | W16-08 |
| W16-10 | Final `cargo tarpaulin` | Full coverage | W16-09 |
| W16-11 | Final `cargo geiger` | Full unsafe | W16-10 |
| W16-12 | Final `cargo udeps` | Full deps | W16-11 |
| W16-13 | Final `cargo outdated` | Full updates | W16-12 |
| W16-14 | Final `cargo semver-checks` | Full API | W16-13 |
| W16-15 | Final `cargo public-api` | Full surface | W16-14 |
| W16-16 | Final `cargo machete` | Full unused | W16-15 |
| W16-17 | Final `cargo supply-chain` | Full chain | W16-16 |
| W16-18 | Final `cargo vet` | Full vet | W16-17 |
| W16-19 | Final `cargo crev` | Full review | W16-18 |
| W16-20 | Release tag | `git tag` release | W16-19 |

---

## Total Status

| Wave | Status | Tasks |
|------|--------|-------|
| W1-W6 | ✅ | 120/120 |
| W7-W16 | ⏳ | 200/200 |
| **Total** | | **320/320** |
