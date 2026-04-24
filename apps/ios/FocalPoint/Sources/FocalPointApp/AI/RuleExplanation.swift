#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

/// Rule explanation view with "Explain Like I'm 5" (ELI5) summarization button.
/// iOS 18+: users can tap to get a simplified, child-friendly explanation
/// of the rule's purpose and behavior via on-device Apple Intelligence API.
public struct RuleExplanation: View {
    let ruleName: String
    let explanation: String
    @State private var simplifiedExplanation: String? = nil
    @State private var isSimplifying: Bool = false
    @State private var showSimplified: Bool = false

    public init(ruleName: String, explanation: String) {
        self.ruleName = ruleName
        self.explanation = explanation
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Rule Explanation")
                    .font(.subheading.weight(.semibold))
                Spacer()
                if AppleIntelligenceSettings.isAvailable() {
                    Button(action: toggleSimplified) {
                        HStack(spacing: 4) {
                            Image(systemName: showSimplified ? "sparkles" : "questionmark.circle")
                            Text("ELI5")
                                .font(.caption.weight(.semibold))
                        }
                    }
                    .tint(Color.app.accent)
                    .disabled(isSimplifying)
                }
            }

            // Display current explanation (original or simplified)
            if showSimplified, let simplified = simplifiedExplanation {
                VStack(alignment: .leading, spacing: 8) {
                    Text("Simplified Explanation")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(.secondary)

                    Text(simplified)
                        .font(.body)
                        .foregroundStyle(Color.app.foreground.opacity(0.8))
                        .textSelection(.enabled)
                        .coachyWritingTools()
                }
                .padding(12)
                .background(Color.app.accent.opacity(0.08))
                .cornerRadius(8)

                Button(action: resetExplanation) {
                    Text("Show Original")
                        .font(.caption.weight(.semibold))
                }
                .tint(Color.app.accent)
            } else {
                Text(explanation)
                    .font(.body)
                    .foregroundStyle(Color.app.foreground.opacity(0.8))
                    .textSelection(.enabled)
                    .coachyWritingTools()
            }

            if isSimplifying {
                HStack(spacing: 8) {
                    ProgressView()
                        .scaleEffect(0.8)
                    Text("Simplifying explanation...")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
        }
        .padding(12)
        .background(Color.app.surface)
        .cornerRadius(8)
    }

    private func toggleSimplified() {
        if let _ = simplifiedExplanation {
            showSimplified.toggle()
        } else {
            simplifyExplanation()
        }
    }

    private func simplifyExplanation() {
        isSimplifying = true

        Task {
            if #available(iOS 18, *) {
                // Construct a summarization prompt: "Explain this like I'm 5"
                let summaryPrompt = "Summarize this rule explanation in simple, child-friendly language (ELI5 style), making it easy to understand for anyone. Keep it to 2-3 sentences."
                let fullPrompt = """
                Original explanation: \(explanation)

                \(summaryPrompt)
                """

                // Attempt on-device summarization
                if let simplified = WritingToolsClient.transformText(fullPrompt, tone: .friendly) {
                    withAnimation {
                        self.simplifiedExplanation = simplified
                        self.showSimplified = true
                    }
                } else {
                    // Fallback: show a generic message
                    withAnimation {
                        self.simplifiedExplanation = "This rule helps you stay focused by applying a specific action when a certain event happens."
                        self.showSimplified = true
                    }
                }
            }

            isSimplifying = false
        }
    }

    private func resetExplanation() {
        withAnimation {
            showSimplified = false
        }
    }
}

#Preview {
    RuleExplanation(
        ruleName: "Focus Session Reward",
        explanation: """
        This rule fires when a focus session is completed successfully (session_completed event).
        The rule evaluates the session duration and awards wallet credits based on the focus
        intensity level. Minimum 25-minute sessions qualify. Multiple back-to-back sessions
        within the cooldown window are deduplicated. The explanation template substitutes
        {rule_name} with the rule's display name, {event_type} with the triggering event type,
        and {event_id} with the unique event identifier for audit tracing.
        """
    )
    .environmentObject(CoreHolder.preview())
}
#endif
