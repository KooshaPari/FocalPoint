import SwiftUI
import SnapshotTesting

// MARK: - Mock Data Holder

/// Mock CoreHolder for snapshot testing.
/// Returns predictable sample data so snapshots are deterministic.
class MockCoreHolder: NSObject, ObservableObject {
    @Published var isInitialized = true
    @Published var currentUserRole: String = "self"
    @Published var deviceName: String = "Test Device"

    // Mock functions for core FFI calls
    func mockGetRules() -> [String] {
        return [
            "Instagram: 2 hours/day",
            "TikTok: 1 hour/day",
            "Focus block: 9am-12pm (all apps)",
        ]
    }

    func mockGetRewardBalance() -> Int {
        return 4500 // 45 minutes in reward credit
    }

    func mockGetAuditChainLength() -> Int {
        return 42 // Example: 42 audit records
    }
}

// MARK: - Snapshot Assertion Helpers

/// Assert snapshot for a SwiftUI view.
/// On first run (record=true), saves baseline. On subsequent runs, compares.
func assertViewSnapshot<V: View>(
    view: V,
    name: String,
    record: Bool = false,
    file: StaticString = #file,
    testName: String = #function,
    line: UInt = #line
) {
    assertSnapshot(
        of: view,
        as: .image(on: .iPhone13Pro),
        named: name,
        record: record,
        file: file,
        testName: testName,
        line: line
    )
}

// MARK: - Device Configurations

extension ViewImageConfig {
    static let iPhone13Pro = ViewImageConfig(
        size: .init(width: 390, height: 844),
        safeAreaInsets: .init(top: 47, left: 0, bottom: 34, right: 0),
        traits: .init(userInterfaceStyle: .light)
    )

    static let iPhone13ProDark = ViewImageConfig(
        size: .init(width: 390, height: 844),
        safeAreaInsets: .init(top: 47, left: 0, bottom: 34, right: 0),
        traits: .init(userInterfaceStyle: .dark)
    )
}
