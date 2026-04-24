# Plugin SDK Architecture Research: Comprehensive Survey & Adoption Plan

**Date:** 2026-04-23
**Document Version:** 1.0
**Scope:** FocalPoint plugin ecosystem architecture analysis, spanning WASM runtimes, extension frameworks, rule DSLs, connector integrations, and UI builders.

---

## Executive Summary

FocalPoint currently has isolated connector and template infrastructure (focus-connectors, focus-templates, ConnectorRegistry, WebhookRegistry) but lacks a distributable plugin SDK. This research evaluates five architectural dimensions to recommend a single integrated stack: **WASM-based execution model with TOML manifest, CEL rule language, MCP bridging for LLM tool-calling, and React Flow for visual rule builders**. The approach unblocks an MVP in which third parties can build, sign, and distribute connectors without monorepo access.

**Key finding:** iOS App Store policy forbids runtime WASM JIT; plugins must be compiled into the binary at build time. This constraint requires either App Store sideload or a separate macOS/web distribution channel. Within that boundary, the architecture is sound and implementable in 6 weeks.

---

## Category 1: WASM Plugin Runtimes for Mobile/Embedded

### 1.1 Extism: Extensibility-First Framework

**Overview:** Extism is a lightweight WASM framework designed specifically for building extensible applications with untrusted plugin code. It provides:

- **PDKs (Plug-in Development Kits)** for Rust, JavaScript/TypeScript, Python, C, C#, Go, Zig, and more, allowing plugin authors to compile to `.wasm` binaries with built-in host communication.
- **Host SDKs** for Rust, Go, Java, .NET, Python, and JavaScript, enabling host applications to load and invoke plugins.
- **XTP Bindgen** tool to generate PDK bindings and define custom plugin system interfaces.

**Verdict:** Extism is production-grade for server and edge scenarios. The TypeScript PDK + Rust host pairing is excellent. However, it lacks explicit iOS consideration and doesn't address App Store JIT restrictions.

*Source:* [Extism GitHub – Main Repository](https://github.com/extism/extism) (accessed 2026-04-23); [Extism PDK Documentation](https://extism.org/docs/concepts/pdk/) (accessed 2026-04-23)

### 1.2 Wasmtime, Wasmer, WasmEdge Comparison

**Performance Profile (January 2026):**

| Runtime | Memory Footprint (1GB edge device) | Cold Start | JIT Compilation | Best For |
|---------|-------------------------------------|-----------|-----------------|----------|
| **Wasm3** | 4 MB | <1 ms | Interpreter | Ultra-low latency, minimal footprint |
| **Wasmtime** | 15 MB | 3 ms | Yes (Cranelift) | Security, WASI compliance, cross-platform |
| **Wasmer** | 12 MB | 2 ms | Yes (LLVM/Cranelift) | Portability, plugin system, compiler flexibility |
| **WasmEdge** | 8 MB | 1.5 ms | Yes + AOT | Edge/IoT, TensorFlow integration, async networking |

**Detailed Evaluation:**

- **Wasmtime** [(Wasmtime Security)](https://docs.wasmtime.dev/security.html), maintained by the Bytecode Alliance, is the industry standard for secure, standards-compliant WASM execution. It implements WASI APIs with capability-based filesystem access.
- **Wasmer** [(Wasmer GitHub)](https://wasmer.io/wasmer-vs-wasmtime) offers a plugin system and multiple compiler backends (LLVM, Cranelift, Singlepass), enabling higher customization for embedded deployments.
- **WasmEdge** [(WasmEdge Performance)](https://withbighair.com/webassembly/2025/05/11/Runtime-choices.html) is optimized for edge compute and IoT, with built-in TensorFlow integration and async I/O for cloud-native scenarios.

**Recommendation:** Wasmtime for security and standards compliance; WasmEdge if edge/IoT distribution is a priority.

*Sources:* [WebAssembly Runtime Benchmarks 2026](https://wasmruntime.com/en/benchmarks) (accessed 2026-04-23); [Comparing WebAssembly Runtimes](https://reintech.io/blog/wasmtime-vs-wasmer-vs-wasmedge-wasm-runtime-comparison-2026) (accessed 2026-04-23)

### 1.3 Security Sandboxing: wasm-bindgen, WASI Threads, WASI-HTTP

**WASI Capability Model:** WASI (WebAssembly System Interface) implements capability-based security: plugins can only access files/network resources explicitly granted by the host. No ambient authority.

- **wasi-threads** [(Bytecode Alliance Article)](https://bytecodealliance.org/articles/wasi-threads) enables pthreads-style multi-threading in WASM modules without breaking sandboxing. Pre-release as of 2026.
- **wasi-http** and networking APIs are part of WASI Preview 2, allowing controlled outbound HTTP calls without exposing raw sockets.
- **wasm-bindgen** [(wasm-bindgen Releases)](https://github.com/wasm-bindgen/wasm-pack/releases) is the Rust-to-WASM glue layer, enabling typed function boundaries and host callback definitions.

**Verdict for FocalPoint:** WASI capabilities are sufficient for a connector SDK: plugins can read their assigned data bucket, make HTTP calls to specified endpoints, and write audit logs, all without privilege escalation. Thread support is nice-to-have; the capability model is the hard requirement.

*Sources:* [Node.js WASI Documentation](https://nodejs.org/api/wasi.html) (accessed 2026-04-23); [WASI and the WebAssembly Component Model](https://eunomia.dev/blog/2025/02/16/wasi-and-the-webassembly-component-model-current-status/) (accessed 2026-04-23)

### 1.4 iOS App Store Policy: The Critical Blocker

**Finding:** Apple's App Store policy (Rule 4.2.2 and the "Executable Code" clause) **forbids runtime JIT interpretation and dynamic code generation** on iOS. This means:

- A native iOS app **cannot** dynamically load and execute WASM bytecode at runtime using a WASM JIT (Wasmtime, Wasmer, WasmEdge).
- WASM modules **must** be pre-compiled and linked into the binary at build time, or executed only on macOS/web.
- Any runtime plugin loading is considered a violation of App Store guidelines.

**Evidence:** Historical enforcement on a-Shell (Hacker News discussions) and explicit rule wording. iOS 17+ offers no JITerpreter exception for third-party apps.

**Implications for FocalPoint:**
1. **iOS distribution:** Plugins must be compiled into the release binary. No runtime plugin marketplace on iOS.
2. **Sideload/Web:** Desktop (macOS) and web deployments can use full runtime plugin loading.
3. **macOS App Store:** macOS apps have looser rules; sideloaded WASM plugin loading is viable on macOS.

*Sources:* [Apple App Review Guidelines](https://developer.apple.com/app-store/review/guidelines/) (accessed 2026-04-23); [Apple requests removal of WebAssembly (Hacker News)](https://news.ycombinator.com/item?id=25032008) (accessed 2026-04-23)

---

## Category 2: Extension Architectures in Shipping Apps

### 2.1 Raycast: TypeScript + Manifest-Driven

**Architecture:** Raycast extensions are built in TypeScript/React with a single `package.json` manifest defining commands, preferences, and tools.

- **Manifest Structure:** Superset of npm's `package.json`, declaring commands as top-level objects with properties like `mode`, `title`, `description`, and entry point.
- **Runtime:** Node.js on Raycast's servers; TypeScript compiled to JavaScript at deploy time.
- **API Surface:** Fully typed SDK exposing UI components (List, Detail, Form), file I/O, HTTP, and preference management.
- **Distribution:** Raycast plugin marketplace (GitHub repo submission) or private distribution.

**Verdict:** Excellent for desktop/CLI scenarios, but not applicable to mobile. The manifest-first approach is worth adopting.

*Source:* [Raycast Manifest API Documentation](https://developers.raycast.com/information/manifest) (accessed 2026-04-23)

### 2.2 Obsidian: TypeScript API + Community Vetting

**Architecture:** Obsidian plugins are standalone TypeScript packages with full access to a rich API for file operations, UI rendering, and metadata.

- **Plugin Type Definitions:** Central `obsidian-api` GitHub repo provides comprehensive TSDoc-annotated types.
- **Submission:** Pull request to `obsidian-releases` repo to list plugin in community marketplace.
- **Vetting:** Community review; no automated security scanning, but transparent code visibility.
- **Distribution:** GitHub-hosted repos with install via Obsidian's plugin browser.

**Verdict:** Simple, transparent, and low-friction. The GitHub-centric distribution model is lightweight. Applicable to desktop but requires code review overhead.

*Sources:* [Obsidian Plugin Documentation](https://docs.obsidian.md/Reference/TypeScript+API/Plugin) (accessed 2026-04-23); [Obsidian Sample Plugin](https://github.com/obsidianmd/obsidian-sample-plugin) (accessed 2026-04-23)

### 2.3 VS Code: Activation Events + Contribution Points

**Architecture:** VS Code extensions declare static metadata (contribution points) and runtime hooks (activation events) in `package.json`.

- **Contribution Points:** Statically registered features (commands, views, language grammars, keybindings) that extend VS Code before the extension code runs.
- **Activation Events:** Declarative triggers (e.g., `onLanguage:python`, `onCommand:my.command`) that cause the extension to load.
- **Type Safety:** Fully typed Node.js API; extensions are essentially Node.js modules.
- **Distribution:** VS Code Marketplace with automatic vetting (security scanning, signing).
- **Publisher Identity:** `<publisher>.<name>` scheme ensures uniqueness and tied to a publisher account.

**Verdict:** Mature, standards-based ecosystem. Contribution points + activation events pattern is excellent for reducing startup cost. Best-in-class marketplace vetting.

*Sources:* [VS Code Contribution Points API](https://code.visualstudio.com/api/references/contribution-points) (accessed 2026-04-23); [VS Code Activation Events](https://code.visualstudio.com/api/references/activation-events) (accessed 2026-04-23)

### 2.4 Zed: WASM + WIT Interface (Industry Pioneer, 2024)

**Architecture:** Zed extensions are written in **Rust, compiled to WebAssembly, and communicate with the host via WebAssembly Interface Types (WIT)**.

- **Compilation Pipeline:** ExtensionBuilder cross-compiles Rust to `wasm32-wasip2` target; includes Tree-sitter grammar compilation via wasi-sdk.
- **API Definition:** WIT files define the extension-host interface; `wit_bindgen` generates typed Rust bindings.
- **Distribution:** GitHub-based (submodule in `zed-industries/extensions`); each extension is a Rust crate.
- **Cross-Editor Standard (2025):** Zed is collaborating on an open IDE Extension Standard for portable WASM-based extensions across editors.

**Verdict:** This is the most advanced shipping extension system. WASM + WIT is the future. The approach is proven and aligns with industry standardization efforts.

*Sources:* [Zed Extensions Architecture Blog](https://zed.dev/blog/zed-decoded-extensions) (accessed 2026-04-23); [Zed Cross-Editor Standard Discussion](https://github.com/zed-industries/zed/discussions/48464) (accessed 2026-04-23)

### 2.5 Tauri: Rust Host + IPC Bridge

**Architecture:** Tauri apps pair a Rust backend with a WebView frontend. Plugins are Rust-based commands that communicate across an IPC boundary.

- **Command System:** A Rust function is invoked in response to an IPC request from the frontend (JavaScript/WebView).
- **IPC Protocol:** Custom protocol (ipc://) or postMessage fallback; async, typed, namespace-isolated.
- **Plugin System:** Modular Rust crates exposing pre-built functionality (SQL, HTTP, shortcuts, etc.).
- **Security Model:** Clear boundary between frontend (untrusted) and Rust backend (trusted); IPC is the enforcement point.

**Verdict:** Excellent for desktop apps; not applicable to mobile due to iOS restrictions. The IPC pattern is relevant for FocalPoint's webhook delivery model.

*Source:* [Tauri Inter-Process Communication](https://v2.tauri.app/concept/inter-process-communication/) (accessed 2026-04-23)

### 2.6 Apple Shortcuts / App Intents (2024 Update)

**Architecture (2024 Evolution):** App Intents (introduced iOS 16, evolved 2024) allow Swift apps to expose actions/queries to Shortcuts, Spotlight, widgets, Control Center, and Apple Intelligence.

- **Shift from Intent Definition Files:** 2024 moved entirely to Swift-based intent definitions (via `@AppIntent` macro). No separate IntentFile or code generation.
- **Integration Points:** Shortcuts.app, Siri, Spotlight, widgets, Control Center, Apple Intelligence (new in 2024).
- **Type Safety:** Full Swift typing; Xcode 16+ auto-generates title strings.
- **Limitation:** App-internal only; no third-party plugin SDK.

**Verdict:** Not applicable for third-party plugin distribution. Important for FocalPoint to expose actions to Shortcuts on iOS (e.g., trigger a focus session), but doesn't solve the plugin SDK problem.

*Sources:* [App Intents 2024 Updates](https://matthewcassinelli.com/whats-new-in-app-intents-in-2024/) (accessed 2026-04-23); [Apple App Intents Documentation](https://developer.apple.com/documentation/appintents) (accessed 2026-04-23)

---

## Category 3: Rule Languages & DSLs

### 3.1 CEL (Common Expression Language) — Google Standard

**Overview:** CEL is an open-source, non-Turing-complete expression language designed for rapid, safe policy evaluation.

- **Performance:** Nanoseconds-to-microseconds evaluation; used in Kubernetes API policies and Google Cloud services.
- **Four-Valued Logic:** Supports partial evaluation over incomplete data, returning definitive or "need more info" results.
- **Non-Turing-Complete:** Guarantees termination; no infinite loops or unbounded recursion.
- **Implementations:** Go, Java, Python, C++, and JavaScript/TypeScript via transpilers.
- **Use Cases:** Kubernetes CEL validation rules, Cloud identity policies, API gateways.

**Example Rule (CEL):**
```
request.method == "GET" && request.path.startsWith("/public/")
```

**Verdict for FocalPoint:** CEL is ideal for simple, high-performance trigger/action rules. It's battle-tested at scale and language-agnostic. The four-valued logic is valuable for partial data scenarios.

*Source:* [CEL: Common Expression Language (cel.dev)](https://cel.dev/) (accessed 2026-04-23); [Google OSS Blog: CEL for Portable Policy](https://opensource.googleblog.com/2024/06/common-expressions-for-portable-policy.html) (accessed 2026-04-23)

### 3.2 Open Policy Agent / Rego — Declarative Policy

**Overview:** OPA is a general-purpose policy engine. Rego is its declarative query language (Datalog-inspired, extended for JSON/documents).

- **Language:** Rego is unification-based, bottom-up evaluation. Rules are facts and derived conclusions.
- **Use Cases:** Kubernetes admission control, API authorization, infrastructure-as-code validation, supply-chain policy.
- **Modularity:** OPA bundles enable packaging policies with data; high-level abstractions (role-based access control, attribute-based access control).

**Example Rule (Rego):**
```
allow {
  input.method == "GET"
  input.path[0] == "public"
}
```

**Verdict for FocalPoint:** OPA/Rego is more powerful than CEL but heavier-weight. Excellent for complex, multi-fact policies (e.g., "user is in group X and has permission Y"). Rego's unification makes data-driven rules simpler. Trade-off: higher cognitive load for simpler rules.

*Source:* [Open Policy Agent Documentation](https://www.openpolicyagent.org/docs/policy-language) (accessed 2026-04-23)

### 3.3 Datalog Variants: Cozo, Differential Datalog

**Overview:** Pure Datalog is a bottom-up logic programming language; modern variants add incrementality and graph/vector capabilities.

- **Cozo:** Transactional relational-graph-vector database with Datalog query language; HNSW vector search integrated into Datalog.
- **Differential Datalog (DDlog):** Language for incremental computation; ideal for continuous rule updates in response to input changes.

**Verdict for FocalPoint:** Datalog is overkill for simple trigger/action rules. Use Cozo if the rule engine needs to perform graph traversals (e.g., "find all users in the same team and notify them"). Otherwise, CEL is simpler.

*Sources:* [Cozo GitHub](https://github.com/cozodb/cozo) (accessed 2026-04-23); [Differential Datalog GitHub](https://github.com/vmware-archive/differential-datalog) (accessed 2026-04-23)

### 3.4 Adoption Recommendation: CEL + TOML Sidecar for FocalPoint

**Rationale:**
- CEL's performance and simplicity suit FocalPoint's focus-session triggers.
- TOML for static rule definitions (manifest-like); CEL for dynamic conditions.
- Easy for third-party rule authors to learn and audit.
- Can be embedded in a single `rules.cel` file within a connector package.

**Example FocalPoint Rule (TOML + CEL):**
```toml
[rule.on_calendar_block]
description = "Trigger focus when calendar shows a block"
condition = '''
event.duration_minutes >= 30 && 
event.title.matches(".*focus.*|.*deep.*work.*")
'''
actions = ["notify", "start_session"]
```

---

## Category 4: Connector / Integration Frameworks

### 4.1 Apache Camel: Enterprise Integration Patterns (EIP) DSL

**Overview:** Camel is a mature framework implementing 50+ Enterprise Integration Patterns (from the Hohpe-Woolf book).

- **Route DSL:** Java fluent API or XML/YAML configuration defining message flows (filter, transform, route, enrich).
- **Component Library:** 300+ connectors (HTTP, FTP, Kafka, databases, cloud services).
- **Example:** `from("file:inbox").filter().simple("${body.length} > 100").to("file:outbox")`

**Verdict for FocalPoint:** Camel is over-engineered for connector distribution. It's suitable for enterprise monoliths but not for lightweight third-party SDKs. Skip.

*Source:* [Apache Camel Route DSL](https://camel.apache.org/manual/dsl.html) (accessed 2026-04-23)

### 4.2 n8n Nodes: JSON Schema UI + Node.js Execution

**Architecture:** n8n is a workflow automation platform where integrations are **n8n nodes** (Node.js modules exporting a JSON schema UI and execute function).

- **Node Structure:** Each node is a TypeScript/JavaScript module with metadata (name, description, properties, credentials), a JSON schema for input validation, and an execute async function.
- **Schema-Driven UI:** n8n generates forms from JSON schema; no custom UI code needed.
- **Distribution:** Nodes are published to npm or hosted in the community; n8n loads them at runtime.
- **Example:** GitHub node exposes actions (create issue, list repos) with parameterized inputs/outputs.

**Verdict for FocalPoint:** n8n's node pattern is excellent for workflow-based integrations. Applicable for connectors that transform/aggregate data. Less suitable for real-time triggers.

*Source:* [n8n Workflow Export/Import Documentation](https://docs.n8n.io/workflows/export-import/) (accessed 2026-04-23)

### 4.3 Airbyte Connectors: Python/Java + JSON Schema

**Architecture:** Airbyte is a data integration platform. Connectors are Python or Java programs exporting:

- **Spec:** JSON schema defining configuration (database credentials, API keys, filters).
- **Discover:** JSON schema describing available data streams.
- **Read:** Async streaming of records in JSON-Lines format.
- **State Management:** Watermarks/cursors for incremental syncs.

**Verdict for FocalPoint:** Airbyte's design is optimized for data pipelines (batch extraction, incremental sync). Not suitable for event-driven connectors or real-time triggers. Skip.

*Source:* [Airbyte Schema Reference](https://docs.airbyte.com/platform/connector-development/schema-reference) (accessed 2026-04-23)

### 4.4 Pipedream Components: Node.js + npm Ecosystem

**Architecture:** Pipedream allows developers to build **sources** (triggers) and **actions** (responses) as Node.js modules.

- **Component Type:** TypeScript files exporting `defineSource()` or `defineAction()` with props schema, async execute function, and timer config.
- **Runtime:** Serverless Node.js on Pipedream; full npm access; async/await native.
- **Distribution:** Components published to Pipedream's registry; CLI-based deployment.
- **Ecosystem:** 3,000+ apps, 10,000+ actions available.

**Verdict for FocalPoint:** Pipedream's pattern is lightweight and production-tested. Source (trigger) and action (webhook receiver) separation is clean. Export to this pattern is valuable.

*Source:* [Pipedream Components Overview](https://pipedream.com/docs/components) (accessed 2026-04-23)

### 4.5 Model Context Protocol (MCP): LLM Tool Integration Standard

**Overview:** MCP (introduced by Anthropic, November 2024) is an open protocol enabling LLM-safe tool and resource discovery.

- **Tool Calling:** Tools expose input schema (JSON schema), execute handlers, and return results. LLMs can call tools dynamically.
- **Servers:** MCP servers expose tools and resources; clients (LLM apps) call them via a standardized protocol.
- **Rapid Adoption:** 16,000+ MCP servers published as of April 2026.
- **Use Case:** "Tell Claude to send a Slack message" — Claude sees the tool schema, calls it with params, gets the response.

**Verdict for FocalPoint:** **MCP is essential for LLM integration.** If FocalPoint plans to expose connector actions to Claude (e.g., "use this focus connector to trigger a session"), implement an MCP server wrapping the connector interface. This unblocks AI-driven focus coaching.

*Sources:* [Model Context Protocol Specification](https://modelcontextprotocol.io/specification/2025-11-25) (accessed 2026-04-23); [Introducing MCP (Anthropic Blog)](https://www.anthropic.com/news/model-context-protocol) (accessed 2026-04-23)

---

## Category 5: Rule & Workflow Builder UI Libraries

### 5.1 ReactFlow (now XyFlow): Node Graph Editor

**Overview:** ReactFlow is the de facto standard for building node-based visual editors in React.

- **Features:** Drag-and-drop nodes, customizable edges, pan/zoom, minimap, multiple renders, selection, delete, copy/paste.
- **Customization:** Fully themable; node components are React; edge paths are SVG.
- **Performance:** Handles 1,000+ nodes efficiently.
- **Recent Rebranding:** ReactFlow is now part of XyFlow (maintained by Berlin-based team).

**Verdict for FocalPoint:** Use ReactFlow/XyFlow for visual focus-rule builders. Example: drag trigger + condition + action nodes to construct "if calendar block >= 30min, start focus session."

*Source:* [ReactFlow Documentation](https://reactflow.dev/) (accessed 2026-04-23); [XyFlow Node-Based UIs](https://xyflow.com/) (accessed 2026-04-23)

### 5.2 Blockly (Google): Block-Based Programming

**Overview:** Blockly is Google's visual block-based programming library (like Scratch). Blocks snap together; the library generates code.

- **Code Generation:** Blockly can emit Dart, Go, JavaScript, Lua, PHP, Python, or custom code.
- **Use Cases:** Educational tools, automation for non-programmers.
- **Accessibility:** Keyboard navigation, high-contrast themes.

**Verdict for FocalPoint:** Blockly is excellent for non-technical users (e.g., "block for HTTP request, block for send notification"). Trade-off: less expressive than text-based rules.

**Recommendation:** If FocalPoint targets non-technical focus rule authors, use Blockly + Blockly-to-CEL code generation. Otherwise, skip.

### 5.3 Rete.js: Modular Node Editor

**Overview:** Rete.js is a modular framework for building node-based editors with sockets, control flow, and data flow.

- **Architecture:** Sockets (input/output), nodes with I/O sockets, connections between sockets.
- **Engine:** Optional execution engine to evaluate the graph.
- **Plugins:** Extensions for visual effects, rendering, persistence.

**Verdict for FocalPoint:** Rete.js is more low-level than ReactFlow. If custom socket types or execution engine are needed, choose Rete; otherwise, ReactFlow is simpler.

### 5.4 iOS SwiftUI Picker / Native Components (2024)

**Overview:** SwiftUI Picker is Apple's standard for selecting options in iOS apps.

- **Built-in:** Part of SwiftUI; integrates with iOS design language.
- **Variants:** Menu-style picker, segmented picker, date/time picker.
- **Limitation:** Picker is for simple single-value selection, not complex visual rule building.

**Verdict for FocalPoint:** Use SwiftUI Picker for simple trigger selection (e.g., "When calendar block >= 30 minutes"). For complex rule building, use web-based (ReactFlow) UI.

*Source:* [SwiftUI Picker Documentation](https://codewithchris.com/swiftui-picker/) (accessed 2026-04-23)

---

## Category 6: Recommended Plugin SDK Architecture for FocalPoint

### 6.1 Overall Stack

**Execution Model:** WASM-based (Wasmtime host, Rust PDK for plugin authors)
**Manifest Format:** TOML (lightweight, human-readable, matches existing focus-templates design)
**Rule Language:** CEL (simple, performant, non-Turing-complete)
**Distribution:** Git-based (GitHub refs + SHA256 hash) with ed25519 signing
**Signing Key Policy:** Single root keypair (FocalPoint's ed25519-dalek); plugins rotated yearly
**Capability Model:** Per-plugin WASI sandbox with explicit resource grants (data bucket, HTTP endpoints, audit log)
**UI Builder:** ReactFlow for visual rule composition; exports to CEL
**LLM Integration:** MCP server wrapping connector interface for Claude/LLM tool calling
**iOS Compliance:** Plugins compiled into iOS binary at build time; runtime plugin loading only on macOS/web

### 6.2 Plugin Manifest Format (TOML)

**Rationale:** TOML is human-readable, matches FocalPoint's focus-templates precedent, and requires minimal parsing.

```toml
[plugin]
name = "slack-notifier-connector"
version = "0.1.0"
author = "Jane Smith <jane@example.com>"
description = "Send Slack notifications from focus sessions"
license = "Apache-2.0"
# Ed25519 public key for the author (verified at install time)
author_pubkey = "4d21c4e3f8a7b9e1c2d5f6a8b9e0c1d2"

# WASM binary hash (SHA256)
wasm_hash = "sha256:a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1"

# Capabilities requested by this plugin
[capabilities]
http_endpoints = ["https://hooks.slack.com/*"]
audit_log = true
data_bucket = "slack_credentials"

# Runtime configuration (user-provided)
[config]
slack_webhook_url = { type = "secret", required = true }
default_channel = { type = "string", required = false, default = "#focus-logs" }

# Trigger rules (CEL expressions)
[triggers.on_session_start]
description = "When a focus session begins"
condition = '''
session.duration_minutes >= 15 && 
user.preferences.slack_notifications == true
'''
action = "notify_slack"

[triggers.on_session_end]
description = "When a focus session ends"
condition = "session.completed == true"
action = "log_session_summary"
```

### 6.3 Execution Model: WASM Sandbox

**Host Runtime:** Wasmtime (Rust-based, WASI-compliant)
**Plugin Language:** Rust → WASM (via wasm-bindgen and Extism Rust PDK)
**Boundary Crossing:** Typed function calls via wit_bindgen / Extism host API
**Runtime Grants:** WASI capability tokens for file access, HTTP, logging
**Error Handling:** WASM trap (panic) is caught and logged; plugin is disabled; user receives notification

**Example Plugin Invocation (Rust pseudocode):**
```rust
// Host code
let plugin = wasmtime::load_module(&wasm_bytes)?;
let result = plugin.call_function("on_session_start", &input_json)?;
// Plugin runs in sandbox; can only call allowed HTTP endpoints
```

### 6.4 Capability and Permission Model

**Principle:** Plugins run with minimal ambient authority; resources are explicitly granted.

| Capability | Grant Type | Example |
|------------|-----------|---------|
| **HTTP Endpoint Access** | Allowlist of URL patterns | `https://hooks.slack.com/*` |
| **Audit Log Write** | Boolean flag (can plugin log events?) | `true` |
| **Data Bucket Read** | Bucket name from FocalPoint's vault | `slack_credentials`, `user_preferences` |
| **Timer / Scheduling** | Interval in seconds (if trigger is periodic) | `60` (once per minute) |
| **Filesystem Write** | Path pattern (for local logs/cache) | `/var/log/focalpint-plugins/` |

**Enforcement:** Wasmtime's WASI implements capability-based security; undefined capabilities result in sandbox violation (trap).

### 6.5 Distribution Model

**Registry Option:** None (initially); plugins distributed via Git refs.
**Distribution Steps:**
1. Plugin author creates GitHub repo with `manifest.toml` + `src/lib.rs` (Rust WASM) + CI build pipeline.
2. Author signs the `.wasm` binary with ed25519-dalek: `openssl dgst -sha256 -sign plugin_key.pem plugin.wasm > plugin.wasm.sig`
3. FocalPoint user adds the plugin to their config:
   ```toml
   [plugins.slack_notifier]
   git_url = "https://github.com/janesmith/focalpint-slack.git"
   git_ref = "v0.1.0"  # Tag or commit hash
   author_pubkey = "4d21c4e3f8a7b9e1c2d5f6a8b9e0c1d2"
   ```
4. FocalPoint CLI fetches the repo, verifies the signature, loads the WASM module.

**Signing Strategy:** FocalPoint publishes a "trusted plugin authors" list (optional); users can enable "only allow trusted plugins" in settings.

### 6.6 Signing Model & Root Keypair Policy

**Current Approach:** Each plugin author has their own ed25519 keypair; they sign their releases.

**Root Keypair (FocalPoint):** Maintains a master keypair only for:
1. Signing the "trusted authors" registry (optional).
2. Signing official connectors (Slack, Notion, etc.) bundled with FocalPoint.
3. Rotating keypairs annually; old key published for audit.

**Verification Flow:**
```
User Config: author_pubkey = "..."
Downloaded Manifest: author_pubkey = "..."
Downloaded .wasm.sig: [ed25519 signature]

Verify: ed25519_verify(plugin.wasm, plugin.wasm.sig, author_pubkey) == OK
```

### 6.7 iOS App Store Compliance Strategy

**Hard Constraint:** iOS App Store forbids runtime JIT and dynamic code loading.

**Compliance Path:**
1. **iOS Distribution:** Plugins are pre-compiled and statically linked into the FocalPoint iOS binary at build time.
   - Build pipeline: CI fetches plugin repos (with version tags), compiles WASM → native (via WASM2C or similar), links into binary.
   - Plugin updates on iOS require an App Store update.
   - This is acceptable for "official" connectors (Slack, Notion, Apple Calendar).

2. **macOS / Web Distribution:** Full runtime plugin loading allowed.
   - macOS app can download and execute WASM plugins at runtime.
   - Web app (if applicable) loads WASM in the browser (via Wasmtime.js).

3. **Alternative: Sideload / Testflight:** Distribute iOS builds via sideloading or TestFlight, which allows more flexibility but doesn't reach the App Store.

**Decision:** If Apple App Store presence is critical, accept iOS plugin limitations (pre-compiled only). If flexibility is paramount, release via sideload or macOS only.

**Recommendation:** Ship iOS with 3-5 official built-in connectors; market macOS app as the "extensible" version with runtime plugins.

---

## 7-Week Implementation Plan (AI-Driven, Ordered by Dependency)

### Phase 1: Foundation (Weeks 1-2)

**Goal:** Core WASM host, manifest parsing, signing infrastructure.

**Tasks:**
1. **WASM Host Integration (Wasmtime):**
   - Create `focal-wasm-host` crate (Rust).
   - Implement `PluginRuntime` struct wrapping Wasmtime instance.
   - Add `load_plugin(wasm_bytes: &[u8]) -> Result<Plugin>`.
   - Implement WASI capability filtering (HTTP allowlist, data bucket grants).
   - **Effort:** 5 parallel agent subtasks; ~3-4 tool calls each. **~1.5h wall clock.**

2. **Manifest Parser (TOML):**
   - Create `focal-plugin-manifest` crate.
   - Serde TOML deserialize to `PluginManifest` struct.
   - Validate manifest schema (required fields, capability types).
   - **Effort:** 2 agent subtasks; ~2-3 tool calls each. **~0.75h wall clock.**

3. **Ed25519 Signing Verification:**
   - Integrate `ed25519-dalek` (already in use).
   - Add `sign_wasm()` and `verify_wasm()` functions.
   - Create CLI tool: `focalpint plugin sign <wasm_file> <key_file>`.
   - **Effort:** 1 agent subtask; ~4-5 tool calls. **~0.5h wall clock.**

**Deliverables:**
- `focal-wasm-host`, `focal-plugin-manifest`, `focal-crypto` crates (Rust).
- End-to-end test: load unsigned WASM, invoke function, receive JSON result.

---

### Phase 2: Rule Engine & Triggers (Weeks 2-3)

**Goal:** CEL rule evaluation; trigger evaluation pipeline.

**Tasks:**
1. **CEL Rule Evaluation:**
   - Integrate `cel` Rust crate (or use Go via FFI, or Python via PyO3).
   - Create `focal-rules-engine` crate.
   - Implement `EvaluateRule { condition: String, context: serde_json::Value } -> Result<bool>`.
   - Cache compiled CEL programs for performance.
   - **Effort:** 3 agent subtasks (CEL library integration, caching, error handling); ~5-6 tool calls total. **~1.5h wall clock.**

2. **Manifest Rule Parsing:**
   - Extend `focal-plugin-manifest` to parse `[triggers]` section.
   - For each trigger, store (description, condition_cel, action_name).
   - **Effort:** 1 agent subtask; ~2-3 tool calls. **~0.5h wall clock.**

3. **Trigger Evaluation Pipeline:**
   - Create `TriggerEvaluator` that matches session events (start, end, pause) against plugin rules.
   - Invoke plugin's action function (e.g., `on_session_start`) if rule evaluates true.
   - **Effort:** 2 agent subtasks (evaluator, action invocation); ~4-5 tool calls. **~1h wall clock.**

**Deliverables:**
- `focal-rules-engine` crate.
- Plugin manifest with `[triggers]` section.
- Integration test: manifest with CEL rule, session event triggers action.

---

### Phase 3: Plugin Packaging & CLI (Week 3-4)

**Goal:** CLI tool for developers to create, test, build, sign plugins.

**Tasks:**
1. **Plugin Scaffolding (CLI):**
   - `focalpint plugin scaffold <name>` generates boilerplate Rust project.
   - Pre-configured `Cargo.toml` with Extism Rust PDK, wasm-bindgen.
   - Example plugin: "echo connector" (accepts message, returns echoed message).
   - **Effort:** 1 agent subtask; ~3-4 tool calls. **~0.75h wall clock.**

2. **Plugin Build Pipeline:**
   - `focalpint plugin build --release` compiles Rust → WASM.
   - Validates manifest schema.
   - Outputs: `plugin.wasm` + `manifest.toml`.
   - **Effort:** 1 agent subtask; ~2-3 tool calls. **~0.5h wall clock.**

3. **Plugin Local Testing:**
   - `focalpint plugin test <plugin_dir>` loads plugin, simulates trigger events, captures output.
   - Generates test report (pass/fail for each trigger).
   - **Effort:** 2 agent subtasks (test harness, output reporting); ~4-5 tool calls. **~1h wall clock.**

4. **Plugin Signing & Publishing:**
   - `focalpint plugin sign <wasm_file> --key <ed25519_key_file>` produces `.sig` file.
   - `focalpint plugin publish <github_url> --tag v0.1.0` creates GitHub release with `.wasm` + `manifest.toml` + `.sig`.
   - **Effort:** 2 agent subtasks (signing, GitHub integration); ~4-5 tool calls. **~1h wall clock.**

**Deliverables:**
- `focalpint` CLI plugin subcommands.
- Example plugin (echo connector).
- Documentation: "Build Your First Plugin" guide.

---

### Phase 4: Manifest-Based Discovery & Registry (Week 4-5)

**Goal:** Runtime plugin loading from manifest; dependency resolution.

**Tasks:**
1. **Plugin Discovery & Loading:**
   - FocalPoint config file: `[plugins]` section with git URLs + refs.
   - Runtime: fetch plugin manifest from git, verify signature, load WASM.
   - Handle version mismatch (e.g., plugin requires WASM host API v2, but host is v1).
   - **Effort:** 3 agent subtasks (git fetch, signature verify, version negotiation); ~5-6 tool calls. **~1.5h wall clock.**

2. **Plugin Registry (Optional):**
   - GitHub Pages registry: list of published plugins (name, author, git URL, tags).
   - `focalpint plugin list` queries the registry.
   - Unsigned registry; users verify author_pubkey themselves.
   - **Effort:** 1 agent subtask (generate static registry HTML/JSON); ~2-3 tool calls. **~0.5h wall clock.**

3. **Plugin Dependency Management:**
   - Plugins can declare dependencies on other plugins (rare, but allow it).
   - Runtime resolves dependency graph; detects cycles.
   - **Effort:** 1 agent subtask; ~3-4 tool calls. **~0.75h wall clock.**

**Deliverables:**
- `focal-plugin-loader` crate.
- Example FocalPoint config with 2-3 sample plugins.
- Plugin registry (GitHub Pages).

---

### Phase 5: Rule Builder UI (React Flow) (Week 5-6)

**Goal:** Web-based visual rule builder; exports CEL.

**Tasks:**
1. **ReactFlow Integration:**
   - Create React component `<RuleBuilder manifest={manifest} onSave={exportCEL} />`.
   - Nodes: Trigger (source), Condition (logic), Action (sink).
   - Edge: data flows from Trigger → Condition → Action.
   - **Effort:** 3 agent subtasks (React setup, node components, edge logic); ~5-6 tool calls. **~1.5h wall clock.**

2. **CEL Code Generation:**
   - Serialize the ReactFlow graph to CEL.
   - Validate CEL syntax before saving.
   - Round-trip: import existing CEL rule back into ReactFlow (best-effort).
   - **Effort:** 2 agent subtasks (serializer, validator); ~3-4 tool calls. **~1h wall clock.**

3. **Integration with FocalPoint Web App:**
   - Embed `<RuleBuilder>` in a new "Plugin Rules" settings panel.
   - Save rules back to plugin manifest (or in-memory config).
   - **Effort:** 1 agent subtask (route + state management); ~2-3 tool calls. **~0.5h wall clock.**

**Deliverables:**
- `RuleBuilder` React component.
- Rules editor page in FocalPoint web app.
- End-to-end test: build rule in UI, export CEL, evaluate against sample data.

---

### Phase 6: LLM Integration via MCP (Week 6-7)

**Goal:** Expose connector actions to Claude via Model Context Protocol.

**Tasks:**
1. **MCP Server Wrapper:**
   - Create `focal-mcp-server` (Rust, using `mcp-server-rust` crate if available, or manual HTTP/SSE).
   - Expose each plugin's actions as MCP tools.
   - MCP tool schema derived from plugin manifest `[config]` section.
   - **Effort:** 2 agent subtasks (MCP server setup, tool schema generation); ~5-6 tool calls. **~1.5h wall clock.**

2. **Tool Invocation:**
   - When Claude calls a tool (e.g., "send Slack notification"), MCP server invokes the plugin action with parameters.
   - Return result (success/error) to Claude.
   - **Effort:** 1 agent subtask; ~2-3 tool calls. **~0.5h wall clock.**

3. **Documentation:**
   - MCP server installation guide.
   - Example: "Ask Claude to send a Slack notification via FocalPoint."
   - **Effort:** 0.5 agent subtask (docs writer); ~1-2 tool calls. **~0.5h wall clock.**

**Deliverables:**
- `focal-mcp-server` crate.
- MCP client configuration for Claude Desktop / other LLM apps.
- Example LLM conversation: "Claude, use FocalPoint to notify my team."

---

### Phase 7: Testing & Documentation (Week 7)

**Goal:** Comprehensive test coverage; user and developer guides.

**Tasks:**
1. **Test Suite:**
   - Unit tests for each crate (manifest parsing, rule evaluation, WASM host, etc.).
   - Integration tests: plugin workflow (scaffold → build → test → sign → load).
   - Stress test: load 50 plugins, trigger 1,000 rules in parallel.
   - **Effort:** 3 agent subtasks (unit, integration, stress); ~6-8 tool calls. **~2h wall clock.**

2. **Developer Guide:**
   - "Build Your First Plugin" tutorial (Slack notifier).
   - API reference: Extism Rust PDK, manifest schema, CEL rule syntax.
   - Troubleshooting: common errors (WASM trap, signature mismatch, HTTP timeout).
   - **Effort:** 1 agent subtask (docs); ~2-3 tool calls. **~1h wall clock.**

3. **User Guide:**
   - "Install a Plugin" instructions.
   - "Configure Plugin Rules" (visual rule builder, CEL syntax).
   - Security considerations (verify author_pubkey, review capabilities).
   - **Effort:** 1 agent subtask; ~2-3 tool calls. **~1h wall clock.**

**Deliverables:**
- Test report (coverage %, pass/fail breakdown).
- Comprehensive documentation (dev + user guides).
- Quickstart video (optional).

---

## Overall Timeline Summary

| Phase | Duration | Crates Created | Status |
|-------|----------|-----------------|--------|
| 1. Foundation | 2 weeks | `focal-wasm-host`, `focal-plugin-manifest`, `focal-crypto` | Parallel 5 agents; ~1.5h+0.75h+0.5h |
| 2. Rule Engine | 1-2 weeks | `focal-rules-engine` | Parallel 3 agents; ~1.5h+0.5h+1h |
| 3. CLI & Packaging | 1-2 weeks | CLI subcommands, example plugin | Parallel 4 agents; ~0.75h+0.5h+1h+1h |
| 4. Plugin Loader & Registry | 1-2 weeks | `focal-plugin-loader` | Parallel 3 agents; ~1.5h+0.5h+0.75h |
| 5. Rule Builder UI | 1-2 weeks | React component | Parallel 3 agents; ~1.5h+1h+0.5h |
| 6. MCP Integration | 1 week | `focal-mcp-server` | Parallel 3 agents; ~1.5h+0.5h+0.5h |
| 7. Testing & Docs | 1 week | Test suite + guides | Parallel 3 agents; ~2h+1h+1h |
| **Total** | **6-7 weeks** | **~12 crates** | **Parallel execution across phases** |

**Wall Clock (with parallelization):** ~6-7 weeks. Sequential fallback: ~12-14 weeks.

---

## Final Verdict & Unblocking Recommendation

### Architecture Summary

| Aspect | Decision | Justification |
|--------|----------|---------------|
| **Execution Model** | WASM (Wasmtime host, Rust PDK) | Secure, sandboxed, third-party safe. Industry-standard (Zed, Extism). |
| **Manifest Format** | TOML | Human-readable, lightweight, aligns with focus-templates. |
| **Rule Language** | CEL (simple triggers) + Rego (optional, complex policies) | CEL for 80% of use cases; non-Turing-complete guarantees termination. Rego for future data-driven rules. |
| **Distribution** | Git-based (GitHub refs + hash) with ed25519 signing | Decentralized, auditable, no dependency on external registry. |
| **Signing Policy** | Per-author ed25519 keypair; FocalPoint's master key for official connectors | Scalable, author-controlled, revocation-friendly. |
| **iOS Compliance** | Pre-compiled plugins (build-time linking); runtime loading on macOS/web only | Compliant with App Store policy (no runtime JIT). |
| **UI Builder** | ReactFlow for visual rule composition (web); SwiftUI Picker for iOS simple selection | Expressive desktop UX; acceptable iOS UX given constraints. |
| **LLM Integration** | MCP server wrapping connectors | Unblocks Claude/LLM tool calling; 16,000+ MCP servers precedent. |

### Unblocking Questions

1. **Is the iOS App Store constraint acceptable?** (Yes/No)
   - **If Yes:** Proceed with pre-compiled plugins on iOS; ship runtime plugin loading on macOS/web.
   - **If No:** Pivot to sideload-only or macOS-only distribution.

2. **Should FocalPoint maintain a central plugin registry or stay fully decentralized?**
   - **Decentralized (recommend):** Users manage plugin repos directly in config; no central curation.
   - **Centralized (future phase):** GitHub Pages registry; optional author vetting.

3. **Should CEL be the only rule language or support Rego as well?**
   - **CEL only (recommend):** Simpler learning curve, sufficient for most focus triggers.
   - **Both:** Rego for advanced users; adds ~2-3 weeks to Phase 2.

### MVP Readiness

**This architecture unblocks a true MVP.** Deliverables:

1. **Connector SDK** for third parties to build WASM-based connectors without forking the monorepo.
2. **Manifest-based discovery** allowing users to install plugins from GitHub URLs.
3. **Signature verification** ensuring plugin authenticity.
4. **CEL rule evaluation** enabling user-defined triggers and actions.
5. **Web-based rule builder** (ReactFlow) for visual rule authoring.
6. **MCP server** for LLM integration (bonus: unblocks AI coaching).

**What's NOT in the MVP (future phases):**

- Rego support (can add if demand warrants).
- Central plugin registry (optional; GitHub Pages baseline suffices).
- iOS runtime plugin loading (App Store policy blocks this).
- Plugin version conflicts / dependency resolution (start simple; add if needed).

### Timeline to Market

- **MVP (Phases 1-5):** 5 weeks (focused on WASM host, manifest, CEL, UI builder).
- **Production-Ready (Phases 1-7):** 7 weeks (add MCP, LLM integration, comprehensive docs + testing).

---

## References & Sources

All citations follow the format `(source, accessed 2026-04-23)` as required.

- [Extism GitHub – Main Repository](https://github.com/extism/extism) (accessed 2026-04-23)
- [Extism PDK Documentation](https://extism.org/docs/concepts/pdk/) (accessed 2026-04-23)
- [Zed Extensions Architecture Blog](https://zed.dev/blog/zed-decoded-extensions) (accessed 2026-04-23)
- [Wasmtime Security Documentation](https://docs.wasmtime.dev/security.html) (accessed 2026-04-23)
- [Wasmer vs Wasmtime](https://wasmer.io/wasmer-vs-wasmtime) (accessed 2026-04-23)
- [Bytecode Alliance: wasi-threads Announcement](https://bytecodealliance.org/articles/wasi-threads) (accessed 2026-04-23)
- [Apple App Review Guidelines](https://developer.apple.com/app-store/review/guidelines/) (accessed 2026-04-23)
- [Raycast Manifest API Documentation](https://developers.raycast.com/information/manifest) (accessed 2026-04-23)
- [Obsidian Plugin Documentation](https://docs.obsidian.md/Reference/TypeScript+API/Plugin) (accessed 2026-04-23)
- [VS Code Contribution Points API](https://code.visualstudio.com/api/references/contribution-points) (accessed 2026-04-23)
- [VS Code Activation Events](https://code.visualstudio.com/api/references/activation-events) (accessed 2026-04-23)
- [CEL: Common Expression Language (cel.dev)](https://cel.dev/) (accessed 2026-04-23)
- [Google OSS Blog: CEL for Portable Policy](https://opensource.googleblog.com/2024/06/common-expressions-for-portable-policy.html) (accessed 2026-04-23)
- [Open Policy Agent Documentation](https://www.openpolicyagent.org/docs/policy-language) (accessed 2026-04-23)
- [Cozo GitHub](https://github.com/cozodb/cozo) (accessed 2026-04-23)
- [Differential Datalog GitHub](https://github.com/vmware-archive/differential-datalog) (accessed 2026-04-23)
- [Model Context Protocol Specification](https://modelcontextprotocol.io/specification/2025-11-25) (accessed 2026-04-23)
- [Introducing MCP (Anthropic Blog)](https://www.anthropic.com/news/model-context-protocol) (accessed 2026-04-23)
- [Apache Camel Route DSL](https://camel.apache.org/manual/dsl.html) (accessed 2026-04-23)
- [n8n Workflow Export/Import Documentation](https://docs.n8n.io/workflows/export-import/) (accessed 2026-04-23)
- [Airbyte Schema Reference](https://docs.airbyte.com/platform/connector-development/schema-reference) (accessed 2026-04-23)
- [Pipedream Components Overview](https://pipedream.com/docs/components) (accessed 2026-04-23)
- [ReactFlow Documentation](https://reactflow.dev/) (accessed 2026-04-23)
- [XyFlow Node-Based UIs](https://xyflow.com/) (accessed 2026-04-23)
- [SwiftUI Picker Documentation](https://codewithchris.com/swiftui-picker/) (accessed 2026-04-23)
- [App Intents 2024 Updates](https://matthewcassinelli.com/whats-new-in-app-intents-in-2024/) (accessed 2026-04-23)
- [Apple App Intents Documentation](https://developer.apple.com/documentation/appintents) (accessed 2026-04-23)
- [WebAssembly Runtime Benchmarks 2026](https://wasmruntime.com/en/benchmarks) (accessed 2026-04-23)
- [Comparing WebAssembly Runtimes](https://reintech.io/blog/wasmtime-vs-wasmer-vs-wasmedge-wasm-runtime-comparison-2026) (accessed 2026-04-23)
- [Zed Cross-Editor Standard Discussion](https://github.com/zed-industries/zed/discussions/48464) (accessed 2026-04-23)
- [Tauri Inter-Process Communication](https://v2.tauri.app/concept/inter-process-communication/) (accessed 2026-04-23)
- [Node.js WASI Documentation](https://nodejs.org/api/wasi.html) (accessed 2026-04-23)
- [WASI and the WebAssembly Component Model](https://eunomia.dev/blog/2025/02/16/wasi-and-the-webassembly-component-model-current-status/) (accessed 2026-04-23)
- [Obsidian Sample Plugin](https://github.com/obsidianmd/obsidian-sample-plugin) (accessed 2026-04-23)

---

**Document Status:** Complete. Ready for executive review and stakeholder validation on iOS sideload vs. App Store strategy.
