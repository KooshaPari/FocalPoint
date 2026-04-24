# Remote MCP Server: HTTP/SSE + WebSocket

This guide explains how to connect Claude Desktop, Cursor, or other remote agents to FocalPoint's MCP server using HTTP/SSE or WebSocket transports.

## Quick Start

### Option 1: HTTP/SSE (Recommended for Claude Desktop)

Start the server:

```bash
FOCALPOINT_MCP_HTTP_TOKEN="your-secret-token" \
  cargo run --release --features http-sse -- --mode http
```

Server listens at `http://127.0.0.1:8473`. Requires bearer token in `Authorization: Bearer <token>` header.

**Endpoints:**
- `GET /mcp/tools` — Tool catalog (JSON)
- `POST /mcp/tool/{tool_name}` — Invoke tool with JSON payload
- `GET /mcp/events` — SSE stream of responses

### Option 2: WebSocket (Full-Duplex JSON-RPC 2.0)

Start the server:

```bash
FOCALPOINT_MCP_HTTP_TOKEN="your-secret-token" \
  cargo run --release --features websocket -- --mode ws
```

Server listens at `ws://127.0.0.1:8474/mcp/ws`. Uses JSON-RPC 2.0 with bearer authentication.

**Authentication:** Send initial message:

```json
{
  "token": "your-secret-token",
  "id": 0
}
```

Then invoke tools via JSON-RPC:

```json
{
  "jsonrpc": "2.0",
  "method": "focalpoint.tasks.list",
  "params": {},
  "id": 1
}
```

## Integration with Claude Desktop

Claude Desktop requires an HTTP endpoint with proper CORS and authentication. To expose your local MCP server securely:

### 1. Install nginx or use Caddy for reverse proxy

**Using Caddy (simpler, auto-TLS):**

```bash
brew install caddy
```

Create `Caddyfile`:

```
mcp.local:8443 {
  reverse_proxy localhost:8473
  encode gzip
  
  # Require bearer token
  @protected {
    not header Authorization "Bearer {$TOKEN}"
  }
  error @protected 401
}
```

Start Caddy:

```bash
TOKEN="your-secret-token" caddy run
```

### 2. Configure Claude Desktop

Add to `~/.claude/config.json`:

```json
{
  "mcpServers": {
    "focalpoint": {
      "url": "https://mcp.local:8443",
      "auth": {
        "type": "bearer",
        "token": "your-secret-token"
      }
    }
  }
}
```

Claude Desktop will now connect to your local FocalPoint MCP server.

## Integration with Cursor

Cursor uses a similar MCP protocol. Configure in Cursor settings:

```json
{
  "mcpServers": {
    "focalpoint": {
      "transport": "http",
      "url": "http://127.0.0.1:8473",
      "headers": {
        "Authorization": "Bearer your-secret-token"
      }
    }
  }
}
```

## Security Considerations

### 1. Bearer Token Generation

Use a strong, random token (minimum 32 characters):

```bash
openssl rand -hex 32
```

### 2. Network Isolation

- **Development:** Keep server on `127.0.0.1` (localhost only)
- **Local Network:** Use TLS reverse proxy (Caddy) with valid certificates
- **Remote Access:** Use VPN or SSH tunnel, never expose over plain HTTP

### 3. TLS Reverse Proxy (Recommended)

For remote access, always use TLS:

```bash
# Start FocalPoint MCP on localhost
FOCALPOINT_MCP_HTTP_TOKEN="secret" \
  cargo run --features http-sse -- --mode http

# Expose via Caddy with auto HTTPS
caddy reverse-proxy --from mcp.example.com --to localhost:8473
```

### 4. Token Rotation

Periodically rotate your bearer token:

```bash
# Generate new token
NEW_TOKEN=$(openssl rand -hex 32)

# Update Caddy
TOKEN="$NEW_TOKEN" caddy reload

# Update Claude/Cursor config
```

## Rate Limiting

Both transports enforce rate limiting: **100 requests/minute per connection**.

- **HTTP/SSE:** Returns HTTP 429 on excess
- **WebSocket:** Returns JSON-RPC error code -32000

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32000,
    "message": "Rate limit exceeded: 100 req/min"
  },
  "id": 1
}
```

## Configuration Reference

### Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `FOCALPOINT_MCP_HTTP_ADDR` | `127.0.0.1:8473` | HTTP/SSE bind address |
| `FOCALPOINT_MCP_WS_ADDR` | `127.0.0.1:8474` | WebSocket bind address |
| `FOCALPOINT_MCP_HTTP_TOKEN` | `focalpoint-default-insecure-token` | Bearer authentication token |
| `FOCALPOINT_DB` | Platform default | Path to FocalPoint database |

### CLI Flags

```bash
focalpoint-mcp-server --help

Usage: focalpoint-mcp-server [OPTIONS]

Options:
  --mode <MODE>    Transport mode: stdio (default), http, ws [default: stdio]
  --db <PATH>      Path to FocalPoint core.db
  -h, --help       Print help
```

## Troubleshooting

### "Connection refused" on 127.0.0.1:8473

Check if server is running:

```bash
lsof -i :8473
```

If not, start the server with HTTP mode:

```bash
FOCALPOINT_MCP_HTTP_TOKEN="token" cargo run --features http-sse -- --mode http
```

### "Authentication failed"

Verify your bearer token matches in:

1. Server startup: `FOCALPOINT_MCP_HTTP_TOKEN="..."`
2. Client request: `Authorization: Bearer ...`

### "Rate limit exceeded"

Requests are limited to 100/min per connection. If you're hitting this:

- Batch requests where possible
- Use WebSocket for lower latency (full-duplex)
- Increase time between requests

### "Feature not enabled"

Ensure you compiled with the correct feature flag:

```bash
# For HTTP/SSE
cargo build --release --features http-sse

# For WebSocket
cargo build --release --features websocket

# For both
cargo build --release --features remote
```

## Tool Catalog (27 Tools)

All tools are available via HTTP/SSE and WebSocket transports. See [FocalPoint Tools](../api/tools.md) for complete reference.

**Read-only (15):**
- `focalpoint.tasks.list` — List all tasks
- `focalpoint.rules.list` — List all rules
- `focalpoint.wallet.balance` — Get wallet balance
- And 12 more...

**Write (12):**
- `focalpoint.tasks.add` — Create task
- `focalpoint.rules.upsert` — Update rule
- And 10 more...

## Performance Tips

1. **Use WebSocket for sustained sessions** — Lower latency, full-duplex
2. **Batch tool calls** — HTTP/SSE requires separate requests per tool
3. **Reuse connections** — Keep TLS handshakes minimal with reverse proxy
4. **Monitor rate limits** — Log 429/429 errors to detect bottlenecks

## Advanced: Self-Hosted Deployment

For production environments:

```bash
# 1. Start MCP server on internal network
FOCALPOINT_MCP_HTTP_ADDR="0.0.0.0:8473" \
FOCALPOINT_MCP_HTTP_TOKEN="$(openssl rand -hex 32)" \
  focalpoint-mcp-server --mode http &

# 2. Expose via Caddy with HTTPS, auth, rate limiting
caddy run # with custom Caddyfile
```

See [Deployment Guide](../deployment/mcp.md) for Kubernetes, Docker Compose, and systemd setups.

## See Also

- [MCP Protocol Spec](https://modelcontextprotocol.io/)
- [Claude API Docs](https://claude.ai/docs)
- [FocalPoint Architecture](../architecture/mcp-server.md)
