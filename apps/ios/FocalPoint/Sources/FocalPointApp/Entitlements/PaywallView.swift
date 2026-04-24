import SwiftUI

/// Subscription paywall sheet.
/// Shows tier comparison and enables purchases via StoreKit2Manager.
/// Displayed when user hits a tier limit (e.g., tries to add 4th rule on Free tier).
///
/// Design:
/// - Half-sheet modal (iOS 16+)
/// - Feature comparison table
/// - Purchase buttons for each tier
/// - Dismiss button ("Maybe later")
///
/// Stub (v1):
/// - Tiers are hardcoded; real App Store product IDs to be wired at launch
/// - Purchase flow is stubbed; will integrate StoreKit2Manager when product IDs are live
struct PaywallView: View {
    @Environment(\.dismiss) var dismiss
    @Environment(\.entitlements) var entitlements: EntitlementModel

    @StateObject private var storeKit2Manager = StoreKit2Manager()
    @State private var selectedTier: SubscriptionTier = .plus

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                // Header
                header

                ScrollView {
                    VStack(spacing: 20) {
                        // Tier selector (segmented control)
                        billingToggle

                        // Feature comparison
                        tierComparison

                        // Purchase button
                        purchaseButton

                        // Footnote
                        footnote
                    }
                    .padding(20)
                }

                // Dismiss button
                Divider()
                Button("Maybe later") {
                    dismiss()
                }
                .frame(maxWidth: .infinity)
                .padding(16)
                .foregroundColor(.secondary)
            }
            .navigationTitle("Upgrade")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }

    // MARK: - Views

    private var header: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Unlock More Features")
                .font(.title2)
                .fontWeight(.bold)

            Text("Upgrade to Plus, Pro, or Family to unlock unlimited rules, all connectors, and more.")
                .font(.callout)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(20)
        .background(Color(.systemGray6))
    }

    private var billingToggle: some View {
        HStack(spacing: 12) {
            Button(action: { selectedTier = .plus }) {
                Text("Monthly")
                    .frame(maxWidth: .infinity)
                    .font(.system(.body, design: .default))
                    .padding(12)
                    .background(selectedTier == .plus ? Color.blue : Color(.systemGray6))
                    .foregroundColor(selectedTier == .plus ? .white : .primary)
                    .cornerRadius(8)
            }

            Button(action: { selectedTier = .pro }) {
                VStack(spacing: 2) {
                    Text("Annual")
                        .font(.body)
                    Text("Save 33%")
                        .font(.caption)
                        .foregroundColor(.green)
                }
                .frame(maxWidth: .infinity)
                .padding(12)
                .background(selectedTier == .pro ? Color.blue : Color(.systemGray6))
                .foregroundColor(selectedTier == .pro ? .white : .primary)
                .cornerRadius(8)
            }
        }
    }

    private var tierComparison: some View {
        VStack(spacing: 12) {
            // Plus
            tierCard(
                tier: .plus,
                price: selectedTier == .plus ? "$4.99/mo" : "$39.99/yr",
                isSelected: true
            )

            // Pro
            tierCard(
                tier: .pro,
                price: selectedTier == .plus ? "$9.99/mo" : "$79.99/yr",
                isSelected: false
            )

            // Family
            tierCard(
                tier: .family,
                price: "$14.99/mo",
                isSelected: false,
                subtitle: "Up to 5 family members"
            )
        }
    }

    @ViewBuilder
    private func tierCard(
        tier: SubscriptionTier,
        price: String,
        isSelected: Bool,
        subtitle: String? = nil
    ) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text(tier.displayName)
                        .font(.headline)
                    if let subtitle {
                        Text(subtitle)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                Spacer()

                Text(price)
                    .font(.title3)
                    .fontWeight(.semibold)
                    .foregroundColor(.blue)
            }

            Divider()

            // Key features (show first 3)
            VStack(alignment: .leading, spacing: 6) {
                ForEach(tier.features.prefix(3), id: \.self) { feature in
                    HStack(spacing: 8) {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.green)
                            .font(.caption)
                        Text(feature)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                if tier.features.count > 3 {
                    Text("+ \(tier.features.count - 3) more")
                        .font(.caption)
                        .foregroundColor(.blue)
                }
            }
        }
        .padding(12)
        .background(Color(.systemGray6))
        .cornerRadius(12)
        .overlay(
            RoundedRectangle(cornerRadius: 12)
                .stroke(isSelected ? Color.blue : Color.clear, lineWidth: 2)
        )
        .onTapGesture {
            withAnimation { selectedTier = tier }
        }
    }

    private var purchaseButton: some View {
        Button(action: handlePurchase) {
            HStack {
                if storeKit2Manager.isLoading {
                    ProgressView()
                        .tint(.white)
                } else {
                    Text("Upgrade to \(selectedTier.displayName)")
                        .fontWeight(.semibold)
                }
            }
            .frame(maxWidth: .infinity)
            .padding(16)
            .background(Color.blue)
            .foregroundColor(.white)
            .cornerRadius(12)
        }
        .disabled(storeKit2Manager.isLoading)
    }

    private var footnote: some View {
        VStack(spacing: 8) {
            if let error = storeKit2Manager.error {
                Text("Error: \(error)")
                    .font(.caption)
                    .foregroundColor(.red)
            }

            VStack(spacing: 4) {
                Text("Cancel anytime. Subscription renews automatically.")
                    .font(.caption2)
                    .foregroundColor(.secondary)

                Link("View Terms", destination: URL(string: "https://focalpoint.app/terms")!)
                    .font(.caption2)
                    .foregroundColor(.blue)
            }
            .frame(maxWidth: .infinity)
        }
    }

    // MARK: - Actions

    private func handlePurchase() {
        Task {
            await storeKit2Manager.fetchProducts()

            // TODO(v1): Initiate purchase for selected tier
            // Once real product IDs are wired:
            // if let product = storeKit2Manager.products.first(where: { $0.id == productIdForTier(selectedTier) }) {
            //     await storeKit2Manager.purchase(product: product)
            // }
        }
    }
}

// MARK: - Environment

extension EnvironmentValues {
    @Entry var entitlements: EntitlementModel = EntitlementModel()
}

// MARK: - Previews

#Preview {
    PaywallView()
        .environment(\.entitlements, EntitlementModel())
}
