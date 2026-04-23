#if canImport(SwiftUI) && canImport(BackgroundTasks)
import Foundation
import BackgroundTasks

/// Registers a BGAppRefreshTask so the sync + eval loop keeps firing when
/// the app is backgrounded. Without this, rules that depend on connector
/// events (Canvas deadlines, GCal events, GitHub pushes) stop triggering
/// the moment the user leaves the app.
///
/// Required Info.plist entries (added via project.yml):
///   BGTaskSchedulerPermittedIdentifiers = ["com.koosha.focalpoint.refresh"]
///   UIBackgroundModes = ["fetch", "processing"]
@MainActor
public enum BackgroundSync {
    public static let taskId = "com.koosha.focalpoint.refresh"

    /// Register the BGTaskScheduler handler. Must be called before
    /// `application(_:didFinishLaunchingWithOptions:)` returns — SwiftUI
    /// apps call it from the App struct init or the root `.task` block.
    public static func register() {
        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: taskId,
            using: nil
        ) { task in
            guard let refresh = task as? BGAppRefreshTask else {
                task.setTaskCompleted(success: false)
                return
            }
            handle(refresh)
        }
    }

    /// Schedule the next refresh. iOS decides when to actually fire it —
    /// typically no sooner than 15 min, and throttled based on app-usage
    /// heuristics. Call this at app launch and after every foreground→
    /// background transition.
    public static func schedule(earliestMinutes: Int = 15) {
        let req = BGAppRefreshTaskRequest(identifier: taskId)
        req.earliestBeginDate = Date(timeIntervalSinceNow: Double(earliestMinutes * 60))
        do {
            try BGTaskScheduler.shared.submit(req)
        } catch {
            print("[FocalPoint] BG refresh schedule failed: \(error)")
        }
    }

    private static func handle(_ task: BGAppRefreshTask) {
        // Always re-schedule before doing work so the system keeps
        // granting us time.
        schedule()

        task.expirationHandler = {
            // iOS gives us a soft deadline — surrender gracefully.
            task.setTaskCompleted(success: false)
        }

        Task { @MainActor in
            _ = CoreHolder.shared.syncTick()
            _ = CoreHolder.shared.evalTick()
            NotificationDispatcher.shared.tick(core: CoreHolder.shared.core)
            task.setTaskCompleted(success: true)
        }
    }
}
#endif
