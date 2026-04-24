#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Dismissible banner displaying proactive nudge proposals from the always-on engine.
/// Renders at top of RootTabView when `pendingNudges` is non-empty. Coachy pose
/// adapts based on nudge kind. Tapping navigates to the relevant tab.
struct NudgeBannerView: View {
    @EnvironmentObject private var holder: CoreHolder
    @State private var selectedNudgeIndex: Int = 0

    var body: some View {
        if !holder.pendingNudges.isEmpty {
            let nudge = holder.pendingNudges[selectedNudgeIndex]
            VStack(spacing: 12) {
                HStack(spacing: 12) {
                    // Coachy mascot with pose adapted to nudge kind.
                    CoachyView(state: coachyStateFor(nudge))
                        .frame(width: 44, height: 44)
                        .clipShape(Circle())

                    VStack(alignment: .leading, spacing: 4) {
                        Text(titleFor(nudge.kind))
                            .font(.system(.headline, design: .default))
                            .foregroundColor(.app.text.primary)

                        Text(nudge.reason)
                            .font(.system(.caption, design: .default))
                            .foregroundColor(.app.text.secondary)
                            .lineLimit(2)
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)

                    VStack(spacing: 8) {
                        Button(action: { dismissCurrentNudge() }) {
                            Image(systemName: "xmark")
                                .font(.system(size: 12, weight: .semibold))
                                .foregroundColor(.app.text.secondary)
                                .frame(width: 24, height: 24)
                        }
                        .contentShape(Circle())

                        if holder.pendingNudges.count > 1 {
                            Text("\(selectedNudgeIndex + 1)/\(holder.pendingNudges.count)")
                                .font(.system(.caption2, design: .monospaced))
                                .foregroundColor(.app.text.secondary)
                        }
                    }
                }
                .padding(12)
                .background(Color.app.surface.secondary)
                .cornerRadius(12)
                .onTapGesture {
                    navigateTo(nudge.kind)
                }
            }
            .padding(.horizontal)
            .padding(.vertical, 8)
            .background(Color.app.background.ignoresSafeArea(edges: .top))
        }
    }

    private func coachyStateFor(_ nudge: NudgeProposalDto) -> CoachyState {
        let (pose, emotion) = poseAndEmotionFor(nudge.kind)
        return CoachyState(
            pose: pose,
            emotion: emotion,
            bubbleText: nil
        )
    }

    private func poseAndEmotionFor(_ kind: NudgeKindDto) -> (CoachyPose, CoachyEmotion) {
        switch kind {
        case .startFocus:
            return (.confident, .excited)
        case .takeBreak:
            return (.encouraging, .warm)
        case .reviewDeadline:
            return (.curiousThinking, .concerned)
        case .streakAtRisk:
            return (.sternToughLove, .stern)
        case .windDown:
            return (.sleepyDisappointed, .tired)
        }
    }

    private func titleFor(_ kind: NudgeKindDto) -> String {
        switch kind {
        case .startFocus:
            return "Time to Focus"
        case .takeBreak:
            return "Take a Break"
        case .reviewDeadline:
            return "Upcoming Deadline"
        case .streakAtRisk:
            return "Protect Your Streak"
        case .windDown:
            return "Wind Down"
        }
    }

    private func navigateTo(_ kind: NudgeKindDto) {
        switch kind {
        case .startFocus:
            // Navigate to Focus tab (index 1 in RootTabView).
            // TODO: Implement tab selection via coordinator or deep link.
            break
        case .takeBreak:
            // Navigate to Home tab (index 3).
            break
        case .reviewDeadline:
            // Navigate to Tasks tab (index 2).
            break
        case .streakAtRisk:
            // Navigate to Rewards tab (index 5) to show streak status.
            break
        case .windDown:
            // Navigate to Settings sleep section.
            // TODO: Implement settings deep link.
            break
        }
    }

    private func dismissCurrentNudge() {
        holder.pendingNudges.remove(at: selectedNudgeIndex)
        if holder.pendingNudges.isEmpty {
            selectedNudgeIndex = 0
        } else if selectedNudgeIndex >= holder.pendingNudges.count {
            selectedNudgeIndex = holder.pendingNudges.count - 1
        }
    }
}

#Preview {
    ZStack {
        Color.app.background.ignoresSafeArea()
        VStack {
            NudgeBannerView()
                .environmentObject(CoreHolder.shared)
            Spacer()
        }
    }
    .preferredColorScheme(.dark)
}
#endif
