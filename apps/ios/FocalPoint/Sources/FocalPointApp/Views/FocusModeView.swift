#if canImport(SwiftUI)
import SwiftUI
import UserNotifications
import ActivityKit
import DesignSystem
import MascotUI
import FocalPointCore

/// Focus Mode timer. The missing piece that wires `focus:session_started` /
/// `focus:session_completed` into the same event pipeline connector sync
/// uses, so the `deep-work-starter` template pack actually fires at runtime.
///
/// Traces to: template-pack trigger `focus:session_started` /
/// `focus:session_completed`.
///
/// Entry-point decision: slotted between Today and Tasks as its own tab
/// ("Focus", `timer` icon). A standalone tab makes the feature discoverable
/// (users coming for focus-session credits need to find this within one tap)
/// and keeps the Today tab's morning-brief content uncluttered.
struct FocusModeView: View {
    @EnvironmentObject private var holder: CoreHolder

    @Environment(\.scenePhase) private var scenePhase
    @State private var plannedMinutes: Int = 25
    @State private var customMinutes: Int = 30
    @State private var usingCustom: Bool = false

    @State private var phase: Phase = .idle
    @State private var startedAt: Date?
    @State private var pausedRemaining: TimeInterval?
    @State private var banner: String?
    @State private var liveActivity: Activity<FocusSessionAttributes>?

    private let presetChips: [Int] = [25, 45, 60, 90]

    // Observing App Intent notifications
    private let pauseNotification = NotificationCenter.default.publisher(for: NSNotification.Name("FocusSessionPause"))
    private let resumeNotification = NotificationCenter.default.publisher(for: NSNotification.Name("FocusSessionResume"))
    private let cancelNotification = NotificationCenter.default.publisher(for: NSNotification.Name("FocusSessionCancel"))

    enum Phase: Equatable {
        case idle
        case running
        case paused
        case completed
    }

    var body: some View {
        NavigationStack {
            TimelineView(.periodic(from: .now, by: 1.0)) { context in
                content(now: context.date)
            }
            .navigationTitle("Focus")
            .background(Color.app.background.ignoresSafeArea())
        }
    }

    // MARK: - Content

    @ViewBuilder
    private func content(now: Date) -> some View {
        let total = selectedMinutes * 60
        let remaining = remainingSeconds(now: now, totalSeconds: total)
        let progress = total > 0 ? 1.0 - (Double(remaining) / Double(total)) : 0.0

        ScrollView {
            VStack(spacing: 24) {
                coachyHero(progress: progress, running: phase == .running)

                if phase == .idle {
                    durationPicker
                    startButton
                } else {
                    countdown(remaining: remaining, progress: progress)
                    controls(total: total, remaining: remaining)
                }

                if let b = banner {
                    Text(b)
                        .font(.subheadline.weight(.medium))
                        .foregroundStyle(Color.app.accent)
                        .padding(12)
                        .frame(maxWidth: .infinity)
                        .background(
                            RoundedRectangle(cornerRadius: 16, style: .continuous)
                                .fill(Color.app.surface)
                        )
                        .transition(.opacity)
                }
            }
            .padding()
            .onChange(of: remaining) { _, newValue in
                // Update Live Activity if running.
                if #available(iOS 16.1, *) {
                    if phase == .running, let activity = liveActivity {
                        let newState = FocusSessionAttributes.ContentState(
                            remainingSeconds: newValue,
                            totalSeconds: selectedMinutes * 60,
                            isPaused: false,
                            ruleName: "Deep Work",
                            coachyPose: "confident",
                            upcomingBreakIn: nil,
                            timestamp: Date()
                        )
                        Task {
                            await activity.update(using: newState)
                        }
                    }
                }

                if phase == .running, newValue <= 0 {
                    complete(actualMinutes: selectedMinutes, natural: true)
                }
            }
            .onChange(of: scenePhase) { _, phase in
                if phase == .active {
                    reconcileOnForeground()
                }
            }
            // Listen for App Intent notifications (pause, resume, cancel)
            .onReceive(pauseNotification) { _ in
                guard phase == .running else { return }
                let remaining = remainingSeconds(now: Date(), totalSeconds: selectedMinutes * 60)
                pause(remaining: remaining)
            }
            .onReceive(resumeNotification) { _ in
                guard phase == .paused else { return }
                let remaining = remainingSeconds(now: Date(), totalSeconds: selectedMinutes * 60)
                resume(remaining: remaining)
            }
            .onReceive(cancelNotification) { _ in
                stopDiscard()
            }
        }
    }

    // MARK: - Coachy hero

    @ViewBuilder
    private func coachyHero(progress: Double, running: Bool) -> some View {
        let state: CoachyState = {
            switch phase {
            case .idle:
                return CoachyState(pose: .confident, emotion: .neutral, bubbleText: nil)
            case .running, .paused:
                return CoachyState(
                    pose: .confident,
                    emotion: .happy,
                    bubbleText: phase == .paused ? "Paused. Breathe." : "Stay with it."
                )
            case .completed:
                return CoachyState(
                    pose: .celebratory,
                    emotion: .excited,
                    bubbleText: "Session complete!"
                )
            }
        }()

        CoachyView(state: state, size: 220)
            .scaleEffect(running ? 1.0 + 0.02 * sin(progress * .pi * 8) : 1.0)
            .animation(.easeInOut(duration: 1.0), value: progress)
            .frame(maxWidth: .infinity)
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 20, style: .continuous)
                    .fill(Color.app.surface)
            )
    }

    // MARK: - Duration picker

    private var durationPicker: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Duration")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))
            HStack(spacing: 10) {
                ForEach(presetChips, id: \.self) { m in
                    Button {
                        plannedMinutes = m
                        usingCustom = false
                    } label: {
                        Text("\(m) min")
                            .font(.subheadline.weight(.semibold))
                            .padding(.horizontal, 14)
                            .padding(.vertical, 10)
                            .background(
                                RoundedRectangle(cornerRadius: 16, style: .continuous)
                                    .fill(
                                        (!usingCustom && plannedMinutes == m)
                                            ? Color.app.accent
                                            : Color.app.surface
                                    )
                            )
                            .foregroundStyle(
                                (!usingCustom && plannedMinutes == m)
                                    ? Color.app.accentOn
                                    : Color.app.foreground
                            )
                    }
                    .buttonStyle(.plain)
                }
            }

            HStack {
                Toggle("Custom", isOn: $usingCustom)
                    .toggleStyle(.switch)
                    .tint(Color.app.accent)
                    .labelsHidden()
                Text("Custom")
                    .font(.subheadline)
                    .foregroundStyle(Color.app.foreground.opacity(0.8))
                Spacer()
                if usingCustom {
                    Stepper(value: $customMinutes, in: 5...180, step: 5) {
                        Text("\(customMinutes) min")
                            .font(.subheadline.weight(.semibold))
                            .foregroundStyle(Color.app.foreground)
                    }
                    .labelsHidden()
                    Text("\(customMinutes) min")
                        .font(.subheadline.weight(.semibold))
                        .foregroundStyle(Color.app.foreground)
                }
            }
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    // MARK: - Start / countdown / controls

    private var startButton: some View {
        Button {
            start(minutes: selectedMinutes)
        } label: {
            Label("Start focus session", systemImage: "play.fill")
                .font(.headline)
                .frame(maxWidth: .infinity)
                .padding(.vertical, 14)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.accent)
                )
                .foregroundStyle(Color.app.accentOn)
        }
        .buttonStyle(.plain)
    }

    private func countdown(remaining: Int, progress: Double) -> some View {
        ZStack {
            Circle()
                .stroke(Color.app.surface, lineWidth: 14)
            Circle()
                .trim(from: 0, to: max(0.0, min(1.0, progress)))
                .stroke(
                    Color.app.accent,
                    style: StrokeStyle(lineWidth: 14, lineCap: .round)
                )
                .rotationEffect(.degrees(-90))
                .animation(.linear(duration: 1.0), value: progress)
            VStack(spacing: 4) {
                Text(formatMMSS(remaining))
                    .font(AppTypography.timerLarge)
                    .foregroundStyle(Color.app.foreground)
                Text(phase == .paused ? "Paused" : "Remaining")
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
            }
        }
        .frame(width: 240, height: 240)
        .padding(.vertical, 8)
    }

    private func controls(total: Int, remaining: Int) -> some View {
        HStack(spacing: 12) {
            if phase == .running {
                Button("Pause") { pause(remaining: remaining) }
                    .buttonStyle(.bordered)
            } else if phase == .paused {
                Button("Resume") { resume(remaining: remaining) }
                    .buttonStyle(.bordered)
            }
            Button("End early") {
                let elapsed = max(0, total - remaining)
                let actualMin = max(1, Int((Double(elapsed) / 60.0).rounded()))
                complete(actualMinutes: actualMin, natural: false)
            }
            .buttonStyle(.borderedProminent)
            .tint(Color.app.accent)
            Button(role: .destructive) {
                stopDiscard()
            } label: {
                Text("Stop")
            }
            .buttonStyle(.bordered)
        }
    }

    // MARK: - Actions

    private var selectedMinutes: Int {
        usingCustom ? customMinutes : plannedMinutes
    }

    private func start(minutes: Int) {
        emitHostEvent(
            type: "focus:session_started",
            payload: ["minutes": minutes]
        )
        _ = holder.core.pushMascotEvent(event: .focusSessionStarted(minutes: UInt32(minutes)))
        withAnimation(.easeInOut(duration: 0.25)) {
            phase = .running
            startedAt = Date()
            pausedRemaining = nil
            banner = nil
        }

        // Request Live Activity (iOS 16.1+) for Lock Screen + Dynamic Island.
        if #available(iOS 16.1, *) {
            let attributes = FocusSessionAttributes(
                sessionTitle: "Focus Session",
                startedAt: Date(),
                plannedDuration: minutes * 60,
                breakInterval: 300, // 5-min break suggestion
                bgTint: "blue",
                coachyEmoji: "🧘"
            )
            let initialState = FocusSessionAttributes.ContentState(
                remainingSeconds: minutes * 60,
                totalSeconds: minutes * 60,
                isPaused: false,
                ruleName: "Deep Work",
                coachyPose: "confident",
                upcomingBreakIn: nil,
                timestamp: Date()
            )
            do {
                liveActivity = try Activity.request(
                    attributes: attributes,
                    contentState: initialState,
                    pushType: nil
                )
            } catch {
                // Live Activities not available or user denied; graceful fallback.
                banner = "Live Activity unavailable on this device"
            }
        }

        // Schedule a local notification at the planned end time so the user
        // gets a "session complete" push even if the app is backgrounded or
        // closed. Identifier is predictable so a later Stop can cancel it.
        scheduleCompletionPush(minutes: minutes)
        // Drive one sync+eval tick immediately so rules don't wait 60s.
        _ = holder.syncTick()
        _ = holder.evalTick()
    }

    /// Cancel any pending session-complete notification. Called on Stop
    /// (discard) and on natural completion so we don't double-fire.
    private func cancelCompletionPush() {
        UNUserNotificationCenter.current().removePendingNotificationRequests(
            withIdentifiers: ["focalpoint.focus.complete"]
        )
    }

    private func scheduleCompletionPush(minutes: Int) {
        let center = UNUserNotificationCenter.current()
        center.removePendingNotificationRequests(withIdentifiers: ["focalpoint.focus.complete"])
        let content = UNMutableNotificationContent()
        content.title = "Focus session complete"
        content.body = "\(minutes) min done. Nice work."
        content.sound = .default
        let trigger = UNTimeIntervalNotificationTrigger(
            timeInterval: TimeInterval(minutes * 60),
            repeats: false
        )
        let req = UNNotificationRequest(
            identifier: "focalpoint.focus.complete",
            content: content,
            trigger: trigger
        )
        center.add(req) { _ in }
    }

    /// Called from the App-level scenePhase observer when the user returns
    /// to the app. If the timer ran out while we were backgrounded, fire
    /// the natural-completion path so wallet/mascot/rules all see the
    /// session-completed event.
    private func reconcileOnForeground() {
        guard phase == .running else { return }
        let total = selectedMinutes * 60
        guard let start = startedAt else { return }
        let elapsed = Date().timeIntervalSince(start)
        if elapsed >= TimeInterval(total) {
            complete(actualMinutes: selectedMinutes, natural: true)
        }
    }

    private func pause(remaining: Int) {
        pausedRemaining = TimeInterval(remaining)
        phase = .paused

        // Update Live Activity to show paused state.
        if #available(iOS 16.1, *) {
            if let activity = liveActivity {
                let newState = FocusSessionAttributes.ContentState(
                    remainingSeconds: remaining,
                    totalSeconds: selectedMinutes * 60,
                    isPaused: true,
                    ruleName: "Deep Work",
                    coachyPose: "confident",
                    upcomingBreakIn: nil,
                    timestamp: Date()
                )
                Task {
                    await activity.update(using: newState)
                }
            }
        }
    }

    private func resume(remaining: Int) {
        // Re-anchor startedAt so `remainingSeconds` math stays consistent.
        let total = selectedMinutes * 60
        startedAt = Date().addingTimeInterval(-TimeInterval(total - remaining))
        pausedRemaining = nil
        phase = .running

        // Update Live Activity to show resumed state.
        if #available(iOS 16.1, *) {
            if let activity = liveActivity {
                let newState = FocusSessionAttributes.ContentState(
                    remainingSeconds: remaining,
                    totalSeconds: total,
                    isPaused: false,
                    ruleName: "Deep Work",
                    coachyPose: "confident",
                    upcomingBreakIn: nil,
                    timestamp: Date()
                )
                Task {
                    await activity.update(using: newState)
                }
            }
        }
    }

    private func stopDiscard() {
        cancelCompletionPush()

        // End the Live Activity.
        if #available(iOS 16.1, *) {
            if let activity = liveActivity {
                Task {
                    await activity.end(using: activity.contentState, dismissalPolicy: .immediate)
                    liveActivity = nil
                }
            }
        }

        withAnimation(.easeInOut(duration: 0.25)) {
            phase = .idle
            startedAt = nil
            pausedRemaining = nil
            banner = nil
        }
    }

    private func complete(actualMinutes: Int, natural: Bool) {
        cancelCompletionPush()

        // End the Live Activity.
        if #available(iOS 16.1, *) {
            if let activity = liveActivity {
                Task {
                    let finalState = FocusSessionAttributes.ContentState(
                        remainingSeconds: 0,
                        totalSeconds: actualMinutes * 60,
                        isPaused: false,
                        ruleName: "Deep Work",
                        coachyPose: "celebratory",
                        upcomingBreakIn: nil,
                        timestamp: Date()
                    )
                    await activity.end(using: finalState, dismissalPolicy: .after(Date(timeIntervalSinceNow: 3.5)))
                    liveActivity = nil
                }
            }
        }

        emitHostEvent(
            type: "focus:session_completed",
            payload: ["minutes": actualMinutes, "natural": natural]
        )
        _ = holder.core.pushMascotEvent(event: .focusSessionCompleted(minutes: UInt32(actualMinutes)))
        withAnimation(.easeInOut(duration: 0.3)) {
            phase = .completed
            startedAt = nil
            pausedRemaining = nil
            banner = natural
                ? "Session complete! Credits: TBD"
                : "Ended early — \(actualMinutes) min logged."
        }
        _ = holder.syncTick()
        _ = holder.evalTick()
        // Return to idle after a short beat so the user can start another.
        DispatchQueue.main.asyncAfter(deadline: .now() + 3.5) {
            if phase == .completed {
                withAnimation(.easeInOut(duration: 0.3)) {
                    phase = .idle
                }
            }
        }
    }

    private func emitHostEvent(type: String, payload: [String: Any]) {
        let payloadJson: String
        if let data = try? JSONSerialization.data(withJSONObject: payload, options: []),
           let s = String(data: data, encoding: .utf8)
        {
            payloadJson = s
        } else {
            payloadJson = "{}"
        }
        do {
            try holder.core.hostEvents().emit(
                dto: HostEventDto(
                    eventType: type,
                    confidence: 1.0,
                    payloadJson: payloadJson,
                    dedupeKey: nil
                )
            )
            holder.bump()
        } catch {
            // Loud, visible — no silent fallback.
            banner = "Failed to emit \(type): \(error)"
        }
    }

    // MARK: - Timing helpers

    private func remainingSeconds(now: Date, totalSeconds: Int) -> Int {
        switch phase {
        case .idle, .completed:
            return totalSeconds
        case .paused:
            return Int(pausedRemaining ?? TimeInterval(totalSeconds))
        case .running:
            guard let start = startedAt else { return totalSeconds }
            let elapsed = now.timeIntervalSince(start)
            return max(0, totalSeconds - Int(elapsed))
        }
    }

    private func formatMMSS(_ seconds: Int) -> String {
        let m = seconds / 60
        let s = seconds % 60
        return String(format: "%02d:%02d", m, s)
    }
}
#endif
