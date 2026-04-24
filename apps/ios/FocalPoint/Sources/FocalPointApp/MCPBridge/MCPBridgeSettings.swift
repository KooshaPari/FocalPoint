import SwiftUI

#if canImport(Darwin)

/// Settings UI for the MCP bridge.
/// Allows users to enable/disable the in-process MCP server and view the socket path.
struct MCPBridgeSettings: View {
    @AppStorage("app.mcpBridgeEnabled") private var mcpBridgeEnabled = false
    @Environment(\.scenePhase) private var scenePhase
    @State private var isStarting = false
    @State private var error: String?

    private let socketPath = MCPBridgeServer.containerSocketPath

    var body: some View {
        Section("MCP Bridge (Developer)") {
            Toggle("Enable MCP Bridge", isOn: $mcpBridgeEnabled)
                .onChange(of: mcpBridgeEnabled) { oldValue, newValue in
                    if newValue {
                        startBridge()
                    } else {
                        stopBridge()
                    }
                }

            if mcpBridgeEnabled {
                VStack(alignment: .leading, spacing: 8) {
                    Text("Socket Path")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    HStack {
                        Text(socketPath)
                            .font(.caption2)
                            .monospaced()
                            .lineLimit(2)
                            .truncationMode(.middle)
                            .selectableText()

                        Button(action: copySocketPath) {
                            Image(systemName: "doc.on.doc")
                                .font(.caption)
                        }
                        .buttonStyle(.bordered)
                    }

                    Text("Desktop agents (Claude, Cursor) can connect via socat or ncat.")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .padding(.vertical, 8)

                if let error = error {
                    Label(error, systemImage: "exclamationmark.circle")
                        .font(.caption)
                        .foregroundColor(.red)
                }
            }
        }
    }

    private func startBridge() {
        // Bridge startup is handled by the app's CoreHolder
        // Just set the flag; the app will start the server on foreground
        print("MCP Bridge enabled")
    }

    private func stopBridge() {
        // Bridge shutdown is handled by the app's CoreHolder
        print("MCP Bridge disabled")
    }

    private func copySocketPath() {
        UIPasteboard.general.string = socketPath
    }
}

// MARK: - Selectable Text (iOS 15+)

extension Text {
    func selectableText() -> some View {
        #if os(iOS)
        if #available(iOS 15.0, *) {
            return AnyView(self.textSelection(.enabled))
        }
        #endif
        return AnyView(self)
    }
}

#endif // canImport(Darwin)
