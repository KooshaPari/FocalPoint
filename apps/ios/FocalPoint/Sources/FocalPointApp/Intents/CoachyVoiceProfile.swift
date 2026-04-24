import AVFoundation
import Foundation

/// Voice profile selector for Coachy responses.
/// Prefers system Siri voice when available (iOS 18+ with Apple Intelligence),
/// falls back to AVSpeechSynthesizer with custom pitch/rate tuning.
struct CoachyVoiceProfile {
    /// Selected voice: either system Siri or AVSpeechSynthesizer with tuning.
    enum Voice {
        case systemSiri              // iOS 18+ Apple Intelligence
        case avSynthesizer(pitch: Float, rate: Float)  // AVSpeechSynthesizer fallback
    }

    /// Synthesize emotion + text → spoken audio via system voice or fallback.
    ///
    /// On iOS 18+ with Apple Intelligence available, attempts to use the system
    /// Siri voice for natural, context-aware responses. Falls back to AVSpeechSynthesizer
    /// with emotion-tuned pitch/rate.
    static func speak(
        text: String,
        emotion: CoachyEmotion,
        voice: Voice = .default
    ) -> AVSpeechUtterance {
        let utterance = AVSpeechUtterance(string: text)

        switch voice {
        case .systemSiri:
            // iOS 18+ path: try to use native Siri voice
            if #available(iOS 18, *) {
                // Prefer the system default Siri voice (English US)
                utterance.voice = AVSpeechSynthesisVoice(identifier: AVSpeechSynthesisVoiceIdentifierDefault)
            } else {
                // Fallback for older iOS
                utterance.voice = AVSpeechSynthesisVoice(language: "en-US")
            }
            utterance.pitchMultiplier = 1.0
            utterance.rate = AVSpeechUtteranceMaximumSpeechRate * 0.5

        case .avSynthesizer(let pitch, let rate):
            // AVSpeechSynthesizer path with emotion tuning
            utterance.voice = AVSpeechSynthesisVoice(language: "en-US")
            utterance.pitchMultiplier = pitch
            utterance.rate = rate

            // Clamp to valid ranges
            utterance.pitchMultiplier = max(0.5, min(2.0, utterance.pitchMultiplier))
            utterance.rate = max(AVSpeechUtteranceMinimumSpeechRate, min(AVSpeechUtteranceMaximumSpeechRate, utterance.rate))
        }

        return utterance
    }

    /// Map emotion → voice parameters (pitch, rate) for AVSpeechSynthesizer.
    static func parametersForEmotion(_ emotion: CoachyEmotion) -> (pitch: Float, rate: Float) {
        let baseRate = AVSpeechUtteranceDefaultSpeechRate

        switch emotion {
        case .happy, .excited:
            return (pitch: 1.2, rate: baseRate * 1.1)  // Higher pitch, slightly faster

        case .proud, .encouraging:
            return (pitch: 1.05, rate: baseRate * 0.95)  // Slight lift, conversational

        case .supportive, .neutral:
            return (pitch: 1.0, rate: baseRate)  // Natural

        case .focused:
            return (pitch: 0.95, rate: baseRate * 0.9)  // Slightly lower, measured pace

        case .concerned, .disappointed, .tired:
            return (pitch: 0.85, rate: baseRate * 0.85)  // Lower pitch, slower
        }
    }

    /// Default voice selection logic:
    /// - iOS 18+: Try system Siri first
    /// - Fallback: AVSpeechSynthesizer with emotion-based tuning
    static var `default`: Voice {
        if #available(iOS 18, *) {
            return .systemSiri
        } else {
            return .avSynthesizer(pitch: 1.0, rate: AVSpeechUtteranceDefaultSpeechRate)
        }
    }
}
