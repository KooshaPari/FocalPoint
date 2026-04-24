# Input Module

Handles keyboard input and menu commands for iOS with external keyboards, iPad, and Mac (Designed for iPad).

## Files

- **KeyboardShortcuts.swift** — SwiftUI `.keyboardShortcut(...)` modifiers and notification-based event dispatch
  - `KeyboardShortcutsModifier` — applies all 15 global shortcuts
  - `AVAILABLE_SHORTCUTS` — canonical list of all shortcuts for display
  - `View.withKeyboardShortcuts()` — convenience extension to apply shortcuts

- **MenuCommands.swift** — macOS menu bar contributions (File, Edit, View, Focus menus)
  - `FocalPointMenuCommands` — implements `Commands` protocol for standard Mac menus
  - Mirrors keyboard shortcuts in the menu bar for discoverability

## Integration

1. **In FocalPointApp.swift**:
   ```swift
   .withKeyboardShortcuts()  // Add to the main content view
   FocalPointMenuCommands()  // Add to the Scene body
   ```

2. **In Settings → Support**:
   - Add button to open `KeyboardShortcutsSheetView`
   - Users can reference all shortcuts from the app

## Shortcuts

### Global (15 total)
- ⌘N: New Task
- ⌘F: Start Focus
- ⌘.: Cancel Focus
- ⌘1–9: Tab switching
- ⌘,: Settings
- ⌘⇧S: Sync Now
- ⌘⇧R: Run Rules

### Focus Tab (1 total)
- Space: Start/Pause Focus

## Notification Center Events

Shortcuts dispatch via `NotificationCenter.default` to allow cross-view handling:

- `KeyboardShortcut.NewTask`
- `KeyboardShortcut.StartFocus`
- `KeyboardShortcut.CancelFocus`
- `KeyboardShortcut.SelectTab` (object: tab index)
- `KeyboardShortcut.OpenSettings`
- `KeyboardShortcut.SyncNow`
- `KeyboardShortcut.RunRules`
- `KeyboardShortcut.ShowShortcuts` (⌘? in Focus menu)

Observers can subscribe to any of these to react to user input.

## Testing

Snapshot tests in `Tests/FocalPointAppSnapshotTests/KeyboardShortcutsTests.swift`:
- Renders the shortcuts sheet without layout bugs
- Verifies all shortcuts are defined
- Tests categorization by context
- Tests individual row and badge rendering

## References

- Documentation: `docs-site/guides/keyboard_shortcuts.md`
- Settings UI: `Settings/SettingsView.swift` and `Settings/KeyboardShortcutsSheetView.swift`
- FR-KB-001: Keyboard Shortcuts UI
