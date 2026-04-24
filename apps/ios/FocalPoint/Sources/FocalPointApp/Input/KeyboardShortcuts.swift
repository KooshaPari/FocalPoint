#if canImport(SwiftUI)
import SwiftUI
import FocalPointCore

/// Keyboard shortcuts for iOS/iPadOS/Designed-for-iPad apps.
/// Available on physical keyboards and external keyboards.
///
/// Shortcuts:
/// - ⌘N: new task
/// - ⌘F: start focus session
/// - ⌘.: cancel focus session
/// - ⌘1–9: switch between tabs
/// - ⌘,: open Settings
/// - ⌘⇧S: Sync now
/// - ⌘⇧R: Run rules now
/// - Space: start/pause focus (when Focus tab active)
public struct KeyboardShortcutsModifier: ViewModifier {
    @EnvironmentObject private var holder: CoreHolder
    @State private var selectedTabIndex: Int = 0

    public func body(content: Content) -> some View {
        content
            .keyboardShortcut("n", modifiers: .command) {
                // ⌘N: new task
                // Delegates to TasksView action
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.NewTask"), object: nil)
            }
            .keyboardShortcut("f", modifiers: .command) {
                // ⌘F: start focus session
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.StartFocus"), object: nil)
            }
            .keyboardShortcut(".", modifiers: .command) {
                // ⌘.: cancel focus session
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.CancelFocus"), object: nil)
            }
            .keyboardShortcut("1", modifiers: .command) {
                selectedTabIndex = 0
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 0)
            }
            .keyboardShortcut("2", modifiers: .command) {
                selectedTabIndex = 1
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 1)
            }
            .keyboardShortcut("3", modifiers: .command) {
                selectedTabIndex = 2
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 2)
            }
            .keyboardShortcut("4", modifiers: .command) {
                selectedTabIndex = 3
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 3)
            }
            .keyboardShortcut("5", modifiers: .command) {
                selectedTabIndex = 4
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 4)
            }
            .keyboardShortcut("6", modifiers: .command) {
                selectedTabIndex = 5
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 5)
            }
            .keyboardShortcut("7", modifiers: .command) {
                selectedTabIndex = 6
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 6)
            }
            .keyboardShortcut("8", modifiers: .command) {
                selectedTabIndex = 7
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 7)
            }
            .keyboardShortcut("9", modifiers: .command) {
                selectedTabIndex = 8
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 8)
            }
            .keyboardShortcut(",", modifiers: .command) {
                // ⌘,: open Settings
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.OpenSettings"), object: nil)
            }
            .keyboardShortcut("s", modifiers: [.command, .shift]) {
                // ⌘⇧S: Sync now
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SyncNow"), object: nil)
            }
            .keyboardShortcut("r", modifiers: [.command, .shift]) {
                // ⌘⇧R: Run rules now
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.RunRules"), object: nil)
            }
    }
}

/// Keyboard shortcut definition for in-app reference.
public struct KeyboardShortcut {
    public let key: String
    public let modifiers: String
    public let action: String
    public let context: String

    public init(key: String, modifiers: String, action: String, context: String = "Global") {
        self.key = key
        self.modifiers = modifiers
        self.action = action
        self.context = context
    }
}

/// Complete list of available keyboard shortcuts.
public let AVAILABLE_SHORTCUTS: [KeyboardShortcut] = [
    KeyboardShortcut(key: "N", modifiers: "⌘", action: "New Task", context: "Global"),
    KeyboardShortcut(key: "F", modifiers: "⌘", action: "Start Focus Session", context: "Global"),
    KeyboardShortcut(key: ".", modifiers: "⌘", action: "Cancel Focus Session", context: "Global"),
    KeyboardShortcut(key: "1", modifiers: "⌘", action: "Switch to Today", context: "Global"),
    KeyboardShortcut(key: "2", modifiers: "⌘", action: "Switch to Focus", context: "Global"),
    KeyboardShortcut(key: "3", modifiers: "⌘", action: "Switch to Tasks", context: "Global"),
    KeyboardShortcut(key: "4", modifiers: "⌘", action: "Switch to Home", context: "Global"),
    KeyboardShortcut(key: "5", modifiers: "⌘", action: "Switch to Rules", context: "Global"),
    KeyboardShortcut(key: "6", modifiers: "⌘", action: "Switch to Rewards", context: "Global"),
    KeyboardShortcut(key: "7", modifiers: "⌘", action: "Switch to Stats", context: "Global"),
    KeyboardShortcut(key: "8", modifiers: "⌘", action: "Switch to Coachy", context: "Global"),
    KeyboardShortcut(key: "9", modifiers: "⌘", action: "Switch to Activity", context: "Global"),
    KeyboardShortcut(key: ",", modifiers: "⌘", action: "Open Settings", context: "Global"),
    KeyboardShortcut(key: "S", modifiers: "⌘⇧", action: "Sync Now", context: "Global"),
    KeyboardShortcut(key: "R", modifiers: "⌘⇧", action: "Run Rules Now", context: "Global"),
    KeyboardShortcut(key: "Space", modifiers: "", action: "Start/Pause Focus", context: "Focus Tab"),
]

extension View {
    /// Apply all keyboard shortcuts to a view.
    public func withKeyboardShortcuts() -> some View {
        self.modifier(KeyboardShortcutsModifier())
    }
}
#endif
