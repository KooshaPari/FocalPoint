#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

struct RulesView: View {
    @State private var rules: [String] = ["Deep work — no social", "Evening wind-down"]

    var body: some View {
        NavigationStack {
            List {
                Section("Rules") {
                    ForEach(rules, id: \.self) { rule in
                        Text(rule).foregroundStyle(Color.app.foreground)
                    }
                }
                Section {
                    Button {
                        rules.append("New rule \(rules.count + 1)")
                    } label: {
                        Label("Add rule", systemImage: "plus.circle.fill")
                            .foregroundStyle(Color.app.accent)
                    }
                }
            }
            .navigationTitle("Rules")
        }
    }
}
#endif
