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
        } dynamicIsland: { _ in
            // Dynamic Island support (iOS 16.1+)
            // Expanded and compact views auto-generated from lock screen content above
            EmptyView()
        }
    }
}
