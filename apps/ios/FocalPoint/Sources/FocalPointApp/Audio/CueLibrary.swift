import AVFoundation
import UIKit

/// Cue catalog with paired haptic patterns. All 8 cues deterministically synthesized.
enum FocalPointCue: String, CaseIterable {
    case ruleFire = "rule-fire"
    case achievement = "achievement"
    case interventionWarn = "intervention-warn"
    case focusStart = "focus-start"
    case focusEnd = "focus-end"
    case error = "error"
    case success = "success"
    case mascotAcknowledge = "mascot-acknowledge"

    /// Human-readable label for each cue.
    var label: String {
        switch self {
        case .ruleFire: return "Rule Fire"
        case .achievement: return "Achievement"
        case .interventionWarn: return "Intervention Warning"
        case .focusStart: return "Focus Start"
        case .focusEnd: return "Focus End"
        case .error: return "Error"
        case .success: return "Success"
        case .mascotAcknowledge: return "Mascot Acknowledge"
        }
    }

    /// Duration in seconds (matches synthesized WAV).
    var duration: Double {
        switch self {
        case .ruleFire: return 0.08
        case .achievement: return 0.2
        case .interventionWarn: return 0.15
        case .focusStart: return 0.3
        case .focusEnd: return 0.25
        case .error: return 0.1
        case .success: return 0.18
        case .mascotAcknowledge: return 0.12
        }
    }

    /// Paired haptic feedback for this cue.
    var hapticPattern: CueHapticPattern {
        switch self {
        case .ruleFire: return .ascendingChime
        case .achievement: return .resonantBell
        case .interventionWarn: return .descendingPulse
        case .focusStart: return .calmFadeIn
        case .focusEnd: return .resolvingDoubleTap
        case .error: return .sharpBuzz
        case .success: return .percussiveTick
        case .mascotAcknowledge: return .friendlyBlip
        }
    }
}

/// Haptic feedback pattern paired with audio cue.
enum CueHapticPattern {
    case ascendingChime
    case resonantBell
    case descendingPulse
    case calmFadeIn
    case resolvingDoubleTap
    case sharpBuzz
    case percussiveTick
    case friendlyBlip

    /// Execute the haptic pattern on the device.
    func play() {
        switch self {
        case .ascendingChime:
            playAscendingChime()
        case .resonantBell:
            playResonantBell()
        case .descendingPulse:
            playDescendingPulse()
        case .calmFadeIn:
            playCalmFadeIn()
        case .resolvingDoubleTap:
            playResolvingDoubleTap()
        case .sharpBuzz:
            playSharpBuzz()
        case .percussiveTick:
            playPercussiveTick()
        case .friendlyBlip:
            playFriendlyBlip()
        }
    }

    // MARK: - Haptic Implementations

    private func playAscendingChime() {
        let gen = UIImpactFeedbackGenerator(style: .light)
        for delay in [0.0, 0.03, 0.06] {
            DispatchQueue.main.asyncAfter(deadline: .now() + delay) {
                gen.impactOccurred()
            }
        }
    }

    private func playResonantBell() {
        let gen = UIImpactFeedbackGenerator(style: .medium)
        gen.impactOccurred()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) {
            let continuousGen = UIImpactFeedbackGenerator(style: .light)
            continuousGen.impactOccurred()
        }
    }

    private func playDescendingPulse() {
        let generator = UIImpactFeedbackGenerator(style: .heavy)
        for (index, intensity) in [(0.0, UIImpactFeedbackGenerator.FeedbackStyle.heavy),
                                    (0.04, UIImpactFeedbackGenerator.FeedbackStyle.medium),
                                    (0.08, UIImpactFeedbackGenerator.FeedbackStyle.light)].enumerated() {
            DispatchQueue.main.asyncAfter(deadline: .now() + intensity.0) {
                UIImpactFeedbackGenerator(style: intensity.1).impactOccurred()
            }
        }
    }

    private func playCalmFadeIn() {
        let gen = UIImpactFeedbackGenerator(style: .light)
        gen.impactOccurred()
    }

    private func playResolvingDoubleTap() {
        let gen = UIImpactFeedbackGenerator(style: .medium)
        gen.impactOccurred()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.12) {
            UIImpactFeedbackGenerator(style: .heavy).impactOccurred()
        }
    }

    private func playSharpBuzz() {
        let gen = UIImpactFeedbackGenerator(style: .heavy)
        gen.impactOccurred()
    }

    private func playPercussiveTick() {
        let gen = UIImpactFeedbackGenerator(style: .medium)
        gen.impactOccurred()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.09) {
            UIImpactFeedbackGenerator(style: .heavy).impactOccurred()
        }
    }

    private func playFriendlyBlip() {
        let gen = UIImpactFeedbackGenerator(style: .light)
        gen.impactOccurred()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.04) {
            UIImpactFeedbackGenerator(style: .light).impactOccurred()
        }
    }
}

/// Manager to play audio cues with synchronized haptic feedback.
class CuePlayer: NSObject, AVAudioPlayerDelegate {
    static let shared = CuePlayer()

    private var audioPlayers: [String: AVAudioPlayer] = [:]
    private let audioSession = AVAudioSession.sharedInstance()

    override init() {
        super.init()
        setupAudioSession()
    }

    private func setupAudioSession() {
        do {
            try audioSession.setCategory(.default, options: [.defaultToSpeaker, .mixWithOthers])
            try audioSession.setActive(true, options: .notifyOthersOnDeactivation)
        } catch {
            print("⚠️ CuePlayer: Audio session setup failed: \(error)")
        }
    }

    /// Play a cue with its paired haptic feedback.
    func play(_ cue: FocalPointCue) {
        playAudio(cue)
        cue.hapticPattern.play()
    }

    /// Play audio portion only (for testing).
    func playAudio(_ cue: FocalPointCue) {
        guard let url = Bundle.main.url(forResource: cue.rawValue, withExtension: "wav",
                                        subdirectory: "Audio/Cues") else {
            print("⚠️ Cue audio not found: \(cue.rawValue).wav")
            return
        }

        do {
            let player = try AVAudioPlayer(contentsOf: url)
            player.delegate = self
            audioPlayers[cue.rawValue] = player
            player.play()
        } catch {
            print("⚠️ CuePlayer: Failed to play \(cue.rawValue): \(error)")
        }
    }

    /// Stop all playing cues.
    func stopAll() {
        audioPlayers.forEach { _, player in player.stop() }
        audioPlayers.removeAll()
    }

    func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        let keysToRemove = audioPlayers.filter { $0.value === player }.map { $0.key }
        for key in keysToRemove {
            audioPlayers.removeValue(forKey: key)
        }
    }
}
