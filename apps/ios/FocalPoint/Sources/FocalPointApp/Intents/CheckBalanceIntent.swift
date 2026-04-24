import AppIntents
import FocalPointCore

/// Siri Shortcut to check wallet balance. Example: "Hey Siri, check FocalPoint balance"
struct CheckBalanceIntent: AppIntent {
    static var title: LocalizedStringResource = "Check FocalPoint Balance"
    static var openAppWhenRun = false

    static var description = IntentDescription(
        "Check your current FocalPoint wallet balance and available credits.",
        categoryIdentifier: "wallet"
    )

    @MainActor
    func perform() async throws -> some IntentResult {
        let core = CoreHolder.shared.core
        let walletApi = core.wallet()
        let summary = try walletApi.load()

        let balanceText = String(format: "Balance: %d credits, Pending: %d", summary.available, summary.pending)
        return .result(value: balanceText, opensAppWhenTapped: false)
    }
}
