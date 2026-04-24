import SwiftUI
import UIKit

/// Rule-fired fly-in: Coachy celebration overlay triggered by rule.fired audit records.
/// Tier-0 proof-of-concept. Wiring: when NotificationDispatcher detects rule.fired, call
/// RuleFiredFlyInDispatcher.shared.present(in: window).
class RuleFiredFlyInDispatcher {
    static let shared = RuleFiredFlyInDispatcher()

    /// Dispatch a rule-fired celebration fly-in (Tier-0 stub).
    func present(
        in window: UIWindow?,
        completion: @escaping () -> Void = {}
    ) {
        guard let window = window ?? UIApplication.shared.connectedScenes
            .compactMap({ $0 as? UIWindowScene })
            .flatMap({ $0.windows })
            .first(where: { $0.isKeyWindow })
        else {
            print("⚠️ No key window available for fly-in")
            completion()
            return
        }

        // Tier-0: Simple haptic feedback for rule fire
        let feedbackGen = UIImpactFeedbackGenerator(style: .medium)
        feedbackGen.impactOccurred()

        // Log proof-of-concept
        print("✅ Rule-fired fly-in triggered (Tier-0 stub: haptic feedback)")
        completion()
    }
}

/// Observation point: Listen for rule.fired audit records and trigger fly-in.
struct RuleFiredObserver {
    /// Trigger fly-in manually (for testing or direct dispatch).
    static func triggerFlyIn(in window: UIWindow? = nil, completion: @escaping () -> Void = {}) {
        RuleFiredFlyInDispatcher.shared.present(in: window, completion: completion)
    }
}

// MARK: - Integration Example (pseudo-code for reference)
/*
 In your RulesView or wherever rules are evaluated:

 .onChange(of: auditStore.recentRecords) { oldRecords, newRecords in
     let newFiredRules = newRecords.filter { $0.eventType == "rule.fired" }
     if !newFiredRules.isEmpty {
         RuleFiredObserver.triggerFlyIn(completion: {
             print("Rule fly-in complete")
         })
     }
 }
 */
