#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

struct HomeView: View {
    @State private var activeRule: ActiveRule? = ActiveRule(
        id: RuleId("demo"),
        title: "Deep work — no social",
        endsAt: Date().addingTimeInterval(45 * 60)
    )

    /// Cycles Coachy through all poses on launch so the device-demo showcases
    /// every rendering. Each pose holds for ~2.5s. Loops forever.
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
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    ruleCard

                    CoachyView(state: coachyState, size: 220)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(
                            RoundedRectangle(cornerRadius: 20, style: .continuous)
                                .fill(Color.app.surface)
                        )

                    statsStrip
                }
                .padding()
            }
            .navigationTitle("FocalPoint")
            .background(Color.app.background.ignoresSafeArea())
            .onAppear(perform: startCoachyCycle)
        }
    }

    private func startCoachyCycle() {
        Task { @MainActor in
            while true {
                let (p, e, b) = demoPoses[poseIndex % demoPoses.count]
                withAnimation(.easeInOut(duration: 0.6)) {
                    coachyState = CoachyState(pose: p, emotion: e, bubbleText: b)
                }
                poseIndex += 1
                try? await Task.sleep(nanoseconds: UInt64(cycleInterval * 1_000_000_000))
            }
        }
    }

    @ViewBuilder
    private var ruleCard: some View {
        if let rule = activeRule {
            VStack(alignment: .leading, spacing: 8) {
                Text("Active rule")
                    .font(.caption.weight(.semibold))
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                Text(rule.title)
                    .font(.title3.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)
                if let endsAt = rule.endsAt {
                    HStack(spacing: 6) {
                        Image(systemName: "timer")
                        Text("Ends \(endsAt.formatted(date: .omitted, time: .shortened))")
                    }
                    .font(.subheadline.weight(.medium))
                    .foregroundStyle(Color.app.accent)
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(18)
            .background(
                RoundedRectangle(cornerRadius: 20, style: .continuous)
                    .fill(Color.app.surface)
            )
            .overlay(
                RoundedRectangle(cornerRadius: 20, style: .continuous)
                    .strokeBorder(Color.app.accent.opacity(0.25), lineWidth: 1)
            )
        } else {
            Text("No active rule")
                .foregroundStyle(Color.app.foreground.opacity(0.7))
        }
    }

    private var statsStrip: some View {
        HStack(spacing: 12) {
            statChip(icon: "flame.fill", label: "Streak", value: "7d")
            statChip(icon: "diamond.fill", label: "Credits", value: "42")
            statChip(icon: "lock.shield.fill", label: "Bypass", value: "2")
        }
    }

    private func statChip(icon: String, label: String, value: String) -> some View {
        VStack(spacing: 4) {
            Image(systemName: icon)
                .font(.system(size: 18))
                .foregroundStyle(Color.app.accent)
            Text(value)
                .font(.title3.weight(.bold))
                .foregroundStyle(Color.app.foreground)
            Text(label)
                .font(.caption)
                .foregroundStyle(Color.app.foreground.opacity(0.7))
        }
        .frame(maxWidth: .infinity)
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }
}
#endif
