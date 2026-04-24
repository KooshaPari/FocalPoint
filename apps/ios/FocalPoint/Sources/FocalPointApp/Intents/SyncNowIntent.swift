import AppIntents
import FocalPointCore

/// Siri Shortcut to trigger an immediate sync. Example: "Hey Siri, sync FocalPoint"
struct SyncNowIntent: AppIntent {
    static var title: LocalizedStringResource = "Sync FocalPoint"
    static var openAppWhenRun = false

    static var description = IntentDescription(
        "Trigger an immediate sync of FocalPoint data with connected services.",
        categoryIdentifier: "sync"
    )

    @MainActor
    func perform() async throws -> some IntentResult {
        let report = CoreHolder.shared.syncTick()
        CoreHolder.shared.bump()

        let message = "Sync completed: \(report.eventsProcessed) events processed"
        return .result(value: message, opensAppWhenTapped: false)
    }
}
