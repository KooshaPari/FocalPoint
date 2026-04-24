#if canImport(SwiftUI)
import SwiftUI
import FocalPointCore

/// Menu commands for macOS (Designed for iPad) and iPadOS with external keyboard.
/// Contributes to standard macOS menu bar for File, Edit, View, and custom Focus menus.
public struct FocalPointMenuCommands: Commands {
    @EnvironmentObject private var holder: CoreHolder

    public var body: some Commands {
        // File menu
        CommandGroup(replacing: .newItem) {
            Button("New Task") {
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.NewTask"), object: nil)
            }
            .keyboardShortcut("n", modifiers: .command)
        }

        // Edit menu
        CommandGroup(replacing: .pasteboard) {
            Button("Sync Now") {
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SyncNow"), object: nil)
            }
            .keyboardShortcut("s", modifiers: [.command, .shift])

            Divider()

            Button("Run Rules Now") {
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.RunRules"), object: nil)
            }
            .keyboardShortcut("r", modifiers: [.command, .shift])
        }

        // View menu: Tab navigation
        CommandGroup(replacing: .toolbar) {
            Menu("Go to Tab") {
                Button("Today (⌘1)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 0)
                }
                .keyboardShortcut("1", modifiers: .command)

                Button("Focus (⌘2)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 1)
                }
                .keyboardShortcut("2", modifiers: .command)

                Button("Tasks (⌘3)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 2)
                }
                .keyboardShortcut("3", modifiers: .command)

                Button("Home (⌘4)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 3)
                }
                .keyboardShortcut("4", modifiers: .command)

                Button("Rules (⌘5)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 4)
                }
                .keyboardShortcut("5", modifiers: .command)

                Button("Rewards (⌘6)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 5)
                }
                .keyboardShortcut("6", modifiers: .command)

                Button("Stats (⌘7)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 6)
                }
                .keyboardShortcut("7", modifiers: .command)

                Button("Coachy (⌘8)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 7)
                }
                .keyboardShortcut("8", modifiers: .command)

                Button("Activity (⌘9)") {
                    NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.SelectTab"), object: 8)
                }
                .keyboardShortcut("9", modifiers: .command)
            }
        }

        // Focus menu (custom)
        CommandMenu("Focus") {
            Button("Start Focus Session") {
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.StartFocus"), object: nil)
            }
            .keyboardShortcut("f", modifiers: .command)

            Button("Cancel Focus Session") {
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.CancelFocus"), object: nil)
            }
            .keyboardShortcut(".", modifiers: .command)

            Divider()

            Button("Keyboard Shortcuts") {
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.ShowShortcuts"), object: nil)
            }
            .keyboardShortcut("?", modifiers: .command)
        }

        // Settings command
        CommandGroup(replacing: .appSettings) {
            Button("Settings") {
                NotificationCenter.default.post(name: NSNotification.Name("KeyboardShortcut.OpenSettings"), object: nil)
            }
            .keyboardShortcut(",", modifiers: .command)
        }
    }
}
#endif
