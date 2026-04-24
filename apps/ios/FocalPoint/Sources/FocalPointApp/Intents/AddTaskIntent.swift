import AppIntents
import FocalPointCore

/// Siri Shortcut to add a new task. Example: "Hey Siri, add focus task buy milk"
struct AddTaskIntent: AppIntent {
    @Parameter(title: "Task Title") var title: String
    @Parameter(title: "Priority", description: "Task priority (optional)", default: "Normal")
    var priority: String
    @Parameter(title: "Duration (minutes)", description: "Estimated duration in minutes (optional)")
    var duration: Int?

    static var title: LocalizedStringResource = "Add Focus Task"
    static var openAppWhenRun = false

    static var description = IntentDescription(
        "Add a new focus task to your FocalPoint inbox.",
        categoryIdentifier: "tasks"
    )

    @MainActor
    func perform() async throws -> some IntentResult {
        let core = CoreHolder.shared.core
        let taskApi = core.tasks()

        let input = TaskInputDto(
            title: title,
            priority: priority,
            estimatedMinutes: UInt32(duration ?? 0)
        )

        let taskId = try taskApi.add(input: input)
        CoreHolder.shared.bump()

        return .result(value: "Added task: \(title)", opensAppWhenTapped: false)
    }
}
