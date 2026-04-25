# target-pruner

Automated tool for reclaiming disk space by pruning old cargo build artifacts and related caches.

## Overview

Phenotype multi-agent workspace frequently accumulates stale `target/` directories (6-8 GB each) from parallel cargo builds. This tool safely removes old builds based on access time (atime), preserving active builds.

## Usage

```bash
# View targets that would be pruned (dry-run)
target-pruner --dry-run

# Actually prune old targets
target-pruner --prune

# Prune and show full report with bytes reclaimed
target-pruner --prune --report
```

## Scope

Targets for pruning (in priority order):

1. **Worktree targets** (`repos/.worktrees/*/target`) — safe to delete after branch is pushed
2. **Completed-push targets** — targets whose branch exists on `origin/`
3. **Archived worktrees** (`.worktrees/**` with age >7 days + no uncommitted changes)

## Limitations

**atime is unreliable during active sessions.** APFS `du` and file access commands reset atime to "today". If a repo is being actively built, the pruner will not free that `target/` because it appears recent. Solution: use `rm -rf <repo>/target` directly when you know the build is complete.

## Expansion Roadmap

Future versions will also prune:

- **node_modules** directories (especially in worktrees) — often 1-3 GB per project
- **Homebrew cache** (`~/Library/Caches/Homebrew`) — coordinated with `disk-emergency.rs`
- **npm cache** (`~/.npm/_cacache`) — if `disk-emergency` has completed

## Configuration

See `target-budget.toml` for age thresholds and exclusion patterns.

## See Also

- `/repos/docs/governance/disk_budget_policy.md`
- `/repos/scripts/disk-emergency.rs` — emergency playbook for 100% disk situations
