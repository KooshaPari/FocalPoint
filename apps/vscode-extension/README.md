# FocalPoint FPL — VS Code Extension

**Version 0.1.0** | Professional language support for `.fpl` files (FocalPoint Focus Policy Language).

Boost your rule authoring workflow with syntax highlighting, snippets, compile-on-save, and IR preview.

## Features

### Syntax Highlighting
- Full TextMate grammar for `.fpl` files (Starlark-based)
- Scoped color support for keywords, macros, functions, strings, numbers, and comments
- Language configuration with bracket matching and auto-indent

### Snippets (10 patterns)
Autocomplete prefixes for the most common FPL constructs:

| Prefix | Description |
|--------|-------------|
| `rule` | Full rule definition with event trigger, conditions, and actions |
| `reward` | Reward user action with credits and streak tracking |
| `penalize` | Penalize action with credit deduction |
| `remind` | Time-based reminder with cron schedule |
| `celebrate` | Celebrate milestones with sound/haptic feedback |
| `block` | Block distracting apps during time windows |
| `task` | Scheduled task with priority and deadline |
| `schedule` | Cron-based schedule trigger |
| `connector` | Configure external data source (Canvas, GCal, etc.) |
| `enforcement` | App/domain blocking policy |

### Commands

#### `focalpoint.compile` — Compile FPL (Dry-Run)
- **Shortcut:** `Cmd+Shift+K` (macOS) / `Ctrl+Shift+K` (Windows/Linux)
- Runs `focus rules import --dry-run` on the current file
- Shows compilation errors with line numbers in the Problems panel
- Disables on next successful compile

#### `focalpoint.run` — Import Rule to Local DB
- Runs `focus rules import --db=<path>` on the current file
- Persists the rule to your local FocalPoint database
- Requires `focus` CLI in PATH and valid database path in config

#### `focalpoint.previewIr` — Preview Intermediate Representation
- **Shortcut:** `Cmd+Shift+I` (macOS) / `Ctrl+Shift+I` (Windows/Linux)
- Opens a split pane showing the compiled IR in JSON or YAML format
- Runs `focus eval` to generate the IR
- Useful for debugging rule compilation and understanding the internal representation

### Auto-Compile on Save
Enabled by default. Automatically compiles the FPL file when saved; disable in settings if preferred.

### Hover Provider
Hover over FPL keywords and macros to see inline documentation:
- `rule`, `on_event`, `on_schedule`
- `reward`, `penalize`, `block`
- `notify`, `grant_credit`, `confidence_gte`, `payload_eq`
- `task`, `connector`, `scene`, `enforcement`

### Code Lens
Above each `rule(...)` definition, view two code lenses:
- **"$(play) Compile to IR"** — Jump to compile this rule
- **"$(preview) Show IR Hash"** — Open IR preview in split pane

## Configuration

Open VS Code settings (`Code` → `Preferences` → `Settings`, or `Cmd+,`):

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `focalpoint.focusBinary` | string | `focus` | Path to the `focus` CLI binary (use just `focus` if in PATH) |
| `focalpoint.database` | string | `~/Library/Application Support/focalpoint/core.db` | SQLite database path |
| `focalpoint.autoCompileOnSave` | boolean | `true` | Auto-compile on save |
| `focalpoint.irPreviewFormat` | string | `json` | IR output format: `json` or `yaml` |

**Example settings.json:**
```json
{
  "focalpoint.focusBinary": "/Users/you/focalpoint-repo/target/release/focus",
  "focalpoint.database": "~/.focalpoint/core.db",
  "focalpoint.autoCompileOnSave": true,
  "focalpoint.irPreviewFormat": "json"
}
```

## Requirements

- **VS Code** 1.95.0 or later
- **`focus` CLI** installed and in PATH (or configured path)
- **FocalPoint workspace** with `.fpl` files

To build the CLI locally:
```bash
cd /path/to/FocalPoint
cargo build -p focus-cli --release
export PATH="$(pwd)/target/release:$PATH"
```

## Quick Start

1. **Install the extension** from the VS Code Marketplace (or load locally for development).
2. **Open a `.fpl` file** from your FocalPoint workspace.
3. **Type `rule`** → hit `Ctrl+Space` (or `Cmd+Space` on macOS) → select "Rule (High-Level)" and fill in the template.
4. **Hit `Cmd+Shift+K`** to compile and check syntax.
5. **Hit `Cmd+Shift+I`** to preview the compiled IR in a split pane.
6. **Hit `Cmd+Shift+Enter`** (or use "FocalPoint: Import Rule to Local DB" from the command palette) to persist to your local database.

## Example FPL File

```fpl
# Block social apps during deep work (from deep-work-starter.fpl)
rule(
    id="deep-work-social-block",
    name="Deep work — no social",
    priority=80,
    cooldown_seconds=600,
    duration_seconds=3000,
    trigger=on_event("focus:session_started"),
    conditions=[],
    actions=[
        block(profile="social", duration_seconds=3000, rigidity="hard"),
    ],
    explanation_template="Social apps locked while {rule_name} is active.",
    enabled=1
)
```

## Architecture

- **Extension Entry:** `src/extension.ts`
  - Command handlers for compile, run, preview
  - Hover provider, code lens, diagnostics
  - Status bar updates on compile/import state

- **TextMate Grammar:** `syntaxes/fpl.tmLanguage.json`
  - Starlark-based tokenization
  - FPL macro recognition (reward, penalize, remind, celebrate, block, etc.)
  - FPL action recognition (on_event, on_schedule, notify, grant_credit, etc.)
  - FPL condition recognition (confidence_gte, payload_eq, etc.)

- **Snippets:** `snippets/fpl.json` (10 patterns)

- **Language Config:** `language-configuration.json`
  - Comment syntax, bracket pairs, auto-indent

## Known Limitations

- LSP server (`focus lsp`) is stubbed; hover/code-lens hints are static for now
- Diagnostics are reported only for compile-time errors (no real-time linting yet)
- IR preview spawns a subprocess; large rules may take a few seconds

## Contributing

Found a bug or have a feature request? Open an issue on the [FocalPoint GitHub repository](https://github.com/KooshaPari/FocalPoint/issues).

## License

MIT

---

**Made with ❤️ for FocalPoint rule authors.**
