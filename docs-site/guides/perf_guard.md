# Performance Guard

FocalPoint's performance regression detection system ensures critical benchmarks stay within tolerance.

## What It Does

The bench-guard tool monitors five criterion benchmarks across FocalPoint:

| Benchmark | Baseline | Purpose |
|-----------|----------|---------|
| `ir_hash/small` | 1ms | IR hash computation for small structures |
| `ir_hash/large` | 10ms | IR hash computation for large structures |
| `eval_tick` | 5ms | Single rule evaluation tick |
| `audit_verify/1k_tail` | 10ms | Audit chain verification (1k records) |
| `starlark_compile/small` | 50ms | Small Starlark program compilation |
| `starlark_compile/large` | 500ms | Large Starlark program compilation |
| `scheduler_packing/small` | 240µs | Task bin packing (50 tasks) |
| `scheduler_packing/medium` | 940µs | Task bin packing (100 tasks) |
| `scheduler_packing/large` | 1.4ms | Task bin packing (200 tasks) |

## Tolerance

All benchmarks allow a **30% regression threshold**. This tolerance captures normal variance from CI infrastructure, dependency updates, and algorithmic improvements that don't break the contract.

**Threshold formula:** `current_time ≤ baseline_time × 1.30`

If current time exceeds this, the check fails and blocks the PR.

## Reading the Output

When bench-guard runs on your PR, you'll see a markdown table in the PR comments:

```
| Benchmark | Baseline (ns) | Current (ns) | Change | Status |
|-----------|--------------|-------------|--------|--------|
| ir_hash/small | 1000000 | 1050000 | +5.0% | ✅ OK |
| eval_tick | 5000000 | 7200000 | +44.0% | ❌ REGRESSION |
```

- **✅ OK:** Within tolerance; no action needed.
- **❌ REGRESSION:** Exceeds threshold; investigate before merging.

## Investigating Regressions

1. **Is it real?** Run locally:
   ```bash
   cargo bench --workspace
   ```
   Compare your results to the table in the PR.

2. **Is it algorithmic?** If you changed the implementation:
   - Profile with `cargo flamegraph --bench <name>`
   - Check for new allocations, extra loops, or worse complexity

3. **Is it a fluke?** If the regression is <5% and you made no perf-critical changes:
   - Run again locally; CI variance is common
   - If consistent, investigate further

4. **Can you fix it?** Optimize and re-push. The workflow runs on every commit.

## Updating the Baseline

When a regression is **intentional** (e.g., adding features, fixing correctness bugs), update the baseline with approval:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint
cargo build -p bench-guard --release
./target/release/bench-guard --update-baseline
git add docs/reference/perf_baseline.json
git commit -m "perf: accept baseline regression due to <reason>"
```

**Important:** Only update after:
1. Benchmarking locally to confirm the new baseline is stable
2. Documenting in the commit message why the regression is acceptable
3. Getting explicit approval from the team (include in PR description)

## Histogram Mode

For observability integration, bench-guard emits Grafana-compatible histogram buckets:

```bash
./target/release/bench-guard --format=histogram
```

Output:
```
ir_hash/small:
  [0ns-200000ns]  ██
  [200000ns-400000ns]  ████
  ...
```

These buckets are 10-quantile divisions up to 2x the baseline mean. Use for dashboard widgets and latency alerting.

## Baseline Location

- **File:** `docs/reference/perf_baseline.json`
- **Format:** JSON with per-benchmark mean (ns) and histogram buckets
- **Owned by:** The team; changes require review
- **Locked during release:** No baseline updates after RC1

## Skipping the Check

If bench-guard fails due to CI infrastructure variance (not code changes):

1. Document the variance in the PR
2. Re-run the workflow from the Actions tab
3. If still fails, contact the team before merging with `gh pr merge --admin`
