# FocalPoint Language (FPL) Specification

## Language Choice: Starlark

**Language**: Starlark (Python subset, sandboxable, used by Bazel, Google)

**Rationale** (comparison table):

| Language | Readability | Sandboxability | Bazel/OSS Precedent | Rust Crate | Learning Curve | Config-native | Score |
|----------|------------|-----------------|----------------------|------------|----------------|---------------|-------|
| **Starlark** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ✅ (Google standard) | starlark-rust | Low (Python subset) | ⭐⭐⭐⭐ | **20** |
| Rhai | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | Limited (Rust-only) | rhai | Medium | ⭐⭐⭐ | 15 |
| HCL | ⭐⭐⭐ | ⭐⭐⭐ | ✅ (Terraform) | hcl | Medium | ⭐⭐⭐⭐⭐ | 16 |
| Lua | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | Classic (Redis, Nginx) | mlua | Low-Medium | ⭐⭐⭐ | 17 |
| Dhall | ⭐⭐ | ⭐⭐⭐⭐ | Niche (functional) | dhall | High | ⭐⭐⭐⭐⭐ | 13 |
| CEL | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | Google (policy) | cel-rust | Medium | ⭐⭐⭐ | 14 |

**Starlark wins because**:
1. **Familiar syntax** (Python subset, ~70% of developers know Python)
2. **Sandbox-proven** (used at Google scale for Bazel; no filesystem/network by default)
3. **Deterministic evaluation** (no side effects, reproducible runs)
4. **Rich tooling** (`starlark-rust` has Rust FFI, `starlark-go` has Go bindings, `starlark` has Python)
5. **Documentation** (Starlark reference manual is clear; Bazel BUILD examples are everywhere)
6. **Loops & conditionals** (unlike HCL, which is declarative-only; unlike CEL, which is expression-only)
7. **Cross-language adoption** (Bazel uses it; TensorFlow uses it; Pants uses it)

---

## Grammar & Syntax

### File Structure

```starlark
# focalpoint.fpl.toml — workspace manifest (lists all entry .fpl files)
[workspace]
name = "school-focus-coaching"
version = "1.0.0"
entry_files = [
  "rules/deep-work-social-block.fpl",
  "rules/pomodoro-reward.fpl",
  "templates/school-weekday-focus.fpl",
]
signature_required = true
```

### FPL Grammar (EBNF-ish)

```ebnf
<document> ::= <import>* <definition>*

<import> ::= "import" STRING "as" IDENT
           | "import" STRING
  
<definition> ::= <rule_def>
               | <connector_def>
               | <template_def>
               | <task_def>
               | <schedule_def>
               | <pose_def>
               | <voice_def>
               | <sound_def>
               | <coaching_def>
               | <policy_def>
               | <mutation_def>
               | <ritual_def>
               | <query_def>

<rule_def> ::= "rule" STRING "{" <rule_body> "}"

<rule_body> ::= ("description" "=" STRING)?
              ("trigger" "{" <trigger> "}")?
              ("when" "{" <condition> "}")?
              ("then" "{" <action>+ "}")?
              ("metadata" "{" <metadata> "}")?

<trigger> ::= IDENT "(" <args> ")"
            | IDENT "." STRING "=" STRING

<condition> ::= <boolean_expr>

<action> ::= IDENT "(" <args> ")"
           | "do" IDENT "with" <args>

<boolean_expr> ::= <term> ("and" <term>)* | <term> ("or" <term>)*
<term> ::= "not" <term> | <primary>
<primary> ::= "(" <boolean_expr> ")" | FUNCTION_CALL | BOOLEAN_LITERAL

<args> ::= (IDENT "=" VALUE)* | (VALUE ("," VALUE)*)?
```

---

## Concrete Syntax Examples

### Rule Definition

```starlark
rule "deep-work-social-block" {
  description = "During deep work, block social apps for 120 minutes"
  id = "rule-dw-social-v1"
  
  trigger {
    type = "UserStartsSession"
    session_type = "focus"
  }
  
  when {
    time_in_range(start_hour=8, end_hour=16)
    and user_has_focus_mode()
    and not is_weekend()
  }
  
  then {
    enforce_policy("social-media-lockout", duration_minutes=120)
    emit_event("social_blocked", {"reason": "deep_work"})
  }
  
  metadata {
    author = "coaching-team"
    created = "2026-04-23"
    tags = ["focus", "social", "enforcement"]
  }
}
```

### Template Definition (Composition)

```starlark
template "school-weekday-focus" {
  description = "Focus coaching for school week (Mon–Fri, 8am–4pm)"
  
  # Inputs: allow customization when instantiating
  inputs {
    student_id = "string"
    reward_points = "number"
    default = 50
    focus_duration_min = "number"
    default = 90
  }
  
  # Rules to include
  rule "pomodoro-timer" {
    trigger { type = "UserStartsSession", session_type = "focus" }
    when { time_in_range(8, 16) and day_of_week(["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]) }
    then { 
      schedule_task("pomodoro_countdown", delay_ms=0, duration_ms=focus_duration_min*60*1000)
      show_notification("pomodoro_started", "Focus timer started: {focus_duration_min} minutes")
    }
  }
  
  rule "pomodoro-reward" {
    trigger { type = "EventFired", event_name = "pomodoro_completed" }
    when { user_has_earned_reward(student_id) }
    then { 
      apply_mutation("add_points", wallet="student_points", amount=reward_points)
      show_pose("cheerleader_happy", speech="Great focus session!")
    }
  }
}
```

### Connector Definition

```starlark
connector "canvas" {
  description = "Canvas LMS integration"
  kind = "oauth2"
  version = "1.0.0"
  
  auth {
    type = "oauth2"
    client_id = "${env.CANVAS_CLIENT_ID}"  # Environment interpolation
    scopes = ["accounts:read", "submissions:read", "assignments:read"]
  }
  
  endpoint "list_assignments" {
    method = "GET"
    path = "/api/v1/courses/{course_id}/assignments"
    
    params {
      course_id = { type = "string", required = true }
      per_page = { type = "number", default = 100 }
    }
    
    output = {
      type = "array",
      items = { type = "object" }
    }
  }
  
  endpoint "submit_grade" {
    method = "PUT"
    path = "/api/v1/courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}"
    
    params {
      course_id = { type = "string", required = true }
      assignment_id = { type = "string", required = true }
      user_id = { type = "string", required = true }
      grade = { type = "number", required = true }
    }
  }
}
```

### Task Definition

```starlark
task "log_deep_work_event" {
  description = "Log a deep work session to audit trail"
  
  inputs {
    session_id = { type = "string", required = true }
    duration_ms = { type = "number", required = true }
    apps_blocked = { type = "array", default = [] }
  }
  
  outputs {
    event_id = { type = "string" }
    timestamp = { type = "string" }
  }
  
  # Handler: built-in
  handler = "builtin:emit_event"
  handler_config {
    event_type = "deep_work_session"
    payload = {
      session_id = inputs.session_id,
      duration_ms = inputs.duration_ms,
      apps_blocked = inputs.apps_blocked,
      timestamp = now_iso8601(),
    }
  }
  
  timeout_ms = 5000
  
  retry {
    max_attempts = 3
    backoff_ms = 1000
  }
}
```

### Schedule Definition

```starlark
schedule "daily_morning_coaching" {
  description = "Daily coaching reminder at 8am"
  
  trigger {
    type = "daily_at"
    hour = 8
    minute = 0
  }
  
  timezone = "America/New_York"
  enabled = true
  
  # Fire these rules when the schedule triggers
  fire_rules = ["morning_focus_setup", "daily_goal_review"]
}

schedule "weekday_focus_blocks" {
  trigger {
    type = "weekly_at"
    day = "Monday"
    hour = 9
    minute = 0
  }
  
  timezone = "America/New_York"
  fire_rules = ["weekday_deep_work_rules"]
}
```

### Pose Definition (Mascot Visual State)

```starlark
pose "cheerleader_happy" {
  description = "Cheerleader mascot with thumbs up, happy expression"
  
  character = "cheerleader"
  pose = "thumbs_up"
  emotion = "happy"
  accessory = "pom_poms"
  
  speech_bubble {
    text = "You're crushing it! Keep up the great work!"
    alignment = "center"
    background = "cloud"
  }
  
  voice_cue = "voice_cheerleader_encouragement"
  sound_cue = "sound_success_chime"
  haptic_cue = "success_pulse"
  
  entry_animation {
    type = "slide_from_right"
    duration_ms = 300
    easing = "ease_out"
  }
  
  hold_duration_ms = 2000
  
  exit_animation {
    type = "fade_out"
    duration_ms = 200
  }
}
```

### Coaching Config Definition

```starlark
coaching_config "encouraging_mentor" {
  description = "Encouraging, supportive tone with mentor character"
  
  tone = "encouraging"
  language = "en"
  
  voice {
    voice_id = "voice_mentor_calm"
    speed = 1.0
    pitch = 1.0
    accent = "american"
  }
  
  text_templates {
    "notification_focus_started" = "Great! You're starting a focus session. I'll help you stay on track.",
    "notification_distraction_blocked" = "I've blocked distracting apps. Stay focused—you've got this!",
    "notification_reward_earned" = "Awesome work! You've earned {points} points for your dedication.",
  }
  
  notification_style = "banner"
}
```

### Enforcement Policy Definition

```starlark
policy "social-media-lockout" {
  description = "Enforce no social media access during focus sessions"
  
  targets = ["instagram", "tiktok", "twitter", "reddit", "snapchat"]
  
  threshold {
    type = "duration"
    max_ms = 120 * 60 * 1000  # 120 minutes
  }
  
  on_violation {
    action = "block_apps"
    message = "Social media is blocked during your focus session."
    fallback_action = "show_notification"
  }
  
  grace_period_ms = 30000  # 30 seconds before enforcement
}
```

### Wallet Mutation Definition

```starlark
mutation "reward_focus_session" {
  description = "Award points for completing a focus session"
  
  wallet_type = "student_points"
  operation = "add"
  amount = 50
  reason = "Completed a focus session"
  
  # Conditional: only award if session was >= 60 minutes
  when {
    session_duration_ms >= 60 * 60 * 1000
  }
}
```

### Ritual Definition (Habit Loop)

```starlark
ritual "focus_habit_loop" {
  description = "Habit loop for building a consistent focus routine"
  
  steps = [
    {
      sequence = 1
      name = "Setup"
      cue = "user_opens_focus_mode"
      routine = task("configure_focus_environment")
      reward = pose("thumbs_up_happy")
      estimated_duration_ms = 5000
    },
    {
      sequence = 2
      name = "Focus Work"
      cue = "pomodoro_timer_running"
      routine = task("block_distractions")
      estimated_duration_ms = 25 * 60 * 1000  # 25 minutes
    },
    {
      sequence = 3
      name = "Reward"
      cue = "pomodoro_completed"
      routine = task("apply_points_reward")
      reward = pose("cheerleader_happy")
      estimated_duration_ms = 3000
    },
  ]
  
  daily_goal = 4  # 4 focus cycles per day
  
  tracking {
    track_completion = true
    track_duration = true
    track_quality = false
  }
  
  rewards = [
    mutation("add_points_per_session", amount=50),
    mutation("bonus_points_daily_goal", amount=200, when={ daily_sessions >= 4 }),
  ]
}
```

### Sound Cue Definition

```starlark
sound "success_chime" {
  description = "Success notification sound"
  
  asset_url = "https://cdn.focalpoint.io/sounds/success-chime.m4a"
  asset_hash = "sha256:abc123def456..."
  duration_ms = 800
  volume_level = 0.8
  tags = ["positive", "reward"]
  usage = "reward"
}
```

### Audit Query Definition

```starlark
query "focus_sessions_by_date" {
  description = "Aggregate focus sessions per day for a student"
  
  event_filter {
    event_types = ["focus_session_completed"]
    when {
      user_id == target_user_id
    }
  }
  
  projections = ["user_id", "session_duration_ms", "timestamp"]
  
  aggregations = [
    { type = "count", field = null },
    { type = "sum", field = "session_duration_ms" },
  ]
  
  time_range {
    start = "2026-04-01T00:00:00Z"
    end = "2026-04-30T23:59:59Z"
  }
}
```

---

## Type System & Conversions

### Primitive Types

```starlark
# Starlark has: None, bool, int, float, string, list, dict, function

# FPL adds semantic types that map to IR:
@type("Trigger")      # Coerces to Trigger enum
@type("Condition")    # Coerces to Condition enum
@type("Action")       # Coerces to Action enum
@type("Task")         # Coerces to Task IR node
@type("Schedule")     # Coerces to Schedule IR node
@type("Pose")         # Coerces to Pose IR node
@type("Sound")        # Coerces to SoundCue IR node
@type("Policy")       # Coerces to EnforcementPolicy IR node
@type("Mutation")     # Coerces to WalletMutation IR node
@type("Ritual")       # Coerces to Ritual IR node
@type("Query")        # Coerces to AuditQuery IR node
```

### Runtime Type Checking

```starlark
# FPL compiler validates types:

rule "example" {
  trigger { type = "UserStartsSession", session_type = "focus" }  # ✓ Trigger type
  when { time_in_range(8, 16) }  # ✓ Condition type
  then { enforce_policy("policy_id") }  # ✓ Action type
}

# Type mismatch error:
rule "bad_example" {
  trigger { some_string = "hello" }  # ✗ Error: not a valid Trigger
}
```

### Conversions to IR

Each FPL construct is a **serializable mapping to IR**:

```rust
// In focus-lang compiler:
impl ToIR for FplRule {
  fn to_ir(&self) -> Result<Rule> {
    let trigger = self.trigger.to_ir()?;
    let conditions = self.conditions.iter().map(|c| c.to_ir()).collect::<Result<_>>()?;
    let actions = self.actions.iter().map(|a| a.to_ir()).collect::<Result<_>>()?;
    
    Ok(Rule {
      id: self.id.clone(),
      name: self.name.clone(),
      description: self.description.clone(),
      enabled: true,
      trigger,
      conditions,
      actions,
      metadata: self.metadata.to_ir()?,
      signature: None,  // Signature applied after compilation
    })
  }
}
```

---

## Composition & Module System

### Importing Templates

```starlark
import "templates/school-weekday-focus.fpl" as weekday_focus

# Instantiate the template with overrides
rule "monday_focus_rules" {
  include weekday_focus.rules
  
  # Override specific inputs
  override {
    focus_duration_min = 90
    reward_points = 75
  }
  
  # Add additional rules specific to this student
  rule "extra_morning_motivation" {
    trigger { type = "UserStartsSession", session_type = "focus" }
    then { show_pose("cheerleader_happy", text="Let's crush today's goals!") }
  }
}
```

### Conditional Rule Sets

```starlark
# Define rules conditionally based on user/student properties

rule "canvas_submission_reward" if has_connector("canvas") {
  description = "Only reward Canvas submissions if Canvas is configured"
  trigger { type = "EventFired", event_name = "canvas_assignment_submitted" }
  then { apply_mutation("add_points", amount=50) }
}

rule "slack_focus_announcement" if has_connector("slack") and get_coaching_config().tone == "encouraging" {
  description = "Post to Slack when student completes a focus session (if enabled)"
  trigger { type = "EventFired", event_name = "focus_session_completed" }
  then { call_connector("slack", "post_message", text="Great focus session!") }
}
```

### Helper Functions & Local Vars

```starlark
# Helper function (Starlark closures)
def focus_reward(points, bonus_multiplier=1.0):
  return points * bonus_multiplier

# Use in mutations
mutation "focus_reward_with_multiplier" {
  amount = focus_reward(50, bonus_multiplier=1.5)
}

# Local variables
def get_school_hours():
  return {
    "start": 8,
    "end": 16,
  }

hours = get_school_hours()

rule "school_hours_only" {
  when {
    time_in_range(start_hour=hours["start"], end_hour=hours["end"])
  }
}
```

---

## Testing FPL Files

### Inline Tests

```starlark
# In deep-work-social-block.fpl

rule "deep-work-social-block" {
  # ... rule definition ...
}

# Test module
@test
def test_rule_fires_during_focus() {
  rule = get_rule("deep-work-social-block")
  
  trigger_event = {
    "type": "UserStartsSession",
    "session_type": "focus",
    "time": "2026-04-23T10:00:00Z",  # 10am, within 8–16 range
    "is_weekend": False,
  }
  
  assert rule.matches(trigger_event) == True
}

@test
def test_rule_does_not_fire_on_weekend() {
  rule = get_rule("deep-work-social-block")
  
  trigger_event = {
    "type": "UserStartsSession",
    "session_type": "focus",
    "time": "2026-04-27T10:00:00Z",  # Saturday
    "is_weekend": True,
  }
  
  assert rule.matches(trigger_event) == False
}
```

### Sibling Test Files

Or use `.fpl.test` companion files for larger test suites:

```starlark
# deep-work-social-block.fpl.test

load("deep-work-social-block.fpl", "rule")

test_suite = "deep-work-social-block"

def test_rule_fires_during_focus():
  assert rule.matches({ ... })

def test_rule_does_not_fire_on_weekend():
  assert not rule.matches({ ... })

def test_actions_are_applied():
  result = rule.apply({ ... })
  assert "social-media-lockout" in result.actions
```

### Running Tests

```bash
$ focus lang test
Running tests in rules/deep-work-social-block.fpl...
  ✓ test_rule_fires_during_focus
  ✓ test_rule_does_not_fire_on_weekend
  ✓ test_actions_are_applied

3/3 tests passed (42ms)

$ focus lang test --coverage
Coverage: 92% (23/25 rules have tests)
```

---

## Error Handling & Diagnostics

### Compilation Errors with Source Positions

```
Error: Type mismatch in rule "bad-rule" (rules/example.fpl:5:3)
  Expected: Condition
  Got: string "some_string"
  
  5 |    when { some_string = "hello" }
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^
  
  Hint: Did you mean to call a condition function? E.g., user_attribute("some_string", "hello")
```

### Warning: Unused Rules

```
Warning: Rule "old_rule" is defined but never fired (rules/unused.fpl:10:1)
  Possible causes:
  - Rule is disabled by condition
  - No trigger matches expected events
  - Rule was deprecated and should be removed
  
  Hint: Remove or enable this rule.
```

---

## Toolchain

### FPL Compiler

```bash
# Compile FPL → IR JSON
$ focus lang build rules/deep-work-social-block.fpl
Output: /tmp/deep-work-social-block.ir.json (1.2 KB)
Hash: sha256:abc123def456...

# Check FPL syntax without compiling
$ focus lang check
Checking 12 .fpl files...
  ✓ rules/deep-work-social-block.fpl
  ✓ rules/pomodoro-reward.fpl
  ✗ templates/broken-template.fpl:8: Syntax error
All checks: 11/12 passed

# Format FPL (like rustfmt)
$ focus lang fmt --write rules/*.fpl
Formatted 12 files

# Pretty-print IR (human-readable)
$ focus lang explain rules/deep-work-social-block.fpl
Rule: deep-work-social-block
  ID: rule-dw-social-v1
  Trigger: UserStartsSession { session_type: "focus" }
  Conditions:
    - time_in_range(8, 16)
    - user_has_focus_mode()
    - not is_weekend()
  Actions:
    - enforce_policy("social-media-lockout", duration_minutes=120)
    - emit_event("social_blocked", ...)
```

### VS Code Extension Skeleton

```json
{
  "name": "focalpoint-lang",
  "publisher": "focalpoint",
  "version": "0.1.0",
  "engines": {
    "vscode": "^1.75.0"
  },
  "activationEvents": [
    "onLanguage:focalpoint"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "languages": [
      {
        "id": "focalpoint",
        "aliases": [
          "FocalPoint",
          "fpl"
        ],
        "extensions": [
          ".fpl"
        ],
        "configuration": "./language-configuration.json"
      }
    ],
    "grammars": [
      {
        "language": "focalpoint",
        "scopeName": "source.fpl",
        "path": "./syntaxes/focalpoint.tmLanguage.json"
      }
    ]
  }
}
```

### Tree-Sitter Grammar Scaffold

```scheme
; tree-sitter grammar for FPL
(program
  (imports) @import
  (definitions) @definition
)

(rule_definition
  "rule" @keyword
  name: (string) @entity.name.function
  body: (rule_body) @meta.block
)

(rule_body
  "description"? @keyword
  "trigger"? @keyword
  "when"? @keyword
  "then"? @keyword
  "metadata"? @keyword
)

(trigger
  (identifier) @function
  (arguments)? @parameters
)

(condition
  (boolean_expression) @expression
)
```

### LSP Server

```rust
// focus-lang-lsp crate
use tower_lsp::*;

#[tokio::main]
async fn main() {
  let stdin = tokio::io::stdin();
  let stdout = tokio::io::stdout();
  
  let (service, messages) = LspService::new(|client| {
    Backend { client }
  });
  
  Server::new(stdin, stdout, service).run(messages).await;
}

struct Backend {
  client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
  async fn did_open(&self, params: DidOpenTextDocumentParams) {
    // Parse FPL, collect diagnostics
  }
  
  async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
    // Suggest keywords, rule names, connectors, etc.
    Ok(Some(CompletionResponse::Array(vec![
      CompletionItem {
        label: "rule".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
      },
      // ... more completions
    ])))
  }
  
  async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
    // Show type info, documentation for hovered symbol
  }
}
```

---

## Starlark Sandboxing

FPL runs Starlark in a **restricted environment**:

```rust
use starlark::environment::Globals;
use starlark::syntax::{AstModule, Dialect};

fn create_fpl_globals() -> Globals {
  let mut globals = Globals::new();
  
  // Allow: basic data types, operators
  // Block: file I/O, network, subprocess, sys
  
  globals.set("rule", builtin_rule);
  globals.set("connector", builtin_connector);
  globals.set("task", builtin_task);
  // ... more builtins
  
  // Block dangerous builtins
  // globals.remove("open");
  // globals.remove("exec");
  
  globals
}

pub fn compile_fpl_with_sandbox(source: &str) -> Result<Vec<Document>> {
  let ast = AstModule::parse("rule.fpl", source, &Dialect::Standard)?;
  
  let globals = create_fpl_globals();
  let mut starlark_vm = Evaluator::new(&ast, &globals)?;
  
  // Evaluate in restricted context; no I/O side effects
  starlark_vm.eval_module()?;
  
  // Extract IR documents from evaluation
  let rules = starlark_vm.module.exports();
  // Convert to IR...
  
  Ok(ir_docs)
}
```

---

## File Organization

```
focalpoint.fpl.toml          # Workspace manifest

rules/
  deep-work-social-block.fpl
  deep-work-social-block.fpl.test
  pomodoro-reward.fpl

templates/
  school-weekday-focus.fpl
  high-school-focus.fpl

connectors/
  canvas.fpl
  slack.fpl

tasks/
  log_event.fpl
  send_notification.fpl

schedules/
  daily_morning.fpl

poses/
  cheerleader_happy.fpl
  mentor_thinking.fpl

coaching/
  encouraging_mentor.fpl

policies/
  social_media_lockout.fpl

mutations/
  reward_focus_session.fpl

rituals/
  focus_habit_loop.fpl

sounds/
  success_chime.fpl

queries/
  focus_sessions_by_date.fpl
```

---

## Performance & Constraints

- **Compile time**: <200 ms for 100 rules on M-series laptop
- **FPL file size**: 200–1000 bytes per rule (average 500 bytes)
- **Total workspace**: <1 MB FPL source
- **Starlark runtime**: <10 ms to evaluate a 500-line module
- **Memory**: <50 MB for a full workspace evaluation

---

## Summary

**FPL (FocalPoint Language)** is:

- **Syntax**: Starlark (Python subset, sandboxed, proven at scale)
- **Structure**: Blocks for rule, connector, template, task, schedule, pose, etc.
- **Composition**: Imports, templates with inputs, conditional rules
- **Testing**: Inline `@test` functions or `.fpl.test` companion files
- **Errors**: Source-position diagnostics with helpful hints
- **Toolchain**: Compiler (build, check, fmt), VS Code extension, LSP, Tree-Sitter grammar
- **Sandboxing**: Starlark runtime with no filesystem/network access by default
- **File extension**: `.fpl` (or `.fpl.toml` for manifests, `.fpl.test` for tests)

Developers write `.fpl` files, version-control them in git, and the FPL compiler produces IR JSON that all four surfaces consume.
