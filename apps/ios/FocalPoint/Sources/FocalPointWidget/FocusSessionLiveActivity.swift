import ActivityKit
import AppIntents
import SwiftUI
import WidgetKit

// MARK: - App Intents for iOS 17+

@available(iOS 17, *)
struct FocusPauseIntent: AppIntent {
    static let title: LocalizedStringResource = "Pause focus session"
    static let description: LocalizedStringResource = "Pause the current focus session"
    static let openAppWhenRun: Bool = true

    func perform() async throws -> some IntentResult {
        NotificationCenter.default.post(name: NSNotification.Name("FocusSessionPause"), object: nil)
        return .result()
    }
}

@available(iOS 17, *)
struct FocusResumeIntent: AppIntent {
    static let title: LocalizedStringResource = "Resume focus session"
    static let description: LocalizedStringResource = "Resume the current focus session"
    static let openAppWhenRun: Bool = true

    func perform() async throws -> some IntentResult {
        NotificationCenter.default.post(name: NSNotification.Name("FocusSessionResume"), object: nil)
        return .result()
    }
}

@available(iOS 17, *)
struct FocusCancelIntent: AppIntent {
    static let title: LocalizedStringResource = "Cancel focus session"
    static let description: LocalizedStringResource = "Cancel the current focus session"
    static let openAppWhenRun: Bool = true

    func perform() async throws -> some IntentResult {
        NotificationCenter.default.post(name: NSNotification.Name("FocusSessionCancel"), object: nil)
        return .result()
    }
}

// MARK: - Live Activity Attributes & State

/// Attributes passed when the activity is requested (static for the lifetime of the session).
@available(iOS 16.1, *)
public struct FocusSessionAttributes: ActivityAttributes {
    public typealias ContentState = FocusSessionContentState

    public struct FocusSessionContentState: Codable, Hashable {
        public var remainingSeconds: Int
        public var totalSeconds: Int
        public var isPaused: Bool
        public var ruleName: String
        public var coachyPose: String // e.g. "confident", "celebratory", "neutral"
        public var upcomingBreakIn: Int? // seconds until next break, if applicable
        public var timestamp: Date

        public init(
            remainingSeconds: Int,
            totalSeconds: Int,
            isPaused: Bool,
            ruleName: String = "Deep Work",
            coachyPose: String = "confident",
            upcomingBreakIn: Int? = nil,
            timestamp: Date = Date()
        ) {
            self.remainingSeconds = remainingSeconds
            self.totalSeconds = totalSeconds
            self.isPaused = isPaused
            self.ruleName = ruleName
            self.coachyPose = coachyPose
            self.upcomingBreakIn = upcomingBreakIn
            self.timestamp = timestamp
        }
    }

    public var sessionTitle: String
    public var startedAt: Date
    public var plannedDuration: Int // seconds
    public var breakInterval: Int? // seconds, optional
    public var bgTint: String // hex or system color name
    public var coachyEmoji: String // e.g. "🧘", "🚀", "💪"

    public init(
        sessionTitle: String = "Focus Session",
        startedAt: Date = Date(),
        plannedDuration: Int = 1500,
        breakInterval: Int? = nil,
        bgTint: String = "blue",
        coachyEmoji: String = "🧘"
    ) {
        self.sessionTitle = sessionTitle
        self.startedAt = startedAt
        self.plannedDuration = plannedDuration
        self.breakInterval = breakInterval
        self.bgTint = bgTint
        self.coachyEmoji = coachyEmoji
    }
}

// MARK: - Lock Screen Banner View

@available(iOS 16.1, *)
private struct LockScreenBannerView: View {
    let attributes: FocusSessionAttributes
    let state: FocusSessionAttributes.ContentState

    var body: some View {
        HStack(spacing: 10) {
            Text(attributes.coachyEmoji)
                .font(.title2)

            VStack(alignment: .leading, spacing: 2) {
                Text(attributes.sessionTitle)
                    .font(.subheadline.weight(.semibold))
                    .foregroundStyle(.primary)

                HStack(spacing: 8) {
                    Text(formatMMSS(state.remainingSeconds))
                        .font(.caption.monospacedDigit().weight(.semibold))
                        .foregroundStyle(.primary)

                    if state.isPaused {
                        Text("Paused")
                            .font(.caption.weight(.medium))
                            .foregroundStyle(.secondary)
                    }
                }
            }

            Spacer()

            ProgressView(value: Double(state.totalSeconds - state.remainingSeconds), total: Double(state.totalSeconds))
                .frame(width: 40)
        }
        .padding(.vertical, 10)
        .padding(.horizontal, 12)
        .background(Color.blue.opacity(0.08))
        .cornerRadius(10)
    }

    private func formatMMSS(_ seconds: Int) -> String {
        let mins = max(0, seconds / 60)
        let secs = max(0, seconds % 60)
        return String(format: "%02d:%02d", mins, secs)
    }
}

// MARK: - Dynamic Island Compact View (Left/Right)

@available(iOS 16.1, *)
private struct DynamicIslandCompactView: View {
    let attributes: FocusSessionAttributes
    let state: FocusSessionAttributes.ContentState

    var body: some View {
        HStack(spacing: 6) {
            Text(attributes.coachyEmoji)
                .font(.headline)

            VStack(alignment: .leading, spacing: 1) {
                Text(formatMinutes(state.remainingSeconds))
                    .font(.caption.monospacedDigit().weight(.semibold))
                    .foregroundStyle(.primary)

                if state.isPaused {
                    Text("Paused")
                        .font(.system(size: 9, weight: .medium, design: .default))
                        .foregroundStyle(.secondary)
                }
            }
        }
        .lineLimit(1)
    }

    private func formatMinutes(_ seconds: Int) -> String {
        let mins = max(0, seconds / 60)
        return "\(mins)m"
    }
}

// MARK: - Dynamic Island Expanded View

@available(iOS 16.1, *)
private struct DynamicIslandExpandedView: View {
    let attributes: FocusSessionAttributes
    let state: FocusSessionAttributes.ContentState

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                Text(attributes.coachyEmoji)
                    .font(.title)
                Text(attributes.sessionTitle)
                    .font(.headline.weight(.semibold))
                Spacer()
            }

            VStack(spacing: 8) {
                // Timer
                HStack(spacing: 12) {
                    VStack(alignment: .leading, spacing: 2) {
                        Text("Remaining")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Text(formatMMSS(state.remainingSeconds))
                            .font(.system(size: 24, weight: .semibold, design: .monospaced))
                            .foregroundStyle(.primary)
                    }

                    Spacer()

                    // Progress ring
                    ZStack {
                        Circle()
                            .stroke(Color.gray.opacity(0.2), lineWidth: 2)
                        Circle()
                            .trim(from: 0, to: max(0, min(1, Double(state.totalSeconds - state.remainingSeconds) / Double(state.totalSeconds))))
                            .stroke(Color.blue, style: StrokeStyle(lineWidth: 2, lineCap: .round))
                            .rotationEffect(.degrees(-90))

                        Text(formatProgress(state.remainingSeconds, total: state.totalSeconds))
                            .font(.caption.weight(.semibold))
                            .foregroundStyle(.secondary)
                    }
                    .frame(width: 50, height: 50)
                }

                // Break info if applicable
                if let breakIn = state.upcomingBreakIn {
                    HStack(spacing: 8) {
                        Image(systemName: "clock.badge")
                            .font(.caption)
                        Text("Break in \(formatMinutes(breakIn))")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }

                // Control buttons (iOS 17+)
                if #available(iOS 17, *) {
                    HStack(spacing: 8) {
                        if state.isPaused {
                            Button(intent: FocusResumeIntent()) {
                                Label("Resume", systemImage: "play.fill")
                                    .font(.caption.weight(.semibold))
                            }
                            .buttonStyle(.bordered)
                        } else {
                            Button(intent: FocusPauseIntent()) {
                                Label("Pause", systemImage: "pause.fill")
                                    .font(.caption.weight(.semibold))
                            }
                            .buttonStyle(.bordered)
                        }

                        Button(intent: FocusCancelIntent(), role: .destructive) {
                            Label("Cancel", systemImage: "xmark")
                                .font(.caption.weight(.semibold))
                        }
                        .buttonStyle(.bordered)
                    }
                }
            }
        }
        .padding(.vertical, 8)
    }

    private func formatMMSS(_ seconds: Int) -> String {
        let mins = max(0, seconds / 60)
        let secs = max(0, seconds % 60)
        return String(format: "%02d:%02d", mins, secs)
    }

    private func formatMinutes(_ seconds: Int) -> String {
        let mins = max(0, seconds / 60)
        return "\(mins)m"
    }

    private func formatProgress(_ remaining: Int, total: Int) -> String {
        let percent = total > 0 ? ((total - remaining) * 100) / total : 0
        return "\(percent)%"
    }
}

// MARK: - Minimal Ring View (Lockscreen minimal presentation)

@available(iOS 16.1, *)
private struct MinimalRingView: View {
    let state: FocusSessionAttributes.ContentState

    var body: some View {
        ZStack {
            Circle()
                .stroke(Color.gray.opacity(0.3), lineWidth: 3)

            Circle()
                .trim(from: 0, to: max(0, min(1, Double(state.totalSeconds - state.remainingSeconds) / Double(state.totalSeconds))))
                .stroke(Color.blue, style: StrokeStyle(lineWidth: 3, lineCap: .round))
                .rotationEffect(.degrees(-90))

            Text("\(max(0, state.remainingSeconds / 60))")
                .font(.system(size: 14, weight: .semibold, design: .monospaced))
                .foregroundStyle(.primary)
        }
        .frame(width: 60, height: 60)
    }
}

// MARK: - Live Activity Definition

@available(iOS 16.1, *)
struct FocusSessionLiveActivity: Widget {
    var body: some WidgetConfiguration {
        ActivityConfiguration(
            for: FocusSessionAttributes.self
        ) { context in
            // Lock screen (banner + minimal ring)
            VStack(spacing: 12) {
                LockScreenBannerView(
                    attributes: context.attributes,
                    state: context.state
                )

                MinimalRingView(state: context.state)
            }
            .padding()
        } dynamicIsland: { context in
            // Dynamic Island: compact + expanded presentations (iOS 16.1+)
            if #available(iOS 16.2, *) {
                DynamicIsland {
                    // Expanded
                    DynamicIslandExpandedView(
                        attributes: context.attributes,
                        state: context.state
                    )
                } compactLeading: {
                    // Compact leading
                    DynamicIslandCompactView(
                        attributes: context.attributes,
                        state: context.state
                    )
                } compactTrailing: {
                    // Compact trailing
                    MinimalRingView(state: context.state)
                } minimal: {
                    // Minimal (pill)
                    Text(String(format: "%dm", max(0, context.state.remainingSeconds / 60)))
                        .font(.caption.monospacedDigit().weight(.semibold))
                }
            }
        }
    }
}
