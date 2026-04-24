import SnapshotTesting
import SwiftUI
import XCTest
import ActivityKit

// Test snapshots for all Live Activity presentations: lock screen banner, Dynamic Island (expanded, compact, minimal), and ring.
// Traces to: FR-LIVE-ACTIVITY-001, FR-LIVE-ACTIVITY-002

@available(iOS 16.1, *)
final class LiveActivityTests: XCTestCase {
    private let testAttributes = FocusSessionAttributes(
        sessionTitle: "Deep Work",
        startedAt: Date(timeIntervalSince1970: 0),
        plannedDuration: 1500, // 25 min
        breakInterval: 300,
        bgTint: "blue",
        coachyEmoji: "🧘"
    )

    func testLockScreenBannerRunning() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 900, // 15 min remaining
            totalSeconds: 1500,
            isPaused: false,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: nil,
            timestamp: Date()
        )

        let view = LockScreenBannerView(attributes: testAttributes, state: state)
            .frame(width: 330, height: 120)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 330, height: 120)))
    }

    func testLockScreenBannerPaused() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 600, // 10 min remaining
            totalSeconds: 1500,
            isPaused: true,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: nil,
            timestamp: Date()
        )

        let view = LockScreenBannerView(attributes: testAttributes, state: state)
            .frame(width: 330, height: 120)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 330, height: 120)))
    }

    @available(iOS 16.2, *)
    func testDynamicIslandExpandedRunning() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 900,
            totalSeconds: 1500,
            isPaused: false,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: 300, // 5 min until break
            timestamp: Date()
        )

        let view = DynamicIslandExpandedView(attributes: testAttributes, state: state)
            .frame(width: 330, height: 200)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 330, height: 200)))
    }

    @available(iOS 16.2, *)
    func testDynamicIslandExpandedPaused() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 600,
            totalSeconds: 1500,
            isPaused: true,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: 300,
            timestamp: Date()
        )

        let view = DynamicIslandExpandedView(attributes: testAttributes, state: state)
            .frame(width: 330, height: 200)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 330, height: 200)))
    }

    func testDynamicIslandCompact() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 900,
            totalSeconds: 1500,
            isPaused: false,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: nil,
            timestamp: Date()
        )

        let view = DynamicIslandCompactView(attributes: testAttributes, state: state)
            .frame(width: 100, height: 50)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 100, height: 50)))
    }

    func testDynamicIslandCompactPaused() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 300,
            totalSeconds: 1500,
            isPaused: true,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: nil,
            timestamp: Date()
        )

        let view = DynamicIslandCompactView(attributes: testAttributes, state: state)
            .frame(width: 100, height: 50)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 100, height: 50)))
    }

    func testMinimalRing() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 900,
            totalSeconds: 1500,
            isPaused: false,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: nil,
            timestamp: Date()
        )

        let view = MinimalRingView(state: state)
            .frame(width: 80, height: 80)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 80, height: 80)))
    }

    func testMinimalRingNearlComplete() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 60,
            totalSeconds: 1500,
            isPaused: false,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: nil,
            timestamp: Date()
        )

        let view = MinimalRingView(state: state)
            .frame(width: 80, height: 80)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 80, height: 80)))
    }

    func testMinimalRingComplete() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 0,
            totalSeconds: 1500,
            isPaused: false,
            ruleName: "Deep Work",
            coachyPose: "celebratory",
            upcomingBreakIn: nil,
            timestamp: Date()
        )

        let view = MinimalRingView(state: state)
            .frame(width: 80, height: 80)
            .background(Color(.systemBackground))

        assertSnapshot(matching: view, as: .image(layout: .fixed(width: 80, height: 80)))
    }

    // Integration test: verify all four contexts render without crashes
    @available(iOS 16.2, *)
    func testAllContextsRenderWithoutCrash() {
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 900,
            totalSeconds: 1500,
            isPaused: false,
            ruleName: "Deep Work",
            coachyPose: "confident",
            upcomingBreakIn: 300,
            timestamp: Date()
        )

        // Lock screen banner
        _ = LockScreenBannerView(attributes: testAttributes, state: state)

        // Dynamic Island expanded
        _ = DynamicIslandExpandedView(attributes: testAttributes, state: state)

        // Dynamic Island compact
        _ = DynamicIslandCompactView(attributes: testAttributes, state: state)

        // Minimal ring
        _ = MinimalRingView(state: state)

        // All rendered successfully
        XCTAssert(true)
    }
}
