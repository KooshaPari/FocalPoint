import SwiftUI
import DesignSystem

/// Small status banner shown at the top of the Home tab when sync is active.
///
/// Shows sync progress, errors, or conflict count.
struct SyncStatusBanner: View {
    let status: CloudKitSyncStatus?
    let isInProgress: Bool
    let lastSyncTime: Date?
    let conflictCount: Int

    var body: some View {
        if isInProgress || shouldShowBanner {
            VStack(spacing: 0) {
                HStack(spacing: 8) {
                    if isInProgress {
                        ProgressView()
                            .scaleEffect(0.75)
                            .tint(Color.app.accent)
                    } else if conflictCount > 0 {
                        Image(systemName: "exclamationmark.circle.fill")
                            .foregroundStyle(.orange)
                    } else {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundStyle(.green)
                    }

                    VStack(alignment: .leading, spacing: 2) {
                        if isInProgress {
                            Text("Syncing...")
                                .font(.caption)
                                .fontWeight(.medium)
                                .foregroundStyle(.primary)
                        } else if conflictCount > 0 {
                            Text("\(conflictCount) sync conflict\(conflictCount == 1 ? "" : "s")")
                                .font(.caption)
                                .fontWeight(.medium)
                                .foregroundStyle(.orange)
                            if let lastSync = lastSyncTime {
                                Text("Last sync: \(formatSyncTime(lastSync))")
                                    .font(.caption2)
                                    .foregroundStyle(.secondary)
                            }
                        } else if let lastSync = lastSyncTime {
                            Text("Synced")
                                .font(.caption)
                                .fontWeight(.medium)
                                .foregroundStyle(.secondary)
                            Text(formatSyncTime(lastSync))
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }

                    Spacer()

                    if let status = status, case .unavailable(let reason) = status {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundStyle(.red)
                            .help(reason)
                    }
                }
                .padding(.horizontal, 12)
                .padding(.vertical, 10)
                .background(Color(.systemGray6))

                Divider()
            }
        }
    }

    private var shouldShowBanner: Bool {
        isInProgress || conflictCount > 0 || lastSyncTime != nil
    }

    private func formatSyncTime(_ date: Date) -> String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .short
        return formatter.localizedString(for: date, relativeTo: Date())
    }
}

#Preview {
    VStack(spacing: 0) {
        SyncStatusBanner(status: .available, isInProgress: true, lastSyncTime: nil, conflictCount: 0)

        SyncStatusBanner(
            status: .available,
            isInProgress: false,
            lastSyncTime: Date().addingTimeInterval(-300),
            conflictCount: 0
        )

        SyncStatusBanner(
            status: .available,
            isInProgress: false,
            lastSyncTime: Date().addingTimeInterval(-3600),
            conflictCount: 2
        )

        Spacer()
    }
}
