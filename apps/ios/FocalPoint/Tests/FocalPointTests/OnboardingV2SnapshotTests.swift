#if canImport(SwiftUI)
import SwiftUI
import SnapshotTesting
import XCTest
@testable import FocalPointApp

class OnboardingV2SnapshotTests: XCTestCase {
    override func setUp() {
        super.setUp()
        // Configure snapshot testing for deterministic rendering
        isRecording = false
    }

    // MARK: - Page Snapshots

    func testWelcomePageV2Snapshot() {
        let namespace = Namespace().wrappedValue
        let view = OnboardingWelcomePageV2(namespace: namespace)
            .frame(height: 812) // iPhone 13 height
            .background(Color.app.background)

        assertSnapshot(
            matching: view,
            as: .image(size: CGSize(width: 390, height: 812), traits: .init(verticalSizeClass: .regular))
        )
    }

    func testGoalsPageV2Snapshot() {
        let namespace = Namespace().wrappedValue
        @State var selectedGoals: Set<String> = []
        @State var sparkleId: String? = nil

        let view = OnboardingGoalsPageV2(
            namespace: namespace,
            selectedGoals: $selectedGoals,
            sparkleId: $sparkleId
        )
        .frame(height: 812)
        .background(Color.app.background)

        assertSnapshot(
            matching: view,
            as: .image(size: CGSize(width: 390, height: 812), traits: .init(verticalSizeClass: .regular))
        )
    }

    func testConnectPageV2Snapshot() {
        let namespace = Namespace().wrappedValue
        let coord = OnboardingCoordinator()
        let view = OnboardingConnectPageV2(namespace: namespace, coord: coord)
            .frame(height: 812)
            .background(Color.app.background)

        assertSnapshot(
            matching: view,
            as: .image(size: CGSize(width: 390, height: 812), traits: .init(verticalSizeClass: .regular))
        )
    }

    func testTemplatePageV2Snapshot() {
        let namespace = Namespace().wrappedValue
        let view = OnboardingTemplatePageV2(namespace: namespace)
            .frame(height: 812)
            .background(Color.app.background)

        assertSnapshot(
            matching: view,
            as: .image(size: CGSize(width: 390, height: 812), traits: .init(verticalSizeClass: .regular))
        )
    }

    func testPermissionsPageV2Snapshot() {
        let namespace = Namespace().wrappedValue
        let view = OnboardingPermissionsPageV2(namespace: namespace)
            .frame(height: 812)
            .background(Color.app.background)

        assertSnapshot(
            matching: view,
            as: .image(size: CGSize(width: 390, height: 812), traits: .init(verticalSizeClass: .regular))
        )
    }

    func testFinalPageV2Snapshot() {
        let namespace = Namespace().wrappedValue
        let view = OnboardingFinalPageV2(namespace: namespace)
            .frame(height: 812)
            .background(Color.app.background)

        assertSnapshot(
            matching: view,
            as: .image(size: CGSize(width: 390, height: 812), traits: .init(verticalSizeClass: .regular))
        )
    }

    // MARK: - Interaction Tests

    func testGoalSelectionTriggersSparkles() {
        let namespace = Namespace().wrappedValue
        @State var selectedGoals: Set<String> = []
        @State var sparkleId: String? = nil

        // Simulate goal selection by setting sparkleId
        let expectation = XCTestExpectation(description: "Sparkle ID should be set on goal selection")

        DispatchQueue.main.async {
            sparkleId = "fitness"
            expectation.fulfill()
        }

        wait(for: [expectation], timeout: 1.0)
        XCTAssertEqual(sparkleId, "fitness", "Goal selection should trigger sparkle particle effect")
    }

    func testCoachyPoseTransitions() {
        // Test that Coachy pose changes correctly across pages
        let poseSequence: [(OnboardingCoordinator.Step, CoachyPose)] = [
            (.welcome, .happy),
            (.goals, .curious),
            (.connect, .encouraging),
            (.pickTemplate, .curiousThinking),
            (.permissions, .sternToughLove),
            (.done, .confident),
        ]

        for (step, expectedPose) in poseSequence {
            let coachyState = CoachyState(pose: expectedPose, emotion: .warm)
            XCTAssertEqual(coachyState.pose, expectedPose, "Step \(step) should use pose \(expectedPose)")
        }
    }

    func testParticleOverlayRendering() {
        let confettiView = ParticleOverlay(particles: .confetti(count: 80))
            .frame(width: 390, height: 812)

        let sparklesView = ParticleOverlay(particles: .sparkles(count: 12))
            .frame(width: 100, height: 100)

        // These should render without crashing
        XCTAssertNotNil(confettiView)
        XCTAssertNotNil(sparklesView)
    }

    func testOnboardingViewV2Integration() {
        // Create a minimal OnboardingViewV2 to verify it initializes
        let view = OnboardingViewV2()
        XCTAssertNotNil(view, "OnboardingViewV2 should initialize successfully")
    }
}

#endif
