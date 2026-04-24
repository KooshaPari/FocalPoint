# Connector SDK Guide

Complete guide to authoring a new FocalPoint connector. FocalPoint's event-source architecture supports plug-and-play connectors for any data provider. This guide covers decision criteria, trait contracts, auth patterns, event mapping, testing, and the registration workflow.

## When to Write a Connector

Before implementing, verify that the data source is worth integrating:

- **Rich event types.** The API must emit meaningful events (state changes, user actions, timestamps). Metadata-only APIs are poor fits.
- **Reliable sync.** Does the provider offer polling, webhooks, or both? Polling without rate limits is fragile.
- **User relevance.** Does the event type correlate with screen time, focus, or productivity signals? (e.g., GitHub commits, Linear issues closed, Fitbit activity).
- **Auth simplicity.** Can you implement OAuth2, API token, or HMAC webhook verification in ~200 LOC? If not, defer.

**Decision checklist** (answer Yes to all before proceeding):
- [ ] API has ≥2 distinct event types (e.g., `issue_created`, `issue_closed`)?
- [ ] API supports polling (REST GET + cursor) OR webhooks (HTTPS POST + HMAC)?
- [ ] Rate limits are documented and manageable (≥100 req/hour for polling)?
- [ ] Auth method is OAuth2, PAT, API key, or HMAC-backed?
- [ ] Sync < 5 sec for typical account (not counting network latency)?

## Connector Trait Contract

All connectors implement `focus_connectors::Connector`. Reference the trait in `/crates/focus-connectors/src/lib.rs`:

```rust
#[async_trait]
pub trait Connector: Send + Sync {
    fn manifest(&self) -> &ConnectorManifest;
    async fn health(&self) -> HealthState;
    async fn sync(&self, cursor: Option<String>) -> Result<SyncOutcome>;
}
```

### manifest()

Returns metadata: id, version, auth strategy, sync mode, entity types, event types, and verification tier.

```rust
ConnectorManifest {
    id: "github".into(),
    version: "0.1.0".into(),
    display_name: "GitHub".into(),
    auth_strategy: AuthStrategy::OAuth2,
    sync_mode: SyncMode::Polling { cadence_seconds: 300 },
    capabilities: vec!["webhook_signature_verification".into()],
    entity_types: vec!["repository".into(), "pull_request".into()],
    event_types: vec![
        "github:push".into(),
        "github:pull_request_opened".into(),
    ],
    tier: VerificationTier::Verified,
    health_indicators: vec!["api_token_valid".into(), "last_sync_ok".into()],
}
```

**Fields:**
- `id`: Lowercase, hyphen-separated (e.g., `"linear"`, `"apple-health"`). Used as namespace prefix for events.
- `auth_strategy`: One of `OAuth2`, `ApiKey`, `Hmac`, `Pat`.
- `sync_mode`: `Polling { cadence_seconds }` or `Webhook` or `Both`. Polling cadence is the minimum interval the orchestrator respects; choose 300–3600 depending on event freshness requirements.
- `capabilities`: Declare features. Include `"webhook_signature_verification"` if handling webhooks.
- `entity_types`: Entity kinds emitted (e.g., `["issue", "repository"]`). Used for filtering and rules.
- `event_types`: Full list of event kinds emitted in `connector_id:event_name` format.
- `tier`: `Verified` (production-ready, tested), `Beta` (functional but limited testing), or `Unverified` (stub).
- `health_indicators`: Health check signals to monitor (e.g., `"api_token_valid"`, `"last_sync_ok"`).

### health()

Validate that credentials are live and the API is reachable. Return `HealthState`:

```rust
pub enum HealthState {
    Healthy,
    Unauthenticated,
    Failing(String),
}
```

Typically: call a lightweight API endpoint (e.g., `GET /user` for GitHub), verify auth tokens, and report errors.

```rust
async fn health(&self) -> HealthState {
    match self.client.verify_token().await {
        Ok(_) => HealthState::Healthy,
        Err(ConnectorError::Unauthorized(_)) => HealthState::Unauthenticated,
        Err(e) => HealthState::Failing(e.to_string()),
    }
}
```

### sync()

Fetch events since the last cursor. Return `SyncOutcome`:

```rust
pub struct SyncOutcome {
    pub events: Vec<NormalizedEvent>,
    pub next_cursor: Option<String>,
    pub partial: bool,
}
```

**Semantics:**
- `events`: List of `NormalizedEvent` objects (see below).
- `next_cursor`: Opaque string (URL-encoded timestamp, pagination token, etc.) passed to the next `sync()` call. If `None`, the orchestrator does not update the persisted cursor.
- `partial`: `true` if this sync covered only a subset of the account's data (e.g., paginated API responses where you stopped early due to rate limits). The orchestrator will call `sync()` again with the same cursor on the next tick to continue.

## NormalizedEvent Contract

Each connector emits `NormalizedEvent` (from `focus_events` crate):

```rust
pub struct NormalizedEvent {
    pub event_id: Uuid,                   // Unique per event (generate with Uuid::new_v4())
    pub connector_id: String,              // "linear", "github", etc.
    pub account_id: Uuid,                  // User account UUID
    pub event_type: EventType::Custom(...),// "linear:issue_created"
    pub occurred_at: DateTime<Utc>,        // When the event happened in the source
    pub effective_at: DateTime<Utc>,       // When FocalPoint received it (usually now)
    pub dedupe_key: String,                // Unique within connector; prevents duplicate processing
    pub confidence: f64,                   // 0.0–1.0; how sure you are (0.99 typical)
    pub payload: serde_json::Value,        // Event details (provider-specific schema)
    pub raw_ref: Option<TraceRef>,         // Link back to source (for audits)
}
```

**Key fields:**

- `event_id`: Always generate with `Uuid::new_v4()` (unique per process run).
- `connector_id`: Must match your manifest `id`.
- `event_type`: Use `EventType::Custom("connector_id:event_name".into())`. Namespace with connector ID to avoid collisions.
- `occurred_at` vs `effective_at`: `occurred_at` is when the event happened in the provider's timeline; `effective_at` is when FocalPoint received it (usually `Utc::now()`).
- `dedupe_key`: Prevent duplicate event emission across syncs. Build it from stable identifiers:
  ```rust
  let dedupe_key = EventFactory::new_dedupe_key(
      "linear",
      &format!("{}-issue-{}", account_id, issue.id),
      occurred_at,
  );
  ```
- `confidence`: 1.0 if you're certain of the event; <1.0 if uncertain (e.g., inferred state). Useful for rules engines.
- `payload`: Serde-serialized JSON with event-specific data. Include entity IDs, summaries, and links. No secrets.
- `raw_ref`: Trace back to the source for auditing. Include source name and provider's entity ID.

## Auth Patterns

Choose one strategy per connector:

### API Key / Bearer Token

Simplest; suitable for personal API keys or service tokens.

```rust
pub struct SimpleAuth {
    token: String,
}

impl SimpleAuth {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }

    pub fn bearer_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}
```

Store tokens securely:
- iOS: Use Keychain via `focus-ffi` SecureSecretStore trait.
- CLI: Pass via environment variable or secure CLI prompt.

**Rate-limit handling:**
```rust
if response.status() == 429 {
    if let Some(retry_after) = response.headers().get("Retry-After") {
        let secs = retry_after.to_str()?.parse::<u64>()?;
        tokio::time::sleep(Duration::from_secs(secs)).await;
        return self.retry_request(...).await;
    }
}
```

### OAuth2

Multi-user scenarios; requires callback URL and token refresh.

Use the `oauth2` crate (in workspace dependencies):

```rust
use oauth2::{
    AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl,
    Client, PkceCodeChallenge, Scope,
};

pub fn oauth_client() -> Client {
    Client::new(
        ClientId::new("your-client-id".to_string()),
        Some(ClientSecret::new("your-client-secret".to_string())),
        AuthUrl::new("https://provider.com/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://provider.com/oauth/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080/callback".to_string()).unwrap())
}

pub async fn exchange_code(client: &Client, code: &str) -> Result<AccessToken> {
    let token = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await?;
    Ok(token.access_token().secret().clone())
}
```

**Token refresh:**
```rust
if response.status() == 401 {
    let new_token = refresh_access_token(&refresh_token).await?;
    token_store.set_token(new_token.clone());
    return self.retry_request_with_token(&new_token).await;
}
```

### HMAC Webhook Signature Verification

For webhook-based connectors. Provider sends a signature in the request header; verify it before processing.

```rust
use focus_connectors::signature_verifiers::verify_github_webhook;

#[post("/webhook/github")]
pub async fn github_webhook(
    body: bytes::Bytes,
    signature: String,
    secret: String,
) -> Result<StatusCode> {
    verify_github_webhook(&body, &signature, &secret)?;
    // Process payload...
    Ok(StatusCode::OK)
}
```

### Polling vs Push

**Polling:** Connector calls `sync()` on a schedule. Simpler (no webhook server), but higher latency and API overhead.

**Webhooks:** Provider calls your webhook server when events occur. Lower latency, but requires a persistent URL and signature validation.

**Hybrid:** Polling for baseline data, webhooks for real-time updates. Dedupe keys prevent double-emission.

## Event Mapping

Transform API responses into `NormalizedEvent` objects. Build a mapper struct:

```rust
pub struct MyConnectorEventMapper {
    account_id: Uuid,
}

impl MyConnectorEventMapper {
    pub fn new(account_id: Uuid) -> Self {
        Self { account_id }
    }

    pub fn map_items(&self, items: Vec<ProviderItem>) -> Vec<NormalizedEvent> {
        items.into_iter()
            .flat_map(|item| self.map_item(item))
            .collect()
    }

    fn map_item(&self, item: ProviderItem) -> Vec<NormalizedEvent> {
        let mut events = Vec::new();

        let occurred_at = parse_timestamp(&item.created_at);

        let dedupe_key = EventFactory::new_dedupe_key(
            "myconnector",
            &format!("{}-{}", item.id, item.event_type),
            occurred_at,
        );

        events.push(NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: "myconnector".into(),
            account_id: self.account_id,
            event_type: EventType::Custom(format!("myconnector:{}", item.event_type).into()),
            occurred_at,
            effective_at: Utc::now(),
            dedupe_key,
            confidence: 0.95,
            payload: serde_json::json!({
                "id": item.id,
                "summary": item.summary,
                "status": item.status,
            }),
            raw_ref: Some(TraceRef {
                source: "myconnector-api".into(),
                id: item.id,
            }),
        });

        events
    }
}
```

**Confidence scores:**
- `1.0` — Certainty (e.g., explicit event from provider API).
- `0.95–0.99` — High confidence with minor inference.
- `0.8–0.95` — Inferred from state changes (e.g., "completed" status → "item_completed" event).
- `<0.8` — Heuristic or guessed; avoid unless necessary.

## Manifest Schema

Define your connector manifest in `connector_manifest.rs`:

```toml
[connector]
id = "linear"
version = "0.1.0"
name = "Linear Issue Tracking"

[auth]
strategy = "api_key"  # "api_key", "oauth2", "hmac", "pat"

[sync]
mode = "polling"
cadence_seconds = 300

[capabilities]
webhook_signature = false

[entities]
types = ["issue", "project"]

[events]
types = [
  "linear:issue_created",
  "linear:issue_closed",
  "linear:issue_updated",
]

[verification]
tier = "verified"

[health]
indicators = ["api_key_valid", "last_sync_ok"]
```

## Testing

### Unit Tests with wiremock

Mock the provider's API responses using `wiremock`:

```rust
#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    // Traces to: FR-CONNECTOR-LINEAR-SYNC-001
    #[tokio::test]
    async fn linear_sync_maps_issues() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
                "data": {
                    "issues": {
                        "nodes": [
                            {
                                "id": "LIN-123",
                                "identifier": "ACME-456",
                                "title": "Fix login bug",
                                "state": "Done",
                                "createdAt": "2026-04-20T10:00:00Z",
                                "updatedAt": "2026-04-23T15:30:00Z"
                            }
                        ]
                    }
                }
            }"#))
            .mount(&mock_server)
            .await;

        let client = LinearClient::new_with_url(
            reqwest::Client::new(),
            mock_server.uri(),
        );
        let connector = LinearConnectorBuilder::new()
            .account_id(Uuid::new_v4())
            .client(client)
            .build();

        let outcome = connector.sync(None).await.unwrap();
        assert_eq!(outcome.events.len(), 2); // Created + closed
        assert!(outcome.events.iter().any(|e| e.event_type.to_string().contains("issue_created")));
    }
}
```

### Golden-File Fixtures

Store realistic API responses as `.json` files in `tests/fixtures/`:

```json
{
  "data": {
    "viewer": {
      "id": "user-123",
      "name": "Alice"
    }
  }
}
```

Load in tests:
```rust
#[test]
fn parse_fixture() {
    let json = std::fs::read_to_string("tests/fixtures/linear_issues.json")
        .expect("fixture not found");
    let issues: Vec<LinearIssue> = serde_json::from_str(&json).unwrap();
    assert!(!issues.is_empty());
}
```

### Rate-Limit Simulation

Test 429 handling:

```rust
#[tokio::test]
async fn handles_429_retry() {
    let mock_server = MockServer::start().await;

    // First call: 429
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .append_header("Retry-After", "1")
        )
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // Second call: 200 OK
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .mount(&mock_server)
        .await;

    // Test retry logic...
}
```

## Adding to the Registry

1. **Create crate directory:**
   ```bash
   mkdir crates/connector-myname
   ```

2. **Generate scaffold** (see CLI section below):
   ```bash
   focus connector new myname --auth token --events "event_created,event_updated"
   ```

3. **Add to workspace `Cargo.toml`:**
   ```toml
   [workspace]
   members = [
       ...,
       "crates/connector-myname",
   ]
   ```

4. **Update `focus-connectors` registry:**
   In `crates/focus-connectors/src/registry.rs`, add your connector to the registry loader (if using dynamic registry) or declare it as a feature gate:
   ```rust
   #[cfg(feature = "connector_myname")]
   pub mod connector_myname;
   ```

5. **Create docs-site page:**
   - Path: `docs-site/connectors/myname.md`
   - Template: Include setup steps, auth flow, event types, limitations, and troubleshooting.

## Per-Connector Docs-Site Page

Each connector gets a dedicated page under `docs-site/connectors/`:

```markdown
# MyConnector Integration

## Overview

Brief description. Link to provider's docs.

## Getting Started

### Prerequisites
- MyConnector account
- API token (or OAuth setup instructions)

### Authorization

#### API Token
1. Log into MyConnector dashboard
2. Go to Settings > API Tokens
3. Create a token with scopes: `read:items`, `read:projects`
4. Copy and paste into FocalPoint Settings > Connectors

#### OAuth2
1. FocalPoint redirects you to MyConnector login
2. Grant permissions
3. FocalPoint stores the token securely

## Supported Events

| Event | Type | Payload |
|-------|------|---------|
| `myconnector:item_created` | Entity creation | `{ id, name, created_at }` |
| `myconnector:item_updated` | Entity state change | `{ id, status, updated_at }` |

## Sync Frequency

Polls every 5 minutes (300 seconds).

## Rate Limits

MyConnector allows 100 requests/hour. FocalPoint respects `Retry-After` headers.

## Troubleshooting

**"Unauthorized" health check:**
- Verify token is still valid in MyConnector dashboard
- Regenerate if expired

**Missing events:**
- Check the app's Settings > Connectors > MyConnector > Logs
```

## CLI Scaffolder: `focus connector new`

Use the scaffolder to bootstrap a new connector:

```bash
focus connector new linear --auth oauth2 --events "issue_created,issue_closed,issue_updated"
```

**Output:**
- `crates/connector-linear/Cargo.toml` — manifest with dependencies
- `crates/connector-linear/src/lib.rs` — Connector trait impl stub
- `crates/connector-linear/src/api.rs` — HTTP client skeleton
- `crates/connector-linear/src/auth.rs` — Auth helper
- `crates/connector-linear/src/models.rs` — API response types
- `crates/connector-linear/src/events.rs` — Event mapper stub
- `crates/connector-linear/src/connector_manifest.rs` — Manifest TOML
- `crates/connector-linear/tests/wiremock_tests.rs` — Wiremock test scaffold

Then:
```bash
cargo check -p connector-linear
# Verify it compiles
```

Edit and implement the stubs. See **Testing** section for examples.

## Workflow Summary

1. **Decide.** Answer the decision checklist. Create an AgilePlus spec.
2. **Scaffold.** Run `focus connector new <name> --auth <type> --events <types>`.
3. **Implement auth.** Write token storage, bearer header, or OAuth flow in `auth.rs`.
4. **Implement API client.** Write HTTP methods in `api.rs` (e.g., `get_issues()`, `get_user()`).
5. **Define models.** Serde-derive structs in `models.rs` for API responses.
6. **Map events.** Implement `EventMapper` in `events.rs`. Emit normalized events with dedupe keys.
7. **Implement trait.** Fill in `manifest()`, `health()`, `sync()` in `lib.rs`.
8. **Test.** Write wiremock + golden-file tests. Verify 429 retry logic.
9. **Register.** Add to workspace `Cargo.toml`, create docs-site page.
10. **Verify.** Run `cargo test -p connector-myname`. Check against **Pre-Merge Checklist** (see reference docs).

## Performance & Rate-Limiting Best Practices

- **Batch requests.** Use GraphQL batch queries or REST list endpoints instead of per-item fetches.
- **Respect backoff.** Parse `Retry-After` header; implement exponential backoff if needed.
- **Cursor semantics.** Save a timestamp or pagination token. Avoid re-fetching the entire history on every sync.
- **Timeout guards.** Set `reqwest::Client` timeouts (`connect_timeout`, `timeout`). Prevent hung syncs.
- **Health checks.** Implement lightweight health() calls (e.g., `GET /user`) that don't count against rate limits (if possible).

## FAQ

**Q: Can I emit the same event twice (two different dedupe_keys)?**
A: Yes, if they represent separate occurrences or state changes. Dedupe keys prevent identical events in the same sync; distinct keys allow multiple events for the same entity.

**Q: What if the API doesn't support cursor-based pagination?**
A: Use a timestamp-based cursor. Store the `max(occurred_at)` from the last sync and query `updated_at > cursor` on the next call. Be careful with clock skew.

**Q: Should I emit events for historical data?**
A: On first sync, yes (with appropriate `occurred_at` timestamps). Subsequent syncs should only emit new/changed entities.

**Q: How do I handle secrets (API keys, OAuth tokens)?**
A: Never log, serialize, or include secrets in events. Store via `SecureSecretStore` (iOS/Android) or environment variables (CLI). Reference only the token ID in logs.

**Q: Can a connector emit events from multiple sources?**
A: Yes. Use distinct `entity_types` and `event_types` in the manifest. Map each to separate `NormalizedEvent` instances.
