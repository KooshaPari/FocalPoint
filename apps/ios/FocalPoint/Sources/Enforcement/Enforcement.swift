import Foundation
import os

/// Placeholder matching the Rust enforcement policy shape.
public struct EnforcementPolicy: Hashable, Sendable {
    public let blockedBundleIds: [String]
    public let endsAt: Date?

    public init(blockedBundleIds: [String], endsAt: Date?) {
        self.blockedBundleIds = blockedBundleIds
        self.endsAt = endsAt
    }
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

/// Real driver shell; body is gated on the FamilyControls entitlement being
/// present and on a physical device. Everywhere else it falls back to a stub.
public struct FamilyControlsEnforcementDriver: EnforcementDriver {
    private let log = Logger(subsystem: "app.focalpoint.enforcement", category: "familycontrols")

    public init() {}

    public func apply(policy: EnforcementPolicy) {
        #if canImport(FamilyControls) && !targetEnvironment(simulator)
        // TODO: requires com.apple.developer.family-controls entitlement.
        // Intended implementation: AuthorizationCenter request + ManagedSettings
        //   ShieldConfiguration + DeviceActivitySchedule keyed on policy.endsAt.
        log.info("FamilyControls apply (placeholder) blocked=\(policy.blockedBundleIds.count, privacy: .public)")
        #else
        log.info("FamilyControls unavailable; no-op apply blocked=\(policy.blockedBundleIds.count, privacy: .public)")
        #endif
    }

    public func retract() {
        #if canImport(FamilyControls) && !targetEnvironment(simulator)
        // TODO: requires com.apple.developer.family-controls entitlement.
        log.info("FamilyControls retract (placeholder)")
        #else
        log.info("FamilyControls unavailable; no-op retract")
        #endif
    }
}
