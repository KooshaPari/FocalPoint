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
                        Text(verified == true ? "Chain verified" : verified == false ? "Chain tampered" : "Not yet verified")
                            .font(.caption)
                        Spacer()
                        Button("Verify") { verifyChain() }
                            .font(.caption)
                    }
                }
                if records.isEmpty {
                    Section {
                        HStack(spacing: 12) {
                            CoachyView(state: CoachyState(pose: .curious, emotion: .neutral, bubbleText: "Nothing's happened yet."), size: 80)
                            Text("Add a task, connect a tool, or hit Sync now — activity will show up here.")
                                .font(.caption)
                                .foregroundStyle(Color.app.foreground.opacity(0.7))
                        }
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
                    Button { reload() } label: { Image(systemName: "arrow.clockwise") }
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
