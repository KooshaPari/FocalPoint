---
title: Testing Your Connector
description: Write unit tests, integration tests, and end-to-end tests for your connector.
---

# Testing Connectors

Every connector must include comprehensive tests before publication. FocalPoint provides testing tools and fixtures.

## Test Types

### Unit Tests

Test individual components in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assignment_json() {
        let json = r#"{
            "id": "123",
            "title": "Midterm",
            "due_at": "2026-05-15T23:59:59Z"
        }"#;

        let assignment = parse_assignment(json)?;
        assert_eq!(assignment.id, "123");
        assert_eq!(assignment.title, "Midterm");
    }

    #[test]
    fn test_calculate_hours_until_due() {
        let due_at = "2026-04-23T14:00:00Z";
        let now = "2026-04-23T10:00:00Z";
        
        let hours = hours_until_due(due_at, now);
        assert_eq!(hours, 4);
    }
}
```

### Integration Tests

Test connector interactions with mocked services:

```rust
#[tokio::test]
async fn test_fetch_assignments() {
    let mock_canvas = MockCanvas::new();
    let connector = CanvasConnector::new(mock_canvas);
    
    let assignments = connector.fetch_assignments().await?;
    
    assert_eq!(assignments.len(), 5);
    assert!(assignments[0].due_at.is_some());
}
```

### End-to-End Tests

Test full workflow with test fixtures:

```bash
focalpoint connector test   --manifest connector.toml   --auth-token "test_token_abc123"   --mock-events test-fixtures/events.json
```

## Test Fixtures

Use fixtures for repeatable test data:

```json
{
  "assignments": [
    {
      "id": "123",
      "title": "Midterm Project",
      "due_at": "2026-05-15T23:59:59Z",
      "points_possible": 100
    }
  ]
}
```

## Running Tests Locally

```bash
# Unit + integration tests
cargo test --manifest-path crates/connector-canvas/Cargo.toml

# Full test suite
cargo test --workspace

# With coverage
cargo tarpaulin --out Html --output-dir coverage/
```

## Test Coverage Requirements

- **Minimum**: 70% line coverage
- **Goal**: 85%+ with critical path 100%

Critical paths:

1. OAuth token exchange
2. Event emission
3. Error handling
4. Permission validation

## Continuous Integration

Your connector will be tested in CI:

```bash
# Runs on every PR
cargo clippy -- -D warnings
cargo fmt --check
cargo test --workspace
```

See [Verification Criteria](../governance/verification) for publishing requirements.
