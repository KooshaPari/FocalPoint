# FocalPoint Intermediate Representation (IR) Specification

## Format Choice: JSON

**Format**: JSON (not MessagePack, Protobuf, or CBOR)

**Rationale**:
- **Human-readable**: Debuggable and auditable in diffs; engineers can inspect the IR without special tools
- **Language-agnostic**: JavaScript, Python, Go, Rust, Swift all have native JSON support; no runtime dependency on Protobuf/MessagePack libraries for consumers
- **Stable & standardized**: JSON Schema validation is mature; drift detection works well
- **Git-friendly**: Diffs are meaningful; merge conflicts are resolvable by hand
- **Signature-friendly**: Canonical JSON formatting (RFC 7159 + stable key ordering) enables deterministic hashing (content-addressing)
- **API transparency**: REST endpoints return JSON natively; no extra serialization layer
- **Trade-offs accepted**: ~20% larger than MessagePack (not a concern for <1 MB workspaces); microseconds slower than Protobuf on large batches (< 1 s compile time is acceptable)

**Schema versioning**: IR documents include a top-level `version: "1.0"` field. Migration logic handles `v1 → v2` upgrades in-place without breaking existing packs.

---

## IR Schema: Top-Level Document Structure

Every FocalPoint document (rule, connector, template, task, etc.) follows this shape:

```json
{
  "version": "1.0",
  "kind": "Rule" | "Connector" | "Template" | "Task" | "Schedule" | "Pose" | "CoachingConfig" | "EnforcementPolicy" | "WalletMutation" | "Ritual" | "SoundCue" | "AuditQuery",
  "id": "rule-dw-social-v1",
  "name": "deep-work-social-block",
  "description": "During deep work, block social apps",
  "metadata": {
    "author": "coaching-team",
    "created": "2026-04-23T10:30:00Z",
    "modified": "2026-04-23T10:30:00Z",
    "version": "1.0",
    "tags": ["focus", "social", "enforcement"]
  },
  "body": { /* variant-specific content */ },
  "signature": {
    "hash": "sha256:abc123def456...",
    "public_key": "ed25519:xyz789...",
    "signature": "base64:...",
    "timestamp": "2026-04-23T10:30:00Z"
  }
}
```

---

## Canonical IR Types (Rust Source of Truth)

Define all IR types as Rust enums with `serde` derives. JSON Schema and TypeScript types are **generated** from these via tooling (e.g., `serde_json_schema`, `ts-rs`).

### Trigger (Union Type)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Trigger {
  UserStartsSession { session_type: String },
  EventFired { event_name: String },
  TimeElapsed { duration_ms: u64 },
  ScheduleCron { cron_expression: String, timezone: String },
  WebhookReceived { path: String, method: String },
  UserAction { action_type: String, target: String },
  ConditionMet { condition: Condition },
}

// Serializes to:
// { "type": "UserStartsSession", "value": { "session_type": "focus" } }
// or
// { "type": "EventFired", "value": { "event_name": "canvas_submission" } }
```

### Condition (Boolean Expression)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum Condition {
  #[serde(rename = "and")]
  And { conditions: Vec<Condition> },
  #[serde(rename = "or")]
  Or { conditions: Vec<Condition> },
  #[serde(rename = "not")]
  Not { condition: Box<Condition> },
  
  #[serde(rename = "time_in_range")]
  TimeInRange { start_hour: u8, end_hour: u8 },
  #[serde(rename = "day_of_week")]
  DayOfWeek { days: Vec<String> }, // ["Monday", "Tuesday", ...]
  #[serde(rename = "user_attribute")]
  UserAttribute { key: String, value: String },
  #[serde(rename = "event_property")]
  EventProperty { property: String, expected: serde_json::Value },
  #[serde(rename = "custom_predicate")]
  CustomPredicate { name: String, args: serde_json::Value },
}

// Serializes to:
// {
//   "op": "and",
//   "conditions": [
//     { "op": "time_in_range", "start_hour": 8, "end_hour": 16 },
//     { "op": "user_attribute", "key": "focus_mode_active", "value": "true" }
//   ]
// }
```

### Action (What to Execute)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
  #[serde(rename = "enforce_policy")]
  EnforcePolicy {
    policy_id: String,
    #[serde(default)]
    params: std::collections::BTreeMap<String, serde_json::Value>,
  },
  
  #[serde(rename = "emit_event")]
  EmitEvent {
    event_type: String,
    #[serde(default)]
    payload: std::collections::BTreeMap<String, serde_json::Value>,
  },
  
  #[serde(rename = "apply_mutation")]
  ApplyMutation {
    mutation_id: String,
    #[serde(default)]
    params: std::collections::BTreeMap<String, serde_json::Value>,
  },
  
  #[serde(rename = "schedule_task")]
  ScheduleTask {
    task_id: String,
    delay_ms: Option<u64>,
    #[serde(default)]
    params: std::collections::BTreeMap<String, serde_json::Value>,
  },
  
  #[serde(rename = "trigger_sequence")]
  TriggerSequence { actions: Vec<Action> },
  
  #[serde(rename = "show_notification")]
  ShowNotification {
    notification_id: String,
    text: String,
    #[serde(default)]
    duration_ms: Option<u64>,
  },
}
```

### Rule (Top-Level)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub enabled: bool,
  pub trigger: Trigger,
  pub conditions: Vec<Condition>,
  pub actions: Vec<Action>,
  pub metadata: RuleMetadata,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMetadata {
  pub author: String,
  pub created: String, // RFC 3339 timestamp
  pub modified: String,
  pub version: String,
  #[serde(default)]
  pub tags: Vec<String>,
}
```

### Connector (Integration Definition)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
  pub id: String,
  pub name: String,
  pub version: String,
  pub kind: String, // "oauth", "api_key", "webhook", "rpc"
  pub schema: ConnectorSchema,
  pub auth: AuthScheme,
  pub endpoints: Vec<Endpoint>,
  pub metadata: ConnectorMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorSchema {
  pub properties: std::collections::BTreeMap<String, PropertyDef>,
  pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthScheme {
  #[serde(rename = "oauth2")]
  OAuth2 { client_id: String, scopes: Vec<String> },
  #[serde(rename = "api_key")]
  ApiKey { key_name: String },
  #[serde(rename = "bearer")]
  Bearer { token_name: String },
  #[serde(rename = "basic")]
  Basic { username_field: String, password_field: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
  pub method: String, // "GET", "POST", "PUT"
  pub path: String,
  pub description: Option<String>,
  pub params: std::collections::BTreeMap<String, ParamDef>,
}
```

### Template (Reusable Composition)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub inputs: std::collections::BTreeMap<String, InputDef>,
  pub rules: Vec<Rule>,
  pub metadata: TemplateMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDef {
  pub type_: String, // "string", "number", "boolean", "schedule"
  pub default: Option<serde_json::Value>,
  pub description: Option<String>,
}
```

### Task (Executable Unit)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub handler: TaskHandler,
  pub inputs: std::collections::BTreeMap<String, ParamDef>,
  pub outputs: std::collections::BTreeMap<String, ParamDef>,
  pub timeout_ms: Option<u64>,
  pub retry_policy: Option<RetryPolicy>,
  pub metadata: TaskMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TaskHandler {
  #[serde(rename = "builtin")]
  Builtin { name: String },
  #[serde(rename = "connector")]
  Connector { connector_id: String, endpoint: String },
  #[serde(rename = "webhook")]
  Webhook { url: String, method: String },
  #[serde(rename = "sequence")]
  Sequence { tasks: Vec<Task> },
}
```

### Schedule (Temporal Trigger)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
  pub id: String,
  pub name: String,
  pub trigger: ScheduleTrigger,
  pub timezone: String,
  pub enabled: bool,
  pub metadata: ScheduleMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ScheduleTrigger {
  #[serde(rename = "cron")]
  Cron { expression: String },
  #[serde(rename = "interval")]
  Interval { every_ms: u64 },
  #[serde(rename = "daily_at")]
  DailyAt { hour: u8, minute: u8 },
  #[serde(rename = "weekly_at")]
  WeeklyAt { day: String, hour: u8, minute: u8 },
  #[serde(rename = "monthly_at")]
  MonthlyAt { day: u8, hour: u8, minute: u8 },
}
```

### Pose (Mascot Visual State)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pose {
  pub id: String,
  pub name: String,
  pub character: String, // "default", "mentor", "cheerleader"
  pub pose: String, // "neutral", "thumbs_up", "thinking", "excited"
  pub emotion: String, // "happy", "neutral", "sad", "confused"
  pub accessory: Option<String>, // "glasses", "hat", "none"
  pub speech_bubble: Option<SpeechBubble>,
  pub voice_cue: Option<String>, // reference to voice_id
  pub sound_cue: Option<String>, // reference to sound_id
  pub haptic_cue: Option<String>, // "light_tap", "medium_vibration", "success_pulse"
  pub entry_animation: Option<Animation>,
  pub hold_duration_ms: Option<u64>,
  pub exit_animation: Option<Animation>,
  pub metadata: PoseMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechBubble {
  pub text: String,
  pub text_alignment: Option<String>, // "left", "center", "right"
  pub background_style: Option<String>, // "cloud", "rectangular", "rounded"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
  pub type_: String, // "fade_in", "slide_from_left", "bounce", "pop"
  pub duration_ms: u64,
  pub easing: Option<String>, // "ease_in", "ease_out", "linear"
}
```

### CoachingConfig (Tone & Voice Settings)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoachingConfig {
  pub id: String,
  pub name: String,
  pub tone: String, // "encouraging", "neutral", "challenging", "humorous"
  pub language: String, // "en", "es", "fr"
  pub voice_profile: Option<VoiceProfile>,
  pub text_templates: std::collections::BTreeMap<String, String>,
  pub notification_style: Option<String>, // "toast", "banner", "modal"
  pub metadata: CoachingMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceProfile {
  pub voice_id: String,
  pub speed: f32, // 0.5..2.0
  pub pitch: f32, // 0.5..2.0
  pub accent: Option<String>, // "american", "british"
}
```

### EnforcementPolicy (Rule Constraint)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementPolicy {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub targets: Vec<String>, // what this policy enforces on
  pub threshold: Option<Threshold>,
  pub action_on_violation: Action,
  pub grace_period_ms: Option<u64>,
  pub metadata: PolicyMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Threshold {
  #[serde(rename = "count")]
  Count { max: u64 },
  #[serde(rename = "duration")]
  Duration { max_ms: u64 },
  #[serde(rename = "frequency")]
  Frequency { max_per_hour: u64 },
  #[serde(rename = "custom")]
  Custom { predicate: String },
}
```

### WalletMutation (Points/Rewards)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMutation {
  pub id: String,
  pub name: String,
  pub wallet_type: String, // "points", "badges", "currency"
  pub operation: MutationOp,
  pub amount: i64,
  pub reason: String,
  pub conditional: Option<Condition>,
  pub metadata: MutationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MutationOp {
  Add,
  Subtract,
  Multiply,
  Set,
  Transfer,
}
```

### Ritual (Habit Loop Sequence)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ritual {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub steps: Vec<RitualStep>,
  pub daily_goal: Option<u64>, // repeats per day
  pub tracking: RitualTracking,
  pub rewards: Vec<WalletMutation>,
  pub metadata: RitualMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualStep {
  pub sequence: u32,
  pub name: String,
  pub description: Option<String>,
  pub cue: String, // trigger or contextual cue
  pub routine: Task, // what to do
  pub reward: Option<String>, // visual/audio reward
  pub estimated_duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualTracking {
  pub enabled: bool,
  pub track_completion: bool,
  pub track_duration: bool,
  pub track_quality: bool,
}
```

### SoundCue (Audio Asset Reference)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundCue {
  pub id: String,
  pub name: String,
  pub asset_url: String, // CDN or local path
  pub asset_hash: String, // sha256: for integrity
  pub duration_ms: u64,
  pub volume_level: f32, // 0.0..1.0
  pub tags: Vec<String>, // "positive", "notification", "reward"
  pub usage: String, // "reward", "notification", "error"
  pub metadata: SoundMetadata,
}
```

### AuditQuery (Event Query)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditQuery {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub event_filter: EventFilter,
  pub projections: Vec<String>, // columns to select
  pub aggregations: Vec<Aggregation>,
  pub time_range: Option<TimeRange>,
  pub metadata: AuditMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
  pub event_types: Vec<String>,
  pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Aggregation {
  #[serde(rename = "count")]
  Count { field: Option<String> },
  #[serde(rename = "sum")]
  Sum { field: String },
  #[serde(rename = "avg")]
  Average { field: String },
  #[serde(rename = "distinct")]
  Distinct { field: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
  pub start: String, // RFC 3339
  pub end: Option<String>,
}
```

---

## Signature & Content-Addressing

Every IR document is signed for integrity and authenticity:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
  pub hash: String, // "sha256:<hex>" of canonical JSON
  pub public_key: String, // "ed25519:<base64>"
  pub signature: String, // "base64:<ed25519-signature>"
  pub timestamp: String, // RFC 3339
}
```

**Canonical JSON format** (for deterministic hashing):
1. All keys sorted alphabetically
2. No whitespace (compact)
3. UTF-8 encoding
4. Consistent date formatting (RFC 3339)

```rust
pub fn canonical_json(doc: &Document) -> Result<String> {
  let json = serde_json::to_string(doc)?;
  let value: serde_json::Value = serde_json::from_str(&json)?;
  let canonical = serde_json::to_string(&sort_json_object(&value))?;
  Ok(canonical)
}

pub fn content_hash(doc: &Document) -> Result<String> {
  let canonical = canonical_json(doc)?;
  let hash = sha256(canonical.as_bytes());
  Ok(format!("sha256:{}", hex::encode(hash)))
}

pub fn sign(doc: &Document, signing_key: &SigningKey) -> Result<Signature> {
  let hash = content_hash(doc)?;
  let signature = signing_key.sign(hash.as_bytes());
  Ok(Signature {
    hash,
    public_key: format!("ed25519:{}", base64::encode(signing_key.public_key())),
    signature: base64::encode(&signature),
    timestamp: chrono::Utc::now().to_rfc3339(),
  })
}
```

**Applications**:
- **Integrity**: Verify document hasn't been modified
- **Authenticity**: Verify document came from expected author
- **Distribution**: Sign packs once, distribute to many users
- **Audit trail**: Git commits become audit records ("who signed this, when")

---

## Schema Evolution (v1 → v2)

IR is versioned. Migrations are explicit and one-way:

```rust
pub fn migrate_v1_to_v2(doc_v1: &DocumentV1) -> Result<DocumentV2> {
  // Explicit migration logic
  // Old fields map to new fields or are dropped with warning
  // New required fields get sensible defaults
  // Incompatible changes are rejected (e.g., removing a required field)
}

// At parse time:
pub fn load_document(json: &str) -> Result<Document> {
  let value: serde_json::Value = serde_json::from_str(json)?;
  let version = value["version"].as_str().unwrap_or("1.0");
  
  match version {
    "1.0" => {
      let doc_v1: DocumentV1 = serde_json::from_str(json)?;
      Ok(migrate_v1_to_v2(&doc_v1)?)
    },
    "2.0" => serde_json::from_str(json).map_err(|e| e.into()),
    _ => Err(format!("Unknown version: {}", version).into()),
  }
}
```

---

## Example IR Documents

### Rule (deep-work-social-block)

```json
{
  "version": "1.0",
  "kind": "Rule",
  "id": "rule-dw-social-v1",
  "name": "deep-work-social-block",
  "description": "During deep work, block social apps",
  "metadata": {
    "author": "coaching-team",
    "created": "2026-04-23T10:30:00Z",
    "modified": "2026-04-23T10:30:00Z",
    "version": "1.0",
    "tags": ["focus", "social", "enforcement"]
  },
  "body": {
    "enabled": true,
    "trigger": {
      "type": "UserStartsSession",
      "value": {
        "session_type": "focus"
      }
    },
    "conditions": [
      {
        "op": "time_in_range",
        "start_hour": 8,
        "end_hour": 16
      },
      {
        "op": "and",
        "conditions": [
          {
            "op": "user_attribute",
            "key": "focus_mode_active",
            "value": "true"
          }
        ]
      }
    ],
    "actions": [
      {
        "type": "enforce_policy",
        "policy_id": "social-media-lockout",
        "params": {
          "duration_minutes": 120,
          "notification": "social-blocked-during-focus"
        }
      }
    ]
  },
  "signature": {
    "hash": "sha256:abc123def456...",
    "public_key": "ed25519:xyz789...",
    "signature": "base64:...",
    "timestamp": "2026-04-23T10:30:00Z"
  }
}
```

---

## JSON Schema Export

From the Rust enum definitions, generate JSON Schema (via `serde_json_schema` or manual tooling):

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Document",
  "type": "object",
  "required": ["version", "kind", "id", "name"],
  "properties": {
    "version": { "type": "string" },
    "kind": {
      "type": "string",
      "enum": ["Rule", "Connector", "Template", "Task", "Schedule", "Pose", "CoachingConfig", "EnforcementPolicy", "WalletMutation", "Ritual", "SoundCue", "AuditQuery"]
    },
    "id": { "type": "string" },
    "name": { "type": "string" },
    "description": { "type": ["string", "null"] },
    "metadata": { "$ref": "#/definitions/Metadata" },
    "body": {
      "oneOf": [
        { "$ref": "#/definitions/Rule" },
        { "$ref": "#/definitions/Connector" },
        { "$ref": "#/definitions/Template" },
        // ... etc for each kind
      ]
    },
    "signature": { "$ref": "#/definitions/Signature" }
  }
}
```

---

## TypeScript Codegen

From the same Rust definitions, generate TypeScript types (via `ts-rs` or similar):

```typescript
export interface Document {
  version: string;
  kind: "Rule" | "Connector" | "Template" | "Task" | "Schedule" | "Pose" | "CoachingConfig" | "EnforcementPolicy" | "WalletMutation" | "Ritual" | "SoundCue" | "AuditQuery";
  id: string;
  name: string;
  description?: string;
  metadata: Metadata;
  body: Rule | Connector | Template | Task | Schedule | Pose | CoachingConfig | EnforcementPolicy | WalletMutation | Ritual | SoundCue | AuditQuery;
  signature?: Signature;
}

export interface Rule {
  enabled: boolean;
  trigger: Trigger;
  conditions: Condition[];
  actions: Action[];
}

export type Trigger =
  | { type: "UserStartsSession"; value: { session_type: string } }
  | { type: "EventFired"; value: { event_name: string } }
  | { type: "TimeElapsed"; value: { duration_ms: number } }
  | { type: "ScheduleCron"; value: { cron_expression: string; timezone: string } }
  | { type: "WebhookReceived"; value: { path: string; method: string } }
  | { type: "UserAction"; value: { action_type: string; target: string } }
  | { type: "ConditionMet"; value: { condition: Condition } };

// ... rest of types
```

---

## Swift Codegen

From Rust, generate Swift Codable types (via `swift-codegen` or manual):

```swift
struct Document: Codable {
  let version: String
  let kind: Kind
  let id: String
  let name: String
  let description: String?
  let metadata: Metadata
  let body: AnyCodable
  let signature: Signature?
}

enum Trigger: Codable {
  case userStartsSession(sessionType: String)
  case eventFired(eventName: String)
  case timeElapsed(durationMs: UInt64)
  // ... etc
}
```

---

## Summary

FocalPoint IR is:

- **Format**: JSON (human-readable, git-friendly, stable hashing)
- **Versioning**: Top-level `version: "1.0"`, explicit v1→v2 migrations
- **Canonical types**: Rust enums with serde, auto-codegen to JSON Schema + TypeScript + Swift
- **Content-addressing**: SHA256 hash + ed25519 signature on canonical JSON
- **Signatures**: Preserve integrity, authenticity, and audit trail
- **Schema**: Single `Document` wrapper; 12 primitive types (Rule, Connector, Template, Task, Schedule, Pose, CoachingConfig, EnforcementPolicy, WalletMutation, Ritual, SoundCue, AuditQuery)

All authoring surfaces (FPL, Graph, CLI, GUI Wizard) compile to this one IR format.
