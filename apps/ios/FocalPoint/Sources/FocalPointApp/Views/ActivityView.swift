#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Live tail of the audit chain — rewards, penalties, policy, rituals,
/// connector connects, task lifecycle. Newest first. Backs the "Activity"
/// tab so users can see what the core is actually doing.
struct ActivityView: View {
    @EnvironmentObject private var holder: CoreHolder
    @State private var records: [AuditRecordDto] = []
    @State private var verified: Bool?
    @State private var loadError: String?

    var body: some View {
        NavigationStack {
            List {
                Section {
                    HStack {
                        Image(systemName: verified == true ? "checkmark.seal.fill" : "questionmark.seal")
                            .foregroundStyle(verified == true ? Color.app.accent : Color.app.foreground.opacity(0.5))
                            .accessibilityLabel(verified == true ? String(localized: "Verified", defaultValue: "Verified") : String(localized: "Not verified", defaultValue: "Not verified"))
                        Text(verified == true ? String(localized: "Chain verified", defaultValue: "Chain verified") : verified == false ? String(localized: "Chain tampered", defaultValue: "Chain tampered") : String(localized: "Not yet verified", defaultValue: "Not yet verified"))
                            .font(.caption)
                        Spacer()
                        Button(String(localized: "Verify", defaultValue: "Verify")) { verifyChain() }
                            .font(.caption)
                            .accessibilityLabel(String(localized: "Verify chain", defaultValue: "Verify chain"))
                    }
                    .accessibilityElement(children: .combine)
                    .accessibilityLabel(String(localized: "Audit chain verification status", defaultValue: "Audit chain verification status"))
                }
                if records.isEmpty {
                    Section {
                        HStack(spacing: 12) {
                            CoachyView(state: CoachyState(pose: .curious, emotion: .neutral, bubbleText: String(localized: "Nothing's happened yet.", defaultValue: "Nothing's happened yet.")), size: 80)
                            Text(String(localized: "Add a task, connect a tool, or hit Sync now — activity will show up here.", defaultValue: "Add a task, connect a tool, or hit Sync now — activity will show up here."))
                                .font(.caption)
                                .foregroundStyle(Color.app.foreground.opacity(0.7))
                        }
                        .accessibilityElement(children: .combine)
                        .accessibilityLabel(String(localized: "No activity yet", defaultValue: "No activity yet"))
                    }
                } else {
                    ForEach(records, id: \.id) { rec in
                        row(rec)
                    }
                }
                if let e = loadError {
                    Text(e).foregroundStyle(.red).font(.caption2)
                }
            }
            .navigationTitle("Activity")
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button { reload() } label: {
                        Image(systemName: "arrow.clockwise")
                            .accessibilityLabel(String(localized: "Reload activity", defaultValue: "Reload activity"))
                    }
                }
            }
            .task(id: holder.revision) {
                reload()
            }
        }
    }

    @ViewBuilder
    private func row(_ r: AuditRecordDto) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            HStack {
                Text(label(for: r.recordType))
                    .font(.body.weight(.semibold))
                Spacer()
                Text(shortTime(r.occurredAtIso))
                    .font(.caption2)
                    .foregroundStyle(Color.app.foreground.opacity(0.5))
            }
            if !r.subjectRef.isEmpty {
                Text(r.subjectRef)
                    .font(.caption2)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
            }
            Text(r.payloadJson)
                .font(.caption2.monospaced())
                .foregroundStyle(Color.app.foreground.opacity(0.7))
                .lineLimit(2)
        }
        .padding(.vertical, 2)
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: label(for: r.recordType), defaultValue: label(for: r.recordType)))
        .accessibilityValue(String(localized: shortTime(r.occurredAtIso), defaultValue: shortTime(r.occurredAtIso)))
    }

    private func label(for kind: String) -> String {
        switch kind {
        case _ where kind.hasPrefix("wallet."): return "💎 " + kind
        case _ where kind.hasPrefix("penalty."): return "🚧 " + kind
        case _ where kind.hasPrefix("policy."): return "🛡️ " + kind
        case _ where kind.hasPrefix("connector."): return "🔌 " + kind
        case _ where kind.hasPrefix("task."): return "✅ " + kind
        case _ where kind.hasPrefix("ritual."): return "🔥 " + kind
        default: return "• " + kind
        }
    }

    private func shortTime(_ iso: String) -> String {
        let f = ISO8601DateFormatter()
        f.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        let d = f.date(from: iso) ?? ISO8601DateFormatter().date(from: iso) ?? Date()
        let rel = RelativeDateTimeFormatter()
        rel.unitsStyle = .abbreviated
        return rel.localizedString(for: d, relativeTo: Date())
    }

    private func reload() {
        do {
            records = try holder.core.audit().recent(limit: 100)
            loadError = nil
        } catch {
            loadError = "Load failed: \(error.localizedDescription)"
        }
    }

    private func verifyChain() {
        do {
            verified = try holder.core.audit().verifyChain()
        } catch {
            verified = nil
            loadError = "Verify failed: \(error.localizedDescription)"
        }
    }
}
#endif
