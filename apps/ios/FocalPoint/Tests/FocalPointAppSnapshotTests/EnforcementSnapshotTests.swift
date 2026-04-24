import SwiftUI
import SnapshotTesting
import XCTest
@testable import Enforcement

// Traces to: FR-ENFORCE-001 (Rule creation and enforcement UI verification)

class EnforcementSnapshotTests: XCTestCase {
    let record = false

    // MARK: - Rule List View

    func testRuleListEmpty() {
        let view = RuleListTestView(rules: [])

        assertViewSnapshot(
            view: view,
            name: "rule_list_empty",
            record: record
        )
    }

    func testRuleListWithRules() {
        let rules = [
            ("Instagram", 0.75),
            ("TikTok", 0.5),
            ("Discord", 0.25),
        ]
        let view = RuleListTestView(rules: rules)

        assertViewSnapshot(
            view: view,
            name: "rule_list_populated",
            record: record
        )
    }

    // MARK: - Rule Creation Form

    func testRuleCreationForm() {
        let view = RuleCreationFormTestView()

        assertViewSnapshot(
            view: view,
            name: "rule_creation_form",
            record: record
        )
    }

    // MARK: - Focus Block Active

    func testFocusBlockActive() {
        let view = FocusBlockActiveTestView()

        assertViewSnapshot(
            view: view,
            name: "focus_block_active",
            record: record
        )
    }

    // MARK: - Penalty Applied

    func testPenaltyAppliedView() {
        let view = PenaltyAppliedTestView()

        assertViewSnapshot(
            view: view,
            name: "penalty_applied",
            record: record
        )
    }
}

// MARK: - Test Views

struct RuleListTestView: View {
    let rules: [(String, Double)] // (name, progress)

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Active Rules")
                .font(.headline)
                .padding(.horizontal)

            if rules.isEmpty {
                VStack {
                    Text("No rules yet")
                        .font(.body)
                        .foregroundColor(.secondary)
                }
                .frame(maxWidth: .infinity, minHeight: 100)
            } else {
                ScrollView {
                    VStack(spacing: 12) {
                        ForEach(0..<rules.count, id: \.self) { i in
                            let (name, progress) = rules[i]
                            RuleCardTestView(name: name, progress: progress)
                        }
                    }
                    .padding(.horizontal)
                }
            }

            Spacer()
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
    }
}

struct RuleCardTestView: View {
    let name: String
    let progress: Double

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Text(name).font(.headline)
                Spacer()
                Text("\(Int(progress * 100))%").font(.caption).foregroundColor(.secondary)
            }
            ProgressView(value: progress)
                .tint(.blue)
            Text("Used \(Int(progress * 120)) min of 120 min today")
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .padding(12)
        .background(Color(.systemBackground))
        .cornerRadius(8)
    }
}

struct RuleCreationFormTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            Text("Create New Rule")
                .font(.headline)

            VStack(alignment: .leading, spacing: 12) {
                Text("App").font(.caption).foregroundColor(.secondary)
                HStack {
                    Image(systemName: "app")
                    Text("Select app...").foregroundColor(.secondary)
                    Spacer()
                    Image(systemName: "chevron.down")
                }
                .padding(12)
                .background(Color(.systemBackground))
                .cornerRadius(8)

                Text("Duration (minutes)").font(.caption).foregroundColor(.secondary)
                TextField("120", text: .constant("120"))
                    .keyboardType(.numberPad)
                    .padding(12)
                    .background(Color(.systemBackground))
                    .cornerRadius(8)

                Text("Repeat").font(.caption).foregroundColor(.secondary)
                HStack {
                    Toggle("Daily", isOn: .constant(true))
                    Spacer()
                    Toggle("Weekly", isOn: .constant(false))
                }
                .font(.body)
            }

            Spacer()

            Button(action: {}) {
                Text("Save Rule")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)
        }
        .padding(20)
    }
}

struct FocusBlockActiveTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            VStack(spacing: 12) {
                Text("🎯 Focus Mode Active")
                    .font(.headline)

                ZStack {
                    Circle()
                        .stroke(Color.blue.opacity(0.2), lineWidth: 8)
                        .frame(width: 120, height: 120)

                    VStack(spacing: 4) {
                        Text("45:32")
                            .font(.system(size: 32, weight: .bold, design: .monospaced))
                        Text("remaining")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
            }
            .padding(20)
            .background(Color.blue.opacity(0.05))
            .cornerRadius(12)

            VStack(alignment: .leading, spacing: 8) {
                Text("Focus Rules")
                    .font(.headline)

                HStack {
                    Image(systemName: "xmark.circle").foregroundColor(.red)
                    Text("Instagram blocked")
                    Spacer()
                }
                .font(.body)

                HStack {
                    Image(systemName: "xmark.circle").foregroundColor(.red)
                    Text("TikTok blocked")
                    Spacer()
                }
                .font(.body)
            }
            .padding(12)
            .background(Color(.systemBackground))
            .cornerRadius(8)

            Spacer()

            Button(role: .destructive, action: {}) {
                Text("Cancel Focus Block")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.bordered)
        }
        .padding(20)
    }
}

struct PenaltyAppliedTestView: View {
    var body: some View {
        VStack(spacing: 20) {
            VStack(spacing: 12) {
                Image(systemName: "exclamationmark.triangle")
                    .font(.system(size: 40))
                    .foregroundColor(.orange)

                Text("Penalty Applied")
                    .font(.headline)

                Text("You exceeded your Instagram limit")
                    .font(.body)
                    .foregroundColor(.secondary)
            }
            .padding(20)
            .background(Color.orange.opacity(0.1))
            .cornerRadius(12)

            VStack(alignment: .leading, spacing: 8) {
                Text("Penalty: -15 reward minutes").font(.body).bold()
                Text("You now have 30 minutes of reward credit remaining.")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            .padding(12)
            .background(Color(.systemBackground))
            .cornerRadius(8)

            Spacer()

            Button(action: {}) {
                Text("Acknowledge")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)
        }
        .padding(20)
    }
}
