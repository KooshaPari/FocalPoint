# Connector SDK — Narrow Contract

> Source: arch doc lines 354–415 + 1573–1606. Stub; fill in as Phase 3 approaches.

## The contract (intentionally small)

```rust
#[async_trait]
pub trait Connector: Send + Sync {
    fn manifest(&self) -> &ConnectorManifest;
    async fn health(&self) -> HealthState;
    async fn sync(&self, cursor: Option<String>) -> Result<SyncOutcome>;
}
```

Three methods. That's the entire API surface a connector author implements.

## Manifest

```rust
struct ConnectorManifest {
    id: String,
    version: String,
    display_name: String,
    auth_strategy: AuthStrategy,
    sync_mode: SyncMode,
    capabilities: Vec<ConnectorCapability>,
    entity_types: Vec<String>,
    event_types: Vec<String>,
}
```

## Events

All events normalize to `focus_events::NormalizedEvent`. Required fields:
`event_id`, `connector_id`, `account_id`, `event_type`, `occurred_at`,
`effective_at`, `dedupe_key`, `confidence`, `payload`.

**Dedupe key** is the single most important thing connector authors get
right. Formula: `format!("{source}:{entity_id}:{occurred_at_unix}")`.

## Auth strategies

- `OAuth2 { scopes }` — standard OAuth2 code flow; redirect URL via platform deep-link
- `ApiKey` — user-supplied key in secure storage
- `DeviceBrokered` — leverages platform APIs (e.g. Screen Time itself)
- `None` — for testkit fixtures

## Sync modes

- `Polling { cadence_seconds }` — connector polled on schedule
- `Webhook` — requires `services/webhook-ingest/`
- `Hybrid` — polls with webhook-triggered refresh

## Authoring checklist

- [ ] Implement `Connector` trait
- [ ] Ship at least one `fixture.json` for `connector-testkit` replay
- [ ] Document `event_type` → `payload` schema in crate README
- [ ] Dedupe key is stable across restarts and network retries
- [ ] `health()` transitions to `Unauthenticated` when token expires (not `Failing`)

## Reference

- `crates/connector-canvas/` — Canvas LMS reference implementation
- `crates/connector-testkit/` — fixture-replay harness

## Not yet specified (Phase 3)

- Template/marketplace format for community connectors
- Signing + verification for third-party connectors
- Versioning + backwards-compat policy (semver + capability negotiation)
