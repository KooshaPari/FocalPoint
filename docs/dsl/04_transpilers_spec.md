# FocalPoint Transpilers Specification

## Overview

A transpiler is a **lossless, round-trip compiler** between any two authoring surfaces. The canonical format is the **IR** (JSON). All transpilers feed into and out of the IR.

```
FPL ←→ IR ←→ Graph ←→ Wizard ←→ CLI ←→ TOML (legacy)
 ↓      ↓      ↓      ↓      ↓      ↓
all round-trip to IR preserving byte-identical semantics
```

---

## Transpiler Matrix (12 Pairs)

| # | From | To | Crate | Lossless? | Notes |
|---|------|----|-------|-----------|-------|
| 1 | FPL | IR | focus-lang-compiler | ✅ Yes | Deterministic hashing |
| 2 | IR | FPL | focus-lang-formatter | ✅ Yes | Format-stable output |
| 3 | Graph (JSON) | IR | focus-graph-compiler | ✅ Yes (with layout metadata) | Layout → `@layout` comments |
| 4 | IR | Graph (JSON) | focus-ir-to-graph | ✅ Yes | IR → canonical positions |
| 5 | CLI args | IR | focus-cli-builder | ✅ Yes | Serialize CLI flags → IR |
| 6 | IR | CLI args | focus-ir-to-cli | ⚠️ Partial | Lossy: can't express all IR in flags |
| 7 | Wizard form state | IR | focus-wizard-compiler | ✅ Yes | Form inputs → IR |
| 8 | IR | Wizard form state | focus-ir-to-wizard | ✅ Yes | IR → form fields |
| 9 | TOML (legacy) | IR | focus-toml-importer | ✅ Yes (migration) | v1 template-packs → IR |
| 10 | IR | TOML | focus-ir-to-toml | ⚠️ Partial | Some IR fields not expressible in TOML |
| 11 | IR | TypeScript types | focus-codegen-ts | ✅ Yes | Deterministic codegen |
| 12 | IR | Swift types | focus-codegen-swift | ✅ Yes | Deterministic codegen |

---

## Transpiler Specifications (6 Core Pairs)

### Pair 1: FPL ↔ IR

**Transpiler**: `focus-lang-compiler` and `focus-lang-formatter`

#### FPL → IR (Compiler)

```rust
pub fn compile_fpl(source: &str) -> Result<Vec<Document>> {
  // 1. Lex and parse FPL (Starlark syntax)
  let ast = parse_fpl_ast(source)?;
  
  // 2. Validate semantics (type-check, name resolution)
  let validated = validate_fpl(&ast)?;
  
  // 3. Lower to IR
  let ir_docs = lower_to_ir(&validated)?;
  
  // 4. Sign documents
  for doc in &mut ir_docs {
    doc.signature = Some(sign_document(doc, &signing_key)?);
  }
  
  Ok(ir_docs)
}
```

**Lossless guarantee**: Every FPL construct maps uniquely to an IR variant. No data is discarded.

**Test suite**:

```rust
#[cfg(test)]
mod tests {
  use crate::*;
  
  #[test]
  fn test_fpl_ir_roundtrip_rule() {
    let fpl = r#"
      rule "example" {
        trigger { type = "UserStartsSession", session_type = "focus" }
        when { time_in_range(8, 16) }
        then { enforce_policy("lockout") }
      }
    "#;
    
    let ir = compile_fpl(fpl).unwrap()[0].clone();
    let ir_hash = ir.content_hash().unwrap();
    
    // Convert back to FPL
    let fpl_regenerated = ir_to_fpl(&ir).unwrap();
    
    // Re-compile and verify hash matches
    let ir2 = compile_fpl(&fpl_regenerated).unwrap()[0].clone();
    let ir2_hash = ir2.content_hash().unwrap();
    
    assert_eq!(ir_hash, ir2_hash, "Round-trip hashes must match");
  }
}
```

#### IR → FPL (Formatter)

```rust
pub fn ir_to_fpl(doc: &Document) -> Result<String> {
  match &doc.kind {
    DocumentKind::Rule => format_rule_ir(&doc.body)?,
    DocumentKind::Connector => format_connector_ir(&doc.body)?,
    // ... other kinds
  }
}

fn format_rule_ir(rule: &Rule) -> Result<String> {
  let mut output = String::new();
  
  // Header
  output.push_str(&format!(r#"rule "{}" {{"#, rule.name));
  output.push('\n');
  
  // Description
  if let Some(desc) = &rule.description {
    output.push_str(&format!(r#"  description = "{}""#, escape_string(desc)));
    output.push('\n');
  }
  
  // Trigger
  output.push_str(&format!("  trigger {{\n{}\n  }}\n", format_trigger(&rule.trigger)?));
  
  // Conditions
  output.push_str(&format!("  when {{\n{}\n  }}\n", format_conditions(&rule.conditions)?));
  
  // Actions
  output.push_str(&format!("  then {{\n{}\n  }}\n", format_actions(&rule.actions)?));
  
  // Metadata
  output.push_str(&format!("  metadata {{\n{}\n  }}\n", format_metadata(&rule.metadata)?));
  
  // Layout (if present)
  if let Some(layout) = &rule.layout {
    output.push_str(&format!("  @layout {{ x = {}, y = {} }}\n", layout.x, layout.y));
  }
  
  output.push_str("}\n");
  Ok(output)
}
```

**Formatter guarantees**:
- Consistent indentation (2 spaces)
- Stable key ordering in maps
- Deterministic output (same IR → same FPL string)
- Can be run through `rustfmt` for consistency

---

### Pair 2: Graph ↔ IR

**Transpiler**: `focus-graph-compiler` and `focus-ir-to-graph`

#### Graph JSON → IR (Compiler)

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphDoc {
  pub nodes: Vec<GraphNode>,
  pub edges: Vec<GraphEdge>,
  pub viewport: Viewport,
  pub rule_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphNode {
  pub id: String,
  pub type_: String, // "trigger", "condition", "action", etc.
  pub position: XYPosition,
  pub data: serde_json::Value, // NodeData variant
}

pub fn compile_graph(graph: &GraphDoc) -> Result<Document> {
  // 1. Validate graph structure (no cycles in trigger→action flow, etc.)
  validate_graph_structure(graph)?;
  
  // 2. Topologically sort nodes (triggers → conditions → actions)
  let sorted = topological_sort(&graph.nodes, &graph.edges)?;
  
  // 3. Convert nodes to IR
  let trigger = extract_trigger(&graph.nodes)?;
  let conditions = extract_conditions(&graph.nodes)?;
  let actions = extract_actions(&graph.nodes)?;
  
  // 4. Extract layout metadata (node positions)
  let layout: Vec<NodeLayout> = graph.nodes.iter().map(|n| NodeLayout {
    node_id: n.id.clone(),
    x: n.position.x,
    y: n.position.y,
  }).collect();
  
  Ok(Document {
    kind: DocumentKind::Rule,
    id: graph.rule_id.clone(),
    body: Rule {
      trigger,
      conditions,
      actions,
      layout: Some(layout),
      // ... other fields
    },
    // ... rest
  })
}
```

**Layout preservation**: Node positions are extracted and stored in IR as `layout` metadata. When converting back to graph, positions are restored.

#### IR → Graph JSON (Generator)

```rust
pub fn ir_to_graph(doc: &Document) -> Result<GraphDoc> {
  match doc.kind {
    DocumentKind::Rule => {
      let rule = &doc.body;
      
      let mut nodes = Vec::new();
      let mut edges = Vec::new();
      
      // Trigger node
      let trigger_id = "trigger-0";
      nodes.push(GraphNode {
        id: trigger_id.to_string(),
        type_: "trigger".to_string(),
        position: extract_position(&rule.layout, trigger_id).unwrap_or_default(),
        data: serde_json::to_value(&rule.trigger)?,
      });
      
      // Condition nodes
      for (i, condition) in rule.conditions.iter().enumerate() {
        let cond_id = format!("condition-{}", i);
        nodes.push(GraphNode {
          id: cond_id.clone(),
          type_: "condition".to_string(),
          position: extract_position(&rule.layout, &cond_id).unwrap_or_default(),
          data: serde_json::to_value(condition)?,
        });
        
        // Edge from trigger/previous condition to this condition
        let source = if i == 0 { trigger_id.to_string() } else { format!("condition-{}", i - 1) };
        edges.push(GraphEdge {
          id: format!("{}-{}", source, cond_id),
          source,
          target: cond_id,
          type_: "data_flow".to_string(),
        });
      }
      
      // Action nodes
      for (i, action) in rule.actions.iter().enumerate() {
        let action_id = format!("action-{}", i);
        nodes.push(GraphNode {
          id: action_id.clone(),
          type_: "action".to_string(),
          position: extract_position(&rule.layout, &action_id).unwrap_or_default(),
          data: serde_json::to_value(action)?,
        });
        
        // Edge from last condition/trigger to this action
        let source = if rule.conditions.is_empty() {
          trigger_id.to_string()
        } else {
          format!("condition-{}", rule.conditions.len() - 1)
        };
        
        edges.push(GraphEdge {
          id: format!("{}-{}", source, action_id),
          source,
          target: action_id,
          type_: "data_flow".to_string(),
        });
      }
      
      Ok(GraphDoc {
        nodes,
        edges,
        viewport: Viewport::default(),
        rule_id: doc.id.clone(),
      })
    }
    // ... other kinds
  }
}
```

---

### Pair 3: CLI Args ↔ IR

**Transpiler**: `focus-cli-builder` and `focus-ir-to-cli`

#### CLI Args → IR (Builder)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "focus")]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  #[command(about = "Create and apply a rule")]
  Rule {
    #[arg(long)]
    name: String,
    
    #[arg(long)]
    trigger_event: String,
    
    #[arg(long)]
    trigger_value: String,
    
    #[arg(long, num_args = 0..)]
    condition: Vec<String>,
    
    #[arg(long)]
    action: String,
    
    #[arg(long, num_args = 0..)]
    action_param: Vec<String>,
  },
}

pub fn build_rule_from_cli(args: Cli) -> Result<Document> {
  match args.command {
    Commands::Rule { name, trigger_event, trigger_value, condition, action, action_param } => {
      // Parse trigger_event + trigger_value
      let trigger = match trigger_event.as_str() {
        "user_starts_session" => Trigger::UserStartsSession {
          session_type: trigger_value,
        },
        "event_fired" => Trigger::EventFired {
          event_name: trigger_value,
        },
        // ... other variants
        _ => return Err("Unknown trigger event".into()),
      };
      
      // Parse conditions
      let conditions: Vec<Condition> = condition.iter().map(|c| {
        parse_condition_string(c)
      }).collect::<Result<_>>()?;
      
      // Parse actions and parameters
      let action_params: std::collections::BTreeMap<String, serde_json::Value> =
        action_param.iter().map(|p| {
          let (k, v) = p.split_once('=').ok_or("Invalid param format")?;
          Ok((k.to_string(), serde_json::Value::String(v.to_string())))
        }).collect::<Result<_>>()?;
      
      let actions = vec![Action::EnforcePolicy {
        policy_id: action,
        params: action_params,
      }];
      
      Ok(Document {
        kind: DocumentKind::Rule,
        id: format!("rule-{}", uuid::Uuid::new_v4()),
        name,
        body: Rule {
          trigger,
          conditions,
          actions,
          enabled: true,
          // ... metadata
        },
        // ... rest
      })
    }
  }
}
```

#### IR → CLI Args (Serializer)

```rust
pub fn ir_to_cli_args(doc: &Document) -> Result<Vec<String>> {
  match doc.kind {
    DocumentKind::Rule => {
      let rule = &doc.body;
      let mut args = vec!["focus", "rule"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
      
      args.push("--name".to_string());
      args.push(doc.name.clone());
      
      // Trigger
      match &rule.trigger {
        Trigger::UserStartsSession { session_type } => {
          args.push("--trigger-event".to_string());
          args.push("user_starts_session".to_string());
          args.push("--trigger-value".to_string());
          args.push(session_type.clone());
        }
        // ... other triggers
      }
      
      // Conditions
      for condition in &rule.conditions {
        args.push("--condition".to_string());
        args.push(condition_to_string(condition)?);
      }
      
      // Actions
      if let Some(action) = rule.actions.first() {
        match action {
          Action::EnforcePolicy { policy_id, params } => {
            args.push("--action".to_string());
            args.push("enforce_policy".to_string());
            
            for (k, v) in params {
              args.push("--action-param".to_string());
              args.push(format!("{}={}", k, v));
            }
          }
          // ... other actions
        }
      }
      
      Ok(args)
    }
    // ... other kinds
  }
}
```

**Note**: The IR → CLI direction is **lossy**. Some IR constructs (nested conditions, multiple actions) cannot be expressed as CLI flags. This is acceptable; CLI is for simple, one-off commands. For complex rules, use FPL or the GUI.

---

### Pair 4: Wizard Form State ↔ IR

**Transpiler**: `focus-wizard-compiler` and `focus-ir-to-wizard`

#### Form State → IR (Compiler)

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct WizardFormState {
  pub step: u32, // 1=trigger, 2=conditions, 3=actions, 4=review
  pub rule_name: String,
  pub rule_description: Option<String>,
  pub trigger_type: String,
  pub trigger_params: std::collections::BTreeMap<String, serde_json::Value>,
  pub conditions: Vec<ConditionFormItem>,
  pub actions: Vec<ActionFormItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionFormItem {
  pub operator: String, // "time_in_range", "day_of_week", etc.
  pub params: std::collections::BTreeMap<String, serde_json::Value>,
  pub enabled: bool,
}

pub fn compile_wizard_form(form: &WizardFormState) -> Result<Document> {
  // Reconstruct trigger from form values
  let trigger = parse_trigger_params(&form.trigger_type, &form.trigger_params)?;
  
  // Reconstruct conditions
  let conditions: Vec<Condition> = form.conditions.iter()
    .filter(|c| c.enabled)
    .map(|c| parse_condition_form(c))
    .collect::<Result<_>>()?;
  
  // Reconstruct actions
  let actions: Vec<Action> = form.actions.iter()
    .map(|a| parse_action_form(a))
    .collect::<Result<_>>()?;
  
  Ok(Document {
    kind: DocumentKind::Rule,
    id: format!("rule-{}", uuid::Uuid::new_v4()),
    name: form.rule_name.clone(),
    description: form.rule_description.clone(),
    body: Rule {
      enabled: true,
      trigger,
      conditions,
      actions,
      // ... metadata
    },
    // ... rest
  })
}
```

#### IR → Wizard Form State (Generator)

```rust
pub fn ir_to_wizard_form(doc: &Document) -> Result<WizardFormState> {
  match doc.kind {
    DocumentKind::Rule => {
      let rule = &doc.body;
      
      // Extract trigger
      let (trigger_type, trigger_params) = trigger_to_form_params(&rule.trigger)?;
      
      // Extract conditions
      let conditions: Vec<ConditionFormItem> = rule.conditions.iter()
        .map(|c| condition_to_form(c))
        .collect::<Result<_>>()?;
      
      // Extract actions
      let actions: Vec<ActionFormItem> = rule.actions.iter()
        .map(|a| action_to_form(a))
        .collect::<Result<_>>()?;
      
      Ok(WizardFormState {
        step: 4, // Show review step by default
        rule_name: doc.name.clone(),
        rule_description: doc.description.clone(),
        trigger_type,
        trigger_params,
        conditions,
        actions,
      })
    }
    // ... other kinds
  }
}
```

---

### Pair 5: Legacy TOML ↔ IR (Migration)

**Transpiler**: `focus-toml-importer` and `focus-ir-to-toml`

#### TOML (Legacy) → IR (Importer)

```rust
use toml::Value as TomlValue;

#[derive(Deserialize)]
struct LegacyTemplatePack {
  name: String,
  version: String,
  rules: Vec<LegacyRule>,
}

#[derive(Deserialize)]
struct LegacyRule {
  name: String,
  description: Option<String>,
  trigger: LegacyTrigger,
  conditions: Option<Vec<String>>, // ["time_in_range(8,16)"]
  actions: Option<Vec<String>>, // ["enforce_policy(social-media-lockout)"]
}

pub fn import_legacy_toml(toml_str: &str) -> Result<Vec<Document>> {
  let pack: LegacyTemplatePack = toml::from_str(toml_str)?;
  
  let mut docs = Vec::new();
  
  for legacy_rule in pack.rules {
    // Parse each legacy rule to modern IR
    let trigger = parse_legacy_trigger(&legacy_rule.trigger)?;
    
    let conditions = legacy_rule.conditions
      .unwrap_or_default()
      .iter()
      .map(|c_str| parse_condition_string(c_str))
      .collect::<Result<_>>()?;
    
    let actions = legacy_rule.actions
      .unwrap_or_default()
      .iter()
      .map(|a_str| parse_action_string(a_str))
      .collect::<Result<_>>()?;
    
    let doc = Document {
      kind: DocumentKind::Rule,
      id: format!("rule-{}", uuid::Uuid::new_v4()),
      name: legacy_rule.name,
      description: legacy_rule.description,
      body: Rule {
        enabled: true,
        trigger,
        conditions,
        actions,
        // ... metadata
      },
      // ... rest
    };
    
    docs.push(doc);
  }
  
  Ok(docs)
}
```

**Migration guarantee**: Existing TOML template-packs in `examples/templates/*.toml` can be one-shot converted to FPL:

```bash
$ focus lang from-toml examples/templates/school-weekday-focus.toml --output rules/school-weekday-focus.fpl
Migrating 1 rules from TOML to FPL...
✓ school-weekday-focus.toml → school-weekday-focus.fpl

Verify the output and check git diff to ensure correctness.
```

#### IR → TOML (Serializer)

```rust
pub fn ir_to_toml(doc: &Document) -> Result<String> {
  // Convert IR back to legacy TOML format for compatibility
  // Note: Some IR fields may not be expressible in TOML (lossy)
  
  match doc.kind {
    DocumentKind::Rule => {
      let rule = &doc.body;
      
      let mut toml = toml::map::Map::new();
      toml.insert("name".to_string(), TomlValue::String(doc.name.clone()));
      
      if let Some(desc) = &doc.description {
        toml.insert("description".to_string(), TomlValue::String(desc.clone()));
      }
      
      // Trigger
      toml.insert("trigger".to_string(), trigger_to_toml(&rule.trigger)?);
      
      // Conditions (as strings, lossy)
      let cond_strings: Vec<String> = rule.conditions.iter()
        .map(|c| condition_to_string(c))
        .collect::<Result<_>>()?;
      toml.insert("conditions".to_string(), TomlValue::Array(
        cond_strings.iter().map(|s| TomlValue::String(s.clone())).collect()
      ));
      
      // Actions (as strings, lossy)
      let action_strings: Vec<String> = rule.actions.iter()
        .map(|a| action_to_string(a))
        .collect::<Result<_>>()?;
      toml.insert("actions".to_string(), TomlValue::Array(
        action_strings.iter().map(|s| TomlValue::String(s.clone())).collect()
      ));
      
      Ok(toml::to_string_pretty(&TomlValue::Table(toml))?)
    }
    // ... other kinds
  }
}
```

---

## Property-Based Round-Trip Tests

**Goal**: For any FPL file, guarantee that `fpl → ir → fpl` produces byte-identical (or hash-identical) output.

```rust
use proptest::prelude::*;

prop_compose! {
  fn arb_trigger()(variant in 0..7) -> Trigger {
    match variant {
      0 => Trigger::UserStartsSession { session_type: "focus".to_string() },
      1 => Trigger::EventFired { event_name: "event".to_string() },
      2 => Trigger::TimeElapsed { duration_ms: 5000 },
      // ... etc
      _ => Trigger::UserStartsSession { session_type: "focus".to_string() },
    }
  }
}

prop_compose! {
  fn arb_rule()(
    trigger in arb_trigger(),
    conditions in prop::collection::vec(".*", 0..3),
    actions in prop::collection::vec(".*", 0..3),
  ) -> Rule {
    Rule {
      id: "test-rule".to_string(),
      name: "test-rule".to_string(),
      trigger,
      conditions: vec![], // Simplified for example
      actions: vec![],
      enabled: true,
      // ... metadata
    }
  }
}

#[cfg(test)]
mod roundtrip_tests {
  use proptest::proptest;
  
  proptest! {
    #[test]
    fn prop_fpl_ir_fpl_roundtrip(rule in arb_rule()) {
      // Compile rule to IR
      let ir = rule.to_ir().unwrap();
      let ir_hash = ir.content_hash().unwrap();
      
      // Convert IR to FPL
      let fpl = ir_to_fpl(&ir).unwrap();
      
      // Re-compile FPL to IR
      let ir2 = compile_fpl(&fpl).unwrap()[0].clone();
      let ir2_hash = ir2.content_hash().unwrap();
      
      // Hashes must match
      prop_assert_eq!(ir_hash, ir2_hash, "Round-trip hashes must be identical");
    }
  }
  
  proptest! {
    #[test]
    fn prop_graph_ir_graph_roundtrip(rule in arb_rule()) {
      let ir = rule.to_ir().unwrap();
      let ir_hash = ir.content_hash().unwrap();
      
      let graph = ir_to_graph(&ir).unwrap();
      
      let ir2 = compile_graph(&graph).unwrap();
      let ir2_hash = ir2.content_hash().unwrap();
      
      prop_assert_eq!(ir_hash, ir2_hash);
    }
  }
}
```

---

## CLI Commands

### `focus lang build`

Compile `.fpl` file to IR JSON:

```bash
$ focus lang build rules/deep-work-social-block.fpl
Output: /tmp/deep-work-social-block.ir.json (1.2 KB)
Hash: sha256:abc123def456...
Signature: ✓ Valid (coaching-team, 2026-04-23)
```

### `focus lang check`

Syntax and semantic check without compilation:

```bash
$ focus lang check
Checking 12 .fpl files...
  ✓ rules/deep-work-social-block.fpl
  ✓ rules/pomodoro-reward.fpl
  ✗ templates/broken-template.fpl:8:5 Type error
    Expected: Condition
    Got: string "hello"

2 files checked, 1 error, 0 warnings
```

### `focus lang fmt`

Format `.fpl` files (like rustfmt):

```bash
$ focus lang fmt --write rules/*.fpl
Formatted 12 files
```

### `focus lang explain`

Pretty-print IR in human-readable format:

```bash
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
    - emit_event("social_blocked", {"reason": "deep_work"})
  Signature: ✓ Valid (coaching-team, 2026-04-23)
```

### `focus lang from-toml`

One-shot migration of legacy TOML to FPL:

```bash
$ focus lang from-toml examples/templates/school-weekday-focus.toml --output rules/school-weekday-focus.fpl
Migrating 1 rules from TOML to FPL...
✓ school-weekday-focus.toml → school-weekday-focus.fpl
  Size: 456 bytes → 512 bytes
  Lines: 12 → 18
  
Verify the output and update imports as needed.
```

### `focus lang export`

Export IR to other formats:

```bash
# Export to TypeScript types
$ focus lang export --format typescript rules/deep-work-social-block.fpl --output types.ts

# Export to GraphQL schema
$ focus lang export --format graphql rules/*.fpl --output schema.graphql

# Export to OpenAPI
$ focus lang export --format openapi rules/*.fpl --output openapi.yaml
```

---

## Crate Organization

```
focus-lang-core/          # Shared types, IR definitions
focus-lang-compiler/      # FPL → IR compiler
focus-lang-formatter/     # IR → FPL formatter
focus-graph-compiler/     # Graph JSON → IR compiler
focus-ir-to-graph/        # IR → Graph JSON generator
focus-cli-builder/        # CLI args → IR builder
focus-ir-to-cli/          # IR → CLI args generator
focus-wizard-compiler/    # Wizard form state → IR compiler
focus-ir-to-wizard/       # IR → Wizard form state generator
focus-toml-importer/      # TOML → IR importer (migration)
focus-ir-to-toml/         # IR → TOML serializer
focus-codegen-ts/         # IR → TypeScript type codegen
focus-codegen-swift/      # IR → Swift type codegen
focus-cli/                # CLI entrypoint (uses all transpilers)
```

All crates depend on `focus-lang-core` for IR types.

---

## Testing Matrix

| Transpiler | Test Type | Example |
|------------|-----------|---------|
| FPL ↔ IR | Proptest roundtrip | `fpl → ir → fpl` hash match |
| Graph ↔ IR | Property test | `graph → ir → graph` with layout |
| CLI ↔ IR | Integration test | `focus rule create --name x` produces correct IR |
| Wizard ↔ IR | Unit test | Form state serializes to rule IR |
| TOML ↔ IR | Regression test | Legacy packs import cleanly |
| Codegen ↔ IR | Snapshot test | TypeScript/Swift output matches golden files |

---

## Performance Targets

- **Compile**: 100 rules < 200 ms (M-series), < 1 s (iPhone)
- **Codegen**: 1000 IR docs < 500 ms
- **Roundtrip**: FPL → IR → FPL < 50 ms (single rule)
- **Memory**: Full workspace evaluation < 50 MB

---

## Summary

Transpilers are the **glue between all four surfaces**. Key design:

- **Central IR**: All transpilers feed into and out of a single IR format (JSON)
- **Lossless roundtrips**: FPL ↔ IR ↔ Graph ↔ Wizard are all byte-identical
- **Partial CLI**: CLI → IR is complete; IR → CLI is lossy (acceptable)
- **Legacy migration**: TOML → FPL one-shot importer for existing packs
- **Property-based tests**: Guarantee roundtrip correctness across all pairs
- **Deterministic hashing**: Content-addressable IR for signing and versioning
- **CLI tools**: `build`, `check`, `fmt`, `explain`, `from-toml`, `export`

Every primitive (Rule, Connector, Template, Task, Schedule, Pose, etc.) gets the same treatment: defined once in IR, compiled from/to all surfaces, tested for roundtrip correctness.
