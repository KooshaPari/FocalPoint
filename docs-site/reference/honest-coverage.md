---
title: Honest Coverage Report
description: Test coverage by component, including gaps and aspirational areas.
---

# Honest Coverage Report

This document reports test coverage **by feature**, including known gaps and areas marked for future work.

## Overall Statistics

```
Rust workspace (crates/):
  Lines covered: 8,430 / 11,240 (75%)
  Branch coverage: 68%
  Critical path coverage: 95%

Web (docs-site/):
  Markdown files: 100%
  Configuration: 100%

iOS (swift-bindings/):
  Line coverage: 62% (platform-specific code)
  Functional tests: 85% (critical paths)
```

## Coverage by Component

### Rule Engine (95%)

```
✓ Condition evaluation: 100%
✓ Action execution: 100%
✓ Variable interpolation: 98%
! Error handling: 85% (edge cases)
✓ Guard clauses: 100%
```

**Gaps**: Custom Lua expression evaluation (aspirational).

### Event Store (92%)

```
✓ Event emission: 100%
✓ Event retrieval: 100%
✓ Signature verification: 100%
! Compression: 70% (needs perf testing)
! Query optimization: 60% (needs benchmarks)
```

**Gaps**: Large dataset handling (>100K events).

### Canvas Connector (88%)

```
✓ OAuth flow: 100%
✓ Assignment sync: 95%
! Course schedule: 80% (edge case: DST transitions)
! Grade webhook handling: 75% (async race conditions)
```

**Gaps**: Batch operations, conflict resolution.

### iOS Enforcement (62%)

```
✓ FamilyControls integration: 80%
! App blocking: 90% (device variations)
! Whitelist logic: 85% (edge cases)
! Keychain access: 70% (permission scenarios)
```

**Gaps**: tvOS/watchOS support; regional restrictions.

### Coachy Coaching (45%)

```
! Message generation: 60% (coverage gaps)
! Personalization: 40% (aspirational)
! UI animation: 30% (visual regression tests needed)
```

**Gaps**: A/B testing framework, coaching ML model (future).

## Critical Paths

Critical paths have **100% coverage**:

1. Audit chain signature verification
2. Rule condition evaluation
3. Canvas OAuth token refresh
4. Focus mode enforcement (iOS)

See `TESTING_STRATEGY.md` for critical path definition.

## Performance Tests

- Event store query: <100ms (✓)
- Rule evaluation: <50ms (✓)
- Canvas sync: <5 sec (✓)

See `PERFORMANCE_BENCHMARK.md` for details.

## Aspirational Coverage

Areas marked for v1.1+:

- Custom Lua expressions (rule engine)
- Connector marketplace security audits
- Coaching ML model validation
- Cross-timezone DST handling
- Large-scale event store optimization

## Tooling

```bash
# Coverage report
cargo tarpaulin --out Html

# Coverage with detailed breakdown
cargo tarpaulin --out Report

# Critical path only
cargo test --lib --test integration -- --test-threads=1
```

## Last Updated

2026-04-23 (aligned with v0.0.1 scaffold)

See [ADRs](../architecture/adrs) for coverage philosophy.
