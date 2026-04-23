#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

/// Real settings, backed by the core where applicable.
public struct SettingsView: View {
    @EnvironmentObject private var holder: CoreHolder
    @AppStorage("app.darkMode") private var darkMode: Bool = true
    @AppStorage("app.hasOnboarded") private var hasOnboarded: Bool = false
    @AppStorage("app.notifications") private var notificationsEnabled: Bool = true
    @AppStorage("app.devModeUnlocked") private var devModeUnlocked: Bool = false

    @State private var connectors: [ConnectorHandleSummary] = []
    @State private var canvas: CanvasConnectionRecord?
    @State private var showCanvasAuth: Bool = false
    @State private var showGCalAuth: Bool = false
    @State private var showGitHubAuth: Bool = false
    @State private var versionTapCount: Int = 0
    @State private var lastSyncSummary: String?
    @State private var auditExportUrl: URL?
    @State private var exportError: String?

    public init() {}

    /// Fixed display order for connectors in the Settings list.
    private let orderedConnectorIds: [String] = ["canvas", "gcal", "github"]

    private let connectorDisplayNames: [String: String] = [
        "canvas": "Canvas",
        "gcal": "Google Calendar",
        "github": "GitHub",
    ]

    public var body: some View {
        NavigationStack {
            Form {
                Section("Account") {
                    LabeledContent("User", value: "local-user")
                    LabeledContent("Version") {
                        Text(holder.core.appVersion())
                            .onTapGesture { bumpVersion() }
                    }
                }

                Section("Theme") {
                    Toggle("Dark mode", isOn: $darkMode)
                        .tint(Color.app.accent)
                }

                Section("Notifications") {
                    Toggle("Enabled", isOn: $notificationsEnabled)
                        .tint(Color.app.accent)
                }

                Section("Connectors") {
                    ForEach(orderedConnectorIds, id: \.self) { id in
                        connectorRow(id: id, summary: summary(for: id))
                    }
                    Button {
                        let report = holder.syncTick()
                        lastSyncSummary = "Synced \(report.connectorsSynced) connector(s), pulled \(report.eventsPulled) event(s)\(report.errors.isEmpty ? "" : ", errors: \(report.errors.count)")"
                    } label: {
                        HStack {
                            Image(systemName: "arrow.triangle.2.circlepath")
                            Text("Sync now")
                        }
                    }
                    Button {
                        let sync = holder.syncTick()
                        if let eval = holder.evalTick() {
                            lastSyncSummary = "Synced \(sync.connectorsSynced) conn / \(sync.eventsPulled) ev; evaluated \(eval.eventsEvaluated), fired \(eval.decisionsFired)"
                        } else {
                            lastSyncSummary = "Synced \(sync.connectorsSynced) conn / \(sync.eventsPulled) ev; eval failed"
                        }
                    } label: {
                        HStack {
                            Image(systemName: "bolt.badge.checkmark")
                            Text("Run rules now")
                        }
                    }
                    if let lastSyncSummary {
                        Text(lastSyncSummary)
                            .font(.caption2)
                            .foregroundStyle(Color.app.foreground.opacity(0.6))
                    }
                }

                Section("Data") {
                    if let url = auditExportUrl {
                        ShareLink(item: url) {
                            Label("Share audit chain export", systemImage: "square.and.arrow.up")
                        }
                    }
                    Button {
                        exportAuditChain()
                    } label: {
                        Label(auditExportUrl == nil ? "Export audit chain" : "Regenerate export", systemImage: "doc.badge.arrow.up")
                    }
                    if let err = exportError {
                        Text(err).font(.caption2).foregroundStyle(.red)
                    }
                }

                if devModeUnlocked {
                    Section("Developer") {
                        Button("Re-run onboarding") {
                            hasOnboarded = false
                        }
                        Button("Reset Canvas connection") {
                            CanvasBridge.clear()
                            canvas = nil
                        }
                        NavigationLink("Coachy character sheet") {
                            CoachyDebugView()
                        }
                    }
                }
            }
            .navigationTitle("Settings")
            .background(Color.app.background.ignoresSafeArea())
            .sheet(isPresented: $showCanvasAuth) {
                CanvasAuthView(
                    onConnected: { record in
                        canvas = record
                        holder.bump()
                    }
                )
            }
            .sheet(isPresented: $showGCalAuth) {
                GCalAuthView(
                    onConnected: {
                        holder.bump()
                    }
                )
            }
            .sheet(isPresented: $showGitHubAuth) {
                GitHubAuthView(
                    onConnected: {
                        holder.bump()
                    }
                )
            }
        }
        .task(id: holder.revision) {
            loadConnectors()
            canvas = CanvasBridge.load()
        }
    }

    @ViewBuilder
    private func connectorRow(id: String, summary: ConnectorHandleSummary?) -> some View {
        let displayName = connectorDisplayNames[id] ?? id
        let status = summary?.health ?? "Not connected"
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text(displayName).font(.body.weight(.semibold))
                Spacer()
                Text(status)
                    .font(.caption2.weight(.bold))
                    .foregroundStyle(Color.app.accent)
            }
            connectorActions(id: id, summary: summary)
        }
        .padding(.vertical, 4)
    }

    @ViewBuilder
    private func connectorActions(id: String, summary: ConnectorHandleSummary?) -> some View {
        switch id {
        case "canvas":
            if let canvas {
                Text("Connected: \(canvas.instanceUrl)")
                    .font(.caption).foregroundStyle(Color.app.foreground.opacity(0.7))
                Text("Token …\(canvas.tokenFingerprint)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
                Button("Disconnect") {
                    CanvasBridge.clear()
                    self.canvas = nil
                }.font(.caption)
            } else {
                Button("Connect") { showCanvasAuth = true }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                    .controlSize(.small)
            }
        case "gcal":
            if let summary {
                Text("Next sync: \(summary.nextSyncAtIso)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
            }
            Button("Connect") { showGCalAuth = true }
                .buttonStyle(.borderedProminent)
                .tint(Color.app.accent)
                .controlSize(.small)
        case "github":
            if let summary {
                Text("Next sync: \(summary.nextSyncAtIso)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
            }
            Button("Connect") { showGitHubAuth = true }
                .buttonStyle(.borderedProminent)
                .tint(Color.app.accent)
                .controlSize(.small)
        default:
            if let summary {
                Text("Next sync: \(summary.nextSyncAtIso)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
            }
        }
    }

    private func summary(for id: String) -> ConnectorHandleSummary? {
        connectors.first { $0.connectorId.lowercased().contains(id) }
    }

    private func loadConnectors() {
        connectors = holder.core.sync().connectors()
    }

    private func bumpVersion() {
        versionTapCount += 1
        if versionTapCount >= 5 { devModeUnlocked = true }
    }

    /// Write `audit.recent(limit: 5000)` as JSONL to a tempfile and expose
    /// it via `ShareLink` so the user can hand it off through Files /
    /// AirDrop / Mail. The export is the source-of-truth provenance
    /// artifact — every wallet / penalty / policy / connector / task /
    /// ritual / notify / host-event mutation + its hash-chain position.
    private func exportAuditChain() {
        exportError = nil
        do {
            let records = try holder.core.audit().recent(limit: 5000)
            let lines = records.map { rec -> String in
                let obj: [String: Any] = [
                    "id": rec.id,
                    "record_type": rec.recordType,
                    "subject_ref": rec.subjectRef,
                    "occurred_at": rec.occurredAtIso,
                    "payload_json": rec.payloadJson,
                    "hash": rec.hash,
                ]
                if let data = try? JSONSerialization.data(withJSONObject: obj),
                   let s = String(data: data, encoding: .utf8) {
                    return s
                }
                return "{}"
            }
            let body = lines.joined(separator: "\n") + "\n"
            let ts = ISO8601DateFormatter().string(from: Date())
                .replacingOccurrences(of: ":", with: "-")
            let filename = "focalpoint-audit-\(ts).jsonl"
            let url = FileManager.default.temporaryDirectory.appendingPathComponent(filename)
            try body.write(to: url, atomically: true, encoding: .utf8)
            auditExportUrl = url
        } catch {
            exportError = "Export failed: \(error.localizedDescription)"
        }
    }
}

// MARK: - Debug view (relocates the original pose-cycling demo)

struct CoachyDebugView: View {
    var body: some View {
        // Re-use the original cycling showcase so the demo isn't lost.
        DebugCoachyShowcase()
            .navigationTitle("Coachy debug")
            .background(Color.app.background.ignoresSafeArea())
    }
}
#endif
