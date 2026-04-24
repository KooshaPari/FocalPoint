import SwiftUI
import SnapshotTesting
import XCTest
@testable import FocalPointApp
@testable import FocalPointCore

// Traces to: FR-RITUALS-WEEKLY (Weekly Review UI), FR-RITUALS-MONTHLY (Monthly Retrospective UI)

class RitualViewsSnapshotTests: XCTestCase {
    let record = false

    // MARK: - Weekly Review

    func testWeeklyReviewLoading() {
        let view = WeeklyReviewLoadingTestView()

        assertViewSnapshot(
            view: view,
            name: "weekly_review_loading",
            record: record
        )
    }

    func testWeeklyReviewLoaded() {
        let view = WeeklyReviewLoadedTestView()

        assertViewSnapshot(
            view: view,
            name: "weekly_review_loaded",
            record: record
        )
    }

    // MARK: - Monthly Retrospective

    func testMonthlyRetroLoading() {
        let view = MonthlyRetroLoadingTestView()

        assertViewSnapshot(
            view: view,
            name: "monthly_retro_loading",
            record: record
        )
    }

    func testMonthlyRetroLoaded() {
        let view = MonthlyRetroLoadedTestView()

        assertViewSnapshot(
            view: view,
            name: "monthly_retro_loaded",
            record: record
        )
    }
}

// MARK: - Weekly Review Test Views

struct WeeklyReviewLoadingTestView: View {
    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            VStack(spacing: 20) {
                VStack(spacing: 12) {
                    RoundedRectangle(cornerRadius: 12, style: .continuous)
                        .fill(Color.app.surface)
                        .frame(height: 200)

                    ProgressView()
                        .controlSize(.large)
                }
                .padding(20)
            }
            .padding()
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color.app.background)
    }
}

struct WeeklyReviewLoadedTestView: View {
    let review = WeeklyReviewDto(
        focusHoursTotal: 18.5,
        sessionsCompleted: 12,
        creditsEarned: 450,
        creditsSpent: 120,
        tasksCompleted: 8,
        tasksSlipped: 2,
        topRulesFired: ["Focus Block", "App Limit: Instagram", "Deep Work Session"],
        streaksExtended: ["Morning Focus", "No Distractions"],
        winsSummary: "Fantastic week! You hit your focus target and extended two streaks. Deep work sessions are becoming more consistent.",
        growthArea: "Try to complete slipped tasks earlier in the week rather than pushing them to Friday.",
        coachyOpening: "What a week! You've been absolutely crushing it with your focus time.",
        generatedAtIso: ISO8601DateFormatter().string(from: Date())
    )

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Hero section
                VStack(spacing: 8) {
                    Text("Your Week")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                        .frame(maxWidth: .infinity, alignment: .leading)

                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                        .frame(height: 120)
                }
                .padding()
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                )

                // Stats grid
                VStack(spacing: 12) {
                    HStack(spacing: 12) {
                        statCardPreview(icon: "flame.fill", label: "Focus Hours", value: "18.5", unit: "hrs")
                        statCardPreview(icon: "circle.fill", label: "Sessions", value: "12", unit: "")
                    }
                    HStack(spacing: 12) {
                        statCardPreview(icon: "diamond.fill", label: "Credits Earned", value: "450", unit: "")
                        statCardPreview(icon: "minus.circle.fill", label: "Credits Spent", value: "120", unit: "")
                    }
                }

                // Rules card
                VStack(alignment: .leading, spacing: 10) {
                    Text("Top Rules Fired")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                    ForEach(Array(review.topRulesFired.enumerated()), id: \.offset) { idx, rule in
                        HStack(spacing: 8) {
                            Image(systemName: "bolt.fill")
                                .foregroundStyle(Color.app.accent)
                                .frame(width: 20)
                            Text(rule)
                                .font(.subheadline)
                            Spacer()
                            Text("#\(idx + 1)")
                                .font(.caption2.weight(.bold))
                                .padding(.horizontal, 8).padding(.vertical, 3)
                                .background(Capsule().fill(Color.app.accent.opacity(0.18)))
                                .foregroundStyle(Color.app.accent)
                        }
                    }
                }
                .padding(16)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                )

                // Summary card
                VStack(alignment: .leading, spacing: 12) {
                    Text("Reflection")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(Color.app.foreground.opacity(0.7))

                    VStack(alignment: .leading, spacing: 4) {
                        Text("Wins")
                            .font(.footnote.weight(.semibold))
                            .foregroundStyle(Color.app.accent)
                        Text(review.winsSummary)
                            .font(.caption)
                            .foregroundStyle(Color.app.foreground.opacity(0.85))
                            .lineLimit(4)
                    }

                    Divider()

                    VStack(alignment: .leading, spacing: 4) {
                        Text("Growth Area")
                            .font(.footnote.weight(.semibold))
                            .foregroundStyle(Color.orange)
                        Text(review.growthArea)
                            .font(.caption)
                            .foregroundStyle(Color.app.foreground.opacity(0.85))
                            .lineLimit(4)
                    }
                }
                .padding(16)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                )
            }
            .padding()
        }
        .background(Color.app.background)
    }

    private func statCardPreview(icon: String, label: String, value: String, unit: String) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack(spacing: 6) {
                Image(systemName: icon)
                    .foregroundStyle(Color.app.accent)
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
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 12, style: .continuous)
                .fill(Color.app.background)
        )
    }
}

// MARK: - Monthly Retro Test Views

struct MonthlyRetroLoadingTestView: View {
    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            VStack(spacing: 20) {
                VStack(spacing: 12) {
                    RoundedRectangle(cornerRadius: 12, style: .continuous)
                        .fill(Color.app.surface)
                        .frame(height: 200)

                    ProgressView()
                        .controlSize(.large)
                }
                .padding(20)
            }
            .padding()
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color.app.background)
    }
}

struct MonthlyRetroLoadedTestView: View {
    let retro = MonthlyRetroDto(
        monthLabel: "April 2026",
        focusHoursTotal: 72.5,
        weeklyFocusHours: [16.2, 18.5, 19.0, 18.8],
        daysActive: 28,
        focusTheme: "Deep Work & Flow States",
        reflection: "This month brought significant growth in sustained focus. Daily rituals became more automatic, and we saw improved consistency across all weeks.",
        focusHoursDelta: 8.5,
        tasksCompletedDelta: 12,
        creditsEarnedDelta: 450,
        coachyReflection: "April was a strong month! Let's build on this momentum.",
        generatedAtIso: ISO8601DateFormatter().string(from: Date())
    )

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Hero section
                VStack(spacing: 8) {
                    Text("Monthly Reflection")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                        .frame(maxWidth: .infinity, alignment: .leading)

                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                        .frame(height: 120)
                }
                .padding()
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface.opacity(0.8))
                )

                // Month header
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
                                Text("72.5")
                                    .font(.title2.weight(.bold))
                                Text("hrs")
                                    .font(.caption2)
                                    .foregroundStyle(Color.app.foreground.opacity(0.6))
                            }
                        }

                        Spacer()

                        VStack(alignment: .trailing, spacing: 4) {
                            Text("Days Active")
                                .font(.caption)
                                .foregroundStyle(Color.app.foreground.opacity(0.7))
                            Text("28")
                                .font(.title2.weight(.bold))
                        }
                    }
                }
                .padding(16)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                )

                // Weekly breakdown
                VStack(alignment: .leading, spacing: 12) {
                    Text("Weekly Breakdown")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(Color.app.foreground.opacity(0.7))

                    ForEach(Array(retro.weeklyFocusHours.enumerated()), id: \.offset) { idx, hours in
                        HStack(spacing: 12) {
                            Text("W\(idx + 1)")
                                .font(.caption.weight(.semibold))
                                .foregroundStyle(Color.app.foreground.opacity(0.7))
                                .frame(width: 30)

                            GeometryReader { geo in
                                ZStack(alignment: .leading) {
                                    RoundedRectangle(cornerRadius: 4, style: .continuous)
                                        .fill(Color.app.foreground.opacity(0.1))

                                    RoundedRectangle(cornerRadius: 4, style: .continuous)
                                        .fill(Color.app.accent)
                                        .frame(width: geo.size.width * (hours / 20.0))
                                }
                            }
                            .frame(height: 24)

                            Text(String(format: "%.1f h", hours))
                                .font(.caption2.weight(.semibold))
                                .foregroundStyle(Color.app.foreground)
                                .frame(width: 45, alignment: .trailing)
                        }
                    }
                }
                .padding(16)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface)
                )

                // Theme & Reflection
                VStack(alignment: .leading, spacing: 12) {
                    VStack(alignment: .leading, spacing: 6) {
                        Text("Theme")
                            .font(.caption.weight(.semibold))
                            .foregroundStyle(Color.app.accent)
                        Text(retro.focusTheme)
                            .font(.subheadline)
                    }

                    Divider()

                    VStack(alignment: .leading, spacing: 6) {
                        Text("Reflection")
                            .font(.caption.weight(.semibold))
                            .foregroundStyle(Color.orange)
                        Text(retro.reflection)
                            .font(.caption)
                            .foregroundStyle(Color.app.foreground.opacity(0.85))
                            .lineLimit(6)
                    }
                }
                .padding(16)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(Color.app.surface.opacity(0.8))
                )
            }
            .padding()
        }
        .background(Color.app.background)
    }
}
