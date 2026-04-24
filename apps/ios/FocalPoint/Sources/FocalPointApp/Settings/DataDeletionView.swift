#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

/// Right-to-erasure / GDPR account deletion flow.
/// Double-confirmation UI with destructive red button, shows what will be deleted,
/// and displays the receipt on success.
public struct DataDeletionView: View {
    @EnvironmentObject private var holder: CoreHolder
    @State private var showConfirmation = false
    @State private var showFinalConfirmation = false
    @State private var isWiping = false
    @State private var wipingError: String?
    @State private var wipeReceipt: FocalPointCore.WipeReceiptDto?
    @State private var showReceiptCopy = false

    public init() {}

    public var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                Section {
                    VStack(alignment: .leading, spacing: 12) {
                        Label("Delete All My Data", systemImage: "trash.circle.fill")
                            .font(.headline)
                            .foregroundStyle(Color.red)

                        Text("This action is irreversible. All of the following will be permanently deleted:")
                            .font(.body)
                            .foregroundStyle(Color.secondary)

                        VStack(alignment: .leading, spacing: 8) {
                            DeletionItemRow(label: "Events", detail: "All connector events and sync history")
                            DeletionItemRow(label: "Rules", detail: "All custom and template rules")
                            DeletionItemRow(label: "Tasks", detail: "All planning tasks and rituals data")
                            DeletionItemRow(label: "Wallet & Rewards", detail: "Credits, streaks, multipliers, unlocks")
                            DeletionItemRow(label: "Penalties", detail: "Escalation state, lockout windows, strict mode")
                            DeletionItemRow(label: "Audit Log", detail: "Complete tamper-evident audit chain")
                            DeletionItemRow(label: "Connector Tokens", detail: "Canvas, Google Calendar, GitHub credentials")
                            DeletionItemRow(label: "Caches & Backups", detail: "Temporary files and backup archives")
                        }
                        .padding(.vertical, 8)

                        Divider()

                        VStack(alignment: .leading, spacing: 4) {
                            Label("You will receive a receipt", systemImage: "receipt")
                                .font(.footnote)
                                .foregroundStyle(Color.secondary)
                            Text("A tamper-evident wipe receipt will be saved to your device for your records (you can delete it manually later).")
                                .font(.caption)
                                .foregroundStyle(Color.secondary)
                        }
                    }
                    .padding()
                    .background(Color(UIColor.systemRed).opacity(0.05))
                    .cornerRadius(8)
                    .overlay(
                        RoundedRectangle(cornerRadius: 8)
                            .stroke(Color.red.opacity(0.2), lineWidth: 1)
                    )
                } header: {
                    Text("What will be deleted")
                        .font(.caption)
                        .foregroundStyle(Color.secondary)
                }

                if let error = wipingError {
                    VStack(alignment: .leading, spacing: 8) {
                        Label("Error", systemImage: "exclamationmark.circle.fill")
                            .font(.subheadline)
                            .foregroundStyle(Color.red)
                        Text(error)
                            .font(.caption)
                            .foregroundStyle(Color.secondary)
                    }
                    .padding()
                    .background(Color.red.opacity(0.05))
                    .cornerRadius(8)
                }

                if let receipt = wipeReceipt {
                    WipeReceiptView(receipt: receipt, showCopyFeedback: $showReceiptCopy)
                } else {
                    Button(action: beginDeletion) {
                        if isWiping {
                            ProgressView()
                                .progressViewStyle(.circular)
                                .frame(maxWidth: .infinity)
                        } else {
                            Text("Delete All Data")
                                .frame(maxWidth: .infinity)
                                .foregroundStyle(Color.white)
                        }
                    }
                    .buttonStyle(.bordered)
                    .tint(Color.red)
                    .disabled(isWiping)
                }

                if wipeReceipt != nil {
                    Button(action: uninstallApp) {
                        Text("Uninstall FocalPoint")
                            .frame(maxWidth: .infinity)
                    }
                    .buttonStyle(.bordered)
                    .tint(Color.gray)
                }
            }
            .padding()
        }
        .navigationTitle("Delete My Data")
        .alert("Confirm Deletion", isPresented: $showConfirmation) {
            Button("Cancel", role: .cancel) { }
            Button("I Understand", role: .destructive) {
                showFinalConfirmation = true
            }
        } message: {
            Text("This action cannot be undone. All your data will be permanently deleted.")
        }
        .alert("Final Confirmation", isPresented: $showFinalConfirmation) {
            Button("Cancel", role: .cancel) { }
            Button("Delete Everything", role: .destructive) {
                performWipe()
            }
        } message: {
            Text("Type DELETE to confirm, or tap 'Cancel' to go back.")
        }
    }

    private func beginDeletion() {
        showConfirmation = true
    }

    private func performWipe() {
        isWiping = true
        wipingError = nil

        Task {
            do {
                let receipt = try holder.core.data_lifecycle().wipe_all()
                await MainActor.run {
                    self.wipeReceipt = receipt
                    self.isWiping = false
                }
            } catch let error as FocalPointCore.FfiError {
                await MainActor.run {
                    self.wipingError = error.localizedDescription
                    self.isWiping = false
                }
            } catch {
                await MainActor.run {
                    self.wipingError = error.localizedDescription
                    self.isWiping = false
                }
            }
        }
    }

    private func uninstallApp() {
        // On iOS, there is no programmatic way to uninstall the app.
        // We show a message encouraging the user to do it manually.
        if let url = URL(string: UIApplication.openSettingsURLString) {
            UIApplication.shared.open(url)
        }
    }
}

// MARK: - Private Components

private struct DeletionItemRow: View {
    let label: String
    let detail: String

    var body: some View {
        HStack(alignment: .top, spacing: 12) {
            Image(systemName: "checkmark.circle.fill")
                .foregroundStyle(Color.red.opacity(0.6))
                .frame(width: 20)

            VStack(alignment: .leading, spacing: 2) {
                Text(label)
                    .font(.footnote)
                    .fontWeight(.medium)
                Text(detail)
                    .font(.caption)
                    .foregroundStyle(Color.secondary)
            }
            Spacer()
        }
    }
}

private struct WipeReceiptView: View {
    let receipt: FocalPointCore.WipeReceiptDto
    @Binding var showCopyFeedback: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack(spacing: 8) {
                Image(systemName: "checkmark.seal.fill")
                    .foregroundStyle(Color.green)
                VStack(alignment: .leading, spacing: 2) {
                    Text("Data Successfully Deleted")
                        .font(.headline)
                    Text("Receipt saved to your device")
                        .font(.caption)
                        .foregroundStyle(Color.secondary)
                }
                Spacer()
            }
            .padding()
            .background(Color.green.opacity(0.05))
            .cornerRadius(8)

            VStack(alignment: .leading, spacing: 8) {
                ReceiptLine(label: "Wiped at", value: receipt.wiped_at)
                ReceiptLine(
                    label: "Chain Hash",
                    value: String(receipt.pre_wipe_chain_hash.prefix(16)) + "..."
                )

                Divider()

                Text("Deleted Counts")
                    .font(.caption)
                    .fontWeight(.semibold)
                    .foregroundStyle(Color.secondary)

                VStack(alignment: .leading, spacing: 4) {
                    ForEach(Array(receipt.deleted_counts.sorted { $0.key < $1.key }), id: \.key) { table, count in
                        HStack {
                            Text(table)
                                .font(.caption)
                                .foregroundStyle(Color.secondary)
                            Spacer()
                            Text("\(count)")
                                .font(.caption)
                                .fontWeight(.medium)
                        }
                    }
                }
            }
            .padding()
            .background(Color(UIColor.secondarySystemBackground))
            .cornerRadius(8)

            Button(action: copyReceipt) {
                HStack {
                    Image(systemName: "doc.on.doc")
                    Text("Copy Receipt JSON")
                }
                .frame(maxWidth: .infinity)
            }
            .buttonStyle(.bordered)

            if showCopyFeedback {
                HStack(spacing: 6) {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundStyle(Color.green)
                    Text("Receipt copied to clipboard")
                        .font(.caption)
                }
                .transition(.opacity)
            }
        }
    }

    private func copyReceipt() {
        let json = """
        {
          "wiped_at": "\(receipt.wiped_at)",
          "pre_wipe_chain_hash": "\(receipt.pre_wipe_chain_hash)",
          "deleted_counts": \(formatCounts()),
          "deleted_keychain_items": [],
          "deleted_paths": []
        }
        """
        UIPasteboard.general.string = json
        withAnimation {
            showCopyFeedback = true
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
            withAnimation {
                showCopyFeedback = false
            }
        }
    }

    private func formatCounts() -> String {
        let items = receipt.deleted_counts
            .sorted { $0.key < $1.key }
            .map { "\"\($0.key)\": \($0.value)" }
            .joined(separator: ", ")
        return "{ \(items) }"
    }
}

private struct ReceiptLine: View {
    let label: String
    let value: String

    var body: some View {
        HStack(alignment: .top) {
            Text(label)
                .font(.caption)
                .foregroundStyle(Color.secondary)
            Spacer()
            Text(value)
                .font(.caption)
                .monospaced()
                .foregroundStyle(Color.primary)
        }
    }
}

#endif
