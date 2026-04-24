# Connector SDK: 10-Minute Getting Started

This guide walks you through scaffolding a connector, implementing the stub methods, and registering it in your app.

## Prerequisites

- Rust 1.75+
- FocalPoint workspace checked out
- `cargo` in PATH

## Step 1: Generate the Scaffold

```bash
cd /path/to/FocalPoint
./target/debug/focus connector new my-connector --tier private --auth apikey
```

Output:
```
✓ connector scaffold created: crates/connector-my-connector
  crate: connector-my-connector
  tier: private
  auth: apikey
  sync: polling

Next steps:
  1. cd crates/connector-my-connector
  2. cargo check -p connector-my-connector
  3. Edit src/lib.rs, src/api.rs, src/auth.rs, src/events.rs
  4. cargo test -p connector-my-connector
  5. Register in ConnectorRegistry via register_my_connector()
```

## Step 2: Verify It Compiles

```bash
cd crates/connector-my-connector
cargo check -p connector-my-connector
```

You should see no errors (the scaffold is valid Rust).

## Step 3: Implement Three Methods

### Method 1: Stub the API Client

Open `src/api.rs`. You'll find:

```rust
pub async fn hello(&self) -> Result<String> {
    debug!("MyConnectorClient: hello");
    Ok("Hello from MyConnector".to_string())
}
```

**Replace** with your real API calls. Example (for GitHub):

```rust
pub async fn list_issues(&self, owner: &str, repo: &str) -> Result<Vec<Issue>> {
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    self.get::<Vec<Issue>>(&url).await
}
```

### Method 2: Implement Auth

Open `src/auth.rs`. The scaffold generates the right module based on your `--auth` flag.

**For OAuth2** (`--auth oauth2`):
```rust
pub async fn refresh_token(&self, refresh_token: &str) -> Result<String> {
    let req = self.client.refresh_token(RefreshToken::new(refresh_token.to_string()));
    let token = req.request_async(&async_http_client).await?;
    Ok(token.access_token().secret().clone())
}
```

**For API Key** (`--auth apikey`):
```rust
// Already stubbed. Just verify the key is stored securely:
let auth = MyConnectorApiKey::new(env::var("MY_CONNECTOR_API_KEY")?);
let key = auth.key(); // returns SecretString
```

**For None** (`--auth none`):
```rust
// No auth needed; public APIs only.
```

### Method 3: Map Events

Open `src/events.rs`. Currently it returns `None`:

```rust
pub fn map_event(_raw: &str) -> Option<Event> {
    // TODO: Parse raw JSON/data and map to Event
    None
}
```

Implement it to parse your service's webhook/API payloads:

```rust
use serde_json::Value;

pub fn map_event(raw: &str) -> Option<Event> {
    let json: Value = serde_json::from_str(raw).ok()?;
    
    match json.get("type")?.as_str()? {
        "issue_opened" => {
            let issue_id = json.get("issue")?.get("id")?.as_str()?;
            Some(Event {
                id: Uuid::new_v4(),
                connector: "my-connector".into(),
                event_type: "issue:opened".into(),
                payload: serde_json::json!({ "issue_id": issue_id }),
                emitted_at: Utc::now(),
            })
        }
        _ => None,
    }
}
```

## Step 4: Run Tests

```bash
cargo test -p connector-my-connector
```

The scaffold includes a wiremock integration test that mocks `/hello`:

```rust
#[tokio::test]
async fn test_hello_endpoint() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(wiremock::matchers::path("/hello"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Hello from MyConnector"))
        .mount(&mock_server)
        .await;

    let client = MyConnectorClient::new(reqwest::Client::new());
    let result = client.hello().await;

    assert!(result.is_ok());
}
```

**Add more tests** for your real API methods (also using wiremock).

## Step 5: Register in Your App

In your app's initialization code (e.g., `src/main.rs` or `main.rs`):

```rust
use connector_my_connector::register_my_connector;
use focus_connectors::ConnectorRegistry;

fn main() {
    let registry = ConnectorRegistry::new();
    registry.register(register_my_connector());
    // ... rest of app init
}
```

The `register_my_connector()` function creates a `Arc<dyn Connector>` that the app can track and sync on a cadence.

## Step 6: Run the Sync (if integrated)

If your app has a sync orchestrator:

```rust
let outcome = registry.connector("my-connector")
    .sync()
    .await?;
    
println!("Events pulled: {}", outcome.events_pulled);
println!("Errors: {:?}", outcome.errors);
```

## Command-Line Options Reference

```bash
focus connector new <name> [--tier TIER] [--auth AUTH] [--sync-mode SYNC]
```

| Flag | Default | Values |
|------|---------|--------|
| `--tier` | `private` | `official`, `verified`, `mcp-bridged`, `private` |
| `--auth` | `none` | `oauth2`, `apikey`, `none` |
| `--sync-mode` | `polling` | `polling`, `webhook`, `hybrid` |

### Tier Semantics

- **official**: Built-in, Google-signed, blessed by FocalPoint team.
- **verified**: Audited by FocalPoint security team; production-ready for third-party services.
- **mcp-bridged**: Wrapped via MCP (Model Context Protocol); delegates to an MCP host.
- **private**: Your service; no audit; for personal/team use.

### Auth Semantics

- **oauth2**: OAuth2 bearer token + refresh flow. Connector stores tokens securely (keychain feature).
- **apikey**: Static API key. Stored in SecretString (OS keychain if available).
- **none**: No auth; public APIs only.

### Sync Mode Semantics

- **polling**: Connector ticks on a cadence (e.g., every 5 min). App-controlled.
- **webhook**: Service sends events to your app; connector just validates + persists.
- **hybrid**: Starts polling; upgrades to webhook if service supports it; falls back to polling on webhook failure.

## Troubleshooting

### `cargo check` fails

Verify the workspace root's `Cargo.toml` has the new crate in `members`:

```toml
[workspace]
members = [
    "crates/focus-connectors",
    "crates/connector-my-connector",  # <- Check it's here
    # ...
]
```

If missing, edit `Cargo.toml` manually or re-run scaffold (it attempts to update for you).

### Auth module doesn't have the right variant

Pass `--auth` when scaffolding:

```bash
# Regenerate with correct auth
focus connector new my-connector --tier private --auth oauth2
```

(The old crate will not be overwritten; remove `crates/connector-my-connector` first if you want a fresh start.)

### Tests fail with wiremock errors

Ensure `wiremock = "0.6"` is in `Cargo.toml` dev-dependencies (it is, by default).

### Events not being mapped

Add debug logging in `src/events.rs`:

```rust
use tracing::{debug, warn};

pub fn map_event(raw: &str) -> Option<Event> {
    debug!("Mapping raw event: {}", raw);
    // ... your code
    warn!("Could not map event: {}", raw);
    None
}
```

Run with `RUST_LOG=debug cargo test` to see logs.

## Next Steps

- **Persistence:** Add a `StateStore` trait to cache connector state (last sync time, cursor, etc.).
- **Error handling:** Extend `ConnectorError` with service-specific errors.
- **Health checks:** Implement `health()` to return `Healthy`, `Degraded`, or `Offline`.
- **Metrics:** Integrate `prometheus` for sync duration, event count, error rate.
- **Documentation:** Add service-specific notes to `README.md`.

## Reference

- [Connector trait](../../crates/focus-connectors/src/lib.rs) — the public contract
- [Event model](../../crates/focus-events/src/lib.rs) — event schema
- [Canvas example](../../crates/connector-canvas) — full reference implementation
