import SwiftUI

/// Helper view modifier to gate features behind a paywall.
/// Usage: `MyView().withFeatureGate(feature: .customSessions, paywallPresented: $shown)`
extension View {
    func featureGate(
        feature: FeatureGateName,
        entitlements: EntitlementModel,
        paywallPresented: Binding<Bool>
    ) -> some View {
        modifier(FeatureGateModifier(
            feature: feature,
            entitlements: entitlements,
            paywallPresented: paywallPresented
        ))
    }
}

/// Feature gate enumeration.
enum FeatureGateName: String, CaseIterable {
    case customSessionDurations
    case customConnectors
    case voiceCoach
    case liveActivity
    case homeKitWidget
    case advancedAnalytics
    case proactiveNudges
    case customCoachy
    case templateMarketplace
    case familyDashboard

    var requiredTier: SubscriptionTier {
        switch self {
        case .customSessionDurations, .voiceCoach, .liveActivity, .homeKitWidget:
            return .plus
        case .customConnectors:
            return .plus // Free has 1 connector, Plus has all 4
        case .advancedAnalytics, .proactiveNudges, .customCoachy, .templateMarketplace:
            return .pro
        case .familyDashboard:
            return .family
        }
    }

    var displayName: String {
        switch self {
        case .customSessionDurations:
            return "Custom session durations"
        case .customConnectors:
            return "All connectors"
        case .voiceCoach:
            return "Voice coaching"
        case .liveActivity:
            return "Lock screen activity"
        case .homeKitWidget:
            return "Home screen widget"
        case .advancedAnalytics:
            return "Advanced analytics"
        case .proactiveNudges:
            return "Proactive nudges"
        case .customCoachy:
            return "Custom Coachy cosmetics"
        case .templateMarketplace:
            return "Template marketplace"
        case .familyDashboard:
            return "Family dashboard"
        }
    }

    var paywallMessage: String {
        return "\(displayName) is available on \(requiredTier.displayName)"
    }
}

// MARK: - Modifier

struct FeatureGateModifier: ViewModifier {
    let feature: FeatureGateName
    let entitlements: EntitlementModel
    @Binding var paywallPresented: Bool

    func body(content: Content) -> some View {
        let isGated = !canAccessFeature()

        if isGated {
            ZStack {
                content
                    .disabled(true)
                    .opacity(0.5)

                VStack(spacing: 12) {
                    Image(systemName: "lock.fill")
                        .font(.title)
                        .foregroundColor(.secondary)

                    Text(feature.paywallMessage)
                        .font(.headline)

                    Button("Upgrade") {
                        paywallPresented = true
                    }
                    .buttonStyle(.borderedProminent)
                }
                .padding(20)
                .background(Color(.systemBackground))
                .cornerRadius(12)
            }
        } else {
            content
        }
    }

    private func canAccessFeature() -> Bool {
        entitlements.tier.rawValue >= feature.requiredTier.rawValue
    }
}

// MARK: - Convenience Methods

extension EntitlementModel {
    /// Check if a feature is accessible.
    func hasAccess(to feature: FeatureGateName) -> Bool {
        tier.rawValue >= feature.requiredTier.rawValue
    }

    /// Enum ordering for comparison (Free < Plus < Pro < Family).
    private var tierRank: Int {
        switch tier {
        case .free: return 0
        case .plus: return 1
        case .pro: return 2
        case .family: return 3
        }
    }
}

// MARK: - Debug / Testing

#if DEBUG
struct FeatureGatePreview: View {
    @State private var paywallPresented = false
    @State private var entitlements = EntitlementModel()

    var body: some View {
        VStack(spacing: 20) {
            Text("Feature: Voice Coaching")
                .font(.headline)

            Text("Current tier: \(entitlements.tier.displayName)")
                .font(.caption)
                .foregroundColor(.secondary)

            VStack {
                Text("This feature requires Plus tier")
                    .font(.callout)
            }
            .featureGate(
                feature: .voiceCoach,
                entitlements: entitlements,
                paywallPresented: $paywallPresented
            )

            Spacer()

            // Toggle tier for testing
            Picker("Tier", selection: $entitlements.tier) {
                ForEach(SubscriptionTier.allCases, id: \.self) { tier in
                    Text(tier.displayName).tag(tier)
                }
            }
        }
        .padding(20)
        .sheet(isPresented: $paywallPresented) {
            PaywallView()
        }
    }
}

#Preview {
    FeatureGatePreview()
}
#endif
