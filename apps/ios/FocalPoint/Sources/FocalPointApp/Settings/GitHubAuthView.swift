#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

/// GitHub connect flow — accepts a Personal Access Token.
///
/// GitHub doesn't need a full OAuth dance for read-only contribution data;
/// a scoped PAT is enough. The token is handed to
/// `FocalPointCore.connector().connectGithub(pat:)` which stores it in the
/// iOS keychain on the Rust side.
public struct GitHubAuthView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var pat: String = ""
    @State private var busy: Bool = false
    @State private var err: String?
    var onConnected: () -> Void

    public init(onConnected: @escaping () -> Void) {
        self.onConnected = onConnected
    }

    public var body: some View {
        NavigationStack {
            Form {
                Section {
                    SecureField("ghp_…", text: $pat)
                        .textInputAutocapitalization(.never)
                        .autocorrectionDisabled()
                } header: {
                    Text("Personal Access Token")
                } footer: {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Generate one at github.com/settings/tokens.")
                        Text("Scopes needed: public_repo, read:user (for contributions).")
                            .foregroundStyle(Color.app.foreground.opacity(0.6))
                    }
                    .font(.caption2)
                }

                Section {
                    Button {
                        Task { await connect() }
                    } label: {
                        HStack {
                            if busy { ProgressView() } else { Image(systemName: "chevron.left.forwardslash.chevron.right") }
                            Text(busy ? "Connecting…" : "Connect")
                        }
                        .frame(maxWidth: .infinity)
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                    .disabled(busy || pat.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                }
            }
            .navigationTitle("Connect GitHub")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
            .alert("Connection failed", isPresented: Binding(
                get: { err != nil },
                set: { if !$0 { err = nil } }
            )) {
                Button("OK", role: .cancel) { err = nil }
            } message: {
                Text(err ?? "")
            }
        }
    }

    private func connect() async {
        busy = true
        defer { busy = false }
        let trimmed = pat.trimmingCharacters(in: .whitespacesAndNewlines)
        do {
            try CoreHolder.shared.core.connector().connectGithub(pat: trimmed)
            onConnected()
            dismiss()
        } catch {
            err = "\(error.localizedDescription)"
        }
    }
}
#endif
