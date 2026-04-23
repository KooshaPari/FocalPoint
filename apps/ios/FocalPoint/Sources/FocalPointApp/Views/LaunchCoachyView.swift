#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI

/// LaunchCoachyView — the Coachy sleep→wake sequence that plays once on the
/// first cold launch. See `docs/mascot/coachy-art-direction.md` §7.
///
/// Three phases:
///   1. Sleeping (1.0s)             — sleepyDisappointed + tired, Z's, breathing
///   2. Eyes open + stretch (1.5s)  — encouraging + neutral, flame wisp
///   3. Full wake + bubble (1.0s)   — confident + proud, "Ready?" bubble
///
/// Identity preservation: every phase applies
/// `matchedGeometryEffect(id: "coachy.identity", in: namespace)` on the Coachy
/// view so the transition into the next screen (HomeView / onboarding) animates
/// as a continuous character, not a cut.
struct LaunchCoachyView: View {
    /// Called when the sequence completes. Host decides what to route to.
    var onFinish: () -> Void

    /// Matched-geometry namespace owned by the host so the destination view
    /// can participate in the hand-off. Accepting it as a parameter (rather
    /// than owning it here) is what preserves identity across parent transitions.
    var namespace: Namespace.ID

    @State private var phase: Phase = .sleeping
    @State private var breathing: CGFloat = 1.0
    @State private var zOffset: CGFloat = 0
    @State private var zOpacity: Double = 0

    private enum Phase {
        case sleeping, stretching, awake
    }

    var body: some View {
        ZStack {
            Color.app.background.ignoresSafeArea()

            VStack(spacing: 24) {
                ZStack {
                    // Sleeping Z's — only rendered during phase 1.
                    if phase == .sleeping {
                        sleepingZs
                            .offset(x: 60, y: -40 + zOffset)
                            .opacity(zOpacity)
                    }

                    coachyForPhase
                        .scaleEffect(breathing)
                        .matchedGeometryEffect(id: "coachy.identity", in: namespace)
                }

                // Bubble only in phase 3. Uses the same chat-bubble modifier
                // HomeView uses, so visually it reads as Coachy's first line.
                if phase == .awake {
                    Text("Ready?")
                        .chatBubble()
                        .transition(.opacity.combined(with: .scale(scale: 0.9)))
                }
            }
        }
        .onAppear(perform: runSequence)
    }

    // MARK: - Phase rendering

    @ViewBuilder
    private var coachyForPhase: some View {
        switch phase {
        case .sleeping:
            CoachyAnimationEngine.view(
                pose: .sleepyDisappointed,
                emotion: .tired,
                bubbleText: nil,
                size: 240
            )
        case .stretching:
            CoachyAnimationEngine.view(
                pose: .encouraging,
                emotion: .neutral,
                bubbleText: nil,
                size: 240
            )
        case .awake:
            CoachyAnimationEngine.view(
                pose: .confident,
                emotion: .proud,
                bubbleText: nil, // bubble is rendered separately so the
                                 // transition is explicitly animated.
                size: 240
            )
        }
    }

    private var sleepingZs: some View {
        VStack(alignment: .leading, spacing: 2) {
            Text("Z").font(.system(size: 14, weight: .bold, design: .rounded))
                .opacity(0.5)
            Text("Z").font(.system(size: 20, weight: .bold, design: .rounded))
                .opacity(0.7)
            Text("Z").font(.system(size: 28, weight: .bold, design: .rounded))
        }
        .foregroundStyle(Color.app.foreground.opacity(0.6))
    }

    // MARK: - Choreography

    private func runSequence() {
        // Breathing loop during phase 1.
        withAnimation(.easeInOut(duration: 1.0).repeatCount(1, autoreverses: true)) {
            breathing = 1.03
        }
        // Z's drift up + fade out across phase 1.
        withAnimation(.easeOut(duration: 1.0)) {
            zOffset = -24
            zOpacity = 0.9
        }

        // Phase 2: stretch after 1.0s.
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
            withAnimation(.easeInOut(duration: 1.5)) {
                phase = .stretching
                breathing = 1.0
                zOpacity = 0
            }
        }

        // Phase 3: fully awake + bubble at 2.5s.
        DispatchQueue.main.asyncAfter(deadline: .now() + 2.5) {
            withAnimation(.easeInOut(duration: 1.0)) {
                phase = .awake
            }
        }

        // Finish at 3.5s.
        DispatchQueue.main.asyncAfter(deadline: .now() + 3.5) {
            onFinish()
        }
    }
}

/// Convenience wrapper that owns the matched-geometry namespace so callers
/// don't have to declare `@Namespace` themselves. Use `LaunchCoachyView`
/// directly when the caller needs the namespace (e.g. to participate in the
/// hand-off transition).
struct LaunchCoachySequence: View {
    var onFinish: () -> Void
    @Namespace private var ns

    var body: some View {
        LaunchCoachyView(onFinish: onFinish, namespace: ns)
    }
}
#endif
