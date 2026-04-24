# Templates Registry Server

Marketplace service for FocalPoint template packs. Standalone Rust + Axum service with SQLite catalog.

## Features

- **Search**: Full-text search over pack name/author (60 req/min per IP)
- **Browse**: Fetch pack manifests, signatures, and READMEs
- **Rate**: Submit 1-5 star ratings (anonymous, per-IP limited, 10 req/min)
- **Upload**: Publish new packs with ed25519 signature verification (bearer token auth)
- **Catalog Loading**: Auto-discover packs from `examples/templates/` on startup

## Endpoints

All endpoints return JSON. Base path: `/api/v1`.

### `GET /api/v1/search?q=<query>`

Search packs by name or author substring (60 req/min per IP).

### `GET /api/v1/packs/:id`

Fetch pack manifest with signature and README.

### `POST /api/v1/packs/:id/rate`

Submit a 1-5 star rating (anonymous, 10 req/min per IP).

### `POST /api/v1/packs`

Upload new pack (requires `Authorization: Bearer <token>`, 10 req/min per IP).

## Running Locally

```bash
cd services/templates-registry

TEMPLATES_REGISTRY_BIND=0.0.0.0:8080 \
TEMPLATES_REGISTRY_DB=/tmp/templates.db \
TEMPLATES_CATALOG_PATH=examples/templates \
TEMPLATES_REGISTRY_TOKEN=dev-token \
cargo run
```

## Environment Variables

| Variable | Default |
|----------|---------|
| `TEMPLATES_REGISTRY_BIND` | `127.0.0.1:8080` |
| `TEMPLATES_REGISTRY_DB` | `/tmp/templates.db` |
| `TEMPLATES_CATALOG_PATH` | `examples/templates` |
| `TEMPLATES_REGISTRY_TOKEN` | `dev-token-change-in-prod` |
| `RUST_LOG` | (not set) |

## Testing

```bash
cargo test -p templates-registry
```

8+ tests verify: search filtering, pack retrieval, rating validation (1-5), rate limiting, bearer auth, anonymous rating, signature verification, ownership validation.

## Deployment

See `docs-site/guides/templates_marketplace_hosting.md` for Docker, systemd, Fly.io, and Kubernetes recipes.

## API Reference

See `services/templates-registry/openapi.yaml` for complete OpenAPI 3.0 spec.

## Architecture

```
handlers.rs       — HTTP endpoints
  ├─ db.rs        — SQLite queries (packs, ratings, search)
  ├─ ratelimit.rs — Token bucket per IP
  ├─ auth.rs      — Bearer token validation (const-time)
  └─ error.rs     — HTTP error responses
```

## License

Part of FocalPoint. See LICENSE.
