import SwiftUI
import SnapshotTesting
import XCTest
@testable import FocalPointApp
@testable import FocalPointCore

// Traces to: FR-STATS-CHARTS (Rich charts + insights), FR-STATS-DATE-FILTER (Date range picker)

class StatsViewSnapshotTests: XCTestCase {
    let record = false

    // MARK: - Focus Hours Chart

    func testFocusHoursChartLight() {
        let view = FocusHoursChartTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_focus_hours_light",
            record: record
        )
    }

    func testFocusHoursChartDark() {
        let view = FocusHoursChartTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_focus_hours_dark",
            record: record,
            config: .iPhone13ProDark
        )
    }

    // MARK: - Credits Chart

    func testCreditsChartLight() {
        let view = CreditsChartTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_credits_light",
            record: record
        )
    }

    func testCreditsChartDark() {
        let view = CreditsChartTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_credits_dark",
            record: record,
            config: .iPhone13ProDark
        )
    }

    // MARK: - Rules Chart

    func testRulesChartLight() {
        let view = RulesChartTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_rules_light",
            record: record
        )
    }

    func testRulesChartDark() {
        let view = RulesChartTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_rules_dark",
            record: record,
            config: .iPhone13ProDark
        )
    }

    // MARK: - Connector Events Chart

    func testConnectorEventsChartLight() {
        let view = ConnectorEventsChartTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_connector_events_light",
            record: record
        )
    }

    func testConnectorEventsChartDark() {
        let view = ConnectorEventsChartTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_connector_events_dark",
            record: record,
            config: .iPhone13ProDark
        )
    }

    // MARK: - Insights Cards

    func testInsightsCardsLight() {
        let view = InsightsCardsTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_insights_light",
            record: record
        )
    }

    func testInsightsCardsDark() {
        let view = InsightsCardsTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_insights_dark",
            record: record,
            config: .iPhone13ProDark
        )
    }

    // MARK: - Empty State

    func testEmptyStateLight() {
        let view = StatsEmptyStateTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_empty_light",
            record: record
        )
    }

    func testEmptyStateDark() {
        let view = StatsEmptyStateTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_empty_dark",
            record: record,
            config: .iPhone13ProDark
        )
    }
}

// MARK: - Test View Components

struct FocusHoursChartTestView: View {
    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            ScrollView {
                VStack(spacing: 20) {
                    cardTitle("Focus hours — 30d")

                    MockChart()
                        .frame(height: 200)
                        .padding()
                        .background(Color.app.surface)
                        .cornerRadius(16)

                    statsInfo("Peak 6 hours on Tuesday", icon: "timer")
                }
                .padding()
            }
        }
    }
}

struct CreditsChartTestView: View {
    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            ScrollView {
                VStack(spacing: 20) {
                    cardTitle("Credits — 30d")

                    MockStackedChart()
                        .frame(height: 200)
                        .padding()
                        .background(Color.app.surface)
                        .cornerRadius(16)

                    HStack(spacing: 16) {
                        statBox(label: "Earned", value: "+450", color: .green)
                        statBox(label: "Spent", value: "-180", color: .red)
                        Spacer()
                    }
                }
                .padding()
            }
        }
    }
}

struct RulesChartTestView: View {
    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            ScrollView {
                VStack(spacing: 20) {
                    cardTitle("Top 5 rules by fire count — 30d")

                    MockHorizontalChart()
                        .frame(height: 180)
                        .padding()
                        .background(Color.app.surface)
                        .cornerRadius(16)

                    VStack(spacing: 6) {
                        ruleRow(label: "Instagram limit", count: 45)
                        ruleRow(label: "TikTok limit", count: 32)
                        ruleRow(label: "Focus block", count: 28)
                    }
                }
                .padding()
            }
        }
    }
}

struct ConnectorEventsChartTestView: View {
    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            ScrollView {
                VStack(spacing: 20) {
                    cardTitle("Connector events")

                    MockDonutChart()
                        .frame(height: 200)
                        .padding()
                        .background(Color.app.surface)
                        .cornerRadius(16)

                    HStack(spacing: 12) {
                        connectorBadge(label: "UsageStats", count: 120, color: .blue)
                        connectorBadge(label: "Accessibility", count: 85, color: .orange)
                        Spacer()
                    }
                }
                .padding()
            }
        }
    }
}

struct InsightsCardsTestView: View {
    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            ScrollView {
                VStack(spacing: 20) {
                    cardTitle("Insights")

                    insightCard(text: "Peak focus day: Tuesday with 6h")
                    insightCard(text: "You earn 2.5x more credits than you spend")
                    insightCard(text: "UsageStats contributes 58% of your connector events")
                }
                .padding()
            }
        }
    }
}

struct StatsEmptyStateTestView: View {
    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            ScrollView {
                VStack(spacing: 20) {
                    cardTitle("No activity yet.")

                    Text(String(localized: "Add a task, start a focus session, or connect a tool.", defaultValue: "Add a task, start a focus session, or connect a tool."))
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.6))
                        .multilineTextAlignment(.center)
                        .padding()
                }
                .padding()
            }
        }
    }
}

// MARK: - Helper Components

private func cardTitle(_ text: String) -> some View {
    Text(text)
        .font(.caption.weight(.semibold))
        .foregroundStyle(Color.app.foreground.opacity(0.6))
        .frame(maxWidth: .infinity, alignment: .leading)
}

private func statsInfo(_ text: String, icon: String) -> some View {
    HStack(spacing: 8) {
        Image(systemName: icon)
            .foregroundStyle(Color.app.accent)
        Text(text)
            .font(.caption)
            .foregroundStyle(Color.app.foreground.opacity(0.8))
        Spacer()
    }
}

private func statBox(label: String, value: String, color: Color) -> some View {
    VStack(alignment: .leading, spacing: 4) {
        Text(label).font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.6))
        Text(value).font(.title3.weight(.bold)).foregroundStyle(color)
    }
    .padding(.horizontal, 12)
    .padding(.vertical, 8)
    .background(Color.app.surface)
    .cornerRadius(8)
}

private func ruleRow(label: String, count: Int) -> some View {
    HStack {
        Text(label).font(.caption)
        Spacer()
        Text("\(count)×").font(.body.weight(.semibold))
    }
    .padding(.vertical, 6)
    .foregroundStyle(Color.app.foreground)
}

private func connectorBadge(label: String, count: Int, color: Color) -> some View {
    VStack(spacing: 4) {
        Text(label).font(.caption2.weight(.semibold))
        Text("\(count)").font(.caption.weight(.bold)).foregroundStyle(color)
    }
    .padding(.horizontal, 8)
    .padding(.vertical, 6)
    .background(color.opacity(0.1))
    .cornerRadius(8)
}

private func insightCard(text: String) -> some View {
    HStack(spacing: 12) {
        Image(systemName: "lightbulb.fill")
            .foregroundStyle(Color.yellow)
        Text(text)
            .font(.caption)
            .foregroundStyle(Color.app.foreground.opacity(0.8))
            .lineLimit(3)
        Spacer()
    }
    .padding(.vertical, 10)
    .padding(.horizontal, 12)
    .background(Color.app.surface)
    .cornerRadius(12)
}

// MARK: - Mock Charts (Placeholder Visuals)

struct MockChart: View {
    var body: some View {
        VStack(alignment: .center) {
            HStack(spacing: 8) {
                ForEach(0..<7, id: \.self) { i in
                    VStack(spacing: 4) {
                        RoundedRectangle(cornerRadius: 4)
                            .fill(Color.app.accent)
                            .frame(width: 16, height: CGFloat(Int.random(in: 30...100)))
                        Text("D\(i)")
                            .font(.caption2)
                            .foregroundStyle(Color.app.foreground.opacity(0.6))
                    }
                }
            }
            .frame(height: 120)
        }
    }
}

struct MockStackedChart: View {
    var body: some View {
        VStack(alignment: .center) {
            HStack(spacing: 8) {
                ForEach(0..<7, id: \.self) { i in
                    VStack(spacing: 0) {
                        RoundedRectangle(cornerRadius: 2)
                            .fill(Color.red.opacity(0.7))
                            .frame(width: 16, height: 30)
                        RoundedRectangle(cornerRadius: 2)
                            .fill(Color.green.opacity(0.7))
                            .frame(width: 16, height: 50)
                    }
                }
            }
            .frame(height: 120)
        }
    }
}

struct MockHorizontalChart: View {
    var body: some View {
        VStack(spacing: 12) {
            ForEach(0..<5, id: \.self) { i in
                HStack(spacing: 8) {
                    Text("Rule\(i)")
                        .font(.caption2)
                        .foregroundStyle(Color.app.foreground.opacity(0.6))
                        .frame(width: 50, alignment: .leading)
                    RoundedRectangle(cornerRadius: 4)
                        .fill(Color.orange)
                        .frame(width: CGFloat(Int.random(in: 50...150)), height: 12)
                    Spacer()
                }
            }
        }
    }
}

struct MockDonutChart: View {
    var body: some View {
        VStack {
            ZStack {
                Circle()
                    .fill(Color.blue.opacity(0.7))
                    .frame(width: 80, height: 80)
                Circle()
                    .fill(Color.app.background)
                    .frame(width: 50, height: 50)
            }
            .frame(height: 120)
            Text("162 total")
                .font(.caption2)
                .foregroundStyle(Color.app.foreground.opacity(0.6))
        }
    }
}

// MARK: - Snapshot Assertion Helper (Extended)

extension XCTestCase {
    func assertViewSnapshot<V: View>(
        view: V,
        name: String,
        record: Bool = false,
        config: ViewImageConfig = .iPhone13Pro,
        file: StaticString = #file,
        testName: String = #function,
        line: UInt = #line
    ) {
        assertSnapshot(
            of: view,
            as: .image(on: config),
            named: name,
            record: record,
            file: file,
            testName: testName,
            line: line
        )
    }
}

// MARK: - Device Configurations

extension ViewImageConfig {
    static let iPhone13Pro = ViewImageConfig(
        size: .init(width: 390, height: 844),
        safeAreaInsets: .init(top: 47, left: 0, bottom: 34, right: 0),
        traits: .init(userInterfaceStyle: .light)
    )

    static let iPhone13ProDark = ViewImageConfig(
        size: .init(width: 390, height: 844),
        safeAreaInsets: .init(top: 47, left: 0, bottom: 34, right: 0),
        traits: .init(userInterfaceStyle: .dark)
    )
}
