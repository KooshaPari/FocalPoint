#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI

/// Overlay view for rule-fired visual fly-ins.
/// Slides in from the right with celebratory pose, holds for 1.8s, exits right.
struct RuleFiredFlyInView: View {
    @ObservedObject var presenter: RuleFiredFlyInPresenter
    @State private var offset: CGFloat = 400
    @State private var opacity: Double = 0

    var body: some View {
        if presenter.isPresenting, let flyIn = presenter.currentFlyIn {
            ZStack {
                // Semi-transparent background (optional)
                Color.black.opacity(0.3)
                    .ignoresSafeArea()
                    .onTapGesture {
                        withAnimation {
                            presenter.isPresenting = false
                        }
                    }

                // Fly-in from the right
                HStack(spacing: 16) {
                    CoachyView(
                        state: CoachyState(
                            pose: .celebratory,
                            emotion: .excited,
                            bubbleText: flyIn.bubble
                        ),
                        size: 140
                    )

                    if flyIn.isAggregated {
                        VStack(alignment: .leading, spacing: 4) {
                            Text("\(flyIn.ruleCount) rules fired")
                                .font(.headline.weight(.bold))
                                .foregroundStyle(Color.app.accent)
                            Text("Great progress on your goals!")
                                .font(.caption)
                                .foregroundStyle(Color.app.foreground.opacity(0.7))
                        }
                    }

                    Spacer()
                }
                .padding(16)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                        .shadow(radius: 8, y: 4)
                )
                .padding()
                .offset(x: offset)
                .opacity(opacity)
                .frame(maxWidth: .infinity, alignment: .trailing)
                .onAppear {
                    withAnimation(.easeOut(duration: 0.4)) {
                        offset = 0
                        opacity = 1.0
                    }
                    // Schedule exit after hold time
                    DispatchQueue.main.asyncAfter(deadline: .now() + 1.2) {
                        withAnimation(.easeIn(duration: 0.3)) {
                            offset = 400
                            opacity = 0
                        }
                    }
                }
            }
            .transition(.identity)
        }
    }
}

#Preview {
    ZStack {
        Color.app.background.ignoresSafeArea()

        RuleFiredFlyInView(
            presenter: {
                let p = RuleFiredFlyInPresenter.shared
                p.currentFlyIn = RuleFlyInState(
                    bubble: "Morning routine locked in!",
                    isAggregated: false,
                    ruleCount: 1
                )
                p.isPresenting = true
                return p
            }()
        )
    }
}
#endif
