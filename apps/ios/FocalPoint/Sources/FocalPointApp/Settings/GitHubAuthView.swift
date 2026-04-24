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
        if busy {
            coachyConnectingView(provider: "GitHub")
        } else {
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
                                Image(systemName: "chevron.left.forwardslash.chevron.right")
                                Text("Connect")
                            }
                            .frame(maxWidth: .infinity)
                        }
                        .buttonStyle(.borderedProminent)
                        .tint(Color.app.accent)
                        .disabled(pat.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
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
    }

    @ViewBuilder
    private func coachyConnectingView(provider: String) -> some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [.blue.opacity(0.1), .purple.opacity(0.1)]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            VStack(spacing: 20) {
                CoachyView(
                    state: CoachyState(
                        pose: .encouraging,
                        emotion: .happy,
                        bubbleText: "Connecting to \(provider)…"
                    ),
                    size: 200
                )
                ProgressView()
                    .controlSize(.large)
            }
            .padding()
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
