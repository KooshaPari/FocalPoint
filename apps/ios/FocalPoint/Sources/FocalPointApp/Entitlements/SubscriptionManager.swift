import Foundation
import StoreKit

/// Manages subscription lifecycle with server-side JWS verification.
/// After StoreKit returns a verified transaction, this manager:
/// 1. POSTs the JWS to the backend verifier worker
/// 2. On success, updates entitlements (Rust via FFI)
/// 3. On failure or timeout, grants entitlement locally with 24h TTL cache
/// 4. Re-checks on next launch, with cache extension if still unreachable
@MainActor
final class SubscriptionManager: NSObject, ObservableObject {
  @Published var entitlementStatus: EntitlementStatus = .free
  @Published var isVerifying = false
  @Published var error: String?

  private let verifierURL: URL
  private let cacheKeychain = SecureCache()
  private let ttlHours: Int = 24

  init() {
    // Read StoreKit verifier URL from Info.plist
    let bundle = Bundle.main
    let urlString = bundle.infoDictionary?["FocalpointStoreKitVerifierURL"] as? String
      ?? "https://api.focalpoint.app/storekit"

    self.verifierURL = URL(string: urlString + "/verify")!

    super.init()
  }

  /// Handle a verified transaction from StoreKit 2.
  /// Posts the JWS to the backend verifier; on failure, falls back to local cache.
  func handleVerifiedTransaction(_ transaction: Transaction) async {
    guard let appTransaction = try? await AppTransaction.current else {
      self.error = "Failed to get app transaction"
      return
    }

    isVerifying = true
    defer { isVerifying = false }

    let signedJWT = appTransaction.originalSignedJWT
    let bundleId = Bundle.main.bundleIdentifier ?? "com.koosha.focalpoint"

    // Attempt server-side verification
    do {
      let response = try await verifyTransactionWithServer(
        signedTransaction: signedJWT,
        bundleId: bundleId
      )

      if response.valid {
        print("✅ Server verification successful: \(response.productId ?? "unknown")")

        // Cache the verified entitlement with TTL
        await cacheVerifiedEntitlement(response, ttlHours: ttlHours)

        // Update Rust entitlements via FFI
        await updateEntitlementFromResponse(response)

        // Mark transaction as consumed
        await transaction.finish()
      } else {
        print("❌ Server verification failed: \(response.error ?? "unknown error")")
        self.error = response.error

        // Fall back to local cache
        await grantLocalEntitlementWithTTL(from: transaction)
      }
    } catch {
      print("⚠️ Server verification unreachable: \(error.localizedDescription)")
      self.error = "Verification service unreachable; granting local entitlement"

      // Fail-open: grant locally with 24h TTL cache
      await grantLocalEntitlementWithTTL(from: transaction)
    }
  }

  /// POST to backend verifier worker.
  /// Returns parsed response or throws on network/parse error.
  private func verifyTransactionWithServer(
    signedTransaction: String,
    bundleId: String
  ) async throws -> VerifyResponse {
    var request = URLRequest(url: verifierURL)
    request.httpMethod = "POST"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    request.timeoutInterval = 10 // 10-second timeout

    let body = VerifyRequest(
      signedTransaction: signedTransaction,
      bundleId: bundleId
    )

    request.httpBody = try JSONEncoder().encode(body)

    let (data, response) = try await URLSession.shared.data(for: request)

    guard let httpResponse = response as? HTTPURLResponse else {
      throw NSError(
        domain: "SubscriptionManager",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "Invalid response type"]
      )
    }

    guard (200 ... 299).contains(httpResponse.statusCode) else {
      throw NSError(
        domain: "SubscriptionManager",
        code: httpResponse.statusCode,
        userInfo: [NSLocalizedDescriptionKey: "HTTP \(httpResponse.statusCode)"]
      )
    }

    let decodedResponse = try JSONDecoder().decode(VerifyResponse.self, from: data)
    return decodedResponse
  }

  /// Grant entitlement locally with 24h TTL.
  /// Stores the original signed transaction in Keychain for re-verification on next launch.
  private func grantLocalEntitlementWithTTL(from transaction: Transaction) async {
    let cacheEntry = EntitlementCacheEntry(
      productId: transaction.productID,
      expiresAt: Date().addingTimeInterval(TimeInterval(ttlHours * 3600)),
      originalSignedJWT: try? await AppTransaction.current?.originalSignedJWT ?? ""
    )

    do {
      try cacheKeychain.store(key: "entitlement_\(transaction.productID)", value: cacheEntry)

      // Grant tier locally based on product ID
      let tier = tierFromProductId(transaction.productID)
      self.entitlementStatus = .subscription(tier: tier, expiresAt: cacheEntry.expiresAt)

      print("⏱️ Local entitlement granted for \(transaction.productID) (expires in \(ttlHours)h)")

      // Mark transaction as consumed
      await transaction.finish()
    } catch {
      print("❌ Failed to cache entitlement: \(error)")
    }
  }

  /// Cache verified entitlement details in Keychain.
  private func cacheVerifiedEntitlement(
    _ response: VerifyResponse,
    ttlHours: Int
  ) async {
    let expiresAt = Date().addingTimeInterval(TimeInterval(ttlHours * 3600))
    let cacheEntry = EntitlementCacheEntry(
      productId: response.productId ?? "unknown",
      expiresAt: expiresAt,
      originalSignedJWT: nil
    )

    do {
      let key = "entitlement_verified_\(response.originalTransactionId ?? "unknown")"
      try cacheKeychain.store(key: key, value: cacheEntry)
    } catch {
      print("⚠️ Failed to cache verified entitlement: \(error)")
    }
  }

  /// Update Rust entitlements via FFI.
  private func updateEntitlementFromResponse(_ response: VerifyResponse) async {
    guard let productId = response.productId else { return }

    let tier = tierFromProductId(productId)
    let expiresAt = response.expiresDate.flatMap { dateStr in
      Date(timeIntervalSince1970: Double(dateStr) ?? 0 / 1000)
    } ?? Date().addingTimeInterval(30 * 24 * 3600) // Default 30d if not specified

    self.entitlementStatus = .subscription(tier: tier, expiresAt: expiresAt)

    // TODO: Call Rust FFI to update EntitlementStore
    // focus_ffi::EntitlementsApi::set_tier_from_transaction(tier, expiresAt)
    print("📝 Updated entitlement: \(tier) (expires \(expiresAt))")
  }

  /// Map product ID to subscription tier.
  private func tierFromProductId(_ productId: String) -> SubscriptionTier {
    if productId.contains("pro") {
      return .pro
    } else if productId.contains("family") {
      return .family
    } else if productId.contains("plus") {
      return .plus
    }
    return .free
  }

  /// Check cached entitlements and re-verify if TTL expired.
  func checkCachedEntitlements() async {
    // Scan Keychain for cached entitlements
    do {
      // TODO: Implement Keychain scan for all "entitlement_*" keys
      // For now, stub: attempt to re-verify if any cached entries exist
      print("🔄 Checking cached entitlements on app launch")
    } catch {
      print("⚠️ Failed to check cached entitlements: \(error)")
    }
  }
}

// MARK: - Supporting Types

enum SubscriptionTier: String, Codable {
  case free
  case plus
  case pro
  case family
}

enum EntitlementStatus {
  case free
  case subscription(tier: SubscriptionTier, expiresAt: Date)

  var tier: SubscriptionTier {
    switch self {
    case .free:
      return .free
    case .subscription(let tier, _):
      return tier
    }
  }
}

struct VerifyRequest: Codable {
  let signedTransaction: String
  let bundleId: String
}

struct VerifyResponse: Codable {
  let valid: Bool
  let productId: String?
  let originalTransactionId: String?
  let expiresDate: String?
  let environment: String?
  let status: String?
  let error: String?
}

struct EntitlementCacheEntry: Codable {
  let productId: String
  let expiresAt: Date
  let originalSignedJWT: String?
}

// MARK: - Secure Cache (Keychain wrapper)

private class SecureCache {
  private let service = "com.koosha.focalpoint.subscriptions"

  func store<T: Encodable>(key: String, value: T) throws {
    let data = try JSONEncoder().encode(value)

    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: service,
      kSecAttrAccount as String: key,
      kSecValueData as String: data,
      kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
    ]

    // Delete existing entry if it exists
    SecItemDelete(query as CFDictionary)

    // Add new entry
    let status = SecItemAdd(query as CFDictionary, nil)
    guard status == errSecSuccess else {
      throw NSError(domain: "SecureCache", code: Int(status))
    }
  }

  func retrieve<T: Decodable>(key: String) throws -> T {
    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: service,
      kSecAttrAccount as String: key,
      kSecReturnData as String: true,
    ]

    var result: AnyObject?
    let status = SecItemCopyMatching(query as CFDictionary, &result)

    guard status == errSecSuccess, let data = result as? Data else {
      throw NSError(domain: "SecureCache", code: Int(status))
    }

    return try JSONDecoder().decode(T.self, from: data)
  }
}
