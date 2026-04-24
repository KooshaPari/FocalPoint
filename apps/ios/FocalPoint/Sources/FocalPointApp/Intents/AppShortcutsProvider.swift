import AppIntents

/// Registers all 6 FocalPoint Siri Shortcuts with suggested phrases.
struct FocalPointAppShortcutsProvider: AppShortcutsProvider {
    static var appShortcuts: [AppShortcut] {
        AppShortcut(
            intent: AddTaskIntent(),
            phrases: [
                "Add focus task \(.parameter(\.title))",
                "Create task \(.parameter(\.title))",
                "Add task \(.parameter(\.title)) to FocalPoint"
            ],
            shortTitle: "Add Focus Task",
            systemImageName: "checkmark.circle.fill"
        )

        AppShortcut(
            intent: StartFocusIntent(),
            phrases: [
                "Start focus session",
                "Begin focus",
                "Start FocalPoint focus for \(.parameter(\.duration)) minutes"
            ],
            shortTitle: "Start Focus Session",
            systemImageName: "timer"
        )

        AppShortcut(
            intent: CheckBalanceIntent(),
            phrases: [
                "Check FocalPoint balance",
                "How many credits do I have",
                "What's my FocalPoint balance"
            ],
            shortTitle: "Check Balance",
            systemImageName: "wallet.pass"
        )

        AppShortcut(
            intent: SyncNowIntent(),
            phrases: [
                "Sync FocalPoint",
                "Sync my data",
                "Update FocalPoint"
            ],
            shortTitle: "Sync Now",
            systemImageName: "arrow.clockwise"
        )

        AppShortcut(
            intent: GetNextFocusIntent(),
            phrases: [
                "When's my next focus",
                "What's my next focus session",
                "Show next focus time"
            ],
            shortTitle: "Next Focus",
            systemImageName: "calendar"
        )

        AppShortcut(
            intent: LogNoteIntent(),
            phrases: [
                "Log focus note \(.parameter(\.note))",
                "Add note \(.parameter(\.note))",
                "Log \(.parameter(\.note))"
            ],
            shortTitle: "Log Note",
            systemImageName: "note.text"
        )

        AppShortcut(
            intent: CoachyConversationIntent(),
            phrases: [
                "Hey Coachy",
                "Ask Coachy how I'm doing",
                "Coachy status",
                "What's my focus status"
            ],
            shortTitle: "Ask Coachy",
            systemImageName: "sparkles"
        )
    }
}
