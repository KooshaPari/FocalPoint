import Foundation

#if canImport(Darwin)
import Darwin

/// In-process MCP server for the iOS FocalPoint app.
///
/// When enabled (via @AppStorage), spawns a Unix-domain-socket server
/// on ~/Library/Containers/com.koosha.focalpoint/Data/Library/mcp.sock
/// allowing desktop agents (Claude Desktop, Cursor) to drive the live iOS app.
///
/// - Important: Socket mode is experimental and requires explicit user opt-in.
/// - The socket is created with 0600 permissions (user-only read/write).
/// - All tool calls are routed through the core storage layer and logged to audit.
actor MCPBridgeServer {
    typealias InputOutput = (Data) -> Void

    private let socketPath: String
    private var serverSocket: Int32 = -1
    private var isRunning = false
    private var dispatcher: MCPToolDispatcher?

    /// Initialize the MCP bridge server.
    /// - Parameter coreHolder: Reference to the app's CoreHolder for state access
    nonisolated init(socketPath: String, coreHolder: CoreHolder) {
        self.socketPath = socketPath

        // Dispatcher will route tool calls to the core storage layer
        let dispatcher = MCPToolDispatcher(coreHolder: coreHolder)
        Task {
            await self.setDispatcher(dispatcher)
        }
    }

    nonisolated static let containerSocketPath: String = {
        let containerPath = NSSearchPathForDirectoriesInDomains(
            .libraryDirectory,
            .userDomainMask,
            true
        ).first ?? NSHomeDirectory()
        return "\(containerPath)/Containers/com.koosha.focalpoint/Data/Library/mcp.sock"
    }()

    /// Start the MCP server.
    /// The server listens for JSON-RPC 2.0 messages in MCP protocol format.
    func start() async throws {
        guard !isRunning else { return }

        // Ensure directory exists
        let dir = URL(fileURLWithPath: socketPath).deletingLastPathComponent()
        try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)

        // Remove stale socket file if it exists
        try? FileManager.default.removeItem(atPath: socketPath)

        // Create Unix domain socket
        serverSocket = socket(AF_UNIX, SOCK_STREAM, 0)
        guard serverSocket >= 0 else {
            throw MCPBridgeError.socketCreationFailed(errno)
        }

        // Bind socket to path
        var addr = sockaddr_un()
        addr.sun_family = sa_family_t(AF_UNIX)

        // Safely copy path to sun_path (max 104 chars on most Unix systems)
        let pathBytes = socketPath.utf8
        guard pathBytes.count < MemoryLayout<sockaddr_un>.stride else {
            close(serverSocket)
            throw MCPBridgeError.socketPathTooLong
        }

        _ = withUnsafeMutableBytes(of: &addr.sun_path) { buffer in
            memcpy(buffer.baseAddress!, Array(pathBytes), pathBytes.count)
        }

        let addrSize = MemoryLayout<sockaddr_un>.size
        let bindResult = withUnsafePointer(to: &addr) { addrPtr in
            bind(
                serverSocket,
                UnsafeRawPointer(addrPtr).assumingMemoryBound(to: sockaddr.self),
                socklen_t(addrSize)
            )
        }

        guard bindResult == 0 else {
            close(serverSocket)
            throw MCPBridgeError.bindFailed(errno)
        }

        // Set socket permissions to 0600 (user-only)
        try FileManager.default.setAttributes(
            [.protectionKey: FileProtectionType.complete],
            ofItemAtPath: socketPath
        )

        // Listen for incoming connections
        guard listen(serverSocket, 5) == 0 else {
            close(serverSocket)
            throw MCPBridgeError.listenFailed(errno)
        }

        isRunning = true

        // Accept connections in background
        Task.detached(priority: .background) { [weak self] in
            await self?.acceptConnections()
        }
    }

    /// Stop the MCP server and clean up.
    func stop() async {
        guard isRunning else { return }

        if serverSocket >= 0 {
            close(serverSocket)
            serverSocket = -1
        }

        try? FileManager.default.removeItem(atPath: socketPath)
        isRunning = false
    }

    /// Accept incoming connections and handle them.
    private func acceptConnections() async {
        while isRunning {
            var clientAddr = sockaddr_un()
            var clientAddrLen = socklen_t(MemoryLayout<sockaddr_un>.size)

            let clientSocket = withUnsafeMutablePointer(to: &clientAddr) { addrPtr in
                accept(
                    serverSocket,
                    UnsafeRawPointer(addrPtr).assumingMemoryBound(to: sockaddr.self),
                    &clientAddrLen
                )
            }

            guard clientSocket >= 0 else {
                // EINTR (interrupted) is expected during shutdown
                if errno != EINTR {
                    print("MCPBridge: accept failed: \(errno)")
                }
                continue
            }

            // Handle client in background
            Task.detached(priority: .default) { [weak self] in
                await self?.handleClient(clientSocket)
            }
        }
    }

    /// Handle a single client connection.
    private func handleClient(_ clientSocket: Int32) async {
        defer { close(clientSocket) }

        var buffer = [UInt8](repeating: 0, count: 4096)

        while isRunning {
            let bytesRead = read(clientSocket, &buffer, buffer.count)

            guard bytesRead > 0 else { return }

            let requestData = Data(buffer[0..<bytesRead])

            // Dispatch tool call through the MCP protocol
            if let dispatcher = dispatcher {
                let responseData = await dispatcher.dispatch(requestData)
                _ = write(clientSocket, (responseData as NSData).bytes, responseData.count)
            }
        }
    }

    private func setDispatcher(_ dispatcher: MCPToolDispatcher) {
        self.dispatcher = dispatcher
    }
}

enum MCPBridgeError: Error {
    case socketCreationFailed(Int32)
    case bindFailed(Int32)
    case listenFailed(Int32)
    case socketPathTooLong
}

#endif // canImport(Darwin)
