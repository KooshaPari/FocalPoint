# Observability Runbook

This guide covers structured logging, distributed tracing, and metrics for FocalPoint.

## Setup

### 1. Initialize Tracing

In your main binary:

```rust
use focus_observability::init_tracing;

#[tokio::main]
async fn main() {
    // Initialize tracing with JSON output (production) or pretty printing (dev)
    init_tracing("focus-sync", None); // Respects FOCALPOINT_LOG_LEVEL env var
    
    // ... rest of app
}
```

**Environment Variables:**
- `FOCALPOINT_LOG_LEVEL` — `trace`, `debug`, `info`, `warn`, `error`. Default: `info`.
- `FOCALPOINT_LOG_FORMAT` — `json` or `pretty`. Default: `json`.
- `RUST_LOG` — Standard tracing filter (e.g., `focus_sync=debug,focus_eval=trace`).

### 2. Optional: Export to OpenTelemetry

Export distributed traces to Jaeger, Grafana Tempo, or Honeycomb:

```rust
use focus_observability::init_otel;

#[tokio::main]
async fn main() {
    init_tracing("focus-sync", None);
    
    // Export traces to a local Jaeger instance (port 4317 = gRPC OTLP)
    if let Err(e) = init_otel(Some("http://localhost:4317")).await {
        eprintln!("OTEL init failed: {}", e);
    }
    
    // ... rest of app
}
```

### 3. Emit Metrics

Access the global metrics registry from anywhere:

```rust
use focus_observability::MetricsRegistry;
use std::time::Instant;

let metrics = MetricsRegistry::global();

// Sync operation
let start = Instant::now();
sync_connector("github").await;
let duration = start.elapsed().as_secs_f64();

metrics.inc_connector_syncs("github", 1.0);
metrics.record_sync_duration("github", duration);
```

Metrics are exposed via `/metrics` endpoint in Prometheus text format.

## Observability Backends

### Jaeger (Local Development)

**Docker:**

```bash
docker run -d \
  --name jaeger \
  -p 4317:4317/tcp \
  -p 16686:16686/tcp \
  jaegertracing/all-in-one:latest
```

**Access UI:** http://localhost:16686

**Query Examples:**
- Service: "focus-sync"
- Operation: "connector.sync"
- Tags: connector_id=github

### Grafana Loki + Promtail (Logs)

**docker-compose.yml:**

```yaml
services:
  loki:
    image: grafana/loki:latest
    ports:
      - "3100:3100"
    volumes:
      - ./loki-config.yml:/etc/loki/local-config.yml

  promtail:
    image: grafana/promtail:latest
    volumes:
      - ./promtail-config.yml:/etc/promtail/config.yml
      - /var/log:/var/log

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
```

Configure FocalPoint to write logs to a file, then Promtail ships them to Loki.

### Honeycomb (Cloud)

**Setup:**

```bash
export HONEYCOMB_API_KEY="your-api-key"
export HONEYCOMB_DATASET="focalpoint"
```

**Code:**

```rust
init_otel(Some("https://api.honeycomb.io")).await?;
```

## Common Queries

### Find All Slow Connector Syncs (>5 seconds)

**Jaeger (UI):**

1. Service: `focus-sync`
2. Operation: `connector.sync`
3. Tag filter: `duration_ms > 5000`

**Prometheus (if collecting latency histograms):**

```promql
histogram_quantile(0.95, connector_sync_duration_seconds)
```

### Audit Append Errors (Last Hour)

**Loki:**

```json
{
  "job": "focus-audit",
  "level": "error",
  "span_name": "audit.append"
}
```

### Rule Evaluation Match Rate

**Prometheus:**

```promql
sum(rate(rule_evaluations_total[5m])) by (rule_id)
```

### Wallet Mutations by Reason

**Loki:**

```json
{
  "span_name": "wallet.mutate",
  "reason": "daily_streak"
}
```

## Span Conventions

All FocalPoint spans follow a structured format with typed attributes.

### connector.sync

Emitted when a connector's sync orchestrator runs.

**Attributes:**
- `connector_id` (string) — e.g., "github", "strava"
- `state` (string, optional) — "syncing", "synced", "failed"
- `duration_ms` (u64, optional) — Time in milliseconds
- `error` (string, optional) — Error message if sync failed

**Example:**

```rust
use focus_observability::ConnectorSpanAttrs;
use tracing::Instrument;

let attrs = ConnectorSpanAttrs::new("github".to_string())
    .with_state("syncing".to_string())
    .with_duration(1234);

async {
    // ... sync logic
}.instrument(tracing::info_span!("connector.sync", 
    connector_id = "github",
    state = "syncing"
)).await
```

### rule.evaluate

Emitted per rule evaluation in the rules engine.

**Attributes:**
- `rule_id` (string) — e.g., "rule-123"
- `rule_type` (string, optional) — e.g., "time_window", "budget"
- `matched` (bool, optional) — Did the rule match?
- `duration_ms` (u64, optional)
- `error` (string, optional)

### audit.append

Emitted when audit log entries are appended.

**Attributes:**
- `audit_type` (string) — e.g., "reward_grant", "penalty_apply"
- `entry_count` (usize, optional)
- `duration_ms` (u64, optional)
- `error` (string, optional)

### wallet.mutate

Emitted when wallet state changes (reward/penalty).

**Attributes:**
- `wallet_id` (string)
- `delta` (i64) — Points added/removed
- `reason` (string, optional) — e.g., "daily_streak", "violation_penalty"
- `error` (string, optional)

## Privacy & PII Filtering

The `SpanPrivacyFilter` automatically redacts:

- Email addresses: `user@example.com` → `[REDACTED_EMAIL]`
- Phone numbers: `(555) 555-0123` → `[REDACTED_PHONE]`
- API tokens: `Bearer sk_live_...` → `[REDACTED_TOKEN]`
- URL credentials: `https://user:pass@host` → `https://[REDACTED_CREDS]@host`

**Never log:**
- User emails/IDs
- Connector API keys
- Rule/task definitions (UUIDs only)
- Personal data (tasks, durations, locations)

## Metrics Export

The `MetricsRegistry` exposes metrics in Prometheus text format.

**Available Counters:**
- `connector_syncs_total{connector_id="..."}` — Total syncs per connector
- `rule_evaluations_total{rule_id="..."}` — Total evaluations per rule
- `audit_appends_total{audit_type="..."}` — Total appends per type

**Available Histograms:**
- `connector_sync_duration_seconds{connector_id="..."}`
- `rule_eval_duration_seconds{rule_id="..."}`

**Scrape (Prometheus):**

```yaml
scrape_configs:
  - job_name: "focalpoint"
    static_configs:
      - targets: ["localhost:8080"]
    metrics_path: "/metrics"
```

## Troubleshooting

### Traces not appearing in Jaeger

1. Verify OTEL endpoint is reachable: `curl http://localhost:4317`
2. Check `init_otel()` was called and returned `Ok(())`
3. Ensure service spans are being created (check local logs first)
4. Verify Jaeger is configured to accept OTLP/gRPC on port 4317

### High log volume (production)

1. Set `FOCALPOINT_LOG_LEVEL=info` (default is `info`)
2. Use `RUST_LOG` to silence noisy dependencies:
   ```bash
   RUST_LOG=focus_sync=info,tokio=warn,hyper=warn
   ```
3. Consider sampling spans at the OTEL collector level

### Metrics not updating

1. Ensure `MetricsRegistry::global()` is called (singleton)
2. Verify metrics are incremented in the right code path
3. Check `/metrics` endpoint returns data

## References

- [tracing](https://docs.rs/tracing/)
- [tracing-subscriber](https://docs.rs/tracing-subscriber/)
- [OpenTelemetry Jaeger Exporter](https://docs.rs/opentelemetry-jaeger/)
- [Prometheus Client](https://docs.rs/prometheus/)
