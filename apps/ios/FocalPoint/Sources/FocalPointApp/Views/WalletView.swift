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
            Text(String(localized: "Balance", defaultValue: "Balance"))
                .font(.caption.weight(.semibold)).foregroundStyle(Color.app.foreground.opacity(0.6))
            HStack(alignment: .firstTextBaseline) {
                Text("\(wallet?.balance ?? 0)")
                    .font(AppTypography.heroNumber)
                    .foregroundStyle(Color.app.accent)
                Text(String(localized: "credits", defaultValue: "credits"))
                    .font(.body).foregroundStyle(Color.app.foreground.opacity(0.6))
            }
            HStack(spacing: 16) {
                Stat(label: String(localized: "Earned", defaultValue: "Earned"), value: wallet?.earned ?? 0)
                Stat(label: String(localized: "Spent", defaultValue: "Spent"), value: wallet?.spent ?? 0)
                if let m = wallet?.multiplier, m != 1.0 {
                    Stat(label: String(localized: "Mult", defaultValue: "Mult"), value: Int(m * 100))
                }
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Balance: \(wallet?.balance ?? 0) credits", defaultValue: "Balance: \(wallet?.balance ?? 0) credits"))
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
                .accessibilityHidden(true)
        )
    }

    @ViewBuilder
    private var streaksCard: some View {
        if let streaks = wallet?.streaks, !streaks.isEmpty {
            VStack(alignment: .leading, spacing: 8) {
                Text(String(localized: "Streaks", defaultValue: "Streaks"))
                    .font(.caption.weight(.semibold)).foregroundStyle(Color.app.foreground.opacity(0.6))
                ForEach(streaks, id: \.name) { s in
                    HStack {
                        Image(systemName: "flame.fill")
                            .foregroundStyle(Color.app.accent)
                            .accessibilityHidden(true)
                        Text(s.name).font(.body)
                        Spacer()
                        Text(String(localized: "\(s.count) days", defaultValue: "\(s.count) days"))
                            .font(.body.weight(.semibold))
                    }
                    .accessibilityElement(children: .combine)
                    .accessibilityLabel(String(localized: "\(s.name): \(s.count) days", defaultValue: "\(s.name): \(s.count) days"))
                }
            }
            .accessibilityElement(children: .combine)
            .accessibilityLabel(String(localized: "Active streaks", defaultValue: "Active streaks"))
            .padding()
            .frame(maxWidth: .infinity, alignment: .leading)
            .background(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .fill(Color.app.surface)
                    .accessibilityHidden(true)
            )
        }
    }

    private var shopSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(String(localized: "Spend credits", defaultValue: "Spend credits"))
                .font(.caption.weight(.semibold)).foregroundStyle(Color.app.foreground.opacity(0.6))
            ForEach(redemptions) { r in
                redemptionRow(r)
            }
            if let lastSpendMessage {
                Text(lastSpendMessage)
                    .font(.caption2)
                    .foregroundStyle(Color.app.accent)
                    .accessibilityLiveRegion(.polite)
            }
            if let spendError {
                Text(spendError).font(.caption2).foregroundStyle(.red)
                    .accessibilityLiveRegion(.assertive)
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: "Redeem credits", defaultValue: "Redeem credits"))
    }

    private func redemptionRow(_ r: Redemption) -> some View {
        let canAfford = (wallet?.balance ?? 0) >= Int64(r.cost)
        return Button {
            spend(r)
        } label: {
            HStack {
                VStack(alignment: .leading, spacing: 2) {
                    Text(r.title).font(.body.weight(.semibold))
                    Text(String(localized: "\(r.cost) credits", defaultValue: "\(r.cost) credits"))
                        .font(.caption).foregroundStyle(Color.app.foreground.opacity(0.6))
                }
                Spacer()
                Image(systemName: canAfford ? "arrow.right.circle.fill" : "lock.fill")
                    .foregroundStyle(canAfford ? Color.app.accent : Color.app.foreground.opacity(0.3))
                    .accessibilityLabel(canAfford ? String(localized: "Purchase available", defaultValue: "Purchase available") : String(localized: "Locked - insufficient credits", defaultValue: "Locked - insufficient credits"))
                    .accessibilityHidden(false)
            }
            .padding()
            .frame(maxWidth: .infinity)
            .background(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .fill(Color.app.surface.opacity(canAfford ? 1.0 : 0.5))
                    .accessibilityHidden(true)
            )
        }
        .accessibilityLabel(String(localized: r.title, defaultValue: r.title))
        .accessibilityValue(String(localized: "Costs \(r.cost) credits. \(canAfford ? "Available" : "Not enough credits")", defaultValue: "Costs \(r.cost) credits. \(canAfford ? "Available" : "Not enough credits")"))
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
                Text(label).font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.6))
                Text("\(value)").font(.callout.weight(.semibold))
            }
            .accessibilityElement(children: .combine)
            .accessibilityLabel(String(localized: "\(label): \(value)", defaultValue: "\(label): \(value)"))
        }
    }
}
#endif
