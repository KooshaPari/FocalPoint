#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Real-state-driven home view. Reads wallet, penalty, rules, and mascot
/// state from the core on appear + on every `holder.revision` bump.
struct HomeView: View {
    @EnvironmentObject private var holder: CoreHolder

    @State private var wallet: WalletSummary?
    @State private var penalty: PenaltyStateSummary?
    @State private var topRule: RuleSummary?
    @State private var coachy: CoachyState = .placeholder
    @State private var loadError: String?

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    ruleCard

                    CoachyView(state: coachy, size: 220)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(
                            RoundedRectangle(cornerRadius: 20, style: .continuous)
                                .fill(Color.app.surface)
                        )

                    statsStrip

                    if let e = loadError {
                        Text(e).font(.caption2).foregroundStyle(.red)
                    }
                }
                .padding()
            }
            .navigationTitle("FocalPoint")
            .background(Color.app.background.ignoresSafeArea())
        }
        .task(id: holder.revision) { await reload() }
    }

    // MARK: - Rule card

    @ViewBuilder
    private var ruleCard: some View {
        if let rule = topRule {
            VStack(alignment: .leading, spacing: 8) {
                Text("Active rule")
                    .font(.caption.weight(.semibold))
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                Text(rule.name)
                    .font(.title3.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)
                HStack(spacing: 6) {
                    Image(systemName: "bolt.fill")
                    Text("Priority \(rule.priority)")
                }
                .font(.subheadline.weight(.medium))
                .foregroundStyle(Color.app.accent)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(18)
            .background(
                RoundedRectangle(cornerRadius: 20, style: .continuous)
                    .fill(Color.app.surface)
            )
            .overlay(
                RoundedRectangle(cornerRadius: 20, style: .continuous)
                    .strokeBorder(Color.app.accent.opacity(0.25), lineWidth: 1)
            )
        } else {
            Text("No active rule")
                .foregroundStyle(Color.app.foreground.opacity(0.7))
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(18)
                .background(
                    RoundedRectangle(cornerRadius: 20, style: .continuous)
                        .fill(Color.app.surface)
                )
        }
    }

    // MARK: - Stats

    private var statsStrip: some View {
        HStack(spacing: 12) {
            statChip(icon: "flame.fill", label: "Streak", value: streakValue)
            statChip(icon: "diamond.fill", label: "Credits", value: creditsValue)
            statChip(icon: "lock.shield.fill", label: "Bypass", value: bypassValue)
        }
    }

    private var streakValue: String {
        guard let w = wallet, let first = w.streaks.first else { return "—" }
        return "\(first.count)"
    }
    private var creditsValue: String {
        guard let w = wallet else { return "—" }
        return "\(w.balance)"
    }
    private var bypassValue: String {
        guard let p = penalty else { return "—" }
        return "\(p.bypassBudget)"
    }

    private func statChip(icon: String, label: String, value: String) -> some View {
        VStack(spacing: 4) {
            Image(systemName: icon)
                .font(.system(size: 18))
                .foregroundStyle(Color.app.accent)
            Text(value)
                .font(.title3.weight(.bold))
                .foregroundStyle(Color.app.foreground)
            Text(label)
                .font(.caption)
                .foregroundStyle(Color.app.foreground.opacity(0.7))
        }
        .frame(maxWidth: .infinity)
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    // MARK: - Loader

    @MainActor
    private func reload() async {
        // Push a DailyCheckIn so Coachy has a real mood.
        let state = holder.core.pushMascotEvent(event: .dailyCheckIn)
        withAnimation(.easeInOut(duration: 0.4)) {
            coachy = CoachyBridging.coachyState(from: state)
        }

        do {
            wallet = try holder.core.wallet().load()
        } catch { loadError = "wallet: \(error)" }

        do {
            penalty = try holder.core.penalty().load()
        } catch { loadError = "penalty: \(error)" }

        do {
            let all = try holder.core.rules().listEnabled()
            topRule = all.max(by: { $0.priority < $1.priority })
        } catch { loadError = "rules: \(error)" }
    }
}
#endif
