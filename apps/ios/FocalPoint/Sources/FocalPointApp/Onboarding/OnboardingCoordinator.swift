#if canImport(SwiftUI)
import Foundation
import Combine
import FocalPointCore

/// Drives the 5-page first-run flow. Pure state machine; SwiftUI views bind
/// into it via `@ObservedObject` or `@StateObject`. Deliberately unit-test
/// friendly — no direct SwiftUI imports.
@MainActor
public final class OnboardingCoordinator: ObservableObject {
    public enum Step: Int, CaseIterable, Identifiable {
        case welcome
        case goals
        case connect
        case pickTemplate
        case permissions

        public var id: Int { rawValue }

        public var title: String {
            switch self {
            case .welcome: return "Meet Coachy"
            case .goals: return "What are you focusing on?"
            case .connect: return "Connect your life"
            case .pickTemplate: return "Pick a starting rule"
            case .permissions: return "Grant permissions"
            }
        }
    }

    public enum Goal: String, CaseIterable, Identifiable {
        case school
        case work
        case sleep
        case exercise
        case social
        case creative

        public var id: String { rawValue }

        public var displayName: String {
            switch self {
            case .school: return "School & grades"
            case .work: return "Deep work"
            case .sleep: return "Better sleep"
            case .exercise: return "Move more"
            case .social: return "Use less social"
            case .creative: return "Creative focus"
            }
        }

        public var iconSystemName: String {
            switch self {
            case .school: return "graduationcap.fill"
            case .work: return "laptopcomputer"
            case .sleep: return "moon.zzz.fill"
            case .exercise: return "figure.run"
            case .social: return "bubble.left.and.bubble.right.fill"
            case .creative: return "paintpalette.fill"
            }
        }
    }

    @Published public private(set) var step: Step = .welcome
    @Published public var goals: Set<Goal> = []
    @Published public var canvasConnected: Bool = false
    @Published public var selectedTemplateId: String?
    @Published public var notificationsGranted: Bool = false
    @Published public var familyControlsGranted: Bool = false

    public let minGoals: Int = 1
    public let maxGoals: Int = 3

    public init() {}

    // MARK: - Navigation

    public var canAdvance: Bool {
        switch step {
        case .welcome: return true
        case .goals: return (minGoals...maxGoals).contains(goals.count)
        case .connect: return true // skipping Canvas is allowed
        case .pickTemplate: return selectedTemplateId != nil
        case .permissions: return true
        }
    }

    public func advance() {
        guard canAdvance else { return }
        if let next = Step(rawValue: step.rawValue + 1) {
            step = next
        }
    }

    public func back() {
        if let prev = Step(rawValue: step.rawValue - 1) {
            step = prev
        }
    }

    public func jump(to step: Step) {
        self.step = step
    }

    public func toggleGoal(_ goal: Goal) {
        if goals.contains(goal) {
            goals.remove(goal)
        } else if goals.count < maxGoals {
            goals.insert(goal)
        }
    }

    public var isFinalStep: Bool { step == .permissions }

    // MARK: - Completion

    /// Seed rules into the core based on the selected template + goals.
    /// Returns count of rules inserted for tests to assert against.
    @discardableResult
    public func completeAndSeed(into core: FocalPointCore) throws -> Int {
        var installed: [String] = []
        if let id = selectedTemplateId,
           let t = RuleTemplates.all.first(where: { $0.id == id })
        {
            try core.mutations().upsert(rule: t.draft)
            installed.append(t.draft.id)
        }
        // Always seed a gentle baseline so the UI isn't empty.
        if installed.isEmpty {
            try core.mutations().upsert(rule: RuleTemplates.deepWorkSocialBlock.draft)
            installed.append(RuleTemplates.deepWorkSocialBlock.draft.id)
        }
        return installed.count
    }

    /// For tests — reset back to start.
    public func reset() {
        step = .welcome
        goals = []
        canvasConnected = false
        selectedTemplateId = nil
        notificationsGranted = false
        familyControlsGranted = false
    }
}
#endif
