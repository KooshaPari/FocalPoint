# RFC 0001: Plugin SDK for Connectors and Templates

**Author(s):** <maintainer-name> (@<handle>)
**Date:** 2026-04-23
**Status:** Accepted (Phase-1 Shipped: 2026-04-24)
**Related Issue(s):** FocalPoint/issues (plugin ecosystem)
**Target Release:** v0.2.0 (Phase 2)

## Summary

FocalPoint currently requires monorepo access and Rust expertise to publish new connectors and rule templates. This RFC proposes a **Plugin SDK** that enables third-party developers to build, sign, and distribute connectors and templates without monorepo access. The approach uses:

1. **WASM modules** (via Extism PDK + Wasmtime) for safe, sandboxed plugin execution.
2. **TOML manifests** (similar to npm `package.json`) for plugin metadata, entrypoints, and configuration.
3. **Plugin registry** (GitHub-based, with cryptographic signing) for distribution.
4. **Capability-based access** (via WASI) for I/O and network operations.

The design respects iOS App Store policy (plugins compile into the binary; no runtime JIT) while enabling a full plugin marketplace on macOS and web.

## Motivation

1. **Ecosystem Growth:** Currently, each new connector requires a PR to FocalPoint's monorepo. This is a high friction for third-party developers.
2. **Distribution Barrier:** End users cannot easily install or share connectors without forking or using a custom build.
3. **Missed Opportunities:** Popular integrations (Slack, Notion, Stripe) are blocked on monorepo merge timelines.
4. **Community Governance:** A plugin system establishes clear interface boundaries and reduces coupling between FocalPoint core and integrations.

## Design

### 1. Plugin SDK Structure

Each plugin is a standalone crate or package:

```
my-connector-slack/
├── Cargo.toml (or package.json for TS connectors)
├── src/
│   ├── lib.rs (Rust entry point)
│   └── manifest.toml (plugin metadata)
├── examples/
│   └── config.json (OAuth scopes, required fields)
├── tests/
└── README.md
```

### 2. Manifest Schema

A `manifest.toml` defines the plugin's identity, capabilities, and interface:

```toml
[plugin]
name = "connector-slack"
version = "0.1.0"
authors = ["Alice <alice@example.com>"]
license = "MIT OR Apache-2.0"
description = "Slack workspace event connector"

# Declares this is a connector; alternatives: "template", "rule-pack"
type = "connector"

# FocalPoint API version this plugin targets
api_version = "0.1.0"

# GitHub signing key (public) for verifying plugin releases
signing_key = "github.com/alice/connector-slack/releases"

# Required capabilities (WASI subset)
[capabilities]
# This connector needs to make HTTPS calls to Slack API
http = true
# This connector needs to access its own config storage
filesystem = { scope = "plugin-local" }

# Connector-specific interface
[interface]
connector = {
  scope = "workspace",  # "workspace" or "account"
  auth = "oauth2",       # oauth2, api_key, jwt, custom
  events = [
    { name = "message_posted", fields = ["user", "text", "timestamp"] },
    { name = "file_shared", fields = ["user", "file_url", "timestamp"] }
  ]
}

# Example OAuth configuration
[config.oauth]
client_id = "env:SLACK_CLIENT_ID"
client_secret = "env:SLACK_CLIENT_SECRET"
authorize_url = "https://slack.com/oauth/v2/authorize"
token_url = "https://slack.com/api/oauth.v2.access"
scopes = ["channels:history", "users:read"]
```

### 3. Connector Interface (WIT/WASM)

The plugin implements a standard WASM Interface Type (WIT):

```wit
// connector.wit
interface connector {
  type event = record {
    id: string,
    kind: string,
    timestamp: u64,
    data: string, // JSON-encoded event payload
  }

  type config = record {
    client-id: string,
    client-secret: string,
    workspace-id: option<string>,
  }

  // Initialize the connector with config
  init: func(config: config) -> result<string, string>

  // Fetch events since last checkpoint
  fetch-events: func(since: u64) -> result<list<event>, string>

  // Get the next checkpoint (for resumption)
  get-checkpoint: func() -> u64

  // OAuth flow (if applicable)
  get-auth-url: func() -> string
  exchange-token: func(code: string) -> result<config, string>
}
```

### 4. Plugin Registry & Signing

Plugins are signed with ED25519 keys and published to a registry:

```
registry.focalpoint.app/
├── connectors/
│   ├── slack/
│   │   ├── 0.1.0.wasm (signed)
│   │   └── 0.1.0.sig
│   └── ...
├── templates/
└── rule-packs/
```

**Signing Process:**
```bash
# Generate key (one-time)
focalpoint plugin keygen --name connector-slack

# Build and sign plugin
focalpoint plugin build --release
focalpoint plugin sign --key ./slack.key --output my-connector-slack-0.1.0.wasm

# Publish to registry
focalpoint plugin publish --registry registry.focalpoint.app ./my-connector-slack-0.1.0.wasm
```

### 5. Installation & Verification

End users install plugins via the CLI or dashboard:

```bash
focalpoint plugin install https://registry.focalpoint.app/connectors/slack/0.1.0
```

The client:
1. Fetches the `.wasm` and `.sig` files.
2. Verifies the signature against a pinned public key (from the manifest or a keyserver).
3. Adds the plugin to the local plugin registry.
4. On app start, loads all installed plugins into Wasmtime.

### 6. Backward Compatibility

- Existing built-in connectors (Canvas, GitHub, Google Calendar) remain in the core.
- The `Connector` trait is unchanged; plugins implement it via WASM bindings.
- Rule DSL and event schema are versioned; `api_version` in the manifest enforces compatibility checks.

## Drawbacks

1. **iOS Limitation:** App Store policy forbids runtime JIT. iOS users can only use plugins bundled at build time. This requires separate distribution channels (sideload, TestFlight, or macOS-only).
2. **WASM Overhead:** Plugin execution has modest overhead (~10–20% vs native). For most connectors, this is acceptable; high-frequency polling might suffer.
3. **Verification Burden:** End users must verify plugin signatures. UI/UX design is critical to avoid false confidence or signature confusion.
4. **Ecosystem Fragmentation:** Without curated vetting, low-quality or malicious plugins could emerge. We will establish a "verified plugins" category.

## Alternatives Considered

### Alt 1: Pure Rust plugin system (cargo plugin)

Write plugins as Rust crates, compile dynamically:
- **Pros:** No WASM overhead, native performance.
- **Cons:** Requires Rust compiler on end-user's system; unsafe; breaks iOS entirely. Not viable.

### Alt 2: TypeScript/JavaScript + Node.js Runtime

Embed Node.js; write plugins in TypeScript:
- **Pros:** Lower friction for web developers; familiar ecosystem (npm).
- **Cons:** Node.js runtime is heavy (100+ MB); breaks iOS/mobile; licensing complexity (GPL-adjacent dependencies).

### Alt 3: Fully Interpreted DSL (no compilation)

Create a FocalPoint-specific plugin DSL (similar to Lua/Starlark):
- **Pros:** Simple security model; no compilation step.
- **Cons:** Limited expressiveness for complex connectors; slower than compiled code; less portable.

**Chosen:** WASM + Extism because it balances safety, performance, and portability. WIT provides a clear interface contract.

## Unresolved Questions

1. **iOS distribution:** Should we offer sideload support (via TestFlight) for early adopters? Or macOS-only initially?
2. **Plugin marketplace moderation:** Who vets "official" plugins? GitHub-based curation (like VS Code Marketplace) or a formal review board?
3. **Billing/monetization:** Should plugin authors be able to charge for connectors? (Out of scope for v0.2; can be deferred.)
4. **Update propagation:** When a plugin has a security fix, how do we notify users? Automatic updates or explicit opt-in?

## Decision

**Status:** Pending RFC discussion (14-day window: 2026-04-23 to 2026-05-07).

Maintainers and community are invited to weigh in on the [related GitHub discussion](#).

---

## References

- **Research:** `/docs/research/plugin_sdk_architecture_2026_04.md` (comprehensive survey of WASM runtimes, extension systems, and iOS constraints).
- **Extism:** https://extism.org/
- **Wasmtime:** https://docs.wasmtime.dev/
- **WASI:** https://wasi.dev/
- **Zed Extensions (industry precedent):** https://zed.dev/blog/zed-decoded-extensions
- **Raycast Plugins:** https://developers.raycast.com/
- **Apple App Store Policy (Rule 4.2.2):** https://developer.apple.com/app-store/review/guidelines/

---

## Implementation Progress

### Phase 1: WASM Sandbox (SHIPPED 2026-04-24)

✅ Crates delivered:
- `focus-plugin-sdk/` — Core runtime wrapping wasmtime:
  - `PluginRuntime` with capability caps: 10MB memory, 5s timeout
  - No network or filesystem (host-provided config via linear memory)
  - Ed25519 signature verification
  - NDJSON event serialization

- `crates/focus-plugin-sdk/examples/hello-connector/` — Reference plugin:
  - Minimal WASM binary (~50 LOC Rust)
  - Exports `poll(config_ptr, config_len) -> (ptr, len)` ABI
  - Returns hardcoded NDJSON event for validation

✅ Integration:
- `focus-webhook-server`: New route `POST /plugins/:id/poll`
  - Invokes WASM runtime
  - Serializes concurrent exec per plugin (1 running at a time)
  - Returns normalized NDJSON events

✅ Tests (6):
- Memory cap enforcement
- Timeout handling (5s wall-clock)
- Signature verification (Ed25519)
- Unsigned plugin rejection
- Hello-connector compilation to wasm32-unknown-unknown
- Concurrent exec serialization

✅ Manifest:
- TOML schema in `manifest.rs`: plugin metadata, capabilities (http_client, timer), interface
- Phase-1 supports: http=false, filesystem=none, timer=false (all capabilities disabled)

### Phase 2A: HTTP Capability & Slack Reference (SHIPPED 2026-04-24)

✅ HTTP capability: host-proxied HTTP client
- `HttpProxy` with 30 req/min per-plugin rate-limiting
- URL allowlist enforcement from `plugin.toml` `[capabilities.http.allowlist]`
- 5s timeout per request; 10MB response size cap
- Tests (6): allowed domain success, denied domain rejected, rate-limit hit (under/at/over), response size bounded, timeout enforced

✅ Slack reference connector: `examples/slack-reference/`
- Declares `http = { allowlist = ["api.slack.com", "hooks.slack.com"] }` in manifest
- Implements `poll(config_ptr, config_len) -> i64` ABI
- Emits `slack:message_posted` and `slack:file_shared` events
- ~80 LOC Rust → wasm32-unknown-unknown
- Tests (2): event serialization, config parsing with HTTP capability

### Phase 2B: Build Tools (planned, 4 weeks)

- [ ] `focalpoint plugin build` — Compile Rust/TS plugin to WASM
- [ ] `focalpoint plugin sign` — Ed25519 signing with keygen support
- [ ] `focalpoint template install --plugin-wasm=<path>` — Validator + registry loader
- [ ] Timer capability: high-resolution timeout support

### Phase 2C: Dashboard UI & Verification (planned, 4 weeks)

- [ ] Plugin install/uninstall UI
- [ ] Signature verification feedback
- [ ] Plugin health metrics dashboard
- [ ] Capability request approval flow

### Phase 2D: Marketplace & Distribution (planned)

- [ ] GitHub-based plugin registry
- [ ] Curated "verified" badge
- [ ] iOS: TestFlight sideload (if App Store permits)

---

**Phase-1 Shipped:** 2026-04-24
**Phase-2A Shipped:** 2026-04-24 (HTTP capability + Slack reference)
**Phase-2B Start:** 2026-05-01 (estimated, build tools)
**Phase-2B Timeline:** 4 weeks (plugin build, sign, registry loader)
**Phase-2C Timeline:** 4 weeks (dashboard UI, verification flow)
