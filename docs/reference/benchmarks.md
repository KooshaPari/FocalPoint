# Criterion Benchmarks — FocalPoint

Baseline performance data for hottest code paths. Run locally during development to detect regressions before CI.

## Running Benchmarks

Run all benchmarks:
```bash
cargo bench --workspace
```

Run by crate:
```bash
cargo bench -p focus-ir
cargo bench -p focus-eval
cargo bench -p focus-audit
cargo bench -p focus-lang
```

Run a specific benchmark:
```bash
cargo bench -p focus-ir -- ir_hash
cargo bench -p focus-eval -- eval_tick
cargo bench -p focus-audit -- verify
cargo bench -p focus-lang -- starlark_compile
```

## Benchmarks

### 1. focus-ir: `ir_hash` (2 benches)

**Purpose**: Measure SHA-256 hashing performance of IR canonical-JSON representations.

**Targets**:
- Small document (single rule): `<10µs`
- 1000-rule document: `<5ms`

**Run**:
```bash
cargo bench -p focus-ir -- ir_hash
```

**Expected output**:
```
ir_hash/content_hash_small_document        time:   [~5 µs ... ~8 µs]
ir_hash/content_hash_1000_rule_document    time:   [~2 ms ... ~3 ms]
```

**Interpretation**:
- Linear scaling with document size is expected.
- Regressions >10% suggest changes to SHA-256 iteration or JSON canonicalization.

### 2. focus-eval: `eval_tick` (3 benches)

**Purpose**: Measure rule evaluation dispatch latency per event and per-rule-match.

**Targets**:
- 100 events × 50 rules: `<5ms` total (50µs per event, 1µs per match)
- Per-event latency (10-100 rules): `<50µs`

**Run**:
```bash
cargo bench -p focus-eval -- eval_tick
```

**Expected output**:
```
eval_tick/eval_tick_100_events_50_rules    time:   [~2 ms ... ~4 ms]
eval_tick/eval_tick_per_event/10_rules     time:   [~1 µs ... ~2 µs]
eval_tick/eval_tick_per_event/25_rules     time:   [~2 µs ... ~3 µs]
eval_tick/eval_tick_per_event/50_rules     time:   [~5 µs ... ~7 µs]
eval_tick/eval_tick_per_event/100_rules    time:   [~10 µs ... ~15 µs]
```

**Interpretation**:
- Latency should scale linearly with rule count.
- Regressions >15% suggest changes to rule iteration, condition evaluation, or action dispatch.

### 3. focus-audit: `verify` (2 benches)

**Purpose**: Measure audit chain verification throughput (entries/sec) and incremental verification cost.

**Targets**:
- Full chain (10k entries): `>10k entries/sec` (~1µs per entry)
- Incremental tail (1k entries): `<10ms`

**Run**:
```bash
cargo bench -p focus-audit -- verify
```

**Expected output**:
```
audit_verify_10k_entries                   time:   [~8 ms ... ~12 ms]
audit_verify_incremental_1k                time:   [~0.8 ms ... ~1.2 ms]
```

**Interpretation**:
- Throughput >10k entries/sec indicates good constant-factor performance.
- Regressions >20% suggest changes to SHA-256, hash comparison, or chain iteration.
- Incremental verification should be much faster than full (i.e., ~10% of 10k cost).

### 4. focus-lang: `starlark_compile` (2 benches)

**Purpose**: Measure Starlark→IR compilation latency for FPL programs of varying complexity.

**Targets**:
- 200-line program (20 rules): `<50ms`
- 2000-line program (200 rules): `<500ms`

**Run**:
```bash
cargo bench -p focus-lang -- starlark_compile
```

**Expected output**:
```
starlark_compile/starlark_compile_200_line   time:   [~10 ms ... ~20 ms]
starlark_compile/starlark_compile_2000_line  time:   [~80 ms ... ~150 ms]
```

**Interpretation**:
- Compilation should scale roughly linearly with rule count.
- Regressions >25% suggest changes to tokenization, AST construction, type checking, or codegen.
- If 2000-line exceeds 200ms, investigate parser or type checker bottlenecks.

## Regression Detection

Use Criterion's built-in thresholds:

```bash
# Run with verbose output to see baseline vs. current
cargo bench --workspace -- --verbose

# Criterion will warn on >5% regression by default
```

To set custom thresholds per benchmark, see `benches/*.rs` and adjust the `BenchmarkGroup` configuration.

## Adding New Benchmarks

1. Create a new file in `crates/<name>/benches/<name>.rs`
2. Add `[[bench]] name = "<name>", harness = false` to `Cargo.toml`
3. Ensure `criterion = { workspace = true }` is in `[dev-dependencies]`
4. Structure:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};

   fn bench_my_feature(c: &mut Criterion) {
       c.bench_function("my_feature", |b| {
           b.iter(|| {
               // Code to benchmark, wrapped in black_box()
           });
       });
   }

   criterion_group!(benches, bench_my_feature);
   criterion_main!(benches);
   ```
5. Document target latency and regression thresholds in this file.

## CI Integration

GitHub Actions runs benchmarks on `main` to detect regressions. See `.github/workflows/bench.yml` (future).

Currently, benchmarks run locally only — add CI integration when performance gates are finalized.
