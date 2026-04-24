#if canImport(SwiftUI)
import Foundation

/// Tracks per-step completion in onboarding via @AppStorage.
/// Every page writes its step index on .onAppear to enable
/// resume flows and progress tracking.
public struct OnboardingResumeState {
    private static let stepKeyPrefix = "app.onboarding.step."

    /// Get the current step index (0-based). Returns -1 if never started.
    public static func getCurrentStepIndex() -> Int {
        let userDefaults = UserDefaults.standard
        // Sum all completed steps to determine current progress
        var maxStep = -1
        for step in 0...6 { // 0=consent, 1=welcome, ..., 6=done
            if userDefaults.bool(forKey: stepKeyPrefix + String(step)) {
                maxStep = step
            }
        }
        return maxStep
    }

    /// Mark a step as completed.
    public static func completeStep(_ stepIndex: Int) {
        let userDefaults = UserDefaults.standard
        userDefaults.set(true, forKey: stepKeyPrefix + String(stepIndex))
    }

    /// Reset all step tracking (called when starting over).
    public static func resetTracking() {
        let userDefaults = UserDefaults.standard
        for step in 0...6 {
            userDefaults.removeObject(forKey: stepKeyPrefix + String(step))
        }
    }

    /// Get progress label like "3/6 steps complete".
    public static func getProgressLabel() -> String {
        let currentStep = getCurrentStepIndex()
        let totalSteps = 6 // consent through permissions (0-5), done is 6
        if currentStep < 0 {
            return "Ready to start"
        }
        return "\(currentStep + 1)/\(totalSteps) steps complete"
    }

    /// Check if user has made partial progress (started but not finished).
    public static func hasPartialProgress() -> Bool {
        let currentStep = getCurrentStepIndex()
        return currentStep >= 0 && currentStep < 6
    }
}

#endif
