---
title: FocalPoint Webhook Server
description: FocalPoint documentation
---
# FocalPoint Webhook Server

## Overview

`focalpoint-webhook-server` is a binary that listens for incoming webhook deliveries from GitHub, Canvas, and Google Calendar, verifies their signatures, and dispatches them to the appropriate handler for normalization into `NormalizedEvent`s.

## Running the Server

### Development

```bash
cargo run -p focus-webhook-server -- \
  --bind 127.0.0.1:8472 \
  --db ~/.focalpoint/core.db
```

### Environment Variables

The server reads provider credentials from environment:

- `FOCALPOINT_GITHUB_WEBHOOK_SECRET` — GitHub webhook secret (HMAC-SHA256).
  If not set, GitHub handler is not registered.
- `FOCALPOINT_CANVAS_JWKS_URL` — Canvas JWKS endpoint for JWT validation.
  If not set, Canvas handler is not registered.
- `FOCALPOINT_GCAL_CHANNEL_TOKEN` — Google Calendar watch channel token.
  If not set, GCal handler is not registered.

### Flags

- `--bind HOST:PORT` (default: `127.0.0.1:8472`) — TCP address to listen on.
- `--db PATH` (default: empty) — Path to `core.db` for future integration.

### Health Check

```bash
curl http://127.0.0.1:8472/healthz
# {"status":"ok"}
```

## Webhook Configuration

### GitHub

1. Go to your GitHub repository → Settings → Webhooks.
2. Click "Add webhook".
3. **Payload URL**: `https://<your-domain>/webhooks/github`
4. **Content type**: `application/json`
5. **Secret**: Generate a random secret and set `FOCALPOINT_GITHUB_WEBHOOK_SECRET=<secret>`.
6. **Events**: Select events (e.g. "Push", "Pull requests", "Issues", "Issue comments").
7. Click "Add webhook".

**Note**: The server verifies the `X-Hub-Signature-256` header using constant-time comparison to prevent timing attacks.

### Canvas (LTI)

Canvas webhooks use JWT authentication. Configuration varies by Canvas admin settings.

1. Set `FOCALPOINT_CANVAS_JWKS_URL` to your Canvas instance's JWKS endpoint.
2. **Payload URL**: `https://<your-domain>/webhooks/canvas`
3. The server validates the `X-Canvas-LTI-JWT` header structure (stub implementation; full JWKS verification is pending).

### Google Calendar

Google Calendar uses watch channels with channel tokens.

1. Set `FOCALPOINT_GCAL_CHANNEL_TOKEN` to a secure random token.
2. Subscribe to watch channels via the GCal API (`POST /calendars/{calendarId}/events/watch`).
3. The server will verify `X-Goog-Channel-Token` matches the configured token.

## Signature Verifiers

### GitHub HMAC-SHA256

- **Header**: `X-Hub-Signature-256`
- **Format**: `sha256=<hex-encoded-digest>`
- **Algorithm**: `HMAC-SHA256(secret, body)`
- **Constant-time comparison**: Yes (via `subtle` crate)

### Canvas LTI JWT (Stub)

- **Header**: `X-Canvas-LTI-JWT`
- **Format**: 3-part JWT (`header.payload.signature`)
- **Current**: Structure validation only
- **TODO**: Fetch JWKS, validate signature + `iss`/`aud`/`exp`

### Google Calendar Channel Token

- **Header**: `X-Goog-Channel-Token`
- **Format**: Opaque token string
- **Constant-time comparison**: Yes (via `subtle` crate)

## Production Deployment

### TLS Termination

Always run the server **behind a reverse proxy** (nginx, Caddy, etc.) that handles TLS termination. GitHub requires HTTPS for webhooks.

Example nginx config:

```nginx
upstream webhook_server {
    server 127.0.0.1:8472;
}

server {
    listen 443 ssl http2;
    server_name webhooks.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location /webhooks/ {
        proxy_pass http://webhook_server;
        proxy_set_header X-Forwarded-For $remote_addr;
        proxy_set_header X-Forwarded-Proto https;
    }
}
```

### Rate Limiting

Add rate limiting at the proxy layer:

```nginx
limit_req_zone $remote_addr zone=webhook_limit:10m rate=100r/s;

location /webhooks/ {
    limit_req zone=webhook_limit burst=10 nodelay;
    proxy_pass http://webhook_server;
}
```

### Logging

The server uses `tracing` for structured logging. Control verbosity with the `RUST_LOG` environment variable:

```bash
RUST_LOG=focus_webhook_server=debug cargo run -p focus-webhook-server -- --bind 0.0.0.0:8472
```

### Systemd Service (Example)

```ini
[Unit]
Description=FocalPoint Webhook Server
After=network.target

[Service]
Type=simple
User=focalpoint
WorkingDirectory=/opt/focalpoint
ExecStart=/opt/focalpoint/bin/focalpoint-webhook-server \
  --bind 127.0.0.1:8472 \
  --db /opt/focalpoint/data/core.db

Environment="FOCALPOINT_GITHUB_WEBHOOK_SECRET=<secret>"
Environment="FOCALPOINT_CANVAS_JWKS_URL=https://canvas.example.com/.well-known/jwks.json"
Environment="FOCALPOINT_GCAL_CHANNEL_TOKEN=<token>"
Environment="RUST_LOG=focus_webhook_server=info"

Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

## HTTP Response Codes

| Code | Meaning |
|------|---------|
| 202  | Webhook accepted and queued for processing |
| 401  | Signature verification failed |
| 404  | Unknown connector ID |
| 500  | Internal processing error |

## Testing Locally

### GitHub Webhook

Generate a valid HMAC signature and POST:

```bash
SECRET="my-secret"
PAYLOAD='{"action":"opened",...}'

# Compute HMAC-SHA256
SIG=$(echo -n "$PAYLOAD" | openssl dgst -sha256 -hmac "$SECRET" -hex | cut -d' ' -f2)

curl -X POST http://127.0.0.1:8472/webhooks/github \
  -H "X-Hub-Signature-256: sha256=$SIG" \
  -H "Content-Type: application/json" \
  -d "$PAYLOAD"
```

## Future Enhancements

- [ ] Full Canvas LTI JWT validation with JWKS fetching
- [ ] Google Calendar watch channel event parsing
- [ ] Per-connector rate limiting
- [ ] Dead letter queue for failed deliveries
- [ ] Webhook event replay via UI
