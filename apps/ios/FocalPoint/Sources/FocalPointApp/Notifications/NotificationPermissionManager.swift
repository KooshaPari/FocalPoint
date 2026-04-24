#if canImport(SwiftUI)
import Foundation
import UserNotifications
import SwiftUI

/// Manages iOS notification permissions and category registration.
/// Integrates with onboarding step 2 via @AppStorage observable.
@MainActor
public final class NotificationPermissionManager: NSObject, ObservableObject {
    public static let shared = NotificationPermissionManager()

    @AppStorage("app.notificationStatus")
    private var notificationStatus: String = "notDetermined"

    @Published public private(set) var authStatus: UNAuthorizationStatus = .notDetermined

    private override init() {
        super.init()
        checkAuthorizationStatus()
        registerNotificationCategories()
    }

    /// Requests notification permissions (alert + sound + badge + criticalAlert if entitled).
    /// Returns true if permission was granted (or already granted).
    public func requestNotificationPermissions() async -> Bool {
        var options: UNAuthorizationOptions = [.alert, .sound, .badge, .providesAppNotificationSettings]

        // Request critical alerts if the app has the entitlement.
        // This requires Apple-approved capability; fails gracefully if not present.
        options.insert(.criticalAlert)

        do {
            let granted = try await UNUserNotificationCenter.current().requestAuthorization(options: options)

            DispatchQueue.main.async {
                self.authStatus = granted ? .authorized : .denied
                self.notificationStatus = granted ? "granted" : "denied"
            }

            return granted
        } catch {
            print("[FocalPoint] requestAuthorization failed: \(error)")
            DispatchQueue.main.async {
                self.authStatus = .denied
                self.notificationStatus = "denied"
            }
            return false
        }
    }

    /// Checks current notification authorization status.
    public func checkAuthorizationStatus() {
        let center = UNUserNotificationCenter.current()
        center.getNotificationSettings { settings in
            DispatchQueue.main.async {
                self.authStatus = settings.authorizationStatus
                self.notificationStatus = self.statusString(settings.authorizationStatus)
            }
        }
    }

    /// Registers the 4 notification action categories.
    private func registerNotificationCategories() {
        let categories: Set<UNNotificationCategory> = [
            makeCoachyNudgeCategory(),
            makeRitualReminderCategory(),
            makeRuleFireCategory(),
            makeBackupCompleteCategory(),
        ]

        UNUserNotificationCenter.current().setNotificationCategories(categories)
    }

    // MARK: - Category Definitions

    /// COACHY_NUDGE — Snooze 10m / Dismiss
    private func makeCoachyNudgeCategory() -> UNNotificationCategory {
        let snoozeAction = UNNotificationAction(
            identifier: "COACHY_SNOOZE_10M",
            title: "Snooze 10m",
            options: [.foreground]
        )
        let dismissAction = UNNotificationAction(
            identifier: "COACHY_DISMISS",
            title: "Dismiss",
            options: [.destructive]
        )
        return UNNotificationCategory(
            identifier: "COACHY_NUDGE",
            actions: [snoozeAction, dismissAction],
            intentIdentifiers: [],
            hiddenPreviewsBodyPlaceholder: "Coachy has a reminder",
            options: [.customDismissAction]
        )
    }

    /// RITUAL_REMINDER — Do Now / Skip Today
    private func makeRitualReminderCategory() -> UNNotificationCategory {
        let doNowAction = UNNotificationAction(
            identifier: "RITUAL_DO_NOW",
            title: "Do Now",
            options: [.foreground]
        )
        let skipAction = UNNotificationAction(
            identifier: "RITUAL_SKIP_TODAY",
            title: "Skip Today",
            options: []
        )
        return UNNotificationCategory(
            identifier: "RITUAL_REMINDER",
            actions: [doNowAction, skipAction],
            intentIdentifiers: [],
            hiddenPreviewsBodyPlaceholder: "Time for your ritual",
            options: [.customDismissAction]
        )
    }

    /// RULE_FIRED — View / Dismiss
    private func makeRuleFireCategory() -> UNNotificationCategory {
        let viewAction = UNNotificationAction(
            identifier: "RULE_FIRED_VIEW",
            title: "View",
            options: [.foreground]
        )
        let dismissAction = UNNotificationAction(
            identifier: "RULE_FIRED_DISMISS",
            title: "Dismiss",
            options: [.destructive]
        )
        return UNNotificationCategory(
            identifier: "RULE_FIRED",
            actions: [viewAction, dismissAction],
            intentIdentifiers: [],
            hiddenPreviewsBodyPlaceholder: "A rule has fired",
            options: [.customDismissAction]
        )
    }

    /// BACKUP_COMPLETE — View Details / Dismiss
    private func makeBackupCompleteCategory() -> UNNotificationCategory {
        let viewAction = UNNotificationAction(
            identifier: "BACKUP_VIEW_DETAILS",
            title: "View Details",
            options: [.foreground]
        )
        let dismissAction = UNNotificationAction(
            identifier: "BACKUP_DISMISS",
            title: "Dismiss",
            options: []
        )
        return UNNotificationCategory(
            identifier: "BACKUP_COMPLETE",
            actions: [viewAction, dismissAction],
            intentIdentifiers: [],
            hiddenPreviewsBodyPlaceholder: "Backup completed",
            options: [.customDismissAction]
        )
    }

    // MARK: - Helpers

    private func statusString(_ status: UNAuthorizationStatus) -> String {
        switch status {
        case .notDetermined:
            return "notDetermined"
        case .denied:
            return "denied"
        case .authorized:
            return "granted"
        case .provisional:
            return "provisional"
        case .ephemeral:
            return "ephemeral"
        @unknown default:
            return "unknown"
        }
    }

    /// Opens the Settings app at the Notifications section for this app.
    public func openSettings() {
        guard let settingsURL = URL(string: UIApplication.openSettingsURLScheme + "://notification") else {
            return
        }
        if UIApplication.shared.canOpenURL(settingsURL) {
            UIApplication.shared.open(settingsURL)
        }
    }
}

// MARK: - UNUserNotificationCenterDelegate

@MainActor
extension NotificationPermissionManager: UNUserNotificationCenterDelegate {
    /// Handles foreground notification reception.
    public func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        let category = notification.request.content.categoryIdentifier
        print("[FocalPoint] Foreground notification received: category=\(category)")

        // Show alert + badge + sound even in foreground
        completionHandler([.banner, .badge, .sound])
    }

    /// Handles user interaction with notification actions.
    public func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        let actionId = response.actionIdentifier
        let category = response.notification.request.content.categoryIdentifier
        let notificationId = response.notification.request.identifier

        print("[FocalPoint] User action: category=\(category), action=\(actionId)")

        // Emit audit record via HostEventApi so Rust core can track user response
        Task {
            await emitAuditRecord(
                notificationId: notificationId,
                category: category,
                actionId: actionId
            )
        }

        completionHandler()
    }

    /// Emits an audit record for the user's notification action.
    private func emitAuditRecord(notificationId: String, category: String, actionId: String) async {
        // TODO: Integrate with HostEventApi to emit audit records.
        // Example structure:
        //   AuditRecord {
        //     recordType: "notify.user_action",
        //     payload: {
        //       "notification_id": notificationId,
        //       "category": category,
        //       "action_id": actionId,
        //       "timestamp": ISO8601
        //     }
        //   }
        // For now, log locally.
        print("[FocalPoint] Would emit audit: category=\(category), action=\(actionId)")
    }
}
#endif
