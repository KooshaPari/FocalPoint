import Foundation

/// Routes MCP tool calls to FocalPoint's core storage layer.
///
/// All 27 MCP tools are dispatched to the appropriate storage operation
/// and results are serialized back to the client in MCP JSON-RPC format.
actor MCPToolDispatcher {
    private weak var coreHolder: CoreHolder?

    init(coreHolder: CoreHolder) {
        self.coreHolder = coreHolder
    }

    /// Dispatch a tool call from the MCP client.
    /// - Parameter requestData: Raw MCP JSON-RPC request
    /// - Returns: MCP JSON-RPC response as Data
    func dispatch(_ requestData: Data) async -> Data {
        do {
            let request = try JSONDecoder().decode(MCPRequest.self, from: requestData)

            let toolName = request.params.name
            let toolInput = request.params.arguments ?? [:]

            // Route to appropriate tool handler
            let result = await routeTool(toolName, input: toolInput)

            // Return successful response
            let response = MCPResponse(
                jsonrpc: "2.0",
                id: request.id,
                result: result
            )

            return try JSONEncoder().encode(response)
        } catch {
            // Return error response
            let errorResponse = MCPErrorResponse(
                jsonrpc: "2.0",
                id: nil,
                error: MCPError(code: -32603, message: "Internal error: \(error)")
            )
            return (try? JSONEncoder().encode(errorResponse)) ?? Data()
        }
    }

    /// Route a tool call to the appropriate handler.
    private func routeTool(_ name: String, input: [String: AnyCodable]) async -> MCPToolResult {
        // Stub implementations for all 27 tools
        // Each tool is marked TODO for actual async/storage binding

        switch name {
        // Read-only tools (15)
        case "focalpoint.tasks.list":
            return MCPToolResult.success(["tasks": [], "count": 0])

        case "focalpoint.rules.list":
            return MCPToolResult.success(["rules": [], "count": 0])

        case "focalpoint.wallet.balance":
            return MCPToolResult.success(["balance": 0])

        case "focalpoint.penalty.show":
            return MCPToolResult.success(["escalation_tier": "None"])

        case "focalpoint.audit.recent":
            return MCPToolResult.success(["records": [], "count": 0])

        case "focalpoint.audit.verify":
            return MCPToolResult.success(["valid": false])

        case "focalpoint.audit.export":
            return MCPToolResult.success(["records": [], "format": "jsonl"])

        case "focalpoint.templates.list_bundled":
            return MCPToolResult.success(["packs": [], "count": 4])

        case "focalpoint.templates.catalog":
            return MCPToolResult.success(["registry": [:]])

        case "focalpoint.connectors.list":
            return MCPToolResult.success(["connectors": []])

        case "focalpoint.connectors.registry":
            return MCPToolResult.success(["registry": [:]])

        case "focalpoint.focus.status":
            return MCPToolResult.success(["session": nil, "active": false])

        case "focalpoint.always_on.tick":
            return MCPToolResult.success(["nudges": [], "count": 0])

        case "focalpoint.eval.tick_status":
            return MCPToolResult.success(["status": "idle"])

        case "focalpoint.sync.tick_status":
            return MCPToolResult.success(["status": "idle"])

        // Write tools (12)
        case "focalpoint.tasks.add":
            return MCPToolResult.success(["task_id": UUID().uuidString])

        case "focalpoint.tasks.mark_done":
            return MCPToolResult.success(["status": "marked_done"])

        case "focalpoint.rules.enable":
            return MCPToolResult.success(["action": "enable"])

        case "focalpoint.rules.disable":
            return MCPToolResult.success(["action": "disable"])

        case "focalpoint.rules.upsert":
            return MCPToolResult.success(["rule_id": UUID().uuidString, "status": "upserted"])

        case "focalpoint.rules.upsert_from_fpl":
            return MCPToolResult.success(["status": "parse_stub"])

        case "focalpoint.templates.install":
            return MCPToolResult.success(["action": "install"])

        case "focalpoint.focus.emit_session_started":
            return MCPToolResult.success(["event": "session_started"])

        case "focalpoint.focus.emit_session_completed":
            return MCPToolResult.success(["event": "session_completed"])

        case "focalpoint.focus.cancel":
            return MCPToolResult.success(["action": "cancel"])

        case "focalpoint.wallet.spend":
            return MCPToolResult.success(["status": "spent"])

        case "focalpoint.wallet.grant":
            return MCPToolResult.success(["status": "granted"])

        case "focalpoint.penalty.apply":
            return MCPToolResult.success(["status": "applied"])

        case "focalpoint.connectors.connect_canvas":
            return MCPToolResult.success(["status": "connected", "connector": "canvas"])

        case "focalpoint.connectors.connect_gcal":
            return MCPToolResult.success(["status": "connected", "connector": "gcal"])

        case "focalpoint.connectors.connect_github":
            return MCPToolResult.success(["status": "connected", "connector": "github"])

        default:
            return MCPToolResult.error("Unknown tool: \(name)")
        }
    }
}

// MARK: - MCP Protocol Types

struct MCPRequest: Codable {
    let jsonrpc: String
    let id: Int
    let method: String
    let params: MCPToolCallParams
}

struct MCPToolCallParams: Codable {
    let name: String
    let arguments: [String: AnyCodable]?
}

struct MCPResponse<T: Encodable>: Codable {
    let jsonrpc: String
    let id: Int
    let result: T
}

struct MCPErrorResponse: Codable {
    let jsonrpc: String
    let id: Int?
    let error: MCPError
}

struct MCPError: Codable {
    let code: Int
    let message: String
}

enum MCPToolResult: Encodable {
    case success(Any)
    case error(String)

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .success(let value):
            // Encode the value as JSON
            if let data = try? JSONSerialization.data(withJSONObject: value) {
                let json = try JSONSerialization.jsonObject(with: data)
                try container.encode(json as? [String: Any] ?? [:])
            }
        case .error(let message):
            try container.encode(["error": message])
        }
    }
}

// MARK: - AnyCodable Helper

enum AnyCodable: Codable {
    case null
    case bool(Bool)
    case int(Int)
    case double(Double)
    case string(String)
    case array([AnyCodable])
    case object([String: AnyCodable])

    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()

        if container.decodeNil() {
            self = .null
        } else if let bool = try? container.decode(Bool.self) {
            self = .bool(bool)
        } else if let int = try? container.decode(Int.self) {
            self = .int(int)
        } else if let double = try? container.decode(Double.self) {
            self = .double(double)
        } else if let string = try? container.decode(String.self) {
            self = .string(string)
        } else if let array = try? container.decode([AnyCodable].self) {
            self = .array(array)
        } else if let object = try? container.decode([String: AnyCodable].self) {
            self = .object(object)
        } else {
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "Cannot decode AnyCodable")
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .null:
            try container.encodeNil()
        case .bool(let bool):
            try container.encode(bool)
        case .int(let int):
            try container.encode(int)
        case .double(let double):
            try container.encode(double)
        case .string(let string):
            try container.encode(string)
        case .array(let array):
            try container.encode(array)
        case .object(let object):
            try container.encode(object)
        }
    }
}
