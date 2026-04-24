---
title: Auth Flows
description: Implement OAuth 2.0, API key, or token-based authentication.
---

# Authentication Flows

Connectors use one of three authentication strategies: OAuth 2.0, API key, or none (public data).

## OAuth 2.0

Most external services use OAuth 2.0. Your connector will:

1. Redirect user to service OAuth screen
2. Receive authorization code
3. Exchange for access token (backend)
4. Use access token to call service APIs

### Canvas Example

```rust
// Redirect user to Canvas OAuth
let auth_url = format!(
    "https://{}/login/oauth2/auth?client_id={}&redirect_uri={}&response_type=code",
    canvas_host, client_id, redirect_uri
);

// Receive callback with code
let token = exchange_code_for_token(&code)?;

// Use token to call Canvas API
let assignments = canvas_api::get_assignments(&token)?;
```

### FocalPoint OAuth Bridge

FocalPoint provides an OAuth bridge to handle:

1. Secure credential storage (Keychain/Keystore)
2. Token refresh
3. Scope validation
4. Revocation

Your connector calls:

```rust
use focalpoint_sdk::oauth;

let token = oauth::get_or_refresh_token("canvas-lms")?;
```

## API Key

Some services use static API keys. Store securely:

```rust
use focalpoint_sdk::secrets;

let api_key = secrets::get_secret("macrofactor_api_key")?;
```

Users configure API keys in Settings → Connectors → [Connector Name] → API Key.

## No Auth (Public)

Some connectors don't need authentication (e.g., public calendar feeds). Declare:

```toml
[auth]
type = "none"
```

## Token Lifecycle

FocalPoint handles token refresh automatically:

1. **Expired?** → Request new token via refresh_token
2. **Revoked?** → Prompt user to re-authenticate
3. **Invalid scope?** → Show permission re-request dialog

## Security Best Practices

1. **Never hardcode credentials** in your connector code
2. **Use Keychain/Keystore** for local token storage
3. **Validate HTTPS** — always use TLS
4. **Scope minimalism** — request only needed permissions
5. **Audit logging** — log auth events to audit chain

## Testing Auth Flows

Use FocalPoint's test harness:

```bash
focalpoint connector test-auth --manifest connector.toml
```

This will:

1. Simulate OAuth flow
2. Validate token exchange
3. Check permission scopes
4. Test token refresh
