#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

struct HomeView: View {
    @State private var activeRule: ActiveRule? = ActiveRule(
        id: RuleId("demo"),
        title: "Deep work — no social",
        endsAt: Date().addingTimeInterval(45 * 60)
    )

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    ruleCard
                    CoachyView(state: .placeholder)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(
                            RoundedRectangle(cornerRadius: 20, style: .continuous)
                                .fill(Color.app.surface)
                        )
                }
                .padding()
            }
            .navigationTitle("FocalPoint")
            .background(Color.app.background.ignoresSafeArea())
        }
    }

    @ViewBuilder
    private var ruleCard: some View {
        if let rule = activeRule {
            VStack(alignment: .leading, spacing: 8) {
                Text("Active rule")
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                Text(rule.title)
                    .font(.title3.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)
                if let endsAt = rule.endsAt {
                    Text("Ends \(endsAt.formatted(date: .omitted, time: .shortened))")
                        .font(.subheadline)
                        .foregroundStyle(Color.app.accent)
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 20, style: .continuous)
                    .fill(Color.app.surface)
            )
        } else {
            Text("No active rule")
                .foregroundStyle(Color.app.foreground.opacity(0.7))
        }
    }
}
#endif
