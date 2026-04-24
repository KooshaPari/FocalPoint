#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Monthly Retrospective ritual — end-of-month reflection.
///
/// Displays month summary: total focus hours, weekly breakdown with mini bar chart,
/// month-over-month deltas (focus hours, tasks, credits), theme & reflection.
/// Coachy with thoughtful pose + desaturated palette for reflection mood.
///
/// Traces to: planning-coach ritual (Monthly Retrospective).
struct MonthlyRetroView: View {
    @EnvironmentObject private var holder: CoreHolder

    // Monthly Retro state.
    @State private var retro: MonthlyRetroDto?
    @State private var retroLoading: Bool = true

    // Error surface.
    @State private var alertMessage: String?
    @State private var showAlert: Bool = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    if retroLoading {
                        coachyLoadingView(
                            isLoading: $retroLoading,
                            pose: .thoughtful,
                            emotion: .focused,
                            reason: "Reflecting on your month…"
                        )
                    } else if let retro {
                        monthlyRetroContent(retro)
                    } else {
                        emptyStateCard
                    }
                }
                .padding()
            }
            .navigationTitle("Monthly Retrospective")
            .background(Color.app.background.ignoresSafeArea())
        }
        .task(id: holder.revision) { await loadMonthlyRetro() }
        .alert("Monthly retrospective error", isPresented: $showAlert, presenting: alertMessage) { _ in
            Button("OK", role: .cancel) { alertMessage = nil }
        } message: { msg in
            Text(msg)
        }
    }

    // MARK: - Content Views

    private func monthlyRetroContent(_ retro: MonthlyRetroDto) -> some View {
        VStack(spacing: 20) {
            // Hero: Coachy thoughtful
            coachyHero(message: retro.coachyReflection, pose: .thoughtful, emotion: .focused)

            // Month header + total focus
            monthHeaderCard(retro)

            // Weekly breakdown with mini bars
            weeklyBreakdownCard(retro)

            // Month-over-month deltas
            if hasDeltas(retro) {
                deltasCard(retro)
            }

            // Theme + reflection
            themeReflectionCard(retro)

            // Share button
            shareButton(retro)
        }
    }

    private func coachyHero(message: String, pose: CoachyPose, emotion: CoachyEmotion) -> some View {
        VStack(spacing: 8) {
            Text("Monthly Reflection")
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
                .fill(Color.app.surface.opacity(0.8))
        )
    }

    private func monthHeaderCard(_ retro: MonthlyRetroDto) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            Text(retro.monthLabel)
                .font(.title.weight(.bold))
                .foregroundStyle(Color.app.foreground)

            HStack(spacing: 16) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Total Focus")
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                    HStack(alignment: .lastTextBaseline, spacing: 3) {
                        Text(String(format: "%.1f", retro.focusHoursTotal))
                            .font(.title2.weight(.bold))
                            .foregroundStyle(Color.app.foreground)
                        Text("hrs")
                            .font(.caption2)
                            .foregroundStyle(Color.app.foreground.opacity(0.6))
                    }
                }
                .accessibilityElement(children: .combine)
                .accessibilityLabel(String(localized: "Total focus hours: \(String(format: "%.1f", retro.focusHoursTotal)) hours", defaultValue: "Total focus hours: \(String(format: "%.1f", retro.focusHoursTotal)) hours"))

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Days Active")
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                    Text(String(retro.daysActive))
                        .font(.title2.weight(.bold))
                        .foregroundStyle(Color.app.foreground)
                }
                .accessibilityElement(children: .combine)
                .accessibilityLabel(String(localized: "Days active: \(retro.daysActive)", defaultValue: "Days active: \(retro.daysActive)"))
            }
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func weeklyBreakdownCard(_ retro: MonthlyRetroDto) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Weekly Breakdown")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))

            VStack(spacing: 10) {
                ForEach(Array(retro.weeklyFocusHours.enumerated()), id: \.offset) { idx, hours in
                    weeklyBarRow(week: idx + 1, hours: hours, maxHours: retro.weeklyFocusHours.max() ?? 1)
                }
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Weekly Breakdown", defaultValue: "Weekly Breakdown"))
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func weeklyBarRow(week: Int, hours: Double, maxHours: Double) -> some View {
        let percentage = maxHours > 0 ? hours / maxHours : 0
        return HStack(spacing: 12) {
            Text("W\(week)")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))
                .frame(width: 30)

            GeometryReader { geo in
                ZStack(alignment: .leading) {
                    RoundedRectangle(cornerRadius: 4, style: .continuous)
                        .fill(Color.app.foreground.opacity(0.1))

                    RoundedRectangle(cornerRadius: 4, style: .continuous)
                        .fill(
                            LinearGradient(
                                gradient: Gradient(colors: [Color.app.accent, Color.app.accent.opacity(0.7)]),
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                        .frame(width: geo.size.width * percentage)
                }
            }
            .frame(height: 24)

            Text(String(format: "%.1f h", hours))
                .font(.caption2.weight(.semibold))
                .foregroundStyle(Color.app.foreground)
                .frame(width: 45, alignment: .trailing)
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Week \(week): \(String(format: "%.1f", hours)) hours", defaultValue: "Week \(week): \(String(format: "%.1f", hours)) hours"))
    }

    private func deltasCard(_ retro: MonthlyRetroDto) -> some View {
        VStack(spacing: 12) {
            Text("Month-over-Month")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))
                .frame(maxWidth: .infinity, alignment: .leading)

            HStack(spacing: 12) {
                deltaChip(
                    icon: "flame.fill",
                    label: "Focus Hours",
                    delta: String(format: "%+.1f", retro.focusHoursDelta),
                    trend: retro.focusHoursDelta >= 0 ? "up" : "down"
                )
                deltaChip(
                    icon: "checkmark.circle.fill",
                    label: "Tasks",
                    delta: String(format: "%+d", retro.tasksCompletedDelta),
                    trend: retro.tasksCompletedDelta >= 0 ? "up" : "down"
                )
                deltaChip(
                    icon: "diamond.fill",
                    label: "Credits",
                    delta: String(format: "%+d", retro.creditsEarnedDelta),
                    trend: retro.creditsEarnedDelta >= 0 ? "up" : "down"
                )
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Month-over-Month Changes", defaultValue: "Month-over-Month Changes"))
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func deltaChip(icon: String, label: String, delta: String, trend: String) -> some View {
        let isPositive = delta.first != "-"
        let color: Color = isPositive ? .green : (delta == "+0" ? Color.app.foreground.opacity(0.5) : .red)

        return VStack(alignment: .center, spacing: 6) {
            Image(systemName: icon)
                .foregroundStyle(color)
            Text(label)
                .font(.caption2)
                .foregroundStyle(Color.app.foreground.opacity(0.7))
            HStack(spacing: 2) {
                Image(systemName: trend == "up" ? "arrow.up.right" : "arrow.down.right")
                    .font(.caption2.weight(.bold))
                Text(delta)
                    .font(.subheadline.weight(.bold))
            }
            .foregroundStyle(color)
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "\(label): \(delta)", defaultValue: "\(label): \(delta)"))
        .frame(maxWidth: .infinity)
        .padding(.vertical, 10)
        .background(
            RoundedRectangle(cornerRadius: 12, style: .continuous)
                .fill(Color.app.background)
        )
    }

    private func themeReflectionCard(_ retro: MonthlyRetroDto) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            if !retro.focusTheme.isEmpty {
                VStack(alignment: .leading, spacing: 6) {
                    Text("Theme")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(Color.app.accent)
                    Text(retro.focusTheme)
                        .font(.subheadline)
                        .foregroundStyle(Color.app.foreground)
                }
                .accessibilityElement(children: .combine)
                .accessibilityLabel(String(localized: "Theme: \(retro.focusTheme)", defaultValue: "Theme: \(retro.focusTheme)"))

                Divider()
                    .background(Color.app.foreground.opacity(0.15))
            }

            if !retro.reflection.isEmpty {
                VStack(alignment: .leading, spacing: 6) {
                    Text("Reflection")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(Color.orange)
                    Text(retro.reflection)
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.85))
                        .lineLimit(6)
                }
                .accessibilityElement(children: .combine)
                .accessibilityLabel(String(localized: "Reflection: \(retro.reflection)", defaultValue: "Reflection: \(retro.reflection)"))
            }
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface.opacity(0.8))
        )
    }

    private func shareButton(_ retro: MonthlyRetroDto) -> some View {
        Button(action: { shareToJournal(retro) }) {
            HStack(spacing: 8) {
                Image(systemName: "square.and.arrow.up")
                Text("Share to Journal")
            }
            .font(.subheadline.weight(.semibold))
            .frame(maxWidth: .infinity)
            .padding(.vertical, 12)
            .background(Color.app.accent.opacity(0.8))
            .foregroundStyle(Color.app.accentOn)
            .clipShape(RoundedRectangle(cornerRadius: 12, style: .continuous))
        }
        .accessibilityLabel(String(localized: "Share monthly retrospective to journal", defaultValue: "Share monthly retrospective to journal"))
    }

    private var emptyStateCard: some View {
        VStack(spacing: 12) {
            CoachyView(
                state: CoachyState(pose: .thoughtful, emotion: .focused, bubbleText: "No month data yet — check back later."),
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
    private func loadMonthlyRetro() async {
        retroLoading = true
        defer { retroLoading = false }
        do {
            let r = try holder.core.rituals().generateMonthlyRetro()
            withAnimation { retro = r }
        } catch {
            showError("Couldn't load monthly retrospective: \(error)")
        }
    }

    // MARK: - Actions

    private func shareToJournal(_ retro: MonthlyRetroDto) {
        let markdown = generateMonthlyRetroMarkdown(retro)

        #if os(iOS)
        UIPasteboard.general.string = markdown

        let alert = UIAlertController(
            title: "Share to Journal",
            message: "Monthly retrospective copied to clipboard. Paste it into your journal app.",
            preferredStyle: .alert
        )
        alert.addAction(UIAlertAction(title: "OK", style: .default))

        if let scene = UIApplication.shared.connectedScenes.first as? UIWindowScene,
           let window = scene.windows.first {
            window.rootViewController?.present(alert, animated: true)
        }
        #endif
    }

    private func generateMonthlyRetroMarkdown(_ retro: MonthlyRetroDto) -> String {
        var md = "# Monthly Retrospective\n\n"
        md += "**\(retro.monthLabel)**\n\n"

        md += "## Summary\n"
        md += "- Total Focus Hours: \(String(format: "%.1f", retro.focusHoursTotal)) hrs\n"
        md += "- Days Active: \(retro.daysActive)\n\n"

        md += "## Weekly Breakdown\n"
        for (idx, hours) in retro.weeklyFocusHours.enumerated() {
            md += "- Week \(idx + 1): \(String(format: "%.1f", hours)) hrs\n"
        }
        md += "\n"

        md += "## Month-over-Month\n"
        md += "- Focus Hours: \(String(format: "%+.1f", retro.focusHoursDelta))\n"
        md += "- Tasks Completed: \(String(format: "%+d", retro.tasksCompletedDelta))\n"
        md += "- Credits Earned: \(String(format: "%+d", retro.creditsEarnedDelta))\n\n"

        if !retro.focusTheme.isEmpty {
            md += "## Theme\n"
            md += "\(retro.focusTheme)\n\n"
        }

        if !retro.reflection.isEmpty {
            md += "## Reflection\n"
            md += "\(retro.reflection)\n"
        }

        return md
    }

    private func hasDeltas(_ retro: MonthlyRetroDto) -> Bool {
        retro.focusHoursDelta != 0 || retro.tasksCompletedDelta != 0 || retro.creditsEarnedDelta != 0
    }

    private func showError(_ msg: String) {
        alertMessage = msg
        showAlert = true
    }
}
#endif
