# iOS In-Process MCP Bridge

The FocalPoint iOS app can expose an MCP server over a Unix-domain socket, allowing desktop agents (Claude Desktop, Cursor, etc.) to drive the live iOS app session via MCP tools.

## Overview

When enabled, the FocalPoint iOS app spawns an MCP server that listens on:

```
~/Library/Containers/com.koosha.focalpoint/Data/Library/mcp.sock
```

Desktop agents can connect to this socket using tools like `socat` or `ncat` and call all 27 MCP tools directly against the iOS app's live state.

## Enabling the Bridge

### In the iOS App

1. Open **Settings** → **Developer**
2. Toggle **MCP Bridge** to ON
3. The socket path is displayed and can be copied to clipboard

**Default:** OFF (disabled for security)

## Using the Bridge from Desktop

### With Claude Desktop

Configure in `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "focalpoint-ios": {
      "command": "socat",
      "args": [
        "-",
        "unix-connect:~/Library/Containers/com.koosha.focalpoint/Data/Library/mcp.sock"
      ]
    }
  }
}
```

### With Manual socat

```bash
# On macOS: establish tunnel to iOS app
socat - unix-connect:~/Library/Containers/com.koosha.focalpoint/Data/Library/mcp.sock

# Or with netcat-style interface:
ncat -U ~/Library/Containers/com.koosha.focalpoint/Data/Library/mcp.sock
```

### From a Rust Agent

```rust
// Connect to iOS MCP bridge
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

let mut socket = UnixStream::connect(
    format!("{}/.../mcp.sock", 
        dirs::config_local_dir().unwrap().display())
)?;

// Send MCP STDIO protocol messages
socket.write_all(b"{...mcp request...}")?;
```

## Use Cases

### Power User Voice Commands

```
"Hey Claude, add a task for me"
→ Claude calls `focalpoint.tasks.add` over iOS bridge
→ Task appears instantly in iOS app
```

### Developer Testing

Test the iOS app's MCP surface without rebuilding or redeploying:

```bash
# Run the iOS app in simulator/device
# Enable MCP Bridge in Settings

# From another terminal, test tools:
cargo build -p focus-mcp-server
# Pipe tool invocations to the socket
```

### Cross-Platform Sync

A desktop agent can:
1. Read iOS app state via `focalpoint.audit.export` and `focalpoint.focus.status`
2. Write decisions back via `focalpoint.rules.upsert` and `focalpoint.wallet.spend`
3. Keep desktop and iOS app synchronized

## Architecture

### Socket Transport

The iOS app runs a separate MCP transport layer:

```
iOS FocalPoint App
  ├── Core State (SQLite, in-memory stores)
  ├── Domain Logic (rules, wallet, penalties)
  └── MCP Unix Socket Server (opt-in)
       └─ Listens on ~/Library/Containers/.../mcp.sock
          Speaks MCP STDIO protocol over the socket
          Serializes all tool calls through the storage layer
```

### Security

- **Socket file permissions:** `0600` (user-only read/write)
- **Disabled by default:** Users must explicitly enable in Settings
- **Audit trail:** All tool calls logged to audit store
- **No credentials over socket:** Connectors store credentials in Keychain

## Troubleshooting

### "Connection refused"

- Verify MCP Bridge is ON in iOS Settings → Developer
- Check socket path: `ls ~/Library/Containers/com.koosha.focalpoint/Data/Library/mcp.sock`
- Ensure app is running (bridge only active while app is foreground/backgrounded)

### "Permission denied"

- Verify socket file is readable: `ls -la ~/Library/Containers/.../mcp.sock`
- Ensure you're using the correct path (container changes per app ID)

### Slow Tool Calls

- iOS app may be suspended; bring to foreground
- Check device memory; bridge is in-process and uses app's memory
- Consider batching tool calls to reduce round-trips

## Implementation Status

The bridge is enabled via a Swift target `FocalPointMCPBridge` (Unix-domain-socket server only). Build requirements:

- Xcode 15+
- iOS 14+
- `#if canImport(Darwin)` guard (macOS/iOS only; not tvOS/watchOS)

See `/Apps/FocalPointApp/FocalPointMCPBridge/` for the implementation.

## See Also

- `docs/mcp/focalpoint_mcp_server.md` — Full MCP tool catalog
- `crates/focus-mcp-server/src/` — Rust MCP server implementation
- `apps/FocalPointApp/FocalPointMCPBridge/` — iOS socket server implementation
