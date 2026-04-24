#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Week-at-a-glance stats derived from the audit chain. Aggregates
/// audit.recent(limit: 500) in-process — no new FFI surface, no new
/// storage. Shows focus hours, credit flow, top-firing rules, and
/// streak counts.
struct StatsView: View {
    @EnvironmentObject private var holder: CoreHolder
    @State private var records: [AuditRecordDto] = []
    @State private var wallet: WalletSummary?
    @State private var loadError: String?

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    header
                    if records.isEmpty {
                        empty
                    } else {
                        focusCard
                        creditCard
                        rulesCard
                        streaksCard
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
        }
    }

    private var header: some View {
        HStack {
            CoachyView(
                state: CoachyState(pose: .confident, emotion: .proud, bubbleText: "Here's your week."),
                size: 100
            )
            Spacer()
        }
    }

    private var empty: some View {
        VStack(spacing: 8) {
            Text(String(localized: "No activity yet this week.", defaultValue: "No activity yet this week."))
                .font(.body.weight(.semibold))
            Text(String(localized: "Add a task, start a focus session, or connect a tool.", defaultValue: "Add a task, start a focus session, or connect a tool."))
                .font(.caption)
                .foregroundStyle(Color.app.foreground.opacity(0.6))
                .multilineTextAlignment(.center)
        }
        .padding()
    }

    // MARK: - Cards

    private var focusCard: some View {
        let minutes = focusMinutesThisWeek()
        return card(title: String(localized: "Focus time — last 7 days", defaultValue: "Focus time — last 7 days")) {
            HStack(alignment: .firstTextBaseline) {
                Text(formatMinutes(minutes))
                    .font(.title.weight(.semibold))
                    .foregroundStyle(Color.app.accent)
                Spacer()
                Image(systemName: "timer")
                    .foregroundStyle(Color.app.accent.opacity(0.5))
                    .accessibilityLabel(String(localized: "Focus time indicator", defaultValue: "Focus time indicator"))
            }
            Text(String(localized: "Across \(focusSessionCount()) sessions.", defaultValue: "Across \(focusSessionCount()) sessions."))
                .font(.caption).foregroundStyle(Color.app.foreground.opacity(0.6))
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Focus time this week", defaultValue: "Focus time this week"))
        .accessibilityValue(String(localized: formatMinutes(minutes), defaultValue: formatMinutes(minutes)))
    }

    private var creditCard: some View {
        let earned = creditDelta(kind: "wallet.grant_credit")
        let spent = creditDelta(kind: "wallet.spend_credit")
        return card(title: String(localized: "Credits this week", defaultValue: "Credits this week")) {
            HStack(spacing: 24) {
                stat(label: String(localized: "Earned", defaultValue: "Earned"), value: "+\(earned)", tint: Color.app.accent)
                stat(label: String(localized: "Spent", defaultValue: "Spent"), value: "-\(spent)", tint: Color.app.accent.opacity(0.7))
                Spacer()
                if let w = wallet {
                    stat(label: String(localized: "Balance", defaultValue: "Balance"), value: "\(w.balance)", tint: Color.app.accent)
                }
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Credits this week", defaultValue: "Credits this week"))
    }

    private var rulesCard: some View {
        let top = topRules(limit: 5)
        return card(title: String(localized: "Top firing rules — last 7 days", defaultValue: "Top firing rules — last 7 days")) {
            if top.isEmpty {
                Text(String(localized: "No rules fired yet.", defaultValue: "No rules fired yet."))
                    .font(.caption).foregroundStyle(Color.app.foreground.opacity(0.6))
            } else {
                ForEach(Array(top.enumerated()), id: \.offset) { _, row in
                    HStack {
                        Text(row.ruleId.prefix(8) + "…")
                            .font(.caption.monospaced())
                            .foregroundStyle(Color.app.foreground.opacity(0.6))
                        Spacer()
                        Text("\(row.count)×").font(.body.weight(.semibold))
                    }
                    .accessibilityLabel(String(localized: "Rule \(row.ruleId.prefix(8)): \(row.count) fires", defaultValue: "Rule \(row.ruleId.prefix(8)): \(row.count) fires"))
                }
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Top firing rules", defaultValue: "Top firing rules"))
    }

    @ViewBuilder
    private var streaksCard: some View {
        if let streaks = wallet?.streaks, !streaks.isEmpty {
            card(title: String(localized: "Active streaks", defaultValue: "Active streaks")) {
                ForEach(streaks, id: \.name) { s in
                    HStack {
                        Image(systemName: "flame.fill")
                            .foregroundStyle(Color.app.accent)
                            .accessibilityHidden(true)
                        Text(s.name).font(.body)
                        Spacer()
                        Text(String(localized: "\(s.count) days", defaultValue: "\(s.count) days"))
                            .font(.body.weight(.semibold))
                    }
                    .accessibilityElement(children: .combine)
                    .accessibilityLabel(String(localized: "\(s.name): \(s.count) days", defaultValue: "\(s.name): \(s.count) days"))
                }
            }
            .accessibilityElement(children: .combine)
            .accessibilityLabel(String(localized: "Active streaks", defaultValue: "Active streaks"))
        }
    }

    // MARK: - Aggregation (audit-driven)

    private func focusMinutesThisWeek() -> Int {
        records
            .filter { $0.recordType == "host.event.emitted" }
            .compactMap { rec -> Int? in
                guard let obj = parse(rec.payloadJson),
                      obj["event_type"] as? String == "focus:session_completed",
                      let min = (obj["payload"] as? [String: Any])?["minutes"] as? Int else {
                    return nil
                }
                return min
            }
            .reduce(0, +)
    }

    private func focusSessionCount() -> Int {
        records
            .filter { $0.recordType == "host.event.emitted" }
            .filter {
                guard let obj = parse($0.payloadJson) else { return false }
                return obj["event_type"] as? String == "focus:session_completed"
            }
            .count
    }

    private func creditDelta(kind: String) -> Int64 {
        records
            .filter { $0.recordType == kind }
            .compactMap { rec -> Int64? in
                guard let obj = parse(rec.payloadJson) else { return nil }
                return (obj["amount"] as? Int).map(Int64.init) ?? (obj["amount"] as? Int64)
            }
            .reduce(Int64(0), +)
    }

    private struct RuleFireRow {
        let ruleId: String
        let count: Int
    }

    private func topRules(limit: Int) -> [RuleFireRow] {
        var tally: [String: Int] = [:]
        for rec in records where rec.recordType == "rule.fired" {
            guard let obj = parse(rec.payloadJson),
                  let id = obj["rule_id"] as? String else { continue }
            tally[id, default: 0] += 1
        }
        return tally
            .map { RuleFireRow(ruleId: $0.key, count: $0.value) }
            .sorted { $0.count > $1.count }
            .prefix(limit)
            .map { $0 }
    }

    // MARK: - Helpers

    private func parse(_ json: String) -> [String: Any]? {
        guard let data = json.data(using: .utf8) else { return nil }
        return (try? JSONSerialization.jsonObject(with: data)) as? [String: Any]
    }

    private func formatMinutes(_ m: Int) -> String {
        if m < 60 { return "\(m)m" }
        let h = m / 60
        let r = m % 60
        return r == 0 ? "\(h)h" : "\(h)h \(r)m"
    }

    @ViewBuilder
    private func card(title: String, @ViewBuilder body: () -> some View) -> some View {
        VStack(alignment: .leading, spacing: 8) {
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

    private func stat(label: String, value: String, tint: Color) -> some View {
        VStack(alignment: .leading, spacing: 2) {
            Text(label).font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.6))
            Text(value).font(.title3.weight(.bold)).foregroundStyle(tint)
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "\(label): \(value)", defaultValue: "\(label): \(value)"))
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
}
#endif
