# E2E Smoke Test Guide

## Overview

The FocalPoint E2E smoke test validates the full loop without iOS:
1. **Event Injection** — synthetic connector events appended to audit log
2. **Rule Evaluation** — rules fire against normalized events
3. **Wallet Mutation** — credits granted, streaks incremented
4. **Audit Verification** — hash chain integrity verified

All scenarios run in <30s with JSON reporting. No external services required.

## Running Locally

### One-Liner
```bash
cargo run -p focuspoint-e2e --bin smoke --release
```

### With JSON Output
```bash
cargo run -p focuspoint-e2e --bin smoke --release 2>&1 | tail -50
```

### Using the Runner Script
```bash
cargo run --manifest-path scripts/smoke-e2e.rs
```

## Scenarios Covered

### Scenario 1: GitHub PR Merged → Wallet Grant
**What it tests:**
- Event normalization (GitHub connector, pr_merged)
- Rule firing (grant 10 credits)
- Wallet state mutation (earned_credits += 10)
- Audit append (wallet.grant_credit record)
- Chain verification (prev_hash links, hash integrity)

**Assertions:**
- Initial balance == 0
- Balance after grant == 10
- Audit chain length == 1
- Audit record_type == "wallet.grant_credit"
- Chain verify passes

### Scenario 2: Focus Session Completed → Streak Increment
**What it tests:**
- Event trigger (focus:session_completed)
- Streak mutation (daily_focus += 1)
- Audit append (wallet.streak_increment record)

**Assertions:**
- Initial streaks count == 0
- Streak "daily_focus" created
- Streak count == 1
- Audit chain length == 1
- Audit record_type == "wallet.streak_increment"
- Chain verify passes

### Scenario 3: Multi-Mutation Chain Integrity
**What it tests:**
- Sequential mutations (grant → streak → grant)
- Chain linking (each record's prev_hash points correctly)
- Full chain verification after 3 appends
- Final wallet balance (15 credits)

**Assertions:**
- Chain length == 3
- Record 0: prev_hash == GENESIS
- Record 1: prev_hash == Record 0 hash
- Record 2: prev_hash == Record 1 hash
- Full chain verify passes
- Final balance == 15

### Scenario 4: Event Normalization
**What it tests:**
- Event creation (NormalizedEvent)
- Confidence validation [0.0, 1.0]
- Payload structure (JSON with required fields)
- Connector ID and event type fields

**Assertions:**
- Connector ID == "github"
- Event type == pr_merged
- Confidence in valid range
- Payload has required fields

## JSON Report Structure

```json
{
  "timestamp": "2026-04-24T12:34:56Z",
  "scenarios_passed": 4,
  "scenarios_failed": 0,
  "assertions": {
    "total": 24,
    "passed": 24,
    "failed": 0
  },
  "failures": [],
  "success": true,
  "elapsed_secs": 0.123
}
```

**Fields:**
- `timestamp`: Run time in ISO-8601
- `scenarios_passed` / `scenarios_failed`: Count of pass/fail test groups
- `assertions`: Individual assertion counters
- `failures`: Array of failed scenario names
- `success`: Boolean (true if all scenarios pass)
- `elapsed_secs`: Wall-clock runtime in seconds

## CI Behavior

The GitHub Actions workflow (`e2e-smoke.yml`):
- Runs on every push and pull request
- Uses standard Linux runner (no macOS/Windows billing)
- 2-minute timeout (test must finish in <30s)
- `continue-on-error: true` to unblock CI despite GitHub Actions billing issues
- Captures JSON report and emits to logs

**CI Job Definition:**
```yaml
- name: Run E2E Smoke Test
  run: cargo run -p focuspoint-e2e --bin smoke --release
  timeout-minutes: 2
  continue-on-error: true
```

## Interpreting Results

### All Pass ✓
```
Scenarios passed: 4
Scenarios failed: 0
Total assertions: 24
Passed: 24
Failed: 0
success: true
```

Exit code: **0**. Safe to merge.

### Failure ✗
```
Scenarios passed: 3
Scenarios failed: 1
Total assertions: 24
Passed: 22
Failed: 2
failures: ["focus_session_completed"]
success: false
```

Exit code: **1**. Debug the failed scenario (use `-RUST_LOG=debug` for more context).

### Timeout (CI only)
Test execution exceeded 30s. Check for:
- Slow SQLite operations (check db file location)
- Excessive logging (reduce tracing verbosity)
- CI runner overload (check GitHub Actions status)

## Extending the Test

### Adding a New Scenario

1. **Create a new function:**
   ```rust
   fn scenario_my_feature(results: &mut TestResults) -> Result<()> {
       println!("\nScenario N: My Feature");
       // Assertions and mutations...
       results.record_scenario("my_feature", scenario_passed);
       Ok(())
   }
   ```

2. **Call from main():**
   ```rust
   if let Err(e) = scenario_my_feature(&mut results) {
       results.scenarios_failed += 1;
       results.failures.push("my_feature: error".to_string());
   }
   ```

3. **Use mock audit sink and in-memory wallets** (no persistent state across scenarios).

### Mocking Storage

The test uses:
- `MockAuditSink` — in-memory append-only chain with SHA-256 verification
- In-memory `RewardWallet` — no persistence layer
- No EventStore/RuleStore/WalletStore (those are unit-tested in `crates/` tests)

For integration with the SQLite adapter, see `crates/focus-storage/src/sqlite/` tests.

## Troubleshooting

### Build Fails
```
error[E0433]: cannot find crate `focus_audit` in this edition
```
→ Ensure `tests/e2e/Cargo.toml` has path dependencies and repo-root `Cargo.toml` has NOT excluded the e2e dir.

### Runtime Panic: "unwrap" Failed
→ MockAuditSink mutex poisoned. Rebuild with `cargo clean`.

### Test Exceeds 30s
→ Run in release mode: `--release`. Debug mode is ~3x slower.

### Chain Verify Fails
→ Audit record payload was mutated or hash computation is non-deterministic. Check JSON serialization order.

## Next Steps

- **Unit tests:** Run `cargo test --workspace` for individual crate tests
- **Integration tests:** `crates/focus-eval/` has tests with mocked EventStore
- **iOS/Android tests:** Use `crates/focus-ffi` + native test harnesses (separate from this)

