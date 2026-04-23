#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

/// Well-known event type strings. Kept as a simple list until the FFI exposes
/// a first-class enum for them; the existing templates reference these verbatim.
enum WellKnownEvents {
    static let all: [String] = [
        "focus:session_started",
        "focus:session_completed",
        "canvas:assignment_due_soon",
        "canvas:assignment_submitted",
        "health:workout_completed",
        "health:sleep_debt_reported",
        "clock:hour_22",
        "clock:weekend",
        "manual:user_initiated",
    ]
}

public struct RuleDetailView: View {
    public enum Mode: Equatable {
        case create
        case edit(ruleId: String)
    }

    @Environment(\.dismiss) private var dismiss
    @EnvironmentObject private var holder: CoreHolder

    @State private var draft: RuleDraft
    @State private var cooldownChoice: CooldownChoice
    @State private var saveError: String?

    let mode: Mode

    public init(mode: Mode, seed: RuleDraft) {
        self.mode = mode
        self._draft = State(initialValue: seed)
        self._cooldownChoice = State(initialValue: CooldownChoice.closest(toSeconds: seed.cooldownSeconds))
    }

    public static func blankDraft() -> RuleDraft {
        RuleDraft(
            id: "rule-\(Int(Date().timeIntervalSince1970))",
            name: "New rule",
            triggerEvent: WellKnownEvents.all.first ?? "manual:user_initiated",
            actions: [],
            priority: 50,
            cooldownSeconds: nil,
            durationSeconds: nil,
            explanationTemplate: "{rule_name} fired on {event_type}:{event_id}.",
            enabled: true
        )
    }

    public var body: some View {
        NavigationStack {
            Form {
                Section("Identity") {
                    TextField("Name", text: $draft.name)
                    Toggle("Enabled", isOn: $draft.enabled)
                }

                Section("Trigger") {
                    Picker("Event", selection: $draft.triggerEvent) {
                        ForEach(WellKnownEvents.all, id: \.self) { ev in
                            Text(ev).tag(ev)
                        }
                    }
                }

                Section("Actions") {
                    ForEach(draft.actions.indices, id: \.self) { idx in
                        ActionRow(action: $draft.actions[idx])
                    }
                    .onDelete { idxs in
                        draft.actions.remove(atOffsets: idxs)
                    }
                    Menu("Add action") {
                        Button("Block social (50m)") {
                            draft.actions.append(.block(profile: "social", durationSeconds: Int64(60 * 50)))
                        }
                        Button("Unblock social") {
                            draft.actions.append(.unblock(profile: "social"))
                        }
                        Button("Grant credit") {
                            draft.actions.append(.grantCredit(amount: 10))
                        }
                        Button("Deduct credit") {
                            draft.actions.append(.deductCredit(amount: 10))
                        }
                        Button("Streak increment") {
                            draft.actions.append(.streakIncrement(name: "focus"))
                        }
                        Button("Streak reset") {
                            draft.actions.append(.streakReset(name: "focus"))
                        }
                        Button("Notify") {
                            draft.actions.append(.notify(message: "Heads up!"))
                        }
                    }
                }

                Section("Tuning") {
                    Stepper("Priority: \(draft.priority)", value: $draft.priority, in: 0...100)
                    Picker("Cooldown", selection: $cooldownChoice) {
                        ForEach(CooldownChoice.allCases) { c in
                            Text(c.label).tag(c)
                        }
                    }
                    .onChange(of: cooldownChoice) { _, new in
                        draft.cooldownSeconds = new.seconds
                    }
                }

                Section {
                    TextField("Explanation template", text: $draft.explanationTemplate, axis: .vertical)
                        .lineLimit(2...5)
                } header: {
                    Text("Explanation")
                } footer: {
                    Text("Substitutions: {rule_name}, {event_type}, {event_id}.")
                        .font(.caption2)
                }

                if let e = saveError {
                    Section {
                        Text(e).foregroundStyle(.red).font(.caption)
                    }
                }
            }
            .navigationTitle(mode == .create ? "New Rule" : "Edit Rule")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Save") { save() }
                }
            }
        }
    }

    private func save() {
        do {
            try holder.core.mutations().upsert(rule: draft)
            holder.bump()
            dismiss()
        } catch {
            saveError = "\(error)"
        }
    }
}

private struct ActionRow: View {
    @Binding var action: RuleActionDto

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(label)
                .font(.callout.weight(.semibold))
            detailEditor
        }
    }

    private var label: String {
        switch action {
        case .grantCredit: return "Grant credit"
        case .deductCredit: return "Deduct credit"
        case .block: return "Block"
        case .unblock: return "Unblock"
        case .streakIncrement: return "Streak +"
        case .streakReset: return "Streak reset"
        case .notify: return "Notify"
        }
    }

    @ViewBuilder
    private var detailEditor: some View {
        switch action {
        case .grantCredit(let amount):
            Stepper("Amount: \(amount)", value: Binding(
                get: { amount },
                set: { action = .grantCredit(amount: $0) }
            ), in: 0...500)
        case .deductCredit(let amount):
            Stepper("Amount: \(amount)", value: Binding(
                get: { amount },
                set: { action = .deductCredit(amount: $0) }
            ), in: 0...500)
        case .block(let profile, let secs):
            TextField("Profile", text: Binding(
                get: { profile },
                set: { action = .block(profile: $0, durationSeconds: secs) }
            ))
            Text("Duration: \(secs / 60) min").font(.caption)
        case .unblock(let profile):
            TextField("Profile", text: Binding(
                get: { profile },
                set: { action = .unblock(profile: $0) }
            ))
        case .streakIncrement(let n):
            TextField("Streak name", text: Binding(
                get: { n },
                set: { action = .streakIncrement(name: $0) }
            ))
        case .streakReset(let n):
            TextField("Streak name", text: Binding(
                get: { n },
                set: { action = .streakReset(name: $0) }
            ))
        case .notify(let message):
            TextField("Message", text: Binding(
                get: { message },
                set: { action = .notify(message: $0) }
            ))
        }
    }
}

enum CooldownChoice: String, CaseIterable, Identifiable, Hashable {
    case off
    case fiveMinutes
    case oneHour
    case oneDay

    var id: String { rawValue }
    var label: String {
        switch self {
        case .off: return "Off"
        case .fiveMinutes: return "5 minutes"
        case .oneHour: return "1 hour"
        case .oneDay: return "1 day"
        }
    }
    var seconds: Int64? {
        switch self {
        case .off: return nil
        case .fiveMinutes: return 5 * 60
        case .oneHour: return 60 * 60
        case .oneDay: return 24 * 60 * 60
        }
    }
    static func closest(toSeconds s: Int64?) -> CooldownChoice {
        guard let s else { return .off }
        if s <= 10 * 60 { return .fiveMinutes }
        if s <= 2 * 60 * 60 { return .oneHour }
        return .oneDay
    }
}
#endif
