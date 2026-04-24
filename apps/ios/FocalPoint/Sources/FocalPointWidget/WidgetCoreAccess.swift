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

/// Lightweight read-only helper to fetch widget data from the shared SQLite database.
/// Opens the App Group container's FocalPoint.sqlite in read-only mode.
/// Returns empty snapshot on any error (graceful fallback).
enum WidgetCoreAccess {
    /// Open the shared App Group SQLite database and fetch a snapshot.
    /// Queries (all read-only, SQLITE_OPEN_READONLY):
    /// - Wallet balance: SELECT (earned - spent) FROM wallet LIMIT 1
    /// - Streak days: SELECT count FROM wallet_streaks WHERE name = 'focus' LIMIT 1
    /// - Top 3 tasks: SELECT title FROM tasks WHERE status = 'active' ORDER BY priority DESC LIMIT 3
    /// Returns a placeholder if the database is not yet available, app is not installed, or query fails.
    static func fetchSnapshot() -> WidgetSnapshot {
        let groupId = "group.com.koosha.focalpoint"
        guard let groupURL = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: groupId) else {
            // App Group not available (e.g., entitlement not granted in dev build).
            return .init()
        }

        let dbPath = groupURL.appendingPathComponent("FocalPoint.sqlite").path
        guard FileManager.default.fileExists(atPath: dbPath) else {
            // Main app hasn't created the database yet.
            return .init()
        }

        var db: OpaquePointer?
        let openResult = sqlite3_open_v2(dbPath, &db, SQLITE_OPEN_READONLY, nil)
        guard openResult == SQLITE_OK, let db = db else {
            // Failed to open database.
            return .init()
        }
        defer { sqlite3_close(db) }

        var creditsBalance = 0
        var streakDays = 0
        var topPriorities: [String] = []

        // Query 1: Fetch wallet balance (earned - spent).
        let walletSQL = "SELECT (earned - spent) FROM wallet LIMIT 1"
        var walletStmt: OpaquePointer?
        if sqlite3_prepare_v2(db, walletSQL, -1, &walletStmt, nil) == SQLITE_OK, let stmt = walletStmt {
            defer { sqlite3_finalize(stmt) }
            if sqlite3_step(stmt) == SQLITE_ROW {
                creditsBalance = Int(sqlite3_column_int(stmt, 0))
            }
        }

        // Query 2: Fetch streak days (generic "streak" from wallet_streaks).
        let streakSQL = "SELECT count FROM wallet_streaks WHERE name = 'focus' LIMIT 1"
        var streakStmt: OpaquePointer?
        if sqlite3_prepare_v2(db, streakSQL, -1, &streakStmt, nil) == SQLITE_OK, let stmt = streakStmt {
            defer { sqlite3_finalize(stmt) }
            if sqlite3_step(stmt) == SQLITE_ROW {
                streakDays = Int(sqlite3_column_int(stmt, 0))
            }
        }

        // Query 3: Fetch top 3 active tasks ordered by priority (descending).
        let tasksSQL = "SELECT title FROM tasks WHERE status = 'active' ORDER BY priority DESC LIMIT 3"
        var tasksStmt: OpaquePointer?
        if sqlite3_prepare_v2(db, tasksSQL, -1, &tasksStmt, nil) == SQLITE_OK, let stmt = tasksStmt {
            defer { sqlite3_finalize(stmt) }
            while sqlite3_step(stmt) == SQLITE_ROW {
                if let cStr = sqlite3_column_text(stmt, 0) {
                    let title = String(cString: cStr)
                    topPriorities.append(title)
                }
            }
        }

        return WidgetSnapshot(
            creditsBalance: creditsBalance,
            streakDays: streakDays,
            coachyPoseName: "neutral",
            topPriorities: topPriorities
        )
    }
}
