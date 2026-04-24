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

// MARK: - Live Activity Provider

@available(iOS 16.1, *)
struct FocusSessionActivityProvider: TimelineProvider {
    typealias Entry = ActivityEntry<FocusSessionAttributes>

    func placeholder(in context: Context) -> ActivityEntry<FocusSessionAttributes> {
        let attrs = FocusSessionAttributes(sessionTitle: "Focus")
        let state = FocusSessionAttributes.ContentState(
            remainingSeconds: 1500,
            totalSeconds: 1500,
            isPaused: false
        )
        return ActivityEntry(attributes: attrs, contentState: state, timedOut: false)
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<ActivityEntry<FocusSessionAttributes>>) -> Void) {
        // Placeholder; the app will push updates via ActivityKit APIs.
        let entry = placeholder(in: context)
        let timeline = Timeline(entries: [entry], policy: .never)
        completion(timeline)
    }
}

// MARK: - Live Activity UI

@available(iOS 16.1, *)
struct FocusSessionActivityView: View {
    let attributes: FocusSessionAttributes
    let state: FocusSessionAttributes.ContentState

    @Environment(\.activityFamily) var family
    @Environment(\.scenePhase) var scenePhase

    var body: some View {
        switch family {
        case .default:
            compactView
        case .expanded:
            expandedView
        @unknown default:
            compactView
        }
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

    private var expandedView: some View {
        VStack(spacing: 12) {
            HStack {
                VStack(alignment: .leading, spacing: 2) {
                    Text("Focus Session")
                        .font(.headline.weight(.semibold))
                    Text(attributes.sessionTitle)
                        .font(.subheadline)
                        .foregroundStyle(.secondary)
                }
                Spacer()
                Text(formatMMSS(state.remainingSeconds))
                    .font(.title2.monospacedDigit().weight(.bold))
            }

            // Progress ring
            ZStack {
                Circle()
                    .stroke(Color.gray.opacity(0.3), lineWidth: 8)
                Circle()
                    .trim(
                        from: 0,
                        to: state.totalSeconds > 0
                            ? 1.0 - Double(state.remainingSeconds) / Double(state.totalSeconds)
                            : 0
                    )
                    .stroke(Color.blue, style: StrokeStyle(lineWidth: 8, lineCap: .round))
                    .rotationEffect(.degrees(-90))

                VStack(spacing: 4) {
                    Text("🧘")
                        .font(.system(size: 32))
                    if state.isPaused {
                        Text("Paused")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }
            }
            .frame(height: 100)

            Button(intent: OpenFocalPointAppIntent()) {
                Label("Return to app", systemImage: "arrow.right")
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 8)
                    .background(Color.blue)
                    .foregroundStyle(.white)
                    .cornerRadius(8)
            }
            .buttonStyle(.plain)
        }
        .padding()
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
            DynamicIslandContent(
                attributes: context.attributes,
                state: context.state
            )
        }
        .supplementalActivityFamilies([.small])
    }
}

// MARK: - Dynamic Island Presentation

@available(iOS 16.1, *)
struct DynamicIslandContent: View {
    let attributes: FocusSessionAttributes
    let state: FocusSessionAttributes.ContentState

    var body: some View {
        DynamicIsland {
            // Expanded state (hold-expanded on Dynamic Island)
            VStack(spacing: 8) {
                HStack {
                    Text("Focus Session")
                        .font(.headline.weight(.semibold))
                    Spacer()
                    Text(formatMMSS(state.remainingSeconds))
                        .font(.title2.monospacedDigit().weight(.bold))
                }

                ZStack {
                    Circle()
                        .stroke(Color.gray.opacity(0.3), lineWidth: 4)
                    Circle()
                        .trim(
                            from: 0,
                            to: state.totalSeconds > 0
                                ? 1.0 - Double(state.remainingSeconds) / Double(state.totalSeconds)
                                : 0
                        )
                        .stroke(Color.blue, style: StrokeStyle(lineWidth: 4, lineCap: .round))
                        .rotationEffect(.degrees(-90))

                    Text("🧘")
                        .font(.system(size: 20))
                }
                .frame(height: 60)

                Button(intent: OpenFocalPointAppIntent()) {
                    Text("Open")
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 6)
                        .background(Color.blue)
                        .foregroundStyle(.white)
                        .cornerRadius(6)
                }
                .buttonStyle(.plain)
            }
            .padding()
        } compactLeading: {
            // Compact leading (left side of Dynamic Island)
            HStack(spacing: 4) {
                Text("🧘")
                    .font(.system(size: 12))
                Text(formatMMSS(state.remainingSeconds))
                    .font(.caption.monospacedDigit().weight(.semibold))
            }
        } compactTrailing: {
            // Compact trailing (right side of Dynamic Island)
            if state.isPaused {
                Image(systemName: "pause.fill")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } else {
                ProgressView(value: progressValue())
                    .tint(.blue)
                    .frame(width: 30)
            }
        } minimal: {
            // Minimal (when swiped away)
            Text("🧘")
                .font(.caption)
        }
    }

    private func progressValue() -> Double {
        guard state.totalSeconds > 0 else { return 0 }
        return 1.0 - Double(state.remainingSeconds) / Double(state.totalSeconds)
    }

    private func formatMMSS(_ seconds: Int) -> String {
        let mins = max(0, seconds / 60)
        let secs = max(0, seconds % 60)
        return String(format: "%02d:%02d", mins, secs)
    }
}

// MARK: - App Intent (Return to App)

@available(iOS 16.1, *)
struct OpenFocalPointAppIntent: AppIntent {
    static var title: LocalizedStringResource = "Open FocalPoint"
    static var description: IntentDescription = "Return to the focus timer in FocalPoint."
    static var openAppWhenRun: Bool = true

    func perform() async throws -> some IntentResult {
        return .result()
    }
}
