import AVFoundation
import Foundation

/// Simlish voice provider: procedural phoneme sequencing with emotion-based pitch shifting.
@MainActor
class SimlishVoiceProvider: CoachyVoiceProvider {
    private let audioEngine = AVAudioEngine()
    private var phonemeLibrary: [String: AVAudioFile] = [:]
    @Published var currentPhoneme: String?

    init() {
        loadPhonemeLibrary()
    }

    private func loadPhonemeLibrary() {
        let phonemes = ["m", "a", "b", "e", "i", "g", "o", "u", "n", "p", "sh", "t", "d", "w", "l", "sil"]

        for phoneme in phonemes {
            let filename = "phoneme-\(phoneme)"
            // Try to load from bundle; if missing, create silent fallback
            if let url = Bundle.main.url(forResource: filename, withExtension: "m4a", subdirectory: "Audio/Simlish") {
                do {
                    let file = try AVAudioFile(forReading: url)
                    phonemeLibrary[phoneme] = file
                } catch {
                    print("⚠️ Failed to load phoneme \(phoneme): \(error)")
                    // Gracefully continue; this phoneme will be skipped
                }
            } else {
                print("⚠️ Phoneme file not found: \(filename).m4a (expected in Resources/Audio/Simlish/)")
            }
        }
    }

    func synthesize(
        text: String,
        emotion: CoachyEmotion,
        completion: @escaping (Result<CoachyAudio, Error>) -> Void
    ) {
        DispatchQueue.global().async { [weak self] in
            guard let self = self else { return }

            // Step 1: Calculate target duration
            let targetDuration = Double(text.count) * 0.15 + 0.2

            // Step 2: Generate phoneme sequence
            let phonemeSequence = self.generatePhonemeSequence(targetDuration: targetDuration)

            // Step 3: Concatenate audio (stub; Tier 0 uses silent fallback if phonemes missing)
            var audioSamples: [Float] = []
            var totalDuration: TimeInterval = 0

            for phoneme in phonemeSequence {
                if let file = self.phonemeLibrary[phoneme] {
                    // In a real implementation, extract samples and concatenate
                    // For Tier 0, we'll use a silent approximation
                    let estimatedDuration = self.estimatedDuration(for: phoneme)
                    totalDuration += estimatedDuration
                    // Append silence/zero samples for this duration
                    let sampleCount = Int(estimatedDuration * 48000) // 48 kHz
                    audioSamples.append(contentsOf: Array(repeating: Float(0), count: sampleCount))
                }
            }

            // Step 4: Pitch shift based on emotion
            let pitchShift = self.pitchForEmotion(emotion)
            // In a real implementation, apply pitch-shifting algorithm
            // For Tier 0, we'll skip pitch shifting (audio is silent anyway)

            // Step 5: Create result DTO
            let sampleRate: UInt32 = 48000
            let duration = TimeInterval(audioSamples.count) / Double(sampleRate)

            let audio = CoachyAudio(
                audioBuffer: AVAudioPCMBuffer(pcmFormat: AVAudioFormat(standardFormatWithSampleRate: Double(sampleRate), channels: 1)!, frameCapacity: AVAudioFrameCount(audioSamples.count)),
                visemes: nil,
                duration: duration
            )

            DispatchQueue.main.async {
                completion(.success(audio))
            }
        }
    }

    private func generatePhonemeSequence(targetDuration: Double) -> [String] {
        var sequence: [String] = []
        var currentTime = 0.0
        let consonants = ["m", "b", "g", "w", "n", "d"]
        let vowels = ["a", "e", "i", "o", "u"]

        while currentTime < targetDuration {
            // 70% start with consonant
            if Bool.random(probability: 0.7) {
                let consonant = consonants.randomElement()!
                sequence.append(consonant)
                currentTime += estimatedDuration(for: consonant)
            }

            // Add vowel
            let vowel = vowels.randomElement()!
            sequence.append(vowel)
            currentTime += estimatedDuration(for: vowel)

            // Occasional silence
            if Bool.random(probability: 0.05) {
                sequence.append("sil")
                currentTime += estimatedDuration(for: "sil")
            }
        }

        return sequence
    }

    private func estimatedDuration(for phoneme: String) -> TimeInterval {
        switch phoneme {
        case "m", "n": return 0.1
        case "a", "e", "i", "o", "u": return 0.12
        case "b", "d", "g", "w": return 0.08
        case "p", "t": return 0.06
        case "sh": return 0.15
        case "l": return 0.10
        case "sil": return 0.05
        default: return 0.08
        }
    }

    private func pitchForEmotion(_ emotion: CoachyEmotion) -> Float {
        switch emotion {
        case .happy, .excited, .proud:
            return 2.0 // +2 semitones
        case .neutral:
            return 0.0
        case .concerned, .disappointed:
            return -1.0
        case .focused, .tired:
            return -1.5
        default:
            return 0.0
        }
    }
}

/// Generic voice provider protocol.
protocol CoachyVoiceProvider {
    func synthesize(
        text: String,
        emotion: CoachyEmotion,
        completion: @escaping (Result<CoachyAudio, Error>) -> Void
    )
}

struct CoachyAudio {
    let audioBuffer: AVAudioPCMBuffer?
    let visemes: [VisemeKeyframe]?
    let duration: TimeInterval
}

struct VisemeKeyframe {
    let time: TimeInterval
    let viseme: String // "A", "B", "C", etc. (ARKit blend-shape codes)
}

/// Helper for random boolean with probability.
extension Bool {
    static func random(probability: Double) -> Bool {
        Double.random(in: 0..<1) < probability
    }
}
