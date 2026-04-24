#if canImport(Darwin)

/// FocalPointMCPBridge — In-process MCP server for iOS app.
///
/// When enabled via @AppStorage("app.mcpBridgeEnabled"), the FocalPoint iOS app
/// spawns a Unix-domain-socket MCP server at:
///   ~/Library/Containers/com.koosha.focalpoint/Data/Library/mcp.sock
///
/// Desktop agents (Claude Desktop, Cursor, custom clients) can connect and call
/// all 27 MCP tools directly against the live iOS app session.
///
/// - Note: Socket is created with 0600 permissions (user-only read/write).
/// - Note: All tool calls are logged to the audit trail.
/// - Note: Bridge is disabled by default for security.
enum MCPBridge {
    /// The shared MCP server instance.
    /// Owned by CoreHolder; lifecycle tied to app foreground/background state.
    nonisolated(unsafe) static var shared: MCPBridgeServer?

    /// Start the MCP bridge server.
    /// Called from CoreHolder.startMCPBridgeIfEnabled() when appropriate.
    static func start(socketPath: String, coreHolder: CoreHolder) async throws {
        let server = MCPBridgeServer(socketPath: socketPath, coreHolder: coreHolder)
        try await server.start()
        shared = server
    }

    /// Stop the MCP bridge server.
    /// Called when the app moves to background or bridge is disabled.
    static func stop() async {
        if let server = shared {
            await server.stop()
            shared = nil
        }
    }
}

#endif // canImport(Darwin)
