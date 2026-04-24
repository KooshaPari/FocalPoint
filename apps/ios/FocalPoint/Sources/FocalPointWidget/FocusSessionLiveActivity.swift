import ActivityKit
import SwiftUI
import WidgetKit

// MARK: - Live Activity Attributes & State

/// Attributes passed when the activity is requested (static for the lifetime of the session).
@available(iOS 16.1, *)
public struct FocusSessionAttributes: ActivityAttributes {
    public typealias ContentState = FocusSessionContentState

    public struct FocusSessionContentState: Codable, Hashable {
        public var remainingSeconds: Int
        public var totalSeconds: Int
        public var isPaused: Bool
        public var timestamp: Date

        public init(remainingSeconds: Int, totalSeconds: Int, isPaused: Bool, timestamp: Date = Date()) {
            self.remainingSeconds = remainingSeconds
            self.totalSeconds = totalSeconds
            self.isPaused = isPaused
            self.timestamp = timestamp
        }
    }

    public var sessionTitle: String

    public init(sessionTitle: String = "Focus Session") {
        self.sessionTitle = sessionTitle
    }
}

// MARK: - Live Activity UI

@available(iOS 16.1, *)
struct FocusSessionActivityView: View {
    let attributes: FocusSessionAttributes
    let state: FocusSessionAttributes.ContentState

    var body: some View {
        compactView
    }

    private var compactView: some View {
        VStack(spacing: 4) {
            HStack(spacing: 8) {
                Text("🧘 Focus")
                    .font(.subheadline.weight(.semibold))
                Spacer()
                Text(formatMMSS(state.remainingSeconds))
                    .font(.headline.monospacedDigit())
            }
            if state.isPaused {
                Text("Paused")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .padding(.vertical, 8)
        .padding(.horizontal, 10)
        .background(Color.blue.opacity(0.1))
        .cornerRadius(8)
    }

    private func formatMMSS(_ seconds: Int) -> String {
        let mins = max(0, seconds / 60)
        let secs = max(0, seconds % 60)
        return String(format: "%02d:%02d", mins, secs)
    }
}

// MARK: - Live Activity Definition

@available(iOS 16.1, *)
struct FocusSessionLiveActivity: Widget {
    var body: some WidgetConfiguration {
        ActivityConfiguration(
            for: FocusSessionAttributes.self
        ) { context in
            FocusSessionActivityView(
                attributes: context.attributes,
                state: context.state
            )
        } dynamicIsland: { context in
            DynamicIsland {
                // Expanded state (hold-expanded on Dynamic Island)
                VStack(spacing: 8) {
                    HStack {
                        Text("Focus Session")
                            .font(.headline.weight(.semibold))
                        Spacer()
                        Text(formatMMSS(context.state.remainingSeconds))
                            .font(.title2.monospacedDigit().weight(.bold))
                    }

                    ZStack {
                        Circle()
                            .stroke(Color.gray.opacity(0.3), lineWidth: 4)
                        Circle()
                            .trim(
                                from: 0,
                                to: context.state.totalSeconds > 0
                                    ? 1.0 - Double(context.state.remainingSeconds) / Double(context.state.totalSeconds)
                                    : 0
                            )
                            .stroke(Color.blue, style: StrokeStyle(lineWidth: 4, lineCap: .round))
                            .rotationEffect(.degrees(-90))

                        Text("🧘")
                            .font(.system(size: 20))
                    }
                    .frame(height: 60)

                    Button("Open") {
                        // App intent removed; let system handle tap
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 6)
                    .background(Color.blue)
                    .foregroundStyle(.white)
                    .cornerRadius(6)
                }
                .padding()
            } compactLeading: {
                // Compact leading (left side of Dynamic Island)
                HStack(spacing: 4) {
                    Text("🧘")
                        .font(.system(size: 12))
                    Text(formatMMSS(context.state.remainingSeconds))
                        .font(.caption.monospacedDigit().weight(.semibold))
                }
            } compactTrailing: {
                // Compact trailing (right side of Dynamic Island)
                if context.state.isPaused {
                    Image(systemName: "pause.fill")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                } else {
                    ProgressView(value: progressValue(context.state))
                        .tint(.blue)
                        .frame(width: 30)
                }
            } minimal: {
                // Minimal (when swiped away)
                Text("🧘")
                    .font(.caption)
            }
        }
    }

    private func progressValue(_ state: FocusSessionAttributes.ContentState) -> Double {
        guard state.totalSeconds > 0 else { return 0 }
        return 1.0 - Double(state.remainingSeconds) / Double(state.totalSeconds)
    }

    private func formatMMSS(_ seconds: Int) -> String {
        let mins = max(0, seconds / 60)
        let secs = max(0, seconds % 60)
        return String(format: "%02d:%02d", mins, secs)
    }
}
