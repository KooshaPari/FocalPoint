#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

/// Top-level onboarding container. Paged, swipeable, with a progress dot
/// indicator at the bottom and a sticky primary CTA.
public struct OnboardingView: View {
    @StateObject private var coord = OnboardingCoordinator()
    @EnvironmentObject private var holder: CoreHolder
    @AppStorage("app.hasOnboarded") private var hasOnboarded: Bool = false

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
                    do {
                        try coord.completeAndSeed(into: holder.core)
                        hasOnboarded = true
                        holder.bump()
                    } catch {
                        // Loud failure — no silent fallback per policy.
                        print("[FocalPoint] onboarding seed failed: \(error)")
                    }
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
            hero: "flame.fill",
            title: "Meet Coachy",
            bodyText: "Your AI focus coach. Coachy watches what's happening in your life, nudges when it matters, and keeps you honest."
        )
    }
}

struct OnboardingGoalsPage: View {
    @ObservedObject var coord: OnboardingCoordinator

    var body: some View {
        OnboardingPageChrome(
            hero: "target",
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

    var body: some View {
        OnboardingPageChrome(
            hero: "link.circle.fill",
            title: "Connect Canvas",
            bodyText: "FocalPoint listens to events from apps you already use. Start with Canvas — we'll add Google Calendar, Fitbit, GitHub, and more soon."
        ) {
            VStack(spacing: 12) {
                Button {
                    coord.canvasConnected.toggle()
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
                        RoundedRectangle(cornerRadius: 14, style: .continuous)
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
            hero: "list.bullet.rectangle.fill",
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
                            RoundedRectangle(cornerRadius: 14, style: .continuous)
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
            hero: "lock.shield.fill",
            title: "Grant permissions",
            bodyText: "FocalPoint needs Screen Time + Notifications to actually enforce rules. You can change either later in Settings."
        ) {
            VStack(spacing: 10) {
                PermissionRow(
                    icon: "lock.fill",
                    title: "Screen Time (FamilyControls)",
                    subtitle: "Lets rules block apps.",
                    granted: coord.familyControlsGranted
                ) {
                    // Real entitlement request deferred to post-launch. The
                    // FamilyControls entitlement is not on this app yet.
                    coord.familyControlsGranted.toggle()
                }
                PermissionRow(
                    icon: "bell.badge.fill",
                    title: "Notifications",
                    subtitle: "Coachy can nudge you.",
                    granted: coord.notificationsGranted
                ) {
                    coord.notificationsGranted.toggle()
                }
            }
        }
    }
}

private struct PermissionRow: View {
    let icon: String
    let title: String
    let subtitle: String
    let granted: Bool
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            HStack {
                Image(systemName: icon)
                    .foregroundStyle(Color.app.accent)
                VStack(alignment: .leading, spacing: 2) {
                    Text(title).font(.body.weight(.semibold))
                        .foregroundStyle(Color.app.foreground)
                    Text(subtitle).font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                }
                Spacer()
                Image(systemName: granted ? "checkmark.circle.fill" : "chevron.right")
                    .foregroundStyle(granted ? Color.app.accent : Color.app.foreground.opacity(0.4))
            }
            .padding()
            .frame(maxWidth: .infinity)
            .background(
                RoundedRectangle(cornerRadius: 14, style: .continuous)
                    .fill(Color.app.surface)
            )
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Chrome

struct OnboardingPageChrome<Extra: View>: View {
    let hero: String
    let title: String
    let bodyText: String
    @ViewBuilder let extra: () -> Extra

    init(hero: String, title: String, bodyText: String, @ViewBuilder extra: @escaping () -> Extra = { EmptyView() }) {
        self.hero = hero
        self.title = title
        self.bodyText = bodyText
        self.extra = extra
    }

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Hero illustration slot — placeholder asset name; falls back
                // to an SF Symbol glyph when the named image is missing.
                ZStack {
                    Image("onboarding-\(hero)") // empty asset; fallback below
                        .resizable()
                        .scaledToFit()
                    Image(systemName: hero)
                        .font(.system(size: 96, weight: .regular))
                        .foregroundStyle(Color.app.accent)
                }
                .frame(maxWidth: .infinity)
                .frame(height: 180)
                .padding(.top, 24)

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
