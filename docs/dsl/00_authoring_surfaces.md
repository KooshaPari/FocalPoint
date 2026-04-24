# FocalPoint Authoring Surfaces

## Philosophy

FocalPoint primitives (Rules, Connectors, Tasks, Schedules, Mascot Poses, Coaching Configs, Enforcement Policies, Wallet Mutations, Rituals, Sound Cues, Audit Queries) deserve **four coequal authoring surfaces**, not a GUI-first with CLI-as-afterthought design. Each surface serves a different user:

- **Scripting DSL (FPL)**: Developers, version control, CI/CD, bulk authoring
- **Visual Node Builder**: Designers, educators, visual thinkers, flow clarity
- **CLI/API**: Automation, programmatic generation, headless deployments
- **GUI Wizard**: Educators, first-time users, step-by-step guidance

**Zero privileging, full round-tripping**: Users should be able to:
1. Start in the GUI wizard to learn the shape
2. Click "Show me the DSL" and copy the generated `.fpl` file
3. Version-control it in git, edit in VS Code
4. Execute from `focus lang apply ./rule.fpl`
5. Re-import to the visual builder
6. All four surfaces compile to **identical IR bytes** (content-addressable, hashable for signing)

This mirrors successful systems: **Unreal Engine's Blueprint ↔ C++ parity, Unity ShaderGraph ↔ HLSL round-trip, Shortcuts.app live preview, Terraform HCL ↔ Cloud UI equivalence**. No surface should be "locked in"—users should be able to migrate between them without data loss or hidden conversion artifacts.

---

## Primitive × Surface Matrix (48-cell status grid)

| Primitive | DSL (FPL) | Node Builder | CLI/API | GUI Wizard | Canonical UX |
|-----------|-----------|--------------|---------|------------|--------------|
| **Rules** | ✅ First-class | ✅ Graph (Trigger→Condition→Action) | ✅ Full CRUD | ✅ Step 1: When, Step 2: Then | DSL-first; show graph on demand |
| **Connectors** | ✅ `connector` block | ✅ Config node + port list | ✅ Manifest parser | ✅ OAuth/API key wizard | DSL-first; node visualizes schema |
| **Templates** | ✅ Reusable `template` def | ✅ Template library palette | ✅ Instantiate + override | ✅ Select from gallery, customize fields | DSL; composition via `include` |
| **Tasks** | ✅ `task` block | ✅ Action node (inputs/outputs) | ✅ Register in task registry | ✅ Select action, bind params | DSL; node shows param panel |
| **Schedules** | ✅ Cron + temporal defs | ✅ Trigger node (cron editor) | ✅ Cron string + tz | ✅ "Every weekday 8am EST" wizard | DSL (cron); node has picker |
| **Mascot Poses** | ✅ `pose` block + JSON | ✅ Pose editor canvas (preview) | ✅ Pose registry JSON | ✅ Drag character, set emotion+bubble | DSL; node shows live preview |
| **Coaching Config** | ✅ `coaching` block | ✅ Config panels (audio/image/text) | ✅ Structured JSON config | ✅ Form wizard (tone, language, style) | DSL; node shows rendered preview |
| **Enforcement Policies** | ✅ `policy` block | ✅ Policy config node | ✅ Policy CRUD + eval | ✅ Threshold + action wizard | DSL; node shows enforcement rules |
| **Wallet Mutations** | ✅ `mutation` block | ✅ Mutation node (input→output) | ✅ Mutation applier | ✅ "Reward/deduct points" dialog | DSL; node shows balance impact |
| **Rituals** | ✅ `ritual` block | ✅ Ritual timeline (sequence viz) | ✅ Ritual plan + steps | ✅ Define habit loop (cue→routine→reward) | DSL; node shows habit sequence |
| **Sound Cues** | ✅ `sound` ref + metadata | ✅ Audio node (waveform preview) | ✅ Sound library query | ✅ Upload/select audio + trim | DSL (ref); node shows preview |
| **Audit Queries** | ✅ `query` block | ✅ Query builder (column picker) | ✅ Query executor | ✅ Select event type + filters | DSL; node shows result schema |

---

## Example: Deep-Work-Social-Block Rule (All Four Surfaces)

### Surface 1: Scripting DSL (FPL)
```fpl
rule "deep-work-social-block" {
  description = "During deep work, block social apps"
  id = "rule-dw-social-v1"
  
  trigger {
    event = "user_starts_session"
    session_type = "focus"
  }
  
  condition {
    time_in_range(start_hour=8, end_hour=16)
    and user.has_focus_mode_active()
    and not is_weekend()
  }
  
  action {
    do enforce_policy("social-media-lockout")
    with duration_minutes = 120
    and notification = "social-blocked-during-focus"
  }
  
  metadata {
    author = "coaching-team"
    created = "2026-04-23"
    version = "1.0"
  }
}
```

### Surface 2: Node Builder (ReactFlow Graph)
```
[Trigger: user_starts_session]
  ↓ edge: "focus"
[Condition: time_in_range(8am–4pm)]
  ↓ edge: "AND"
[Condition: user.has_focus_mode_active()]
  ↓ edge: "AND"
[Condition: not is_weekend()]
  ↓ edge: "PASS"
[Action: enforce_policy("social-media-lockout")]
  • duration_minutes: 120 (input port)
  • notification: "social-blocked-during-focus" (input port)
  ↓ edge: "success"
[Outcome: rule_fired]

Minimap shows full rule shape. Right-click on Action node → "Show the DSL" toggles side panel with live FPL source. Every node position is stored in the .fpl file as `@layout` metadata (preserves UX across round-trips).
```

### Surface 3: CLI/API
```bash
# Command-line instantiation
$ focus rules create \
  --name "deep-work-social-block" \
  --trigger-event "user_starts_session" \
  --trigger-value "focus" \
  --condition "time_in_range(8,16)" \
  --condition "user.has_focus_mode_active()" \
  --condition "not is_weekend()" \
  --action "enforce_policy" \
  --action-param "policy_id=social-media-lockout" \
  --action-param "duration_minutes=120" \
  --action-param "notification=social-blocked-during-focus"

# Or: programmatic Rust API
let rule = Rule::builder()
  .name("deep-work-social-block")
  .trigger(Trigger::UserStartsSession { session_type: "focus" })
  .condition(Condition::TimeInRange { start_hour: 8, end_hour: 16 })
  .condition(Condition::UserFocusModeActive)
  .condition(Condition::NotWeekend)
  .action(Action::EnforcePolicy {
    policy_id: "social-media-lockout",
    duration_minutes: 120,
    notification: "social-blocked-during-focus",
  })
  .build()?;

focus_engine.apply_rule(rule).await?;
```

### Surface 4: GUI Wizard (Step-by-Step Form)
```
Step 1: "What triggers this rule?"
  Radio: "User starts a session" ✓
  Dropdown: "Focus session" ✓

Step 2: "When should it apply? (All must be true)"
  ☑ Time of day: 8:00 AM to 4:00 PM (EST)
  ☑ User has focus mode active
  ☑ Not a weekend

Step 3: "What should happen?"
  Action: "Enforce a policy"
    Policy: "Social Media Lockout" (dropdown)
    Duration: 120 minutes (slider: 1–480)
    Notification: "social-blocked-during-focus" (preset)

Step 4: "Review & Name"
  Name: "deep-work-social-block"
  Description: "During deep work, block social apps"
  [Save & Apply] [Show me the DSL] [Preview graph]
```

---

## Parity Principle: Zero-Loss Round-Trip

Every authoring surface must guarantee **byte-identical IR output** (modulo sorting). Test suite:

```rust
#[test]
fn test_roundtrip_fpl_to_ir_to_fpl() {
  let original = read_file("deep-work-social-block.fpl");
  let ir = FplCompiler::compile(&original)?;
  let regenerated = IrFormatter::to_fpl(&ir)?;
  let ir2 = FplCompiler::compile(&regenerated)?;
  
  assert_eq!(ir.content_hash(), ir2.content_hash());
  // Byte-level diff only in formatter whitespace, not semantics
}

#[test]
fn test_roundtrip_graph_to_ir() {
  let graph = load_graph_from_file("deep-work-social-block.graph.json");
  let ir = GraphCompiler::compile(&graph)?;
  let graph2 = IrFormatter::to_graph(&ir)?;
  let ir2 = GraphCompiler::compile(&graph2)?;
  
  assert_eq!(ir.content_hash(), ir2.content_hash());
}

#[test]
fn test_roundtrip_api_to_ir() {
  let api_calls = vec![
    RuleApi::create_rule("deep-work-social-block"),
    RuleApi::add_trigger(...),
    // ... more API calls ...
  ];
  let ir = ApiCompiler::compile(&api_calls)?;
  
  let api_calls2 = IrFormatter::to_api_calls(&ir)?;
  let ir2 = ApiCompiler::compile(&api_calls2)?;
  
  assert_eq!(ir.content_hash(), ir2.content_hash());
}
```

---

## Reference Inspirations (Why Four Surfaces Work)

### 1. **Shortcuts.app (iOS/macOS)**
Shortcuts combines visual blocks (surface 3) with a script export (surface 1). Users can: build visually, "Share → View as Text," share the shortcut code, and re-import. Round-trip tested and lossless. iOS automation is accessible to non-programmers (GUI first) but powerful for developers (script-native).

### 2. **Unreal Engine's Blueprint ↔ C++**
Blueprints (visual node graph) and C++ (imperative code) compile to the same intermediate bytecode. Developers can toggle between them in the editor. Unreal's "Blueprintable" pattern lets you wire nodes and drop into C++ for performance-critical sections—no data loss either way.

### 3. **Unity ShaderGraph ↔ HLSL**
ShaderGraph is a visual node editor for writing GPU shaders. It auto-generates HLSL code. Artists use the GUI; engineers inspect/optimize the generated shader. Round-trip works because ShaderGraph → HLSL is deterministic and lossless; hand-edited HLSL can be re-imported.

### 4. **n8n Workflow JSON ↔ Visual Editor**
n8n workflows are JSON (surface 1) or a visual canvas (surface 2). You can export a workflow as JSON, version-control it, edit it programmatically, and re-import. The JSON is the source of truth; the canvas is a view of it.

### 5. **Hugo (Config.toml ↔ Web UI)**
Hugo is config-first (TOML). The Hugo UI (surfaces 3 & 4) generates the same TOML you'd hand-write. Developers version-control `config.toml`; non-technical users use the UI.

### 6. **Terraform HCL ↔ Terraform Cloud UI**
Terraform Cloud's UI generates HCL (or consumes it). Teams can use the web UI for planning, but the HCL is the reproducible, version-controllable source. Round-trip: HCL → Cloud Plan → Cloud UI → HCL is seamless.

### 7. **GitHub Actions YAML ↔ Workflow Visualizer**
GitHub Actions workflows are YAML (surface 1). GitHub recently added a visual workflow editor that generates the same YAML. Users can edit YAML in their editor or use the visual UI; both compile to identical action runs.

---

## Design Principles for Implementation

### Principle 1: DSL-First Architecture
The **scripting DSL is the canonical source**. All surfaces compile to the same IR; the IR is what the engine consumes. This ensures:
- Version control works (git diffs on `.fpl` files are human-readable)
- Diffing is possible (git diff between two rule versions shows what changed)
- Signing/integrity is deterministic (IR hash is stable)
- Performance is predictable (one compiler path, optimized)

### Principle 2: Node Builder = IR Visualization
The node builder is not an alternative editor; it's a **visual projection of the IR**. Every graph node maps 1:1 to IR terms. Editing in the graph updates the IR; saving the graph re-serializes to FPL.

### Principle 3: CLI/API = Programmatic IR Builder
The Rust API and CLI are **ergonomic wrappers around IR construction**. They validate, transform, and emit IR, not some parallel format. This keeps them synchronized.

### Principle 4: GUI Wizard = Guided IR Authoring
The step-by-step wizard guides users through **canonical IR fields in a safe order**. It prevents invalid IR (e.g., actions without triggers). It generates IR, not some "simple user format" that later gets translated.

### Principle 5: Layout Persistence
Node positions are **stored in FPL as metadata comments**. Round-tripping preserves layout:
```fpl
rule "deep-work-social-block" {
  @layout { x = 100, y = 50 }
  // ... rule body ...
}
```

This ensures the graph-to-FPL-to-graph cycle preserves the UX.

---

## Adoption Order (by Primitive)

### Phase 1: Rules (Smoke Test)
- FPL syntax + compiler
- Node builder (Trigger → Condition → Action graph)
- CLI: `focus rules create`, `focus rules apply`
- GUI wizard (4-step form)
- Transpilers: FPL ↔ IR, Graph ↔ IR, CLI ↔ IR, Wizard ↔ IR
- Round-trip tests (all 4 surfaces)

### Phase 2: Connectors, Templates, Tasks
- FPL `connector`, `template`, `task` blocks
- Node builder nodes for each
- CLI CRUD for each
- GUI forms for each

### Phase 3: Schedules, Poses, Coaching Config
- Temporal DSL for schedules
- Visual pose editor (canvas preview)
- Audio/image/text coaching config forms

### Phase 4: Enforcement Policies, Mutations, Rituals
- Policy expression language
- Wallet visualization (balance impact)
- Habit loop timeline

### Phase 5: Sound Cues, Audit Queries
- Audio library + search
- Query builder (column/filter picker)

---

## Maintenance: Keeping Four Surfaces Synced

**Problem**: As you add features, all four surfaces must stay in sync. Missing a surface breaks parity.

**Solution**: Use **generative testing + specification-driven architecture**.

1. **Single IR schema** (Rust enums, serde)
   - Derive JSON Schema, TypeScript, Swift types
   - All surfaces read/write IR types, not parallel formats

2. **Compiler tower** (FPL → IR, Graph → IR, API → IR, Wizard → IR)
   - Each surface has one entry point: `compile(surface_input) → IR`
   - IR is the single source of truth

3. **Property-based tests** (with Proptest)
   ```rust
   proptest! {
     #[test]
     fn prop_roundtrip_all_surfaces(rule in Rule::arb()) {
       let ir = rule.to_ir();
       
       // FPL round-trip
       let fpl = ir.to_fpl();
       let ir1 = fpl_compile(&fpl)?;
       
       // Graph round-trip
       let graph = ir.to_graph();
       let ir2 = graph_compile(&graph)?;
       
       // API round-trip
       let api = ir.to_api_calls();
       let ir3 = api_compile(&api)?;
       
       assert_eq!(ir.hash(), ir1.hash());
       assert_eq!(ir.hash(), ir2.hash());
       assert_eq!(ir.hash(), ir3.hash());
     }
   }
   ```

4. **Surface-agnostic validation**
   - Validation logic lives in IR validation, not in each surface
   - Surfaces delegate: `surface_input → IR → validate → error or OK`

---

## File Structure

```
focalpoint.fpl.toml          # Workspace manifest (entry points, version)
rules/
  deep-work-social-block.fpl # FPL source
  example-pomodoro.fpl
connectors/
  slack.fpl
  canvas.fpl
templates/
  school-weekday-focus.fpl
tasks/
  log-event.fpl
  ...
```

---

## Security & Isolation

- **FPL sandboxing**: Starlark runtime, no filesystem/network access
- **Surface-agnostic auth**: Each surface validates against the same access control list (who can edit which rules)
- **Signature preservation**: IR docs are signed with ed25519; all surfaces must preserve signatures on round-trip (or regenerate with new signature)

---

## Size Target

- Rules: 100–500 rules in production (each 200–1000 bytes FPL)
- Connectors: 5–20 connectors (each 500–2000 bytes FPL)
- Templates: 10–50 templates (each 300–1500 bytes FPL)
- Total workspace: <1 MB FPL
- Compile time: <200 ms full workspace on M-series laptop, <1 s on iPhone

---

## Summary

FocalPoint users deserve **all four surfaces equally**. The architecture is simple:

1. **One IR schema** (Rust enums)
2. **Four compilers** (FPL, Graph, API, Wizard → IR)
3. **Property-based round-trip tests** (all surfaces produce identical IR)
4. **DSL-first** (FPL is human-readable, version-controllable source)

This mirrors real-world success (Shortcuts, Unreal, Unity, Terraform, n8n, GitHub Actions). Implementation is straightforward once IR is defined and one surface (FPL) works—the rest are "just" compilers to/from the same IR.
