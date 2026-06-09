# Architecture

## Overview
- FocalPoint is a large Rust workspace with desktop, service, tooling, and integration crates.
- The current shape centers on the iOS app, shared focus domain libraries, and supporting services.
- This document is a skeleton that should be expanded with crate-level ownership and boundaries.

## Components
## apps/ios/FocalPoint
- iOS application entrypoint and platform integration surface.

## crates/*
- Shared focus domain crates, connectors, policy, telemetry, storage, and UI support.

## services/*
- Service-level crates such as the GraphQL gateway and templates registry.

## tooling/*
- Maintenance, release, validation, and repository support tooling.

## tests/e2e
- End-to-end validation harness for repo-level workflows.

## Data flow
```text
client input -> app layer -> shared crates -> services/tooling -> external APIs and storage
```

## Key invariants
- Keep domain logic in shared crates rather than in application shells.
- Treat connector and policy boundaries as explicit contracts.
- Maintain consistent behavior between local tooling and shipped binaries.

## Cross-cutting concerns (config, telemetry, errors)
- Config: use shared configuration patterns across crates and apps.
- Telemetry: propagate structured logs and traces across the workspace.
- Errors: normalize failure handling so boundaries can report actionable messages.

## Future considerations
- Replace crate group placeholders with a concrete per-crate breakdown.
- Add startup and sync diagrams for the desktop, service, and tooling paths.
- Capture release and integration assumptions as the workspace evolves.
