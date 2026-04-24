import Foundation

// UniFFI-generated bindings (focus_ffi.swift) now provide the real
// `FocalPointCore` class. Keep Swift-only placeholder types (RuleId,
// ActiveRule) that the UI layer already references; they are not in the UDL.

/// Mirrors `focus_domain::RuleId` (opaque identifier).
public struct RuleId: Hashable, Codable, Sendable {
    public let raw: String
    public init(_ raw: String) { self.raw = raw }
}

/// Minimal placeholder for an active rule; the Rust side owns the real schema.
public struct ActiveRule: Hashable, Codable, Sendable {
    public let id: RuleId
    public let title: String
    public let endsAt: Date?

    public init(id: RuleId, title: String, endsAt: Date?) {
        self.id = id
        self.title = title
        self.endsAt = endsAt
    }
}

// MARK: - Ritual DTOs (placeholder until generated from UDL)

/// Weekly Review data transfer object — surfaces weekly focus metrics.
public struct WeeklyReviewDto: Equatable, Hashable {
    public var focusHoursTotal: Double
    public var sessionsCompleted: Int32
    public var creditsEarned: Int32
    public var creditsSpent: Int32
    public var tasksCompleted: Int32
    public var tasksSlipped: Int32
    public var topRulesFired: [String]
    public var streaksExtended: [String]
    public var winsSummary: String
    public var growthArea: String
    public var coachyOpening: String
    public var generatedAtIso: String

    public init(
        focusHoursTotal: Double,
        sessionsCompleted: Int32,
        creditsEarned: Int32,
        creditsSpent: Int32,
        tasksCompleted: Int32,
        tasksSlipped: Int32,
        topRulesFired: [String],
        streaksExtended: [String],
        winsSummary: String,
        growthArea: String,
        coachyOpening: String,
        generatedAtIso: String
    ) {
        self.focusHoursTotal = focusHoursTotal
        self.sessionsCompleted = sessionsCompleted
        self.creditsEarned = creditsEarned
        self.creditsSpent = creditsSpent
        self.tasksCompleted = tasksCompleted
        self.tasksSlipped = tasksSlipped
        self.topRulesFired = topRulesFired
        self.streaksExtended = streaksExtended
        self.winsSummary = winsSummary
        self.growthArea = growthArea
        self.coachyOpening = coachyOpening
        self.generatedAtIso = generatedAtIso
    }
}

/// Monthly Retrospective data transfer object — surfaces month-level summary.
public struct MonthlyRetroDto: Equatable, Hashable {
    public var monthLabel: String
    public var focusHoursTotal: Double
    public var weeklyFocusHours: [Double]
    public var daysActive: Int32
    public var focusTheme: String
    public var reflection: String
    public var focusHoursDelta: Double
    public var tasksCompletedDelta: Int32
    public var creditsEarnedDelta: Int32
    public var coachyReflection: String
    public var generatedAtIso: String

    public init(
        monthLabel: String,
        focusHoursTotal: Double,
        weeklyFocusHours: [Double],
        daysActive: Int32,
        focusTheme: String,
        reflection: String,
        focusHoursDelta: Double,
        tasksCompletedDelta: Int32,
        creditsEarnedDelta: Int32,
        coachyReflection: String,
        generatedAtIso: String
    ) {
        self.monthLabel = monthLabel
        self.focusHoursTotal = focusHoursTotal
        self.weeklyFocusHours = weeklyFocusHours
        self.daysActive = daysActive
        self.focusTheme = focusTheme
        self.reflection = reflection
        self.focusHoursDelta = focusHoursDelta
        self.tasksCompletedDelta = tasksCompletedDelta
        self.creditsEarnedDelta = creditsEarnedDelta
        self.coachyReflection = coachyReflection
        self.generatedAtIso = generatedAtIso
    }
}
