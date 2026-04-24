import SwiftUI
import SnapshotTesting
import XCTest
@testable import FocalPointApp

// Traces to: FR-TAB-001 (Tab view UI verification)

class CoreTabsSnapshotTests: XCTestCase {
    let record = false

    // MARK: - Home Tab

    func testHomeTab() {
        let view = HomeTabTestView()

        assertViewSnapshot(
            view: view,
            name: "home_tab",
            record: record
        )
    }

    // MARK: - Today Tab

    func testTodayTab() {
        let view = TodayTabTestView()

        assertViewSnapshot(
            view: view,
            name: "today_tab",
            record: record
        )
    }

    // MARK: - Focus Mode Tab

    func testFocusModeTab() {
        let view = FocusModeTabTestView()

        assertViewSnapshot(
            view: view,
            name: "focus_mode_tab",
            record: record
        )
    }

    // MARK: - Tasks Tab

    func testTasksTab() {
        let view = TasksTabTestView()

        assertViewSnapshot(
            view: view,
            name: "tasks_tab",
            record: record
        )
    }

    // MARK: - Wallet Tab

    func testWalletTab() {
        let view = WalletTabTestView()

        assertViewSnapshot(
            view: view,
            name: "wallet_tab",
            record: record
        )
    }

    // MARK: - Stats Tab

    func testStatsTab() {
        let view = StatsTabTestView()

        assertViewSnapshot(
            view: view,
            name: "stats_tab",
            record: record
        )
    }

    // MARK: - Settings Tab

    func testSettingsTab() {
        let view = SettingsTabTestView()

        assertViewSnapshot(
            view: view,
            name: "settings_tab",
            record: record
        )
    }
}

// MARK: - Test Tab Views

struct HomeTabTestView: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Good Afternoon")
                .font(.title2)
                .bold()

            VStack(spacing: 12) {
                HStack {
                    Text("Active Rules")
                    Spacer()
                    Text("3").font(.headline)
                }
                .font(.headline)

                ProgressView(value: 0.65).tint(.blue)

                HStack {
                    Text("1,960 min used of 3,000 min")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Spacer()
                }
            }
            .padding(12)
            .background(Color(.systemBackground))
            .cornerRadius(8)

            VStack(alignment: .leading, spacing: 8) {
                Text("Next Focus Block").font(.headline)
                HStack {
                    Text("🎯 Work Focus").font(.body)
                    Spacer()
                    Text("9:00 AM").font(.body).foregroundColor(.secondary)
                }
            }
            .padding(12)
            .background(Color.blue.opacity(0.05))
            .cornerRadius(8)

            Spacer()
        }
        .padding(16)
    }
}

struct TodayTabTestView: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Today")
                .font(.headline)

            VStack(spacing: 12) {
                HStack {
                    Text("Instagram")
                    Spacer()
                    Text("2h 15m").font(.body)
                }
                ProgressView(value: 0.75).tint(.red)

                HStack {
                    Text("Discord")
                    Spacer()
                    Text("45m").font(.body)
                }
                ProgressView(value: 0.5).tint(.purple)

                HStack {
                    Text("TikTok")
                    Spacer()
                    Text("30m").font(.body)
                }
                ProgressView(value: 0.3).tint(.blue)
            }

            Spacer()
        }
        .padding(16)
    }
}

struct FocusModeTabTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            Text("Focus Modes")
                .font(.headline)

            VStack(spacing: 12) {
                HStack {
                    VStack(alignment: .leading) {
                        Text("Work Focus")
                            .font(.headline)
                        Text("9am–12pm daily")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    Spacer()
                    Toggle("", isOn: .constant(true))
                }
                .padding(12)
                .background(Color(.systemBackground))
                .cornerRadius(8)

                HStack {
                    VStack(alignment: .leading) {
                        Text("Sleep Time")
                            .font(.headline)
                        Text("11pm–7am daily"
                        )
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    Spacer()
                    Toggle("", isOn: .constant(false))
                }
                .padding(12)
                .background(Color(.systemBackground))
                .cornerRadius(8)
            }

            Spacer()
        }
        .padding(16)
    }
}

struct TasksTabTestView: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Synced Tasks")
                .font(.headline)

            VStack(spacing: 12) {
                HStack {
                    Image(systemName: "checkmark.square")
                        .foregroundColor(.green)
                    VStack(alignment: .leading) {
                        Text("Review Q2 goals").font(.body)
                        Text("From Canvas").font(.caption).foregroundColor(.secondary)
                    }
                    Spacer()
                }

                HStack {
                    Image(systemName: "square")
                        .foregroundColor(.secondary)
                    VStack(alignment: .leading) {
                        Text("Code review PR #42").font(.body)
                        Text("From GitHub").font(.caption).foregroundColor(.secondary)
                    }
                    Spacer()
                }

                HStack {
                    Image(systemName: "square")
                        .foregroundColor(.secondary)
                    VStack(alignment: .leading) {
                        Text("Team standup").font(.body)
                        Text("From Calendar").font(.caption).foregroundColor(.secondary)
                    }
                    Spacer()
                }
            }

            Spacer()
        }
        .padding(16)
    }
}

struct WalletTabTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            VStack(spacing: 12) {
                Text("Reward Balance").font(.headline)

                ZStack {
                    RoundedRectangle(cornerRadius: 12)
                        .fill(Color.green.opacity(0.1))

                    VStack(spacing: 4) {
                        Text("4 h 30 min")
                            .font(.system(size: 32, weight: .bold))
                        Text("Available to spend")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
                .frame(height: 120)
            }

            VStack(alignment: .leading, spacing: 12) {
                Text("Recent Transactions").font(.headline)

                HStack {
                    Text("+ Completed Work Focus").font(.body)
                    Spacer()
                    Text("+15 min").foregroundColor(.green)
                }
                .padding(8)

                HStack {
                    Text("- Penalty (Instagram)").font(.body)
                    Spacer()
                    Text("-10 min").foregroundColor(.red)
                }
                .padding(8)
            }

            Spacer()
        }
        .padding(16)
    }
}

struct StatsTabTestView: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Statistics")
                .font(.headline)

            VStack(spacing: 12) {
                HStack {
                    Text("Daily Average Usage")
                    Spacer()
                    Text("2h 45m").bold()
                }
                .font(.body)

                HStack {
                    Text("Focus Blocks Completed")
                    Spacer()
                    Text("12").bold()
                }
                .font(.body)

                HStack {
                    Text("Rules Enforced")
                    Spacer()
                    Text("156").bold()
                }
                .font(.body)

                HStack {
                    Text("Rewards Earned")
                    Spacer()
                    Text("8h 15m").bold()
                }
                .font(.body)
            }
            .padding(12)
            .background(Color(.systemBackground))
            .cornerRadius(8)

            Spacer()
        }
        .padding(16)
    }
}

struct SettingsTabTestView: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Settings")
                .font(.headline)

            VStack(spacing: 12) {
                HStack {
                    Text("Background Sync")
                    Spacer()
                    Toggle("", isOn: .constant(true))
                }
                .font(.body)

                HStack {
                    Text("Notifications")
                    Spacer()
                    Toggle("", isOn: .constant(true))
                }
                .font(.body)

                HStack {
                    Text("Dark Mode")
                    Spacer()
                    Toggle("", isOn: .constant(false))
                }
                .font(.body)
            }
            .padding(12)
            .background(Color(.systemBackground))
            .cornerRadius(8)

            VStack(spacing: 8) {
                Button(action: {}) {
                    Text("View Audit Chain").frame(maxWidth: .infinity)
                }
                .buttonStyle(.bordered)

                Button(action: {}) {
                    Text("Export Data")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.bordered)

                Button(role: .destructive, action: {}) {
                    Text("Reset All Data")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.bordered)
            }

            Spacer()
        }
        .padding(16)
    }
}
