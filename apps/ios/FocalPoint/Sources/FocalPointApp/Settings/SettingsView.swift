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
    @State private var versionTapCount: Int = 0

    public init() {}

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
                    ForEach(connectors, id: \.connectorId) { c in
                        connectorRow(c)
                    }
                    if connectors.isEmpty {
                        Text("No connectors registered yet.")
                            .font(.caption)
                            .foregroundStyle(Color.app.foreground.opacity(0.6))
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
        }
        .task(id: holder.revision) {
            loadConnectors()
            canvas = CanvasBridge.load()
        }
    }

    @ViewBuilder
    private func connectorRow(_ c: ConnectorHandleSummary) -> some View {
        let isCanvas = c.connectorId.lowercased().contains("canvas")
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text(c.connectorId).font(.body.weight(.semibold))
                Spacer()
                Text(c.health)
                    .font(.caption2.weight(.bold))
                    .foregroundStyle(Color.app.accent)
            }
            if isCanvas, let canvas {
                Text("Connected: \(canvas.instanceUrl)")
                    .font(.caption).foregroundStyle(Color.app.foreground.opacity(0.7))
                Text("Token …\(canvas.tokenFingerprint)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
                Button("Disconnect") {
                    CanvasBridge.clear()
                    self.canvas = nil
                }.font(.caption)
            } else if isCanvas {
                Button("Connect") { showCanvasAuth = true }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                    .controlSize(.small)
            } else {
                Text("Next sync: \(c.nextSyncAtIso)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
            }
        }
        .padding(.vertical, 4)
    }

    private func loadConnectors() {
        let list = holder.core.sync().connectors()
        if list.isEmpty {
            // Seed a virtual Canvas entry so the UI always exposes the
            // connection action, even before the core registers it.
            connectors = [
                ConnectorHandleSummary(
                    connectorId: "canvas",
                    health: "unregistered",
                    nextSyncAtIso: "-",
                    lastCursor: nil
                ),
            ]
        } else {
            connectors = list
        }
    }

    private func bumpVersion() {
        versionTapCount += 1
        if versionTapCount >= 5 { devModeUnlocked = true }
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
