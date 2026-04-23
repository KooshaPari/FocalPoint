// Enforcement.swift — FocalPoint on-device enforcement driver.
//
// (a) FamilyControlsEnforcementDriver is correctness-reviewed against Apple's
//     2026 FamilyControls / ManagedSettings / DeviceActivity APIs. The flagged
//     branch below is what actually runs on device the moment Apple grants the
//     `com.apple.developer.family-controls` entitlement.
// (b) The `FOCALPOINT_HAS_FAMILYCONTROLS` Swift Active Compilation Condition
//     (set via `SWIFT_ACTIVE_COMPILATION_CONDITIONS` in `project.yml` on the
//     Enforcement + FocalPointApp targets) is the ONLY thing to flip the day
//     Apple approves the entitlement review. See
//     `docs/reference/family_controls_enablement.md` for the exact procedure.
// (c) No CI test exercises the flagged branch: FamilyControls requires a
//     signed app with the entitlement on a real iPhone; simulators and the
//     "Designed for iPad on Mac" runner cannot load the framework or honor
//     the shield. Correctness is maintained by API-shape review + manual
//     device test plan (see docs/reference/family_controls_enablement.md).

import Foundation
import os

#if canImport(FamilyControls)
import FamilyControls
#endif
#if canImport(FamilyControls) && FOCALPOINT_HAS_FAMILYCONTROLS
import ManagedSettings
import DeviceActivity
#endif

#if canImport(FamilyControls) && FOCALPOINT_HAS_FAMILYCONTROLS
extension ManagedSettingsStore.Name {
    /// Dedicated ManagedSettings store namespace for FocalPoint shields.
    /// Keeps our shield configuration isolated from any other store the user
    /// might have from a different app and lets `retract()` scope cleanup.
    static let focalpoint = Self("app.focalpoint.shield")
}

extension DeviceActivityName {
    /// Single monitoring schedule identifier used for the active focus block.
    static let focalpointShield = Self("app.focalpoint.shield")
}
#endif

/// Placeholder matching the Rust enforcement policy shape.
///
/// `tokens` is optional because translating `blockedBundleIds` → FamilyControls
/// `ApplicationToken`s requires the user to pick apps via
/// `FamilyActivityPicker`; bundle IDs alone are insufficient on iOS 16+.
/// The picker output (`FamilyActivitySelection`) is carried through from the
/// UI layer. When nil (or when the flag is off), we fall back to an empty
/// selection and apply no shields — matching the current log-only behavior.
public struct EnforcementPolicy: Sendable {
    // NOTE: intentionally not `Hashable` / `Equatable`. When
    // `canImport(FamilyControls)` is true the `tokens` field is a
    // `FamilyActivitySelection`, which is neither Hashable nor Equatable.
    // Consumers that need identity should key off the owning focus-block id,
    // not the policy value.
    public let blockedBundleIds: [String]
    public let endsAt: Date?
    #if canImport(FamilyControls)
    public let tokens: FamilyActivitySelection?
    #endif

    #if canImport(FamilyControls)
    public init(blockedBundleIds: [String], endsAt: Date?, tokens: FamilyActivitySelection? = nil) {
        self.blockedBundleIds = blockedBundleIds
        self.endsAt = endsAt
        self.tokens = tokens
    }
    #else
    public init(blockedBundleIds: [String], endsAt: Date?) {
        self.blockedBundleIds = blockedBundleIds
        self.endsAt = endsAt
    }
    #endif
}

public protocol EnforcementDriver {
    func apply(policy: EnforcementPolicy)
    func retract()
}

/// Console-logging no-op driver. Safe to use in simulator and previews.
public struct StubEnforcementDriver: EnforcementDriver {
    private let log = Logger(subsystem: "app.focalpoint.enforcement", category: "stub")
    public private(set) var lastAppliedCount: Int = 0

    public init() {}

    public func apply(policy: EnforcementPolicy) {
        log.info("StubEnforcementDriver.apply blocked=\(policy.blockedBundleIds.count, privacy: .public) endsAt=\(policy.endsAt?.description ?? "nil", privacy: .public)")
    }

    public func retract() {
        log.info("StubEnforcementDriver.retract")
    }
}

/// Real FamilyControls-backed driver.
///
/// The flagged branch (`FOCALPOINT_HAS_FAMILYCONTROLS`) holds the production
/// implementation against Apple's FamilyControls / ManagedSettings /
/// DeviceActivity APIs. Until the entitlement lands the branch is dead code
/// to the compiler (the flag is absent), and the driver falls back to
/// structured logging identical to `StubEnforcementDriver` so release builds
/// never accidentally believe enforcement is active.
public struct FamilyControlsEnforcementDriver: EnforcementDriver {
    private let log = Logger(subsystem: "app.focalpoint.enforcement", category: "familycontrols")

    public init() {}

    /// Request FamilyControls authorization for the individual (not a child
    /// account). Must be called once, typically from a settings view, before
    /// any `apply(policy:)` will have effect. No-op outside the flag.
    public func ensureAuthorized() async throws {
        #if canImport(FamilyControls) && FOCALPOINT_HAS_FAMILYCONTROLS
        // `.individual` is correct for users managing their own device. For a
        // parent managing a child's device we would use `.child`; FocalPoint
        // is self-enforcement only.
        try await AuthorizationCenter.shared.requestAuthorization(for: .individual)
        log.info("FamilyControls authorization requested")
        #else
        log.info("FamilyControls unavailable; ensureAuthorized no-op")
        #endif
    }

    public func apply(policy: EnforcementPolicy) {
        #if canImport(FamilyControls) && FOCALPOINT_HAS_FAMILYCONTROLS
        // 1. Resolve the ApplicationToken set. Bundle IDs alone cannot be
        //    mapped to tokens on-device (Apple privacy boundary); the picker
        //    output carried on `policy.tokens` is the source of truth. An
        //    empty selection means "log only" — same effect as stub.
        let selection = policy.tokens ?? FamilyActivitySelection()
        let appTokens = selection.applicationTokens
        let categoryTokens = selection.categoryTokens
        let webTokens = selection.webDomainTokens

        // 2. Install shields via our dedicated ManagedSettingsStore namespace.
        let store = ManagedSettingsStore(named: .focalpoint)
        store.shield.applications = appTokens.isEmpty ? nil : appTokens
        store.shield.applicationCategories = categoryTokens.isEmpty
            ? nil
            : .specific(categoryTokens)
        store.shield.webDomains = webTokens.isEmpty ? nil : webTokens

        // 3. Register a DeviceActivity schedule so the shield is torn down
        //    automatically at `policy.endsAt` even if the app is killed.
        //    When no endsAt is provided we still start a monitoring window
        //    with a far-future end; `retract()` is the authoritative release.
        let now = Date()
        let end = policy.endsAt ?? now.addingTimeInterval(24 * 60 * 60)
        let cal = Calendar.current
        let schedule = DeviceActivitySchedule(
            intervalStart: cal.dateComponents([.hour, .minute, .second], from: now),
            intervalEnd: cal.dateComponents([.hour, .minute, .second], from: end),
            repeats: false
        )
        let center = DeviceActivityCenter()
        do {
            center.stopMonitoring([.focalpointShield])
            try center.startMonitoring(.focalpointShield, during: schedule)
        } catch {
            log.error("DeviceActivity startMonitoring failed: \(String(describing: error), privacy: .public)")
        }

        log.info("FamilyControls apply blocked=\(policy.blockedBundleIds.count, privacy: .public) appTokens=\(appTokens.count, privacy: .public) endsAt=\(end.description, privacy: .public)")
        #else
        // Off-flag: no entitlement, no shield. We intentionally do NOT simulate
        // enforcement — that would be a correctness hazard. The flagged branch
        // above documents exactly what will run once the entitlement lands:
        //   - ManagedSettingsStore(named: .focalpoint).shield.applications = tokens
        //   - DeviceActivityCenter().startMonitoring(.focalpointShield, during: schedule)
        //   - schedule window = [now, policy.endsAt]
        log.info("FamilyControls unavailable; no-op apply blocked=\(policy.blockedBundleIds.count, privacy: .public)")
        #endif
    }

    public func retract() {
        #if canImport(FamilyControls) && FOCALPOINT_HAS_FAMILYCONTROLS
        let store = ManagedSettingsStore(named: .focalpoint)
        store.shield.applications = nil
        store.shield.applicationCategories = nil
        store.shield.webDomains = nil
        DeviceActivityCenter().stopMonitoring([.focalpointShield])
        log.info("FamilyControls retract cleared shield + stopped monitoring")
        #else
        // Off-flag mirror: would clear shield.applications and stopMonitoring
        // on the .focalpointShield DeviceActivityName.
        log.info("FamilyControls unavailable; no-op retract")
        #endif
    }
}
