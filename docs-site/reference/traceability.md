---
title: Traceability Matrix
description: Map requirements to features to tests for verification.
---

# Traceability Matrix

This matrix tracks how every functional requirement maps to features and test cases.

## Format

| FR | Description | Feature | Test(s) | Status |
|----|-------------|---------|---------|--------|
| FR-001 | User can create rules | Rules wizard UI | test_rule_creation | ✓ Shipped |
| FR-002 | Rules evaluate conditions | Rule engine | test_condition_eval | ✓ Shipped |
| FR-003 | Rules execute actions | Rule execution | test_action_exec | ✓ Shipped |
| FR-004 | Canvas connector syncs assignments | Canvas integration | test_canvas_sync | ✓ Shipping |

## Requirements

See `FUNCTIONAL_REQUIREMENTS.md` for full FR list and acceptance criteria.

## Test Coverage

### Unit Tests

All features have >=1 unit test:

```bash
cargo test --workspace
```

### Integration Tests

Cross-system workflows tested:

- Canvas sync → Rule evaluation → Action execution
- Apple Health → Event emission → Coaching message
- Audit chain → Signature verification

### E2E Tests

Full user journeys on actual iOS device:

- Create rule, trigger event, verify block
- Open Canvas connector, sync assignment, verify notification
- Enable focus mode, verify app block, test whitelist

## Requirements Hierarchy

```
Tier 1: Core (rule engine, audit chain, local storage)
Tier 2: Shipping (Canvas connector, iOS enforcement)
Tier 3: Aspirational (advanced connectors, coaching ML)
Tier 4: Nice-to-have (integrations, performance optimizations)
```

## Test Maturity

Current state: **Level 3 (Component Testing)**

- ✓ Unit tests for components
- ✓ Integration tests for subsystems
- ⏳ E2E testing infrastructure
- ⏳ Performance benchmarks
- ⏳ Load testing

See `TESTING_STRATEGY.md` for roadmap.
