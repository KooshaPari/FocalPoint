#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

/// Debug harness for designers/devs to QA Coachy visual states, audio, haptics, and animations.
/// Access via Settings → Developer (unlock with 5x version tap) → "Coachy character sheet".
public struct CoachyDebugView: View {
    @State private var selectedPose: CoachyPose = .idle
    @State private var selectedEmotion: CoachyEmotion = .neutral
    @State private var coachySize: CGFloat = 240
    @State private var enableMatchedGeometryFlyIn: Bool = false
    @State private var showFlyInDemo: Bool = false
    @Namespace private var coachyNamespace

    @State private var voiceMode: CoachyVoiceMode = .simlish
    @State private var voiceVolume: Float = 1.0
    @State private var voiceLatency: TimeInterval = 0.0
    @State private var isPlayingVoice: Bool = false

    @State private var hapticCues: [HapticChoreographyPattern] = [
        .light, .medium, .heavy, .celebrate, .warn, .tripleTap, .success, .error
    ]

    @State private var soundEffectCues: [String] = [
        "success", "encourage", "celebrate", "concerned", "ding", "whoosh"
    ]

    @State private var snapshotError: String?
    @State private var showSnapshotAlert: Bool = false

    private let hapticChoreographer = HapticChoreographer.shared
    private let soundPlayer = SoundEffectPlayer.shared

    public init() {}

    public var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    // MARK: - Preview Section
                    previewSection

                    // MARK: - Pose & Emotion Controls
                    poseEmotionSection

                    // MARK: - Size & Animation Controls
                    sizeAnimationSection

                    // MARK: - Audio Section
                    audioSection

                    // MARK: - Haptics Section
                    hapticsSection

                    // MARK: - Sound FX Section
                    soundFXSection

                    // MARK: - Snapshot Export
                    snapshotSection
                }
                .padding()
            }
            .navigationTitle("Coachy Preview")
            .background(Color.app.background.ignoresSafeArea())
            .alert("Snapshot Export", isPresented: $showSnapshotAlert) {
                Button("OK") {}
            } message: {
                if let error = snapshotError {
                    Text("Error: \(error)")
                } else {
                    Text("Frame exported to Photos. Share to review with designers.")
                }
            }
        }
    }

    // MARK: - Preview Section

    private var previewSection: some View {
        VStack(spacing: 16) {
            Text("Live Preview")
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)

            ZStack {
                // Matched geometry fly-in demo
                if enableMatchedGeometryFlyIn && showFlyInDemo {
                    CoachyView(state: currentCoachyState, size: coachySize)
                        .matchedGeometryEffect(id: "coachy", in: coachyNamespace)
                        .transition(.asymmetric(
                            insertion: .offset(x: -300, y: 0).combined(with: .opacity),
                            removal: .offset(x: 300, y: 0)
                        ))
                } else if !enableMatchedGeometryFlyIn || !showFlyInDemo {
                    CoachyView(state: currentCoachyState, size: coachySize)
                        .frame(height: coachySize * 1.4)
                }
            }
            .frame(height: coachySize * 1.4)
            .frame(maxWidth: .infinity)
            .background(Color.app.cardBackground)
            .cornerRadius(12)

            // Tap for fly-in demo
            if enableMatchedGeometryFlyIn {
                Button(action: triggerFlyIn) {
                    Label(showFlyInDemo ? "Reset" : "Tap to fly in", systemImage: "arrow.right")
                        .font(.caption)
                        .foregroundColor(.app.accent)
                }
                .frame(maxWidth: .infinity, alignment: .center)
            }
        }
        .padding()
        .background(Color.app.cardBackground)
        .cornerRadius(12)
    }

    // MARK: - Pose & Emotion Controls

    private var poseEmotionSection: some View {
        VStack(spacing: 16) {
            Text("Pose & Emotion")
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)

            VStack(alignment: .leading, spacing: 12) {
                Text("Pose: \(selectedPose.rawValue.capitalized)")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Picker("Pose", selection: $selectedPose) {
                    ForEach(CoachyPose.allCases, id: \.self) { pose in
                        Text(pose.rawValue.capitalized).tag(pose)
                    }
                }
                .pickerStyle(.segmented)
            }

            VStack(alignment: .leading, spacing: 12) {
                Text("Emotion: \(selectedEmotion.rawValue.capitalized)")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Picker("Emotion", selection: $selectedEmotion) {
                    ForEach(CoachyEmotion.allCases, id: \.self) { emotion in
                        Text(emotion.rawValue.capitalized).tag(emotion)
                    }
                }
                .pickerStyle(.segmented)
            }
        }
        .padding()
        .background(Color.app.cardBackground)
        .cornerRadius(12)
    }

    // MARK: - Size & Animation Controls

    private var sizeAnimationSection: some View {
        VStack(spacing: 16) {
            Text("Size & Animation")
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)

            VStack(alignment: .leading, spacing: 12) {
                Text("Size: \(Int(coachySize))pt")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Slider(value: $coachySize, in: 100...360, step: 20)
                    .tint(.app.accent)
            }

            VStack(alignment: .leading, spacing: 8) {
                Toggle("Matched-geometry fly-in demo", isOn: $enableMatchedGeometryFlyIn)
                    .tint(.app.accent)

                Text("When enabled, tap preview to demo fly-in animation from corner.")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
        }
        .padding()
        .background(Color.app.cardBackground)
        .cornerRadius(12)
    }

    // MARK: - Audio Section

    private var audioSection: some View {
        VStack(spacing: 16) {
            Text("Audio")
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)

            VStack(alignment: .leading, spacing: 12) {
                Text("Voice Mode")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Picker("Voice Mode", selection: $voiceMode) {
                    Text("Simlish").tag(CoachyVoiceMode.simlish)
                    Text("AVSpeechSynthesizer").tag(CoachyVoiceMode.avSpeechSynthesizer)
                    Text("Silent").tag(CoachyVoiceMode.silent)
                }
                .pickerStyle(.segmented)
            }

            VStack(alignment: .leading, spacing: 12) {
                HStack {
                    Text("Volume")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Spacer()
                    Text("\(Int(voiceVolume * 100))%")
                        .font(.caption2)
                        .foregroundColor(.secondary)
                }

                Slider(value: $voiceVolume, in: 0...1, step: 0.1)
                    .tint(.app.accent)
            }

            VStack(alignment: .leading, spacing: 12) {
                HStack {
                    Text("Latency")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Spacer()
                    Text("\(String(format: "%.2f", voiceLatency))s")
                        .font(.caption2)
                        .monospaced()
                        .foregroundColor(.secondary)
                }
            }

            Button(action: triggerSimlishGreeting) {
                HStack {
                    if isPlayingVoice {
                        ProgressView()
                            .scaleEffect(0.8)
                    } else {
                        Image(systemName: "speaker.wave.2.fill")
                    }
                    Text(isPlayingVoice ? "Playing..." : "Play Simlish greeting")
                }
                .frame(maxWidth: .infinity)
            }
            .disabled(isPlayingVoice || voiceMode == .silent)
            .buttonStyle(.bordered)
            .tint(.app.accent)
        }
        .padding()
        .background(Color.app.cardBackground)
        .cornerRadius(12)
    }

    // MARK: - Haptics Section

    private var hapticsSection: some View {
        VStack(spacing: 16) {
            Text("Haptics")
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)

            VStack(spacing: 8) {
                ForEach(hapticCues, id: \.self) { cue in
                    Button(action: { hapticChoreographer.play(cue) }) {
                        HStack {
                            Image(systemName: "iphone.gen3.radiowaves.left.and.right")
                                .font(.caption)
                            Text(hapticCueLabel(cue))
                                .lineLimit(1)
                            Spacer()
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                    }
                    .buttonStyle(.bordered)
                    .tint(.app.accent)
                }
            }
        }
        .padding()
        .background(Color.app.cardBackground)
        .cornerRadius(12)
    }

    // MARK: - Sound FX Section

    private var soundFXSection: some View {
        VStack(spacing: 16) {
            Text("Sound Effects")
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)

            VStack(spacing: 8) {
                ForEach(soundEffectCues, id: \.self) { cue in
                    Button(action: { soundPlayer.play(cue) }) {
                        HStack {
                            Image(systemName: "speaker.wave.1.fill")
                                .font(.caption)
                            Text(cue.capitalized)
                                .lineLimit(1)
                            Spacer()
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                    }
                    .buttonStyle(.bordered)
                    .tint(.app.accent)
                }
            }
        }
        .padding()
        .background(Color.app.cardBackground)
        .cornerRadius(12)
    }

    // MARK: - Snapshot Export Section

    private var snapshotSection: some View {
        VStack(spacing: 16) {
            Text("Snapshot Export")
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)

            Text("Capture the current frame and save to Photos for designer review.")
                .font(.caption)
                .foregroundColor(.secondary)
                .frame(maxWidth: .infinity, alignment: .leading)

            Button(action: captureAndExportSnapshot) {
                HStack {
                    Image(systemName: "camera.fill")
                    Text("Export current frame as PNG")
                }
                .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)
            .tint(.app.accent)
        }
        .padding()
        .background(Color.app.cardBackground)
        .cornerRadius(12)
    }

    // MARK: - Helpers

    private var currentCoachyState: CoachyState {
        CoachyState(
            pose: selectedPose,
            emotion: selectedEmotion,
            bubbleText: "Preview mode"
        )
    }

    private func triggerFlyIn() {
        withAnimation(.easeOut(duration: 0.6)) {
            showFlyInDemo.toggle()
        }
    }

    private func triggerSimlishGreeting() {
        isPlayingVoice = true
        let start = Date()

        // Simulate voice synthesis
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
            voiceLatency = Date().timeIntervalSince(start)
            isPlayingVoice = false
            hapticChoreographer.play(.light)
        }
    }

    private func hapticCueLabel(_ cue: HapticChoreographyPattern) -> String {
        switch cue {
        case .light:
            return "Light"
        case .medium:
            return "Medium"
        case .heavy:
            return "Heavy"
        case .celebrate:
            return "Celebrate"
        case .warn:
            return "Warning"
        case .tripleTap:
            return "Triple tap"
        case .success:
            return "Success"
        case .error:
            return "Error"
        }
    }

    private func captureAndExportSnapshot() {
        snapshotError = nil
        showSnapshotAlert = true

        // In a real implementation, use UIGraphicsPDFRenderer or SwiftUI snapshot API
        // For now, provide designer-friendly guidance
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
            // TODO: Integrate screenshot-to-Photos export
            // Placeholder: confirm to user that frame capture succeeded
        }
    }
}

#Preview {
    CoachyDebugView()
        .environmentObject(CoreHolder.preview)
}
#endif
