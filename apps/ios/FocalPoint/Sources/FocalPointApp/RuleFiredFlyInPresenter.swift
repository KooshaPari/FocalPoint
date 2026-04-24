#if canImport(SwiftUI)
import SwiftUI
import FocalPointCore
import MascotUI

/// Manages rule-fired visual fly-ins with rate-limiting.
/// Deduplicates via AuditRecord.id and collapses 3+ rules in a 10-second window.
@MainActor
public final class RuleFiredFlyInPresenter {
    public static let shared = RuleFiredFlyInPresenter()

    @Published var currentFlyIn: RuleFlyInState?
    @Published var isPresenting: Bool = false

    private let udKey = "focalpoint.flyin.dispatched_ids"
    private let maxMemoryIds = 500
    private let rateWindowSeconds: TimeInterval = 10.0
    private let collapseThreshold = 3

    private var seen: Set<String> {
        get {
            let arr = UserDefaults.standard.stringArray(forKey: udKey) ?? []
            return Set(arr)
        }
        set {
            var trimmed = Array(newValue)
            if trimmed.count > maxMemoryIds {
                trimmed = Array(trimmed.suffix(maxMemoryIds))
            }
            UserDefaults.standard.set(trimmed, forKey: udKey)
        }
    }

    private var recentFlyInTimes: [Date] = []
    private var pendingRules: [(AuditRecordDto, String)] = []

    private init() {}

    /// Pulls rule.fired records, dedupes, and presents fly-ins with rate-limiting.
    /// Collapses 3+ rules in 10s into a single aggregated fly-in.
    public func tick(core: FocalPointCore, flyInsEnabled: Bool) {
        guard flyInsEnabled else { return }
        guard let records = try? core.audit().recent(limit: 50) else { return }

        var seen = self.seen
        var rulesToPresent: [(AuditRecordDto, String)] = []

        for rec in records where rec.recordType == "rule.fired" && !seen.contains(rec.id) {
            if let explanation = parseRuleExplanation(rec.payloadJson) {
                rulesToPresent.append((rec, explanation))
                seen.insert(rec.id)
            }
        }

        self.seen = seen

        guard !rulesToPresent.isEmpty else { return }

        // Trim recentFlyInTimes to the rate window
        let now = Date()
        recentFlyInTimes.removeAll { now.timeIntervalSince($0) > rateWindowSeconds }

        // Check if we should collapse
        let wouldExceedThreshold = (recentFlyInTimes.count + rulesToPresent.count) >= collapseThreshold

        if wouldExceedThreshold && !recentFlyInTimes.isEmpty {
            // Collapse: show aggregated fly-in
            let count = recentFlyInTimes.count + rulesToPresent.count
            presentAggregatedFlyIn(count: count)
        } else if rulesToPresent.count == 1 {
            // Single rule: show it with explanation
            let (rec, explanation) = rulesToPresent[0]
            presentRuleFlyIn(record: rec, explanation: explanation)
        } else if rulesToPresent.count > 1 {
            // Multiple rules: collapse into aggregate
            let totalCount = recentFlyInTimes.count + rulesToPresent.count
            presentAggregatedFlyIn(count: totalCount)
        }

        recentFlyInTimes.append(now)
    }

    private func presentRuleFlyIn(record: AuditRecordDto, explanation: String) {
        let state = RuleFlyInState(
            bubble: explanation,
            isAggregated: false,
            ruleCount: 1
        )
        withAnimation {
            currentFlyIn = state
            isPresenting = true
        }

        // Auto-dismiss after 1.8 seconds
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.8) { [weak self] in
            withAnimation {
                self?.isPresenting = false
            }
        }
    }

    private func presentAggregatedFlyIn(count: Int) {
        let state = RuleFlyInState(
            bubble: "\(count) rules just fired!",
            isAggregated: true,
            ruleCount: count
        )
        withAnimation {
            currentFlyIn = state
            isPresenting = true
        }

        DispatchQueue.main.asyncAfter(deadline: .now() + 1.8) { [weak self] in
            withAnimation {
                self?.isPresenting = false
            }
        }
    }

    private func parseRuleExplanation(_ json: String) -> String? {
        guard let data = json.data(using: .utf8),
              let obj = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let explanation = obj["explanation_template"] as? String else {
            // Fall back to rule name if available
            if let data = json.data(using: .utf8),
               let obj = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
               let name = obj["rule_name"] as? String {
                return name
            }
            return nil
        }
        return explanation
    }
}

public struct RuleFlyInState: Identifiable {
    public let id: UUID = UUID()
    public let bubble: String
    public let isAggregated: Bool
    public let ruleCount: Int
}
#endif
