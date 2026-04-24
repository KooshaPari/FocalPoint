# Fuzzing Infrastructure

This document describes FocalPoint's fuzzing harnesses, corpus management, and crash triage procedures.

## Overview

Fuzzing validates IR surfaces for three critical invariants:

1. **Hash Stability**: RuleIr round-trips (JSON ↔ IR ↔ JSON) produce identical SHA-256 hashes
2. **Parse Robustness**: Starlark compiler never panics on adversarial inputs
3. **Audit Chain Verification**: AuditRecord sequences yield deterministic verification results

## Fuzzing Targets

### `ir_hash_stability`

**Purpose**: Verify that RuleIr serialization is deterministic and lossless.

**Invariant**:
```
serialize(rule) → deserialize → serialize
    ↓                              ↓
   hash1                         hash2
   hash1 == hash2 (ALWAYS)
```

**Run locally**:
```bash
cargo +nightly fuzz run ir_hash_stability -- -max_len=10000 -runs=10000
```

**Corpus seeds**:
- `deeply_nested_and.json` — deeply nested AND conditions (max depth)
- `long_rule_name.json` — 120+ char rule names
- `unicode_names.json` — emoji, CJK, RTL characters
- `max_int_delays.json` — i64::MAX values in delay fields

### `starlark_parse_invariants`

**Purpose**: Ensure FPL compiler never panics and produces stable IR hashes.

**Invariant**:
```
parse(adversarial_input)
├─ Must never panic (catch_unwind → OK)
└─ If succeeds: hash(IR) is stable across re-parse
```

**Run locally**:
```bash
cargo +nightly fuzz run starlark_parse_invariants -- -runs=10000
```

### `audit_chain_verify`

**Purpose**: Verify AuditRecord SHA-256 hash chains are deterministic.

**Invariant**:
```
chain_hash([entry1, entry2, entry3])
    == chain_hash([entry1, entry2, entry3])  (Same order, same hash)
```

**Run locally**:
```bash
cargo +nightly fuzz run audit_chain_verify -- -runs=5000
```

### `canonical_json`

**Purpose**: Validate that canonical JSON encoding is idempotent.

**Invariant**:
```
canonical(value) → deserialize → canonical
    ↓                              ↓
  hash1                          hash2
  hash1 == hash2
```

**Run locally**:
```bash
cargo +nightly fuzz run canonical_json -- -runs=10000
```

## CI Integration

Nightly fuzzing runs at **2 AM UTC** via `.github/workflows/fuzz.yml`.

**Configuration**:
- **Runs per target**: 10,000 executions
- **Max input size**: 10 KB
- **Timeout per input**: 10 seconds
- **Leak detection**: Enabled
- **Failure behavior**: `continue-on-error: true` (non-blocking; GH Actions billing constraint)

**Artifact collection**: Crashes and slow inputs uploaded to GitHub Actions artifacts.

## Crash Triage & Reproduction

### Step 1: Download Artifacts

1. Open GitHub Actions run for failed fuzzing job
2. Download `fuzz-crashes-<target>.zip`
3. Extract to local directory

### Step 2: Identify Crash

Crashes are stored in `artifacts/<target>/crash-<hash>`. Examine with:

```bash
cd fuzz
cargo +nightly fuzz run ir_hash_stability -- \
  artifacts/ir_hash_stability/crash-<hash> \
  -max_iterations=1
```

Expected output: reproduction of original crash + stack trace.

### Step 3: Minimize Crash

Use cargo-fuzz's built-in minimization:

```bash
cd fuzz
cargo +nightly fuzz cmin ir_hash_stability -- -max_len=500
```

This produces a minimal input in `corpus/ir_hash_stability/` that triggers the same crash.

### Step 4: Add to Corpus

Once minimized, commit the seed:

```bash
git add fuzz/corpus/ir_hash_stability/crash-minimize-<hash>.json
git commit -m "test(fuzz): corpus seed for ir_hash_stability regression"
```

### Step 5: Fix & Verify

1. Fix the underlying bug in focus-ir or focus-lang
2. Verify the crash no longer triggers:
   ```bash
   cargo +nightly fuzz run ir_hash_stability -- \
     corpus/ir_hash_stability/crash-minimize-<hash>.json
   ```
3. Re-run full fuzzing suite (or wait for nightly CI)

## Running Fuzzing Locally

### Quick Smoke Test (2 min)

```bash
cd fuzz
cargo +nightly fuzz run ir_hash_stability -- -max_iterations=1000
```

### Extended Run (1 hour)

```bash
cd fuzz
cargo +nightly fuzz run ir_hash_stability -- \
  -max_len=5000 \
  -runs=100000 \
  -timeout=5
```

### Parallel Targets (3 hours)

```bash
cd fuzz
for target in ir_hash_stability starlark_parse_invariants audit_chain_verify canonical_json; do
  cargo +nightly fuzz run "$target" -- -runs=50000 &
done
wait
```

## Corpus Management

### Adding New Seeds

Place hand-crafted inputs in `fuzz/corpus/<target>/`:

```bash
# Add a hand-crafted valid RuleIr
cat > fuzz/corpus/ir_hash_stability/my_seed.json <<'EOF'
{"id":"rule-x","name":"test",...}
EOF

# Fuzz will use it as a starting point
cargo +nightly fuzz run ir_hash_stability
```

### Pruning Corpus

Remove duplicate/redundant seeds:

```bash
cd fuzz
cargo +nightly fuzz cmin ir_hash_stability
```

This removes any corpus entries that don't add new coverage.

## Differential Testing

Run property-based tests for cross-surface invariants:

```bash
cargo test --test differential -p focus-ir -- --nocapture
```

Tests verify:
- Serde round-trip hash stability (multiple cycles)
- Nested condition serialization determinism
- Complex action sequences preserve hash
- Optional field consistency

## Monitoring & Alerts

### Slow Inputs

Fuzzer saves slow inputs to `artifacts/<target>/slow-<hash>`. Investigate with:

```bash
time cargo +nightly fuzz run ir_hash_stability -- \
  artifacts/ir_hash_stability/slow-<hash> \
  -max_iterations=1
```

Expected wall clock < 100 ms per execution. If slower, profile:

```bash
cargo build -p fuzz --release --bin ir_hash_stability
time target/release/ir_hash_stability artifacts/ir_hash_stability/slow-<hash>
```

### Leak Detection

Fuzzer detects memory leaks via AddressSanitizer. If a leak is found:

1. Check artifact: `artifacts/<target>/leak-<hash>`
2. Minimize: `cargo +nightly fuzz cmin <target>`
3. Reproduce with leak detector enabled:
   ```bash
   LSAN_OPTIONS=verbosity=1 cargo +nightly fuzz run <target> -- \
     corpus/<target>/minimized_leak
   ```

## Debugging Panics

If a fuzzing target panics:

1. Extract crash input:
   ```bash
   hexdump -C artifacts/ir_hash_stability/crash-<hash> | head -20
   ```

2. Add it to your test harness with debug logging:
   ```rust
   #[test]
   fn debug_crash() {
       let crash_input = std::fs::read("artifacts/ir_hash_stability/crash-<hash>").unwrap();
       // Fuzz logic here with println! or dbg!
   }
   ```

3. Run test with backtrace:
   ```bash
   RUST_BACKTRACE=1 cargo test debug_crash -- --nocapture
   ```

## References

- **cargo-fuzz book**: https://rust-fuzz.github.io/book/cargo-fuzz.html
- **libfuzzer**: https://llvm.org/docs/LibFuzzer/
- **Arbitrary crate**: https://docs.rs/arbitrary/
