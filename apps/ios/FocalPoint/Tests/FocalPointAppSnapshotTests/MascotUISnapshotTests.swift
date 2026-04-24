import SwiftUI
import SnapshotTesting
import XCTest
@testable import MascotUI

// Traces to: FR-MASCOT-001 (Coachy mascot animation and state display)

class MascotUISnapshotTests: XCTestCase {
    let record = false

    // MARK: - Mascot Idle State

    func testCoachyIdleState() {
        let view = CoachyTestView(state: .idle)

        assertViewSnapshot(
            view: view,
            name: "coachy_idle_state",
            record: record
        )
    }

    // MARK: - Mascot Engaged State

    func testCoachyEngagedState() {
        let view = CoachyTestView(state: .engaged)

        assertViewSnapshot(
            view: view,
            name: "coachy_engaged_state",
            record: record
        )
    }

    // MARK: - Mascot Celebration

    func testCoachyCelebration() {
        let view = CoachyTestView(state: .celebrating)

        assertViewSnapshot(
            view: view,
            name: "coachy_celebration",
            record: record
        )
    }

    // MARK: - Mascot Warning

    func testCoachyWarning() {
        let view = CoachyTestView(state: .warning)

        assertViewSnapshot(
            view: view,
            name: "coachy_warning",
            record: record
        )
    }
}

// MARK: - Test Views

enum CoachyState {
    case idle
    case engaged
    case celebrating
    case warning
}

struct CoachyTestView: View {
    let state: CoachyState

    var body: some View {
        VStack(spacing: 20) {
            Text("Coachy Mascot — \(state)")
                .font(.headline)

            // Placeholder: Actual mascot view would render here
            ZStack {
                RoundedRectangle(cornerRadius: 12)
                    .fill(Color(.systemBackground))
                    .frame(height: 200)

                VStack(spacing: 8) {
                    Image(systemName: "smileyface")
                        .font(.system(size: 60))
                        .foregroundColor(stateColor())

                    Text(stateDescription())
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }

            Text("Status: \(stateDescription())")
                .font(.caption)
                .padding(8)
                .background(stateColor().opacity(0.1))
                .cornerRadius(4)
        }
        .padding(16)
    }

    private func stateColor() -> Color {
        switch state {
        case .idle:
            return .gray
        case .engaged:
            return .blue
        case .celebrating:
            return .green
        case .warning:
            return .orange
        }
    }

    private func stateDescription() -> String {
        switch state {
        case .idle:
            return "Idle — Waiting for action"
        case .engaged:
            return "Engaged — Guiding you through a focus block"
        case .celebrating:
            return "Celebrating — Goal achieved!"
        case .warning:
            return "Warning — Focus block ending soon"
        }
    }
}
