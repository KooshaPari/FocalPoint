# VS Code Extension for FPL Files

**Quick setup for `.fpl` file authoring in VS Code** — syntax highlighting, auto-compile, and IR preview.

## Install

The FocalPoint FPL extension is available on the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=focalpoint.focalpoint-fpl).

1. Open VS Code
2. Go to Extensions (Cmd+Shift+X / Ctrl+Shift+X)
3. Search for "FocalPoint FPL"
4. Click **Install**

Alternatively, install from the command line:
```bash
code --install-extension focalpoint.focalpoint-fpl
```

Or load the extension locally during development:
```bash
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint/apps/vscode-extension
npm install
npm run dev
# Then in VS Code: Cmd+Shift+P → "Extensions: Install from VSIX" → select the .vsix file
```

## Configure

Open VS Code Settings (`Cmd+,` on macOS, `Ctrl+,` on Windows/Linux) and search for "FocalPoint":

| Setting | Default | Notes |
|---------|---------|-------|
| `focalpoint.focusBinary` | `focus` | Path to the `focus` CLI. If in PATH, just use `focus`. |
| `focalpoint.database` | `~/Library/Application Support/focalpoint/core.db` | SQLite database path. Adjust for your platform. |
| `focalpoint.autoCompileOnSave` | `true` | Auto-compile on save. |
| `focalpoint.irPreviewFormat` | `json` | IR preview format: `json` or `yaml`. |

**Example `settings.json` snippet:**
```json
{
  "focalpoint.focusBinary": "/Users/you/FocalPoint/target/release/focus",
  "focalpoint.database": "~/.focalpoint/core.db",
  "focalpoint.autoCompileOnSave": true,
  "focalpoint.irPreviewFormat": "json"
}
```

To build the `focus` CLI locally:
```bash
cd /path/to/FocalPoint
cargo build -p focus-cli --release
# Binary at: target/release/focus
```

## Usage

### Write a Rule

1. Create or open a `.fpl` file
2. Type `rule` and press `Ctrl+Space` (Cmd+Space on macOS) to autocomplete
3. Fill in the template fields:

```fpl
rule(
    id="my-first-rule",
    name="My first rule",
    trigger=on_event("focus:session_started"),
    conditions=[],
    actions=[
        notify("Focus session active!")
    ],
    priority=100,
    enabled=1
)
```

### Compile & Debug

| Shortcut | Command | Action |
|----------|---------|--------|
| `Cmd+Shift+K` | `focalpoint.compile` | Dry-run compile; shows errors in Problems panel |
| `Cmd+Shift+I` | `focalpoint.previewIr` | Open IR preview in split pane |
| `Cmd+Shift+Enter` | `focalpoint.run` | Import rule to local database |

Or use the Command Palette (Cmd+Shift+P / Ctrl+Shift+P):
- "FocalPoint: Compile FPL (dry-run)"
- "FocalPoint: Import Rule to Local DB"
- "FocalPoint: Preview IR"

### Hover for Hints

Hover over any FPL keyword or macro to see inline documentation:
- `rule`, `task`, `schedule`, `connector`
- `reward`, `penalize`, `block`, `remind`, `celebrate`
- `on_event`, `on_schedule`, `grant_credit`, `notify`
- `confidence_gte`, `payload_eq`

### Use Snippets

Type any of these prefixes and press `Ctrl+Space` / `Cmd+Space`:

| Prefix | Creates |
|--------|---------|
| `rule` | Full rule definition with all fields |
| `reward` | Reward macro with credits and streak |
| `penalize` | Penalize macro |
| `remind` | Time-based reminder with cron |
| `celebrate` | Milestone celebration with sound |
| `block` | App blocking policy |
| `task` | Scheduled task with priority |
| `schedule` | Cron-based trigger |
| `connector` | External data source (Canvas, GCal, etc.) |
| `enforcement` | App/domain blocking enforcement |

### Code Lens

Above each `rule(...)` definition, you'll see two clickable lenses:
- **"$(play) Compile to IR"** — Compiles this rule
- **"$(preview) Show IR Hash"** — Shows the intermediate representation

## Troubleshooting

### "Command not found: focus"
Ensure the `focus` CLI is installed and in your PATH, or set the full path in settings:
```bash
# Build locally
cd /path/to/FocalPoint
cargo build -p focus-cli --release
# Then in VS Code settings:
# "focalpoint.focusBinary": "/Users/you/FocalPoint/target/release/focus"
```

### "Compile failed" with cryptic error
Check the VS Code **Problems** panel (View → Problems, or Cmd+Shift+M) for the full error message. If it references a line number, the editor will highlight that line.

### IR preview opens but is blank
Ensure the rule compiles cleanly first (Cmd+Shift+K). The preview runs `focus eval` on the file; syntax errors will prevent IR generation.

### Database import fails
Verify the database path in settings matches your FocalPoint installation. Default is macOS; adjust for Linux/Windows:
- macOS: `~/Library/Application Support/focalpoint/core.db`
- Linux: `~/.local/share/focalpoint/core.db` (or `$XDG_DATA_HOME/focalpoint/core.db`)
- Windows: `%APPDATA%\focalpoint\core.db`

## What's Next?

Once you've authored a rule:

1. **Compile & preview** to inspect the IR
2. **Import to database** to test locally
3. **Run the FocalPoint iOS app** to see the rule in action
4. **Check logs** with `focus audit list` to verify the rule fired

See the [CLI Guide](./cli_demo.md) for full CLI documentation.

---

Made with ❤️ for FocalPoint rule authors.
