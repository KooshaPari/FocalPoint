#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore

/// Sheet displaying all available keyboard shortcuts.
public struct KeyboardShortcutsSheetView: View {
    @Environment(\.dismiss) var dismiss

    public init() {}

    public var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    // Global shortcuts section
                    Section {
                        VStack(alignment: .leading, spacing: 12) {
                            ForEach(AVAILABLE_SHORTCUTS.filter { $0.context == "Global" }, id: \.action) { shortcut in
                                KeyboardShortcutRow(shortcut: shortcut)
                            }
                        }
                    } header: {
                        Text("Global Shortcuts")
                            .font(.headline)
                            .foregroundStyle(Color.app.accent)
                            .padding(.top, 12)
                    }

                    Divider()

                    // Focus tab specific shortcuts
                    Section {
                        VStack(alignment: .leading, spacing: 12) {
                            ForEach(AVAILABLE_SHORTCUTS.filter { $0.context == "Focus Tab" }, id: \.action) { shortcut in
                                KeyboardShortcutRow(shortcut: shortcut)
                            }
                        }
                    } header: {
                        Text("Focus Tab Shortcuts")
                            .font(.headline)
                            .foregroundStyle(Color.app.accent)
                    }

                    Divider()

                    // Help text
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Tips")
                            .font(.headline)
                            .foregroundStyle(Color.app.accent)

                        Text("Keyboard shortcuts are available on iOS with external keyboards, iPad, and Mac (Designed for iPad).")
                            .font(.caption)
                            .foregroundStyle(.secondary)

                        Text("Menu commands are available in the Mac menu bar when running on macOS.")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    .padding(.vertical, 12)

                    Spacer()
                }
                .padding()
            }
            .navigationTitle("Keyboard Shortcuts")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
            .background(Color.app.background.ignoresSafeArea())
        }
    }
}

/// Single keyboard shortcut row showing key combo and action description.
struct KeyboardShortcutRow: View {
    let shortcut: KeyboardShortcut

    var body: some View {
        HStack(spacing: 16) {
            VStack(alignment: .leading, spacing: 2) {
                Text(shortcut.action)
                    .font(.body.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)

                if shortcut.context != "Global" {
                    Text(shortcut.context)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }

            Spacer()

            // Display the key combination
            HStack(spacing: 4) {
                if !shortcut.modifiers.isEmpty {
                    KeyBadge(label: shortcut.modifiers)
                }
                KeyBadge(label: shortcut.key)
            }
        }
        .padding(.vertical, 8)
        .padding(.horizontal, 12)
        .background(Color.app.foreground.opacity(0.05))
        .cornerRadius(8)
    }
}

/// Styled badge for displaying a key on the keyboard.
struct KeyBadge: View {
    let label: String

    var body: some View {
        Text(label)
            .font(.system(.caption, design: .monospaced))
            .fontWeight(.semibold)
            .foregroundStyle(Color.app.accent)
            .padding(.horizontal, 8)
            .padding(.vertical, 4)
            .background(Color.app.accent.opacity(0.1))
            .cornerRadius(4)
            .border(Color.app.accent.opacity(0.3), width: 0.5)
    }
}

#Preview {
    KeyboardShortcutsSheetView()
        .preferredColorScheme(.dark)
        .background(Color.app.background)
}
#endif
