# Changelog

All notable changes to the FocalPoint FPL VS Code Extension are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-24

### Added
- **Syntax Highlighting** — Full TextMate grammar for `.fpl` files with support for:
  - Keywords: `rule`, `task`, `schedule`, `connector`, `scene`, `coaching`, `enforcement`, `wallet_op`, `morning_brief`, `evening_shutdown`, `sound_cue`, `audit_query`
  - Macros: `reward`, `penalize`, `remind`, `celebrate`, `block`, `unlock_after`, `track_streak`, `if_pattern`
  - Actions: `on_event`, `on_schedule`, `grant_credit`, `notify`, `lock`, `unlock`
  - Conditions: `confidence_gte`, `payload_eq`, `time_in_range`, `after_count`, `on_streak`, `within_window`
  - Comments, strings, numbers, operators, and proper scope naming

- **Snippets** — 10 productivity snippets for common FPL patterns:
  1. `rule` — Full rule definition
  2. `reward` — Reward macro
  3. `penalize` — Penalize macro
  4. `remind` — Time-based reminder
  5. `celebrate` — Milestone celebration
  6. `block` — App blocking policy
  7. `task` — Scheduled task definition
  8. `schedule` — Cron-based trigger
  9. `connector` — External data source configuration
  10. `enforcement` — App/domain blocking policy

- **Commands**:
  - `focalpoint.compile` (Cmd+Shift+K / Ctrl+Shift+K) — Compile FPL with `focus rules import --dry-run`
  - `focalpoint.run` — Import rule to local SQLite database with `focus rules import`
  - `focalpoint.previewIr` (Cmd+Shift+I / Ctrl+Shift+I) — Preview IR in JSON/YAML with split-pane editor

- **Configuration**:
  - `focalpoint.focusBinary` — Path to `focus` CLI (default: `focus`)
  - `focalpoint.database` — SQLite DB path (default: `~/Library/Application Support/focalpoint/core.db`)
  - `focalpoint.autoCompileOnSave` — Auto-compile on save (default: `true`)
  - `focalpoint.irPreviewFormat` — IR output format: `json` or `yaml` (default: `json`)

- **Hover Provider** — Inline documentation for FPL keywords, macros, actions, and conditions

- **Code Lens** — Above each `rule(...)`:
  - "$(play) Compile to IR" — Run compile command
  - "$(preview) Show IR Hash" — Run IR preview command

- **Diagnostics** — Compile-time error reporting with line numbers in the Problems panel

- **Status Bar** — Real-time feedback during compile/import/preview operations

- **Language Configuration**:
  - Comment syntax (`#`)
  - Bracket pair matching
  - Auto-indent rules for FPL control structures
  - Auto-closing pairs for quotes and brackets

### Documentation
- Comprehensive `README.md` with feature overview, configuration guide, and quick start
- Per-command documentation with keyboard shortcuts and usage examples
- Architecture overview and known limitations

---

## Future Roadmap (Post-0.1.0)

- [ ] **Language Server Protocol (LSP)** — Real-time linting, diagnostics, and IntelliSense
- [ ] **Debug Adapter** — Step-through debugging for rules
- [ ] **Rule Template Gallery** — Searchable templates from the FocalPoint template marketplace
- [ ] **Multi-file Workspace** — Lint across linked `.fpl` files and dependencies
- [ ] **TOML Support** — Syntax highlighting for legacy TOML rule format
- [ ] **Visual Rule Builder** — Graphical UI for rule composition (Web-based or integrated)
- [ ] **Test Runner** — Execute FocalPoint test suites directly from VS Code

---

## Contribution Guidelines

This extension is maintained as part of the [FocalPoint](https://github.com/KooshaPari/FocalPoint) project. All contributions follow the Phenotype-org governance policies documented in the main repository.

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.
