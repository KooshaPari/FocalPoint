import SwiftUI
import UIKit

/// Rule-fired fly-in: Coachy celebration overlay triggered by rule.fired audit records.
class RuleFiredFlyInDispatcher {
    static let shared = RuleFiredFlyInDispatcher()

    /// Dispatch a rule-fired celebration fly-in.
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

        // Create the fly-in scene
        let scene = CoachyScene(
            pose: .celebratory,
            emotion: .excited,
            accessories: [],
            bubbleText: nil,
            soundCueId: "rule-fire-whoosh",
            hapticPattern: .mediumTap,
            particleSystems: [.confetti(count: 12)],
            entry: .flyIn(from: .right, duration: 0.4, easing: .easeOut),
            hold: 0.8,
            exit: .flyOut(to: .right, duration: 0.3),
            onComplete: completion
        )

        // Wrap in a UIHostingController and present
        let sceneView = CoachySceneView(scene: scene)
            .environmentObject(CoachyScenePresenter())

        let hostingController = UIHostingController(rootViewController: UIViewController()) {
            sceneView
                .frame(maxWidth: .infinity, maxHeight: .infinity)
        }

        hostingController.view.backgroundColor = .clear

        // Add to window as overlay
        window.addSubview(hostingController.view)
        hostingController.view.frame = window.bounds

        // Remove after hold + exit duration
        let totalDuration = scene.hold + 0.3 // exit duration
        DispatchQueue.main.asyncAfter(deadline: .now() + totalDuration) {
            hostingController.view.removeFromSuperview()
            hostingController.removeFromParent()
            completion()
        }
    }
}

/// Observation point: Listen for rule.fired audit records and trigger fly-in.
/// This would be wired into your NotificationDispatcher or AuditStore observation.
struct RuleFiredObserver {
    static func observeAndDispatch(
        auditStore: AuditStore,
        windowProvider: @escaping () -> UIWindow?
    ) {
        // In a real implementation, observe auditStore for new audit records
        // and filter for rule.fired events.
        // For Tier 0, this is a proof-of-concept wired manually.
    }

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
