#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI

/// Shown on app launch when onboarding is incomplete but user has
/// started. Offers "Pick up where you left off" or "Start over".
public struct ResumeOnboardingView: View {
    @Binding var isPresented: Bool
    @State private var showCoachy = false
    let onResume: () -> Void
    let onRestart: () -> Void

    var body: some View {
        ZStack {
            Color.app.background.ignoresSafeArea()

            VStack(spacing: 24) {
                Spacer()

                // Coachy mascot with welcome-back expression
                CoachyView(
                    state: CoachyState(
                        pose: .encouraging,
                        emotion: .warm,
                        bubbleText: "Welcome back! Let's finish setup 🚀"
                    ),
                    size: 280
                )
                .scaleEffect(showCoachy ? 1.0 : 0.8)
                .opacity(showCoachy ? 1.0 : 0.5)
                .animation(.spring(response: 0.5, dampingFraction: 0.6), value: showCoachy)

                VStack(spacing: 16) {
                    Text("Ready to Resume?")
                        .font(.system(size: 28, weight: .bold))
                        .foregroundStyle(Color.app.foreground)

                    // Progress indicator
                    HStack(spacing: 8) {
                        Image(systemName: "checkmark.circle.fill")
                            .font(.system(size: 16, weight: .semibold))
                            .foregroundStyle(Color.green)
                        Text(OnboardingResumeState.getProgressLabel())
                            .font(.subheadline)
                            .foregroundStyle(Color.app.foreground.opacity(0.7))
                    }

                    // Summary of progress
                    VStack(alignment: .leading, spacing: 8) {
                        SummaryItem(icon: "calendar.circle.fill", title: "Calendar connected")
                        SummaryItem(icon: "checkmark.circle.fill", title: "Permissions granted")
                    }
                    .padding(12)
                    .background(Color.app.surface)
                    .cornerRadius(12)
                }
                .padding(.horizontal, 24)

                Spacer()

                // Action buttons
                VStack(spacing: 12) {
                    Button(action: {
                        onResume()
                        withAnimation(.easeOut(duration: 0.3)) {
                            isPresented = false
                        }
                    }) {
                        HStack(spacing: 8) {
                            Image(systemName: "arrow.forward.circle.fill")
                            Text("Pick up where I left off")
                        }
                        .frame(maxWidth: .infinity)
                        .padding(14)
                        .background(Color.app.accent)
                        .foregroundStyle(.white)
                        .cornerRadius(10)
                    }

                    Button(action: {
                        OnboardingResumeState.resetTracking()
                        onRestart()
                        withAnimation(.easeOut(duration: 0.3)) {
                            isPresented = false
                        }
                    }) {
                        Text("Start over")
                            .frame(maxWidth: .infinity)
                            .padding(14)
                            .background(Color.app.surface)
                            .foregroundStyle(Color.app.foreground)
                            .cornerRadius(10)
                    }
                }
                .padding(.horizontal, 24)
                .padding(.bottom, 32)
            }
        }
        .onAppear {
            withAnimation(.spring(response: 0.6, dampingFraction: 0.6)) {
                showCoachy = true
            }
            SimlishVoice.shared.speak("Welcome back!")
        }
    }
}

// MARK: - Summary Item Component

struct SummaryItem: View {
    let icon: String
    let title: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .font(.system(size: 14, weight: .semibold))
                .foregroundStyle(Color.green)
            Text(title)
                .font(.subheadline)
                .foregroundStyle(Color.app.foreground.opacity(0.8))
            Spacer()
        }
    }
}

#endif
