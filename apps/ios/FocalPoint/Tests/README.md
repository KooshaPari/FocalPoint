# FocalPoint Integration Tests

## Overview

This directory contains XCTest suites for FocalPoint, including:

- **Snapshot Tests** (`FocalPointAppSnapshotTests/`) — UI component rendering
- **Integration Tests** (`FocalPointIntegrationTests/`) — FFI + state machine end-to-end
- **Design System Tests** (`DesignSystemTests/`) — Design token validation
- **Unit Tests** (scattered) — Isolated behavior verification

## Integration Test Harness

### `CoreHolderE2ETests.swift`

Real end-to-end harness exercising CoreHolder + SQLite + FFI (Rust↔Swift bridge).

**Six scenarios:**

| Test | Traces | Coverage |
|------|--------|----------|
| `testOnboardingFullCycle` | FR-PLAN-001, FR-REWARDS-001, FR-DATA-002 | Task add → focus event → eval → mark done → audit trail |
| `testDemoSeedAndReset` | FR-DEMO-001, FR-REWARDS-001 | Seed demo data (tasks, rules, credits) → reset → empty state |
| `testRuleAddWebhookTriggerGrant` | FR-POLICY-001, FR-REWARDS-001, FR-DATA-002 | Add rule → GitHub webhook event → rule fire → wallet grant → audit |
| `testConnectorConnectAndSync` | FR-SYNC-001, FR-DATA-002 | Connector registration → OAuth stub → manual sync tick → audit events |
| `testWipeFlowAllTablesEmpty` | FR-DATA-001 | Full wipe → all tables cleared → wipe receipt generated |
| `testMultiDeviceIndependence` | FR-POLICY-001, FR-DATA-001 | Two distinct CoreHolder instances → no cross-contamination |

### Tempdir Handling

Each test:
1. **setUp()** allocates an isolated `tempDir` via `FileManager.temporaryDirectory`
2. Creates `core.db` inside the tempdir
3. **tearDown()** forcibly removes the entire tempdir via `removeItem()` (not just marking for deletion)

This prevents state leakage between tests and allows safe parallel execution.

```swift
override func setUp() {
    super.setUp()
    let testID = UUID().uuidString
    tempDir = FileManager.default.temporaryDirectory
        .appendingPathComponent("focalpoint-e2e-\(testID)", isDirectory: true)
    try FileManager.default.createDirectory(
        at: tempDir,
        withIntermediateDirectories: true,
        attributes: nil
    )
    coreDbPath = tempDir.appendingPathComponent("core.db").path
}

override func tearDown() {
    if let tempDir = tempDir {
        try? FileManager.default.removeItem(at: tempDir)
    }
    super.tearDown()
}
```

## Flaky Test Policy

### What is a flaky test?

A test that fails intermittently without code changes due to:
- Timing issues (async operations, timers, race conditions)
- Environmental dependencies (temp file cleanup, disk state)
- FFI marshalling edge cases (string encoding, pointer lifetimes)

### Handling Flaky Tests

1. **Identify**: Run the test in isolation 5+ times. If it fails <100%, it's flaky.
   ```bash
   for i in {1..5}; do fastlane integration; done
   ```

2. **Root Cause**: Inspect logs for:
   - `Thread\d+.*Terminated\|EXC_BAD_ACCESS` — FFI memory issue
   - `Broken pipe\|timeout` — Async operation exceeded timeout
   - `Permission denied\|ENOENT` — Tempdir cleanup failed

3. **Fix**: Apply one of:
   - **Increase timeout** (if operation is legitimately slow): use `expectation(timeout:)`
   - **Add synchronization** (if race condition): ensure all async work completes before assertion
   - **Retry tempdir cleanup** (if file system flake): add backoff in tearDown
   - **Isolate FFI boundary** (if marshalling issue): add bounds checking in Swift ↔ Rust

4. **Document**: Add comment to test with root cause and fix applied:
   ```swift
   /// Flaky 2026-04-23: Connector registration sometimes times out if sync() is slow.
   /// Fixed: Added 5s timeout expectation to allow sync to complete.
   func testConnectorConnectAndSync() throws {
       // ...
   }
   ```

5. **Monitor**: Re-run flaky test in CI 3-5 times per build to ensure fix is stable.

### Known Flaky Patterns

| Pattern | Cause | Fix |
|---------|-------|-----|
| `testConnectorConnectAndSync` skip | HTTP client injection unavailable | Phase 2 feature; expected to skip in dev |
| Tempdir cleanup `EACCES` | File locked by background process | Retry with 100ms backoff |
| FFI string encoding panics | Non-UTF8 in event properties | Sanitize JSON before emit |

## Timeout Conventions

### Standard Timeouts

| Operation | Timeout | Notes |
|-----------|---------|-------|
| Core init | 2 s | SQLite setup + audit chain verify |
| Task add/update | 1 s | Local DB write |
| Sync tick | 5 s | May poll connectors; allow HTTP latency |
| Eval tick | 3 s | Rule evaluation + audit sink |
| Wipe all | 10 s | Bulk delete + receipt file write |
| Tempdir cleanup | 2 s | Retry 3x with 100ms backoff if EACCES |

### Using Expectations

For async operations, wrap in `XCTWaiter`:

```swift
let expectation = expectation(description: "Sync completes")
DispatchQueue.global().async {
    _ = try core.sync().tick()
    expectation.fulfill()
}
wait(for: [expectation], timeout: 5.0)
```

Or use `withTimeout` helper (if available in your codebase):

```swift
try withTimeout(5.0) {
    _ = try core.sync().tick()
}
```

## Running Tests

### Locally (Xcode)

```bash
# Run all integration tests
xcodebuild test \
  -scheme FocalPointApp \
  -workspace FocalPoint.xcworkspace \
  -configuration Debug \
  -destination 'generic/platform=macOS,variant=Designed for iPad' \
  -only-testing:FocalPointIntegrationTests

# Run single test
xcodebuild test \
  -scheme FocalPointApp \
  -workspace FocalPoint.xcworkspace \
  -configuration Debug \
  -destination 'generic/platform=macOS,variant=Designed for iPad' \
  -only-testing:FocalPointIntegrationTests/CoreHolderE2ETests/testOnboardingFullCycle
```

### Via Fastlane

```bash
# Run integration tests (existing lane)
cd apps/ios/FocalPoint
fastlane integration

# Full CI pipeline (smoke + snapshot + integration)
fastlane ci
```

### Parallel Execution (Xcode 15+)

Xcode 15+ can parallelize test runs via `maxParallelizationLevel`. Each test's isolated tempdir allows safe parallel execution:

```bash
xcodebuild test \
  -scheme FocalPointApp \
  -workspace FocalPoint.xcworkspace \
  -configuration Debug \
  -maxParallelizationLevel 4 \
  -only-testing:FocalPointIntegrationTests
```

## CI Integration

The `fastlane integration` lane (defined in `fastlane/Fastfile`) runs:

```ruby
lane :integration do
  sh(
    "xcodebuild test " \
    "-scheme FocalPointApp " \
    "-workspace FocalPoint.xcworkspace " \
    "-configuration Debug " \
    "-destination 'generic/platform=macOS,variant=Designed for iPad' " \
    "-only-testing:FocalPointIntegrationTests"
  )
  UI.success("✅ Integration tests complete!")
end
```

This is automatically invoked by the CI lane:

```ruby
lane :ci do
  # ... cargo + clippy + fmt ...
  integration  # Integration tests run after Rust checks
end
```

## Debugging Test Failures

### Inspect Test Logs

Xcode saves detailed logs to:
```
~/Library/Developer/Xcode/DerivedData/
  FocalPoint-<hash>/Logs/Test/
    Run-FocalPointIntegrationTests-<timestamp>.log
```

### Enable SQLite Tracing

In tearDown or a test helper, enable SQLite SQL logging:

```bash
# Before running tests:
export SQLITE_TRACE=1
fastlane integration
```

### Preserve Tempdir on Failure

Temporarily disable tearDown cleanup to inspect the DB:

```swift
override func tearDown() {
    // Comment out for debugging:
    // try? FileManager.default.removeItem(at: tempDir)
    
    // Print path for manual inspection:
    print("DEBUG: Tempdir at \(tempDir?.path ?? "unknown")")
    super.tearDown()
}
```

Then inspect the DB:
```bash
sqlite3 /tmp/focalpoint-e2e-<uuid>/core.db
.schema
SELECT * FROM audit ORDER BY created_at DESC LIMIT 5;
```

## Best Practices

1. **Isolation First**: Each test should be runnable in any order.
2. **No Shared State**: Never use global variables or singletons (except CoreHolder's test instance).
3. **Tempdir Per Test**: Always allocate fresh tempdir in setUp.
4. **Cleanup Always**: Even if test fails, tearDown must run (use defer if needed).
5. **Descriptive Names**: Test name should describe the user scenario, not the implementation.
6. **Assert Intent, Not Details**: Test the outcome (wallet increased, audit record exists) not internal state (specific row IDs).
7. **Document Flaky Fixes**: Inline comment explaining why a test was flaky and how it was fixed.
8. **Trace to FR**: Link each test to Functional Requirements via `/// Traces to: FR-XXX-YYY` comment.

## Future Work

- [ ] **Phase 2**: HTTP client injection for real connector OAuth mocking
- [ ] **Phase 2**: Screenshot capture harness (already scaffolded in `fastlane/Fastfile`)
- [ ] **Phase 3**: Device cloud integration (BrowserStack, Firebase Test Lab)
- [ ] **Continuous**: Monitor flaky test metrics in CI; auto-rerun on transient failures

## References

- **Fastlane config**: `apps/ios/FocalPoint/fastlane/Fastfile`
- **Core FFI bindings**: `crates/focus-ffi/src/lib.rs`
- **Functional Requirements**: `FUNCTIONAL_REQUIREMENTS.md` (root of FocalPoint)
- **Test best practices**: https://developer.apple.com/documentation/xctest/

---

**Last updated:** 2026-04-24  
**Status:** Integration harness complete (6 scenarios, real SQLite, real FFI)  
**Maturity:** Beta (Phase 1 complete; Phase 2 features deferred)
