# Rule Builder: Reverse Code-Generation & Export Formats

The FocalPoint Rule Builder now supports exporting rules in **three formats**, each suited to different authoring workflows. Switch between visual design and text-mode editing with guaranteed round-trip fidelity.

## Three Export Formats

### 1. IR JSON (Canonical)

**Format:** `ir.json`  
**Language:** JSON  
**Use when:** You need the authoritative, content-addressed representation for storage, versioning, or API uploads.

```json
{
  "version": 1,
  "kind": "Rule",
  "id": "rule-001",
  "name": "Focus Lock",
  "body": {
    "kind": "Rule",
    "id": "rule-001",
    "name": "Focus Lock",
    "trigger": {
      "type": "UserStartsSession",
      "value": { "session_type": "deep_work" }
    },
    "conditions": [
      {
        "op": "time_in_range",
        "start_hour": 9,
        "end_hour": 17
      }
    ],
    "actions": [
      {
        "type": "enforce_policy",
        "policy_id": "focus_lock",
        "params": {}
      }
    ],
    "priority": 10,
    "cooldown_seconds": 300,
    "duration_seconds": 3600,
    "enabled": true
  }
}
```

**Properties:**
- **Content-addressed:** SHA-256 hash of canonical JSON is deterministic; same rule = same hash.
- **Versioned:** `version` field tracks schema evolution.
- **Immutable:** Use in `FocalPoint` core, APIs, and event stores.

---

### 2. FPL (focus-lang DSL)

**Format:** `rule.fpl`  
**Language:** Starlark-inspired DSL  
**Use when:** You prefer text-mode authoring, want to share rules in a human-friendly format, or need to integrate with CI/CD pipelines.

```starlark
rule("Focus Lock") {
  priority = 10
  cooldown_seconds = 300
  duration_seconds = 3600
  enabled = true

  trigger {
    type = "UserStartsSession"
    session_type = "deep_work"
  }

  when {
    time_in_range(start_hour = 9, end_hour = 17)
  }

  then {
    enforce_policy(policy_id = "focus_lock")
  }
}
```

**Properties:**
- **Human-readable:** Comments, indentation, familiar syntax.
- **Editable:** Switch from builder UI to text editor mid-workflow.
- **Round-trip:** FPL → parse → IR → serialize → identical IR hash.

**Syntax Guide:**

| Concept | Syntax |
|---------|--------|
| Trigger | `trigger { type = "...", ...fields }` |
| Conditions (AND/OR) | `when { and { ... } or { ... } not { ... } }` |
| Simple condition | `when { time_in_range(start_hour = 9, end_hour = 17) }` |
| Action | `then { enforce_policy(policy_id = "...") }` |
| Nested sequences | `trigger_sequence { action(...) action(...) }` |

---

### 3. CLI Command

**Format:** `add-rule.sh`  
**Language:** POSIX shell  
**Use when:** You need to integrate rules into automation, deploy via CI/CD, or bulk-import from scripts.

```bash
focus rules add \
  --name 'Focus Lock' \
  --id 'rule-001' \
  --priority 10 \
  --enabled true \
  --cooldown 300 \
  --duration 3600 \
  --trigger '{"type":"UserStartsSession","value":{"session_type":"deep_work"}}' \
  --conditions '[{"op":"time_in_range","start_hour":9,"end_hour":17}]' \
  --actions '[{"type":"enforce_policy","policy_id":"focus_lock","params":{}}]'
```

**Properties:**
- **Executable:** Run directly in shell scripts or CI pipelines.
- **POSIX-escaped:** All arguments properly quoted for shell safety.
- **Atomic:** Each rule is a single invocation; no state required.

**Common Patterns:**

Save to a script and execute:
```bash
./add-rule.sh
```

Pipe multiple rules:
```bash
cat rule-1.sh rule-2.sh rule-3.sh | bash
```

Integrate with CI/CD:
```bash
for rule in ./rules/*.sh; do
  bash "$rule" || exit 1
done
```

---

## Round-Trip Guarantee

All three formats support **lossless conversion**:

```
Builder UI (nodes/edges)
    ↓
    → graphToIR() → IR JSON
      ↓
      → irToFpl() → FPL text
      ↓
      → fplParser() → IR JSON
        ↓ (hashes match)
```

**Verification:** Export the same rule in all three formats. The IR JSON hash is identical regardless of source.

---

## Authoring Workflows

### Workflow 1: Visual Design → Export

1. **Build rule in UI** (drag triggers, conditions, actions)
2. **Click "Export ▼"** → select format
3. **Download file** (`.json` / `.fpl` / `.sh`)
4. **Deploy or share** (upload, commit, or run)

### Workflow 2: Text Mode → Visual

1. **Write FPL rule** (in your editor)
2. **Import via Builder** (File → Import JSON; convert FPL → IR first)
3. **Visualize in UI** (nodes appear on canvas)
4. **Refine and re-export**

### Workflow 3: CLI Automation

1. **Generate rules from IR** (`irToCli()`)
2. **Embed in shell scripts** or Terraform modules
3. **Version in Git** (`.sh` files)
4. **CI/CD applies rules** (`bash add-rule.sh`)

---

## Integration Examples

### Python: Load IR and Render Formats

```python
import json
from pathlib import Path

ir = json.load(open('rule-ir.json'))
rule = ir['body']

# Call Rust codegen (or TS equivalent)
from focalpoint import ir_to_fpl, ir_to_cli
fpl = ir_to_fpl(rule)
cli = ir_to_cli(rule)

print(fpl)  # → Starlark
print(cli)  # → shell command
```

### Rust: Generate FPL from Code

```rust
use focus_ir::{RuleIr, TriggerIr, ActionIr, codegen};
use std::collections::BTreeMap;

let rule = RuleIr {
    id: "rule-001".to_string(),
    name: "Focus Lock".to_string(),
    trigger: TriggerIr::UserStartsSession {
        session_type: "deep_work".to_string(),
    },
    actions: vec![ActionIr::EnforcePolicy {
        policy_id: "focus_lock".to_string(),
        params: BTreeMap::new(),
    }],
    // ... other fields
};

let fpl = codegen::ir_to_fpl(&rule);
println!("{}", fpl);  // → Starlark
```

### TypeScript: UI Export Button

```typescript
import { irToFpl, irToCli } from './lib/irToFpl';
import { irToCli } from './lib/irToCli';

const ir = graphToIR(nodes, edges);

// Export menu
<button onClick={() => downloadTextFile(irToFpl(ir), 'rule.fpl')}>
  Export FPL
</button>

<button onClick={() => downloadTextFile(irToCli(ir), 'add-rule.sh')}>
  Export CLI
</button>
```

---

## Best Practices

### For Rule Design Teams

- **Check in FPL files** to version control (human-readable diffs).
- **Use IR JSON** for APIs and data exchange (canonical, versioned).
- **Auto-generate CLI** in CI/CD (executable, auditable).

### For Single Authors

- **Visual builder** for rapid prototyping.
- **Export FPL** when you want to edit by hand or share with reviewers.
- **Re-import FPL** to visualize changes.

### For Automation

- **Generate IR programmatically** (from configs or business logic).
- **Render as FPL or CLI** for deployment.
- **Log all three formats** for audit trails.

---

## FAQ

**Q: Can I switch between formats mid-workflow?**  
A: Yes. Export to FPL, edit in your favorite editor, import back to the builder. The IR hash remains stable.

**Q: What if I edit FPL by hand?**  
A: Parse it back to IR (via `fplParser()`). The builder will re-render your edits on canvas.

**Q: Are all three formats always in sync?**  
A: Yes, by design. The IR is the source of truth; FPL and CLI are pure functions of IR.

**Q: How do I version rules?**  
A: Commit the IR JSON to Git (or your chosen VCS). IR hashes provide content-addressed versioning.

**Q: Can I merge rules?**  
A: Use standard Git merge on FPL files (readable diffs). For conflicts, resolve in FPL text, then re-import.

---

## Supported Triggers

| Trigger | FPL | CLI | Notes |
|---------|-----|-----|-------|
| `UserStartsSession` | ✓ | ✓ | Session type (e.g., "deep_work") |
| `EventFired` | ✓ | ✓ | Event name |
| `TimeElapsed` | ✓ | ✓ | Duration in milliseconds |
| `ScheduleCron` | ✓ | ✓ | Cron expression + timezone |
| `WebhookReceived` | ✓ | ✓ | Path + HTTP method |
| `UserAction` | ✓ | ✓ | Action type + target |

## Supported Conditions

| Condition | FPL | CLI | Notes |
|-----------|-----|-----|-------|
| `and` | ✓ | ✓ | Logical AND |
| `or` | ✓ | ✓ | Logical OR |
| `not` | ✓ | ✓ | Logical NOT |
| `time_in_range` | ✓ | ✓ | Start/end hour (0-23) |
| `day_of_week` | ✓ | ✓ | Days list |
| `user_attribute` | ✓ | ✓ | Key-value match |
| `event_property` | ✓ | ✓ | JSON property match |
| `custom_predicate` | ✓ | ✓ | Name + args |

## Supported Actions

| Action | FPL | CLI | Notes |
|--------|-----|-----|-------|
| `enforce_policy` | ✓ | ✓ | Policy ID + optional params |
| `emit_event` | ✓ | ✓ | Event type + payload |
| `apply_mutation` | ✓ | ✓ | Mutation ID + params |
| `schedule_task` | ✓ | ✓ | Task ID + delay (ms) |
| `trigger_sequence` | ✓ | ✓ | Nested action list |
| `show_notification` | ✓ | ✓ | Notification ID + text + duration |
