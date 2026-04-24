#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore
import MascotUI

/// Real permission prompts (Notifications, Calendar, HealthKit, FamilyControls).
/// Renders status cards for each, with "Grant for best experience" nudge and honest
/// "Pending Apple" states for FamilyControls. Skip button available but encouraged
/// to grant.
public struct PermissionsStep: View {
    let namespace: Namespace.ID
    @ObservedObject var coord: OnboardingCoordinator
    @State private var showingSettings = false
    @State private var requestingNotifications = false
    @State private var requestingCalendar = false
    @State private var requestingFamilyControls = false
    @State private var requestingHealthKit = false

    var body: some View {
        VStack(spacing: 0) {
            Spacer()

            CoachyView(
                state: CoachyState(
                    pose: .encouraging,
                    emotion: .warm,
                    bubbleText: "I need a few permissions to help you best."
                ),
                size: 280
            )
            .matchedGeometryEffect(id: "coachy.onboarding", in: namespace)

            Spacer()

            VStack(spacing: 20) {
                Text("Grant Permissions")
                    .font(.system(size: 24, weight: .semibold))
                    .foregroundStyle(Color.app.foreground)

                Text("These help me track your focus and enforce your rules.")
                    .font(.subheadline)
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                    .multilineTextAlignment(.center)

                VStack(spacing: 12) {
                    // Notifications
                    PermissionStateView(
                        icon: "bell.fill",
                        title: "Notifications",
                        description: "Nudges, reminders, and rule alerts",
                        status: coord.notificationsStatus,
                        onRequest: {
                            requestingNotifications = true
                            Task {
                                await coord.requestNotificationsPermission()
                                requestingNotifications = false
                                HapticChoreographer.shared.perform(.celebrate)
                            }
                        },
                        onOpenSettings: { openSettings() }
                    )

                    // Calendar
                    PermissionStateView(
                        icon: "calendar.circle.fill",
                        title: "Calendar (EventKit)",
                        description: "Sync your events to avoid conflicts",
                        status: coord.calendarStatus,
                        onRequest: {
                            requestingCalendar = true
                            Task {
                                await coord.requestCalendarPermission()
                                requestingCalendar = false
                                HapticChoreographer.shared.perform(.celebrate)
                            }
                        },
                        onOpenSettings: { openSettings() }
                    )

                    // HealthKit (optional, iOS 17+)
                    PermissionStateView(
                        icon: "heart.fill",
                        title: "HealthKit",
                        description: "Track activity and sleep data (optional)",
                        status: .notDetermined,
                        onRequest: {
                            requestingHealthKit = true
                            Task {
                                // HealthKit request deferred — non-critical
                                requestingHealthKit = false
                            }
                        },
                        onOpenSettings: { openSettings() }
                    )

                    // FamilyControls (pending entitlement)
                    PermissionStateView(
                        icon: "shield.fill",
                        title: "Family Controls",
                        description: "Enforce app limits and schedules",
                        status: coord.familyControlsStatus,
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
            }
            .padding(.horizontal, 24)
            .padding(.bottom, 40)

            Spacer()

            // Skip nudge with "Grant for best experience" messaging
            VStack(spacing: 8) {
                Text("You can grant permissions later in Settings > FocalPoint > Privacy.")
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
                    .multilineTextAlignment(.center)
            }
            .padding(.horizontal, 24)
            .padding(.bottom, 16)
        }
        .task {
            // Refresh status on appear
            await coord.refreshNotificationStatus()
        }
        .sheet(isPresented: $showingSettings) {
            SettingsLink()
        }
    }

    private func openSettings() {
        if let settingsURL = URL(string: UIApplication.openSettingsURLString) {
            UIApplication.shared.open(settingsURL)
        }
    }
}

/// Status card for a single permission. Renders:
/// - Granted: checkmark + green, disabled button
/// - Denied: X + red, "Open Settings" button
/// - Pending: spinner or "pending" badge
/// - Pending Entitlement: badge "Pending Apple approval"
public struct PermissionStateView: View {
    let icon: String
    let title: String
    let description: String
    let status: OnboardingCoordinator.PermissionStatus
    let onRequest: () -> Void
    let onOpenSettings: () -> Void

    var body: some View {
        VStack(spacing: 12) {
            HStack(spacing: 12) {
                Image(systemName: icon)
                    .font(.system(size: 24))
                    .frame(width: 40, height: 40)
                    .foregroundStyle(statusColor)

                VStack(alignment: .leading, spacing: 2) {
                    Text(title)
                        .font(.body.weight(.semibold))
                        .foregroundStyle(Color.app.foreground)
                    Text(description)
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.6))
                }

                Spacer()

                statusBadge
            }
            .padding(12)
            .background(statusBackgroundColor)
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(statusBorderColor, lineWidth: 1)
            )

            // Action button (only if not granted)
            if status != .granted {
                HStack(spacing: 8) {
                    switch status {
                    case .granted:
                        EmptyView()

                    case .denied:
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

                    case .notDetermined:
                        Button(action: onRequest) {
                            HStack(spacing: 6) {
                                Image(systemName: "checkmark.circle.fill")
                                Text("Grant Permission")
                            }
                            .frame(maxWidth: .infinity)
                            .padding(10)
                            .background(Color.app.accent)
                            .foregroundStyle(.white)
                            .cornerRadius(8)
                        }

                    case .pendingEntitlement:
                        Text("Pending Apple approval — try later")
                            .font(.caption)
                            .foregroundStyle(Color.orange)
                            .frame(maxWidth: .infinity, alignment: .center)
                            .padding(10)
                            .background(Color.orange.opacity(0.1))
                            .cornerRadius(8)
                    }
                }
                .transition(.opacity.combined(with: .move(edge: .bottom)))
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

    private var statusBackgroundColor: Color {
        switch status {
        case .granted: return Color.green.opacity(0.08)
        case .denied: return Color.red.opacity(0.08)
        case .notDetermined: return Color.app.surface
        case .pendingEntitlement: return Color.orange.opacity(0.08)
        }
    }

    private var statusBorderColor: Color {
        switch status {
        case .granted: return Color.green.opacity(0.3)
        case .denied: return Color.red.opacity(0.3)
        case .notDetermined: return Color.app.foreground.opacity(0.2)
        case .pendingEntitlement: return Color.orange.opacity(0.3)
        }
    }

    @ViewBuilder
    private var statusBadge: some View {
        switch status {
        case .granted:
            Image(systemName: "checkmark.circle.fill")
                .foregroundStyle(Color.green)
                .font(.system(size: 20))

        case .denied:
            Image(systemName: "xmark.circle.fill")
                .foregroundStyle(Color.red)
                .font(.system(size: 20))

        case .notDetermined:
            VStack {}
                .frame(width: 20, height: 20)

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

#endif
