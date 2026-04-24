import AppIntents
import FocalPointCore

/// Siri Shortcut to start a focus session. Example: "Hey Siri, start focus session"
struct StartFocusIntent: AppIntent {
    @Parameter(title: "Duration (minutes)", description: "Session duration in minutes (optional)")
    var duration: Int?
    @Parameter(title: "Rule Name", description: "Specific rule to activate (optional)")
    var rule: String?

    static var title: LocalizedStringResource = "Start Focus Session"
    static var openAppWhenRun = false

    static var description = IntentDescription(
        "Start a FocalPoint focus session with optional duration and rule.",
        categoryIdentifier: "focus"
    )

    @MainActor
    func perform() async throws -> some IntentResult {
        let core = CoreHolder.shared.core
        let rules = core.rules()

        var ruleId: String?
        if let ruleName = rule {
            // Query available rules and try to match by name
            do {
                let allRules = try rules.list()
                if let matched = allRules.first(where: { $0.title.lowercased().contains(ruleName.lowercased()) }) {
                    ruleId = matched.id
                }
            } catch {
                // Proceed without rule if query fails
            }
        }

        // Emit a focus session event to the host event stream
        let payload: [String: Any] = [
            "duration_minutes": duration ?? 25,
            "rule_id": ruleId ?? NSNull(),
            "source": "siri_shortcut"
        ]

        if let jsonData = try? JSONSerialization.data(withJSONObject: payload),
           let jsonString = String(data: jsonData, encoding: .utf8) {
            let hostEvent = HostEventDto(
                eventType: "focus_session_started",
                confidence: 1.0,
                payloadJson: jsonString,
                dedupeKey: nil
            )
            let hostEventApi = core.hostEvents()
            try hostEventApi.emit(dto: hostEvent)
        }

        CoreHolder.shared.bump()
        return .result(value: "Focus session started for \(duration ?? 25) minutes", opensAppWhenTapped: false)
    }
}
