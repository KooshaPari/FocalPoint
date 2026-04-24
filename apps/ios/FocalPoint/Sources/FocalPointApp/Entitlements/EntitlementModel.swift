import SwiftUI
import Observation

/// Observable entitlement state for SwiftUI.
/// Publishes current tier, feature availability, and subscription status.
/// Connected to Rust `EntitlementStore` via FFI.
@Observable
final class EntitlementModel {
    // MARK: - State

    /// Current subscription tier.
    var tier: SubscriptionTier = .free

    /// Whether subscription is active (not expired).
    var isActive: Bool = true

    /// Days until subscription expiry (nil for Free tier).
    var daysUntilExpiry: Int?

    /// Whether entitlements are being refreshed from App Store.
    var isRefreshing: Bool = false

    // MARK: - Dependencies

    private let storeKit2Manager: StoreKit2Manager
    private var refreshTask: Task<Void, Never>?

    // MARK: - Lifecycle

    init(storeKit2Manager: StoreKit2Manager = StoreKit2Manager()) {
        self.storeKit2Manager = storeKit2Manager

        // Initialize with Free tier; will update after FFI sync
        Task {
            await refreshFromRust()
        }
    }

    // MARK: - Public API

    /// Feature gates: check if user can perform an action.

    /// Can add another rule?
    func canAddRule(current: Int) -> Bool {
        switch tier {
        case .free:
            return current < 3
        case .plus, .pro, .family:
            return true
        }
    }

    /// Can add another task?
    func canAddTask(current: Int) -> Bool {
        switch tier {
        case .free:
            return current < 3
        case .plus, .pro, .family:
            return true
        }
    }

    /// How many minutes between connector refreshes?
    func connectorRefreshCadenceMinutes() -> Int {
        switch tier {
        case .free:
            return 240 // 4 hours
        case .plus, .pro, .family:
            return 15 // 15 minutes
        }
    }

    /// Can create custom focus/break durations?
    func hasCustomSessionDurations() -> Bool {
        tier != .free
    }

    /// Voice provider for Coachy.
    func voiceProvider() -> VoiceProvider {
        switch tier {
        case .free:
            return .silent
        case .plus:
            return .native
        case .pro, .family:
            return .elevenLabs
        }
    }

    /// Can use Live Activity (lock screen)?
    func hasLiveActivity() -> Bool {
        tier != .free
    }

    /// Can use HomeKit widget?
    func hasHomeKitWidget() -> Bool {
        tier != .free
    }

    /// How many days to retain audit records?
    func auditRetentionDays() -> Int {
        switch tier {
        case .free:
            return 7
        case .plus:
            return 90
        case .pro:
            return 180
        case .family:
            return 365
        }
    }

    /// Can sync via CloudKit?
    func hasCloudKitSync() -> Bool {
        tier != .free
    }

    /// Max nudges per day?
    func nudgeLimitPerDay() -> Int {
        switch tier {
        case .free:
            return 0
        case .plus:
            return 3
        case .pro, .family:
            return Int.max
        }
    }

    /// Can use proactive 24h-ahead nudges?
    func hasProactiveNudges() -> Bool {
        tier == .pro || tier == .family
    }

    /// Can customize Coachy cosmetics?
    func canCustomizeCoachy() -> Bool {
        tier == .pro || tier == .family
    }

    /// Access to template marketplace?
    func hasTemplateMarketplace() -> Bool {
        tier == .pro || tier == .family
    }

    /// Analytics dashboard tier.
    func analyticsTier() -> AnalyticsTier {
        switch tier {
        case .free:
            return .none
        case .plus:
            return .basic
        case .pro, .family:
            return .advanced
        }
    }

    /// Has family dashboard (for managing child accounts)?
    func hasFamilyDashboard() -> Bool {
        tier == .family
    }

    /// Refresh entitlements from App Store.
    /// Called on app launch, after purchase, and every 24h.
    func refreshFromAppStore() async {
        isRefreshing = true
        defer { isRefreshing = false }

        await storeKit2Manager.requestRefresh()

        // Give App Store a moment to sync
        try? await Task.sleep(for: .seconds(1))

        // Refresh from Rust store
        await refreshFromRust()
    }

    // MARK: - Private

    /// Fetch current entitlements from Rust FFI.
    /// TODO(v1): Wire to focus_ffi::EntitlementsApi::current_tier()
    private func refreshFromRust() async {
        // Stub: assume Free tier until Rust FFI is wired
        // Real implementation:
        // let tierString = focus_ffi.entitlementsApi.currentTier()
        // self.tier = SubscriptionTier(rawValue: tierString) ?? .free

        self.tier = .free
        self.isActive = true
        self.daysUntilExpiry = nil

        print("📊 Entitlements refreshed: \(tier)")
    }
}

// MARK: - Enums

enum SubscriptionTier: String, CaseIterable {
    case free = "free"
    case plus = "plus"
    case pro = "pro"
    case family = "family"

    var displayName: String {
        switch self {
        case .free:
            return "Free"
        case .plus:
            return "Plus"
        case .pro:
            return "Pro"
        case .family:
            return "Family"
        }
    }

    var monthlyPrice: String {
        switch self {
        case .free:
            return "Free"
        case .plus:
            return "$4.99/mo"
        case .pro:
            return "$9.99/mo"
        case .family:
            return "$14.99/mo"
        }
    }

    var annualPrice: String? {
        switch self {
        case .free:
            return nil
        case .plus:
            return "$39.99/yr"
        case .pro:
            return "$79.99/yr"
        case .family:
            return nil // Family is monthly only
        }
    }

    var features: [String] {
        switch self {
        case .free:
            return [
                "3 custom rules",
                "3 tasks/goals",
                "1 connector (4h refresh)",
                "25/45 min sessions",
                "Basic Coachy",
                "7-day audit history",
            ]

        case .plus:
            return [
                "Unlimited rules & tasks",
                "All 4 connectors (15m refresh)",
                "Custom session durations",
                "Coachy voice",
                "Live Activity & widget",
                "90-day audit history",
                "CloudKit sync",
            ]

        case .pro:
            return [
                "Everything in Plus",
                "Premium voice (ElevenLabs)",
                "Template marketplace",
                "Advanced analytics",
                "24h-ahead proactive nudges",
                "Custom Coachy cosmetics",
                "180-day audit history",
            ]

        case .family:
            return [
                "Everything in Pro",
                "Up to 5 family members",
                "Parental dashboard",
                "Shared templates",
                "Family-wide overrides",
                "365-day audit history",
            ]
        }
    }
}

enum VoiceProvider: String {
    case silent = "silent"
    case native = "native"     // AVSpeechSynthesizer
    case elevenLabs = "eleven_labs" // Premium
}

enum AnalyticsTier {
    case none
    case basic
    case advanced
}
