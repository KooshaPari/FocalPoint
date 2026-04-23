#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

public struct RuleTemplateLibraryView: View {
    @Environment(\.dismiss) private var dismiss
    @EnvironmentObject private var holder: CoreHolder
    @State private var toast: String?

    public init() {}

    public var body: some View {
        NavigationStack {
            ScrollView {
                LazyVStack(spacing: 12) {
                    ForEach(RuleTemplates.all) { t in
                        TemplateCard(template: t) {
                            install(t)
                        }
                    }
                }
                .padding(16)
            }
            .background(Color.app.background.ignoresSafeArea())
            .navigationTitle("Templates")
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button("Done") { dismiss() }
                }
            }
            .overlay(alignment: .bottom) {
                if let t = toast {
                    Text(t)
                        .font(.footnote.weight(.semibold))
                        .padding(.horizontal, 14)
                        .padding(.vertical, 8)
                        .background(
                            Capsule().fill(Color.app.accent)
                        )
                        .foregroundStyle(.white)
                        .padding(.bottom, 24)
                        .transition(.opacity)
                }
            }
        }
    }

    private func install(_ t: RuleTemplate) {
        do {
            try holder.core.mutations().upsert(rule: t.draft)
            holder.bump()
            toast = "Installed: \(t.title)"
            Task { @MainActor in
                try? await Task.sleep(nanoseconds: 1_500_000_000)
                toast = nil
            }
        } catch {
            toast = "Install failed: \(error)"
        }
    }
}

private struct TemplateCard: View {
    let template: RuleTemplate
    let onInstall: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text(template.title)
                .font(.body.weight(.semibold))
                .foregroundStyle(Color.app.foreground)
            Text(template.subtitle)
                .font(.caption)
                .foregroundStyle(Color.app.foreground.opacity(0.7))
            HStack(spacing: 6) {
                badge(text: "trigger: \(template.draft.triggerEvent)")
                badge(text: "p\(template.draft.priority)")
                badge(text: "\(template.draft.actions.count) action\(template.draft.actions.count == 1 ? "" : "s")")
            }
            HStack {
                Spacer()
                Button("Install", action: onInstall)
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
            }
        }
        .padding(14)
        .background(
            RoundedRectangle(cornerRadius: 14, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func badge(text: String) -> some View {
        Text(text)
            .font(.caption2.weight(.semibold))
            .padding(.horizontal, 6)
            .padding(.vertical, 2)
            .background(Capsule().fill(Color.app.foreground.opacity(0.08)))
            .foregroundStyle(Color.app.foreground.opacity(0.8))
    }
}
#endif
