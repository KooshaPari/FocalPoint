import SwiftUI

/// Debug view listing all 8 cues with play buttons and haptic+audio toggles.
/// Matches #54 pattern: minimal, functional, test-focused.
struct CueDebugView: View {
    @State private var audioOnly = false
    @State private var hapticOnly = false

    var body: some View {
        NavigationStack {
            List {
                Section("Cue Settings") {
                    Toggle("Audio Only", isOn: $audioOnly)
                    Toggle("Haptic Only", isOn: $hapticOnly)
                }

                Section("Audio Cues (8 total, 119 KB)") {
                    ForEach(FocalPointCue.allCases, id: \.rawValue) { cue in
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text(cue.label)
                                    .font(.headline)
                                Text("\(String(format: "%.0f", cue.duration * 1000)) ms")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }

                            Spacer()

                            Button(action: { playCue(cue) }) {
                                HStack(spacing: 6) {
                                    Image(systemName: "speaker.wave.2.fill")
                                    Text("Play")
                                }
                                .font(.caption)
                                .padding(8)
                                .background(Color.blue.opacity(0.2))
                                .cornerRadius(6)
                            }
                        }
                        .padding(.vertical, 4)
                    }
                }

                Section("Audio Files") {
                    VStack(alignment: .leading, spacing: 8) {
                        ForEach(FocalPointCue.allCases, id: \.rawValue) { cue in
                            HStack {
                                Text(cue.rawValue)
                                    .font(.caption)
                                    .monospaced()
                                Spacer()
                                Text(".wav")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                        }
                    }
                    .padding(.vertical, 4)
                }

                Section("Haptic Patterns") {
                    VStack(alignment: .leading, spacing: 8) {
                        ForEach(FocalPointCue.allCases, id: \.rawValue) { cue in
                            HStack {
                                Text(cue.rawValue)
                                    .font(.caption)
                                    .monospaced()
                                Spacer()
                                Text(hapticPatternName(cue.hapticPattern))
                                    .font(.caption)
                                    .foregroundColor(.blue)
                            }
                        }
                    }
                    .padding(.vertical, 4)
                }
            }
            .navigationTitle("Audio Cues")
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    private func playCue(_ cue: FocalPointCue) {
        if audioOnly {
            CuePlayer.shared.playAudio(cue)
        } else if hapticOnly {
            cue.hapticPattern.play()
        } else {
            CuePlayer.shared.play(cue)
        }
    }

    private func hapticPatternName(_ pattern: CueHapticPattern) -> String {
        switch pattern {
        case .ascendingChime: return "Ascending Chime"
        case .resonantBell: return "Resonant Bell"
        case .descendingPulse: return "Descending Pulse"
        case .calmFadeIn: return "Calm Fade-In"
        case .resolvingDoubleTap: return "Resolving Double-Tap"
        case .sharpBuzz: return "Sharp Buzz"
        case .percussiveTick: return "Percussive Tick"
        case .friendlyBlip: return "Friendly Blip"
        }
    }
}

#Preview {
    CueDebugView()
}
