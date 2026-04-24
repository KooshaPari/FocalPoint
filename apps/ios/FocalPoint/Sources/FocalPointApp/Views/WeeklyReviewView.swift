#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Weekly Review ritual — Sunday evening or on-demand.
///
/// Surfaces weekly focus metrics: total focus hours, sessions completed,
/// credits earned/spent, top-3 rules fired, streaks extended, tasks completed/slipped.
/// Coachy celebrates with proud pose + subtle confetti. Includes "Share to Journal" button
/// to save snapshot to Photos or write to Files.app.
///
/// Traces to: planning-coach ritual (Weekly Review).
struct WeeklyReviewView: View {
    @EnvironmentObject private var holder: CoreHolder

    // Weekly Review state.
    @State private var review: WeeklyReviewDto?
    @State private var reviewLoading: Bool = true

    // Error surface.
    @State private var alertMessage: String?
    @State private var showAlert: Bool = false

    // Confetti trigger
    @State private var showConfetti: Bool = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    if reviewLoading {
                        coachyLoadingView(
                            isLoading: $reviewLoading,
                            pose: .curious,
                            emotion: .focused,
                            reason: "Compiling your week…"
                        )
                    } else if let review {
                        weeklyReviewContent(review)
                    } else {
                        emptyStateCard
                    }
                }
                .padding()
            }
            .navigationTitle("Weekly Review")
            .background(Color.app.background.ignoresSafeArea())
        }
        .task(id: holder.revision) { await loadWeeklyReview() }
        .alert("Weekly review error", isPresented: $showAlert, presenting: alertMessage) { _ in
            Button("OK", role: .cancel) { alertMessage = nil }
        } message: { msg in
            Text(msg)
        }
    }

    // MARK: - Content Views

    private func weeklyReviewContent(_ review: WeeklyReviewDto) -> some View {
        VStack(spacing: 20) {
            // Hero: Coachy celebrating
            coachyHero(message: review.coachyOpening, pose: .proud, emotion: .happy)

            // Stats strip
            statsGrid(review)

            // Rules fired card
            if !review.topRulesFired.isEmpty {
                topRulesCard(review.topRulesFired)
            }

            // Tasks summary
            if review.tasksCompleted > 0 || review.tasksSlipped > 0 {
                tasksSummaryCard(review)
            }

            // Streaks extended
            if !review.streaksExtended.isEmpty {
                streaksCard(review.streaksExtended)
            }

            // Wins summary + Growth area
            summaryCard(review)

            // Share button
            shareButton(review)
        }
        .onAppear {
            withAnimation(.easeInOut(duration: 0.6)) {
                showConfetti = !review.streaksExtended.isEmpty
            }
        }
    }

    private func coachyHero(message: String, pose: CoachyPose, emotion: CoachyEmotion) -> some View {
        VStack(spacing: 8) {
            Text("Your Week")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))
                .frame(maxWidth: .infinity, alignment: .leading)
            CoachyView(
                state: CoachyState(pose: pose, emotion: emotion, bubbleText: message),
                size: 240
            )
            .frame(maxWidth: .infinity)
        }
        .padding()
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func statsGrid(_ review: WeeklyReviewDto) -> some View {
        VStack(spacing: 12) {
            HStack(spacing: 12) {
                statCard(
                    icon: "flame.fill",
                    label: "Focus Hours",
                    value: String(format: "%.1f", review.focusHoursTotal),
                    unit: "hrs"
                )
                statCard(
                    icon: "circle.fill",
                    label: "Sessions",
                    value: String(review.sessionsCompleted),
                    unit: ""
                )
            }
            HStack(spacing: 12) {
                statCard(
                    icon: "diamond.fill",
                    label: "Credits Earned",
                    value: String(review.creditsEarned),
                    unit: ""
                )
                statCard(
                    icon: "minus.circle.fill",
                    label: "Credits Spent",
                    value: String(review.creditsSpent),
                    unit: ""
                )
            }
        }
    }

    private func statCard(icon: String, label: String, value: String, unit: String) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack(spacing: 6) {
                Image(systemName: icon)
                    .foregroundStyle(Color.app.accent)
                    .accessibilityLabel(String(localized: label, defaultValue: label))
                Text(label)
                    .font(.caption2)
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
            }
            HStack(alignment: .lastTextBaseline, spacing: 3) {
                Text(value)
                    .font(.title2.weight(.bold))
                    .foregroundStyle(Color.app.foreground)
                if !unit.isEmpty {
                    Text(unit)
                        .font(.caption2)
                        .foregroundStyle(Color.app.foreground.opacity(0.6))
                }
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "\(label): \(value) \(unit)", defaultValue: "\(label): \(value) \(unit)"))
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 12, style: .continuous)
                .fill(Color.app.background)
        )
    }

    private func topRulesCard(_ rules: [String]) -> some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Top Rules Fired")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))
            ForEach(Array(rules.prefix(3).enumerated()), id: \.offset) { idx, rule in
                HStack(spacing: 8) {
                    Image(systemName: "bolt.fill")
                        .foregroundStyle(Color.app.accent)
                        .frame(width: 20)
                    Text(rule)
                        .font(.subheadline)
                        .foregroundStyle(Color.app.foreground)
                    Spacer()
                    Text("#\(idx + 1)")
                        .font(.caption2.weight(.bold))
                        .padding(.horizontal, 8).padding(.vertical, 3)
                        .background(Capsule().fill(Color.app.accent.opacity(0.18)))
                        .foregroundStyle(Color.app.accent)
                }
                .accessibilityLabel(String(localized: "Rule \(idx + 1): \(rule)", defaultValue: "Rule \(idx + 1): \(rule)"))
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Top Rules Fired", defaultValue: "Top Rules Fired"))
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func tasksSummaryCard(_ review: WeeklyReviewDto) -> some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Tasks Summary")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))

            HStack(spacing: 16) {
                VStack(alignment: .leading, spacing: 4) {
                    HStack(spacing: 4) {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundStyle(Color.app.accent)
                        Text("Completed")
                            .font(.caption)
                            .foregroundStyle(Color.app.foreground.opacity(0.7))
                    }
                    Text(String(review.tasksCompleted))
                        .font(.title3.weight(.bold))
                        .foregroundStyle(Color.app.foreground)
                }
                .accessibilityElement(children: .combine)
                .accessibilityLabel(String(localized: "Tasks completed: \(review.tasksCompleted)", defaultValue: "Tasks completed: \(review.tasksCompleted)"))

                Spacer()

                VStack(alignment: .leading, spacing: 4) {
                    HStack(spacing: 4) {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundStyle(.orange)
                        Text("Slipped")
                            .font(.caption)
                            .foregroundStyle(Color.app.foreground.opacity(0.7))
                    }
                    Text(String(review.tasksSlipped))
                        .font(.title3.weight(.bold))
                        .foregroundStyle(Color.app.foreground)
                }
                .accessibilityElement(children: .combine)
                .accessibilityLabel(String(localized: "Tasks slipped: \(review.tasksSlipped)", defaultValue: "Tasks slipped: \(review.tasksSlipped)"))
            }
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func streaksCard(_ streaks: [String]) -> some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Streaks Extended")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))
            HStack(spacing: 8) {
                ForEach(Array(streaks.prefix(3)), id: \.self) { streak in
                    Text(streak)
                        .font(.caption2.weight(.bold))
                        .padding(.horizontal, 10).padding(.vertical, 5)
                        .background(Capsule().fill(Color.app.accent.opacity(0.18)))
                        .foregroundStyle(Color.app.accent)
                        .accessibilityLabel(String(localized: "Streak: \(streak)", defaultValue: "Streak: \(streak)"))
                }
                Spacer()
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Streaks Extended", defaultValue: "Streaks Extended"))
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func summaryCard(_ review: WeeklyReviewDto) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Reflection")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))

            if !review.winsSummary.isEmpty {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Wins")
                        .font(.footnote.weight(.semibold))
                        .foregroundStyle(Color.app.accent)
                    Text(review.winsSummary)
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.85))
                        .lineLimit(4)
                }
                .accessibilityElement(children: .combine)
                .accessibilityLabel(String(localized: "Wins: \(review.winsSummary)", defaultValue: "Wins: \(review.winsSummary)"))
            }

            Divider()
                .background(Color.app.foreground.opacity(0.15))

            if !review.growthArea.isEmpty {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Growth Area")
                        .font(.footnote.weight(.semibold))
                        .foregroundStyle(Color.orange)
                    Text(review.growthArea)
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.85))
                        .lineLimit(4)
                }
                .accessibilityElement(children: .combine)
                .accessibilityLabel(String(localized: "Growth area: \(review.growthArea)", defaultValue: "Growth area: \(review.growthArea)"))
            }
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func shareButton(_ review: WeeklyReviewDto) -> some View {
        Button(action: { shareToJournal(review) }) {
            HStack(spacing: 8) {
                Image(systemName: "square.and.arrow.up")
                Text("Share to Journal")
            }
            .font(.subheadline.weight(.semibold))
            .frame(maxWidth: .infinity)
            .padding(.vertical, 12)
            .background(Color.app.accent)
            .foregroundStyle(Color.app.accentOn)
            .clipShape(RoundedRectangle(cornerRadius: 12, style: .continuous))
        }
        .accessibilityLabel(String(localized: "Share weekly review to journal", defaultValue: "Share weekly review to journal"))
    }

    private var emptyStateCard: some View {
        VStack(spacing: 12) {
            CoachyView(
                state: CoachyState(pose: .encouraging, emotion: .happy, bubbleText: "No week data yet — check back later."),
                size: 180
            )
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 28)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    @ViewBuilder
    private func coachyLoadingView(
        isLoading: Binding<Bool>,
        pose: CoachyPose,
        emotion: CoachyEmotion,
        reason: String
    ) -> some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            VStack(spacing: 20) {
                CoachyView(
                    state: CoachyState(pose: pose, emotion: emotion, bubbleText: reason),
                    size: 200
                )
                ProgressView()
                    .controlSize(.large)
            }
            .padding()
        }
    }

    // MARK: - Loaders

    @MainActor
    private func loadWeeklyReview() async {
        reviewLoading = true
        defer { reviewLoading = false }
        do {
            let r = try holder.core.rituals().generateWeeklyReview()
            withAnimation { review = r }
        } catch {
            showError("Couldn't load weekly review: \(error)")
        }
    }

    // MARK: - Actions

    private func shareToJournal(_ review: WeeklyReviewDto) {
        let markdown = generateWeeklyReviewMarkdown(review)

        // Try to save to Photos first, fallback to Files.app paste
        #if os(iOS)
        UIPasteboard.general.string = markdown

        let alert = UIAlertController(
            title: "Share to Journal",
            message: "Weekly review copied to clipboard. Paste it into your journal app.",
            preferredStyle: .alert
        )
        alert.addAction(UIAlertAction(title: "OK", style: .default))

        if let scene = UIApplication.shared.connectedScenes.first as? UIWindowScene,
           let window = scene.windows.first {
            window.rootViewController?.present(alert, animated: true)
        }
        #endif
    }

    private func generateWeeklyReviewMarkdown(_ review: WeeklyReviewDto) -> String {
        let weekEnd = ISO8601DateFormatter().string(from: Date())
        var md = "# Weekly Review\n\n"
        md += "**Week ending:** \(weekEnd)\n\n"

        md += "## Metrics\n"
        md += "- Focus Hours: \(String(format: "%.1f", review.focusHoursTotal)) hrs\n"
        md += "- Sessions: \(review.sessionsCompleted)\n"
        md += "- Credits Earned: +\(review.creditsEarned)\n"
        md += "- Credits Spent: -\(review.creditsSpent)\n"
        md += "- Tasks Completed: \(review.tasksCompleted)\n"
        md += "- Tasks Slipped: \(review.tasksSlipped)\n\n"

        if !review.topRulesFired.isEmpty {
            md += "## Top Rules\n"
            for (idx, rule) in review.topRulesFired.prefix(3).enumerated() {
                md += "- #\(idx + 1): \(rule)\n"
            }
            md += "\n"
        }

        if !review.streaksExtended.isEmpty {
            md += "## Streaks Extended\n"
            for streak in review.streaksExtended {
                md += "- \(streak)\n"
            }
            md += "\n"
        }

        md += "## Reflection\n"
        if !review.winsSummary.isEmpty {
            md += "**Wins:** \(review.winsSummary)\n\n"
        }
        if !review.growthArea.isEmpty {
            md += "**Growth Area:** \(review.growthArea)\n"
        }

        return md
    }

    private func showError(_ msg: String) {
        alertMessage = msg
        showAlert = true
    }
}
#endif
