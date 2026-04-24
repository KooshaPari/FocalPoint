import XCTest
import AppIntents
@testable import FocalPointApp

final class AppIntentsTests: XCTestCase {
    /// Test that AddTaskIntent has valid metadata for Siri discovery.
    func testAddTaskIntentMetadata() {
        let intent = AddTaskIntent(title: "Buy milk", priority: "High", duration: 15)
        XCTAssertEqual(AddTaskIntent.title.description, "Add Focus Task")
        XCTAssertFalse(AddTaskIntent.openAppWhenRun)
    }

    /// Test that StartFocusIntent supports optional parameters.
    func testStartFocusIntentParameters() {
        let intent1 = StartFocusIntent(duration: 30, rule: "Deep Work")
        let intent2 = StartFocusIntent(duration: nil, rule: nil)
        // Intents should instantiate with optional parameters
        XCTAssertNotNil(intent1)
        XCTAssertNotNil(intent2)
    }

    /// Test that CheckBalanceIntent metadata is present.
    func testCheckBalanceIntentMetadata() {
        let intent = CheckBalanceIntent()
        XCTAssertEqual(CheckBalanceIntent.title.description, "Check FocalPoint Balance")
        XCTAssertFalse(CheckBalanceIntent.openAppWhenRun)
    }

    /// Test that all shortcuts are registered in AppShortcutsProvider.
    func testAppShortcutsProviderRegistration() {
        let provider = FocalPointAppShortcutsProvider()
        let shortcuts = FocalPointAppShortcutsProvider.appShortcuts
        // Should have 6 shortcuts
        XCTAssertGreaterThanOrEqual(shortcuts.count, 6)
    }

    /// Test that SyncNowIntent has correct title.
    func testSyncNowIntentTitle() {
        XCTAssertEqual(SyncNowIntent.title.description, "Sync FocalPoint")
    }

    /// Test that LogNoteIntent accepts note content parameter.
    func testLogNoteIntentParameter() {
        let intent = LogNoteIntent(note: "Stayed focused for 45 minutes")
        XCTAssertEqual(intent.note, "Stayed focused for 45 minutes")
    }
}
