# Self-Hosting a FocalPoint Templates Marketplace

Host your own FocalPoint template registry for your team, organization, or community.

## Why Self-Host?

- **Privacy**: Templates stay on your infrastructure
- **Control**: Curate the catalog for your specific use case
- **Offline**: Serve cached packs without internet dependency
- **Integration**: Connect to your existing auth, monitoring, and deployment systems

## Quick Start (5 min)

### 1. Clone and build the registry

```bash
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint/services/templates-registry

# Build the binary
cargo build --release

# Or use Docker
docker build -t templates-registry:latest .
```

### 2. Prepare catalog

Create a directory with your template packs:

```bash
mkdir -p /opt/templates
cp examples/templates/*.toml /opt/templates/
# (optional) Add READMEs alongside each TOML
# /opt/templates/deep-work-starter.toml
# /opt/templates/deep-work-starter.md
```

Each `.md` file (alongside a `.toml`) is auto-loaded as that pack's README.

### 3. Start the server

```bash
export TEMPLATES_REGISTRY_BIND=0.0.0.0:8080
export TEMPLATES_REGISTRY_DB=/data/templates.db
export TEMPLATES_CATALOG_PATH=/opt/templates
export TEMPLATES_REGISTRY_TOKEN=your-secure-token-here

./target/release/templates-registry
```

Visit `http://localhost:8080/api/v1/search?q=deep` to test.

## Configuration

### Environment Variables

| Variable | Default | Notes |
|----------|---------|-------|
| `TEMPLATES_REGISTRY_BIND` | `127.0.0.1:8080` | Listen on all interfaces: `0.0.0.0:8080` |
| `TEMPLATES_REGISTRY_DB` | `/tmp/templates.db` | Use persistent path: `/var/lib/templates/db.sqlite` |
| `TEMPLATES_CATALOG_PATH` | `examples/templates` | Directory scanned for `.toml` files on startup |
| `TEMPLATES_REGISTRY_TOKEN` | `dev-token-change-in-prod` | **Change in production.** 32+ char random string. |
| `RUST_LOG` | (not set) | Set to `debug` or `info` for logging |

### Token Generation

Generate a secure token:

```bash
# macOS/Linux
openssl rand -hex 32

# Or use a password manager
pwgen -sy 32 1
```

## Deployment Recipes

### Docker Compose (simplest)

```yaml
# docker-compose.yml
version: "3.9"

services:
  templates-registry:
    image: templates-registry:latest
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      TEMPLATES_REGISTRY_BIND: "0.0.0.0:8080"
      TEMPLATES_REGISTRY_DB: "/data/db.sqlite"
      TEMPLATES_REGISTRY_TOKEN: "${REGISTRY_TOKEN}"
      RUST_LOG: "info"
    volumes:
      - templates-data:/data
      - ./catalogs/production:/app/templates:ro
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/v1/search?q=a"]
      interval: 30s
      timeout: 5s
      retries: 3

volumes:
  templates-data:
    driver: local
```

Run:

```bash
export REGISTRY_TOKEN=$(openssl rand -hex 32)
docker-compose up -d
```

### systemd Service (on Linux)

```ini
# /etc/systemd/system/templates-registry.service
[Unit]
Description=FocalPoint Templates Registry
After=network.target

[Service]
Type=simple
User=templates
Group=templates
WorkingDirectory=/opt/templates-registry
ExecStart=/opt/templates-registry/bin/templates-registry
Restart=on-failure
RestartSec=10

Environment="TEMPLATES_REGISTRY_BIND=0.0.0.0:8080"
Environment="TEMPLATES_REGISTRY_DB=/var/lib/templates/db.sqlite"
Environment="TEMPLATES_CATALOG_PATH=/opt/templates"
Environment="TEMPLATES_REGISTRY_TOKEN=YOUR_SECRET_TOKEN"
Environment="RUST_LOG=info"

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=yes

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable templates-registry
sudo systemctl start templates-registry
sudo systemctl status templates-registry
```

### Fly.io Deployment

```toml
# fly.toml
app = "templates-registry"
primary_region = "sfo"

[build]
  dockerfile = "Dockerfile"

[env]
  TEMPLATES_REGISTRY_BIND = "0.0.0.0:8080"
  TEMPLATES_REGISTRY_DB = "/data/db.sqlite"
  RUST_LOG = "info"

[[mounts]]
source = "templates_data"
destination = "/data"

[[services]]
  internal_port = 8080
  protocol = "tcp"

  [[services.ports]]
    port = 80
    handlers = ["http"]

  [[services.ports]]
    port = 443
    handlers = ["tls", "http"]
```

Deploy:

```bash
flyctl launch --name templates-registry
flyctl secrets set TEMPLATES_REGISTRY_TOKEN=your-secret
flyctl deploy
```

## Catalog Management

### Add a Pack

1. Create or obtain a template pack TOML
2. Optionally create a `.md` README alongside it
3. Copy to your catalog directory
4. Restart the service

Example:

```bash
cat > /opt/templates/company-focus.toml <<EOF
id = "company-focus"
name = "Company Focus"
version = "0.1.0"
author = "engineering-team"
description = "Rules enforced company-wide"

[[rules]]
id = "work-hours"
...
EOF

cat > /opt/templates/company-focus.md <<EOF
# Company Focus

Our standard set of focus rules for during work hours.
...
EOF

sudo systemctl restart templates-registry
```

## CLI Integration

Configure the `focus` CLI to use your registry:

```bash
export FOCALPOINT_TEMPLATE_REGISTRY=https://templates.myorg.com
focus templates search deep
focus templates show deep-work-starter
focus templates rate deep-work-starter --rating 5
```

## Troubleshooting

### Server won't start

Check logs:

```bash
# Docker
docker-compose logs -f templates-registry

# systemd
journalctl -u templates-registry -n 50

# Direct run
RUST_LOG=debug ./templates-registry
```

### Search returns empty

Ensure catalog path exists and has `.toml` files:

```bash
ls -la /opt/templates/*.toml
```

## License

FocalPoint Templates Registry is part of FocalPoint. See LICENSE.
