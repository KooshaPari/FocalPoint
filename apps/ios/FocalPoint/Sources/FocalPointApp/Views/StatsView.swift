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
                    Button { reload() } label: { Image(systemName: "arrow.clockwise") }
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
            Text("No activity yet this week.")
                .font(.body.weight(.semibold))
            Text("Add a task, start a focus session, or connect a tool.")
                .font(.caption)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
        }
        .padding()
    }

    // MARK: - Cards

    private var focusCard: some View {
        let minutes = focusMinutesThisWeek()
        return card(title: "Focus time — last 7 days") {
            HStack(alignment: .firstTextBaseline) {
                Text(formatMinutes(minutes))
                    .font(.system(size: 40, weight: .bold, design: .rounded))
                    .foregroundStyle(Color.app.accent)
                Spacer()
                Image(systemName: "timer").font(.system(size: 32))
                    .foregroundStyle(Color.app.accent.opacity(0.5))
            }
            Text("Across \(focusSessionCount()) sessions.")
                .font(.caption).foregroundStyle(.secondary)
        }
    }

    private var creditCard: some View {
        let earned = creditDelta(kind: "wallet.grant_credit")
        let spent = creditDelta(kind: "wallet.spend_credit")
        return card(title: "Credits this week") {
            HStack(spacing: 24) {
                stat(label: "Earned", value: "+\(earned)", tint: .green)
                stat(label: "Spent", value: "-\(spent)", tint: .orange)
                Spacer()
                if let w = wallet {
                    stat(label: "Balance", value: "\(w.balance)", tint: Color.app.accent)
                }
            }
        }
    }

    private var rulesCard: some View {
        let top = topRules(limit: 5)
        return card(title: "Top firing rules — last 7 days") {
            if top.isEmpty {
                Text("No rules fired yet.").font(.caption).foregroundStyle(.secondary)
            } else {
                ForEach(Array(top.enumerated()), id: \.offset) { _, row in
                    HStack {
                        Text(row.ruleId.prefix(8) + "…")
                            .font(.caption.monospaced())
                            .foregroundStyle(.secondary)
                        Spacer()
                        Text("\(row.count)×").font(.body.weight(.semibold))
                    }
                }
            }
        }
    }

    @ViewBuilder
    private var streaksCard: some View {
        if let streaks = wallet?.streaks, !streaks.isEmpty {
            card(title: "Active streaks") {
                ForEach(streaks, id: \.name) { s in
                    HStack {
                        Image(systemName: "flame.fill").foregroundStyle(.orange)
                        Text(s.name).font(.body)
                        Spacer()
                        Text("\(s.count) days").font(.body.weight(.semibold))
                    }
                }
            }
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
            Text(title).font(.caption.weight(.semibold)).foregroundStyle(.secondary)
            body()
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func stat(label: String, value: String, tint: Color) -> some View {
        VStack(alignment: .leading, spacing: 2) {
            Text(label).font(.caption2).foregroundStyle(.secondary)
            Text(value).font(.title3.weight(.bold)).foregroundStyle(tint)
        }
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
