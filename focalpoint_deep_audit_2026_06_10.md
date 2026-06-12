# FocalPoint V4 Deep Audit

## Snapshot
- HEAD: `7173f974 chore: add Justfile grade targets (grade/grade-fast/grade-json/grade-html)`; prior: `d5af59b8`, `0a3d18c1`.
- Worktree: one untracked root doc, `PR_AUDIT_2026_06_10.md`.
- Workspace members: 50+ members, including core `focus-domain/events/rules/rewards/penalties/audit/storage/sync/ffi/cli`, connector crates, tooling, `tests/e2e`, and `focus-plugin-sdk`.
- Open PR/worktree names by local listing: `.claude/worktrees/` is empty or absent.
- Domain per `SPEC.md`: connector-first screen-time platform with portable Rust core, iOS enforcement shell, rules engine, reward/penalty ledger, audit chain, AI coaching, CLI, MCP, release tooling.

## Build Matrix
- `cargo check --workspace 2>&1 | tail -10`: clean; tail shows `Finished \`dev\` profile...`.
- Practical V4 matrix should keep: `cargo check --workspace`, `cargo test --workspace`, focused checks for `focus-ffi`, `focus-cli`, `focus-sync`, `focus-rules`, and iOS/Xcode if native app is part of L4/L5.

## Crate Inventory
Main crates and lib/main LOC observed:
- `focus-ffi`: 3504 LOC; deps many core crates, UniFFI, connectors, crypto. Key integration surface; too large.
- `focus-cli`: 2882 LOC main; deps storage/audit/planning/templates/rules/replay/rewards/penalties/domain/demo/lang/observability. CLI orchestration is oversized.
- `focus-eval`: 1913 LOC; deps audit/domain/events/rewards/penalties/rules/storage/sync/observability. Evaluation pipeline is monolithic.
- `focus-ir`: 1931 LOC; deps domain/planning/storage/rules, sha2/hex. IR/hash logic large.
- `focus-rules`: 1642 LOC; deps domain/events/coaching, cron/regex. Rules engine core, large.
- `focus-lang`: 1298 LOC; deps starlark/focus-ir. DSL compiler large.
- `focus-scheduler`: 1320 LOC; deps domain/planning/calendar, proptest/criterion. Scheduling core large.
- `focus-rituals`: 1025 LOC; deps domain/planning/scheduler/calendar/coaching/rewards/penalties/mascot/events/audit. Cross-domain coordinator.
- `focus-sync`: 1014 LOC; deps connectors/events/time/observability/tokio. Has ports/retry/cursor modules.
- `focus-policy`: 756 LOC; deps audit/domain/rules. Enforcement policy and callback port.
- `focus-audit`: 695 LOC; deps observability/criterion/hex. Audit store trait exists.
- `focus-connectors`: 659 LOC; deps domain/events/http/jwt/signature/tracing. Connector runtime.
- `focus-templates`: 676 LOC; deps rules/domain/rand_core. Template registry/validation.
- `focus-rule-suggester`: 672 LOC; deps audit/events/rules/storage/domain. Suggestion logic.
- `focus-storage`: 70 LOC top lib plus submodules; deps core domain crates and rusqlite via workspace. Good facade shape.
- `focus-domain`: 213 LOC; pure types, no I/O.
- Connector crates: `connector-canvas` 463, `gcal` 338, `github` 391, `fitbit` 170, `linear` 207, `notion` 196, `readwise` 199, `strava` 241. Mostly HTTP/OAuth wrappers.

## Hex Readiness Score: 5/10
Rationale: domain purity exists (`focus-domain/src/lib.rs:3` says no persistence/I/O), and multiple ports exist: `CalendarPort` (`focus-calendar/src/lib.rs:69`), `CloudKitPort` (`focus-sync/src/cloudkit_port.rs:19`), `ClockPort`, `AuditStore`, storage traits, `EnforcementCallbackPort` (`focus-policy/src/lib.rs:268`). But there is no deliberate `src/domain`, `src/ports`, `src/adapters` directory layout, and `find crates -path '*/src/domain|ports|adapters'` found none. Several crates violate the 500-line mandate, especially `focus-ffi`, `focus-cli`, `focus-eval`, `focus-ir`, `focus-rules`, `focus-lang`, `focus-scheduler`, and `focus-sync`.

## 5 Main Ports To Extract
1. Window/app enforcement manager: create `FocusWindowManagerPort` / `AppEnforcementPort` for platform blockers, app-target state, authorization revoked callbacks. Anchor: `focus-policy/src/lib.rs:250-268`.
2. App focus/session lifecycle: extract session start/complete/foreground-resume lifecycle from CLI/FFI into a port that emits events. Anchor: CLI command dispatch `focus-cli/src/main.rs:255-260`, sync trigger `focus-sync/src/lib.rs:40-45`.
3. Do-not-disturb / screen-time authorization: model authorization/revocation separately from enforcement callbacks. Anchor: `AuthorizationRevoked` callback `focus-policy/src/lib.rs:259-262`.
4. Lifecycle/background scheduler: wrap long-running loop/ticker, foreground resume, scheduled sync, retries. Anchor: `SyncOrchestrator::run_loop` `focus-sync/src/lib.rs:430-437`.
5. Persistence: consolidate `EventStore`, `RuleStore`, `WalletStore`, `PenaltyStore`, `AuditStore`, `TaskStore`, `SyncStore` behind smaller repository/transaction ports. Anchor: storage traits `focus-storage/src/lib.rs:11-52`, audit store `focus-audit/src/lib.rs:190`.

## Hand-Rolled Patterns To Wrap
- CLI parser/orchestrator: massive `Cli`/subcommand enum and dispatch in one main file, `focus-cli/src/main.rs:182-200`, `564-588`.
- DB path/config loader: direct env/HOME resolution, `focus-cli/src/main.rs:590-600`; should be `ConfigPort/AppPaths`.
- Filesystem template/config loading: direct `current_dir`, `read_dir`, `read_to_string`, `write`, `focus-cli/src/main.rs:959-985`, `1021-1025`, `934-937`.
- Telemetry local buffer: direct env endpoint and raw rusqlite schema/flush in telemetry client, `focus-telemetry/src/lib.rs:85-112`, `153-238`; needs transport/storage ports.
- Observability init: env-driven global tracing init, partial OTEL placeholder, `focus-observability/src/lib.rs:66-96`, `109-123`.
- Retry/backoff: exists as `focus-sync::retry` and orchestrator fields `focus-sync/src/lib.rs:107-140`; promote as shared resilience policy instead of sync-only.
- Secret/client config in FFI: direct `FOCALPOINT_*_CLIENT_ID/SECRET` reads, `focus-ffi/src/lib.rs:1556`, `1620-1622`, `1704-1706`.

## 5 SOTA Gaps
1. No consistent hexagonal folder contract despite scattered traits.
2. Oversized crates block V4 parallelism and test isolation.
3. Platform enforcement is callback-oriented but lacks a first-class macOS/iOS window/app/DND adapter boundary.
4. Config, paths, telemetry, and secrets are read directly from env/files in runtime code.
5. Build is clean, but architecture checks for size/import boundaries/port conformance are not yet visible in the matrix.

## 5 Prioritized Next Steps For V4 L2-L5
1. L2: create `focus-ports` or per-crate `ports.rs` contract for lifecycle, enforcement, persistence, telemetry, config, clock; move existing traits without behavior changes.
2. L2: split `focus-cli` into command modules and a thin `main.rs`; keep `main.rs` under 250 LOC.
3. L3: split `focus-ffi` by facade domains: policy, sync, connectors, wallet/penalty, audit, config/secrets.
4. L4: add platform adapter crates/modules for macOS/iOS enforcement: window/app focus, DND/screen-time authorization, lifecycle callbacks, persistence adapter.
5. L5: enforce gates: max 500 LOC per Rust file, `cargo check/test --workspace`, boundary lint/deny direct env/fs/rusqlite outside adapters, plus smoke tests for extracted ports.
