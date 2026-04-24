import AVFoundation
import Foundation

/// Sound effect manager with haptic synchronization. Gracefully degrades if audio files are missing.
class SoundEffectPlayer: NSObject, AVAudioPlayerDelegate {
    static let shared = SoundEffectPlayer()

    private var audioPlayers: [String: AVAudioPlayer] = [:]
    private let feedbackGenerator = UIImpactFeedbackGenerator()
    private let notificationGenerator = UINotificationFeedbackGenerator()
    private var audioSession: AVAudioSession { AVAudioSession.sharedInstance() }

    override init() {
        super.init()
        setupAudioSession()
    }

    private func setupAudioSession() {
        do {
            try audioSession.setCategory(.ambient, options: [.defaultToSpeaker, .mixWithOthers])
            try audioSession.setActive(true)
        } catch {
            print("⚠️ Failed to setup audio session: \(error)")
        }
    }

    /// Play a named sound effect with optional haptic feedback. Gracefully handles missing files.
    func play(_ soundId: String, hapticPattern: HapticPattern? = nil) {
        guard let url = Bundle.main.url(forResource: soundId, withExtension: "m4a", subdirectory: "Audio/SFX") else {
            print("⚠️ Sound file not found: \(soundId).m4a (expected in Resources/Audio/SFX/)")
            return // Graceful fallback: silent if audio missing
        }

        do {
            let player = try AVAudioPlayer(contentsOf: url)
            player.delegate = self
            audioPlayers[soundId] = player
            player.play()

            // Trigger haptic if specified
            if let hapticPattern = hapticPattern {
                playHaptic(hapticPattern)
            }
        } catch {
            print("⚠️ Failed to play sound \(soundId): \(error)")
        }
    }

    /// Play looping ambient sound with fade-in.
    func playAmbient(_ soundId: String, fadeInDuration: TimeInterval = 1.0) {
        guard let url = Bundle.main.url(forResource: soundId, withExtension: "m4a", subdirectory: "Audio/SFX") else {
            print("⚠️ Ambient sound file not found: \(soundId).m4a")
            return
        }

        do {
            let player = try AVAudioPlayer(contentsOf: url)
            player.numberOfLoops = -1 // Loop indefinitely
            player.volume = 0.0 // Start silent for fade-in
            player.play()
            audioPlayers[soundId] = player

            // Fade in over specified duration
            let fadeSteps = Int(fadeInDuration * 30.0) // ~30 FPS
            let volumeStep = 0.4 / Float(fadeSteps) // Target volume 0.4 (quiet for ambient)

            var step = 0
            let timer = Timer.scheduledTimer(withTimeInterval: 1.0 / 30.0, repeats: true) { [weak self, weak player] _ in
                guard let self = self, let player = player else { return }
                step += 1
                player.volume = min(0.4, player.volume + volumeStep)
                if step >= fadeSteps {
                    // Done fading; timer stops
                }
            }

            // Keep timer alive by storing it
            objc_setAssociatedObject(player, "fadeTimer", timer, .OBJC_ASSOCIATION_RETAIN)
        } catch {
            print("⚠️ Failed to play ambient sound \(soundId): \(error)")
        }
    }

    /// Stop ambient sound with fade-out.
    func stopAmbient(_ soundId: String, fadeOutDuration: TimeInterval = 1.0) {
        guard let player = audioPlayers[soundId] else {
            print("⚠️ Ambient sound \(soundId) not playing")
            return
        }

        let fadeSteps = Int(fadeOutDuration * 30.0)
        let volumeStep = player.volume / Float(fadeSteps)

        var step = 0
        let timer = Timer.scheduledTimer(withTimeInterval: 1.0 / 30.0, repeats: true) { [weak self, weak player] _ in
            guard let self = self, let player = player else { return }
            step += 1
            player.volume = max(0.0, player.volume - volumeStep)
            if step >= fadeSteps {
                player.stop()
                self.audioPlayers[soundId] = nil
            }
        }

        objc_setAssociatedObject(player, "fadeOutTimer", timer, .OBJC_ASSOCIATION_RETAIN)
    }

    /// Play haptic pattern synchronously with audio.
    private func playHaptic(_ pattern: HapticPattern) {
        switch pattern {
        case .lightTap:
            let gen = UIImpactFeedbackGenerator(style: .light)
            gen.impactOccurred()
        case .mediumTap:
            let gen = UIImpactFeedbackGenerator(style: .medium)
            gen.impactOccurred()
        case .heavyTap:
            let gen = UIImpactFeedbackGenerator(style: .heavy)
            gen.impactOccurred()
        case .success:
            notificationGenerator.notificationOccurred(.success)
        case .warning:
            notificationGenerator.notificationOccurred(.warning)
        case .tripleSuccess:
            let gen = UIImpactFeedbackGenerator(style: .heavy)
            gen.impactOccurred()
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
                UIImpactFeedbackGenerator(style: .light).impactOccurred()
            }
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.4) {
                UIImpactFeedbackGenerator(style: .heavy).impactOccurred()
            }
        case .decayPattern(let startIntensity, let steps):
            let gen = UIImpactFeedbackGenerator(style: startIntensity)
            gen.impactOccurred()
            for i in 1..<steps {
                let delay = TimeInterval(i) * 0.1
                DispatchQueue.main.asyncAfter(deadline: .now() + delay) {
                    if i % 2 == 0 {
                        gen.impactOccurred()
                    }
                }
            }
        case .none:
            break
        }
    }

    func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        audioPlayers.removeAll { $0.value === player }
    }
}

enum HapticPattern {
    case lightTap
    case mediumTap
    case heavyTap
    case success
    case warning
    case tripleSuccess
    case decayPattern(startIntensity: UIImpactFeedbackGenerator.FeedbackStyle, steps: Int)
    case none
}

// Utility for associating objects with AVAudioPlayer for timer management
private var associatedObjectHandle: UInt8 = 0
