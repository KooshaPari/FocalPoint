#if canImport(SwiftUI)
import SwiftUI
import Charts
import DesignSystem
import MascotUI
import FocalPointCore

// Traces to: FR-STATS-CHARTS (Rich charts + insights), FR-STATS-DATE-FILTER (Date range picker)

/// Rich stats dashboard with SwiftUI Charts framework. Aggregates audit.recent(limit: 500)
/// in-process showing focus hours (30d + 7d avg), credits (14d stacked bar), top 5 rules,
/// and connector events (donut). Includes insights cards, date-range picker (7d/30d/90d/All),
/// pull-to-refresh, and VoiceOver summaries.
struct StatsView: View {
    @EnvironmentObject private var holder: CoreHolder
    @State private var records: [AuditRecordDto] = []
    @State private var wallet: WalletSummary?
    @State private var loadError: String?
    @State private var selectedRange: DateRange = .thirtyDays
    @State private var isRefreshing = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    header
                    dateRangePicker

                    if records.isEmpty {
                        empty
                    } else {
                        focusHoursChart
                        creditFlowChart
                        rulesChart
                        connectorEventsChart
                        insightsCards
                    }
                    if let e = loadError {
                        Text(e).font(.caption2).foregroundStyle(.red)
                    }
                }
                .padding()
            }
            .navigationTitle("Stats")
            .background(Color.app.background.ignoresSafeArea())
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button { reload() } label: {
                        Image(systemName: "arrow.clockwise")
                            .accessibilityLabel(String(localized: "Reload stats", defaultValue: "Reload stats"))
                    }
                }
            }
            .task(id: holder.revision) { reload() }
            .refreshable { await refreshData() }
        }
    }

    // MARK: - UI Components

    private var header: some View {
        HStack {
            CoachyView(
                state: CoachyState(pose: .confident, emotion: .proud, bubbleText: "Your insights await."),
                size: 100
            )
            Spacer()
        }
    }

    private var dateRangePicker: some View {
        HStack(spacing: 8) {
            ForEach(DateRange.allCases, id: \.self) { range in
                Button(action: { selectedRange = range }) {
                    Text(range.label)
                        .font(.caption.weight(.semibold))
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 8)
                        .background(selectedRange == range ? Color.app.accent : Color.app.surface)
                        .foregroundStyle(selectedRange == range ? .white : Color.app.foreground)
                        .cornerRadius(8)
                }
            }
        }
        .accessibilityLabel(String(localized: "Date range selector", defaultValue: "Date range selector"))
    }

    private var empty: some View {
        VStack(spacing: 8) {
            Text(String(localized: "No activity yet.", defaultValue: "No activity yet."))
                .font(.body.weight(.semibold))
            Text(String(localized: "Add a task, start a focus session, or connect a tool.", defaultValue: "Add a task, start a focus session, or connect a tool."))
                .font(.caption)
                .foregroundStyle(Color.app.foreground.opacity(0.6))
                .multilineTextAlignment(.center)
        }
        .padding()
    }

    // MARK: - Charts

    private var focusHoursChart: some View {
        let data = focusHoursChartData(range: selectedRange)
        let avgLine = data.isEmpty ? 0 : Double(data.map(\.hours).reduce(0, +)) / Double(data.count)

        return card(title: String(localized: "Focus hours — \(selectedRange.label)", defaultValue: "Focus hours")) {
            if data.isEmpty {
                Text(String(localized: "No focus data.", defaultValue: "No focus data."))
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
            } else {
                Chart(data, id: \.date) { item in
                    BarMark(
                        x: .value("Date", item.date, unit: .day),
                        y: .value("Hours", item.hours)
                    )
                    .foregroundStyle(Color.app.accent)

                    LineMark(
                        x: .value("Date", item.date, unit: .day),
                        y: .value("Avg", avgLine)
                    )
                    .foregroundStyle(Color.app.accent.opacity(0.5))
                    .lineStyle(StrokeStyle(lineWidth: 2, dash: [5]))
                }
                .chartXAxis {
                    AxisMarks(values: .automatic(desiredCount: 5)) { value in
                        AxisValueLabel(format: .dateTime.month().day())
                    }
                }
                .chartYAxis {
                    AxisMarks(position: .leading)
                }
                .frame(height: 200)
                .accessibilityLabel(String(localized: "Focus hours chart", defaultValue: "Focus hours chart"))
                .accessibilityValue(String(localized: "Peak \(data.map(\.hours).max() ?? 0) hours", defaultValue: "Peak \(data.map(\.hours).max() ?? 0) hours"))
            }
        }
    }

    private var creditFlowChart: some View {
        let data = creditFlowChartData(range: selectedRange)

        return card(title: String(localized: "Credits — \(selectedRange.label)", defaultValue: "Credits")) {
            if data.isEmpty {
                Text(String(localized: "No credit activity.", defaultValue: "No credit activity."))
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
            } else {
                Chart(data, id: \.date) { item in
                    BarMark(
                        x: .value("Date", item.date, unit: .day),
                        yStart: .value("Spent", 0),
                        yEnd: .value("Spent", Double(item.spent))
                    )
                    .foregroundStyle(Color.red.opacity(0.7))

                    BarMark(
                        x: .value("Date", item.date, unit: .day),
                        yStart: .value("Spent", Double(item.spent)),
                        yEnd: .value("Spent", Double(item.spent + item.earned))
                    )
                    .foregroundStyle(Color.green.opacity(0.7))
                }
                .chartXAxis {
                    AxisMarks(values: .automatic(desiredCount: 5)) { value in
                        AxisValueLabel(format: .dateTime.month().day())
                    }
                }
                .chartYAxis {
                    AxisMarks(position: .leading)
                }
                .frame(height: 200)
                .accessibilityLabel(String(localized: "Credits stacked bar chart", defaultValue: "Credits stacked bar chart"))
                .accessibilityValue(String(localized: "Earned \(data.map(\.earned).reduce(0, +)), Spent \(data.map(\.spent).reduce(0, +))", defaultValue: "Earned, Spent"))
            }
        }
    }

    private var rulesChart: some View {
        let top = topRulesChartData(limit: 5)

        return card(title: String(localized: "Top 5 rules by fire count — 30d", defaultValue: "Top 5 rules")) {
            if top.isEmpty {
                Text(String(localized: "No rules fired.", defaultValue: "No rules fired."))
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
            } else {
                Chart(top, id: \.id) { item in
                    BarMark(
                        x: .value("Fires", item.count),
                        y: .value("Rule", item.label)
                    )
                    .foregroundStyle(Color.orange)
                }
                .chartXAxis {
                    AxisMarks(position: .bottom)
                }
                .frame(height: 180)
                .accessibilityLabel(String(localized: "Top rules horizontal bar chart", defaultValue: "Top rules chart"))
                .accessibilityValue(String(localized: "Top rule fired \(top.first?.count ?? 0) times", defaultValue: "Top rule fires"))
            }
        }
    }

    private var connectorEventsChart: some View {
        let events = connectorEventsData()

        return card(title: String(localized: "Connector events", defaultValue: "Connector events")) {
            if events.isEmpty {
                Text(String(localized: "No connector events.", defaultValue: "No connector events."))
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
            } else {
                Chart(events, id: \.source) { item in
                    SectorMark(
                        angle: .value("Count", item.count),
                        innerRadius: .ratio(0.6),
                        angularInset: 2
                    )
                    .foregroundStyle(by: .value("Source", item.source))
                    .opacity(0.8)
                }
                .chartBackground { chartProxy in
                    VStack {
                        Text("\(events.map(\.count).reduce(0, +)) events")
                            .font(.caption.weight(.semibold))
                    }
                }
                .frame(height: 200)
                .accessibilityLabel(String(localized: "Connector events donut chart", defaultValue: "Connector events chart"))
                .accessibilityValue(String(localized: events.map { "\($0.source): \($0.count)" }.joined(separator: ", "), defaultValue: "Connector breakdown"))
            }
        }
    }

    private var insightsCards: some View {
        let insights = generateInsights()

        return VStack(spacing: 12) {
            Text(String(localized: "Insights", defaultValue: "Insights"))
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.6))
                .frame(maxWidth: .infinity, alignment: .leading)

            ForEach(insights, id: \.self) { insight in
                insightCard(text: insight)
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Insights section", defaultValue: "Insights"))
    }

    private func insightCard(text: String) -> some View {
        HStack(spacing: 12) {
            Image(systemName: "lightbulb.fill")
                .foregroundStyle(Color.yellow)
                .accessibilityHidden(true)
            Text(text)
                .font(.caption)
                .foregroundStyle(Color.app.foreground.opacity(0.8))
                .lineLimit(3)
            Spacer()
        }
        .padding(.vertical, 10)
        .padding(.horizontal, 12)
        .background(
            RoundedRectangle(cornerRadius: 12, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    // MARK: - Data Models & Aggregation

    enum DateRange: CaseIterable {
        case sevenDays
        case thirtyDays
        case ninetyDays
        case all

        var label: String {
            switch self {
            case .sevenDays: return "7d"
            case .thirtyDays: return "30d"
            case .ninetyDays: return "90d"
            case .all: return "All"
            }
        }

        var days: Int? {
            switch self {
            case .sevenDays: return 7
            case .thirtyDays: return 30
            case .ninetyDays: return 90
            case .all: return nil
            }
        }
    }

    struct FocusHourItem: Identifiable {
        let id: String
        let date: Date
        let hours: Double
    }

    struct CreditFlowItem: Identifiable {
        let id: String
        let date: Date
        let earned: Int64
        let spent: Int64
    }

    struct TopRuleItem: Identifiable {
        let id: String
        let label: String
        let count: Int
    }

    struct ConnectorEventItem: Identifiable {
        let id: String
        let source: String
        let count: Int
    }

    private func focusHoursChartData(range: DateRange) -> [FocusHourItem] {
        let dayCount = range.days ?? 30
        let cutoffDate = Calendar.current.date(byAdding: .day, value: -dayCount, to: Date()) ?? Date()

        var byDay: [Date: Int] = [:]
        for rec in records where rec.recordType == "host.event.emitted" {
            guard let obj = parse(rec.payloadJson),
                  obj["event_type"] as? String == "focus:session_completed",
                  let min = (obj["payload"] as? [String: Any])?["minutes"] as? Int,
                  rec.timestamp >= cutoffDate else { continue }

            let day = Calendar.current.startOfDay(for: rec.timestamp)
            byDay[day, default: 0] += min
        }

        return byDay
            .sorted { $0.key < $1.key }
            .map { FocusHourItem(id: ISO8601DateFormatter().string(from: $0.key), date: $0.key, hours: Double($0.value) / 60.0) }
    }

    private func creditFlowChartData(range: DateRange) -> [CreditFlowItem] {
        let dayCount = range.days ?? 14
        let cutoffDate = Calendar.current.date(byAdding: .day, value: -dayCount, to: Date()) ?? Date()

        var byDay: [Date: (earned: Int64, spent: Int64)] = [:]
        for rec in records {
            guard rec.timestamp >= cutoffDate else { continue }
            let day = Calendar.current.startOfDay(for: rec.timestamp)

            if rec.recordType == "wallet.grant_credit" {
                guard let obj = parse(rec.payloadJson) else { continue }
                let amt = (obj["amount"] as? Int).map(Int64.init) ?? (obj["amount"] as? Int64) ?? 0
                byDay[day, default: (0, 0)].earned += amt
            } else if rec.recordType == "wallet.spend_credit" {
                guard let obj = parse(rec.payloadJson) else { continue }
                let amt = (obj["amount"] as? Int).map(Int64.init) ?? (obj["amount"] as? Int64) ?? 0
                byDay[day, default: (0, 0)].spent += amt
            }
        }

        return byDay
            .sorted { $0.key < $1.key }
            .map { CreditFlowItem(id: ISO8601DateFormatter().string(from: $0.key), date: $0.key, earned: $0.value.earned, spent: $0.value.spent) }
    }

    private func topRulesChartData(limit: Int) -> [TopRuleItem] {
        var tally: [String: Int] = [:]
        let cutoffDate = Calendar.current.date(byAdding: .day, value: -30, to: Date()) ?? Date()

        for rec in records where rec.recordType == "rule.fired" {
            guard rec.timestamp >= cutoffDate,
                  let obj = parse(rec.payloadJson),
                  let id = obj["rule_id"] as? String else { continue }
            tally[id, default: 0] += 1
        }

        return tally
            .map { TopRuleItem(id: $0.key, label: $0.key.prefix(12) + "…", count: $0.value) }
            .sorted { $0.count > $1.count }
            .prefix(limit)
            .map { $0 }
    }

    private func connectorEventsData() -> [ConnectorEventItem] {
        var bySource: [String: Int] = [:]

        for rec in records where rec.recordType == "connector.event_received" {
            guard let obj = parse(rec.payloadJson),
                  let source = obj["connector_source"] as? String else { continue }
            bySource[source, default: 0] += 1
        }

        return bySource
            .map { ConnectorEventItem(id: $0.key, source: $0.key, count: $0.value) }
            .sorted { $0.count > $1.count }
    }

    private func generateInsights() -> [String] {
        var insights: [String] = []

        let focusData = focusHoursChartData(range: selectedRange)
        if let peak = focusData.max(by: { $0.hours < $1.hours }) {
            let formatter = DateFormatter()
            formatter.dateFormat = "EEEE"
            let day = formatter.string(from: peak.date)
            insights.append(String(localized: "Peak focus day: \(day) with \(Int(peak.hours))h", defaultValue: "Peak focus day: \(day)"))
        }

        let creditData = creditFlowChartData(range: selectedRange)
        if !creditData.isEmpty {
            let totalEarned = creditData.map(\.earned).reduce(0, +)
            let totalSpent = creditData.map(\.spent).reduce(0, +)
            if totalEarned > 0 {
                let ratio = totalSpent > 0 ? Double(totalEarned) / Double(totalSpent) : 0
                insights.append(String(localized: "You earn \(String(format: "%.1f", ratio))x more credits than you spend", defaultValue: "Credit efficiency"))
            }
        }

        let rules = topRulesChartData(limit: 5)
        if !rules.isEmpty {
            insights.append(String(localized: "Top rule fires \(rules.first?.count ?? 0) times in this period", defaultValue: "Top rule activity"))
        }

        let connectors = connectorEventsData()
        if !connectors.isEmpty, let top = connectors.first {
            let pct = Int(Double(top.count) / Double(connectors.map(\.count).reduce(1, +)) * 100)
            insights.append(String(localized: "\(top.source) contributes \(pct)% of your connector events", defaultValue: "Top connector"))
        }

        return insights.isEmpty ? [String(localized: "Keep building your focus habits!", defaultValue: "Keep building your focus habits!")] : insights
    }

    // MARK: - Helpers

    private func parse(_ json: String) -> [String: Any]? {
        guard let data = json.data(using: .utf8) else { return nil }
        return (try? JSONSerialization.jsonObject(with: data)) as? [String: Any]
    }

    @ViewBuilder
    private func card(title: String, @ViewBuilder body: () -> some View) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            Text(title).font(.caption.weight(.semibold)).foregroundStyle(Color.app.foreground.opacity(0.6))
            body()
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
                .accessibilityHidden(true)
        )
    }

    private func reload() {
        do {
            records = try holder.core.audit().recent(limit: 500)
            wallet = try holder.core.wallet().load()
            loadError = nil
        } catch {
            loadError = "Load failed: \(error.localizedDescription)"
        }
    }

    private func refreshData() async {
        isRefreshing = true
        reload()
        isRefreshing = false
    }
}
#endif
