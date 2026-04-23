#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore
import MascotUI

/// Library of bundled starter-pack templates (full multi-rule packs) shipped
/// inside the Rust core via `include_str!`. Distinct from the legacy
/// `RuleTemplateLibraryView` (single hand-written rule drafts).
///
/// Flow:
///   Rules tab → "📚 Templates" → list of packs → tap card → detail →
///   "Install" → `core.templates().install(packId)` → toast + dismiss.
public struct TemplateLibraryView: View {
    @Environment(\.dismiss) private var dismiss
    @EnvironmentObject private var holder: CoreHolder

    @State private var packs: [TemplatePackSummary] = []
    @State private var loadError: String?
    @State private var toast: String?

    public init() {}

    public var body: some View {
        NavigationStack {
            ScrollView {
                LazyVStack(spacing: 12) {
                    if packs.isEmpty, loadError == nil {
                        ProgressView()
                            .padding(.top, 48)
                    }
                    ForEach(packs, id: \.id) { pack in
                        NavigationLink {
                            TemplatePackDetailView(
                                pack: pack,
                                onInstalled: { n in
                                    toast = "\(n) rule\(n == 1 ? "" : "s") installed"
                                    scheduleToastDismiss()
                                    dismiss()
                                }
                            )
                            .environmentObject(holder)
                        } label: {
                            TemplatePackCard(pack: pack)
                        }
                        .buttonStyle(.plain)
                    }
                    if let err = loadError {
                        Text(err)
                            .font(.caption2)
                            .foregroundStyle(.red)
                            .padding(.top, 12)
                    }
                }
                .padding(16)
            }
            .background(Color.app.background.ignoresSafeArea())
            .navigationTitle("Starter Packs")
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button("Done") { dismiss() }
                }
                ToolbarItem(placement: .topBarLeading) {
                    CoachyView(
                        state: CoachyState(
                            pose: .encouraging,
                            emotion: .happy,
                            bubbleText: nil
                        ),
                        size: 32
                    )
                }
            }
            .overlay(alignment: .bottom) { toastView }
        }
        .task { reload() }
    }

    private var toastView: some View {
        Group {
            if let t = toast {
                Text(t)
                    .font(.footnote.weight(.semibold))
                    .padding(.horizontal, 14)
                    .padding(.vertical, 8)
                    .background(Capsule().fill(Color.app.accent))
                    .foregroundStyle(.white)
                    .padding(.bottom, 24)
                    .transition(.opacity)
            }
        }
    }

    private func reload() {
        let list = holder.core.templates().listBundled()
        self.packs = list
        self.loadError = list.isEmpty ? "No bundled packs found." : nil
    }

    private func scheduleToastDismiss() {
        Task { @MainActor in
            try? await Task.sleep(nanoseconds: 1_500_000_000)
            toast = nil
        }
    }
}

// MARK: - Pack Card

private struct TemplatePackCard: View {
    let pack: TemplatePackSummary

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack(alignment: .firstTextBaseline) {
                Text(pack.name)
                    .font(.body.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)
                Spacer()
                Text("v\(pack.version)")
                    .font(.caption2.monospaced())
                    .foregroundStyle(Color.app.foreground.opacity(0.5))
            }
            Text(pack.description)
                .font(.caption)
                .foregroundStyle(Color.app.foreground.opacity(0.75))
                .multilineTextAlignment(.leading)
            HStack(spacing: 6) {
                badge(text: "\(pack.ruleCount) rule\(pack.ruleCount == 1 ? "" : "s")")
                ForEach(pack.recommendedConnectors, id: \.self) { c in
                    connectorChip(c)
                }
            }
            HStack {
                Spacer()
                Image(systemName: "chevron.right")
                    .foregroundStyle(Color.app.foreground.opacity(0.3))
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

    private func connectorChip(_ id: String) -> some View {
        Text(id)
            .font(.caption2.weight(.semibold).monospaced())
            .padding(.horizontal, 7)
            .padding(.vertical, 2)
            .background(Capsule().fill(Color.app.accent.opacity(0.15)))
            .foregroundStyle(Color.app.accent)
    }
}

// MARK: - Detail

private struct TemplatePackDetailView: View {
    @EnvironmentObject private var holder: CoreHolder
    let pack: TemplatePackSummary
    let onInstalled: (UInt32) -> Void

    @State private var installing: Bool = false
    @State private var installError: String?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                header
                metaRow
                Text(pack.description)
                    .font(.body)
                    .foregroundStyle(Color.app.foreground.opacity(0.85))
                if !pack.recommendedConnectors.isEmpty {
                    VStack(alignment: .leading, spacing: 6) {
                        Text("Recommended connectors")
                            .font(.caption.weight(.semibold))
                            .foregroundStyle(Color.app.foreground.opacity(0.7))
                        HStack(spacing: 6) {
                            ForEach(pack.recommendedConnectors, id: \.self) { c in
                                Text(c)
                                    .font(.caption.monospaced())
                                    .padding(.horizontal, 8)
                                    .padding(.vertical, 3)
                                    .background(Capsule().fill(Color.app.accent.opacity(0.15)))
                                    .foregroundStyle(Color.app.accent)
                            }
                        }
                    }
                }
                ruleCountBanner
                if let err = installError {
                    Text(err).font(.caption).foregroundStyle(.red)
                }
                installButton
                    .padding(.top, 6)
            }
            .padding(16)
        }
        .background(Color.app.background.ignoresSafeArea())
        .navigationTitle(pack.name)
        .navigationBarTitleDisplayMode(.inline)
    }

    private var header: some View {
        HStack(alignment: .top, spacing: 12) {
            CoachyView(
                state: CoachyState(pose: .encouraging, emotion: .happy, bubbleText: nil),
                size: 56
            )
            VStack(alignment: .leading, spacing: 4) {
                Text(pack.name)
                    .font(.title3.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)
                Text("by \(pack.author) · v\(pack.version)")
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
            }
        }
    }

    private var metaRow: some View {
        HStack(spacing: 8) {
            Label("\(pack.ruleCount) rule\(pack.ruleCount == 1 ? "" : "s")", systemImage: "list.bullet.rectangle")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.75))
        }
    }

    private var ruleCountBanner: some View {
        HStack {
            Image(systemName: "sparkles")
                .foregroundStyle(Color.app.accent)
            Text("This pack installs \(pack.ruleCount) rule\(pack.ruleCount == 1 ? "" : "s") into your rule list.")
                .font(.callout)
                .foregroundStyle(Color.app.foreground.opacity(0.85))
            Spacer()
        }
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 12, style: .continuous)
                .fill(Color.app.accent.opacity(0.08))
        )
    }

    private var installButton: some View {
        Button {
            install()
        } label: {
            HStack {
                if installing {
                    ProgressView()
                        .tint(.white)
                }
                Text(installing ? "Installing…" : "Install pack")
                    .font(.body.weight(.semibold))
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 12)
        }
        .buttonStyle(.borderedProminent)
        .tint(Color.app.accent)
        .disabled(installing)
    }

    private func install() {
        installing = true
        installError = nil
        do {
            let n = try holder.core.templates().install(packId: pack.id)
            holder.bump()
            installing = false
            onInstalled(n)
        } catch {
            installing = false
            installError = "Install failed: \(error)"
        }
    }
}
#endif
