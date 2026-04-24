# Webhook Integration Guide

FocalPoint webhook server (`focalpoint-webhook-server`) receives push events from GitHub, Canvas, and Google Calendar. This guide explains how to configure each provider and deploy the receiver.

## Overview

The webhook server listens on `POST /webhooks/:connector_id` and `POST /webhooks/:connector_id/:event_type`. Each request is:

1. **Rate-limited**: 100 requests/minute per IP (returns 429 if exceeded)
2. **Signature-verified**: Provider-specific HMAC or JWT verification
3. **Normalized**: Converted to `NormalizedEvent` and forwarded to focus-eval
4. **Audited**: Health metrics track per-connector activity

## Deployment

### Prerequisites

- FocalPoint webhook server binary or Docker image
- TLS certificate (Let's Encrypt or self-signed for testing)
- Public domain or IP reachable from the internet
- Firewall rules allowing inbound HTTPS

### Basic Setup

```bash
# Start the webhook server on 127.0.0.1:8472 (development)
focalpoint-webhook-server \
  --bind 127.0.0.1:8472 \
  --db /path/to/core.db

# Or via Docker
docker run -e FOCALPOINT_GITHUB_WEBHOOK_SECRET=... \
           -e FOCALPOINT_CANVAS_JWKS_URL=... \
           -e FOCALPOINT_GCAL_CHANNEL_TOKEN=... \
           -p 8472:8472 \
           focalpoint-webhook-server
```

### TLS with Nginx Reverse Proxy

For production, place FocalPoint behind a reverse proxy with TLS:

```nginx
upstream focalpoint_webhook {
    server 127.0.0.1:8472;
}

server {
    listen 443 ssl http2;
    server_name webhooks.focalpoint.io;

    ssl_certificate /etc/letsencrypt/live/webhooks.focalpoint.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/webhooks.focalpoint.io/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    # Rate limit at nginx layer (optional, in addition to server-side)
    limit_req_zone $binary_remote_addr zone=webhook_limit:10m rate=10r/s;
    limit_req zone=webhook_limit burst=20 nodelay;

    location / {
        proxy_pass http://focalpoint_webhook;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_buffering off;
        proxy_request_buffering off;
    }
}

server {
    listen 80;
    server_name webhooks.focalpoint.io;
    return 301 https://$server_name$request_uri;
}
```

## GitHub Webhook Configuration

### 1. Set Environment Variable

```bash
export FOCALPOINT_GITHUB_WEBHOOK_SECRET="<your-webhook-secret>"
```

### 2. Register in GitHub

Navigate to your repository → Settings → Webhooks → Add webhook:

- **Payload URL**: `https://webhooks.focalpoint.io/webhooks/github`
- **Content type**: `application/json`
- **Secret**: (same as `FOCALPOINT_GITHUB_WEBHOOK_SECRET`)
- **Events**: Select events to subscribe to:
  - `push` — code pushed to branches
  - `pull_request` — PR opened, closed, synchronized
  - `workflow_run` — CI job completed
  - Custom events as needed
- **Active**: ✓

### 3. Verify

The server extracts `X-GitHub-Event` header to determine event type. HMAC-SHA256 signature is validated against the secret.

**Health check:**

```bash
curl https://webhooks.focalpoint.io/healthz
```

Response:

```json
{
  "status": "ok",
  "timestamp": "2026-04-24T10:00:00Z",
  "connectors": {
    "github": {
      "last_received_at": "2026-04-24T09:55:00Z",
      "hmac_success_count": 42,
      "hmac_failure_count": 0,
      "last_hour_count": 45
    }
  }
}
```

## Canvas LTI Webhook Configuration

### 1. Obtain Canvas JWKS URL

Contact your Canvas administrator for the JWK Set endpoint. Typically:

```
https://<canvas-instance>.instructure.com/.well-known/jwks.json
```

### 2. Set Environment Variables

```bash
export FOCALPOINT_CANVAS_JWKS_URL="https://<canvas-instance>.instructure.com/.well-known/jwks.json"
# Optional: set expected issuer/audience if Canvas enforces them
export FOCALPOINT_CANVAS_ISS="https://<canvas-instance>.instructure.com"
export FOCALPOINT_CANVAS_AUD="https://<your-focalpoint-domain>"
```

### 3. Register in Canvas

As an LTI 1.3 tool consumer:

- **LTI Platform**: Canvas
- **Webhook URL**: `https://webhooks.focalpoint.io/webhooks/canvas`
- **Events**: `assignment_submission`, `course_update`, `user_enrollment`, etc.

Canvas will deliver JWT in `X-Canvas-LTI-JWT` header, signed with its private key.

### 4. JWT Verification Flow

- Webhook server fetches JWKS from `FOCALPOINT_CANVAS_JWKS_URL` (cached for 10 minutes)
- Extracts `kid` from JWT header
- Looks up corresponding key in JWKS
- Validates:
  - JWT signature (RS256)
  - `iss` claim (if `FOCALPOINT_CANVAS_ISS` set)
  - `aud` claim (if `FOCALPOINT_CANVAS_AUD` set)
  - `exp` (not expired)
  - `iat` (not issued in future)

Rejected tokens return **401 Unauthorized**; processing errors return **500**.

## Google Calendar Watch Configuration

### 1. Set Channel Token

```bash
export FOCALPOINT_GCAL_CHANNEL_TOKEN="<unique-secret-token>"
```

### 2. Create Watch Channel

Via Google Calendar API:

```bash
curl -X POST https://www.googleapis.com/calendar/v3/calendars/primary/watch \
  -H "Authorization: Bearer <access-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "focalpoint-watch-1",
    "type": "web_hook",
    "address": "https://webhooks.focalpoint.io/webhooks/gcal",
    "params": {
      "ttl": "604800"
    },
    "token": "focalpoint-gcal-secret"
  }'
```

### 3. Verification

GCal sends `X-Goog-Channel-Token` header in each notification. The server validates it against `FOCALPOINT_GCAL_CHANNEL_TOKEN` via constant-time comparison.

## Rate Limiting

The server enforces **100 requests/minute per IP**. Excess requests receive:

```
HTTP 429 Too Many Requests
Content-Type: text/plain
Rate limit exceeded
```

**For testing or high-volume scenarios**, adjust via environment variable or code:

```rust
// In main.rs: adjust Quota::per_minute(100) to desired limit
let rate_limiter = Arc::new(
    RateLimiter::direct(tower_governor::Quota::per_minute(std::num::NonZeroU32::new(500).unwrap()))
);
```

## Health Endpoint

`GET /healthz` returns server status and per-connector metrics:

```json
{
  "status": "ok",
  "timestamp": "2026-04-24T10:00:00Z",
  "connectors": {
    "github": {
      "last_received_at": "2026-04-24T09:55:00Z",
      "hmac_success_count": 100,
      "hmac_failure_count": 2,
      "last_hour_count": 150
    },
    "canvas": {
      "last_received_at": null,
      "hmac_success_count": 0,
      "hmac_failure_count": 0,
      "last_hour_count": 0
    }
  }
}
```

Fields:
- `last_received_at` — timestamp of last successfully received webhook
- `hmac_success_count` — cumulative signature-verification successes
- `hmac_failure_count` — cumulative signature-verification failures
- `last_hour_count` — requests in last 60 minutes (approximate)

## Event Type Routing

Webhooks can specify event type in URL path:

```
POST /webhooks/:connector_id/:event_type
```

Example:

```bash
curl -X POST https://webhooks.focalpoint.io/webhooks/github/pull_request \
  -H "X-Hub-Signature-256: sha256=..." \
  -H "Content-Type: application/json" \
  -d '{...}'
```

The server extracts event type from either the URL path or provider-specific headers (`X-GitHub-Event`, `X-Canvas-Event`, `X-Goog-Resource-State`). URL path takes precedence.

## Troubleshooting

### 401 Unauthorized

**Cause**: Signature verification failed.

- **GitHub**: Verify `FOCALPOINT_GITHUB_WEBHOOK_SECRET` matches the registered secret in GitHub settings.
- **Canvas**: Verify `FOCALPOINT_CANVAS_JWKS_URL` is accessible and returns valid JWKS; check JWT header `kid` matches a key in JWKS.
- **GCal**: Verify `FOCALPOINT_GCAL_CHANNEL_TOKEN` matches the token sent in `X-Goog-Channel-Token` header.

### 429 Too Many Requests

**Cause**: Rate limit exceeded (100 req/min per IP).

**Solution**:
- Space out requests to <100/min.
- Use batch APIs where available.
- Request rate limit increase if needed; adjust server config.

### 500 Internal Server Error

**Cause**: Payload processing failed after signature verification.

**Solution**:
- Check server logs: `focalpoint-webhook-server` logs at `TRACE`/`DEBUG` level.
- Verify connector handler exists: `GET /healthz` lists registered connectors.
- Check `focus-sync` event-sink is running and accepting events.

### 404 Not Found

**Cause**: Connector not registered.

**Solution**:
- Check environment variables are set:
  - `FOCALPOINT_GITHUB_WEBHOOK_SECRET` for GitHub
  - `FOCALPOINT_CANVAS_JWKS_URL` for Canvas
  - `FOCALPOINT_GCAL_CHANNEL_TOKEN` for GCal
- Restart server if env vars were added.

## Security Considerations

1. **TLS Required**: Always use HTTPS for webhook URLs. Unencrypted webhooks expose secrets.
2. **Secrets in Environment**: Store secrets in `.env` or secrets management system (not in code).
3. **IP Allowlisting**: If provider supports, configure IP allowlist (GitHub's IP ranges, Canvas IP ranges, GCal IP ranges).
4. **Signature Validation**: Server validates all signatures using constant-time comparison to prevent timing attacks.
5. **Cache TTL**: Canvas JWKS cached for 10 minutes; fresh keys fetched if not found (allows key rotation).

## Testing Locally

For development without TLS:

```bash
# Start server
focalpoint-webhook-server --bind 127.0.0.1:8472

# Test GitHub HMAC (from another terminal)
SECRET="test-secret"
BODY='{"repository":{"name":"test"}}'
SIG=$(echo -n "$BODY" | openssl dgst -sha256 -hmac "$SECRET" -hex | cut -d' ' -f2)

curl -X POST http://127.0.0.1:8472/webhooks/github \
  -H "X-Hub-Signature-256: sha256=$SIG" \
  -H "Content-Type: application/json" \
  -d "$BODY"

# Test health endpoint
curl http://127.0.0.1:8472/healthz | jq
```

## Integration with focus-eval

Webhooks are normalized to `NormalizedEvent` and pushed to `focus-sync`'s event sink. The sync pipeline then triggers `focus-eval` to process rules and emit rewards/penalties. See [focus-eval docs](../reference/focus-eval.md) for rule evaluation flow.
