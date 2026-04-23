#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

struct SettingsView: View {
    @AppStorage("app.darkMode") private var darkMode: Bool = true
    @State private var canvasConnected: Bool = false

    var body: some View {
        NavigationStack {
            Form {
                Section("Theme") {
                    Toggle("Dark mode", isOn: $darkMode)
                        .tint(Color.app.accent)
                }
                Section("Canvas") {
                    Button(canvasConnected ? "Disconnect Canvas" : "Connect Canvas") {
                        canvasConnected.toggle()
                    }
                    .foregroundStyle(Color.app.accent)
                }
                Section("About") {
                    LabeledContent("Version", value: "0.0.1")
                    LabeledContent("Core", value: "placeholder")
                }
            }
            .navigationTitle("Settings")
        }
    }
}
#endif
