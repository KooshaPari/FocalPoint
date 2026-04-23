#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

public struct RulesListView: View {
    @EnvironmentObject private var holder: CoreHolder
    @State private var rules: [RuleSummary] = []
    @State private var showTemplates: Bool = false
    @State private var editing: EditingRule?

    private struct EditingRule: Identifiable {
        let rule: RuleSummary
        var id: String { rule.id }
    }
    @State private var creating: Bool = false
    @State private var loadError: String?

    public init() {}

    public var body: some View {
        NavigationStack {
            Group {
                if rules.isEmpty {
                    emptyState
                } else {
                    list
                }
            }
            .navigationTitle("Rules")
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button { creating = true } label: {
                        Image(systemName: "plus.circle.fill")
                    }
                }
                ToolbarItem(placement: .topBarLeading) {
                    Button("Templates") { showTemplates = true }
                }
            }
            .background(Color.app.background.ignoresSafeArea())
            .sheet(isPresented: $showTemplates) {
                RuleTemplateLibraryView().environmentObject(holder)
            }
            .sheet(item: $editing) { wrap in
                let rule = wrap.rule
                RuleDetailView(
                    mode: .edit(ruleId: rule.id),
                    seed: RuleDraft(
                        id: rule.id,
                        name: rule.name,
                        triggerEvent: "focus:session_started",
                        actions: [],
                        priority: rule.priority,
                        cooldownSeconds: nil,
                        durationSeconds: nil,
                        explanationTemplate: rule.explanationTemplate,
                        enabled: rule.enabled
                    )
                )
                .environmentObject(holder)
            }
            .sheet(isPresented: $creating) {
                RuleDetailView(mode: .create, seed: RuleDetailView.blankDraft())
                    .environmentObject(holder)
            }
        }
        .task(id: holder.revision) { await reload() }
    }

    private var list: some View {
        ScrollView {
            LazyVStack(spacing: 10) {
                ForEach(rules, id: \.id) { r in
                    RuleCard(
                        rule: r,
                        onToggle: { newValue in Task { await setEnabled(rule: r, enabled: newValue) } },
                        onOpen: { editing = EditingRule(rule: r) }
                    )
                }
            }
            .padding(16)
        }
    }

    private var emptyState: some View {
        VStack(spacing: 18) {
            Image(systemName: "list.bullet.rectangle")
                .font(.system(size: 56))
                .foregroundStyle(Color.app.accent)
            Text("No rules yet")
                .font(.title3.weight(.semibold))
                .foregroundStyle(Color.app.foreground)
            Text("Tap + to create one, or browse templates.")
                .font(.callout)
                .foregroundStyle(Color.app.foreground.opacity(0.7))
                .multilineTextAlignment(.center)
            HStack(spacing: 12) {
                Button("Browse templates") { showTemplates = true }
                    .buttonStyle(.bordered)
                    .tint(Color.app.accent)
                Button("New rule") { creating = true }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
            }
            if let err = loadError {
                Text(err).font(.caption2).foregroundStyle(.red)
            }
        }
        .padding(32)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    private func reload() async {
        do {
            let list = try holder.core.rules().listEnabled()
            await MainActor.run { self.rules = list; self.loadError = nil }
        } catch {
            await MainActor.run { self.loadError = "\(error)" }
        }
    }

    private func setEnabled(rule: RuleSummary, enabled: Bool) async {
        do {
            try holder.core.mutations().setEnabled(ruleId: rule.id, enabled: enabled)
            holder.bump()
        } catch {
            await MainActor.run { self.loadError = "\(error)" }
        }
    }
}

private struct RuleCard: View {
    let rule: RuleSummary
    let onToggle: (Bool) -> Void
    let onOpen: () -> Void

    var body: some View {
        Button(action: onOpen) {
            HStack(alignment: .top, spacing: 12) {
                VStack(alignment: .leading, spacing: 6) {
                    Text(rule.name)
                        .font(.body.weight(.semibold))
                        .foregroundStyle(Color.app.foreground)
                    HStack(spacing: 6) {
                        priorityBadge
                        Text(rule.explanationTemplate)
                            .font(.caption)
                            .foregroundStyle(Color.app.foreground.opacity(0.7))
                            .lineLimit(1)
                    }
                }
                Spacer()
                Toggle("", isOn: .init(
                    get: { rule.enabled },
                    set: { onToggle($0) }
                ))
                .labelsHidden()
                .tint(Color.app.accent)
                Image(systemName: "chevron.right")
                    .foregroundStyle(Color.app.foreground.opacity(0.3))
            }
            .padding(14)
            .background(
                RoundedRectangle(cornerRadius: 14, style: .continuous)
                    .fill(Color.app.surface)
            )
        }
        .buttonStyle(.plain)
    }

    private var priorityBadge: some View {
        Text("p\(rule.priority)")
            .font(.caption2.weight(.bold))
            .padding(.horizontal, 6)
            .padding(.vertical, 2)
            .background(
                Capsule().fill(Color.app.accent.opacity(0.2))
            )
            .foregroundStyle(Color.app.accent)
    }
}
#endif
