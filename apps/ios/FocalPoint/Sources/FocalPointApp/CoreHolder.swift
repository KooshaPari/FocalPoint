#if canImport(SwiftUI)
import Foundation
import SwiftUI
import FocalPointCore

/// Process-wide singleton wrapping the Rust-backed `FocalPointCore`.
///
/// UniFFI's generated constructor takes a `storagePath`; we store the SQLite
/// DB under `<AppSupport>/focalpoint/core.db`. Exposed as an `ObservableObject`
/// so views can `.environmentObject(...)` it and trigger manual refreshes.
@MainActor
public final class CoreHolder: ObservableObject {
    public static let shared = CoreHolder()

    public let core: FocalPointCore

    /// Bumped when any mutation completes. Views can observe to re-query.
    @Published public private(set) var revision: Int = 0

    private init() {
        let fm = FileManager.default
        let base = (try? fm.url(
            for: .applicationSupportDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: true
        )) ?? fm.temporaryDirectory
        let dir = base.appendingPathComponent("focalpoint", isDirectory: true)
        try? fm.createDirectory(at: dir, withIntermediateDirectories: true)
        let db = dir.appendingPathComponent("core.db")
        do {
            self.core = try FocalPointCore(storagePath: db.path)
        } catch {
            // Fatal: we must have a core. Bubble up via preconditionFailure so
            // testers see a loud failure, not silent degradation.
            preconditionFailure("FocalPointCore init failed: \(error)")
        }
    }

    public func bump() { revision &+= 1 }

    /// Attach an EventKit-backed `CalendarHost` so the Rust Rituals engine
    /// reads real device calendar events for Morning Brief schedule previews
    /// and conflict detection. No-op if the user denies calendar access.
    ///
    /// Traces to: FR-CAL-001.
    public func attachEventKitCalendar() async {
        let host = EventKitCalendarHost()
        let granted = (try? await host.requestAccess()) ?? false
        guard granted else { return }
        core.setCalendarHost(host: host)
    }
}

/// Swift-only Canvas connector bridge. Real OAuth token exchange is stubbed
/// into `UserDefaults` until the Rust FFI gains a `connect_canvas` method
/// (tracked as a follow-up; adding it would require regenerating bindings +
/// XCFramework). The Swift layer is still the only caller, so the API shape
/// here matches what the future FFI will expose.
public struct CanvasConnectionRecord: Codable, Equatable {
    public var instanceUrl: String
    public var connectedAtIso: String
    public var tokenFingerprint: String  // sha-like tail, never the real token

    public init(instanceUrl: String, connectedAtIso: String, tokenFingerprint: String) {
        self.instanceUrl = instanceUrl
        self.connectedAtIso = connectedAtIso
        self.tokenFingerprint = tokenFingerprint
    }
}

public enum CanvasBridge {
    private static let udKey = "focalpoint.canvas.connection"

    public static func load() -> CanvasConnectionRecord? {
        guard let data = UserDefaults.standard.data(forKey: udKey) else { return nil }
        return try? JSONDecoder().decode(CanvasConnectionRecord.self, from: data)
    }

    public static func save(_ record: CanvasConnectionRecord) {
        if let data = try? JSONEncoder().encode(record) {
            UserDefaults.standard.set(data, forKey: udKey)
        }
    }

    public static func clear() {
        UserDefaults.standard.removeObject(forKey: udKey)
    }

    /// Simulate the real `FocalPointCore.connectCanvas(instanceUrl:code:)` FFI
    /// call. In production this would exchange `code` for an access token and
    /// persist via the secret store; here we stash a fingerprint of `code` so
    /// the UI has something to show after the OAuth round-trip.
    public static func connect(instanceUrl: String, code: String) async throws {
        // Light artificial delay so the UI spinner is visible.
        try? await Task.sleep(nanoseconds: 200_000_000)
        let fingerprint = String(code.suffix(6))
        let iso = ISO8601DateFormatter().string(from: Date())
        save(.init(instanceUrl: instanceUrl, connectedAtIso: iso, tokenFingerprint: fingerprint))
    }
}
#endif
