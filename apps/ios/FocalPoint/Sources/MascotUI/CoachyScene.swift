import SwiftUI
import AVFoundation

/// Complete Coachy scene container: pose + emotion + accessories + bubble + sound + haptic + particle effects.
struct CoachyScene {
    let pose: CoachyPose
    let emotion: CoachyEmotion
    let accessories: [CoachyAccessory]
    let bubbleText: String?

    let soundCueId: String?
    let hapticPattern: CoachyHapticPattern?
    let particleSystems: [CoachyParticle]?

    let entry: CoachyEntryAnimation?
    let hold: TimeInterval
    let exit: CoachyExitAnimation?

    let onStart: (() -> Void)?
    let onComplete: (() -> Void)?

    init(
        pose: CoachyPose,
        emotion: CoachyEmotion,
        accessories: [CoachyAccessory] = [],
        bubbleText: String? = nil,
        soundCueId: String? = nil,
        hapticPattern: CoachyHapticPattern? = nil,
        particleSystems: [CoachyParticle]? = nil,
        entry: CoachyEntryAnimation? = nil,
        hold: TimeInterval = 1.0,
        exit: CoachyExitAnimation? = nil,
        onStart: (() -> Void)? = nil,
        onComplete: (() -> Void)? = nil
    ) {
        self.pose = pose
        self.emotion = emotion
        self.accessories = accessories
        self.bubbleText = bubbleText
        self.soundCueId = soundCueId
        self.hapticPattern = hapticPattern
        self.particleSystems = particleSystems
        self.entry = entry
        self.hold = hold
        self.exit = exit
        self.onStart = onStart
        self.onComplete = onComplete
    }
}

enum CoachyEntryAnimation {
    case fade(duration: TimeInterval, easing: UICurve = .easeOut)
    case flyIn(from: CoachyDirection, duration: TimeInterval, easing: UICurve = .easeOut)
    case grow(from: CGFloat, to: CGFloat, duration: TimeInterval, easing: UICurve = .easeOut)
    case bounce(fromScale: CGFloat, toScale: CGFloat, duration: TimeInterval)
}

enum CoachyExitAnimation {
    case fade(duration: TimeInterval)
    case flyOut(to: CoachyDirection, duration: TimeInterval)
    case shrinkToPoint(duration: TimeInterval)
}

enum CoachyDirection {
    case left, right, top, bottom
}

enum CoachyAccessory: String, Codable, CaseIterable {
    case none, headphones, glassesAndBook, trophy, shield, padlock
}

enum CoachyParticle: Hashable {
    case confetti(count: Int)
    case coinTrail(count: Int)
    case redFlash(duration: TimeInterval)
}

enum CoachyHapticPattern {
    case lightTap
    case mediumTap
    case heavyTap
    case success
    case warning
    case tripleSuccess
    case decayPattern(startIntensity: UIImpactFeedbackGenerator.FeedbackStyle, steps: Int)
    case none
}

enum UICurve {
    case linear
    case easeIn
    case easeOut
    case easeInOut
    case custom(Animation)
}

/// CoachyScene presenter: orchestrates audio, haptic, and scene lifecycle.
@MainActor
class CoachyScenePresenter: ObservableObject {
    @Published var voiceMode: CoachyVoiceMode = .simlish
    @Published var soundEffectsEnabled: Bool = true
    @Published var hapticEnabled: Bool = true
    @Published var sfxVolume: Float = 1.0

    private let soundPlayer = SoundEffectPlayer.shared
    private let voiceProvider: CoachyVoiceProvider?

    init() {
        self.voiceProvider = SimlishVoiceProvider()
    }

    func presentScene(
        _ scene: CoachyScene,
        in container: UIView?
    ) {
        // Trigger lifecycle start
        scene.onStart?()

        // Play sound if specified
        if soundEffectsEnabled, let soundCueId = scene.soundCueId {
            soundPlayer.play(soundCueId, hapticPattern: nil)
        }

        // Play haptic if specified
        if hapticEnabled, let hapticPattern = scene.hapticPattern {
            playHaptic(hapticPattern)
        }

        // Schedule exit if hold > 0
        if scene.hold > 0 {
            DispatchQueue.main.asyncAfter(deadline: .now() + scene.hold) {
                scene.onComplete?()
            }
        }
    }

    private func playHaptic(_ pattern: CoachyHapticPattern) {
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
            let gen = UINotificationFeedbackGenerator()
            gen.notificationOccurred(.success)
        case .warning:
            let gen = UINotificationFeedbackGenerator()
            gen.notificationOccurred(.warning)
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
}

enum CoachyVoiceMode: String, Codable {
    case simlish
    case avSpeechSynthesizer
    case silent
}

/// SwiftUI view for a complete CoachyScene.
struct CoachySceneView: View {
    let scene: CoachyScene
    @State private var isAnimating = false
    @EnvironmentObject var presenter: CoachyScenePresenter

    var body: some View {
        ZStack {
            // Particle systems
            if let particles = scene.particleSystems {
                ForEach(particles, id: \.self) { particle in
                    CoachyParticleView(particle: particle)
                }
            }

            // Coachy character
            CoachyView(
                state: CoachyState(
                    pose: scene.pose,
                    emotion: scene.emotion,
                    bubbleText: scene.bubbleText
                ),
                size: 240,
                accessories: scene.accessories
            )
            .transition(computeTransition())
        }
        .onAppear {
            isAnimating = true
            presenter.presentScene(scene, in: nil)
            if scene.hold > 0 {
                DispatchQueue.main.asyncAfter(deadline: .now() + scene.hold) {
                    isAnimating = false
                }
            }
        }
    }

    private func computeTransition() -> AnyTransition {
        if let entry = scene.entry {
            switch entry {
            case .fade(let duration, let easing):
                return AnyTransition.opacity.animation(.easeInOut(duration: duration))
            case .flyIn(let from, let duration, let easing):
                let offset = offsetForDirection(from)
                return AnyTransition.offset(offset).animation(.easeOut(duration: duration))
            case .grow(let from, let to, let duration, let easing):
                return AnyTransition.scale(scale: from).animation(.easeOut(duration: duration))
            case .bounce(let fromScale, let toScale, let duration):
                return AnyTransition.scale(scale: fromScale).animation(.spring(response: duration, dampingFraction: 0.6))
            }
        }
        return AnyTransition.opacity
    }

    private func offsetForDirection(_ direction: CoachyDirection) -> CGSize {
        switch direction {
        case .left:
            return CGSize(width: -300, height: 0)
        case .right:
            return CGSize(width: 300, height: 0)
        case .top:
            return CGSize(width: 0, height: -300)
        case .bottom:
            return CGSize(width: 0, height: 300)
        }
    }
}

/// Particle system view (stub for Tier 0; can be enhanced with Lottie/custom animations later).
struct CoachyParticleView: View {
    let particle: CoachyParticle
    @State private var isAnimating = false

    var body: some View {
        ZStack {
            switch particle {
            case .confetti(let count):
                ForEach(0..<count, id: \.self) { i in
                    Circle()
                        .fill(Color.random)
                        .frame(width: 8, height: 8)
                        .offset(x: CGFloat.random(in: -100...100), y: isAnimating ? 300 : -100)
                        .opacity(isAnimating ? 0 : 1)
                }
            case .coinTrail(let count):
                ForEach(0..<count, id: \.self) { i in
                    Image(systemName: "dollarsign.circle.fill")
                        .font(.system(size: 16))
                        .foregroundColor(.yellow)
                        .offset(x: CGFloat(i) * 20, y: isAnimating ? 200 : 0)
                        .opacity(isAnimating ? 0 : 1)
                }
            case .redFlash(let duration):
                Rectangle()
                    .fill(Color.red.opacity(0.3))
                    .opacity(isAnimating ? 0 : 1)
            }
        }
        .onAppear {
            withAnimation(.easeOut(duration: 2.0)) {
                isAnimating = true
            }
        }
    }
}

extension Color {
    static var random: Color {
        [.red, .orange, .yellow, .green, .blue, .purple, .pink].randomElement() ?? .blue
    }
}
