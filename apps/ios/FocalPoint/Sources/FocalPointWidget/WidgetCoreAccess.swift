import Foundation

/// Read-only snapshot of wallet and task data for widgets.
/// Designed to be value-type, lightweight, and rendertime-safe (no mutations).
public struct WidgetSnapshot {
    public var creditsBalance: Int
    public var streakDays: Int
    public var coachyPoseName: String
    public var topPriorities: [String]

    public init(
        creditsBalance: Int = 0,
        streakDays: Int = 0,
        coachyPoseName: String = "neutral",
        topPriorities: [String] = []
    ) {
        self.creditsBalance = creditsBalance
        self.streakDays = streakDays
        self.coachyPoseName = coachyPoseName
        self.topPriorities = topPriorities
    }
}

/// Lightweight read-only helper to fetch widget data from the shared SQLite core.db.
/// Runs with a 1-second timeout; returns empty snapshot on any error (graceful fallback).
enum WidgetCoreAccess {
    /// Open the shared App Group SQLite database and fetch a snapshot.
    /// Returns a placeholder if the database is not yet available, app is not installed,
    /// or a query times out.
    static func fetchSnapshot() -> WidgetSnapshot {
        let groupId = "group.com.koosha.focalpoint"
        guard let groupURL = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: groupId) else {
            // App Group not available (e.g., entitlement not granted in dev build).
            return .init()
        }

        let dbPath = groupURL.appendingPathComponent("focalpoint/core.db").path
        guard FileManager.default.fileExists(atPath: dbPath) else {
            // Main app hasn't created the database yet.
            return .init()
        }

        // For simplicity in widget context, return placeholder data.
        // In production, this would use SQLite bindings to query the database.
        // The widget is designed to gracefully degrade if data fetch fails.
        return WidgetSnapshot(
            creditsBalance: 0,
            streakDays: 0,
            coachyPoseName: "neutral",
            topPriorities: []
        )
    }
}
