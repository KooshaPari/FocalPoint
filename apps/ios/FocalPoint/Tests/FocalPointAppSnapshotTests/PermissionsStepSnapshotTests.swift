#if canImport(SwiftUI)
import SwiftUI
import SnapshotTesting
import XCTest
@testable import FocalPointApp
import DesignSystem
import FocalPointCore

final class PermissionsStepSnapshotTests: XCTestCase {
    let namespace = Namespace().id

    // Test 1: All permissions granted
    func testAllPermissionsGranted() {
        let coord = OnboardingCoordinator()
        coord.notificationsStatus = .granted
        coord.calendarStatus = .granted
        coord.familyControlsStatus = .granted

        let view = PermissionsStep(namespace: namespace, coord: coord)
            .frame(height: 667)
            .background(Color.app.background)
            .preferredColorScheme(.light)

        assertSnapshot(matching: view, as: .image, named: "permissions_all_granted")
    }

    // Test 2: All permissions denied
    func testAllPermissionsDenied() {
        let coord = OnboardingCoordinator()
        coord.notificationsStatus = .denied
        coord.calendarStatus = .denied
        coord.familyControlsStatus = .pendingEntitlement

        let view = PermissionsStep(namespace: namespace, coord: coord)
            .frame(height: 667)
            .background(Color.app.background)
            .preferredColorScheme(.light)

        assertSnapshot(matching: view, as: .image, named: "permissions_all_denied")
    }

    // Test 3: Pending FamilyControls (Apple entitlement)
    func testPendingFamilyControls() {
        let coord = OnboardingCoordinator()
        coord.notificationsStatus = .granted
        coord.calendarStatus = .notDetermined
        coord.familyControlsStatus = .pendingEntitlement

        let view = PermissionsStep(namespace: namespace, coord: coord)
            .frame(height: 667)
            .background(Color.app.background)
            .preferredColorScheme(.light)

        assertSnapshot(matching: view, as: .image, named: "permissions_pending_family_controls")
    }

    // Test 4: Mixed states (partial grants, one pending)
    func testMixedPermissionStates() {
        let coord = OnboardingCoordinator()
        coord.notificationsStatus = .granted
        coord.calendarStatus = .denied
        coord.familyControlsStatus = .notDetermined

        let view = PermissionsStep(namespace: namespace, coord: coord)
            .frame(height: 667)
            .background(Color.app.background)
            .preferredColorScheme(.light)

        assertSnapshot(matching: view, as: .image, named: "permissions_mixed_states")
    }

    // Test 5: PermissionStateView — granted state
    func testPermissionStateViewGranted() {
        let view = PermissionStateView(
            icon: "bell.fill",
            title: "Notifications",
            description: "Nudges, reminders, and rule alerts",
            status: .granted,
            onRequest: {},
            onOpenSettings: {}
        )
        .padding(24)
        .background(Color.app.background)
        .preferredColorScheme(.light)

        assertSnapshot(matching: view, as: .image, named: "permission_state_granted")
    }

    // Test 6: PermissionStateView — denied state
    func testPermissionStateViewDenied() {
        let view = PermissionStateView(
            icon: "calendar.circle.fill",
            title: "Calendar (EventKit)",
            description: "Sync your events to avoid conflicts",
            status: .denied,
            onRequest: {},
            onOpenSettings: {}
        )
        .padding(24)
        .background(Color.app.background)
        .preferredColorScheme(.light)

        assertSnapshot(matching: view, as: .image, named: "permission_state_denied")
    }

    // Test 7: PermissionStateView — pending entitlement
    func testPermissionStateViewPendingEntitlement() {
        let view = PermissionStateView(
            icon: "shield.fill",
            title: "Family Controls",
            description: "Enforce app limits and schedules",
            status: .pendingEntitlement,
            onRequest: {},
            onOpenSettings: {}
        )
        .padding(24)
        .background(Color.app.background)
        .preferredColorScheme(.light)

        assertSnapshot(matching: view, as: .image, named: "permission_state_pending_entitlement")
    }
}
#endif
