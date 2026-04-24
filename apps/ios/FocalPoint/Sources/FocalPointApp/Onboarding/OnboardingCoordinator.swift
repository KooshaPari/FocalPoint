#if canImport(SwiftUI)
import Foundation
import Combine
import CryptoKit
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
        case consent
        case welcome
        case goals
        case connect
        case pickTemplate
        case permissions
        case done

        public var id: Int { rawValue }

        public var title: String {
            switch self {
            case .consent: return "Privacy & Terms"
            case .welcome: return "Meet Coachy"
            case .goals: return "What are you focusing on?"
            case .connect: return "Connect your life"
            case .pickTemplate: return "Pick a starting rule"
            case .permissions: return "Grant permissions"
            case .done: return "All set!"
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

    @Published public private(set) var step: Step = .consent
    @Published public var goals: Set<Goal> = []
    @Published public var canvasConnected: Bool = false
    @Published public var selectedTemplateId: String?
    @Published public var notificationsStatus: PermissionStatus = .notDetermined
    @Published public var familyControlsStatus: PermissionStatus = .notDetermined
    @Published public var calendarStatus: PermissionStatus = .notDetermined

    // Consent state
    @Published public var privacyAccepted: Bool = false
    @Published public var termsAccepted: Bool = false
    @Published public var diagnosticsEnabled: Bool = false

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
        case .consent: return privacyAccepted && termsAccepted
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

    public var isFinalStep: Bool { step == .done }

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

    /// Prompt for notification authorization via NotificationPermissionManager.
    /// Registers the 4 notification action categories (COACHY_NUDGE, RITUAL_REMINDER,
    /// RULE_FIRED, BACKUP_COMPLETE) and sets up the delegate for handling user responses.
    /// If already granted, does not re-prompt. Fails loudly via status flip, not a silent no-op.
    public func requestNotificationsPermission() async {
        #if canImport(UserNotifications)
        let center = UNUserNotificationCenter.current()
        let settings = await center.notificationSettings()
        if settings.authorizationStatus == .authorized ||
           settings.authorizationStatus == .provisional ||
           settings.authorizationStatus == .ephemeral {
            notificationsStatus = .granted
            // Still set up the delegate in case it wasn't already
            center.delegate = NotificationPermissionManager.shared
            return
        }
        if settings.authorizationStatus == .denied {
            // System won't re-prompt; the UI routes to Settings via
            // `openSettingsURLString`.
            notificationsStatus = .denied
            return
        }

        // Request permissions via NotificationPermissionManager (alert + sound + badge + criticalAlert)
        let granted = await NotificationPermissionManager.shared.requestNotificationPermissions()
        notificationsStatus = granted ? .granted : .denied

        // Set up notification delegate for handling user responses
        if granted {
            center.delegate = NotificationPermissionManager.shared
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

    // MARK: - Consent recording

    /// Record consent acceptance in the audit chain. Called when advancing past
    /// the consent step. Computes SHA-256 hashes of the bundled legal documents
    /// and persists them alongside the acceptance timestamp.
    public func recordConsentAcceptance(into core: FocalPointCore) throws {
        guard privacyAccepted && termsAccepted else { return }

        let privacyHash = SHA256(contentsOf: "PRIVACY.md")
        let termsHash = SHA256(contentsOf: "TERMS.md")
        let now = ISO8601DateFormatter().string(from: Date())

        // Write audit record with the consent payload including doc hashes.
        // This creates a tamper-evident record of what version the user agreed to.
        let payload: [String: String] = [
            "privacy_hash": privacyHash,
            "terms_hash": termsHash,
            "privacy_ver": "1.0",
            "terms_ver": "1.0",
            "timestamp": now,
            "diagnostics_enabled": diagnosticsEnabled ? "true" : "false"
        ]
        try core.audit().recordMutation(
            entity: "consent",
            action: "accepted",
            payload: payload
        )

        // Persist acceptance timestamps for re-prompt on version change.
        UserDefaults.standard.set(now, forKey: "app.consentPrivacy_acceptedAt")
        UserDefaults.standard.set(now, forKey: "app.consentTerms_acceptedAt")
    }

    /// Check if consent documents have been updated (by hash). If so, return true
    /// to trigger re-prompt next launch.
    public func shouldRePromptConsent() -> Bool {
        let privacyHash = SHA256(contentsOf: "PRIVACY.md")
        let termsHash = SHA256(contentsOf: "TERMS.md")
        let stored = UserDefaults.standard.dictionary(forKey: "app.consentDocumentHashes") as? [String: String] ?? [:]
        return stored["privacy"] != privacyHash || stored["terms"] != termsHash
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
        step = .consent
        goals = []
        canvasConnected = false
        selectedTemplateId = nil
        notificationsStatus = .notDetermined
        familyControlsStatus = .pendingEntitlement
        calendarStatus = .notDetermined
        privacyAccepted = false
        termsAccepted = false
        diagnosticsEnabled = false
    }
}

// MARK: - Helpers

/// Compute SHA-256 hash of a bundled resource file (e.g., "PRIVACY.md").
/// Returns a hex-encoded digest.
private func SHA256(contentsOf filename: String) -> String {
    guard let path = Bundle.main.url(forResource: filename, withExtension: nil)?.path,
          let data = try? Data(contentsOf: URL(fileURLWithPath: path)) else {
        // Fallback to empty hash if file not found (shouldn't happen in production).
        return ""
    }

    // Use CryptoKit for SHA-256 hashing.
    let digest = CryptoKit.SHA256.hash(data: data)
    return digest.withUnsafeBytes { ptr in
        ptr.map { String(format: "%02x", $0) }.joined()
    }
}

#endif
