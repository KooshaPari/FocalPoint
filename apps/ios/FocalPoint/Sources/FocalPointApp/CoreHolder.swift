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

        // Try App Group container first (shared with widget); fall back gracefully
        // if entitlement isn't granted (e.g., dev builds without App Group).
        let base: URL
        if let groupURL = fm.containerURL(forSecurityApplicationGroupIdentifier: "group.com.koosha.focalpoint") {
            base = groupURL
        } else {
            // Fallback to app-local Application Support (no widget access, but works in dev).
            base = (try? fm.url(
                for: .applicationSupportDirectory,
                in: .userDomainMask,
                appropriateFor: nil,
                create: true
            )) ?? fm.temporaryDirectory
        }

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

    /// Drive one `SyncOrchestrator::tick()` pass on the Rust core. Called
    /// from both the "Sync now" button in Settings and the foreground
    /// heartbeat timer below.
    @discardableResult
    public func syncTick() -> SyncReportDto {
        let report = core.sync().tick()
        // Bump so any view keyed on `revision` (Settings connector rows,
        // Today tab, Tasks tab) re-renders against fresh state.
        bump()
        return report
    }

    /// Drive one `RuleEvaluationPipeline::tick()` pass on the Rust core.
    /// Should be called right after `syncTick()` so newly-persisted events
    /// immediately flow through rule evaluation and into wallet / penalty /
    /// policy mutations.
    @discardableResult
    public func evalTick() -> EvaluationReportDto? {
        do {
            let report = try core.eval().tick()
            bump()
            return report
        } catch {
            return nil
        }
    }

    private var foregroundTimer: Timer?

    /// Start a foreground heartbeat that ticks the sync orchestrator every
    /// `interval` seconds while the app is active. Call on scene activation;
    /// `stopForegroundSync` on background/teardown. Idempotent — restarts
    /// the timer if already running.
    ///
    /// Each heartbeat runs `syncTick()` to pull new connector events, then
    /// `evalTick()` so those events flow through rule evaluation before the
    /// next beat.
    public func startForegroundSync(interval: TimeInterval = 60) {
        foregroundTimer?.invalidate()
        foregroundTimer = Timer.scheduledTimer(withTimeInterval: interval, repeats: true) { _ in
            Task { @MainActor in
                _ = CoreHolder.shared.syncTick()
                _ = CoreHolder.shared.evalTick()
                // Present any new Notify audit records as local
                // notifications. Deduped by AuditRecord.id.
                NotificationDispatcher.shared.tick(core: CoreHolder.shared.core)
            }
        }
    }

    public func stopForegroundSync() {
        foregroundTimer?.invalidate()
        foregroundTimer = nil
    }

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
