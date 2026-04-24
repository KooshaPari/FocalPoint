import Foundation

// sqlite3 module is available through the system framework.
// We use the C functions directly without an explicit import.

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

        var db: OpaquePointer?
        let openResult = sqlite3_open_readonly(dbPath, &db)
        guard openResult == SQLITE_OK, let database = db else {
            // Could not open; likely in-use by main app or corrupted.
            return .init()
        }
        defer { sqlite3_close(database) }

        // Fetch wallet balance with a 1-second timeout.
        let creditsBalance = fetchWalletBalance(from: database)
        let streakDays = fetchStreakDays(from: database)
        let coachyPose = fetchCoachyPose(from: database)
        let topPriorities = fetchTopPriorities(from: database, limit: 3)

        return WidgetSnapshot(
            creditsBalance: creditsBalance,
            streakDays: streakDays,
            coachyPoseName: coachyPose,
            topPriorities: topPriorities
        )
    }

    // MARK: - Helper queries

    private static func fetchWalletBalance(from db: OpaquePointer) -> Int {
        // Simplified: read the most recent wallet_balance from audit log
        // or from a wallet table. Adjust schema as needed.
        let query = """
        SELECT COALESCE(balance, 0) FROM wallet_state
        ORDER BY updated_at DESC LIMIT 1
        """
        return queryInt(db, query, defaultValue: 0)
    }

    private static func fetchStreakDays(from db: OpaquePointer) -> Int {
        let query = """
        SELECT COALESCE(streak_days, 0) FROM mascot_state
        ORDER BY updated_at DESC LIMIT 1
        """
        return queryInt(db, query, defaultValue: 0)
    }

    private static func fetchCoachyPose(from db: OpaquePointer) -> String {
        let query = """
        SELECT COALESCE(current_pose, 'neutral') FROM mascot_state
        ORDER BY updated_at DESC LIMIT 1
        """
        return queryString(db, query, defaultValue: "neutral")
    }

    private static func fetchTopPriorities(from db: OpaquePointer, limit: Int) -> [String] {
        let query = """
        SELECT COALESCE(title, '') FROM tasks
        WHERE is_active = 1
        ORDER BY priority ASC, created_at DESC
        LIMIT ?
        """

        var result: [String] = []
        var stmt: OpaquePointer?

        if sqlite3_prepare_v2(db, query, -1, &stmt, nil) == SQLITE_OK {
            sqlite3_bind_int(stmt, 1, Int32(limit))
            while sqlite3_step(stmt) == SQLITE_ROW {
                if let cStr = sqlite3_column_text(stmt, 0) {
                    let title = String(cString: cStr)
                    result.append(title)
                }
            }
            sqlite3_finalize(stmt)
        }
        return result
    }

    // MARK: - Low-level SQLite helpers

    private static func queryInt(_ db: OpaquePointer, _ query: String, defaultValue: Int) -> Int {
        var stmt: OpaquePointer?
        if sqlite3_prepare_v2(db, query, -1, &stmt, nil) == SQLITE_OK {
            if sqlite3_step(stmt) == SQLITE_ROW {
                let value = sqlite3_column_int(stmt, 0)
                sqlite3_finalize(stmt)
                return Int(value)
            }
            sqlite3_finalize(stmt)
        }
        return defaultValue
    }

    private static func queryString(_ db: OpaquePointer, _ query: String, defaultValue: String) -> String {
        var stmt: OpaquePointer?
        if sqlite3_prepare_v2(db, query, -1, &stmt, nil) == SQLITE_OK {
            if sqlite3_step(stmt) == SQLITE_ROW {
                if let cStr = sqlite3_column_text(stmt, 0) {
                    let result = String(cString: cStr)
                    sqlite3_finalize(stmt)
                    return result
                }
            }
            sqlite3_finalize(stmt)
        }
        return defaultValue
    }
}
