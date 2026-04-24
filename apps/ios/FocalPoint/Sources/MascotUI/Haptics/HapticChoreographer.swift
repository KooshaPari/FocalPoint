import UIKit

/// Haptic choreography: named patterns for screen interactions.
class HapticChoreographer {
    static let shared = HapticChoreographer()

    private let impactGen = UIImpactFeedbackGenerator()
    private let notificationGen = UINotificationFeedbackGenerator()

    func play(_ pattern: HapticPattern) {
        switch pattern {
        case .light:
            let gen = UIImpactFeedbackGenerator(style: .light)
            gen.impactOccurred()
        case .medium:
            let gen = UIImpactFeedbackGenerator(style: .medium)
            gen.impactOccurred()
        case .heavy:
            let gen = UIImpactFeedbackGenerator(style: .heavy)
            gen.impactOccurred()
        case .celebrate:
            // Medium, light, light sequence
            let gen = UIImpactFeedbackGenerator(style: .medium)
            gen.impactOccurred()
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.15) {
                UIImpactFeedbackGenerator(style: .light).impactOccurred()
            }
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {
                UIImpactFeedbackGenerator(style: .light).impactOccurred()
            }
        case .warn:
            notificationGen.notificationOccurred(.warning)
        case .tripleTap:
            for i in 0..<3 {
                DispatchQueue.main.asyncAfter(deadline: .now() + Double(i) * 0.1) {
                    UIImpactFeedbackGenerator(style: .light).impactOccurred()
                }
            }
        case .success:
            notificationGen.notificationOccurred(.success)
        case .error:
            notificationGen.notificationOccurred(.error)
        }
    }
}

enum HapticPattern {
    case light
    case medium
    case heavy
    case celebrate // medium + light + light
    case warn
    case tripleTap
    case success
    case error
}
