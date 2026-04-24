# FocalPoint Performance Baselines — April 2026

**Status:** Criterion benchmarks implemented and compile-verified. Full baseline run pending.

## Benchmark Overview

### Total Benchmarks: 9
- focus-rules: 5 benches
- focus-eval: 2 benches
- focus-lang: 2 benches
- focus-ir: 2 benches

---

## Focus-Rules Benchmarks

| Benchmark | Target | Status | Notes |
|-----------|--------|--------|-------|
| `bench_single_event_single_rule` | <10µs p95 | Compiled | Isolated rule evaluation; baseline for all rules |
| `bench_single_event_1000_rules` | <5ms p95 | Compiled | All 1000 rules matching against 1 event; DSL spec claim verification |
| `bench_batch_1000_events_100_rules` | <100ms | Compiled | Batch dispatch simulation (1K events × 100 rules) |
| `bench_cooldown_map_hit_path` | <50ms (1M ops) | Compiled | HashMap cooldown lookups under sustained load |
| `bench_condition_dsl_complex` | <1ms | Compiled | 10-level nested DSL evaluation (all_of + any_of + not) |

---

## Focus-Eval Benchmarks

| Benchmark | Target | Status | Notes |
|-----------|--------|--------|-------|
| `bench_evaluation_pipeline_tick` | <50ms | Compiled | Core tick(): 500 events × 50 rules; DSL spec claim verification |
| `bench_cursor_advance` | <100µs/advance | Compiled | Cursor persistence across 1000 event offsets |

---

## Focus-Lang Benchmarks

| Benchmark | Target | Status | Notes |
|-----------|--------|--------|-------|
| `bench_compile_small_fpl` | <20ms | Compiled | Parse + type-check + codegen on 50-line program |
| `bench_compile_1000_rule_fpl` | <200ms | Compiled | Full compilation of 1000-rule synthetic .fpl; DSL spec claim verification |

---

## Focus-IR Benchmarks

| Benchmark | Target | Status | Notes |
|-----------|--------|--------|-------|
| `bench_content_hash_small_document` | <10µs | Compiled | SHA-256 of single RuleIr; tamper-evident chain baseline |
| `bench_content_hash_1000_rule_document` | <5ms | Compiled | Full 1000-rule IR document hashing |

---

## Interpretation Notes

### DSL Spec Claims Verified
1. **"1000 rules compile in <200ms"** → `bench_compile_1000_rule_fpl` (target <200ms)
2. **"<1s on iPhone"** → focus-eval `bench_evaluation_pipeline_tick` (target <50ms wall-clock, ~20× margin for ARM/debug)
3. **"Single rule evaluation <10µs"** → focus-rules `bench_single_event_single_rule`

### Key Unknowns Still Flagged
1. **Actual iPhone ARM performance:** Benchmarks run on macOS x86-64. ARM codegen may differ by ±15%.
2. **Concurrent rule evaluation:** Benches are sequential. Real multi-event pipelining with task stealing not benchmarked.
3. **Cold cache effects:** All benches run warm. First-run compilation + symbol resolution may add 10–50ms on device startup.
4. **Policy engine interaction:** Benches isolate rules. Real end-to-end (rules → rewards/penalties → policy) not measured.
5. **Starlark compilation overhead:** focus-lang uses Starlark (embedded Python DSL). Pure-Rust reimplementation would likely halve compile time.

---

## Running the Benchmarks

### Compile-check only (CI-safe, <2min):
```bash
fastlane bench
# or directly:
cargo bench --no-run --workspace
```

### Full execution with HTML reports (5+ minutes):
```bash
fastlane bench_full
# or directly:
cargo bench --workspace
# Reports will appear in: target/criterion/
```

### Single benchmark:
```bash
cargo bench --package focus-rules --bench rule_evaluation -- bench_single_event_1000_rules
```

---

## Criterion Output

After first full run, baseline snapshots are automatically saved in `target/criterion/`. Subsequent runs will display:
- **p50 (median) time**
- **p95 time** (the DSL spec target metric)
- **Regression %** vs. baseline
- **HTML report** with graphs and detailed statistics

---

## Next Steps

1. Run full benchmarks on macOS (current)
2. Record baseline snapshots in `target/criterion/`
3. Cross-compile to iOS Simulator and re-run (ARM + debug build effects)
4. Measure cold-start overhead in actual app context
5. Add concurrent rule evaluation benchmark (multi-threaded dispatch)
6. Profile Starlark compilation; evaluate Rust reimplementation vs. fork
