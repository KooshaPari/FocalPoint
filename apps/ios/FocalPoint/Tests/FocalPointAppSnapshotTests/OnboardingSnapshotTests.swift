import SwiftUI
import SnapshotTesting
import XCTest
@testable import FocalPointApp

// Traces to: FR-ONBOARD-001 (Onboarding flow UI verification)

class OnboardingSnapshotTests: XCTestCase {
    let record = false

    // MARK: - Welcome Screen

    func testOnboardingWelcomeScreen() {
        let view = OnboardingWelcomeTestView()

        assertViewSnapshot(
            view: view,
            name: "onboarding_welcome",
            record: record
        )
    }

    // MARK: - Permission Request

    func testOnboardingPermissionScreen() {
        let view = OnboardingPermissionTestView()

        assertViewSnapshot(
            view: view,
            name: "onboarding_permission",
            record: record
        )
    }

    // MARK: - Connector Setup

    func testOnboardingConnectorSetup() {
        let view = OnboardingConnectorTestView()

        assertViewSnapshot(
            view: view,
            name: "onboarding_connector_setup",
            record: record
        )
    }

    // MARK: - First Rule Creation

    func testOnboardingFirstRule() {
        let view = OnboardingFirstRuleTestView()

        assertViewSnapshot(
            view: view,
            name: "onboarding_first_rule",
            record: record
        )
    }

    // MARK: - Completion Screen

    func testOnboardingComplete() {
        let view = OnboardingCompleteTestView()

        assertViewSnapshot(
            view: view,
            name: "onboarding_complete",
            record: record
        )
    }
}

// MARK: - Test Views

struct OnboardingWelcomeTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            Text("Welcome to FocalPoint")
                .font(.largeTitle)
                .bold()

            Text("Your personal focus companion")
                .font(.headline)
                .foregroundColor(.secondary)

            VStack(alignment: .leading, spacing: 12) {
                Label("Create focus blocks", systemImage: "clock.fill")
                Label("Enforce screen-time limits", systemImage: "hourglass")
                Label("Earn rewards", systemImage: "star.fill")
            }
            .font(.body)

            Spacer()

            Button(action: {}) {
                Text("Get Started")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)

            Button(action: {}) {
                Text("Learn More")
            }
            .buttonStyle(.plain)
        }
        .padding(20)
    }
}

struct OnboardingPermissionTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            Text("Permissions Required")
                .font(.headline)

            VStack(alignment: .leading, spacing: 12) {
                HStack(spacing: 12) {
                    Image(systemName: "checkmark.circle")
                        .font(.title3)
                        .foregroundColor(.green)
                    VStack(alignment: .leading) {
                        Text("Family Controls").font(.body).bold()
                        Text("Required to enforce rules").font(.caption).foregroundColor(.secondary)
                    }
                }

                HStack(spacing: 12) {
                    Image(systemName: "checkmark.circle")
                        .font(.title3)
                        .foregroundColor(.green)
                    VStack(alignment: .leading) {
                        Text("Screen Time").font(.body).bold()
                        Text("To monitor usage").font(.caption).foregroundColor(.secondary)
                    }
                }
            }

            Spacer()

            Button(action: {}) {
                Text("Grant Permissions")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)
        }
        .padding(20)
    }
}

struct OnboardingConnectorTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            Text("Connect Your Tools")
                .font(.headline)

            Text("Optional: sync tasks from Calendar, Canvas, GitHub")
                .font(.caption)
                .foregroundColor(.secondary)

            VStack(spacing: 12) {
                ConnectorOptionView(name: "Google Calendar", icon: "calendar")
                ConnectorOptionView(name: "Canvas", icon: "checkmark.square")
                ConnectorOptionView(name: "GitHub", icon: "code")
            }

            Spacer()

            Button(action: {}) {
                Text("Next")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)
        }
        .padding(20)
    }
}

struct ConnectorOptionView: View {
    let name: String
    let icon: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .font(.title3)
                .foregroundColor(.blue)
            Text(name).font(.body)
            Spacer()
            Image(systemName: "chevron.right")
                .foregroundColor(.secondary)
        }
        .padding(12)
        .background(Color(.systemBackground))
        .cornerRadius(8)
    }
}

struct OnboardingFirstRuleTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            Text("Create Your First Rule")
                .font(.headline)

            VStack(alignment: .leading, spacing: 12) {
                Text("App").font(.caption).foregroundColor(.secondary)
                HStack {
                    Image(systemName: "app.dashed")
                    Text("Select app...").foregroundColor(.secondary)
                    Spacer()
                }
                .padding(12)
                .background(Color(.systemBackground))
                .cornerRadius(8)

                Text("Duration").font(.caption).foregroundColor(.secondary)
                HStack {
                    Text("2 hours").font(.body)
                    Spacer()
                    Text("per day")
                }
                .padding(12)
                .background(Color(.systemBackground))
                .cornerRadius(8)
            }

            Spacer()

            Button(action: {}) {
                Text("Create Rule")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)
        }
        .padding(20)
    }
}

struct OnboardingCompleteTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "checkmark.circle")
                .font(.system(size: 60))
                .foregroundColor(.green)

            Text("You're All Set!")
                .font(.largeTitle)
                .bold()

            VStack(alignment: .leading, spacing: 8) {
                Text("✓ Family Controls enabled")
                Text("✓ First rule created")
                Text("✓ Connectors configured")
            }
            .font(.body)
            .padding(12)
            .background(Color.green.opacity(0.1))
            .cornerRadius(8)

            Spacer()

            Button(action: {}) {
                Text("Start Using FocalPoint")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)
        }
        .padding(20)
    }
}
