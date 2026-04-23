#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI

/// Relocated from the original `HomeView` demo. Cycles Coachy through every
/// pose+emotion pairing so we can visually QA the character sheet.
struct DebugCoachyShowcase: View {
    @State private var coachyState: CoachyState = .placeholder
    @State private var poseIndex = 0
    private let demoPoses: [(CoachyPose, CoachyEmotion, String)] = [
        (.confident, .focused, "You can do harder things."),
        (.encouraging, .happy, "You've got this!"),
        (.curious, .neutral, "Let's figure it out."),
        (.stern, .concerned, "Focus. No shortcuts."),
        (.celebratory, .excited, "Task complete! Let's go!"),
        (.sleepy, .tired, "Rest up. Tomorrow's a win."),
        (.idle, .neutral, "Finish one task, earn a break."),
    ]
    private let cycleInterval: TimeInterval = 2.6

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                CoachyView(state: coachyState, size: 240)
                    .padding()
                    .background(
                        RoundedRectangle(cornerRadius: 20, style: .continuous)
                            .fill(Color.app.surface)
                    )
                Text("Pose: \(String(describing: coachyState.pose))")
                    .font(.caption.monospaced())
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
            }
            .padding()
        }
        .onAppear(perform: startCoachyCycle)
    }

    private func startCoachyCycle() {
        Task { @MainActor in
            while !Task.isCancelled {
                let (p, e, b) = demoPoses[poseIndex % demoPoses.count]
                withAnimation(.easeInOut(duration: 0.6)) {
                    coachyState = CoachyState(pose: p, emotion: e, bubbleText: b)
                }
                poseIndex += 1
                try? await Task.sleep(nanoseconds: UInt64(cycleInterval * 1_000_000_000))
            }
        }
    }
}
#endif
