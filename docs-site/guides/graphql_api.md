# GraphQL API Guide

The FocalPoint GraphQL API gateway exposes core read APIs (tasks, rules, audit trail, connectors, wallet snapshots, focus sessions) and mutations (mark task done, enable rule, trigger sync) via a typed GraphQL surface. WebSocket subscriptions deliver live audit feed to third-party integrations.

## Overview

- **Service**: `services/graphql-gateway/`
- **Endpoint**: `POST /graphql` (queries, mutations)
- **WebSocket**: `GET /graphql/ws` (subscriptions)
- **Health**: `GET /health`
- **Port**: 8473 (default: 127.0.0.1:8473)
- **Auth**: Bearer token (optional; set `FOCALPOINT_GRAPHQL_TOKEN` env var)
- **Introspection**: Enabled in dev, disabled in prod (set `FOCALPOINT_GRAPHQL_PROD=true`)

## Authentication

All requests support optional Bearer token authentication. If a token is configured, include it in the `Authorization` header:

```bash
curl -H "Authorization: Bearer your-secret-token" \
  http://localhost:8473/graphql \
  -d '{"query": "{ wallet { credits } }"}'
```

Set the token via environment variable:

```bash
export FOCALPOINT_GRAPHQL_TOKEN="secret123"
```

If no token is configured, all requests are allowed.

## Queries

### Tasks

Fetch tasks by optional status filter.

```graphql
query {
  tasks(status: ACTIVE) {
    id
    title
    status
    createdAt
    dueAt
    description
  }
}
```

**Parameters:**
- `status: TaskStatus` (optional) — filter by `ACTIVE`, `DONE`, or `ARCHIVED`

**Response:**
```json
{
  "data": {
    "tasks": [
      {
        "id": "task-1",
        "title": "Complete report",
        "status": "ACTIVE",
        "createdAt": "2026-04-24T10:00:00Z",
        "dueAt": "2026-04-25T17:00:00Z",
        "description": "Finish Q2 report"
      }
    ]
  }
}
```

### Rules

Fetch behavior rules by optional enabled filter.

```graphql
query {
  rules(enabled: true) {
    id
    name
    enabled
    rigidity
    createdAt
    updatedAt
  }
}
```

**Parameters:**
- `enabled: Boolean` (optional) — filter by enabled/disabled

**Response:**
```json
{
  "data": {
    "rules": [
      {
        "id": "rule-1",
        "name": "No social media before 9am",
        "enabled": true,
        "rigidity": "Hard",
        "createdAt": "2026-01-01T00:00:00Z",
        "updatedAt": "2026-04-24T10:00:00Z"
      }
    ]
  }
}
```

### Wallet

Fetch aggregate credit, penalty, and reward state.

```graphql
query {
  wallet {
    credits
    totalPenalties
    totalRewards
    snapshotAt
  }
}
```

**Response:**
```json
{
  "data": {
    "wallet": {
      "credits": 1000,
      "totalPenalties": 50,
      "totalRewards": 250,
      "snapshotAt": "2026-04-24T10:30:00Z"
    }
  }
}
```

### Audit

Fetch tamper-evident audit records with optional filters.

```graphql
query {
  audit(since: "2026-04-23T00:00:00Z", limit: 50) {
    id
    recordType
    subjectRef
    occurredAt
    prevHash
    hash
    payload
  }
}
```

**Parameters:**
- `since: DateTime` (optional) — return records after this timestamp
- `limit: Int` (optional) — max records to return (default: 100)

**Response:**
```json
{
  "data": {
    "audit": [
      {
        "id": "audit-1",
        "recordType": "task_completed",
        "subjectRef": "task-1",
        "occurredAt": "2026-04-24T09:15:00Z",
        "prevHash": "genesis",
        "hash": "abc123def456...",
        "payload": {
          "status": "done",
          "completed_at": "2026-04-24T09:15:00Z"
        }
      }
    ]
  }
}
```

### Connectors

Fetch all configured connector integrations (GitHub, Google Calendar, Canvas, etc).

```graphql
query {
  connectors {
    id
    connectorType
    isConnected
    lastSyncedAt
  }
}
```

**Response:**
```json
{
  "data": {
    "connectors": [
      {
        "id": "conn-github",
        "connectorType": "github",
        "isConnected": true,
        "lastSyncedAt": "2026-04-24T08:30:00Z"
      },
      {
        "id": "conn-gcal",
        "connectorType": "gcal",
        "isConnected": true,
        "lastSyncedAt": "2026-04-24T09:00:00Z"
      }
    ]
  }
}
```

### Focus Sessions

Fetch focus sessions (locked-in time on a task) since an optional timestamp.

```graphql
query {
  focusSessions(since: "2026-04-24T00:00:00Z") {
    id
    taskId
    startedAt
    endedAt
    durationSecs
  }
}
```

**Parameters:**
- `since: DateTime` (optional) — return sessions after this timestamp

**Response:**
```json
{
  "data": {
    "focusSessions": [
      {
        "id": "session-1",
        "taskId": "task-1",
        "startedAt": "2026-04-24T09:00:00Z",
        "endedAt": "2026-04-24T10:30:00Z",
        "durationSecs": 5400
      }
    ]
  }
}
```

## Mutations

### Mark Task Done

Mark a task as completed (requires authentication).

```graphql
mutation {
  markTaskDone(id: "task-1") {
    id
    title
    status
    updatedAt
  }
}
```

**Parameters:**
- `id: String` (required) — task ID

**Response:**
```json
{
  "data": {
    "markTaskDone": {
      "id": "task-1",
      "title": "Complete report",
      "status": "DONE",
      "updatedAt": "2026-04-24T10:35:00Z"
    }
  }
}
```

### Enable Rule

Enable a behavior rule (requires authentication).

```graphql
mutation {
  enableRule(id: "rule-1") {
    id
    name
    enabled
    updatedAt
  }
}
```

**Parameters:**
- `id: String` (required) — rule ID

**Response:**
```json
{
  "data": {
    "enableRule": {
      "id": "rule-1",
      "name": "No social media before 9am",
      "enabled": true,
      "updatedAt": "2026-04-24T10:36:00Z"
    }
  }
}
```

### Trigger Sync

Trigger a connector sync (e.g., pull latest GitHub issues, calendar events) (requires authentication).

```graphql
mutation {
  triggerSync(connectorId: "conn-github") {
    connectorId
    status
    error
  }
}
```

**Parameters:**
- `connectorId: String` (required) — connector ID

**Response:**
```json
{
  "data": {
    "triggerSync": {
      "connectorId": "conn-github",
      "status": "started",
      "error": null
    }
  }
}
```

## Subscriptions

### Audit Feed

Subscribe to live audit events via WebSocket.

```graphql
subscription {
  auditFeed {
    id
    recordType
    subjectRef
    occurredAt
    hash
    payload
  }
}
```

**WebSocket URL:** `ws://localhost:8473/graphql/ws`

**Example with wscat:**

```bash
wscat -c ws://localhost:8473/graphql/ws
> {"type": "start", "payload": {"query": "subscription { auditFeed { id recordType subjectRef occurredAt hash } }"}}
< {"type": "data", "payload": {...}}
```

## Examples

### cURL: Fetch Tasks

```bash
curl -X POST http://localhost:8473/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ tasks { id title status } }"}'
```

### cURL: Mark Task Done

```bash
curl -X POST http://localhost:8473/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer secret123" \
  -d '{"query": "mutation { markTaskDone(id: \"task-1\") { id status } }"}'
```

### Python: Fetch Wallet

```python
import requests
import json

response = requests.post(
    "http://localhost:8473/graphql",
    json={"query": "{ wallet { credits totalRewards } }"}
)
data = response.json()
print(data)
```

### JavaScript: Fetch Rules

```javascript
const response = await fetch("http://localhost:8473/graphql", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    query: "{ rules(enabled: true) { id name rigidity } }"
  })
});
const data = await response.json();
console.log(data);
```

## Introspection

Enable introspection queries to explore the schema:

```graphql
query {
  __schema {
    types {
      name
      description
    }
  }
}
```

Introspection is **enabled by default in development** (FOCALPOINT_GRAPHQL_PROD=false) and **disabled in production** (FOCALPOINT_GRAPHQL_PROD=true) to reduce attack surface.

## Rate Limiting

The API enforces a **100 requests per minute per client IP** rate limit. Clients are identified by the remote IP (or X-Forwarded-For header if behind a proxy).

## Federation & Composition

For advanced use cases (e.g., composing multiple GraphQL services into a single gateway), the schema can be exported and used with Apollo Federation or other composition tools:

```bash
# Export schema
curl http://localhost:8473/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ __schema { types { name } } }"}' \
  | jq '.data.__schema'
```

## Deployment

### Docker

```bash
docker build -t focalpoint-graphql-gateway services/graphql-gateway
docker run -p 8473:8473 \
  -e FOCALPOINT_GRAPHQL_TOKEN="secret123" \
  -e FOCALPOINT_GRAPHQL_PROD="true" \
  focalpoint-graphql-gateway
```

### Docker Compose

```bash
docker-compose -f services/graphql-gateway/docker-compose.yml up
```

### Fly.io

```bash
fly deploy -c services/graphql-gateway/fly.toml
```

## Development

Run the gateway locally:

```bash
cargo run -p focus-graphql-gateway -- --bind 127.0.0.1:8473
```

With a custom token:

```bash
FOCALPOINT_GRAPHQL_TOKEN="dev-token" \
cargo run -p focus-graphql-gateway -- --bind 127.0.0.1:8473
```

## Testing

Run the test suite:

```bash
cargo test -p focus-graphql-gateway
```

Tests cover:
- Query round-trip validation
- Mutation success and idempotency
- Subscription event delivery
- Authentication (valid/invalid tokens)
- Rate limiting per client
