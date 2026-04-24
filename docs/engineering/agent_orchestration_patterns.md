# Agent Orchestration Patterns

## Overview

This document captures the continuous dispatch pattern used in the FocalPoint session (April 2026): parallel Haiku agents operating on non-overlapping file scopes with a 5-minute sweep cadence.

The pattern emerged from several iterations of coordinating multiple agents in a single codebase without interference, maximizing coverage while minimizing conflicts.

## Motivation

In multi-agent environments, naive parallel dispatch leads to:
- **Scope conflicts:** Two agents editing the same file simultaneously.
- **Merge chaos:** Uncoordinated commits make history unreadable.
- **Inefficient coverage:** Some areas never get audited because no agent claims them.

The continuous dispatch pattern solves these by:
1. **Strict non-overlapping scopes** — each file belongs to exactly one lane.
2. **Consistent commit message prefixes** — makes lane authorship clear in git log.
3. **Honest coverage audits** — periodic sweeps verify every lane is reachable.
4. **5-minute wave cadence** — short feedback loops, quick course corrections.

## Core Concepts

### Lane

A **lane** is a non-overlapping file scope assigned to a single agent. Each lane has:

- **id** — unique identifier (e.g., `domain-state`, `connectors`)
- **name** — human-readable label
- **scope** — glob patterns defining which files belong to this lane (e.g., `crates/focus-rules/**/*.rs`)
- **prompt_template** — specialized dispatch prompt for this lane (includes focus areas, constraints, commit prefix)
- **commit_message_prefix** — consistent prefix for all commits from this lane (e.g., `rules:`)

**Non-overlapping invariant:** No file can match two lanes' scope patterns. The orchestrator validates this before dispatch.

### Sweep Cadence

Agents dispatch on a **5-minute cadence**:

```
T+0min     → Lane A dispatches (commit tracker → in_flight=true)
T+2min     → Lane B dispatches (independent scope, no conflicts)
T+4min     → Lane C dispatches
T+5min     → Wave 1 complete; all lanes status checked
           → Lane A completes, marks coverage, resets tracker
T+5min     → Lane A re-dispatches for wave 2
T+10min    → Wave 2 complete; honest audit runs
```

**Rationale:**
- 5 minutes is long enough for an agent to make meaningful progress (3–5 commits).
- Short enough to detect and correct course quickly if a lane is stuck.
- Aligns with token budgets (Haiku agents ~90–150K tokens per session).

### Dispatch Prompt

Each lane's prompt is **highly specialized**:

```
You are reviewing the rule engine, rewards, and penalties layer.

Focus on:
1. Rule evaluation correctness
2. Reward calculation and bounds
3. Penalty application logic
4. Policy compliance and audit trail
5. Test coverage for all rule types

Scope: crates/focus-rules/**/*.rs, crates/focus-rewards/**/*.rs, crates/focus-penalties/**/*.rs

Commit with prefix: 'rules:' (e.g., 'rules: add penalty accumulation tests')

Constraints:
- Do NOT modify files outside this scope
- Do NOT add external dependencies without approval
- Prefer existing abstractions over new traits
```

**Good prompt guidance:**
- Names the specific domain or layer.
- Enumerates 3–5 focus areas (what to look for, what to improve).
- Lists glob patterns explicitly.
- Specifies the commit message prefix.
- Includes non-negotiable constraints (scope, dependencies, architecture).

**Weak guidance:**
- "Improve code quality" (vague).
- "Review everything" (unbounded).
- No commit prefix (makes git log unreadable).

### Tracker State

The orchestrator maintains a **state file** (``.orchestration-state.json``) tracking each lane:

```json
{
  "timestamp": "2026-04-24T07:30:00Z",
  "lanes": {
    "domain-state": {
      "lane_id": "domain-state",
      "last_dispatch": "2026-04-24T07:25:00Z",
      "in_flight": false,
      "last_commit_sha": "abc123...",
      "coverage_count": 3
    },
    "connectors": {
      "lane_id": "connectors",
      "last_dispatch": "2026-04-24T07:20:00Z",
      "in_flight": false,
      "last_commit_sha": "def456...",
      "coverage_count": 2
    }
  }
}
```

**Fields:**
- `in_flight` — true if dispatch is pending completion.
- `last_dispatch` — timestamp of most recent dispatch.
- `last_commit_sha` — most recent commit from this lane (for audit linking).
- `coverage_count` — number of waves this lane has completed.

## Honest Coverage Audits

After every **N waves** (typically N=10, ~50 minutes), run:

```bash
agent-orchestrator audit --since-commit <last-audit-sha>
```

This sweeps the git log and checks:
1. **All lanes are reachable.** Every lane's glob patterns match at least one file.
2. **No orphaned files.** No file in the repo falls outside any lane's scope.
3. **Commit prefix distribution.** Verifies lane authorship is balanced (no lane starved).
4. **File churn.** Which files are changing most frequently? (Signals areas needing focus or over-dispatch.)

**Output example:**
```
Audit: Lane Coverage Analysis

Lane          Status
==================================================
domain-state  42 files
connectors    18 files
rules-rewards 33 files
storage-sync  56 files
audit-security 12 files

All lanes have matching files.
Commit distribution:
  domain:    8 commits
  connector: 5 commits
  rules:     6 commits
  storage:   9 commits
  audit:     3 commits

Hotspots (top 3 files by churn):
  crates/focus-storage/src/lib.rs      (7 commits)
  crates/focus-rules/src/eval.rs       (5 commits)
  crates/focus-sync-store/src/lib.rs   (4 commits)
```

**Action items from audit:**
- If a lane has 0 matches, glob pattern is wrong; fix it.
- If a file is orphaned, assign it to the most relevant lane.
- If one lane has way more commits than others, it may be overspecialized; consider splitting.
- If a lane has low coverage, increase its dispatches or widen its scope.

## Quota & Rate-Limit Handling

### Multi-Hour Wave Recovery

When token budgets are tight or rate limits kick in:

1. **Detect:** Agent returns 429 (rate limit) or session ends before completion.
2. **Log:** Record `in_flight=true` and last commit SHA.
3. **Pause:** Skip new dispatches to that lane for N minutes.
4. **Resume:** After cooldown, re-dispatch with fresh token budget. Agent resumes from last commit.

**Multi-hour example:**
```
T+0min       → All 5 lanes dispatch (wave 1)
T+5min       → Lane 4 hits rate limit; marked in_flight, cooldown=30min
T+30min      → Lane 4 cooldown expires; re-dispatch (wave 1.5)
T+35min      → Lane 4 completes; coverage_count incremented
T+40min      → All 5 lanes ready for wave 2
```

### When to Wait vs. Dispatch More

**Wait (hold dispatch) if:**
- Any lane is `in_flight=true` and last_dispatch < 5 minutes ago.
- Git has uncommitted changes from this lane.
- Orchestrator detects pending merge conflicts.

**Dispatch more if:**
- All lanes have `in_flight=false` and coverage_count < target.
- Honest audit flagged uncovered files.
- Sweep cadence timer has elapsed.

## Non-Overlapping Validation Algorithm

The orchestrator uses a simple set-based algorithm:

```
for each lane:
  for each glob pattern in lane.scope:
    expand glob to file set
    for each file:
      if file in seen_files:
        ERROR: file claimed by both lane[X] and lane[Y]
      else:
        seen_files.insert(file, lane.id)
```

**Complexity:** O(files × patterns), typically < 1 second for most repos.

**Example error:**
```
Error: File 'crates/focus-rules/src/lib.rs' is claimed by both
lane 'rules-rewards' and lane 'shared-eval'. Scopes must be non-overlapping.
```

## Pattern Checklist

When adopting this pattern:

- [ ] Define lanes covering all domain areas (no orphaned files).
- [ ] Make scopes non-overlapping (use orchestrator validation).
- [ ] Write focused prompts for each lane (3–5 focus areas).
- [ ] Choose commit message prefixes that are readable in `git log --oneline`.
- [ ] Set sweep cadence based on token budget (5–10 minutes typical).
- [ ] Run honest audits every N waves (N=10 typical, ~50 minutes).
- [ ] Monitor coverage_count; rebalance if skewed.
- [ ] Handle rate limits gracefully (cooldown + resume).
- [ ] Document lane ownership in your orchestration.toml.

## When This Pattern Works

**Good fit:**
- 2–8 agents on the same repo.
- Well-modularized codebase (crates, modules, clear boundaries).
- Agents have specialized skills (e.g., crypto auditor, connector expert, test writer).
- Workflow allows 5–10 minute feedback loops.
- Coverage is important (honest audits catch blind spots).

**Poor fit:**
- Single agent (overkill).
- Tightly coupled monolithic code (lanes forced to overlap).
- Agents must coordinate within a lane (use a single agent instead).
- Sub-minute response time required (use synchronous tasks).

## Examples in FocalPoint (April 2026)

Five lanes in production during this session:

1. **domain-state** — Domain model, state machine, invariants.
2. **connectors** — OAuth integrations, adapter patterns.
3. **rules-rewards** — Rule evaluation, penalty logic.
4. **storage-sync** — SQLite persistence, merge conflict resolution.
5. **audit-security** — Cryptographic signatures, tamper detection.

Each dispatch:
- Agent receives lane prompt (200–300 words).
- Modifies only files in that lane's scope.
- Commits with lane-specific prefix (e.g., `storage:`, `rules:`).
- Completes within 5 minutes (3–5 commits typical).

**Result:** ~30 commits across 5 lanes in 50 minutes, zero conflicts, clear authorship.

## Further Reading

- `tooling/agent-orchestrator/README.md` — Quick start guide.
- `tooling/agent-orchestrator/orchestration.toml.example` — Fully annotated config.
- `agent-orchestrator lanes list` — See all active lanes.
- `agent-orchestrator lanes dispatch <lane-id>` — View next dispatch prompt.
