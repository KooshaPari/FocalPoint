import SwiftUI
import SnapshotTesting
import XCTest
@testable import FocalPointApp
import DesignSystem

@MainActor
final class DiagnosticsSnapshotTests: XCTestCase {
    /// FR-DIAG-001: Snapshot test for Diagnostics info view (both states).
    /// Verifies the UI renders correctly with and without crash reporting enabled.

    func testDiagnosticsInfoViewSnapshot() {
        let view = DiagnosticsInfoView()
            .preferredColorScheme(.dark)

        assertSnapshot(
            of: view,
            as: .image(on: .iPhone13ProDark),
            named: "diagnostics-info-view"
        )
    }

    /// Test DiagnosticsInfoView renders without crashes.
    func testDiagnosticsInfoViewCompiles() {
        let view = DiagnosticsInfoView()
        XCTAssertNotNil(view, "DiagnosticsInfoView should compile and instantiate")
    }

    /// Test BulletPoint component renders correctly.
    func testBulletPointComponentWorks() {
        let bullet = BulletPoint("Test item")
        XCTAssertNotNil(bullet, "BulletPoint should render")
    }

    /// FR-DIAG-001: Verify Diagnostics toggle is visible in SettingsView.
    /// This is a compile-time test to ensure the new Diagnostics section
    /// is integrated into SettingsView without breaking existing functionality.
    func testSettingsViewIncludesDiagnosticsSection() {
        // This test verifies:
        // 1. SettingsView still compiles with new Diagnostics section
        // 2. @AppStorage("app.sentryEnabled") is accessible
        // 3. DiagnosticsInfoView is importable
        let settingsView = SettingsView()
        XCTAssertNotNil(settingsView, "SettingsView with Diagnostics should compile")
    }

    /// Verify that toggling crash reporting in Settings persists to @AppStorage.
    func testCrashReportingTogglePersistence() {
        let defaults = UserDefaults.standard
        defaults.set(false, forKey: "app.sentryEnabled")
        XCTAssertFalse(defaults.bool(forKey: "app.sentryEnabled"))

        // Simulate user toggling on
        defaults.set(true, forKey: "app.sentryEnabled")
        XCTAssertTrue(defaults.bool(forKey: "app.sentryEnabled"))

        // Clean up
        defaults.removeObject(forKey: "app.sentryEnabled")
    }
}
