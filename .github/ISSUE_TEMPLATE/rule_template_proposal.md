---
name: Rule template proposal
about: Propose a new sample rule pack.
title: "rule: <short name>"
labels: ["rule-template", "proposal"]
assignees: []
---

## Rule name

`<kebab-case-id>` — e.g. `assignment-driven-focus`, `sleep-debt-strict-morning`, `calendar-lock`.

## One-liner

<What does the rule do, in one sentence?>

## Why this helps

<Narrative: what behavior change does this encourage? Who for?>

## Required connectors

- `connector-<source>` — for `<event type>`
- `connector-<source>` — for `<event type>`

## Draft rule DSL

```toml
# Sketch — does not need to be final
[rule]
id = "..."
title = "..."
description = "..."

[[when]]
source = "..."
event = "..."
match = { ... }

[[then]]
action = "lock-apps"
targets = ["com.instagram", "com.tiktok"]
until = "..."
escalation = { ... }
```

## Example event stream (should fire)

```json
[
  { "kind": "...", "payload": { ... } },
  { "kind": "...", "payload": { ... } }
]
```

## Example event stream (should NOT fire)

```json
[
  { "kind": "...", "payload": { ... } }
]
```

## Reward / penalty shape

- Reward on compliance: `...`
- Penalty on violation: `...`
- Escalation tiers: `...`
- Bypass cost: `...`

## Edge cases

- DST / timezone boundaries: ...
- Connector auth lapse: ...
- Event arrives late: ...

## Acceptance

- [ ] DSL validates against the current rule schema
- [ ] Unit tests in `focus-rules/tests/` against the fixture stream
- [ ] Docs page under `docs-site/rules/templates/`
