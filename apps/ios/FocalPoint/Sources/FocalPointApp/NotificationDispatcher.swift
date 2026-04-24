#if canImport(SwiftUI)
import Foundation
import UserNotifications
import FocalPointCore

/// Tails the audit chain, finds `notify.dispatched` records that haven't
/// yet been presented to the user, and schedules one `UNNotificationContent`
/// per record. Deduped by `AuditRecord.id` stored in `UserDefaults` so the
/// same record never fires twice across app launches.
@MainActor
public final class NotificationDispatcher {
    public static let shared = NotificationDispatcher()
    private let udKey = "focalpoint.notify.last_dispatched_ids"
    private let maxMemoryIds = 500

    private init() {}

    private var seen: Set<String> {
        get {
            let arr = UserDefaults.standard.stringArray(forKey: udKey) ?? []
            return Set(arr)
        }
        set {
            // Cap the set so UserDefaults doesn't grow forever.
            var trimmed = Array(newValue)
            if trimmed.count > maxMemoryIds {
                trimmed = Array(trimmed.suffix(maxMemoryIds))
            }
            UserDefaults.standard.set(trimmed, forKey: udKey)
        }
    }

    /// Pulls the recent audit tail, dispatches any new `notify.dispatched`
    /// records as local notifications. Also triggers rule-fired fly-ins.
    /// Safe to call repeatedly — dedup via AuditRecord.id ensures each record fires at most once.
    public func tick(core: FocalPointCore, flyInsEnabled: Bool = true) {
        guard let records = try? core.audit().recent(limit: 50) else { return }
        var seen = self.seen
        for rec in records where rec.recordType == "notify.dispatched" && !seen.contains(rec.id) {
            dispatch(rec)
            seen.insert(rec.id)
        }
        self.seen = seen

        // Trigger rule-fired fly-in presenter
        RuleFiredFlyInPresenter.shared.tick(core: core, flyInsEnabled: flyInsEnabled)
    }

    private func dispatch(_ rec: AuditRecordDto) {
        let message = parseMessage(rec.payloadJson) ?? "Coachy has a nudge for you."
        let content = UNMutableNotificationContent()
        content.title = "FocalPoint"
        content.body = message
        content.sound = .default
        let req = UNNotificationRequest(
            identifier: "focalpoint.notify.\(rec.id)",
            content: content,
            trigger: nil // fire immediately
        )
        UNUserNotificationCenter.current().add(req) { err in
            if let err { print("[FocalPoint] notify dispatch failed: \(err)") }
        }
    }

    private func parseMessage(_ json: String) -> String? {
        guard let data = json.data(using: .utf8),
              let obj = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let msg = obj["message"] as? String else {
            return nil
        }
        return msg
    }
}
#endif
