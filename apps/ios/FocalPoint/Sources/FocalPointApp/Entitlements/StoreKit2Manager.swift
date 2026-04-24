import StoreKit
import Foundation

/// Manages StoreKit 2 in-app purchases and subscription lifecycle.
/// Handles transaction observations, product fetching, and entitlement updates.
///
/// v1 Design:
/// - Fetches products from App Store Connect (stubbed for dev, real IDs at launch)
/// - Observes `Transaction.currentEntitlements` to detect subscription changes
/// - Writes entitlements to Rust `EntitlementStore` via FFI
/// - Caches entitlement state locally; refreshes every 24h or on manual sync
@MainActor
final class StoreKit2Manager: NSObject, ObservableObject {
    @Published var isLoading = false
    @Published var products: [Product] = []
    @Published var error: String?

    // Real App Store Connect product IDs (to be configured at launch)
    private let productIds = [
        "com.focalpoint.plus.monthly",
        "com.focalpoint.plus.annual",
        "com.focalpoint.pro.monthly",
        "com.focalpoint.pro.annual",
        "com.focalpoint.family.monthly",
    ]

    override init() {
        super.init()
        setupTransactionListener()
    }

    /// Start listening for subscription transaction changes.
    /// Called on app launch to detect renewal, expiry, or cancellation.
    private func setupTransactionListener() {
        Task {
            for await result in Transaction.updates {
                switch result {
                case .verified(let transaction):
                    // Transaction is cryptographically signed by Apple
                    await handleVerifiedTransaction(transaction)
                case .unverified(_, let error):
                    self.error = "Unverified transaction: \(error)"
                    print("⚠️ Unverified transaction: \(error)")
                }
            }
        }
    }

    /// Process a verified transaction and update Rust entitlement store.
    private func handleVerifiedTransaction(_ transaction: Transaction) async {
        guard let appTransaction = try? await AppTransaction.current else {
            self.error = "Failed to get app transaction"
            return
        }

        print("✅ Verified transaction for product: \(transaction.productID)")

        // Extract signed receipt (base64)
        let receiptData = appTransaction.originalSignedJWT

        // Update Rust entitlements via FFI
        do {
            // TODO(v1): Wire to focus_ffi::EntitlementsApi::set_tier_from_receipt
            // For now, stub prints the receipt
            print("🔐 Receipt signature: \(receiptData.prefix(50))...")

            // Finish the transaction to mark it as consumed
            await transaction.finish()
        }
    }

    /// Fetch available products from App Store.
    /// Called on paywall sheet init.
    func fetchProducts() async {
        isLoading = true
        error = nil

        do {
            // Request product metadata from App Store
            let fetchedProducts = try await Product.products(for: productIds)

            // Sort by tier and billing period for display
            self.products = fetchedProducts.sorted { a, b in
                tierOrder(for: a.id) < tierOrder(for: b.id)
            }

            print("✅ Fetched \(self.products.count) products")
        } catch {
            self.error = "Failed to fetch products: \(error.localizedDescription)"
            print("❌ Product fetch error: \(error)")
        }

        isLoading = false
    }

    /// Initiate purchase for a product.
    /// Shows Apple's standard purchase sheet (user enters password, etc.).
    func purchase(product: Product) async {
        isLoading = true
        error = nil

        do {
            let result = try await product.purchase()

            switch result {
            case .success(.verified(let transaction)):
                print("✅ Purchase successful: \(product.id)")
                await handleVerifiedTransaction(transaction)
                // Transaction handling will update entitlements

            case .success(.unverified(_, let error)):
                self.error = "Unverified purchase: \(error)"
                print("⚠️ Unverified purchase: \(error)")

            case .userCancelled:
                print("ℹ️ User cancelled purchase")

            case .pending:
                print("⏳ Purchase pending (requires additional verification)")

            @unknown default:
                self.error = "Unknown purchase result"
            }
        } catch {
            self.error = "Purchase failed: \(error.localizedDescription)"
            print("❌ Purchase error: \(error)")
        }

        isLoading = false
    }

    /// Check if user has an active subscription.
    /// Scans through all transactions to find active entitlements.
    func checkActiveSubscription() async -> Bool {
        for await result in Transaction.currentEntitlements {
            switch result {
            case .verified(let transaction):
                // Found an active subscription
                print("✅ Active subscription found: \(transaction.productID)")
                return true
            case .unverified:
                continue
            }
        }
        return false
    }

    /// Request a subscription status from App Store.
    /// Used for debugging or explicit refresh (called after receipt verification).
    func requestRefresh() async {
        do {
            try await AppStore.sync()
            print("🔄 App Store sync requested")
        } catch {
            self.error = "Failed to sync with App Store: \(error.localizedDescription)"
        }
    }

    /// Helper: return tier display order for sorting.
    private func tierOrder(for productId: String) -> Int {
        switch productId {
        case _ where productId.contains("plus"):
            return 1
        case _ where productId.contains("pro"):
            return 2
        case _ where productId.contains("family"):
            return 3
        default:
            return 0
        }
    }
}

// MARK: - Test Stubs for Dev

#if DEBUG
extension StoreKit2Manager {
    /// Stub: simulate Plus tier purchase (dev only).
    func simulatePlusPurchase() {
        self.products = [
            Product.stub(id: "com.focalpoint.plus.monthly", displayName: "Plus Monthly", price: 4.99)
        ]
    }
}

/// Stub Product for SwiftUI previews.
extension Product {
    static func stub(id: String, displayName: String, price: Decimal) -> Product {
        // Placeholder; real Product requires App Store connection.
        // In production, Product is fetched from StoreKit2.
        fatalError("Use real Product from App Store")
    }
}
#endif
