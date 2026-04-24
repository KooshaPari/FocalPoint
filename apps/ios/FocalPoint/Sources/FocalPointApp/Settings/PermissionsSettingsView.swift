#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

/// Permissions management view in Settings. Shows current status of Notifications,
/// Calendar, HealthKit, and FamilyControls, with buttons to grant/revoke or open
/// system Settings.
public struct PermissionsSettingsView: View {
    @StateObject private var coord = OnboardingCoordinator()
    @Environment(\.dismiss) var dismiss
    @State private var requestingNotifications = false
    @State private var requestingCalendar = false
    @State private var requestingHealthKit = false
    @State private var requestingFamilyControls = false

    public init() {}

    public var body: some View {
        NavigationStack {
            Form {
                Section("App Permissions") {
                    PermissionRow(
                        icon: "bell.fill",
                        title: "Notifications",
                        description: "Nudges, reminders, and rule alerts",
                        status: coord.notificationsStatus,
                        isRequesting: requestingNotifications,
                        onRequest: {
                            requestingNotifications = true
                            Task {
                                await coord.requestNotificationsPermission()
                                requestingNotifications = false
                            }
                        },
                        onOpenSettings: { openSettings() }
                    )

                    PermissionRow(
                        icon: "calendar.circle.fill",
                        title: "Calendar (EventKit)",
                        description: "Sync your events to avoid conflicts",
                        status: coord.calendarStatus,
                        isRequesting: requestingCalendar,
                        onRequest: {
                            requestingCalendar = true
                            Task {
                                await coord.requestCalendarPermission()
                                requestingCalendar = false
                            }
                        },
                        onOpenSettings: { openSettings() }
                    )

                    PermissionRow(
                        icon: "heart.fill",
                        title: "HealthKit",
                        description: "Track activity and sleep data (optional)",
                        status: .notDetermined,
                        isRequesting: requestingHealthKit,
                        onRequest: {
                            // HealthKit request deferred in v0.1
                        },
                        onOpenSettings: { openSettings() }
                    )

                    PermissionRow(
                        icon: "shield.fill",
                        title: "Family Controls",
                        description: "Enforce app limits and schedules",
                        status: coord.familyControlsStatus,
                        isRequesting: requestingFamilyControls,
                        onRequest: {
                            requestingFamilyControls = true
                            Task {
                                await coord.requestFamilyControlsPermission()
                                requestingFamilyControls = false
                            }
                        },
                        onOpenSettings: { openSettings() }
                    )
                }

                Section("About Permissions") {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("FocalPoint needs permissions to:")
                            .font(.caption)
                            .fontWeight(.semibold)
                            .foregroundStyle(Color.app.foreground)

                        BulletPoint(text: "Send nudges, reminders, and alerts")
                        BulletPoint(text: "Read your calendar to avoid scheduling conflicts")
                        BulletPoint(text: "Monitor app usage and enforce focus rules")
                        BulletPoint(text: "Sync data with your other Apple devices (if enabled)")
                    }
                    .padding(.vertical, 8)
                }
            }
            .navigationTitle("Permissions")
            .navigationBarTitleDisplayMode(.inline)
            .task {
                // Refresh on appear
                await coord.refreshNotificationStatus()
            }
        }
    }

    private func openSettings() {
        if let settingsURL = URL(string: UIApplication.openSettingsURLString) {
            UIApplication.shared.open(settingsURL)
        }
    }
}

/// A single permission row in the Settings Permissions view. Renders the status,
/// and buttons to grant or open Settings.
struct PermissionRow: View {
    let icon: String
    let title: String
    let description: String
    let status: OnboardingCoordinator.PermissionStatus
    let isRequesting: Bool
    let onRequest: () -> Void
    let onOpenSettings: () -> Void

    var body: some View {
        VStack(spacing: 12) {
            HStack(spacing: 12) {
                Image(systemName: icon)
                    .font(.system(size: 20))
                    .foregroundStyle(statusColor)
                    .frame(width: 32, height: 32)

                VStack(alignment: .leading, spacing: 2) {
                    Text(title)
                        .font(.body.weight(.semibold))
                        .foregroundStyle(Color.app.foreground)
                    Text(description)
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.6))
                }

                Spacer()

                statusLabel
            }

            if status != .granted {
                if status == .denied {
                    Button(action: onOpenSettings) {
                        HStack(spacing: 6) {
                            Image(systemName: "gear")
                            Text("Open Settings")
                        }
                        .frame(maxWidth: .infinity)
                        .padding(10)
                        .background(Color.red.opacity(0.1))
                        .foregroundStyle(Color.red)
                        .cornerRadius(8)
                    }
                } else if status == .notDetermined {
                    Button(action: onRequest) {
                        HStack(spacing: 6) {
                            if isRequesting {
                                ProgressView()
                                    .scaleEffect(0.8)
                            } else {
                                Image(systemName: "checkmark.circle.fill")
                            }
                            Text("Grant Permission")
                        }
                        .frame(maxWidth: .infinity)
                        .padding(10)
                        .background(Color.app.accent)
                        .foregroundStyle(.white)
                        .cornerRadius(8)
                    }
                    .disabled(isRequesting)
                } else if status == .pendingEntitlement {
                    Text("Pending Apple approval — try later")
                        .font(.caption)
                        .foregroundStyle(Color.orange)
                        .frame(maxWidth: .infinity, alignment: .center)
                        .padding(10)
                        .background(Color.orange.opacity(0.1))
                        .cornerRadius(8)
                }
            }
        }
    }

    private var statusColor: Color {
        switch status {
        case .granted: return Color.green
        case .denied: return Color.red
        case .notDetermined: return Color.app.foreground.opacity(0.5)
        case .pendingEntitlement: return Color.orange
        }
    }

    @ViewBuilder
    private var statusLabel: some View {
        switch status {
        case .granted:
            Image(systemName: "checkmark.circle.fill")
                .foregroundStyle(Color.green)
                .font(.system(size: 18))

        case .denied:
            Image(systemName: "xmark.circle.fill")
                .foregroundStyle(Color.red)
                .font(.system(size: 18))

        case .notDetermined:
            Text("Not determined")
                .font(.caption2)
                .foregroundStyle(.secondary)

        case .pendingEntitlement:
            Text("Pending")
                .font(.caption2.weight(.semibold))
                .foregroundStyle(Color.orange)
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(Color.orange.opacity(0.2))
                .cornerRadius(4)
        }
    }
}

/// A bullet point for the About Permissions section.
struct BulletPoint: View {
    let text: String

    var body: some View {
        HStack(alignment: .top, spacing: 8) {
            Text("•")
                .foregroundStyle(Color.app.accent)
            Text(text)
                .font(.caption)
                .foregroundStyle(Color.app.foreground)
        }
    }
}

#endif
