#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore
import MascotUI

/// Duolingo-grade mascot-first onboarding. Full-screen Coachy hero on every page,
/// matched-geometry transitions preserve character identity across steps, micro-animations
/// with SFX and haptics on each beat.
public struct OnboardingViewV2: View {
    @StateObject private var coord = OnboardingCoordinator()
    @EnvironmentObject private var holder: CoreHolder
    @AppStorage("app.hasOnboarded") private var hasOnboarded: Bool = false
    @State private var seedError: String?
    @Namespace private var coachyNS
    @State private var selectedGoals: Set<String> = []
    @State private var goalSparkleId: String? = nil

    public init() {}

    public var body: some View {
        ZStack {
            VStack(spacing: 0) {
                TabView(selection: .init(
                    get: { coord.step },
                    set: { coord.jump(to: $0) }
                )) {
                    OnboardingWelcomePageV2(namespace: coachyNS)
                        .tag(OnboardingCoordinator.Step.welcome)
                        .onAppear { OnboardingResumeState.completeStep(1) }
                    OnboardingGoalsPageV2(namespace: coachyNS, selectedGoals: $selectedGoals, sparkleId: $goalSparkleId)
                        .tag(OnboardingCoordinator.Step.goals)
                        .onAppear { OnboardingResumeState.completeStep(2) }
                    OnboardingConnectPageV2(namespace: coachyNS, coord: coord)
                        .tag(OnboardingCoordinator.Step.connect)
                        .onAppear { OnboardingResumeState.completeStep(3) }
                    OnboardingTemplatePageV2(namespace: coachyNS)
                        .tag(OnboardingCoordinator.Step.pickTemplate)
                        .onAppear { OnboardingResumeState.completeStep(4) }
                    PermissionsStep(namespace: coachyNS, coord: coord)
                        .tag(OnboardingCoordinator.Step.permissions)
                        .onAppear { OnboardingResumeState.completeStep(5) }
                    OnboardingFinalPageV2(namespace: coachyNS)
                        .tag(OnboardingCoordinator.Step.done)
                        .onAppear { OnboardingResumeState.completeStep(6) }
                }
                .tabViewStyle(.page(indexDisplayMode: .never))
                .padding(.bottom, 100)

                footer
                    .padding(.horizontal, 24)
                    .padding(.vertical, 16)
            }
            .background(Color.app.background.ignoresSafeArea())

            if coord.isFinalStep {
                ParticleOverlay(particles: .confetti(count: 80))
                    .ignoresSafeArea()
            }
        }
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
                Button(action: {
                    coord.back()
                    SoundEffectPlayer.shared.play(.pageTurn, variation: .reverse)
                    HapticChoreographer.shared.perform(.softTap)
                }) {
                    Text("Back")
                }
                .buttonStyle(.bordered)
                .tint(Color.app.accent)
            }
            Spacer()
            Button(action: {
                if coord.isFinalStep {
                    do {
                        try coord.recordConsentAcceptance(into: holder.core)
                        try coord.completeAndSeed(into: holder.core)
                        HapticChoreographer.shared.perform(.celebrate)
                        SoundEffectPlayer.shared.play(.fanfare)
                    } catch {
                        seedError = String(describing: error)
                    }
                    hasOnboarded = true
                    holder.bump()
                } else {
                    coord.advance()
                    SoundEffectPlayer.shared.play(.pageTurn)
                    HapticChoreographer.shared.perform(.mediumTap)
                }
            }) {
                Text(coord.isFinalStep ? "Finish" : "Next")
            }
            .buttonStyle(.borderedProminent)
            .tint(Color.app.accent)
            .disabled(!coord.canAdvance)
        }
    }
}

// MARK: - Welcome Page

struct OnboardingWelcomePageV2: View {
    let namespace: Namespace.ID
    @State private var showBubble = false

    var body: some View {
        VStack(spacing: 0) {
            Spacer()

            CoachyView(
                state: CoachyState(
                    pose: .happy,
                    emotion: .warm,
                    bubbleText: "Hi! I'm Coachy, your focus coach."
                ),
                size: 300
            )
            .matchedGeometryEffect(id: "coachy.onboarding", in: namespace)
            .scaleEffect(showBubble ? 1.0 : 0.8)
            .opacity(showBubble ? 1.0 : 0.5)
            .transition(.asymmetric(
                insertion: .move(edge: .bottom).combined(with: .opacity),
                removal: .move(edge: .top).combined(with: .opacity)
            ))

            Spacer()

            VStack(spacing: 16) {
                Text("Focus Coach")
                    .font(.system(size: 32, weight: .bold))
                    .foregroundStyle(Color.app.foreground)

                Text("Let's set up your focus rules and goals together. I'll help you build healthy screen habits.")
                    .font(.body)
                    .foregroundStyle(Color.app.foreground.opacity(0.8))
                    .lineLimit(nil)
                    .multilineTextAlignment(.center)
            }
            .padding(.horizontal, 24)
            .padding(.bottom, 40)

            Spacer()
        }
        .onAppear {
            SimlishVoice.shared.speak("Welcome!")
            withAnimation(.easeOut(duration: 0.6)) {
                showBubble = true
            }
        }
    }
}

// MARK: - Goals Page

struct OnboardingGoalsPageV2: View {
    let namespace: Namespace.ID
    @Binding var selectedGoals: Set<String>
    @Binding var sparkleId: String?
    @State private var localSelected: Set<String> = []

    let goals = [
        ("sleep", "Better Sleep", "Track time to improve rest"),
        ("fitness", "Fitness", "Stay active and healthy"),
        ("study", "Study Focus", "Deep work sessions"),
        ("balance", "Work-Life Balance", "Disconnect to recharge"),
    ]

    var body: some View {
        VStack(spacing: 0) {
            Spacer()

            CoachyView(
                state: CoachyState(
                    pose: .curious,
                    emotion: .engaged,
                    bubbleText: "What are your focus goals?"
                ),
                size: 260
            )
            .matchedGeometryEffect(id: "coachy.onboarding", in: namespace)

            Spacer()

            VStack(spacing: 12) {
                ForEach(goals, id: \.0) { id, title, desc in
                    GoalCard(
                        id: id,
                        title: title,
                        description: desc,
                        isSelected: localSelected.contains(id),
                        onTap: {
                            withAnimation(.spring(response: 0.3, dampingFraction: 0.6)) {
                                if localSelected.contains(id) {
                                    localSelected.remove(id)
                                } else {
                                    localSelected.insert(id)
                                }
                                sparkleId = id
                            }
                            SoundEffectPlayer.shared.play(.select)
                            HapticChoreographer.shared.perform(.lightTap)
                            SimlishVoice.shared.speak("\(title)? Great choice!")
                        }
                    )
                    .if(sparkleId == id) { view in
                        view.overlay(alignment: .topTrailing) {
                            ParticleOverlay(particles: .sparkles(count: 12))
                                .frame(width: 100, height: 100)
                                .offset(x: -20, y: -20)
                        }
                    }
                }
            }
            .padding(.horizontal, 24)
            .padding(.bottom, 40)

            Spacer()
        }
        .onAppear {
            selectedGoals = localSelected
        }
        .onDisappear {
            selectedGoals = localSelected
        }
    }
}

struct GoalCard: View {
    let id: String
    let title: String
    let description: String
    let isSelected: Bool
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            HStack(spacing: 16) {
                Image(systemName: isSelected ? "checkmark.circle.fill" : "circle")
                    .font(.system(size: 24))
                    .foregroundStyle(isSelected ? Color.app.accent : Color.app.foreground.opacity(0.4))
                    .animation(.spring(), value: isSelected)

                VStack(alignment: .leading, spacing: 4) {
                    Text(title)
                        .font(.body.weight(.semibold))
                        .foregroundStyle(Color.app.foreground)
                    Text(description)
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.6))
                }

                Spacer()
            }
            .padding(16)
            .background(
                RoundedRectangle(cornerRadius: 12)
                    .fill(isSelected ? Color.app.accent.opacity(0.1) : Color.app.surface)
            )
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? Color.app.accent : Color.clear, lineWidth: 2)
            )
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Connect Page

struct OnboardingConnectPageV2: View {
    let namespace: Namespace.ID
    @ObservedObject var coord: OnboardingCoordinator
    @State private var connectorConnected = false

    var body: some View {
        VStack(spacing: 0) {
            Spacer()

            CoachyView(
                state: CoachyState(
                    pose: .encouraging,
                    emotion: .warm,
                    bubbleText: "Let's connect a calendar."
                ),
                size: 280
            )
            .matchedGeometryEffect(id: "coachy.onboarding", in: namespace)
            .scaleEffect(connectorConnected ? 1.05 : 1.0)
            .animation(.spring(response: 0.3, dampingFraction: 0.6), value: connectorConnected)

            Spacer()

            VStack(spacing: 16) {
                Text("Connect Your Calendar")
                    .font(.system(size: 24, weight: .semibold))
                    .foregroundStyle(Color.app.foreground)

                Text("I'll sync your events to help manage your time.")
                    .font(.body)
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                    .multilineTextAlignment(.center)

                HStack(spacing: 12) {
                    Button(action: {
                        // Simulate connection
                        withAnimation(.easeOut(duration: 0.4)) {
                            connectorConnected = true
                        }
                        SoundEffectPlayer.shared.play(.success)
                        HapticChoreographer.shared.perform(.celebrate)
                        SimlishVoice.shared.speak("Calendar connected!")
                    }) {
                        HStack(spacing: 8) {
                            Image(systemName: "calendar")
                            Text(connectorConnected ? "Connected" : "Connect Google Calendar")
                        }
                        .frame(maxWidth: .infinity)
                        .padding(12)
                        .background(Color.app.accent)
                        .foregroundStyle(.white)
                        .cornerRadius(8)
                    }
                    .disabled(connectorConnected)
                }

                if connectorConnected {
                    HStack(spacing: 8) {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundStyle(Color.green)
                        Text("Calendar synced!")
                            .font(.subheadline)
                            .foregroundStyle(Color.green)
                    }
                    .transition(.scale.combined(with: .opacity))
                }
            }
            .padding(.horizontal, 24)
            .padding(.bottom, 40)

            Spacer()
        }
    }
}

// MARK: - Template Page

struct OnboardingTemplatePageV2: View {
    let namespace: Namespace.ID
    @State private var selectedTemplate: String?

    let templates = [
        ("pomodoro", "Pomodoro", "25 min focus, 5 min break"),
        ("deepstudy", "Deep Study", "90 min deep work blocks"),
        ("balance", "Balanced", "Mix of focus and breaks"),
    ]

    var body: some View {
        VStack(spacing: 0) {
            Spacer()

            CoachyView(
                state: CoachyState(
                    pose: .curiousThinking,
                    emotion: .neutral,
                    bubbleText: "Pick your focus style."
                ),
                size: 260
            )
            .matchedGeometryEffect(id: "coachy.onboarding", in: namespace)

            Spacer()

            VStack(spacing: 12) {
                Text("Focus Template")
                    .font(.system(size: 24, weight: .semibold))
                    .foregroundStyle(Color.app.foreground)
                    .padding(.bottom, 8)

                ForEach(templates, id: \.0) { id, title, desc in
                    Button(action: {
                        withAnimation(.spring()) {
                            selectedTemplate = id
                        }
                        SoundEffectPlayer.shared.play(.select)
                        HapticChoreographer.shared.perform(.lightTap)
                    }) {
                        HStack(spacing: 16) {
                            VStack(alignment: .leading, spacing: 4) {
                                Text(title)
                                    .font(.body.weight(.semibold))
                                    .foregroundStyle(Color.app.foreground)
                                Text(desc)
                                    .font(.caption)
                                    .foregroundStyle(Color.app.foreground.opacity(0.6))
                            }
                            Spacer()
                            if selectedTemplate == id {
                                Image(systemName: "checkmark")
                                    .foregroundStyle(Color.app.accent)
                                    .font(.system(size: 16, weight: .bold))
                            }
                        }
                        .padding(16)
                        .background(
                            RoundedRectangle(cornerRadius: 12)
                                .fill(selectedTemplate == id ? Color.app.accent.opacity(0.1) : Color.app.surface)
                        )
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .stroke(selectedTemplate == id ? Color.app.accent : Color.clear, lineWidth: 2)
                        )
                    }
                    .buttonStyle(.plain)
                }
            }
            .padding(.horizontal, 24)
            .padding(.bottom, 40)

            Spacer()
        }
    }
}


// MARK: - Final Page

struct OnboardingFinalPageV2: View {
    let namespace: Namespace.ID
    @State private var showCelebration = false

    var body: some View {
        VStack(spacing: 0) {
            Spacer()

            CoachyView(
                state: CoachyState(
                    pose: .confident,
                    emotion: .ecstatic,
                    bubbleText: "You're ready! Let's go!"
                ),
                size: 320
            )
            .matchedGeometryEffect(id: "coachy.onboarding", in: namespace)
            .scaleEffect(showCelebration ? 1.1 : 1.0)
            .animation(.spring(response: 0.5, dampingFraction: 0.5), value: showCelebration)

            Spacer()

            VStack(spacing: 16) {
                Text("Welcome to FocalPoint")
                    .font(.system(size: 28, weight: .bold))
                    .foregroundStyle(Color.app.foreground)

                Text("You're all set. Let's build great focus habits together!")
                    .font(.body)
                    .foregroundStyle(Color.app.foreground.opacity(0.8))
                    .multilineTextAlignment(.center)
            }
            .padding(.horizontal, 24)
            .padding(.bottom, 40)

            Spacer()
        }
        .onAppear {
            withAnimation(.spring(response: 0.6, dampingFraction: 0.6)) {
                showCelebration = true
            }
            SimlishVoice.shared.speak("You're ready!")
            SoundEffectPlayer.shared.play(.fanfare)
        }
    }
}

// MARK: - Particle Overlay

struct ParticleOverlay: View {
    enum ParticleType {
        case confetti(count: Int)
        case sparkles(count: Int)
    }

    let particles: ParticleType

    var body: some View {
        Canvas { context, size in
            switch particles {
            case .confetti(let count):
                for i in 0..<count {
                    let x = CGFloat.random(in: 0...size.width)
                    let y = CGFloat.random(in: 0...size.height)
                    let colors: [Color] = [.red, .orange, .yellow, .green, .blue, .purple]
                    let color = colors[i % colors.count]

                    var path = Path()
                    path.addEllipse(in: CGRect(x: x, y: y, width: 8, height: 8))
                    context.fill(path, with: .color(color))
                }

            case .sparkles(let count):
                for i in 0..<count {
                    let x = CGFloat.random(in: 0...size.width)
                    let y = CGFloat.random(in: 0...size.height)

                    var path = Path()
                    path.addEllipse(in: CGRect(x: x, y: y, width: 4, height: 4))
                    context.fill(path, with: .color(Color.app.accent))
                }
            }
        }
    }
}

// MARK: - Helpers

extension View {
    func `if`<Content: View>(_ condition: Bool, transform: (Self) -> Content) -> AnyView {
        if condition {
            return AnyView(transform(self))
        } else {
            return AnyView(self)
        }
    }
}

#endif
