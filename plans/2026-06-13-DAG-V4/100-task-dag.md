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
- W2: 🔄 IN PROGRESS (starting now)
- W3-W5: ⏳ PENDING
