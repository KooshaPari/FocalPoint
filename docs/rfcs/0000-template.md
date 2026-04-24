# RFC NNNN: Title

**Author(s):** <name> (@github-handle)
**Date:** YYYY-MM-DD
**Status:** Proposed (Accepted / Declined / Implemented)
**Related Issue(s):** #NNN, #MMM
**Target Release:** v0.X.0 (or "unscheduled")

## Summary

One-paragraph summary of the proposal. What problem does this solve, and what is the high-level solution?

## Motivation

Why is this change needed? What problem(s) does it address?

- Describe the current pain point or limitation.
- Explain the impact on users, developers, or maintainers.
- Link to any GitHub issues or discussions.

## Design

Detailed explanation of the proposed solution.

### Changes to Public APIs

If this RFC changes any public trait, function, or data structure, explain:

- **What changes:** List the before/after signatures.
- **Why:** Justify each change.
- **Backward compatibility:** Will this break existing code? How will we migrate users?

Example:

```rust
// Before
pub trait Connector {
    fn emit(&self, event: Event) -> Result<()>;
}

// After
pub trait Connector {
    fn emit(&self, event: Event) -> Result<EmitReceipt>;
    fn config_schema(&self) -> JsonSchema; // NEW
}

// Migration: Existing connectors get a default impl for `config_schema`.
```

### Storage & Persistence

If this RFC affects the event store, audit chain, or connector state:

- Describe any schema changes.
- How will migration work for existing data?
- Are there any backward-compatibility concerns?

### Examples

Provide 1–3 code examples showing how the feature will be used.

```rust
// Example 1: Basic usage
let connector = MyConnector::new();
let schema = connector.config_schema();
println!("{:?}", schema);

// Example 2: Advanced case
for endpoint in connector.endpoints() {
    let resp = connector.fetch(endpoint).await?;
    // ...
}
```

## Drawbacks

What are the downsides of this proposal?

- Complexity added to the codebase.
- Performance impact (if any).
- Backward-compatibility breaks.
- Maintenance burden.
- User confusion (if the feature is unintuitive).

## Alternatives Considered

What other approaches did you consider, and why did you reject them?

### Alternative 1: <name>

Brief description. Why not this?

### Alternative 2: <name>

Brief description. Why not this?

## Unresolved Questions

What is still unclear or needs to be resolved?

- Open question 1.
- Open question 2.

(These can be addressed during implementation or in follow-up RFCs.)

## Decision

**Status:** Pending review.

(This section is filled in by the maintainers after the 14-day discussion period.)

---

## References

- Link to related RFCs.
- Link to related issues or discussions.
- Link to external specifications or standards (if any).

---

## Discussion

This RFC was proposed on [Date]. Discussion took place in [GitHub issue / discussion link] from [Date] to [Date].

**Feedback summary:**
- ✓ Agreement on core design.
- ? Open: How to handle edge case X?
- ✗ Concern: Y might be too complex.

(Fill this in as discussion happens.)
