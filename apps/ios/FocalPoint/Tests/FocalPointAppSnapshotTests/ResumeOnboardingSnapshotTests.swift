import SwiftUI
import SnapshotTesting
import XCTest
@testable import FocalPointApp

// Traces to: FR-ONBOARD-002 (Onboarding resume flow UI verification)

class ResumeOnboardingSnapshotTests: XCTestCase {
    let record = false

    // MARK: - Fresh Start

    func testResumeOnboarding_freshStart() {
        // Simulate user who never started onboarding
        OnboardingResumeState.resetTracking()

        let view = ResumeOnboardingTestView_FreshStart()

        assertViewSnapshot(
            view: view,
            name: "resume_onboarding_fresh_start",
            record: record
        )
    }

    // MARK: - Partial Progress (3/6)

    func testResumeOnboarding_partial() {
        // Simulate user who completed 3 of 6 steps
        OnboardingResumeState.resetTracking()
        OnboardingResumeState.completeStep(1) // welcome
        OnboardingResumeState.completeStep(2) // goals
        OnboardingResumeState.completeStep(3) // connect

        let view = ResumeOnboardingTestView_Partial()

        assertViewSnapshot(
            view: view,
            name: "resume_onboarding_partial_3_of_6",
            record: record
        )
    }

    // MARK: - Resume In Progress (5/6)

    func testResumeOnboarding_almostDone() {
        // Simulate user who is almost done (5 of 6 steps)
        OnboardingResumeState.resetTracking()
        OnboardingResumeState.completeStep(1) // welcome
        OnboardingResumeState.completeStep(2) // goals
        OnboardingResumeState.completeStep(3) // connect
        OnboardingResumeState.completeStep(4) // template
        OnboardingResumeState.completeStep(5) // permissions

        let view = ResumeOnboardingTestView_AlmostDone()

        assertViewSnapshot(
            view: view,
            name: "resume_onboarding_almost_done_5_of_6",
            record: record
        )
    }

    // MARK: - Test Helpers

    func assertViewSnapshot(
        view: some View,
        name: String,
        record: Bool
    ) {
        let hostingController = UIHostingController(rootView: view)
        hostingController.view.frame = CGRect(x: 0, y: 0, width: 390, height: 844)

        assertSnapshot(
            of: hostingController,
            as: .image,
            named: name,
            record: record
        )
    }
}

// MARK: - Test View Helpers

struct ResumeOnboardingTestView_FreshStart: View {
    @State private var isPresented = true

    var body: some View {
        ZStack {
            Color.black.ignoresSafeArea()

            ResumeOnboardingView(
                isPresented: $isPresented,
                onResume: {},
                onRestart: {}
            )
        }
        .preferredColorScheme(.dark)
    }
}

struct ResumeOnboardingTestView_Partial: View {
    @State private var isPresented = true

    var body: some View {
        ZStack {
            Color.black.ignoresSafeArea()

            ResumeOnboardingView(
                isPresented: $isPresented,
                onResume: {},
                onRestart: {}
            )
        }
        .preferredColorScheme(.dark)
    }
}

struct ResumeOnboardingTestView_AlmostDone: View {
    @State private var isPresented = true

    var body: some View {
        ZStack {
            Color.black.ignoresSafeArea()

            ResumeOnboardingView(
                isPresented: $isPresented,
                onResume: {},
                onRestart: {}
            )
        }
        .preferredColorScheme(.dark)
    }
}
