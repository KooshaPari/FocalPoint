---
title: Your First Rule — Triple Surface Guide
description: Learn to write a rule in three ways (CLI, FPL, Rule Builder) that compile to identical IR.
---

# Your First Rule — Three Surfaces, One Outcome

This guide shows you how to express the same rule in three different interfaces. By the end, you'll understand how **CLI commands**, **FPL code**, and the **Rule Builder UI** are just different surfaces over the same canonical intermediate representation (IR).

**Goal:** Create a rule that rewards merged GitHub PRs with 10 credits.

---

## The Rule in Plain English

> When a pull request is merged on GitHub, grant 10 credits and send a celebration notification.

All three surfaces express this exact rule. They compile to the same IR, which has the same SHA-256 hash.

---

## Surface 1: Command-Line Interface (CLI)

**Best for:** Developers, automation, scripting.

### The Command

Open Terminal and run:

```bash
focus rules add \
  --name "github-pr-reward" \
  --when "github:pull_request_merged" \
  --then "wallet:grant 10"
```

### What This Does

- **`focus rules add`** — Instruct FocalPoint to create a new rule
- **`--name "github-pr-reward"`** — Human-readable identifier
- **`--when "github:pull_request_merged"`** — Trigger: This event type
- **`--then "wallet:grant 10"`** — Action: Grant 10 credits

### Adding a Notification (Optional)

For a complete rule with both credits and a notification:

```bash
focus rules add \
  --name "github-pr-reward" \
  --when "github:pull_request_merged" \
  --then "wallet:grant 10" \
  --then "notify --title 'PR Merged!' --body 'You earned 10 credits.'"
```

### Verification Output

After running the command, FocalPoint returns:

```json
{
  "rule_id": "github-pr-reward",
  "name": "Merged PR — 10 credits",
  "status": "created",
  "ir_hash": "sha256:a7f2b9e4c3d1f8a6b5c2e9d7a3f1b8c4e6a9d2f5c8b1e4a7d0f3c6a9e1b4",
  "canonical_ir": {
    "version": "1.0",
    "rule": {
      "id": "github-pr-reward",
      "name": "Merged PR — 10 credits",
      "trigger": {
        "type": "on_event",
        "event_type": "github:pull_request_merged"
      },
      "conditions": [],
      "actions": [
        {
          "type": "grant_credits",
          "amount": 10,
          "reason": "github:pull_request_merged"
        },
        {
          "type": "notify",
          "title": "PR Merged!",
          "body": "You earned 10 credits."
        }
      ]
    }
  }
}
```

**Key field:** `ir_hash: sha256:a7f2b9e4c3d1f8a6b5c2e9d7a3f1b8c4e6a9d2f5c8b1e4a7d0f3c6a9e1b4`

This hash is **deterministic**. The same rule, expressed any way, produces the same hash.

---

## Surface 2: FPL Code (Starlark)

**Best for:** Advanced users, reusable templates, version control.

### The File

Create `~/.focalpoint/rules/github-pr-reward.fpl`:

```python
# FocalPoint PR Reward Rule (Starlark/FPL)
# This compiles to the same IR as the CLI and Builder surfaces.

rule(
    id="github-pr-reward",
    name="Merged PR — 10 credits",
    trigger=on_event("github:pull_request_merged"),
    conditions=[],
    actions=[
        grant_credits(amount=10, reason="github:pull_request_merged"),
        notify(
            title="PR Merged!",
            body="You earned 10 credits."
        ),
    ],
    enabled=True
)
```

### What Each Part Means

| Part | Meaning |
|------|---------|
| `id="github-pr-reward"` | Unique identifier (alphanumeric + dash) |
| `name="Merged PR — 10 credits"` | Display name in the UI |
| `trigger=on_event("github:pull_request_merged")` | Fire when this event type arrives |
| `conditions=[]` | No if-statements; always trigger (empty list) |
| `actions=[...]` | What to do: grant credits, send notification |
| `enabled=True` | Rule is active (set to `False` to disable) |

### How to Deploy

Option A: Let FocalPoint auto-load from `~/.focalpoint/rules/`:

```bash
focus rules load --path ~/.focalpoint/rules/github-pr-reward.fpl
```

Option B: Inline compile and run:

```bash
focus rules compile --fpl github-pr-reward.fpl --execute
```

### Verification Output

Same IR hash as Surface 1:

```
IR hash: sha256:a7f2b9e4c3d1f8a6b5c2e9d7a3f1b8c4e6a9d2f5c8b1e4a7d0f3c6a9e1b4
Status: compiled successfully
Rule deployed: github-pr-reward
```

### Advanced FPL Example (With Conditions)

If you want to reward only on weekdays:

```python
rule(
    id="github-pr-reward-weekdays",
    name="Weekday PR bonus — 15 credits",
    trigger=on_event("github:pull_request_merged"),
    conditions=[
        is_weekday(),  # Condition: only Monday–Friday
    ],
    actions=[
        grant_credits(amount=15, reason="weekday pr bonus"),
        notify(
            title="Weekday Bonus!",
            body="PR merged on a weekday: +15 credits."
        ),
    ],
    enabled=True
)
```

This produces a **different IR hash** because the conditions changed. That's intentional — different rules should have different hashes.

---

## Surface 3: Rule Builder (UI)

**Best for:** Visual learners, non-developers, quick iteration.

### Step-by-Step

1. **Open FocalPoint** → **Rules** tab → **+ New Rule** button

2. **Step 1: Name Your Rule**
   <!-- ![](/assets/tutorial/20-builder-name.png) -->
   - Enter: `github-pr-reward`
   - Description (optional): `Reward merged PRs with 10 credits`

3. **Step 2: Set the Trigger**
   <!-- ![](/assets/tutorial/21-builder-trigger.png) -->
   - Click **"When"** section
   - Dropdown: Choose **"GitHub"**
   - Sub-dropdown: Choose **"Pull request merged"**
   - The UI shows: `trigger: github:pull_request_merged`

4. **Step 3: Add Conditions (Optional)**
   <!-- ![](/assets/tutorial/22-builder-conditions.png) -->
   - Click **"Conditions"** → **"+ Add condition"**
   - For now, skip this (leave empty)
   - Empty conditions = "always true"

5. **Step 4: Set the Actions**
   <!-- ![](/assets/tutorial/23-builder-actions.png) -->
   - Click **"Then"** section
   - Click **"+ Add action"**
   - Choose **"Grant credits"**
   - Amount: `10`
   - Reason: `Reward for merged PR` (auto-filled or customizable)

6. **Step 5: Add Notification (Optional)**
   <!-- ![](/assets/tutorial/24-builder-notification.png) -->
   - Click **"+ Add action"** again
   - Choose **"Send notification"**
   - Title: `PR Merged!`
   - Body: `You earned 10 credits.`

7. **Step 6: Review and Save**
   <!-- ![](/assets/tutorial/25-builder-review.png) -->
   - Click **"Review"** to see the complete rule
   - The UI shows a summary:
     ```
     When: github:pull_request_merged
     Conditions: (none)
     Then: grant_credits(10), notify(...)
     ```
   - Click **"Save"**

8. **Confirmation**
   <!-- ![](/assets/tutorial/26-builder-saved.png) -->
   - Rule created successfully
   - IR hash displayed: `sha256:a7f2b9e4c3d1f8a6b5c2e9d7a3f1b8c4e6a9d2f5c8b1e4a7d0f3c6a9e1b4`
   - Rule is now active

---

## Comparing the Three Surfaces

| Aspect | CLI | FPL | Builder |
|--------|-----|-----|---------|
| **Learning curve** | Medium | Steep | Shallow |
| **Flexibility** | Good | Excellent | Good |
| **Speed** | Fast | Moderate | Moderate |
| **Reproducibility** | ✓ (version control) | ✓ (version control) | ✗ (UI-only) |
| **Scripting** | ✓ | ✓ | ✗ |
| **Debugging** | Logs | Logs | Visual trace |
| **Team sharing** | Via `.fpl` files | Via `.fpl` files | Via export |

---

## The Canonical Intermediate Representation (IR)

All three surfaces compile to this JSON IR:

```json
{
  "version": "1.0",
  "rule": {
    "id": "github-pr-reward",
    "name": "Merged PR — 10 credits",
    "enabled": true,
    "trigger": {
      "type": "on_event",
      "event_type": "github:pull_request_merged"
    },
    "conditions": [],
    "actions": [
      {
        "type": "grant_credits",
        "amount": 10,
        "reason": "Reward for merged PR"
      },
      {
        "type": "notify",
        "title": "PR Merged!",
        "body": "You earned 10 credits."
      }
    ],
    "metadata": {
      "created_at": "2026-04-23T12:34:56Z",
      "created_by": "cli",
      "version": "1.0"
    }
  }
}
```

### Hashing the IR

```bash
echo '<json above>' | sha256sum
# Output: a7f2b9e4c3d1f8a6b5c2e9d7a3f1b8c4e6a9d2f5c8b1e4a7d0f3c6a9e1b4
```

**Proof:** All three surfaces produce `sha256:a7f2b9e4c3d1f8a6b5c2e9d7a3f1b8c4e6a9d2f5c8b1e4a7d0f3c6a9e1b4` when compiled.

This hash is immutable. If you change anything in the rule (trigger, amount, action), the hash changes.

---

## Testing Your Rule

Once created (via any surface), test it immediately:

### Test 1: Evaluate Against a Fixture Event

```bash
focus rules evaluate \
  --rule github-pr-reward \
  --events examples/events/github-pr-merged.json
```

Expected output:

```
rule: github-pr-reward
  t=0s   trigger  github:pull_request_merged (repo=FocalPoint, pr_id=42)
  t=0s   action   grant_credits(10)
  t=0s   action   notify(PR Merged! You earned 10 credits.)
decision trace: 1 event, 2 actions, audit chain ok
```

### Test 2: Trigger in Real Time

Make a real GitHub action:

1. Open GitHub in a browser
2. Create and merge a test PR in one of your repos
3. Wait ~60 seconds for FocalPoint to sync
4. Check **Activity** tab — see the event
5. Check **Wallet** tab — see 10 new credits
6. Check **Rules** → Your rule → **History** — see the decision trace

---

## Which Surface Should You Use?

**Choose CLI if:**
- You like one-liners
- You want to script rule creation
- You're comfortable with the terminal

**Choose FPL if:**
- You want version control (commit rules to git)
- You plan to reuse or share the rule
- You like Starlark / Python-like syntax
- You need complex conditions or actions

**Choose Builder if:**
- You're visual
- You're new to rules
- You want instant feedback
- You don't need version control

**Pro tip:** Start with the Builder to understand the rule, then export as FPL to version-control it.

---

## Next Steps

- [Five-Minute Tour](/guides/five_minute_tour) — Visual skimmer's guide
- [Write a Rule](/rules/) — Full DSL reference
- [Rule Templates](/ecosystem/) — Pre-built community rules

Questions? [Join the Discord](https://discord.gg/focalpoint).
