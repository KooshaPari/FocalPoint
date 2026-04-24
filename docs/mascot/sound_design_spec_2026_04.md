# Sound Design Specification — FocalPoint iOS

**Goal:** Define every Coachy-related sound effect, their triggers, and their pairings with haptic patterns and animations.

**Current state:** No sound effects. Silence on all interactions.

---

## 1. Sound Library & Sourcing

### Free/Licensable Sources

**Sources:**
- [Zapsplat — Free Sound Effects](https://www.zapsplat.com/), accessed 2026-04-23. Over 150,000 sounds, royalty-free for non-commercial (commercial license ~$30/year).
- [Freesound.org](https://freesound.org/), accessed 2026-04-23. Community-uploaded, Creative Commons licensed sounds.
- [Splice](https://splice.com/), accessed 2026-04-23. Royalty-free sound packs and loops ($9–15/month).

### Recommended Strategy

1. **Bootstrap from Zapsplat:** Free tier covers most SFX (session start chime, focus beep, penalty warning, etc.). No licensing overhead for MVP.
2. **Upgrade to paid commercial license** (~$30–50/year) once past beta. Covers all commercial use.
3. **Splice for ambient loops** (focus-mode background hum, session-complete fanfare) if production quality is needed.

---

## 2. Sound Effect Catalog

All sounds are **16-bit PCM WAV at 48 kHz** (standard iOS audio), converted to **compressed m4a (AAC)** for app bundle (~50–100 KB per sound, vs. 500 KB for uncompressed).

| Sound ID | Trigger Event | Description | Zapsplat Link / Alternative | Duration | BPM/Tempo | Mix Level | Haptic Pairing | Loop Points |
|----------|---------------|-------------|--------|----------|-----------|-----------|----------------|------------|
| **session-start-chime** | Focus session begins (user starts timer) | Clear, high-pitched bell chime (like a meditation bell). Single strike, natural decay. | [Meditation bell strike](https://www.zapsplat.com/sound-effect-categories/bells/) | 1.2 s | — | 0.8 | Light tap (.light) | N/A |
| **focus-ambient-loop** | Focus session active (continuous background) | Soft ambient drone, very subtle (30–40 dB below other SFX). Could be a rain loop, forest ambience, or pure sine-wave pulse at sub-audible 40 Hz. | [Ambient rain loop](https://www.zapsplat.com/sound-effect-categories/ambient-soundscapes/) or [Brown noise](https://freesound.org/search/?q=brown+noise&f=duration:%5B10%20TO%20%2A%5D) | 60 s (loops) | ~0.5 Hz (pulses) | 0.15 (very quiet) | None (no interruption) | Set fade-in/fade-out 2.0 s |
| **session-complete-fanfare** | Focus session timer ends (user completed a session) | Triumphant short fanfare, 3-note ascending brass (like a victory flourish). 1–2 s, celebratory tone. | [Victory fanfare, short](https://www.zapsplat.com/sound-effect-categories/success-failure/) | 1.8 s | 120 BPM | 0.9 | Triple success (.heavy, .light, .heavy) | N/A |
| **streak-extension-sparkle** | Daily ritual completed; streak counter increments | Sparkling, magical ascending tones. 4–6 notes, each note slightly higher pitch (pentatonic scale). Glassy, crystalline texture. | [Magic sparkle, ascending](https://www.zapsplat.com/sound-effect-categories/magic-sfx/) | 0.9 s | — | 0.75 | Medium tap × 3 (.medium) | N/A |
| **credit-earned-coin-clink** | Reward added to wallet (credit earned) | Bright coin drop/clink sound. Single metallic "ting" with subtle reverb (like coin landing on marble). | [Coin drop / clink](https://www.zapsplat.com/sound-effect-categories/coins-money/) | 0.4 s | — | 0.7 | Success pulse (.heavy) | N/A |
| **rule-fire-whoosh** | Rule evaluates and fires (penalty or reward instant) | Swooshing air sound, brief wind gust. Doppler-ish, like something fast passes by. | [Whoosh / air rush](https://www.zapsplat.com/sound-effect-categories/whoosh-sfx/) | 0.5 s | — | 0.7 | Medium tap (.medium) | N/A |
| **penalty-escalates-warning-tone** | Penalty tier increases (approaching lockdown) | Three ascending warning beeps (like a microwave or alert). Each beep slightly higher pitch, urgent. | [Alert beep × 3](https://www.zapsplat.com/sound-effect-categories/warning-alarms/) or [Microwave beep](https://freesound.org/search/?q=beep+warning) | 0.9 s | ~200 BPM (fast) | 0.85 | Warning × 2 (.medium, then decay) | N/A |
| **lockdown-engaged-lock-thunk** | Penalty reaches MAX; screen lock active | Heavy mechanical lock click (like a padlock snapping shut). Deep, authoritative, slightly ominous. | [Lock / padlock click](https://www.zapsplat.com/sound-effect-categories/mechanical-sfx/) | 0.6 s | — | 0.9 | Heavy + rumble pattern (haptic decay) | N/A |
| **bubble-appear-soft-pop** | Coachy speech bubble animates in | Soft, gentle pop sound. Subtle (not startling), like a small bubble inflating. | [Soft pop / bubble](https://www.zapsplat.com/sound-effect-categories/pop-bubble-sfx/) | 0.3 s | — | 0.5 | Light tap (.light) | N/A |
| **bubble-dismiss-soft-fade** | Coachy speech bubble animates out (user dismisses) | Tiny descending tone, like a pout or sad trombone (but brief, cute). | [Sad trombone, short](https://freesound.org/search/?q=sad+trombone+short) or [Pout sound](https://www.zapsplat.com/sound-effect-categories/voice-sfx/) | 0.3 s | — | 0.4 | None (transition only) | N/A |
| **coachy-speak-mouth-click** | Coachy Simlish speech plays (consonant articulation) | Tiny lip/mouth click (like popping the tongue). Procedurally triggers on consonant phonemes (/m/, /b/, /p/). Subtle, rhythmic. | [Mouth click, subtle](https://freesound.org/search/?q=mouth+click) or synthesized | 0.08 s (per click) | — | 0.3 | None (sub-audible) | N/A |
| **launch-wake-yawn-stretch** | App cold launch; Coachy wakes up (phase 2 of wake sequence) | Cute yawn + stretch sound combined. Yawn: descending tone (mouth opening), slight vocal undertone. Stretch: creaking, groaning sound (light, not aggressive). Total ~1 s. | [Yawn + stretch combined](https://freesound.org/search/?q=yawn+stretch) or [Creature yawn](https://www.zapsplat.com/sound-effect-categories/creature-sfx/) | 1.2 s | — | 0.6 | Light tap × 2 (.light, .light) | N/A |
| **sleep-disappointed-sigh** | Coachy sleepy/disappointed pose; ritualization complete with 0 rituals done | Sad sigh sound (human-like, but cute/cartoonish). Descending pitch, soft exhale. | [Sigh, sad](https://freesound.org/search/?q=sigh+disappointed) | 0.7 s | — | 0.5 | None (mood only) | N/A |
| **auth-sparkle** | OAuth callback completes; token exchange successful | Brief shimmer/sparkle (like pixie dust). 2–3 ascending chimes at different pitches, very quick. | [Magic sparkle, quick](https://www.zapsplat.com/sound-effect-categories/magic-sfx/) | 0.4 s | — | 0.6 | Light tap (.light) | N/A |
| **sync-whoosh** | Connector sync starts/completes | Whooshing, data-transfer-like sound. Ascending sweep (like a scanner beam). | [Sci-fi swoosh / scan](https://www.zapsplat.com/sound-effect-categories/sci-fi-sfx/) | 0.6 s | — | 0.65 | Light tap (.light) | N/A |
| **think-hum** | Coachy in thinking/curious pose; rituals generation in progress | Soft, intellectual hum (like a computer thinking). Subtle pitch modulation (0.5 Hz wobble). | [Computer beep / processing hum](https://www.zapsplat.com/sound-effect-categories/computer-sfx/) or [Mind thinking tone](https://freesound.org/search/?q=thinking+hum) | 1.5 s (loops during load) | — | 0.4 | None (background) | Fade-in/out 1.0 s |
| **task-clink** | Task creation submitted and stored | Single, bright metallic clink (like a pen tapping glass). Crisp, clear, satisfying. | [Metal clink / tap](https://www.zapsplat.com/sound-effect-categories/metal-sfx/) | 0.3 s | — | 0.6 | Medium success (.medium) | N/A |
| **install-pop** | Ritual template installed (user adds ritual from template) | Cheerful pop or plop sound, like a cork popping or a bubble burst (but positive, celebratory). | [Pop / burst, cheerful](https://www.zapsplat.com/sound-effect-categories/pop-bubble-sfx/) | 0.4 s | — | 0.65 | Medium tap (.medium) | N/A |
| **stats-whoosh** | Stats reload begins or completes; pull-to-refresh | Smooth data-transfer whoosh (like a sliding door or sliding papers). Quick, clean. | [Sci-fi door open / slide](https://www.zapsplat.com/sound-effect-categories/sci-fi-sfx/) | 0.5 s | — | 0.6 | Light tap (.light) | N/A |

---

## 3. Haptic-Sound Pairing Details

Each sound is paired with a haptic pattern via `UIImpactFeedbackGenerator` or `UINotificationFeedbackGenerator` (iOS 10+).

| Sound ID | Haptic Pattern | Feedback Generator | Intensity |
|----------|----------------|-------------------|-----------|
| **session-start-chime** | Light tap | `UIImpactFeedbackGenerator` | `.light` |
| **focus-ambient-loop** | None (continuous, no interruption) | — | — |
| **session-complete-fanfare** | Triple heavy pulse: heavy → light → heavy | `UIImpactFeedbackGenerator` ×3 | `.heavy`, `.light`, `.heavy` |
| **streak-extension-sparkle** | Tap × 3, medium | `UIImpactFeedbackGenerator` ×3 | `.medium` ×3 |
| **credit-earned-coin-clink** | Success pulse | `UINotificationFeedbackGenerator` | `.success` |
| **rule-fire-whoosh** | Medium tap | `UIImpactFeedbackGenerator` | `.medium` |
| **penalty-escalates-warning-tone** | Warning ×2, then decay | `UINotificationFeedbackGenerator` then manual decay | `.warning` → pattern fade |
| **lockdown-engaged-lock-thunk** | Heavy tap + rumble decay pattern | `UIImpactFeedbackGenerator` + custom pattern | `.heavy` + 5-step decay |
| **bubble-appear-soft-pop** | Light tap | `UIImpactFeedbackGenerator` | `.light` |
| **bubble-dismiss-soft-fade** | None (transition only) | — | — |
| **coachy-speak-mouth-click** | None (sub-audible) | — | — |
| **launch-wake-yawn-stretch** | Tap × 2 (medium delay) | `UIImpactFeedbackGenerator` ×2 | `.light`, `.light` (0.3 s apart) |
| **sleep-disappointed-sigh** | None (mood only) | — | — |
| **auth-sparkle** | Light tap | `UIImpactFeedbackGenerator` | `.light` |
| **sync-whoosh** | Light tap on start | `UIImpactFeedbackGenerator` | `.light` |
| **think-hum** | None (background) | — | — |
| **task-clink** | Medium success | `UINotificationFeedbackGenerator` | `.success` (medium intensity) |
| **install-pop** | Medium tap | `UIImpactFeedbackGenerator` | `.medium` |
| **stats-whoosh** | Light tap on dismiss | `UIImpactFeedbackGenerator` | `.light` |

---

## 4. Audio File Format & Delivery

### Codec & Sample Rate

- **Format:** AAC (m4a) in an `.m4a` container.
- **Sample rate:** 48 kHz (standard iOS).
- **Bit rate:** 128 kbps (good quality, manageable file size).
- **Mono:** Yes (all sounds are mono, spatialization not needed for FocalPoint MVP).

### File Naming & Organization

```
Resources/Sounds/
  SoundEffects/
    session-start-chime.m4a
    focus-ambient-loop.m4a
    session-complete-fanfare.m4a
    streak-extension-sparkle.m4a
    credit-earned-coin-clink.m4a
    rule-fire-whoosh.m4a
    penalty-escalates-warning-tone.m4a
    lockdown-engaged-lock-thunk.m4a
    bubble-appear-soft-pop.m4a
    bubble-dismiss-soft-fade.m4a
    coachy-speak-mouth-click.m4a
    launch-wake-yawn-stretch.m4a
    sleep-disappointed-sigh.m4a
    auth-sparkle.m4a
    sync-whoosh.m4a
    think-hum.m4a
    task-clink.m4a
    install-pop.m4a
    stats-whoosh.m4a
  
  Mascot/
    simlish-phonemes/
      m.m4a
      a.m4a
      b.m4a
      ... (12-16 phoneme files per §2 of talking_coachy_spec)
```

### App Bundle Size Impact

- **18 SFX × 50 KB avg:** ~900 KB.
- **16 Simlish phonemes × 30 KB avg:** ~480 KB.
- **Total audio assets:** ~1.4 MB (acceptable for iOS app; users expect 200–500 MB app bundles).

---

## 5. Sound Playback Architecture (iOS)

### Swift Implementation

```swift
/// Sound effect manager with haptic synchronization.
class SoundEffectPlayer: NSObject, AVAudioPlayerDelegate {
    static let shared = SoundEffectPlayer()
    
    private var audioPlayers: [String: AVAudioPlayer] = [:]
    private let feedbackGenerator = UIImpactFeedbackGenerator()
    private let notificationGenerator = UINotificationFeedbackGenerator()
    
    override init() {
        super.init()
        setupAudioSession()
    }
    
    private func setupAudioSession() {
        let session = AVAudioSession.sharedInstance()
        try? session.setCategory(.ambient, options: [.defaultToSpeaker, .mixWithOthers])
        try? session.setActive(true)
    }
    
    /// Play a named sound effect with optional haptic feedback.
    func play(_ soundId: String, hapticPattern: HapticPattern? = nil) {
        guard let url = Bundle.main.url(forResource: soundId, withExtension: "m4a", subdirectory: "Sounds/SoundEffects") else {
            print("❌ Sound not found: \(soundId)")
            return
        }
        
        do {
            let player = try AVAudioPlayer(contentsOf: url)
            player.delegate = self
            audioPlayers[soundId] = player
            
            // Play audio
            player.play()
            
            // Trigger haptic if specified
            if let hapticPattern = hapticPattern {
                playHaptic(hapticPattern)
            }
        } catch {
            print("❌ Failed to play sound: \(error)")
        }
    }
    
    /// Play looping ambient sound (e.g., focus-ambient-loop).
    func playAmbient(_ soundId: String, fadeInDuration: TimeInterval = 1.0) {
        guard let url = Bundle.main.url(forResource: soundId, withExtension: "m4a", subdirectory: "Sounds/SoundEffects") else {
            return
        }
        
        do {
            let player = try AVAudioPlayer(contentsOf: url)
            player.numberOfLoops = -1 // Loop indefinitely
            player.volume = 0.0 // Start silent
            player.play()
            audioPlayers[soundId] = player
            
            // Fade in
            let fadeSteps = Int(fadeInDuration * 30.0) // ~30 FPS
            let volumeStep = 0.4 / Float(fadeSteps) // Target volume 0.4 (quiet)
            
            DispatchQueue.main.asyncAfter(deadline: .now()) {
                var step = 0
                let timer = Timer.scheduledTimer(withTimeInterval: 1.0 / 30.0, repeats: true) { _ in
                    step += 1
                    player.volume = min(0.4, player.volume + volumeStep)
                    if step >= fadeSteps {
                        // Timer will stop automatically
                    }
                }
            }
        } catch {
            print("❌ Failed to play ambient: \(error)")
        }
    }
    
    /// Stop ambient sound with fade-out.
    func stopAmbient(_ soundId: String, fadeOutDuration: TimeInterval = 1.0) {
        guard let player = audioPlayers[soundId] else { return }
        
        let fadeSteps = Int(fadeOutDuration * 30.0)
        let volumeStep = player.volume / Float(fadeSteps)
        
        var step = 0
        let timer = Timer.scheduledTimer(withTimeInterval: 1.0 / 30.0, repeats: true) { _ in
            step += 1
            player.volume = max(0.0, player.volume - volumeStep)
            if step >= fadeSteps {
                player.stop()
                self.audioPlayers[soundId] = nil
            }
        }
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
            // Heavy, light, heavy at 200ms intervals
            let gen = UIImpactFeedbackGenerator(style: .heavy)
            gen.impactOccurred()
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
                UIImpactFeedbackGenerator(style: .light).impactOccurred()
            }
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.4) {
                UIImpactFeedbackGenerator(style: .heavy).impactOccurred()
            }
        case .decayPattern(let startIntensity, let steps):
            var gen = UIImpactFeedbackGenerator(style: startIntensity)
            gen.impactOccurred()
            for i in 1..<steps {
                let delay = TimeInterval(i) * 0.1
                DispatchQueue.main.asyncAfter(deadline: .now() + delay) {
                    // Decay intensity by reducing feedback repetitions
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
        // Remove from active players
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
```

### Invocation (from CoachyScene / Events)

```swift
// Trigger sound + haptic when Coachy rule-fire event fires
SoundEffectPlayer.shared.play(
    "rule-fire-whoosh",
    hapticPattern: .mediumTap
)

// Trigger ambient sound during focus session
SoundEffectPlayer.shared.playAmbient("focus-ambient-loop", fadeInDuration: 1.0)

// Stop ambient on session end
SoundEffectPlayer.shared.stopAmbient("focus-ambient-loop", fadeOutDuration: 0.5)
```

---

## 6. Simlish Phoneme Audio (Tier 0)

### Phoneme Library Sourcing

For Simlish voice (week 1 delivery), record or source 12–16 phoneme samples:

| Phoneme | Sample Filename | Source Strategy | Duration | Pitch Range (Hz) |
|---------|-----------------|-----------------|----------|-------------------|
| **/m/** | phoneme-m.m4a | Record "mmmm" sustain; clip 0.1s | 0.1 s | 100–200 (low hum) |
| **/a/** | phoneme-a.m4a | Record "ahhh" sustain; clip 0.12s | 0.12 s | 200–400 (open vowel) |
| **/b/** | phoneme-b.m4a | Record "buh" stop; clip 0.08s | 0.08 s | 150–300 |
| **/e/** | phoneme-e.m4a | Record "ehhh" sustain; clip 0.12s | 0.12 s | 300–500 |
| **/i/** | phoneme-i.m4a | Record "eee" sustain; clip 0.12s | 0.12 s | 400–600 (high vowel) |
| **/g/** | phoneme-g.m4a | Record "guh" velar stop; clip 0.08s | 0.08 s | 100–300 |
| **/o/** | phoneme-o.m4a | Record "ohh" sustain; clip 0.12s | 0.12 s | 200–400 |
| **/u/** | phoneme-u.m4a | Record "ooo" sustain; clip 0.12s | 0.12 s | 100–300 (low vowel) |
| **/n/** | phoneme-n.m4a | Record "nnn" nasal; clip 0.1s | 0.1 s | 100–200 |
| **/p/** | phoneme-p.m4a | Record "puh" bilabial stop; clip 0.06s | 0.06 s | 150–300 |
| **/ʃ/** (sh) | phoneme-sh.m4a | Record "shhhh" fricative; clip 0.15s | 0.15 s | 2000–4000 (high-frequency hiss) |
| **/t/** | phoneme-t.m4a | Record "tuh" alveolar stop; clip 0.06s | 0.06 s | 200–400 |
| **/d/** | phoneme-d.m4a | Record "duh" voiced alveolar; clip 0.08s | 0.08 s | 200–400 |
| **/w/** | phoneme-w.m4a | Record "wuh" glide; clip 0.08s | 0.08 s | 100–300 |
| **/l/** | phoneme-l.m4a | Record "luh" lateral; clip 0.10s | 0.10 s | 200–400 |
| **/sil/** (silence) | phoneme-sil.m4a | 50ms silence / empty file | 0.05 s | — |

### Sourcing Options

1. **Record in-house:** If a designer / voice actor is available, record phonemes directly into a DAW (e.g., Reaper, Audacity free). This gives full creative control.
   - Time: 1–2 hours (record + edit 16 phonemes).
   - Cost: None (if in-house).

2. **Procedurally synthesize:** Use Apple's AVSpeechSynthesizer or festival (open-source TTS) to generate phoneme audio files offline.
   - Time: 30 min (write script to generate all phonemes).
   - Cost: None.

3. **Freesound/Splice packs:** Search for "phoneme pack" or "voice synthesis samples" on Freesound; many producers share isolated phoneme sets.
   - Time: 30 min (search + download + verify).
   - Cost: Free (Creative Commons) or $9–15/month (Splice).

**Recommendation:** Option 2 (synthesize with AVSpeechSynthesizer) for MVP. Highest ROI (time + cost).

---

## 7. Privacy Manifest (Audio-Related)

Since FocalPoint uses AVAudioSession, AVAudioPlayer, and will eventually use cloud TTS (Tier 2), include these declarations:

```xml
<key>NSPrivacyTracking</key>
<false/>

<!-- For Tier 0 (on-device audio only) -->
<!-- No data transmission; no required declarations. -->

<!-- For Tier 2+ (cloud TTS like ElevenLabs) -->
<key>NSPrivacyAccessedAPITypes</key>
<array>
    <dict>
        <key>NSPrivacyAccessedAPIType</key>
        <string>NSPrivacyAccessedAPISpeechSynthesis</string>
        <key>NSPrivacyAccessedAPITypeReasons</key>
        <array>
            <string>E703</string> <!-- Speech synthesis -->
        </array>
    </dict>
</array>
```

---

## 8. Audio Settings & User Control

### Settings UI (SettingsView update)

```swift
Section("Audio") {
    Toggle("Coachy voice", isOn: $coachyVoiceEnabled)
    
    Toggle("Sound effects", isOn: $soundEffectsEnabled)
    if soundEffectsEnabled {
        Slider("Sound volume", value: $sfxVolume, in: 0...1)
            .help("Volume for all sound effects and Coachy speech")
    }
    
    Toggle("Haptic feedback", isOn: $hapticEnabled)
    if hapticEnabled {
        Picker("Haptic intensity", selection: $hapticIntensity) {
            Text("Light").tag(UIImpactFeedbackGenerator.FeedbackStyle.light)
            Text("Medium").tag(UIImpactFeedbackGenerator.FeedbackStyle.medium)
            Text("Heavy").tag(UIImpactFeedbackGenerator.FeedbackStyle.heavy)
        }
    }
    
    Toggle("Ambient focus loop", isOn: $ambientLoopEnabled)
    if ambientLoopEnabled {
        Slider("Loop volume", value: $ambientVolume, in: 0...0.5)
    }
    
    Button("Test sounds") {
        testSoundAndHapticSequence()
    }
}

private func testSoundAndHapticSequence() {
    // Play a short sound + haptic combo for user validation
    SoundEffectPlayer.shared.play("session-start-chime", hapticPattern: .lightTap)
    DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
        SoundEffectPlayer.shared.play("credit-earned-coin-clink", hapticPattern: .success)
    }
}
```

---

## 9. Audio Focus & Interruption Handling

### AVAudioSession Configuration

When FocalPoint sound is playing (especially ambient loop during focus session), honor system interruptions (phone call, alerts, music):

```swift
func setupAudioSession() {
    let session = AVAudioSession.sharedInstance()
    
    // Category: .ambient — mixes with system audio, respects mute switch
    // Options: .defaultToSpeaker (output to speaker, not receiver), .mixWithOthers (allow concurrent playback)
    try? session.setCategory(.ambient, options: [.defaultToSpeaker, .mixWithOthers])
    try? session.setActive(true)
    
    // Handle interruptions (e.g., incoming call, Siri)
    NotificationCenter.default.addObserver(
        self,
        selector: #selector(handleAudioInterruption(_:)),
        name: AVAudioSession.interruptionNotification,
        object: session
    )
}

@objc private func handleAudioInterruption(_ notification: Notification) {
    guard let userInfo = notification.userInfo,
          let interruptionTypeValue = userInfo[AVAudioSessionInterruptionTypeKey] as? UInt,
          let interruptionType = AVAudioSession.InterruptionType(rawValue: interruptionTypeValue) else {
        return
    }
    
    switch interruptionType {
    case .began:
        // Pause ambient loop on interruption
        SoundEffectPlayer.shared.stopAmbient("focus-ambient-loop", fadeOutDuration: 0.3)
    case .ended:
        // Resume on interruption end (optional; ask user if desired)
        if UserDefaults.standard.bool(forKey: "ambientLoopEnabled") {
            SoundEffectPlayer.shared.playAmbient("focus-ambient-loop", fadeInDuration: 0.5)
        }
    @unknown default:
        break
    }
}
```

---

## 10. Accessibility Considerations

### Silent / Visual-Only Mode

Ensure all sound + haptic pairs have visual equivalents:

1. **Haptic without sound:** If user disables SFX but keeps haptic, they still get tactile feedback (useful for accessibility).
2. **Sound without haptic:** For hearing-impaired users, ensure visual cues (screen flash, animation intensity) accompany sounds.
3. **Silent mode:** Disable all audio + haptic; rely on animation, bubble text, and screen state changes.

### Audio Descriptions (Accessibility)

For each sound, provide a description in Accessibility inspector:

```swift
SoundEffectPlayer.shared.play("session-complete-fanfare", hapticPattern: .tripleSuccess)
// Accessibility hint: "Session complete. Fanfare sound plays with triple haptic pulse."
```

---

## 11. Testing Checklist

- [ ] All 19 SFX load from bundle without errors.
- [ ] Simlish phonemes concatenate and playback without artifacts.
- [ ] Haptic patterns trigger simultaneously with audio (no lag).
- [ ] Ambient loop fades in/out smoothly (no pops or clicks).
- [ ] Audio interruptions (phone call, Siri) pause playback correctly.
- [ ] Settings toggles (voice, SFX, haptic) persist across app restarts.
- [ ] Volume sliders in Settings apply to all SFX and voice dynamically.
- [ ] "Test sounds" button in Settings plays all three types: SFX, voice, ambient.
- [ ] Accessibility mode (silent) disables all audio/haptic without crashes.
- [ ] Privacy Manifest declares audio APIs correctly; App Store submission passes.

---

## 12. Implementation Timeline

| Phase | Week | Component | Effort |
|-------|------|-----------|--------|
| **Tier 0** | 1 | Source/synthesize 19 SFX + 16 phonemes | 4 h |
| | 1 | SoundEffectPlayer class + AVAudioSession | 3 h |
| | 1 | Simlish phoneme concatenation (Rust) | 2 h |
| | 1 | Settings UI (audio toggles/sliders) | 2 h |
| | 1 | PrivacyInfo.xcprivacy update | 0.5 h |
| **Tier 2+** | 8+ | ElevenLabs TTS integration + audio streaming | 4 h |
| | 8+ | Privacy Manifest update for cloud TTS | 1 h |

**Total Tier 0: ~11.5 hours** (audio + SoundEffectPlayer + settings).

---

## 13. References

- [Zapsplat — Free Sound Effects](https://www.zapsplat.com/), accessed 2026-04-23.
- [Freesound.org — Community Sound Library](https://freesound.org/), accessed 2026-04-23.
- [Splice — Royalty-Free Sound Packs](https://splice.com/), accessed 2026-04-23.
- [AVAudioSession Documentation — Apple Developer](https://developer.apple.com/documentation/avfoundation/avaaudiosession), accessed 2026-04-23.
- [UIImpactFeedbackGenerator — Apple Developer](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator), accessed 2026-04-23.
- [Mobile App Animation: 12 Patterns Every Designer Should Study](https://www.svgator.com/blog/mobile-apps-animation-examples/), accessed 2026-04-23.
