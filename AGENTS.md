# FocalPoint — AGENTS.md

## Project Overview
FocalPoint is the Phenotype-org dependency management and tooling orchestrator. Multi-crate Rust workspace.

## Stack
- Language: Rust (workspace with ~30 crates)
- Build: cargo, Taskfile
- Test: cargo test

## Key Commands
- `task install` — Install dependencies
- `task build` — Build all crates
- `task test` — Run tests
- `task quality` — Lint + format + type-check

## Quality Gates
- `cargo check --workspace --all-targets`
- `cargo test --workspace`
- `ruff check src/` (if applicable)
- `ty check src/` (if applicable)

## Branch Discipline
- Feature work in worktrees: `FocalPoint-wtrees/<topic>/`
- Canonical on `main`; no direct authoring in canonical

## Child Agent Usage
Delegate multi-crate analysis and refactor sweeps to subagents (Kimi/Forge-tier). Reserve Opus for architectural decisions only.
