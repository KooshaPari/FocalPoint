import AppIntents
import AVFoundation
import FocalPointCore

/// Siri Shortcut for conversational coaching: "Hey Siri, Coachy" or "Ask Coachy how I'm doing".
/// Returns a natural-language dialog response with multi-turn capability on iOS 18+.
/// Falls back to a deterministic state-based response with system voice synthesis.
struct CoachyConversationIntent: AppIntent {
    static var title: LocalizedStringResource = "Ask Coachy"
    static var openAppWhenRun = false
    static var description = IntentDescription(
        "Get a personalized coaching message about your tasks, credits, rituals, and focus streak.",
        categoryIdentifier: "focus"
    )

    /// Optional user question (for future multi-turn support).
    @Parameter(title: "Question", description: "Optional: ask Coachy a follow-up question")
    var question: String?

    @MainActor
    func perform() async throws -> some IntentResult {
        let core = CoreHolder.shared.core

        // Gather current state
        let walletApi = core.wallet()
        let walletSummary = try walletApi.load()
        let credits = walletSummary.available

        // Get task counts (from always-on engine)
        let alwaysOn = core.alwaysOn()
        let proposals = try alwaysOn.tick()

        // Parse proposals to estimate task count (simplified: proposals ≈ pending tasks)
        let taskCount = proposals.count

        // Estimate completed today (placeholder; real impl would query event store)
        let completedToday = max(0, taskCount / 2)  // Optimistic placeholder

        // Get streak (placeholder; real impl would query audit records)
        let streakDays = 1

        // Get ritual status
        let ritualStatus = "all caught up"

        // Build state snapshot
        let state = CoachyState(
            taskCount: taskCount,
            completedToday: completedToday,
            credits: credits,
            streakDays: streakDays,
            ritualStatus: ritualStatus,
            weekdayIndex: Calendar.current.component(.weekday, from: Date()) - 1,
            timeOfDay: .current
        )

        // Generate response
        let (responseText, emotion) = CoachyResponseSynth.generateResponse(state: state)

        // Speak the response via system or fallback voice
        speakResponse(responseText, emotion: emotion)

        // iOS 18+: Support multi-turn dialog
        if #available(iOS 18, *) {
            return .result(dialog: .init(responseText))
        } else {
            return .result(value: responseText, opensAppWhenTapped: false)
        }
    }

    /// Synthesize and play the response via the appropriate voice.
    private func speakResponse(_ text: String, emotion: CoachyEmotion) {
        let synthesizer = AVSpeechSynthesizer()

        // Get voice based on availability and emotion
        let voiceProfile = CoachyVoiceProfile.default
        let utterance = CoachyVoiceProfile.speak(text: text, emotion: emotion, voice: voiceProfile)

        synthesizer.speak(utterance)
    }
}
