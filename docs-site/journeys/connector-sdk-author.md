---
title: "User Journey: Connector SDK Developer"
description: How developers build and publish custom FocalPoint connectors.
---

# User Journey: Connector SDK Developer

## Persona

**Name**: Carol  
**Context**: Backend engineer, wants to build a Todoist connector  
**Goal**: Integrate Todoist task deadlines into FocalPoint  
**Challenge**: First time building a connector; needs guidance  

## Phase 1: Planning & Scaffolding (2 hours)

### Step 1: Review Documentation (30 min)

Carol reads through:

1. [Connector SDK Spec](../connector-sdk/)
2. [Manifest Format](../connector-sdk/manifest)
3. [Event Schema](../connector-sdk/events)
4. Canvas connector source code (reference)

### Step 2: Create Manifest (30 min)

Carol writes `connector.toml`:

```toml
[connector]
id = "todoist-connector"
name = "Todoist"
version = "0.1.0"
description = "Sync task deadlines from Todoist"
author = "Carol"
license = "MIT"

[auth]
type = "oauth2"
provider = "todoist"
scopes = ["task:read", "project:read"]

[events]
"todoist.task.created" = "Task created in Todoist"
"todoist.task.due_soon" = "Task due approaching"
"todoist.project.deadline" = "Project deadline"

[capabilities]
permissions = ["read:tasks", "read:projects"]
```

### Step 3: Set Up Rust Project (30 min)

```bash
cargo new --lib todoist-connector
cd todoist-connector

# Add dependencies
cargo add tokio serde serde_json reqwest chrono
cargo add focalpoint-sdk
```

### Step 4: Define Event Schemas (30 min)

Carol creates `schemas/task_created.json`:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "task_id": { "type": "string" },
    "title": { "type": "string" },
    "due_date": { "type": "string", "format": "date-time" },
    "project_id": { "type": "string" },
    "priority": { "type": "integer", "minimum": 1, "maximum": 4 }
  },
  "required": ["task_id", "title"]
}
```

## Phase 2: Implementation (6 hours)

### OAuth Flow (2 hours)

```rust
use focalpoint_sdk::oauth;

pub async fn authenticate(auth_code: &str) -> Result<String> {
    let token = oauth::exchange_code_for_token(
        "todoist",
        auth_code,
    ).await?;
    Ok(token)
}
```

### Task Sync (3 hours)

```rust
pub async fn fetch_and_emit_tasks(token: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let tasks = client
        .get("https://api.todoist.com/rest/v2/tasks")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json::<Vec<Task>>()
        .await?;

    for task in tasks {
        emit_event(Event {
            event_type: "todoist.task.created".to_string(),
            payload: serde_json::json!({
                "task_id": task.id,
                "title": task.content,
                "due_date": task.due_date,
                "priority": task.priority,
            }),
        })?;
    }
    Ok(())
}
```

### Tests (1 hour)

```rust
#[tokio::test]
async fn test_fetch_tasks() {
    let mock_token = "test_token";
    let tasks = fetch_and_emit_tasks(mock_token).await;
    assert!(tasks.is_ok());
}

#[test]
fn test_event_schema() {
    let payload = serde_json::json!({
        "task_id": "123",
        "title": "Buy groceries",
        "due_date": "2026-04-24T23:59:59Z",
    });
    assert!(validate_schema(&payload, "task_created"));
}
```

## Phase 3: Testing & Validation (3 hours)

### Local Testing

```bash
# Unit tests
cargo test

# Validation
focalpoint connector validate-manifest connector.toml

# Integration test
focalpoint connector test-auth --manifest connector.toml
```

### Manual Testing

Carol sets up a test Todoist account and:

1. Authorizes FocalPoint
2. Creates test tasks
3. Verifies events are emitted correctly
4. Tests rule triggering

```yaml
name: "Test: Todoist task due soon"
trigger:
  - event_type: "todoist.task.due_soon"
    hours_until: 24
action:
  - send_notification: "Todoist task: {{event.title}}"
  - log_audit: "Test trigger fired"
```

All tests pass. ✓

## Phase 4: Documentation (2 hours)

Carol writes:

1. **README.md**: Setup instructions, examples
2. **CHANGELOG.md**: v0.1.0 initial release
3. **Event reference**: Documenting each event type
4. **Example rule**: "Todoist deadline focus"

README excerpt:

```markdown
# Todoist Connector for FocalPoint

## Installation

1. In FocalPoint, go to Settings → Connectors
2. Search for "Todoist"
3. Tap "Install"
4. Authorize with your Todoist account
5. Done!

## Events Emitted

- `todoist.task.created` — Task created or synced
- `todoist.task.due_soon` — Task approaching deadline
- `todoist.project.deadline` — Project deadline
```

## Phase 5: Publishing (2 hours)

### Prepare Submission

Carol creates a GitHub PR with:

- Source code + tests
- Manifest
- Documentation
- Event schemas

Checklist:

```
[✓] Manifest is valid
[✓] Tests pass (70% coverage)
[✓] OAuth flow tested with real Todoist account
[✓] No hardcoded credentials
[✓] README with setup + troubleshooting
[✓] Example rule provided
[✓] Code formatted (cargo fmt)
[✓] No clippy warnings (cargo clippy)
```

### Verification Process

FocalPoint maintainers review:

**Day 1–2**: Initial review

```
Feedback:
- Missing error handling for network timeouts
- Add rate limiting (Todoist API has quotas)
- Expand example rules
- Add privacy note about what data is accessed
```

Carol addresses feedback:

```rust
// Add timeout handling
let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()?;

// Add rate limiting
rate_limiter.wait_if_necessary();

// Add error context
.context("Failed to fetch tasks from Todoist API")?
```

**Day 3–5**: Final review

Maintainers verify:

- ✓ Security: No credential leaks
- ✓ Functionality: OAuth works; events emit correctly
- ✓ Tests: 75% coverage
- ✓ Documentation: Clear and complete

### Approval & Listing

Connector is merged and listed in marketplace as **Verified**:

```
Todoist
Sync task deadlines and project milestones from Todoist.
Version: 0.1.0 | Downloads: 0 | Rating: (new)
[Install] [View source]
```

## Phase 6: First Users & Feedback (Ongoing)

### User 1: David (Student)

David installs the Todoist connector. He creates a rule:

```yaml
name: "High-priority Todoist task focus"
trigger:
  - event_type: "todoist.task.due_soon"
    priority: 4
action:
  - show_focus_view: "productivity"
  - coach_message: "{{event.title}} due soon"
```

David submits feedback: "Works great! Would love webhook support for real-time updates."

Carol notes this for v0.2.0.

### User 2: Eve (Developer)

Eve uses Todoist for sprint planning. She suggests combining Todoist + GitHub:

```
"I want: GitHub issue → Todoist task → FocalPoint focus block"
```

Carol realizes her connector could be the first in a "project management integration pack."

## Success Metrics

```
v0.1.0 Released: ✓
Verification: Passed (Verified tier)
Test Coverage: 75%
User Downloads (Week 1): 23
User Feedback: Positive (1 feature request)
Code Quality: 100% clippy pass, formatted
```

## Key Moments

| Milestone | Time | Emotion |
|-----------|------|---------|
| Finished auth flow | 2h | Excited |
| Tests pass | 4h | Confident |
| Submitted to marketplace | 13h | Nervous |
| Feedback from maintainers | 16h | Thoughtful |
| Approved & listed | 18h | Proud |
| First user thanks | Week 1 | Fulfilled |

See also: [Connector SDK Spec](../connector-sdk/), [Manifest Format](../connector-sdk/manifest), [Canvas Example](../connectors/canvas)
