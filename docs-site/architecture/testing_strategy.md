# Testing Strategy

FocalPoint employs a three-layer testing approach, each targeting a different boundary of the cross-platform Rust↔iOS integration.

## Test Layers

### Layer 1: Rust Unit Tests

**Scope:** Core domain logic, isolated subsystems.

**Invocation:**
```bash
cargo test --workspace
```

**Coverage:**
- Event sourcing (append-only log + tamper detection)
- Rule evaluation (condition matching, action dispatch)
- Wallet mutations (credit grant, spend, multiplier)
- Penalty tier escalation
- Task lifecycle state machines
- Audit chain verification (SHA-256 hash chains)

**Hermetic:** Yes. Uses in-memory stores (InMemoryAuditStore, InMemoryCalendarPort) + seeded fixtures. No network.

### Layer 2: iOS Snapshot Tests

**Scope:** UI component rendering, design system consistency.

**Invocation:**
```bash
cd apps/ios/FocalPoint
fastlane snapshot
```

**Coverage:**
- Enforcement view hierarchies
- Mascot UI state transitions
- Design system components (palette, typography, layout)
- Onboarding flow screens
- Diagnostics UI

**Hermetic:** Yes. Uses recorded snapshot images + synthetic mock data. No network, no real app state.

**First Run:** `RECORD=true fastlane snapshot` to record baseline snapshots. Add snapshots to version control.

### Layer 3: iOS Integration Tests

**Scope:** End-to-end Rust↔Swift FFI bridge; event pipeline; state mutations.

**Invocation:**
```bash
cd apps/ios/FocalPoint
fastlane integration
```

**Coverage:**
- Host event injection → rule evaluation → wallet credit → audit record
- Task CRUD operations + audit trail
- Audit chain tamper detection (SHA-256 verification)
- Connector registration (Canvas, Slack, GitHub)
- Simlish phoneme mapping (MascotUI voice synthesis)

**Test Target:** `FocalPointIntegrationTests` (XCTest bundle)

**Hermetic:** Yes. Uses temporary SQLite databases (created in FileManager tmpdir, cleaned up post-test). Sets env vars for secret store (memory-backed). No network (HTTP injection deferred to Phase 2).

**Tests:**

1. **test_focus_session_credits_wallet**
   - Traces to: FR-REWARD-001, FR-AUDIT-001
   - Creates core, installs `deep-work-starter` template, emits `focus:session_started` event, ticks sync + eval, verifies rule.fired audit record, asserts wallet balance > 0.

2. **test_task_lifecycle**
   - Traces to: FR-PLAN-001, FR-AUDIT-001
   - Adds task, lists, marks done, removes; verifies task.created, task.status_changed, task.deleted audit records at each step.

3. **test_audit_chain_verify**
   - Traces to: FR-AUDIT-002
   - Performs 10 mutations, verifyChain() → true; tampering via direct SQLite UPDATE, verifyChain() → false.

4. **test_connector_registration_after_connect**
   - Traces to: FR-SYNC-001
   - Sets env vars (FOCALPOINT_CANVAS_CLIENT_ID, etc.), calls connectCanvas() with fake credentials, asserts sync().connectors() contains "canvas".
   - **Skipped on Phase 1:** FFI HTTP client injection not yet available. Deferred to Phase 2.

5. **test_simlish_phoneme_mapping**
   - Traces to: FR-MASCOT-002
   - Verifies SimlishVoice helper deterministically maps input text to phoneme sequences.

## CI/CD Integration

### Local Development

```bash
# All three layers
cargo test --workspace
cd apps/ios/FocalPoint
fastlane snapshot
fastlane integration
```

### GitHub Actions (Future)

Once GitHub Actions billing is resolved:
- Rust unit tests on Linux runner (free tier) — blocks PR merge.
- iOS snapshot tests on macOS runner (billed) — informational; does not block.
- iOS integration tests on macOS runner (billed) — informational; does not block.

### Quality Gates

- **Pre-commit:** Rust clippy + fmt + test (local only; no shell escape).
- **Branch protection:** Rust unit tests must pass. iOS tests are informational.

## Skipped Tests & Rationale

### test_connector_registration_after_connect (Phase 2)

The current FFI surface does not expose HTTP client injection for testing. The Canvas connector integration requires mocking the OAuth callback + HTTP exchange. Deferral allows Phase 1 to ship without external HTTP mocking infrastructure.

**Unblock:** Implement `HttpClientPort` trait, expose injectable HTTP layer in FFI.

### test_audit_chain_verify SQLite Tamper (Partial)

Direct SQLite access for tamper simulation requires either:
- GRDB (Objective-C++ / Swift package) — adds dependency.
- sqlite3 C API — requires Bridging-Header.

Current implementation documents the intent; the test skips gracefully if SQLite direct access is unavailable (sandbox restriction or missing helper). Full implementation deferred to when a test SQLite helper is available.

## Performance Expectations

- **Rust unit tests:** ~30–60 seconds (full workspace).
- **iOS snapshots:** ~2–5 minutes (per-device variant).
- **iOS integration tests:** ~1–2 minutes (tempdir setup + 5 scenarios).

## Future Enhancements

1. **Deterministic Clock Injection:** Allow tests to inject a fixed SystemTime instead of wallclock; reduces flakiness in rule scheduling tests.
2. **HTTP Mock Layer:** Expose HttpClientPort in FFI; unblock connector integration tests.
3. **Snapshot Diffing:** Use `swift-snapshot-testing` visual diffs in CI to detect regressions.
4. **Load Testing:** Parameterized tests to stress-test audit chain hashing + wallet mutations at scale (10K+ records).
5. **Accessibility Testing:** XCUITest automation for accessibility audit (VoiceOver, Dynamic Type).

## Test Traceability

Every test is traced to a Functional Requirement (FR) in the spec:

```swift
// Traces to: FR-REWARD-001, FR-AUDIT-001
func testFocusSessionCreditsWallet() throws { ... }
```

Run the traceability audit:

```bash
grep -r "Traces to:" Tests/ | sort | uniq -c
```

All FRs must have ≥1 test; all tests must reference ≥1 FR.
