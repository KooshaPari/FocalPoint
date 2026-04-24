import Foundation
import CloudKit
import os

/// CloudKit-based multi-device sync client.
///
/// Wraps CKContainer.default().privateCloudDatabase to push/pull sync records
/// (Rules, Tasks, Wallet, Penalties) across iOS/iPadOS/macOS devices.
///
/// Sync is opt-in (default OFF); user enables in Settings.
/// All records are E2E encrypted in the user's private CloudKit database.
actor CloudKitSyncClient {
    private let container: CKContainer
    private let database: CKDatabase

    /// Represents the current sync availability status.
    enum SyncStatus {
        case unavailable(reason: String)
        case available
    }

    init() {
        self.container = CKContainer.default()
        self.database = container.privateCloudDatabase
    }

    /// Check if this device can sync (iCloud account is signed in).
    ///
    /// Returns `nil` if checking, `.unavailable` if iCloud is not available,
    /// or `.available` if ready to sync.
    func checkSyncStatus() async -> SyncStatus {
        do {
            let status = try await container.accountStatus()
            switch status {
            case .available:
                return .available
            case .restricted:
                return .unavailable(reason: "iCloud account is restricted")
            case .noAccount:
                return .unavailable(reason: "No iCloud account signed in")
            case .couldNotDetermine:
                return .unavailable(reason: "Could not determine iCloud status")
            @unknown default:
                return .unavailable(reason: "Unknown iCloud status")
            }
        } catch {
            return .unavailable(reason: "Error checking iCloud status: \(error.localizedDescription)")
        }
    }

    /// Stub: Push local records to CloudKit.
    ///
    /// Real implementation:
    /// 1. Canonicalize record JSON.
    /// 2. Sign with device private key (stored in Keychain).
    /// 3. Create CKRecord with signed payload + device_id.
    /// 4. Batch save to privateCloudDatabase.
    /// 5. Append SyncPush audit record.
    ///
    /// For now, this logs intent and returns success.
    func push(records: [SyncRecord]) async -> Result<Int, SyncError> {
        os_log("CloudKitSync: push() called with %d records (stub)", type: .info, records.count)
        // TODO: Implement real CloudKit push.
        return .success(records.count)
    }

    /// Stub: Pull remote changes from CloudKit.
    ///
    /// Real implementation:
    /// 1. Query CKRecords with modificationDate > last_sync_ts.
    /// 2. For each record, verify device signature.
    /// 3. If verification fails, log tamper alert and skip.
    /// 4. Compare versions; if >30s newer on server, add to conflicts.
    /// 5. Apply non-conflicting records to local SQLite.
    /// 6. Append SyncPull audit record.
    /// 7. Return pulled records and conflicts.
    ///
    /// For now, returns empty result.
    func pull() async -> Result<(pulled: [SyncRecord], conflicts: [ConflictRecord]), SyncError> {
        os_log("CloudKitSync: pull() called (stub)", type: .info)
        // TODO: Implement real CloudKit pull.
        return .success((pulled: [], conflicts: []))
    }

    /// Stub: Verify a device signature on a record.
    ///
    /// Real implementation:
    /// 1. Fetch device's public key from CloudKit.
    /// 2. Verify Ed25519 signature.
    /// 3. Return true if valid, false if tampered.
    ///
    /// For now, always returns true (testing mode).
    func verifySignature(
        deviceId: UUID,
        payloadJson: Data,
        signature: String
    ) async -> Result<Bool, SyncError> {
        os_log("CloudKitSync: verifySignature() called for device %@", type: .debug, deviceId.uuidString)
        // TODO: Implement real Ed25519 verification against device public key.
        return .success(true)
    }

    /// Perform a full sync round: push local changes, pull remote changes.
    ///
    /// Returns the outcome: number pushed, number pulled, conflicts.
    func syncRound() async -> Result<SyncOutcome, SyncError> {
        os_log("CloudKitSync: syncRound() called", type: .info)

        // Check iCloud status first.
        let status = await checkSyncStatus()
        if case .unavailable(let reason) = status {
            return .failure(.cloudKitUnavailable(reason))
        }

        // Stub: for now, return a zero-change outcome.
        return .success(SyncOutcome(
            pushed: 0,
            pulled: 0,
            conflicts: [],
            status: .ok
        ))
    }
}

/// A sync record to be pushed or pulled.
struct SyncRecord: Codable {
    let recordId: UUID
    let recordType: String // "Rule", "Task", "Wallet", "Penalty"
    let deviceId: UUID
    let payloadJson: Data // Canonical JSON bytes
    let deviceSignature: String // Ed25519(payloadJson, device_private_key)
    let version: UInt64
    let syncedAt: Date
}

/// A conflicting record: local and remote versions differ.
struct ConflictRecord: Codable {
    let recordId: UUID
    let recordType: String
    let localVersion: UInt64
    let remoteVersion: UInt64
    let localModifiedAt: Date
    let remoteModifiedAt: Date
    let resolution: ConflictResolution
}

enum ConflictResolution: String, Codable {
    case pendingUser = "pending_user"
    case localWins = "local_wins"
    case remoteWins = "remote_wins"
}

/// Result of a sync round.
struct SyncOutcome: Codable {
    let pushed: Int
    let pulled: Int
    let conflicts: [ConflictRecord]
    let status: SyncStatus

    enum SyncStatus: String, Codable {
        case ok
        case networkError = "network_error"
        case verificationFailed = "verification_failed"
        case partial
    }
}

/// Sync-related errors.
enum SyncError: LocalizedError {
    case cloudKitUnavailable(String)
    case pushFailed(String)
    case pullFailed(String)
    case signatureVerificationFailed
    case networkError(Error)
    case unknown(String)

    var errorDescription: String? {
        switch self {
        case .cloudKitUnavailable(let reason):
            return "iCloud is unavailable: \(reason)"
        case .pushFailed(let reason):
            return "Failed to push to CloudKit: \(reason)"
        case .pullFailed(let reason):
            return "Failed to pull from CloudKit: \(reason)"
        case .signatureVerificationFailed:
            return "Record signature verification failed (possible tampering)"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        case .unknown(let reason):
            return "Unknown sync error: \(reason)"
        }
    }
}
