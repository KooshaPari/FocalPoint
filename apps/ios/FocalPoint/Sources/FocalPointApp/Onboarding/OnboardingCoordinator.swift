#if canImport(SwiftUI)
import Foundation
import Combine
import FocalPointCore
#if canImport(UserNotifications)
import UserNotifications
#endif
#if canImport(EventKit)
import EventKit
#endif
#if canImport(FamilyControls)
import FamilyControls
#endif

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

    /// Tri-state permission status surfaced in the permissions page. Distinct
    /// from a plain Bool so the UI can render "Granted", "Denied (Open
    /// Settings)", "Not yet asked", or "Pending entitlement" without ambiguity.
    public enum PermissionStatus: Equatable {
        case notDetermined
        case granted
        case denied
        /// Entitlement/capability not available in this build (e.g.
        /// FamilyControls without the Apple-approved entitlement).
        case pendingEntitlement
    }

    @Published public private(set) var step: Step = .welcome
    @Published public var goals: Set<Goal> = []
    @Published public var canvasConnected: Bool = false
    @Published public var selectedTemplateId: String?
    @Published public var notificationsStatus: PermissionStatus = .notDetermined
    @Published public var familyControlsStatus: PermissionStatus = .notDetermined
    @Published public var calendarStatus: PermissionStatus = .notDetermined

    // Back-compat mirrors — existing tests/UI touch these as plain Bools.
    public var notificationsGranted: Bool {
        get { notificationsStatus == .granted }
        set { notificationsStatus = newValue ? .granted : .denied }
    }
    public var familyControlsGranted: Bool {
        get { familyControlsStatus == .granted }
        set { familyControlsStatus = newValue ? .granted : .denied }
    }
    public var calendarGranted: Bool {
        get { calendarStatus == .granted }
        set { calendarStatus = newValue ? .granted : .denied }
    }

    public let minGoals: Int = 1
    public let maxGoals: Int = 3

    public init() {
        // FamilyControls entitlement is not yet shipped on this app — Phase 0
        // blocker per CLAUDE.md. Reflect that honestly instead of pretending a
        // toggle will work.
        familyControlsStatus = .pendingEntitlement
    }

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

    // MARK: - Permission requests (real OS calls)

    /// Refresh the notification status from the OS without prompting. Safe to
    /// call every time the page appears.
    public func refreshNotificationStatus() async {
        #if canImport(UserNotifications)
        let settings = await UNUserNotificationCenter.current().notificationSettings()
        switch settings.authorizationStatus {
        case .authorized, .provisional, .ephemeral:
            notificationsStatus = .granted
        case .denied:
            notificationsStatus = .denied
        case .notDetermined:
            notificationsStatus = .notDetermined
        @unknown default:
            notificationsStatus = .notDetermined
        }
        #endif
    }

    /// Prompt for notification authorization. If already granted, does not
    /// re-prompt. Fails loudly via status flip, not a silent no-op.
    public func requestNotificationsPermission() async {
        #if canImport(UserNotifications)
        let center = UNUserNotificationCenter.current()
        let settings = await center.notificationSettings()
        if settings.authorizationStatus == .authorized ||
           settings.authorizationStatus == .provisional ||
           settings.authorizationStatus == .ephemeral {
            notificationsStatus = .granted
            return
        }
        if settings.authorizationStatus == .denied {
            // System won't re-prompt; the UI routes to Settings via
            // `openSettingsURLString`.
            notificationsStatus = .denied
            return
        }
        do {
            let granted = try await center.requestAuthorization(options: [.alert, .badge, .sound])
            notificationsStatus = granted ? .granted : .denied
        } catch {
            notificationsStatus = .denied
        }
        #else
        notificationsStatus = .pendingEntitlement
        #endif
    }

    /// Request FamilyControls. In builds without the entitlement this stays in
    /// `.pendingEntitlement` — we refuse to fake success.
    public func requestFamilyControlsPermission() async {
        #if canImport(FamilyControls)
        if #available(iOS 16.0, *) {
            do {
                try await AuthorizationCenter.shared.requestAuthorization(for: .individual)
                familyControlsStatus = .granted
            } catch {
                // If the entitlement is missing, the system rejects the
                // request. Surface that as pending-entitlement rather than a
                // hard denial, since there's nothing the user can fix today.
                familyControlsStatus = .pendingEntitlement
            }
        } else {
            familyControlsStatus = .pendingEntitlement
        }
        #else
        familyControlsStatus = .pendingEntitlement
        #endif
    }

    /// Prompt for Calendar (EventKit). iOS 17+ uses the full-access API;
    /// earlier versions fall back to `requestAccess(to: .event)`.
    public func requestCalendarPermission() async {
        #if canImport(EventKit)
        let store = EKEventStore()
        if #available(iOS 17.0, *) {
            do {
                let granted = try await store.requestFullAccessToEvents()
                calendarStatus = granted ? .granted : .denied
            } catch {
                calendarStatus = .denied
            }
        } else {
            do {
                let granted = try await withCheckedThrowingContinuation { (cont: CheckedContinuation<Bool, Error>) in
                    store.requestAccess(to: .event) { granted, err in
                        if let err = err { cont.resume(throwing: err) }
                        else { cont.resume(returning: granted) }
                    }
                }
                calendarStatus = granted ? .granted : .denied
            } catch {
                calendarStatus = .denied
            }
        }
        #else
        calendarStatus = .pendingEntitlement
        #endif
    }

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
        notificationsStatus = .notDetermined
        familyControlsStatus = .pendingEntitlement
        calendarStatus = .notDetermined
    }
}
#endif
