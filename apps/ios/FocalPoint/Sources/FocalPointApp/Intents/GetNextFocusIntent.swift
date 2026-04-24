import AppIntents
import FocalPointCore

/// Siri Shortcut to query the next scheduled focus window. Example: "Hey Siri, when's my next focus?"
struct GetNextFocusIntent: AppIntent {
    static var title: LocalizedStringResource = "When's My Next Focus"
    static var openAppWhenRun = false

    static var description = IntentDescription(
        "Get information about your next scheduled focus session or rule trigger.",
        categoryIdentifier: "focus"
    )

    @MainActor
    func perform() async throws -> some IntentResult {
        let core = CoreHolder.shared.core
        let alwaysOn = core.alwaysOn()

        do {
            let proposals = try alwaysOn.tick()
            if let nextProposal = proposals.first {
                let message = "Next focus: \(nextProposal.kind) at \(nextProposal.whenIso)"
                return .result(value: message, opensAppWhenTapped: false)
            } else {
                return .result(value: "No upcoming focus sessions scheduled", opensAppWhenTapped: false)
            }
        } catch {
            return .result(value: "Could not retrieve next focus session", opensAppWhenTapped: false)
        }
    }
}
