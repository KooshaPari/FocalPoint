#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Real wallet view — live balance, streaks, and a minimal "spend credits"
/// shop that exercises the existing `PenaltyApi::quote_bypass` +
/// `PenaltyApi::apply(SpendBypass)` surface, plus wallet mutations for
/// redemption purposes.
struct WalletView: View {
    @EnvironmentObject private var holder: CoreHolder
    @State private var wallet: WalletSummary?
    @State private var penalty: PenaltyStateSummary?
    @State private var loadError: String?
    @State private var spendError: String?
    @State private var lastSpendMessage: String?

    private let redemptions: [Redemption] = [
        .init(id: "break-15", title: "15-min social break", cost: 10, purpose: "redeem:break-15"),
        .init(id: "break-30", title: "30-min social break", cost: 20, purpose: "redeem:break-30"),
        .init(id: "no-lockout-1h", title: "Skip next 1h lockout", cost: 40, purpose: "redeem:skip-lockout-1h"),
        .init(id: "cheat-day", title: "Full cheat day (24h)", cost: 200, purpose: "redeem:cheat-day"),
    ]

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    header
                    balanceCard
                    streaksCard
                    shopSection
                    if let e = loadError {
                        Text(e).font(.caption).foregroundStyle(.red)
                    }
                }
                .padding()
            }
            .navigationTitle("Rewards")
            .background(Color.app.background.ignoresSafeArea())
            .task(id: holder.revision) { reload() }
        }
    }

    private var header: some View {
        HStack {
            CoachyView(
                state: CoachyState(pose: .encouraging, emotion: .happy, bubbleText: "Your credits."),
                size: 100
            )
            Spacer()
        }
    }

    private var balanceCard: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Balance").font(.caption.weight(.semibold)).foregroundStyle(Color.app.foreground.opacity(0.6))
            HStack(alignment: .firstTextBaseline) {
                Text("\(wallet?.balance ?? 0)")
                    .font(AppTypography.heroNumber)
                    .foregroundStyle(Color.app.accent)
                Text("credits").font(.body).foregroundStyle(Color.app.foreground.opacity(0.6))
            }
            HStack(spacing: 16) {
                Stat(label: "Earned", value: wallet?.earned ?? 0)
                Stat(label: "Spent", value: wallet?.spent ?? 0)
                if let m = wallet?.multiplier, m != 1.0 {
                    Stat(label: "Mult", value: Int(m * 100))
                }
            }
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    @ViewBuilder
    private var streaksCard: some View {
        if let streaks = wallet?.streaks, !streaks.isEmpty {
            VStack(alignment: .leading, spacing: 8) {
                Text("Streaks").font(.caption.weight(.semibold)).foregroundStyle(Color.app.foreground.opacity(0.6))
                ForEach(streaks, id: \.name) { s in
                    HStack {
                        Image(systemName: "flame.fill").foregroundStyle(.orange)
                        Text(s.name).font(.body)
                        Spacer()
                        Text("\(s.count) days").font(.body.weight(.semibold))
                    }
                }
            }
            .padding()
            .frame(maxWidth: .infinity, alignment: .leading)
            .background(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .fill(Color.app.surface)
            )
        }
    }

    private var shopSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Spend credits").font(.caption.weight(.semibold)).foregroundStyle(Color.app.foreground.opacity(0.6))
            ForEach(redemptions) { r in
                redemptionRow(r)
            }
            if let lastSpendMessage {
                Text(lastSpendMessage)
                    .font(.caption2)
                    .foregroundStyle(Color.app.accent)
            }
            if let spendError {
                Text(spendError).font(.caption2).foregroundStyle(.red)
            }
        }
    }

    private func redemptionRow(_ r: Redemption) -> some View {
        let canAfford = (wallet?.balance ?? 0) >= Int64(r.cost)
        return Button {
            spend(r)
        } label: {
            HStack {
                VStack(alignment: .leading, spacing: 2) {
                    Text(r.title).font(.body.weight(.semibold))
                    Text("\(r.cost) credits").font(.caption).foregroundStyle(Color.app.foreground.opacity(0.6))
                }
                Spacer()
                Image(systemName: canAfford ? "arrow.right.circle.fill" : "lock.fill")
                    .foregroundStyle(canAfford ? Color.app.accent : Color.app.foreground.opacity(0.3))
            }
            .padding()
            .frame(maxWidth: .infinity)
            .background(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .fill(Color.app.surface.opacity(canAfford ? 1.0 : 0.5))
            )
        }
        .buttonStyle(.plain)
        .disabled(!canAfford)
    }

    private func reload() {
        do {
            wallet = try holder.core.wallet().load()
            penalty = try? holder.core.penalty().load()
            loadError = nil
        } catch {
            loadError = "Load failed: \(error.localizedDescription)"
        }
    }

    private func spend(_ r: Redemption) {
        spendError = nil
        do {
            try holder.core.wallet().applyMutation(
                m: .spendCredit(amount: Int64(r.cost), purpose: r.purpose)
            )
            lastSpendMessage = "Redeemed: \(r.title)"
            holder.bump()
        } catch {
            spendError = "Spend failed: \(error.localizedDescription)"
        }
    }

    struct Redemption: Identifiable {
        let id: String
        let title: String
        let cost: Int
        let purpose: String
    }

    struct Stat: View {
        let label: String
        let value: Int64
        init(label: String, value: Int64) { self.label = label; self.value = value }
        init(label: String, value: Int) { self.label = label; self.value = Int64(value) }
        var body: some View {
            VStack(alignment: .leading, spacing: 2) {
                Text(label).font(.caption2).foregroundStyle(.secondary)
                Text("\(value)").font(.callout.weight(.semibold))
            }
        }
    }
}
#endif
