#if canImport(SwiftUI)
import XCTest
import SwiftUI
import SnapshotTesting
@testable import FocalPointApp

class KeyboardShortcutsTests: XCTestCase {

    /// Test that the KeyboardShortcutsSheetView renders without layout bugs.
    /// Verifies that all shortcuts are displayed correctly with proper formatting.
    func testKeyboardShortcutsSheetRendersCorrectly() {
        let view = KeyboardShortcutsSheetView()
            .preferredColorScheme(.dark)
            .background(Color.app.background)

        let vc = UIHostingController(rootViewController: view)
        vc.view.frame = CGRect(x: 0, y: 0, width: 390, height: 844) // iPhone 12 size

        assertSnapshot(matching: vc, as: .image)
    }

    /// Test that all keyboard shortcuts are present in the AVAILABLE_SHORTCUTS list.
    func testAllShortcutsAreDefined() {
        XCTAssertGreaterThan(AVAILABLE_SHORTCUTS.count, 0, "AVAILABLE_SHORTCUTS should not be empty")

        // Verify expected shortcuts exist
        let actions = AVAILABLE_SHORTCUTS.map { $0.action }

        let expectedActions = [
            "New Task",
            "Start Focus Session",
            "Cancel Focus Session",
            "Switch to Today",
            "Switch to Focus",
            "Switch to Tasks",
            "Open Settings",
            "Sync Now",
            "Run Rules Now",
        ]

        for action in expectedActions {
            XCTAssertTrue(
                actions.contains(action),
                "Expected action '\(action)' not found in AVAILABLE_SHORTCUTS"
            )
        }
    }

    /// Test that keyboard shortcuts are properly categorized by context.
    func testShortcutsAreCategorizedByContext() {
        let globalShortcuts = AVAILABLE_SHORTCUTS.filter { $0.context == "Global" }
        let focusTabShortcuts = AVAILABLE_SHORTCUTS.filter { $0.context == "Focus Tab" }

        XCTAssertGreaterThan(globalShortcuts.count, 0, "Should have global shortcuts")
        XCTAssertGreaterThanOrEqual(focusTabShortcuts.count, 1, "Should have at least one Focus Tab shortcut")
    }

    /// Test that KeyboardShortcutRow renders correctly.
    func testKeyboardShortcutRowRendersCorrectly() {
        let shortcut = KeyboardShortcut(key: "N", modifiers: "⌘", action: "New Task", context: "Global")
        let view = KeyboardShortcutRow(shortcut: shortcut)
            .preferredColorScheme(.dark)
            .background(Color.app.background)
            .padding()

        let vc = UIHostingController(rootViewController: view)
        vc.view.frame = CGRect(x: 0, y: 0, width: 390, height: 100)

        assertSnapshot(matching: vc, as: .image)
    }

    /// Test that KeyBadge renders with proper styling.
    func testKeyBadgeRendersCorrectly() {
        let view = VStack(spacing: 12) {
            KeyBadge(label: "⌘")
            KeyBadge(label: "N")
            KeyBadge(label: "⌘⇧")
        }
        .preferredColorScheme(.dark)
        .background(Color.app.background)
        .padding()

        let vc = UIHostingController(rootViewController: view)
        vc.view.frame = CGRect(x: 0, y: 0, width: 390, height: 200)

        assertSnapshot(matching: vc, as: .image)
    }

    /// Test that the shortcuts list is scrollable and doesn't overflow.
    func testKeyboardShortcutsSheetIsScrollable() {
        let view = KeyboardShortcutsSheetView()
            .preferredColorScheme(.dark)
            .background(Color.app.background)

        let vc = UIHostingController(rootViewController: view)
        vc.view.frame = CGRect(x: 0, y: 0, width: 390, height: 600) // Smaller height to test scrolling

        assertSnapshot(matching: vc, as: .image)
    }
}
#endif
