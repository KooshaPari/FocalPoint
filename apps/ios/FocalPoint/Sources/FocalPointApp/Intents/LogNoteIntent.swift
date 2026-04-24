import AppIntents
import FocalPointCore

/// Siri Shortcut to log a focus note to the audit chain. Example: "Hey Siri, log focus note I got distracted"
struct LogNoteIntent: AppIntent {
    @Parameter(title: "Note Content") var note: String

    static var title: LocalizedStringResource = "Log Focus Note"
    static var openAppWhenRun = false

    static var description = IntentDescription(
        "Log a note to your FocalPoint audit chain for tracking focus observations.",
        categoryIdentifier: "audit"
    )

    @MainActor
    func perform() async throws -> some IntentResult {
        let core = CoreHolder.shared.core
        let hostEventApi = core.hostEvents()

        let payload: [String: Any] = [
            "text": note,
            "source": "siri_shortcut"
        ]

        if let jsonData = try? JSONSerialization.data(withJSONObject: payload),
           let jsonString = String(data: jsonData, encoding: .utf8) {
            let event = HostEventDto(
                eventType: "note.logged",
                confidence: 1.0,
                payloadJson: jsonString,
                dedupeKey: nil
            )
            try hostEventApi.emit(dto: event)
        }

        CoreHolder.shared.bump()
        return .result(value: "Note logged: \(note)", opensAppWhenTapped: false)
    }
}
