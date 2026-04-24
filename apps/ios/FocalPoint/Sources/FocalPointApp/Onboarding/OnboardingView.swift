#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore
import MascotUI
#if canImport(UIKit)
import UIKit
#endif

/// Top-level onboarding container. Paged, swipeable, with a progress dot
/// indicator at the bottom and a sticky primary CTA.
public struct OnboardingView: View {
    @StateObject private var coord = OnboardingCoordinator()
    @EnvironmentObject private var holder: CoreHolder
    @AppStorage("app.hasOnboarded") private var hasOnboarded: Bool = false
    @State private var seedError: String?

    public init() {}

    public var body: some View {
        VStack(spacing: 0) {
            TabView(selection: .init(
                get: { coord.step },
                set: { coord.jump(to: $0) }
            )) {
                OnboardingWelcomePage().tag(OnboardingCoordinator.Step.welcome)
                OnboardingGoalsPage(coord: coord).tag(OnboardingCoordinator.Step.goals)
                OnboardingConnectPage(coord: coord).tag(OnboardingCoordinator.Step.connect)
                OnboardingTemplatePage(coord: coord).tag(OnboardingCoordinator.Step.pickTemplate)
                OnboardingPermissionsPage(coord: coord).tag(OnboardingCoordinator.Step.permissions)
            }
            .tabViewStyle(.page(indexDisplayMode: .always))
            .indexViewStyle(.page(backgroundDisplayMode: .always))

            footer
                .padding(.horizontal, 24)
                .padding(.vertical, 16)
        }
        .background(Color.app.background.ignoresSafeArea())
        .alert(
            "Couldn't set up starter rule",
            isPresented: Binding(
                get: { seedError != nil },
                set: { if !$0 { seedError = nil } }
            ),
            presenting: seedError
        ) { _ in
            Button("OK", role: .cancel) { seedError = nil }
        } message: { err in
            Text(err)
        }
    }

    private var footer: some View {
        HStack(spacing: 12) {
            if coord.step != .welcome {
                Button("Back") { coord.back() }
                    .buttonStyle(.bordered)
                    .tint(Color.app.accent)
            }
            Spacer()
            Button(coord.isFinalStep ? "Finish" : "Next") {
                if coord.isFinalStep {
                    // Always advance past onboarding. Seeding is a
                    // nice-to-have; a seed failure surfaces via alert but
                    // must NOT strand the user on the Finish step.
                    do {
                        try coord.completeAndSeed(into: holder.core)
                    } catch {
                        seedError = String(describing: error)
                    }
                    hasOnboarded = true
                    holder.bump()
                } else {
                    coord.advance()
                }
            }
            .buttonStyle(.borderedProminent)
            .tint(Color.app.accent)
            .disabled(!coord.canAdvance)
        }
    }
}

// MARK: - Pages

struct OnboardingWelcomePage: View {
    var body: some View {
        OnboardingPageChrome(
            coachyState: CoachyState(pose: .confident, emotion: .happy, bubbleText: "I'm Coachy."),
            title: "Meet Coachy",
            bodyText: "Your AI focus coach. Coachy watches what's happening in your life, nudges when it matters, and keeps you honest."
        )
    }
}

struct OnboardingGoalsPage: View {
    @ObservedObject var coord: OnboardingCoordinator

    var body: some View {
        OnboardingPageChrome(
            coachyState: CoachyState(pose: .curious, emotion: .neutral, bubbleText: "What do you want to focus on?"),
            title: "Pick 1–3 focus goals",
            bodyText: "We'll tailor rules to these. You can change them later."
        ) {
            LazyVGrid(
                columns: [GridItem(.flexible()), GridItem(.flexible())],
                spacing: 12
            ) {
                ForEach(OnboardingCoordinator.Goal.allCases) { g in
                    GoalCard(
                        goal: g,
                        selected: coord.goals.contains(g)
                    ) { coord.toggleGoal(g) }
                }
            }
        }
    }
}

private struct GoalCard: View {
    let goal: OnboardingCoordinator.Goal
    let selected: Bool
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            VStack(spacing: 8) {
                Image(systemName: goal.iconSystemName)
                    .font(.system(size: 28))
                    .foregroundStyle(selected ? Color.app.accent : Color.app.foreground.opacity(0.6))
                Text(goal.displayName)
                    .font(.footnote.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)
                    .multilineTextAlignment(.center)
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 18)
            .background(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .fill(Color.app.surface)
            )
            .overlay(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .strokeBorder(
                        selected ? Color.app.accent : Color.clear,
                        lineWidth: 2
                    )
            )
        }
        .buttonStyle(.plain)
    }
}

struct OnboardingConnectPage: View {
    @ObservedObject var coord: OnboardingCoordinator
    @State private var showCanvasAuth: Bool = false

    var body: some View {
        OnboardingPageChrome(
            coachyState: CoachyState(pose: .encouraging, emotion: .happy, bubbleText: "Let's connect your tools."),
            title: "Connect Canvas",
            bodyText: "FocalPoint listens to events from apps you already use. Start with Canvas — we'll add Google Calendar, Fitbit, GitHub, and more soon."
        ) {
            VStack(spacing: 12) {
                Button {
                    // Launch the real ASWebAuthenticationSession Canvas flow
                    // instead of flipping a bool. When the sheet returns a
                    // successful connect we mirror that into coord.canvasConnected.
                    showCanvasAuth = true
                } label: {
                    HStack {
                        Image(systemName: "graduationcap.fill")
                        Text(coord.canvasConnected ? "Canvas: connected" : "Connect Canvas")
                        Spacer()
                        Image(systemName: coord.canvasConnected ? "checkmark.circle.fill" : "chevron.right")
                    }
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(
                        RoundedRectangle(cornerRadius: 16, style: .continuous)
                            .fill(Color.app.surface)
                    )
                    .foregroundStyle(Color.app.foreground)
                }
                .buttonStyle(.plain)

                Text("Skip this if you don't use Canvas — you can wire it up later in Settings.")
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
                    .multilineTextAlignment(.center)
            }
            .sheet(isPresented: $showCanvasAuth) {
                CanvasAuthView(onConnected: { _ in
                    coord.canvasConnected = true
                })
            }
        }
    }
}

struct OnboardingTemplatePage: View {
    @ObservedObject var coord: OnboardingCoordinator

    private var featured: [RuleTemplate] {
        [
            RuleTemplates.assignmentDueTodayLock,
            RuleTemplates.deepWorkSocialBlock,
            RuleTemplates.eveningWindDown,
        ]
    }

    var body: some View {
        OnboardingPageChrome(
            coachyState: CoachyState(pose: .curious, emotion: .happy, bubbleText: "Pick a starting rule."),
            title: "Pick a starting rule",
            bodyText: "You can add more later — this is just to show you how it feels."
        ) {
            VStack(spacing: 10) {
                ForEach(featured) { t in
                    Button {
                        coord.selectedTemplateId = t.id
                    } label: {
                        HStack(alignment: .top, spacing: 12) {
                            Image(systemName: "checkmark.circle.fill")
                                .foregroundStyle(
                                    coord.selectedTemplateId == t.id
                                    ? Color.app.accent
                                    : Color.app.foreground.opacity(0.2)
                                )
                            VStack(alignment: .leading, spacing: 4) {
                                Text(t.title)
                                    .font(.body.weight(.semibold))
                                    .foregroundStyle(Color.app.foreground)
                                Text(t.subtitle)
                                    .font(.caption)
                                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                            }
                            Spacer()
                        }
                        .padding()
                        .frame(maxWidth: .infinity)
                        .background(
                            RoundedRectangle(cornerRadius: 16, style: .continuous)
                                .fill(Color.app.surface)
                        )
                    }
                    .buttonStyle(.plain)
                }
            }
        }
    }
}

struct OnboardingPermissionsPage: View {
    @ObservedObject var coord: OnboardingCoordinator

    var body: some View {
        OnboardingPageChrome(
            coachyState: CoachyState(pose: .stern, emotion: .concerned, bubbleText: "One more thing — permissions."),
            title: "Grant permissions",
            bodyText: "FocalPoint needs Screen Time, Notifications, and Calendar to actually enforce rules and surface the Morning Brief. You can change any of these later in Settings."
        ) {
            VStack(spacing: 10) {
                PermissionRow(
                    icon: "lock.fill",
                    title: "Screen Time (FamilyControls)",
                    subtitle: "Lets rules block apps.",
                    status: coord.familyControlsStatus,
                    pendingMessage: "Pending Apple entitlement — enforcement will activate once Apple approves our review."
                ) {
                    await coord.requestFamilyControlsPermission()
                }
                PermissionRow(
                    icon: "bell.badge.fill",
                    title: "Notifications",
                    subtitle: "Coachy can nudge you.",
                    status: coord.notificationsStatus,
                    pendingMessage: nil
                ) {
                    await coord.requestNotificationsPermission()
                }
                PermissionRow(
                    icon: "calendar",
                    title: "Calendar",
                    subtitle: "Morning Brief + conflict detection.",
                    status: coord.calendarStatus,
                    pendingMessage: nil
                ) {
                    await coord.requestCalendarPermission()
                }
            }
            .task {
                await coord.refreshNotificationStatus()
            }
        }
    }
}

private struct PermissionRow: View {
    let icon: String
    let title: String
    let subtitle: String
    let status: OnboardingCoordinator.PermissionStatus
    /// Only shown when `status == .pendingEntitlement`. Explains why the
    /// toggle is inert.
    let pendingMessage: String?
    let onTap: () async -> Void

    @State private var inFlight = false

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Button {
                guard !inFlight else { return }
                switch status {
                case .pendingEntitlement:
                    // Inert — no OS path to resolve, see copy below.
                    return
                case .denied:
                    openSystemSettings()
                case .notDetermined, .granted:
                    Task {
                        inFlight = true
                        await onTap()
                        inFlight = false
                    }
                }
            } label: {
                HStack {
                    Image(systemName: icon)
                        .foregroundStyle(iconTint)
                    VStack(alignment: .leading, spacing: 2) {
                        Text(title).font(.body.weight(.semibold))
                            .foregroundStyle(Color.app.foreground)
                        Text(subtitle).font(.caption)
                            .foregroundStyle(Color.app.foreground.opacity(0.7))
                    }
                    Spacer()
                    statusAccessory
                }
                .padding()
                .frame(maxWidth: .infinity)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                )
                .opacity(status == .pendingEntitlement ? 0.55 : 1.0)
            }
            .buttonStyle(.plain)
            .disabled(inFlight)

            if status == .pendingEntitlement, let msg = pendingMessage {
                Text(msg)
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
                    .padding(.horizontal, 4)
            }
        }
    }

    private var iconTint: Color {
        switch status {
        case .granted: return .green
        case .denied: return .red
        case .pendingEntitlement: return Color.app.foreground.opacity(0.35)
        case .notDetermined: return Color.app.accent
        }
    }

    @ViewBuilder
    private var statusAccessory: some View {
        switch status {
        case .granted:
            Image(systemName: "checkmark.circle.fill")
                .foregroundStyle(Color.green)
        case .denied:
            HStack(spacing: 6) {
                Image(systemName: "xmark.circle.fill")
                    .foregroundStyle(Color.red)
                Text("Open Settings")
                    .font(.caption.weight(.semibold))
                    .foregroundStyle(Color.app.accent)
            }
        case .notDetermined:
            Image(systemName: "chevron.right")
                .foregroundStyle(Color.app.foreground.opacity(0.4))
        case .pendingEntitlement:
            Text("Pending")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.5))
        }
    }

    private func openSystemSettings() {
        #if canImport(UIKit)
        if let url = URL(string: UIApplication.openSettingsURLString),
           UIApplication.shared.canOpenURL(url) {
            UIApplication.shared.open(url)
        }
        #endif
    }
}

// MARK: - Chrome

struct OnboardingPageChrome<Extra: View>: View {
    let coachyState: CoachyState
    let title: String
    let bodyText: String
    @ViewBuilder let extra: () -> Extra

    init(
        coachyState: CoachyState,
        title: String,
        bodyText: String,
        @ViewBuilder extra: @escaping () -> Extra = { EmptyView() }
    ) {
        self.coachyState = coachyState
        self.title = title
        self.bodyText = bodyText
        self.extra = extra
    }

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Duolingo-style mascot-first hero: Coachy is the focal point
                // of every onboarding page, with a pose/emotion + speech
                // bubble tuned to the step.
                CoachyView(state: coachyState, size: 180)
                    .frame(maxWidth: .infinity)
                    .padding(.top, 16)

                Text(title)
                    .font(.title2.weight(.bold))
                    .foregroundStyle(Color.app.foreground)
                    .multilineTextAlignment(.center)

                Text(bodyText)
                    .font(.body)
                    .foregroundStyle(Color.app.foreground.opacity(0.75))
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 8)

                extra()
                    .padding(.top, 8)
            }
            .padding(.horizontal, 24)
            .padding(.bottom, 48)
        }
    }
}
#endif
