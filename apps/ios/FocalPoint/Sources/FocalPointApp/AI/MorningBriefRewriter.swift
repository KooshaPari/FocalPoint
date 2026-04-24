#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

/// Morning Brief view with Apple Intelligence tone shift buttons (iOS 18+).
/// Displays the brief text and tone transformation options:
/// - Friendly: Warm and conversational
/// - Coach: Direct and motivational
/// - Concise: Short and punchy
/// - Motivational: Inspiring and energetic
///
/// Each button applies a system-provided tone transformation via on-device API.
/// No network calls; all processing on-device using Apple's models.
public struct MorningBriefRewriter: View {
    let briefText: String
    @State private var currentText: String = ""
    @State private var selectedTone: BriefToneOption? = nil
    @State private var isTransforming: Bool = false
    @State private var showCopied: Bool = false

    public init(briefText: String) {
        self.briefText = briefText
        _currentText = State(initialValue: briefText)
    }

    public var body: some View {
        VStack(spacing: 16) {
            // Brief text display (supports Apple Intelligence editing)
            ScrollView {
                Text(currentText)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(12)
                    .background(Color.app.surface)
                    .cornerRadius(8)
                    .textSelection(.enabled)
                    .coachyWritingTools()
            }
            .frame(maxHeight: 200)

            // Tone transformation buttons (iOS 18+ only)
            if AppleIntelligenceSettings.isAvailable() {
                VStack(spacing: 8) {
                    Text("Apple Intelligence Tone Shift")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(.secondary)

                    toneButtons
                }
            }

            // Copy button
            HStack(spacing: 12) {
                Button(action: copyText) {
                    HStack {
                        Image(systemName: "doc.on.doc")
                        Text(showCopied ? "Copied!" : "Copy")
                    }
                }
                .tint(Color.app.accent)

                Spacer()

                if isTransforming {
                    ProgressView()
                        .scaleEffect(0.8)
                }
            }
            .font(.caption.weight(.semibold))
        }
        .padding(16)
    }

    @ViewBuilder
    private var toneButtons: some View {
        LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 8) {
            ForEach(BriefToneOption.allCases, id: \.self) { tone in
                Button(action: { applyTone(tone) }) {
                    VStack(spacing: 4) {
                        Text(tone.rawValue)
                            .font(.caption.weight(.semibold))
                        Text(tone.description)
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                    .frame(maxWidth: .infinity)
                    .padding(8)
                    .background(selectedTone == tone ? Color.app.accent.opacity(0.2) : Color.app.surface)
                    .cornerRadius(6)
                    .overlay(
                        RoundedRectangle(cornerRadius: 6)
                            .stroke(
                                selectedTone == tone ? Color.app.accent : Color.clear,
                                lineWidth: 1
                            )
                    )
                }
                .foregroundStyle(.primary)
                .disabled(isTransforming)
            }
        }
    }

    private func applyTone(_ tone: BriefToneOption) {
        isTransforming = true
        selectedTone = tone

        Task {
            if #available(iOS 18, *) {
                // Attempt on-device tone transformation
                if let transformed = WritingToolsClient.transformText(briefText, tone: tone) {
                    withAnimation {
                        currentText = transformed
                    }
                } else {
                    // Fallback if on-device API unavailable
                    withAnimation {
                        currentText = briefText
                    }
                }
            }

            isTransforming = false
        }
    }

    private func copyText() {
        UIPasteboard.general.string = currentText
        withAnimation {
            showCopied = true
            DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
                withAnimation {
                    showCopied = false
                }
            }
        }
    }
}

#Preview {
    MorningBriefRewriter(
        briefText: """
        Good morning! You have 3 focus sessions scheduled today.
        Your morning ritual is due at 7:00 AM.
        Yesterday you completed 2 of 4 planned tasks.
        Keep up the momentum!
        """
    )
    .environmentObject(CoreHolder.preview())
}
#endif
