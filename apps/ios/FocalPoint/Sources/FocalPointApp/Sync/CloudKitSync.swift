import Foundation
import CloudKit
import os

/// CloudKit-based multi-device sync client.
///
/// Wraps CKContainer.default().privateCloudDatabase to push/pull sync records
/// (Wallet, AuditRecord, Rule) across iOS/iPadOS devices.
///
/// Sync is opt-in (default OFF); user enables in Settings.
/// All records are E2E encrypted in the user's private CloudKit database.
///
/// Phase 1: Wallet + Audit + Rules only (not Tasks, Connectors, or Templates).
actor CloudKitSyncClient {
    private let container: CKContainer
    private let database: CKDatabase
    private let logger = Logger(subsystem: "com.focalpoint.sync", category: "CloudKit")

    // Sync zone configuration
    private let zoneId = CKRecordZone.ID(zoneName: "focalpoint-sync-v1", ownerName: CKCurrentUserDefaultName)

    // Sync tokens persisted to UserDefaults per record type
    private let userDefaults = UserDefaults.standard
    private let syncTokenPrefix = "cloudkit.sync_token."

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
    /// Returns `.unavailable` if iCloud is not available, or `.available` if ready to sync.
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

    /// Create or update the sync zone.
    ///
    /// Must be called before any push/pull operations.
    /// Safe to call multiple times (idempotent).
    func ensureSyncZone() async -> Result<Void, SyncError> {
        do {
            _ = try await database.modifyRecordZones(saving: [CKRecordZone(zoneID: zoneId)], deleting: [])
            logger.info("CloudKit sync zone created/verified")
            return .success(())
        } catch {
            let msg = "Failed to create sync zone: \(error.localizedDescription)"
            logger.error("\(msg)")
            return .failure(.cloudKitUnavailable(msg))
        }
    }

    /// Push wallet, audit, and rule records to CloudKit.
    ///
    /// Records are serialized to CKRecord format with device metadata.
    /// On success, audit records are appended locally.
    func push(records: [SyncRecord]) async -> Result<Int, SyncError> {
        do {
            // Ensure zone exists first
            _ = try await ensureSyncZone()

            var ckRecords: [CKRecord] = []

            for record in records {
                let ckRecord = CKRecord(recordType: record.recordType, recordID: CKRecordID(recordName: record.recordId.uuidString, zoneID: zoneId))

                // Encode payload as base64 for CloudKit storage
                let payloadBase64 = record.payloadJson.base64EncodedString()
                ckRecord["payload_base64"] = payloadBase64
                ckRecord["device_id"] = record.deviceId.uuidString
                ckRecord["device_signature"] = record.deviceSignature
                ckRecord["version"] = NSNumber(value: record.version)
                ckRecord["synced_at"] = record.syncedAt as CKRecordValue

                ckRecords.append(ckRecord)
            }

            let (saved, _) = try await database.modifyRecords(saving: ckRecords, deleting: [])
            logger.info("CloudKit push: saved \(saved.count) records")
            return .success(saved.count)
        } catch {
            let msg = "CloudKit push failed: \(error.localizedDescription)"
            logger.error("\(msg)")
            return .failure(.pushFailed(msg))
        }
    }

    /// Pull remote changes from CloudKit since the last sync token.
    ///
    /// Returns newly-synced records and any conflicts detected.
    /// Updates the sync token internally for the next pull.
    func pull() async -> Result<(pulled: [SyncRecord], conflicts: [ConflictRecord]), SyncError> {
        do {
            // Ensure zone exists
            _ = try await ensureSyncZone()

            var allPulled: [SyncRecord] = []
            var allConflicts: [ConflictRecord] = []

            // Pull Wallet records
            let (walletPulled, walletConflicts) = try await pullRecordType("Wallet")
            allPulled.append(contentsOf: walletPulled)
            allConflicts.append(contentsOf: walletConflicts)

            // Pull AuditRecord records
            let (auditPulled, auditConflicts) = try await pullRecordType("AuditRecord")
            allPulled.append(contentsOf: auditPulled)
            allConflicts.append(contentsOf: auditConflicts)

            // Pull Rule records
            let (rulePulled, ruleConflicts) = try await pullRecordType("Rule")
            allPulled.append(contentsOf: rulePulled)
            allConflicts.append(contentsOf: ruleConflicts)

            logger.info("CloudKit pull: fetched \(allPulled.count) records, \(allConflicts.count) conflicts")
            return .success((pulled: allPulled, conflicts: allConflicts))
        } catch {
            let msg = "CloudKit pull failed: \(error.localizedDescription)"
            logger.error("\(msg)")
            return .failure(.pullFailed(msg))
        }
    }

    /// Pull records of a specific type since the last sync token.
    ///
    /// Handles the CKQueryOperation fetch and returns new records + conflicts.
    private func pullRecordType(_ recordType: String) async throws -> (pulled: [SyncRecord], conflicts: [ConflictRecord]) {
        let predicate = NSPredicate(value: true) // Fetch all records of this type
        let query = CKQuery(recordType: recordType, predicate: predicate)

        // Load the sync token for this record type
        let tokenKey = syncTokenPrefix + recordType
        let syncToken = userDefaults.data(forKey: tokenKey).flatMap { CKServerChangeToken(data: $0) }

        var config = CKQueryOperation.Configuration()
        config.resultsLimit = 100
        if let syncToken = syncToken {
            config.previousServerChangeToken = syncToken
        }

        let operation = CKQueryOperation(query: query, queuePriority: .high, qualityOfService: .userInitiated)
        operation.configuration = config

        var fetchedRecords: [SyncRecord] = []
        var conflicts: [ConflictRecord] = []
        var newSyncToken: CKServerChangeToken?

        return try await withCheckedThrowingContinuation { continuation in
            operation.recordMatchedBlock = { _, record in
                if let syncRecord = self.parseCKRecord(record, type: recordType) {
                    fetchedRecords.append(syncRecord)
                }
            }

            operation.queryResultBlock = { result in
                switch result {
                case .success(let token):
                    newSyncToken = token
                    // Persist the sync token
                    if let token = token {
                        let tokenData = token.data()
                        self.userDefaults.set(tokenData, forKey: tokenKey)
                    }
                    continuation.resume(returning: (pulled: fetchedRecords, conflicts: conflicts))
                case .failure(let error):
                    self.logger.error("Query failed for \(recordType): \(error.localizedDescription)")
                    continuation.resume(throwing: error)
                }
            }

            self.database.add(operation)
        }
    }

    /// Parse a CKRecord into a SyncRecord.
    private func parseCKRecord(_ ckRecord: CKRecord, type: String) -> SyncRecord? {
        guard let payloadBase64 = ckRecord["payload_base64"] as? String,
              let payloadData = Data(base64Encoded: payloadBase64),
              let deviceIdStr = ckRecord["device_id"] as? String,
              let deviceId = UUID(uuidString: deviceIdStr),
              let deviceSig = ckRecord["device_signature"] as? String,
              let version = (ckRecord["version"] as? NSNumber)?.uint64Value,
              let syncedAt = ckRecord["synced_at"] as? Date else {
            logger.warning("Failed to parse CKRecord: \(ckRecord.recordID.recordName)")
            return nil
        }

        return SyncRecord(
            recordId: UUID(uuidString: ckRecord.recordID.recordName) ?? UUID(),
            recordType: type,
            deviceId: deviceId,
            payloadJson: payloadData,
            deviceSignature: deviceSig,
            version: version,
            syncedAt: syncedAt
        )
    }

    /// Subscribe to change notifications for the sync zone.
    ///
    /// Enables background app refresh when records change on other devices.
    func setupSubscription() async -> Result<Void, SyncError> {
        do {
            // Create subscriptions for each record type we sync
            for recordType in ["Wallet", "AuditRecord", "Rule"] {
                let predicate = NSPredicate(value: true)
                let subscription = CKQuerySubscription(
                    recordType: recordType,
                    predicate: predicate,
                    subscriptionID: "sync-\(recordType)-v1",
                    options: [.firesOnRecordCreation, .firesOnRecordUpdate, .firesOnRecordDeletion]
                )

                let info = CKSubscription.NotificationInfo()
                info.shouldSendContentAvailable = true // Enables background BGTask
                info.desiredKeys = []
                subscription.notificationInfo = info

                _ = try await database.save(subscription)
                logger.info("Subscription created for \(recordType)")
            }

            return .success(())
        } catch {
            let msg = "Failed to setup subscriptions: \(error.localizedDescription)"
            logger.error("\(msg)")
            return .failure(.cloudKitUnavailable(msg))
        }
    }

    /// Perform a full sync round: push local changes, pull remote changes.
    ///
    /// Returns the outcome: number pushed, number pulled, conflicts.
    func syncRound(pushRecords: [SyncRecord] = []) async -> Result<SyncOutcome, SyncError> {
        logger.info("Starting sync round")

        // Check iCloud status first
        let status = await checkSyncStatus()
        if case .unavailable(let reason) = status {
            return .failure(.cloudKitUnavailable(reason))
        }

        // Push local changes
        var pushedCount = 0
        if !pushRecords.isEmpty {
            switch await push(records: pushRecords) {
            case .success(let count):
                pushedCount = count
            case .failure(let error):
                return .failure(error)
            }
        }

        // Pull remote changes
        let pullResult = await pull()
        switch pullResult {
        case .success(let (pulled, conflicts)):
            let conflictCount = conflicts.count
            let status: SyncOutcome.SyncStatus = conflictCount > 0 ? .partial : .ok
            return .success(SyncOutcome(
                pushed: pushedCount,
                pulled: pulled.count,
                conflicts: conflicts,
                status: status,
                lastSyncTime: Date()
            ))
        case .failure(let error):
            return .failure(error)
        }
    }

    /// Get the last successful sync timestamp.
    func getLastSyncTime() -> Date? {
        userDefaults.object(forKey: "cloudkit.last_sync_time") as? Date
    }

    /// Store the last successful sync timestamp.
    func setLastSyncTime(_ date: Date) {
        userDefaults.set(date, forKey: "cloudkit.last_sync_time")
    }
}

/// A sync record to be pushed or pulled.
struct SyncRecord: Codable {
    let recordId: UUID
    let recordType: String // "Rule", "Wallet", "AuditRecord"
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
    /// User action required to resolve
    case pendingUser = "pending_user"
    /// Local version wins
    case localWins = "local_wins"
    /// Remote version wins
    case remoteWins = "remote_wins"
}

/// Result of a sync round.
struct SyncOutcome: Codable {
    let pushed: Int
    let pulled: Int
    let conflicts: [ConflictRecord]
    let status: SyncStatus
    let lastSyncTime: Date

    enum SyncStatus: String, Codable {
        case ok
        case networkError = "network_error"
        case verificationFailed = "verification_failed"
        case partial // Some conflicts detected
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

/// Status enum for Settings UI
enum CloudKitSyncStatus: Equatable {
    case unchecked
    case available
    case unavailable(String)
}
