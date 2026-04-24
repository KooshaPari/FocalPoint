# Connector Pre-Merge Checklist

Review before merging a new connector PR. Use this as a code-review template.

## Trait Implementation

- [ ] `manifest()` returns `ConnectorManifest` with:
  - [ ] Unique `id` (lowercase, hyphen-separated)
  - [ ] Non-empty `event_types` list
  - [ ] Correct `auth_strategy` (OAuth2, ApiKey, Hmac, Pat)
  - [ ] `sync_mode` with reasonable `cadence_seconds` (300–3600)
  - [ ] `tier` set to Verified/Beta/Unverified (not stub)
  - [ ] `health_indicators` populated
- [ ] `health()` validates credentials and returns appropriate `HealthState`
- [ ] `sync()` returns `SyncOutcome` with events, next_cursor, and partial flag
- [ ] No `panic!()` or `.unwrap()` in trait impls (use `?` operator)

## Event Mapping

- [ ] All `NormalizedEvent` fields populated:
  - [ ] `event_id` generated with `Uuid::new_v4()`
  - [ ] `connector_id` matches manifest `id`
  - [ ] `event_type` uses `EventType::Custom("connector_id:event_name")`
  - [ ] `occurred_at` set to actual event timestamp from provider
  - [ ] `effective_at` set to `Utc::now()` or receipt time
  - [ ] `dedupe_key` unique within connector (includes id + timestamp)
  - [ ] `confidence` between 0.0–1.0 (typical 0.95–1.0)
  - [ ] `payload` contains relevant entity data, no secrets
  - [ ] `raw_ref` traces back to provider (optional but recommended)
- [ ] Event namespace uses connector prefix (e.g., `"linear:issue_created"`)
- [ ] No duplicate events on successive syncs (check dedupe_key logic)
- [ ] Handles timezone parsing safely (fallback to `Utc::now()` on error)

## Authentication

- [ ] Auth strategy matches manifest declaration
- [ ] Tokens stored via `TokenStore` trait (never hardcoded)
- [ ] Secrets never logged or serialized in events
- [ ] OAuth2: token refresh logic implemented (if required)
- [ ] API Key / Bearer: header construction correct
- [ ] HMAC: signature verification using workspace `signature_verifiers`
- [ ] 401/403 responses handled explicitly (return `Unauthorized`)

## Rate-Limiting

- [ ] 429 responses parsed and handled:
  - [ ] `Retry-After` header respected
  - [ ] Exponential backoff or immediate retry
  - [ ] Does not exceed provider's rate limits in practice
- [ ] Cursor-based pagination reduces redundant fetches
- [ ] Health check is lightweight (not counted against quota if possible)

## Testing

- [ ] Minimum 3 unit tests with `#[test]` or `#[tokio::test]`
  - [ ] Test builder/construction
  - [ ] Test manifest contract
  - [ ] Test sync mapping (wiremock mock or golden fixture)
- [ ] All tests include `// Traces to: FR-CONNECTOR-<ID>-<N>` comment
- [ ] Golden-file fixtures in `tests/fixtures/` for realistic API responses
- [ ] 429 retry test (if applicable)
- [ ] Error cases handled (auth failure, network error, parse error)
- [ ] Tests pass: `cargo test -p connector-<name>`

## Code Quality

- [ ] No compiler warnings: `cargo clippy -p connector-<name> -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No `#[allow(...)]` suppressions without inline justification
- [ ] Dependencies only from workspace (no new external crates without justification)
- [ ] Logging via `tracing::*` macros (not `println!`)
- [ ] All public items have doc comments with examples
- [ ] Module structure clean (api, auth, events, models, lib)

## Documentation

- [ ] New page `docs-site/connectors/<name>.md` with:
  - [ ] Overview and provider link
  - [ ] Setup/authorization instructions
  - [ ] Supported event types table
  - [ ] Sync frequency and rate limits
  - [ ] Troubleshooting section
  - [ ] Example integration (curl or SDK snippet)
- [ ] SDK guide cross-reference (`docs-site/guides/connector_sdk.md`)
- [ ] Inline code comments for non-obvious logic (e.g., state inference)

## Registry & Workspace

- [ ] New crate added to workspace `Cargo.toml` members array
- [ ] Updated `crates/focus-connectors/src/registry.rs` (or feature gate if used)
- [ ] Workspace builds: `cargo check --workspace`
- [ ] No new root-level dependencies introduced

## Manifest Validation

- [ ] `connector_manifest.rs` or inline manifest is valid TOML/JSON
- [ ] All event types listed in manifest match emitted events
- [ ] All entity_types used in rules/filtering are declared
- [ ] Verification tier reflects testing coverage (Verified ≥3 tests + docs)

## Performance

- [ ] Typical sync completes in <5 seconds (network latency excluded)
- [ ] No unbounded loops or recursive calls
- [ ] Memory usage is O(n) in entity count, not exponential
- [ ] Cursor prevents re-fetching entire history on every sync

## Security

- [ ] No secrets in logs, events, or error messages
- [ ] Token validation on every sync (or cache with expiry)
- [ ] HTTPS only (no HTTP fallback for APIs)
- [ ] Signature verification for webhook-based connectors
- [ ] Rate-limit retries don't bypass auth

## Final Checks

- [ ] PR title: `feat(connector): add <name> connector` or similar
- [ ] PR description includes: motivation, event types, auth method, testing summary
- [ ] Linked to AgilePlus spec (if applicable)
- [ ] All CI checks pass (lint, test, doc build)
- [ ] Commit message format adheres to Phenotype conventions
- [ ] No unrelated changes in the same PR

---

**Template for reviewer to paste as comment:**

```
## Connector Pre-Merge Review

**Trait Implementation:** [ ]
**Event Mapping:** [ ]
**Authentication:** [ ]
**Rate-Limiting:** [ ]
**Testing:** [ ]
**Code Quality:** [ ]
**Documentation:** [ ]
**Registry & Workspace:** [ ]
**Manifest Validation:** [ ]
**Performance:** [ ]
**Security:** [ ]

**Approved:** [ ] / **Changes Requested:** [ ]
```
