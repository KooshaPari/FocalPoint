#if canImport(SwiftUI)
import Foundation
import FocalPointCore

/// Canonical pre-built rules users can install verbatim. Kept as Swift
/// literals (not in Rust) so the onboarding + template library can render
/// without round-tripping through the core.
public struct RuleTemplate: Identifiable, Hashable {
    public let id: String
    public let title: String
    public let subtitle: String
    public let draft: RuleDraft

    public init(id: String, title: String, subtitle: String, draft: RuleDraft) {
        self.id = id
        self.title = title
        self.subtitle = subtitle
        self.draft = draft
    }
}

public enum RuleTemplates {
    public static let all: [RuleTemplate] = [
        deepWorkSocialBlock,
        eveningWindDown,
        assignmentDueTodayLock,
        exerciseStreakReward,
        sleepDebtPenalty,
        weekendChillMode,
    ]

    public static let deepWorkSocialBlock = RuleTemplate(
        id: "tmpl-deep-work-social-block",
        title: "Deep-work social block",
        subtitle: "Blocks social apps during a focus session.",
        draft: RuleDraft(
            id: "deep-work-social-block",
            name: "Deep work — no social",
            triggerEvent: "focus:session_started",
            actions: [.block(profile: "social", durationSeconds: Int64(60 * 50))],
            priority: 80,
            cooldownSeconds: nil,
            durationSeconds: 60 * 50,
            explanationTemplate: "Social apps locked while {rule_name} is active.",
            enabled: true
        )
    )

    public static let eveningWindDown = RuleTemplate(
        id: "tmpl-evening-wind-down",
        title: "Evening wind-down",
        subtitle: "Notifies you to log off at 10pm; blocks at 11pm.",
        draft: RuleDraft(
            id: "evening-wind-down",
            name: "Evening wind-down",
            triggerEvent: "clock:hour_22",
            actions: [
                .notify(message: "Winding down — lights out in 1h."),
                .block(profile: "entertainment", durationSeconds: Int64(60 * 60 * 8)),
            ],
            priority: 60,
            cooldownSeconds: 60 * 60 * 8,
            durationSeconds: 60 * 60 * 8,
            explanationTemplate: "{rule_name}: winding down for sleep.",
            enabled: true
        )
    )

    public static let assignmentDueTodayLock = RuleTemplate(
        id: "tmpl-assignment-due-today",
        title: "Assignment due today — lock",
        subtitle: "Locks social when Canvas assignment is due <24h.",
        draft: RuleDraft(
            id: "assignment-due-today",
            name: "Assignment due <24h",
            triggerEvent: "canvas:assignment_due_soon",
            actions: [
                .block(profile: "social", durationSeconds: Int64(60 * 60 * 4)),
                .notify(message: "Canvas: assignment due in <24h. Focus."),
            ],
            priority: 95,
            cooldownSeconds: 60 * 60,
            durationSeconds: 60 * 60 * 4,
            explanationTemplate: "{rule_name} fired on {event_type}:{event_id}.",
            enabled: true
        )
    )

    public static let exerciseStreakReward = RuleTemplate(
        id: "tmpl-exercise-streak",
        title: "Exercise streak reward",
        subtitle: "Grants credit when you complete an exercise session.",
        draft: RuleDraft(
            id: "exercise-streak",
            name: "Exercise streak reward",
            triggerEvent: "health:workout_completed",
            actions: [
                .grantCredit(amount: 20),
                .streakIncrement(name: "exercise"),
            ],
            priority: 40,
            cooldownSeconds: 60 * 60 * 4,
            durationSeconds: nil,
            explanationTemplate: "Nice work. +20 credits for {rule_name}.",
            enabled: true
        )
    )

    public static let sleepDebtPenalty = RuleTemplate(
        id: "tmpl-sleep-debt",
        title: "Sleep-debt penalty",
        subtitle: "Deducts credit when sleep <6h reported.",
        draft: RuleDraft(
            id: "sleep-debt",
            name: "Sleep-debt penalty",
            triggerEvent: "health:sleep_debt_reported",
            actions: [
                .deductCredit(amount: 10),
                .notify(message: "Short sleep logged — take it easy today."),
            ],
            priority: 50,
            cooldownSeconds: 60 * 60 * 12,
            durationSeconds: nil,
            explanationTemplate: "{rule_name}: sleep debt recorded.",
            enabled: true
        )
    )

    public static let weekendChillMode = RuleTemplate(
        id: "tmpl-weekend-chill",
        title: "Weekend chill mode",
        subtitle: "Lifts all blocks Saturdays + Sundays.",
        draft: RuleDraft(
            id: "weekend-chill",
            name: "Weekend chill mode",
            triggerEvent: "clock:weekend",
            actions: [
                .unblock(profile: "social"),
                .unblock(profile: "entertainment"),
            ],
            priority: 10,
            cooldownSeconds: nil,
            durationSeconds: 60 * 60 * 48,
            explanationTemplate: "{rule_name}: weekend, relax.",
            enabled: true
        )
    )
}
#endif
