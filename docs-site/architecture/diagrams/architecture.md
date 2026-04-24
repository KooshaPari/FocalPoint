# Architecture Diagrams

Three key diagrams illustrating FocalPoint's design.

## 1. High-Level C4 Context Diagram

```mermaid
graph TB
    subgraph Platform["Platform Layer"]
        iOS["iOS<br/>(SwiftUI + FamilyControls)"]
        Android["Android<br/>(Kotlin + UsageStats)"]
        Web["Web<br/>(Vue/React)"]
        CLI["CLI<br/>(clap)"]
    end

    subgraph Bridge["UniFFI Bridge"]
        FFI["Swift/Kotlin Bindings<br/>(auto-generated from Rust)"]
    end

    subgraph Core["Rust Core (40+ crates)"]
        Ingestion["Data Ingestion<br/>(focus-sync, connectors)"]
        Processing["Processing Pipeline<br/>(focus-eval, focus-rules)"]
        State["State Management<br/>(focus-storage, focus-rewards, focus-audit)"]
        API["Public API Traits<br/>(Connector, EventStore, RuleStore)"]
    end

    subgraph Persistence["Persistence & Trust"]
        SQLite["SQLite<br/>(local-first)"]
        Chain["Audit Chain<br/>(SHA-256 append-only)"]
    end

    subgraph External["External Services"]
        GitHub["GitHub<br/>(webhooks, API)"]
        Canvas["Canvas LMS<br/>(LTI 1.3)"]
        GCal["Google Calendar<br/>(OAuth2)"]
        Others["Strava, Fitbit, Notion,<br/>Readwise, Linear..."]
    end

    subgraph Optional["Optional Services"]
        WebhookServer["Webhook Server<br/>(HTTP receiver)"]
        MCPServer["MCP Server<br/>(Claude integration)"]
        Registry["Template Registry<br/>(pack catalog)"]
    end

    iOS --> FFI
    Android --> FFI
    Web --> Core
    CLI --> Core
    FFI --> API
    API --> Ingestion
    Ingestion --> Processing
    Processing --> State
    State --> SQLite
    State --> Chain
    Processing --> Optional
    External --> WebhookServer
    WebhookServer --> Ingestion
    Optional -.-> SQLite
```

## 2. Data Flow: GitHub PR → Credits (Sequence Diagram)

```mermaid
sequenceDiagram
    participant GH as GitHub
    participant WH as Webhook<br/>Server
    participant Conn as Connector<br/>(GitHub)
    participant Sync as focus-sync
    participant Store as focus-storage<br/>(SQLite)
    participant Eval as focus-eval<br/>(Rule Engine)
    participant Rules as focus-rules
    participant Wallet as focus-rewards<br/>(Ledger)
    participant Chain as focus-audit<br/>(SHA-256)
    participant UI as iOS/Web UI

    GH->>WH: POST /webhook<br/>(JWS signed)
    WH->>WH: Verify HMAC signature
    WH->>Conn: Transform to Event<br/>(PullRequestMerged)
    Conn->>Sync: append_event(event)
    Sync->>Store: INSERT INTO events table
    Sync->>Chain: record_state_change(action)
    Chain->>Chain: Compute SHA-256,<br/>link to prev hash
    Eval->>Rules: Load user's rules
    Eval->>Eval: Match:<br/>app_github && event_pr_merged
    Eval->>Eval: Rule matched:<br/>"credit 50"
    Eval->>Wallet: credit(user_id, 50)
    Wallet->>Wallet: INSERT ledger entry
    Wallet->>Chain: record_credit(user_id, 50)
    Chain->>Chain: Create audit record,<br/>update hash chain
    UI->>Store: Query wallet balance
    Store-->>UI: Return balance
    UI->>UI: Display "+50 credits!"
```

## 3. Focus Session State Machine

```mermaid
stateDiagram-v2
    [*] --> Idle

    Idle --> Started: User taps<br/>"Start Focus"

    Started --> Paused: User pauses
    Started --> Completed: Timer expires ✓
    Started --> Cancelled: User cancels ✗
    Started --> Interrupted: Distracting app<br/>detected ⚠️

    Paused --> Started: User taps<br/>"Resume"
    Paused --> Cancelled: User gives up ✗

    Interrupted --> Resumed: User refocuses
    Interrupted --> Cancelled: User cancels ✗

    Completed --> Idle: Reward shown<br/>(+credits)

    Resumed --> Idle: Bonus credited<br/>(+resumption bonus)

    Cancelled --> Idle: Penalty applied<br/>(if configured)
```

Each state transition:
- Creates an audit record (via `focus-audit`)
- May trigger a reward or penalty (via `focus-rewards`, `focus-penalties`)
- Persists to SQLite (via `focus-storage`)
- Is published to UI listeners (via `focus-sync`)

## Key Design Patterns

### Trait-Driven Architecture
All major concerns expose stable traits:
- `Connector`: Implement to add a data source
- `EventStore`: Implement to change persistence backend
- `RuleStore`: Implement to change rule loading
- `ClockPort`: Abstract time for testing
- `SecureSecretStore`: Abstract credential storage

### Audit-Driven State
Every mutation is recorded in the append-only chain:
```
Mutation → AuditRecord → SHA-256 Hash → Previous Hash Link
```
This makes all state transitions reproducible and tamper-evident.

### Layered Evaluation
```
Raw Event → Rule Matcher → Policy Check → Ledger Update → Audit Record
```
Each layer is independently testable; failures are explicit.

## For More Details

- **System Overview:** `/architecture/system_overview`
- **Crates Map:** `/architecture/crates_map`
- **FFI Topology:** `/architecture/ffi-topology`
- **Connector Framework:** `/architecture/connector-framework`
- **Testing Strategy:** `/architecture/testing_strategy`
