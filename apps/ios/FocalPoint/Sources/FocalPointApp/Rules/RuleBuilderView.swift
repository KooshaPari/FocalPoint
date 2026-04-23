#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

// =============================================================================
// RuleBuilderView — in-app Rule Authoring Wizard
//
// 4-step flow (When / If / Then / Settings) + Review pane. Drives the wizard
// off the live DSL catalog returned by `core.rulesDsl()` so picker options
// stay in lockstep with the Rust side. Persists via `mutations().upsert`.
//
// Traces to: Task #20 (in-app visual rule builder), FR-RULE-003 (Condition
// DSL), FR-RULE-008 (expanded Action catalog).
// =============================================================================

public struct RuleBuilderView: View {
    @Environment(\.dismiss) private var dismiss
    @EnvironmentObject private var holder: CoreHolder

    @State private var path = NavigationPath()
    @State private var model: BuilderModel
    @State private var catalog: DslCatalog = .empty
    @State private var saveError: String?
    @State private var celebratory: Bool = false

    public init(seed: BuilderModel? = nil) {
        self._model = State(initialValue: seed ?? BuilderModel())
    }

    public var body: some View {
        NavigationStack(path: $path) {
            StepWhenView(model: $model, catalog: catalog, onNext: { path.append(Step.ifCond) })
                .navigationTitle("New Rule")
                .navigationDestination(for: Step.self) { step in
                    switch step {
                    case .ifCond:
                        StepIfView(model: $model, catalog: catalog,
                                   onNext: { path.append(Step.then) })
                    case .then:
                        StepThenView(model: $model, catalog: catalog,
                                     onNext: { path.append(Step.settings) })
                    case .settings:
                        StepSettingsView(model: $model,
                                         onNext: { path.append(Step.review) })
                    case .review:
                        StepReviewView(model: $model,
                                       saveError: saveError,
                                       celebratory: $celebratory,
                                       onSave: save)
                    }
                }
                .toolbar {
                    ToolbarItem(placement: .cancellationAction) {
                        Button("Cancel") { dismiss() }
                    }
                    ToolbarItem(placement: .topBarTrailing) {
                        CoachyView(
                            state: CoachyState(
                                pose: celebratory ? .celebratory : .curious,
                                emotion: celebratory ? .excited : .focused,
                                bubbleText: nil
                            ),
                            size: 44
                        )
                        .accessibilityLabel("Coachy")
                    }
                }
                .background(Color.app.background.ignoresSafeArea())
        }
        .task { await loadCatalog() }
    }

    private func loadCatalog() async {
        let json = holder.core.rulesDsl()
        if let data = json.data(using: .utf8),
           let cat = try? JSONDecoder().decode(DslCatalog.self, from: data) {
            await MainActor.run { self.catalog = cat }
        }
    }

    private func save() {
        do {
            try holder.core.mutations().upsert(rule: model.toRuleDraft())
            holder.bump()
            celebratory = true
            Task { @MainActor in
                try? await Task.sleep(nanoseconds: 700_000_000)
                dismiss()
            }
        } catch {
            saveError = "\(error)"
        }
    }
}

// MARK: - Wizard step identifier

enum Step: Hashable { case ifCond, then, settings, review }

// MARK: - BuilderModel (mutable state across steps)

public struct BuilderModel: Equatable {
    public enum TriggerKind: String, CaseIterable, Identifiable {
        case event = "Event"
        case schedule = "Schedule"
        case stateChange = "StateChange"
        public var id: String { rawValue }
        var label: String {
            switch self {
            case .event: return "Event"
            case .schedule: return "Schedule"
            case .stateChange: return "State change"
            }
        }
    }

    public var name: String = "New rule"
    public var triggerKind: TriggerKind = .event
    public var triggerValue: String = "focus:session_completed"
    public var conditions: [BuilderCondition] = []
    public var actions: [BuilderAction] = []
    public var priority: Int32 = 50
    public var cooldownSeconds: Int64 = 0
    public var durationSeconds: Int64 = 3000 // 50 min default for Block
    public var enabled: Bool = true
    public var explanationTemplate: String =
        "{rule_name} fired on {event_type}:{event_id}."

    public init() {}

    func toRuleDraft() -> RuleDraft {
        RuleDraft(
            id: "rule-\(Int(Date().timeIntervalSince1970))",
            name: name,
            triggerEvent: triggerValue,
            actions: actions.compactMap { $0.toDto() },
            priority: priority,
            cooldownSeconds: cooldownSeconds > 0 ? cooldownSeconds : nil,
            durationSeconds: nil,
            explanationTemplate: explanationTemplate,
            enabled: enabled
        )
    }

    /// JSON preview — includes conditions + full action catalog even for
    /// variants not yet round-trippable through RuleDraft. Drives the Review
    /// pane.
    func toJsonPreview() -> String {
        var obj: [String: Any] = [
            "name": name,
            "trigger": [triggerKind.rawValue: triggerValue],
            "conditions": conditions.map { $0.toJsonObject() },
            "actions": actions.map { $0.toJsonObject() },
            "priority": priority,
            "enabled": enabled,
            "explanation_template": explanationTemplate,
        ]
        if cooldownSeconds > 0 { obj["cooldown_seconds"] = cooldownSeconds }
        guard let d = try? JSONSerialization.data(
            withJSONObject: obj, options: [.prettyPrinted, .sortedKeys]
        ), let s = String(data: d, encoding: .utf8) else {
            return "{}"
        }
        return s
    }
}

// MARK: - Step 1 — When

private struct StepWhenView: View {
    @Binding var model: BuilderModel
    let catalog: DslCatalog
    let onNext: () -> Void

    var body: some View {
        Form {
            Section("Name") {
                TextField("Rule name", text: $model.name)
            }

            Section("Trigger") {
                Picker("Kind", selection: $model.triggerKind) {
                    ForEach(BuilderModel.TriggerKind.allCases) { k in
                        Text(k.label).tag(k)
                    }
                }
                .pickerStyle(.segmented)

                switch model.triggerKind {
                case .event:
                    TextField("Event name", text: $model.triggerValue)
                        .autocorrectionDisabled()
                    if !eventSuggestions.isEmpty {
                        ScrollView(.horizontal, showsIndicators: false) {
                            HStack(spacing: 6) {
                                ForEach(eventSuggestions, id: \.self) { name in
                                    Button(name) { model.triggerValue = name }
                                        .buttonStyle(.bordered)
                                        .font(.caption)
                                }
                            }
                        }
                    }
                case .schedule:
                    TextField("Cron (sec min hour dom mon dow)",
                              text: $model.triggerValue)
                        .autocorrectionDisabled()
                    Text("Example: 0 0 9 * * *  — every day at 09:00 UTC.")
                        .font(.caption2)
                        .foregroundStyle(Color.app.foreground.opacity(0.6))
                case .stateChange:
                    TextField("State key (e.g. wallet.balance)",
                              text: $model.triggerValue)
                        .autocorrectionDisabled()
                }
            }

            Section {
                Button {
                    onNext()
                } label: {
                    Text("Next: If")
                        .frame(maxWidth: .infinity)
                }
                .disabled(model.triggerValue.isEmpty || model.name.isEmpty)
                .buttonStyle(.borderedProminent)
                .tint(Color.app.accent)
            }
        }
    }

    /// Well-known event suggestions; pulled from `WellKnownEventType` via the
    /// existing `WellKnownEvents.all` list. Safe to reference even when
    /// catalog isn't loaded.
    private var eventSuggestions: [String] { WellKnownEvents.all }
}

// MARK: - Step 2 — If (conditions)

private struct StepIfView: View {
    @Binding var model: BuilderModel
    let catalog: DslCatalog
    let onNext: () -> Void

    var body: some View {
        Form {
            Section {
                if model.conditions.isEmpty {
                    Text("No conditions — this rule fires on every trigger.")
                        .font(.callout)
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                }
                ForEach(model.conditions.indices, id: \.self) { idx in
                    ConditionRow(cond: $model.conditions[idx], catalog: catalog)
                }
                .onDelete { idxs in model.conditions.remove(atOffsets: idxs) }

                Menu("+ Add condition") {
                    ForEach(catalogConditionKinds, id: \.self) { kind in
                        Button(kind) {
                            model.conditions.append(BuilderCondition(kind: kind))
                        }
                    }
                }
            } header: {
                Text("Conditions (AND)")
            } footer: {
                Text("Every condition must match. Paths use dotted payload access, e.g. `assignment.late`.")
                    .font(.caption2)
            }

            Section {
                Button {
                    onNext()
                } label: { Text("Next: Then").frame(maxWidth: .infinity) }
                .buttonStyle(.borderedProminent)
                .tint(Color.app.accent)
            }
        }
        .navigationTitle("If")
    }

    private var catalogConditionKinds: [String] {
        if catalog.conditions.isEmpty {
            return BuilderCondition.fallbackKinds
        }
        return catalog.conditions.map { $0.kind }
    }
}

private struct ConditionRow: View {
    @Binding var cond: BuilderCondition
    let catalog: DslCatalog

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(cond.kind)
                .font(.callout.weight(.semibold))
                .foregroundStyle(Color.app.accent)

            let params = catalog.conditions
                .first(where: { $0.kind == cond.kind })?
                .params
                ?? BuilderCondition.defaultParams(for: cond.kind)
            ForEach(params, id: \.name) { p in
                paramField(for: p)
            }

            if let issue = cond.validationIssue {
                Text(issue).font(.caption2).foregroundStyle(.red)
            }
        }
        .padding(.vertical, 4)
    }

    @ViewBuilder
    private func paramField(for param: DslParam) -> some View {
        let binding = Binding<String>(
            get: { cond.paramString(param.name) },
            set: { cond.setParam(param.name, string: $0) }
        )
        HStack {
            Text(param.name).font(.caption).foregroundStyle(.secondary)
            Spacer()
            TextField(param.type, text: binding)
                .multilineTextAlignment(.trailing)
                .textFieldStyle(.roundedBorder)
                .frame(maxWidth: 200)
                .autocorrectionDisabled()
        }
    }
}

// MARK: - Step 3 — Then (actions)

private struct StepThenView: View {
    @Binding var model: BuilderModel
    let catalog: DslCatalog
    let onNext: () -> Void

    var body: some View {
        Form {
            Section {
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 8) {
                        ForEach(allActionKinds, id: \.self) { kind in
                            let supported = BuilderAction.isPersistable(kind: kind)
                            Button {
                                model.actions.append(BuilderAction.default(for: kind))
                            } label: {
                                HStack(spacing: 4) {
                                    Text(kind)
                                    if !supported {
                                        Image(systemName: "hourglass")
                                            .font(.caption2)
                                    }
                                }
                                .font(.caption.weight(.semibold))
                                .padding(.horizontal, 10)
                                .padding(.vertical, 6)
                                .background(
                                    Capsule().fill(
                                        supported
                                            ? Color.app.accent.opacity(0.18)
                                            : Color.app.foreground.opacity(0.08)
                                    )
                                )
                                .foregroundStyle(
                                    supported
                                        ? Color.app.accent
                                        : Color.app.foreground.opacity(0.6)
                                )
                            }
                            .buttonStyle(.plain)
                        }
                    }
                    .padding(.vertical, 4)
                }
            } header: {
                Text("Actions")
            } footer: {
                Text("Tap a chip to add. Variants marked with an hourglass are recognised by the DSL catalog but not yet persistable via the mobile FFI — they're kept in the JSON preview for forward compatibility.")
                    .font(.caption2)
            }

            Section {
                if model.actions.isEmpty {
                    Text("No actions yet. Tap a chip above.")
                        .font(.callout)
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                }
                ForEach(model.actions.indices, id: \.self) { idx in
                    ActionBuilderRow(action: $model.actions[idx])
                }
                .onDelete { idxs in model.actions.remove(atOffsets: idxs) }
            }

            Section {
                Button {
                    onNext()
                } label: { Text("Next: Settings").frame(maxWidth: .infinity) }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                    .disabled(model.actions.isEmpty)
            }
        }
        .navigationTitle("Then")
    }

    private var allActionKinds: [String] {
        if catalog.actions.isEmpty {
            return BuilderAction.fallbackKinds
        }
        return catalog.actions.map { $0.kind }
    }
}

private struct ActionBuilderRow: View {
    @Binding var action: BuilderAction

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(action.kind).font(.callout.weight(.semibold))
            ForEach(action.editableFields, id: \.name) { field in
                let binding = Binding<String>(
                    get: { action.fieldString(field.name) },
                    set: { action.setField(field.name, string: $0) }
                )
                HStack {
                    Text(field.name).font(.caption).foregroundStyle(.secondary)
                    Spacer()
                    TextField(field.type, text: binding)
                        .multilineTextAlignment(.trailing)
                        .textFieldStyle(.roundedBorder)
                        .frame(maxWidth: 200)
                        .autocorrectionDisabled()
                }
            }
        }
        .padding(.vertical, 4)
    }
}

// MARK: - Step 4 — Settings

private struct StepSettingsView: View {
    @Binding var model: BuilderModel
    let onNext: () -> Void

    var body: some View {
        Form {
            Section("Priority") {
                Slider(
                    value: Binding(
                        get: { Double(model.priority) },
                        set: { model.priority = Int32($0) }
                    ),
                    in: 0...100, step: 1
                )
                Text("Priority: \(model.priority) (higher fires first)")
                    .font(.caption)
            }

            Section("Cooldown") {
                Stepper(
                    value: Binding(
                        get: { Double(model.cooldownSeconds) },
                        set: { model.cooldownSeconds = Int64($0) }
                    ),
                    in: 0...(24 * 3600), step: 60
                ) {
                    Text(cooldownLabel)
                }
            }

            Section("Duration (for Block actions)") {
                Stepper(
                    value: Binding(
                        get: { Double(model.durationSeconds) },
                        set: {
                            model.durationSeconds = Int64($0)
                            // Mirror into any Block action the user already added.
                            for i in model.actions.indices
                            where model.actions[i].kind == "Block" {
                                model.actions[i].setField(
                                    "duration_seconds",
                                    string: String(model.durationSeconds)
                                )
                            }
                        }
                    ),
                    in: 60...(24 * 3600), step: 60
                ) {
                    Text("Duration: \(model.durationSeconds / 60) min")
                }
            }

            Section("State") {
                Toggle("Enabled", isOn: $model.enabled)
            }

            Section {
                TextField(
                    "Explanation template",
                    text: $model.explanationTemplate,
                    axis: .vertical
                )
                .lineLimit(2...5)
            } header: {
                Text("Explanation")
            } footer: {
                Text("Use {{event.payload.field}} placeholders to reference event data at fire time. {rule_name}, {event_type}, {event_id} are always substituted.")
                    .font(.caption2)
            }

            Section {
                Button {
                    onNext()
                } label: { Text("Next: Review").frame(maxWidth: .infinity) }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
            }
        }
        .navigationTitle("Settings")
    }

    private var cooldownLabel: String {
        if model.cooldownSeconds <= 0 { return "Cooldown: off" }
        let m = model.cooldownSeconds / 60
        if m < 60 { return "Cooldown: \(m) min" }
        return "Cooldown: \(m / 60) h \(m % 60) min"
    }
}

// MARK: - Step 5 — Review

private struct StepReviewView: View {
    @Binding var model: BuilderModel
    let saveError: String?
    @Binding var celebratory: Bool
    let onSave: () -> Void

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 14) {
                Text("Review")
                    .font(.title3.weight(.semibold))
                Text("RuleDraft JSON — this is exactly what will be persisted.")
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.7))

                Text(model.toJsonPreview())
                    .font(.system(.footnote, design: .monospaced))
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(12)
                    .background(
                        RoundedRectangle(cornerRadius: 12, style: .continuous)
                            .fill(Color.app.surface)
                    )

                if let e = saveError {
                    Text(e).font(.footnote).foregroundStyle(.red)
                }

                Button {
                    onSave()
                } label: { Text("Create").frame(maxWidth: .infinity) }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                    .disabled(model.actions.isEmpty || model.name.isEmpty)
            }
            .padding(16)
        }
        .navigationTitle("Review")
    }
}

// MARK: - BuilderCondition / BuilderAction

public struct BuilderCondition: Equatable {
    public var kind: String
    public var params: [String: String] = [:]

    public init(kind: String) {
        self.kind = kind
    }

    func paramString(_ name: String) -> String { params[name] ?? "" }
    mutating func setParam(_ name: String, string: String) {
        params[name] = string
    }

    var validationIssue: String? {
        let required = Self.defaultParams(for: kind).filter { $0.required }
        for p in required where paramString(p.name).isEmpty {
            return "Missing \(p.name)"
        }
        return nil
    }

    func toJsonObject() -> [String: Any] {
        var parsed: [String: Any] = [:]
        for (k, v) in params {
            if let num = Double(v), !v.isEmpty { parsed[k] = num; continue }
            if v == "true" { parsed[k] = true; continue }
            if v == "false" { parsed[k] = false; continue }
            parsed[k] = v
        }
        return ["kind": kind, "params": parsed]
    }

    static let fallbackKinds: [String] = [
        "confidence_gte", "payload_eq", "payload_in", "payload_gte",
        "payload_lte", "payload_exists", "payload_matches", "source_eq",
        "occurred_within", "all_of", "any_of", "not",
    ]

    static func defaultParams(for kind: String) -> [DslParam] {
        switch kind {
        case "confidence_gte":
            return [DslParam(name: "min", type: "number", required: true, description: nil)]
        case "payload_eq":
            return [
                DslParam(name: "path", type: "string", required: true, description: nil),
                DslParam(name: "value", type: "any", required: true, description: nil),
            ]
        case "payload_in":
            return [
                DslParam(name: "path", type: "string", required: true, description: nil),
                DslParam(name: "values", type: "array<any>", required: true, description: nil),
            ]
        case "payload_gte":
            return [
                DslParam(name: "path", type: "string", required: true, description: nil),
                DslParam(name: "min", type: "number", required: true, description: nil),
            ]
        case "payload_lte":
            return [
                DslParam(name: "path", type: "string", required: true, description: nil),
                DslParam(name: "max", type: "number", required: true, description: nil),
            ]
        case "payload_exists":
            return [DslParam(name: "path", type: "string", required: true, description: nil)]
        case "payload_matches":
            return [
                DslParam(name: "path", type: "string", required: true, description: nil),
                DslParam(name: "pattern", type: "string", required: true, description: nil),
            ]
        case "source_eq":
            return [DslParam(name: "source", type: "string", required: true, description: nil)]
        case "occurred_within":
            return [DslParam(name: "seconds", type: "number", required: true, description: nil)]
        case "all_of", "any_of":
            return [DslParam(name: "conditions", type: "array<condition>",
                             required: true, description: nil)]
        case "not":
            return [DslParam(name: "condition", type: "condition", required: true, description: nil)]
        default:
            return []
        }
    }
}

public struct BuilderAction: Equatable {
    public var kind: String
    public var fields: [String: String] = [:]

    public init(kind: String) { self.kind = kind }

    func fieldString(_ name: String) -> String { fields[name] ?? "" }
    mutating func setField(_ name: String, string: String) {
        fields[name] = string
    }

    var editableFields: [DslParam] {
        Self.editableSchema(for: kind)
    }

    /// Attempt to project into a persistable `RuleActionDto`. Returns nil for
    /// variants the FFI doesn't yet carry (they remain in the JSON preview).
    func toDto() -> RuleActionDto? {
        switch kind {
        case "GrantCredit":
            return .grantCredit(amount: int32("amount", default: 10))
        case "DeductCredit":
            return .deductCredit(amount: int32("amount", default: 10))
        case "Block":
            return .block(
                profile: fieldString("profile").isEmpty ? "social" : fieldString("profile"),
                durationSeconds: int64("duration_seconds", default: 3000)
            )
        case "Unblock":
            return .unblock(
                profile: fieldString("profile").isEmpty ? "social" : fieldString("profile")
            )
        case "StreakIncrement":
            return .streakIncrement(
                name: fieldString("name").isEmpty ? "focus" : fieldString("name")
            )
        case "StreakReset":
            return .streakReset(
                name: fieldString("name").isEmpty ? "focus" : fieldString("name")
            )
        case "Notify":
            return .notify(
                message: fieldString("message").isEmpty ? "Heads up!" : fieldString("message")
            )
        default:
            return nil // EmergencyExit / Intervention / ScheduledUnlockWindow
        }
    }

    func toJsonObject() -> [String: Any] {
        var parsed: [String: Any] = [:]
        for (k, v) in fields {
            if let num = Double(v), !v.isEmpty { parsed[k] = num; continue }
            if v == "true" { parsed[k] = true; continue }
            if v == "false" { parsed[k] = false; continue }
            parsed[k] = v
        }
        return [kind: parsed]
    }

    private func int32(_ name: String, default d: Int32) -> Int32 {
        Int32(fieldString(name)) ?? d
    }
    private func int64(_ name: String, default d: Int64) -> Int64 {
        Int64(fieldString(name)) ?? d
    }

    static func isPersistable(kind: String) -> Bool {
        [
            "GrantCredit", "DeductCredit", "Block", "Unblock",
            "StreakIncrement", "StreakReset", "Notify",
        ].contains(kind)
    }

    static let fallbackKinds: [String] = [
        "GrantCredit", "DeductCredit", "Block", "Unblock",
        "StreakIncrement", "StreakReset", "Notify",
        "EmergencyExit", "Intervention", "ScheduledUnlockWindow",
    ]

    static func `default`(for kind: String) -> BuilderAction {
        var b = BuilderAction(kind: kind)
        switch kind {
        case "GrantCredit", "DeductCredit":
            b.fields["amount"] = "10"
        case "Block":
            b.fields["profile"] = "social"
            b.fields["duration_seconds"] = "3000"
        case "Unblock":
            b.fields["profile"] = "social"
        case "StreakIncrement", "StreakReset":
            b.fields["name"] = "focus"
        case "Notify":
            b.fields["message"] = "Heads up!"
        case "EmergencyExit":
            b.fields["profiles"] = "social,games"
            b.fields["duration_seconds"] = "900"
            b.fields["bypass_cost"] = "25"
            b.fields["reason"] = ""
        case "Intervention":
            b.fields["message"] = ""
            b.fields["severity"] = "Gentle"
        case "ScheduledUnlockWindow":
            b.fields["profile"] = "social"
            b.fields["starts_at"] = ""
            b.fields["ends_at"] = ""
            b.fields["credit_cost"] = "10"
        default:
            break
        }
        return b
    }

    static func editableSchema(for kind: String) -> [DslParam] {
        switch kind {
        case "GrantCredit", "DeductCredit":
            return [DslParam(name: "amount", type: "integer", required: true, description: nil)]
        case "Block":
            return [
                DslParam(name: "profile", type: "string", required: true, description: nil),
                DslParam(name: "duration_seconds", type: "integer", required: true, description: nil),
            ]
        case "Unblock":
            return [DslParam(name: "profile", type: "string", required: true, description: nil)]
        case "StreakIncrement", "StreakReset":
            return [DslParam(name: "name", type: "string", required: true, description: nil)]
        case "Notify":
            return [DslParam(name: "message", type: "string", required: true, description: nil)]
        case "EmergencyExit":
            return [
                DslParam(name: "profiles", type: "csv<string>", required: true, description: nil),
                DslParam(name: "duration_seconds", type: "integer", required: true, description: nil),
                DslParam(name: "bypass_cost", type: "integer", required: true, description: nil),
                DslParam(name: "reason", type: "string", required: true, description: nil),
            ]
        case "Intervention":
            return [
                DslParam(name: "message", type: "string", required: true, description: nil),
                DslParam(name: "severity", type: "Gentle|Firm|Urgent", required: true, description: nil),
            ]
        case "ScheduledUnlockWindow":
            return [
                DslParam(name: "profile", type: "string", required: true, description: nil),
                DslParam(name: "starts_at", type: "iso8601", required: true, description: nil),
                DslParam(name: "ends_at", type: "iso8601", required: true, description: nil),
                DslParam(name: "credit_cost", type: "integer", required: true, description: nil),
            ]
        default:
            return []
        }
    }
}

// MARK: - DslCatalog (mirrors Rust-side `describe_dsl()` output)

public struct DslParam: Decodable, Equatable {
    public let name: String
    public let type: String
    public let required: Bool
    public let description: String?
}

public struct DslConditionSpec: Decodable, Equatable {
    public let kind: String
    public let params: [DslParam]
    public let description: String
}

public struct DslActionSpec: Decodable, Equatable {
    public let kind: String
    public let params: [DslParam]
    public let description: String
}

public struct DslTriggerSpec: Decodable, Equatable {
    public let kind: String
    public let params: [DslParam]
    public let description: String
}

public struct DslCatalog: Decodable, Equatable {
    public let triggers: [DslTriggerSpec]
    public let conditions: [DslConditionSpec]
    public let actions: [DslActionSpec]

    public static let empty = DslCatalog(triggers: [], conditions: [], actions: [])

    public init(triggers: [DslTriggerSpec], conditions: [DslConditionSpec], actions: [DslActionSpec]) {
        self.triggers = triggers
        self.conditions = conditions
        self.actions = actions
    }
}
#endif
