# Cross-Repo Duplication Scan - V4 Prep - 2026-06-10

Scope: FocalPoint, thegent, hwLedger, KWatch, agent-user-status; adjacent BytePort, PlayCua, PhenoCompose, nanovms, dispatch-mcp, cheap-llm-mcp. Scans used requested patterns, with rg equivalents where recursive grep was too broad.

## Duplication Table

| Pattern | Repos with it | Refactor target |
|---|---|---|
| CLI construction and subcommand dispatch | FocalPoint (focus-cli), thegent (thegent-shims, Python CLI surface), agent-user-status (argparse modules) | pheno-cli-kit |
| Error enums + anyhow boundaries | FocalPoint, thegent | pheno-error-kit |
| tracing/env_logger/structlog initialization | FocalPoint, thegent, agent-user-status | pheno-observability |
| Env/config loading | FocalPoint, thegent, agent-user-status | pheno-config |
| Retry/backoff/circuit breaker | thegent, FocalPoint docs/policy; likely focus services | pheno-resilience |
| Git worktree management | thegent, hwLedger docs/process, FocalPoint audit workflow | pheno-gitops |
| PR/issue fetchers and gh shell usage | FocalPoint, thegent, dispatch-mcp adjacency | pheno-pr-client |
| sh/bash dependency loader pattern | No live code hit in requested repos | No V4 extraction unless future hits appear |

## Top 5 Libification Candidates

1. pheno-resilience
   - Refs: thegent/crates/thegent-subprocess/src/lib.rs:160 run_with_retry; thegent/crates/thegent-subprocess/src/lib.rs:274 run_retry; thegent/crates/thegent-shims/src/lock.rs:12 MAX_RETRIES; thegent/crates/thegent-runtime/src/main.rs:19 CircuitBreaker; thegent/crates/thegent-memory/src/client.rs:19 CircuitBreaker; thegent/src/thegent/utils/routing_impl/circuit_breaker.py:44 ProviderCircuitBreakerConfig.
   - Why: retry/circuit logic exists in multiple Rust crates and Python modules with incompatible policies.
   - Consumers: thegent shims/runtime/memory/subprocess/harness, FocalPoint async services/webhook/CI watcher, dispatch-mcp.
   - Estimated LOC reduction: 450-800 LOC initially; more if Python circuit-breaker code is normalized.

2. pheno-gitops
   - Refs: thegent/src/thegent_gitops/worktree.py:198 WorktreePool; thegent/src/thegent_gitops/worktree.py:246 acquire_worktree; thegent/src/thegent_gitops/worktree.py:282 release_worktree; thegent/crates/thegent-shims/src/shims/git_checkout.rs:14 is_inside_git_worktree; thegent/crates/thegent-shims/src/shims/git_checkout.rs:25 is_worktree_clean; thegent/crates/thegent-offload/src/executor.rs:85 worktree add flow.
   - Why: worktree isolation, clean-tree guarding, and temporary checkout cleanup are duplicated across Python orchestration and Rust shims.
   - Consumers: thegent, hwLedger rollout flow, FocalPoint audit/agent workflows, KWatch if it adopts isolated agent work.
   - Estimated LOC reduction: 500-900 LOC plus fewer bespoke shell calls.

3. pheno-observability
   - Refs: FocalPoint/crates/focus-webhook-server/src/main.rs:85 tracing init; FocalPoint/crates/focus-asset-fetcher/src/main.rs:45; FocalPoint/crates/focus-ci-watcher/src/main.rs:60; FocalPoint/crates/focus-mcp-server/src/main.rs:40; FocalPoint/tooling/agent-orchestrator/src/main.rs:47; thegent/crates/harness-native/src/dispatcher.rs:30; thegent/crates/thegent-shims/src/main.rs:942; agent-user-status/codex/AGENTS.user-status-snippet.md:1537.
   - Why: each binary initializes logging/tracing slightly differently, making JSON logs, stderr routing, and filter defaults inconsistent.
   - Consumers: FocalPoint binaries/tooling, thegent Rust crates, agent-user-status Python daemons.
   - Estimated LOC reduction: 120-250 LOC; larger value is consistent telemetry shape.

4. pheno-config
   - Refs: FocalPoint/crates/focus-webhook-server/src/main.rs:301 FOCALPOINT_GITHUB_WEBHOOK_SECRET; FocalPoint/crates/focus-cli/src/main.rs:594 FOCALPOINT_DB; FocalPoint/crates/focus-telemetry/src/lib.rs:96; thegent/crates/thegent-shims/src/main.rs:605 OPENAI_BASE_URL; thegent/src/thegent_gitops/identity.py:155 THGENT_GIT_IDENTITY_MAP; agent-user-status/src/agent_user_status/statusd.py:33 host/port env; agent-user-status/src/agent_user_status/agent_imessage_core.py:24 env path.
   - Why: direct env reads scatter defaults, path expansion, validation, and secret redaction.
   - Consumers: FocalPoint, thegent, agent-user-status, dispatch-mcp, cheap-llm-mcp.
   - Estimated LOC reduction: 300-650 LOC and fewer config drift bugs.

5. pheno-error-kit
   - Refs: FocalPoint/crates/focus-ir/src/lib.rs:637 thiserror::Error; FocalPoint/crates/focus-events/src/lib.rs:7; FocalPoint/crates/focus-backup/src/lib.rs:9; FocalPoint/crates/focus-backup/src/lib.rs:13; thegent/crates/thegent-maif/src/lib.rs:22; thegent/crates/thegent-jsonl/src/lib.rs:29; thegent/crates/thegent-watcher/src/main.rs:29.
   - Why: domain error enums and anyhow::Result app boundaries repeat without shared classification, exit-code mapping, or user-facing formatting.
   - Consumers: FocalPoint Rust crates, thegent Rust crates, KWatch/hwLedger if Rust CLIs grow.
   - Estimated LOC reduction: 180-350 LOC; main win is consistent diagnostics.

## CLI Pattern Notes

The literal CLI listing command found FocalPoint/crates/focus-cli and many thegent CLI test surfaces, but zsh brace escaping produced no root cli* listing for several repos. A direct file scan found agent-user-status has many argparse command modules, including agent-user-status/src/agent_user_status/agent_imessage_commands.py:356, statusd.py:458, webcam_eye_tracker.py:283, and bootstrap_cli.py:446. thegent Rust clap starts at thegent/crates/thegent-shims/src/main.rs:16 and :21.

## 5 Next Steps for V4 L3-L4

1. Start L3 with pheno-observability because it is low-risk: replace repeated tracing/env_logger init in two FocalPoint binaries and one thegent crate first.
2. Build pheno-config next with Rust + Python parity: env prefix, path expansion, typed defaults, secret redaction, and test vectors from FocalPoint/thegent/agent-user-status.
3. Extract pheno-resilience from thegent only first; after tests pass, adopt it in FocalPoint webhook/CI watcher retry paths.
4. Promote thegent_gitops/worktree.py into pheno-gitops with a Rust shim API later; preserve clean-worktree guard semantics from thegent-shims.
5. Add V4 governance checks: forbid new direct std::env::var, ad hoc tracing init, local CircuitBreaker structs, and manual git worktree shell flows unless explicitly waived.
