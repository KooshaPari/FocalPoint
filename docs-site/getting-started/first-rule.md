---
title: First rule walkthrough
description: FocalPoint documentation
---
# First rule walkthrough

Goal: author a rule that locks Instagram when a Canvas assignment is due in less than four hours, and unlocks it the moment submission is detected.

## 1. Rule DSL

Create `examples/rules/assignment-focus.toml`:

```toml
[rule]
id = "assignment-driven-focus"
title = "Lock distractors when Canvas assignments are close to due"
description = """
For each Canvas assignment due within the next 4 hours, lock a configurable
list of social apps. Unlock on submission.
"""

[[when]]
# Trigger 1: assignment within the due window
source = "canvas"
event = "assignment.upcoming"
match = { hours_until_due = { "$lt" = 4 } }

[[unless]]
# Don't fire if the assignment is already submitted
source = "canvas"
event = "assignment.submitted"
match = { assignment_id = "$current" }

[[then]]
action = "lock-apps"
targets = [
  "com.burbn.instagram",
  "com.zhiliaoapp.musically",
  "com.atebits.Tweetie2",
]
until = { event = "canvas.assignment.submitted", assignment_id = "$current" }
escalation = { after = "30m", add_targets = ["com.reddit.Reddit", "com.google.ios.youtube"] }

[metadata]
required_connectors = ["canvas"]
verification_tier = "phenotype-verified"
```

## 2. Evaluate against a fixture stream

```bash
cargo run -p focus-cli -- rule evaluate \
  examples/rules/assignment-focus.toml \
  --events examples/events/canvas-assignment-due.json
```

Expected output:

```
rule: assignment-driven-focus
  t=0s   trigger  canvas.assignment.upcoming (hours_until_due=3.2)
  t=0s   action   lock-apps [com.burbn.instagram, com.zhiliaoapp.musically, com.atebits.Tweetie2]
  t=30m  escalate lock-apps +[com.reddit.Reddit, com.google.ios.youtube]
  t=2h   trigger  canvas.assignment.submitted (assignment_id=42)
  t=2h   action   unlock-apps [all]
decision trace: 4 events, 3 actions, audit chain ok
```

## 3. Inspect the decision trace on device

After deploying the rule, open FocalPoint → Rules → Assignment-driven focus → History. Every decision shows:

- The event that triggered it (`canvas.assignment.upcoming`, assignment_id, due date).
- The rule clauses that matched.
- The action taken and its audit hash.
- Expected duration (until next matching unlock event).

## 4. Tune

Common adjustments:

- **Window size.** Change `hours_until_due = { "$lt" = 4 }` to a stricter 2 hours or looser 8 hours.
- **Target apps.** Replace bundle ids. `com.apple.MobileSMS` is iMessage — usually a bad idea to block. `com.tinyspeck.chatlyio` is Slack — probably a good idea if you're studying.
- **Escalation.** Tune the `after` duration or add more tiers.

## 5. Commit as a rule template proposal

If your rule generalizes, open a [rule template proposal](https://github.com/KooshaPari/FocalPoint/issues/new?template=rule_template_proposal.md). Include:

- The TOML.
- An example event stream.
- Unit tests against the stream.
- A one-paragraph narrative for the docs-site.

Accepted templates ship under [`docs-site/rules/templates/`](/rules/samples).
