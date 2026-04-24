# Talking Coachy Specification — FocalPoint iOS

**Goal:** Enable Coachy to speak via tiered adoption: Simlish gibberish (week 1), accessible AVSpeechSynthesizer (week 3), premium ElevenLabs character voice (month 2), adaptive AI voice (long-term).

**Current state:** Coachy has static bubble text only. No audio output, no lip-sync, no voice personality.

---

## 1. On-Device TTS Options & Analysis

### Option A: Apple Built-In AVSpeechSynthesizer

**Source:** [AVSpeechSynthesizer Documentation — Apple Developer](https://developer.apple.com/documentation/avfoundation/speech-synthesis), accessed 2026-04-23.

**Characteristics:**
- Free (no API calls, no billing).
- System voices: ~20 language-specific voices included (US English, UK English, French, Spanish, etc.).
- Quality: adequate for accessibility, distinct personality per language but not customizable per app.
- Latency: ~100–300ms to first audio output.
- Rate/pitch control: `AVSpeechUtterance` supports `rate` (0.0–2.0, default 1.0) and `pitchMultiplier` (0.5–2.0).
- Data disclosure: **No data transmission** — all synthesis happens on-device. Privacy manifest requires **no disclosure** for AVSpeechSynthesizer alone.

**iOS 17+:** Apple Neural TTS (via same API) offers improved quality voices ([WWDC 2023: Extend Speech Synthesis with personal and custom voices](https://developer.apple.com/videos/play/wwdc2023/10033/), accessed 2026-04-23). Automatically selected if available.

**Personal Voice (iOS 17.4+):** Users can create a synthetic version of their own voice and apps can opt-in to use it ([Using your Personal Voice in iOS](https://bendodson.com/weblog/2024/04/03/using-your-personal-voice-in-an-ios-app/), accessed 2026-04-23). Opt-in per app, stored on-device.

**Use case:** Tier 1 (fallback, accessibility).

---

### Option B: Cloud TTS Services

#### ElevenLabs

**Source:** [ElevenLabs API Pricing](https://elevenlabs.io/pricing/api), accessed 2026-04-23; [ElevenLabs Pricing Breakdown 2026](https://flexprice.io/blog/elevenlabs-pricing-breakdown), accessed 2026-04-23.

**Characteristics:**
- Voice cloning: Create a custom voice from ~30s of audio samples.
- Character voice selection: Choose from professional voices or your cloned voice.
- Quality: High; supports multiple languages and accents.
- Latency: 1–3s for 100–200 characters (streamed, not instant).
- Cost: API Pro tier $99/month = 100 credits; Multilingual v2 = 1 char = 1 credit.
  - **1,000 characters = 1,000 credits = $10–$99/month depending on tier.**
  - For a typical bubble ("Ready?") = 6 characters = $0.06 at scale rates.
  - Per 1,000-character estimate: **$0.50–$0.99 per 1K chars** (API Scale to API Pro).
- Data disclosure: **Requires Privacy Manifest disclosure:** "Speech synthesis" or "Audio processing" category; data sent to ElevenLabs servers.
- Commercial rights: Voice cloning requires Creator tier ($22/month) or higher for commercial use.

**Use case:** Tier 2 (premium unlock).

---

#### OpenAI TTS (GPT-4o audio)

**Characteristics:**
- Same API as text generation but with audio output.
- 6 built-in voices (good quality, limited personality).
- Cost: $0.015 per 1K input tokens, $0.06 per 1K output tokens (variable by model). For audio, rough estimate: **$0.10–0.15 per 1K characters** (cheaper than ElevenLabs but less customizable).
- Latency: 2–5s.
- Data disclosure: **Requires disclosure** (sent to OpenAI servers).

**Use case:** Alternative to ElevenLabs (not recommended over ElevenLabs for character voice).

---

#### Play.ht, Resemble.ai

**Characteristics:**
- Similar pricing and latency to ElevenLabs.
- Play.ht: Better real-time streaming, slightly cheaper ($10/month for 100K characters).
- Resemble.ai: Excellent voice cloning, ~$20/month for custom voice.

**Use case:** Secondary options if ElevenLabs unavailable.

---

### Option C: On-Device Neural TTS (Privacy-First)

#### Piper TTS (Open Source)

**Source:** [Piper — Open-Source Text-to-Speech](https://github.com/rhasspy/piper), accessed 2026-04-23.

**Characteristics:**
- Lightweight, offline neural TTS (ONNX models).
- Bundle size: 20–50 MB per voice (vs. 1–2 MB for system voices).
- Quality: Good (comparable to commercial TTS, but less expressive).
- Latency: 1–2s on-device (slower than system TTS but no network round-trip).
- Languages: Multiple, but English voice selection limited.
- No privacy concerns: runs entirely on-device.

**Verdict:** Viable for long-term (Tier 3) but not Tier 0 due to bundle size. Shelve for now.

---

## 2. Simlish-Style Generative Gibberish (Tier 0)

### Concept

Instead of real speech, generate phonetically plausible but nonsensical "Simlish" (inspired by *The Sims* and *Animal Crossing*). Procedural phoneme sequencing timed to text length, pitch-shifted per character personality.

**Advantages:**
- Zero latency; runs on-device.
- Instantly recognizable Coachy personality.
- No privacy concerns; no external calls.
- Shippable in 1 week.
- No TTS licensing or billing.

**Sources:**
- [How The Sims Uses Phonemes for Character Voice](https://en.wikipedia.org/wiki/The_Sims#Audio_and_music), general knowledge on procedural phoneme generation, accessed 2026-04-23.

### Architecture

1. **Phoneme inventory:** Define 12–16 phonemes as .m4a samples (0.1–0.3s each):
   - Vowels: /a/, /e/, /i/, /o/, /u/, /ɔ/ (all)
   - Consonants: /m/, /n/, /b/, /g/, /p/, /t/, /d/, /ʃ/ (sh), /l/, /w/, /j/ (y)
   - Silence: /sil/ (0.05s rest)

2. **Simlish word generation:**
   - Input: plain-text bubble (e.g., "Ready?")
   - Algorithm:
     1. Calculate duration: `duration_s = text.count * 0.15 + 0.2` (approx. 1 char = 150ms + 200ms intro).
     2. Generate random phoneme sequence of length N such that total duration ≈ `duration_s`.
     3. Sequence rules:
        - Start with a consonant (70% chance): pick from {/m/, /b/, /g/, /w/}.
        - Alternate consonant-vowel-consonant-vowel for naturalness.
        - Add occasional silence phonemes (5% chance per slot) for pausing effect.
        - Avoid consonant clusters (CCC impossible; CC rare).
   4. Map each phoneme to a pre-recorded .m4a file; concatenate into a single audio buffer.
   5. Apply pitch shift based on Coachy's emotion:
      - `.happy`, `.excited`, `.proud`: pitch up +2 semitones.
      - `.neutral`: pitch 0.
      - `.concerned`, `.disappointed`: pitch down -1 semitone.
      - `.focused`, `.tired`: pitch down -1.5 semitones.

3. **Mouth-shape mapping (lip-sync):**
   - Derive mouth shape from phoneme:
     - Vowel phonemes (/a/, /e/, /i/, /o/, /u/): map to mouth-open states (heights 0.5–1.0 in Rive).
     - /m/, /b/, /p/: closed mouth (height 0.0).
     - /ʃ/ (sh), /j/ (y): partially open (height 0.3).
   - Animate Rive `mouth_open` blend shape parameter as phoneme sequence plays.
   - Sync timing: mouth opens 50ms before phoneme audio plays, holds 80% of phoneme duration, closes 50ms after.

### Implementation

**Swift side:**

```swift
/// Generates Simlish audio and synchronized mouth-shape keyframes.
struct SimlishGenerator {
    /// Pre-loaded phoneme audio files (cached)
    private let phonemeLibrary: [String: AVAudioFile] // key: "m", "a", "b", etc.
    
    /// Generate Simlish word audio and mouth animations.
    func generateSpeech(
        text: String,
        emotion: CoachyEmotion
    ) -> (audioBuffer: AVAudioPCMBuffer, mouthKeyframes: [MouthKeyframe]) {
        
        // Step 1: Calculate target duration
        let targetDuration = Double(text.count) * 0.15 + 0.2
        
        // Step 2: Generate phoneme sequence
        let phonemeSequence = generatePhonemeSequence(targetDuration: targetDuration)
        
        // Step 3: Concatenate audio + extract timings
        let audioBuffer = concatenatePhonemes(phonemeSequence)
        let phonemeTimes = extractPhonemeTimings(phonemeSequence)
        
        // Step 4: Pitch shift based on emotion
        let pitchShift = pitchForEmotion(emotion)
        let shiftedBuffer = applyPitchShift(audioBuffer, pitchShift)
        
        // Step 5: Derive mouth keyframes
        let mouthKeyframes = deriveMouthKeyframes(phonemeSequence, phonemeTimes)
        
        return (shiftedBuffer, mouthKeyframes)
    }
    
    private func generatePhonemeSequence(targetDuration: Double) -> [String] {
        var sequence: [String] = []
        var currentTime = 0.0
        let consonants = ["m", "b", "g", "w", "n", "d"]
        let vowels = ["a", "e", "i", "o", "u"]
        
        while currentTime < targetDuration {
            // 70% start with consonant
            if Bool.random(probability: 0.7) {
                sequence.append(consonants.randomElement()!)
                currentTime += 0.08 // consonant duration estimate
            }
            
            // Add vowel
            sequence.append(vowels.randomElement()!)
            currentTime += 0.12
            
            // Occasional silence
            if Bool.random(probability: 0.05) {
                sequence.append("sil")
                currentTime += 0.05
            }
        }
        
        return sequence
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
    
    private func deriveMouthKeyframes(
        _ phonemes: [String],
        _ times: [TimeInterval]
    ) -> [MouthKeyframe] {
        var keyframes: [MouthKeyframe] = []
        
        for (i, phoneme) in phonemes.enumerated() {
            let startTime = times[i]
            let height = mouthHeightForPhoneme(phoneme)
            
            // Mouth opens 50ms before
            keyframes.append(MouthKeyframe(time: startTime - 0.05, height: 0.0))
            keyframes.append(MouthKeyframe(time: startTime, height: height))
            
            // Mouth closes 50ms after
            let endTime = startTime + (times[safe: i+1] ?? times[i]) - startTime
            keyframes.append(MouthKeyframe(time: endTime + 0.05, height: 0.0))
        }
        
        return keyframes
    }
    
    private func mouthHeightForPhoneme(_ phoneme: String) -> Float {
        switch phoneme {
        case "a", "e", "i", "o", "u":
            return Float.random(in: 0.5...1.0) // vowels → wide open
        case "m", "b", "p":
            return 0.0 // closed
        case "ʃ", "j":
            return 0.3 // partially open
        default:
            return 0.2
        }
    }
}

struct MouthKeyframe {
    let time: TimeInterval
    let height: Float // 0.0 = closed, 1.0 = wide open
}
```

**Rive Integration:**
```
// In Coachy.riv state machine:
input mouth_open: number (0.0 to 1.0)

// Blend shape node animates based on mouth_open value
```

**Rust-side exposure (FFI):**
```rust
#[uniffi::export]
pub fn generate_simlish_audio(
    text: String,
    emotion: &str, // "happy", "neutral", etc.
) -> SimlishAudioDto {
    let gen = SimlishGenerator::new();
    gen.generate(&text, emotion)
}

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct SimlishAudioDto {
    pub audio_samples: Vec<f32>,
    pub sample_rate: u32,
    pub mouth_keyframes: Vec<MouthKeyframeDto>,
}

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct MouthKeyframeDto {
    pub time_ms: u64,
    pub height: f32,
}
```

---

## 3. Tier 0 Shipping (Week 1): Simlish + AVSpeechSynthesizer Fallback

### MVP Feature Set

1. **Simlish as primary:** All speech outputs (bubble presents, rule fires, ritual completions) trigger Simlish audio + mouth-shape animation.
2. **Accessibility fallback:** Toggle in Settings: `Coachy voice mode` → `"Simlish"` (default) or `"AVSpeechSynthesizer"` (accessibility).
   - If AVSpeechSynthesizer selected, system reads bubble text using system voice (English US / UK / French / Spanish, user selects).
   - No lip-sync for AVSpeechSynthesizer (visual limit of system voices is low expressivity).
3. **Sound effects:** Pair Simlish with a soft mouth-click consonant sound from sound design library (see §4).
4. **No mouth animation during Simlish speech:** Rive mouth-shape blending can be deferred to Tier 1 if Rive state machine isn't ready.

### Settings UI (SettingsView update)

```swift
Section("Coachy") {
    Picker("Voice mode", selection: $coachyVoiceMode) {
        Text("Simlish (default)").tag(CoachyVoiceMode.simlish)
        Text("Text-to-speech").tag(CoachyVoiceMode.avSpeechSynthesizer)
        Text("Silent").tag(CoachyVoiceMode.silent)
    }
    
    if coachyVoiceMode == .avSpeechSynthesizer {
        Picker("Voice", selection: $selectedVoice) {
            Text("English (US)").tag("en-US")
            Text("English (UK)").tag("en-GB")
            Text("French").tag("fr-FR")
            Text("Spanish").tag("es-ES")
        }
        
        Slider("Speaking rate", value: $speechRate, in: 0.3...2.0)
        Slider("Pitch", value: $pitchMultiplier, in: 0.5...2.0)
        
        Button("Test voice") {
            testVoice()
        }
    }
}
```

### Privacy Manifest (PrivacyInfo.xcprivacy)

**For Tier 0 (Simlish + optional AVSpeechSynthesizer):**

```xml
<key>NSPrivacyTracking</key>
<false/>

<!-- AVSpeechSynthesizer requires NO disclosure (on-device only) -->
<!-- Simlish requires NO disclosure (procedural, on-device only) -->

<!-- Minimal manifest: no required reason APIs, no data collection -->
```

**File location:** `apps/ios/FocalPoint/PrivacyInfo.xcprivacy`

---

## 4. Tier 1 Shipping (Week 3): AVSpeechSynthesizer + Rive Mouth-Sync

### Enhancement

1. **Rive mouth-shape blending:** Integrate mouth_open keyframes into Coachy.riv.
   - When Simlish plays, animate `mouth_open` blend shape 0.0 → 1.0 → 0.0 per phoneme.
   - Provides visual speech sync even without commercial voice.

2. **AVSpeechSynthesizer parity:** If user selects system voice:
   - Still play Simlish audio in background (muted, only for timing).
   - Animate mouth shapes to Simlish keyframes (not synced to actual speech, but consistent).
   - AVSpeechSynthesizer audio plays over it.

3. **No new API costs or privacy implications.**

---

## 5. Tier 2 Shipping (Month 2): ElevenLabs Premium Voice

### Integration Architecture

1. **User opt-in flow:**
   - Settings: "Unlock Coachy's voice" (Premium feature badge).
   - On unlock: ask for consent to send audio snippets to ElevenLabs for voice cloning (or use pre-cloned voice).
   - Option A: User records Coachy voice (record own voice, send ~30s to ElevenLabs for cloning).
   - Option B: FocalPoint-curated Coachy voice (pre-cloned, built-in to premium tier).

2. **Streaming TTS at bubble-present time:**
   ```swift
   // When bubble presents:
   if coachyVoiceMode == .elevenLabs {
       let voiceId = userProfile.coachyVoiceId // cloned voice ID from ElevenLabs
       let bubbleText = "Ready for your first task?"
       
       // Stream TTS from ElevenLabs API
       elevenLabsClient.synthesize(
           text: bubbleText,
           voiceId: voiceId,
           onAudioChunk: { chunk in
               audioPlayer.enqueue(chunk)
           }
       )
   }
   ```

3. **Lip-sync:** Use viseme-to-mouth-shape mapping (see §6 below).

4. **Privacy Manifest update:**
   ```xml
   <key>NSPrivacyTracking</key>
   <false/>
   
   <key>NSPrivacyTrackingDomains</key>
   <array>
       <!-- ElevenLabs receives bubble text for synthesis -->
   </array>
   
   <key>NSPrivacyOptional</key>
   <false/>
   
   <!-- Required reason API if accessing Keychain for ElevenLabs API key -->
   <key>NSPrivacyTracking</key>
   <false/>
   ```
   (Full disclosure follows Apple's template; FocalPoint legal review required.)

### Rust-Side ElevenLabs Bridge

```rust
#[uniffi::export]
pub async fn synthesize_with_elevenlabs(
    text: String,
    voice_id: String,
    api_key: String,
) -> ElevenLabsAudioDto {
    let client = ElevenLabsClient::new(&api_key);
    let response = client.synthesize(&text, &voice_id).await?;
    
    // Extract audio and visemes
    ElevenLabsAudioDto {
        audio_samples: response.audio,
        sample_rate: response.sample_rate,
        visemes: response.visemes, // ElevenLabs provides viseme timings
        duration_ms: response.duration_ms,
    }
}

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct ElevenLabsAudioDto {
    pub audio_samples: Vec<f32>,
    pub sample_rate: u32,
    pub visemes: Vec<VisemeDto>,
    pub duration_ms: u64,
}

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct VisemeDto {
    pub time_ms: u64,
    pub viseme: String, // e.g., "A", "B", "C" (ARKit blend-shape codes)
}
```

---

## 6. Lip-Sync & Viseme Mapping

### Viseme Standard (ARKit Blend Shapes)

Apple's ARKit defines ~52 blend-shape weights for facial expression. For a 2D stylized mascot, we collapse to 8 key visemes:

| Viseme | ARKit Blend Shape | Coachy Mouth Height | Examples |
|--------|-------------------|-------------------|----------|
| **A** | jawOpen | 1.0 | /a/, /ɑ/, /ə/ (open vowels) |
| **B** | mouthClose | 0.0 | /m/, /b/, /p/ (closed, bilabials) |
| **C** | mouthOpen | 0.7 | /ɪ/, /ɛ/, /e/ (mid vowels) |
| **D** | mouthWide | 0.9 | /i/, /aɪ/, /eɪ/ (wide vowels) |
| **E** | mouthFunnel | 0.3 | /ʃ/, /ʒ/, /tʃ/ (fricatives, rounded) |
| **F** | tongueOut | 0.2 | /θ/, /ð/ (dental, tongue slightly forward) |
| **G** | jawLeft / jawRight | — | Lateral consonants /l/ (jaw shifts, not mouth height) |
| **H** | neutral | 0.1 | /n/, /ŋ/ (nasal, minimal mouth opening) |

### Mapping Sources

1. **Simlish (Tier 0):** Use phoneme-to-mouth-height mapping (see §2).
2. **ElevenLabs (Tier 2):** Receive viseme codes from ElevenLabs API; map to mouth_open Rive parameter:
   ```swift
   func mapVisemeToMouthHeight(_ viseme: String) -> Float {
       switch viseme {
       case "A": return 1.0
       case "B": return 0.0
       case "C": return 0.7
       case "D": return 0.9
       case "E": return 0.3
       case "F": return 0.2
       case "H": return 0.1
       default: return 0.1
       }
   }
   ```

3. **Animation:** As audio plays, interpolate mouth_open from current keyframe viseme to next, using easing:
   ```swift
   // At time T, between viseme[i] and viseme[i+1]:
   let t = (currentTime - viseme[i].time) / (viseme[i+1].time - viseme[i].time)
   let mouthHeight = lerp(
       viseme[i].mouthHeight,
       viseme[i+1].mouthHeight,
       t: min(1.0, easeInOutCubic(t))
   )
   riveViewModel.setInputValue("mouth_open", mouthHeight)
   ```

---

## 7. Tier 3 (Long-Term): Adaptive AI Voice

### Vision

Generative voice that adapts to:
- User's mood (from Ritual mood check-ins or Wallet state).
- Time of day (morning energetic, evening calming).
- Current pose/emotion (celebratory voice tone for `.celebratory`, stern for `.sternToughLove`).
- User preference (age, accent, gender perception).

### Approach

1. **Voice parameterization:** Use OpenAI TTS with voice control (pitch, speaking rate, tone hints) or ElevenLabs emotional voice synthesis (if released).
2. **Mood → voice tone mapping:**
   - Happy mood + morning → upbeat, faster rate (1.3×).
   - Stressed mood + evening → calm, slower rate (0.8×), lower pitch.
3. **Pose modulation:** `.sternToughLove` → add warning tone (lower pitch, slight growl in Rive animation).

### Defer to post-launch (Month 3+).

---

## 8. CoachyVoice Protocol & Config

### Swift Protocol

```swift
protocol CoachyVoiceProvider {
    /// Synthesize Coachy's speech from bubble text.
    func synthesize(
        text: String,
        emotion: CoachyEmotion,
        completion: @escaping (Result<CoachyAudio, Error>) -> Void
    )
}

struct CoachyAudio {
    let audioBuffer: AVAudioPCMBuffer
    let visemes: [VisemeKeyframe]? // optional; nil for Simlish
    let duration: TimeInterval
}

struct VisemeKeyframe {
    let time: TimeInterval
    let viseme: String // "A", "B", "C", etc.
}
```

### Rust-Side Config DTO

```rust
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Enum)]
pub enum CoachyVoiceConfig {
    Simlish,
    AvSpeechSynthesizer {
        voice_code: String,       // "en-US", "en-GB", etc.
        speaking_rate: f32,       // 0.3 to 2.0
        pitch_multiplier: f32,    // 0.5 to 2.0
    },
    ElevenLabs {
        api_key: String,
        voice_id: String,         // cloned voice ID or preset
    },
    Silent,
}

#[uniffi::export]
pub fn get_voice_config() -> CoachyVoiceConfig {
    // Fetch from settings store
}

#[uniffi::export]
pub fn set_voice_config(config: CoachyVoiceConfig) {
    // Persist to settings store
}
```

### App Integration

```swift
@EnvironmentObject var voiceConfig: CoachyVoiceConfig

// When bubble presents:
let provider: CoachyVoiceProvider = switch voiceConfig {
case .simlish:
    SimlishVoiceProvider()
case .avSpeechSynthesizer(let voiceCode, let rate, let pitch):
    AvSpeechVoiceProvider(voiceCode: voiceCode, rate: rate, pitch: pitch)
case .elevenLabs(let apiKey, let voiceId):
    ElevenLabsVoiceProvider(apiKey: apiKey, voiceId: voiceId)
case .silent:
    SilentVoiceProvider()
}

provider.synthesize(text: bubbleText, emotion: currentEmotion) { result in
    switch result {
    case .success(let audio):
        audioPlayer.play(audio.audioBuffer)
        if let visemes = audio.visemes {
            animateVisemes(visemes)
        }
    case .failure(let error):
        // Fallback to text-only bubble
        print("Voice synthesis failed: \(error)")
    }
}
```

---

## 9. App Store Privacy Disclosures

### Tier 0: Simlish Only

**PrivacyInfo.xcprivacy:**
```xml
<!-- No required reason APIs used. -->
<!-- No external data transmission. -->
<!-- No disclosure needed. -->
```

**App Store descriptio section (Data & Privacy):** "Coachy's voice is synthesized on-device with no data collection."

---

### Tier 1: AVSpeechSynthesizer Only

**PrivacyInfo.xcprivacy:**
```xml
<!-- AVSpeechSynthesizer is on-device; no disclosure required. -->
```

**App Store:** Same as Tier 0.

---

### Tier 2: ElevenLabs Optional

**PrivacyInfo.xcprivacy:**
```xml
<key>NSPrivacyTracking</key>
<false/>

<!-- If ElevenLabs enabled: -->
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

**App Store Data & Privacy section:**
```
ElevenLabs voice synthesis (Premium):
- Data collected: Text of bubbles spoken (user-facing copy).
- Data not collected: Audio recordings, user identity.
- Data shared with: ElevenLabs (if Premium voice enabled).
- Retention: ElevenLabs retains audio for up to 30 days per their policy.
- User can opt-out: Settings → Coachy → Voice mode → Simlish or silent.
```

---

## 10. Implementation Timeline

| Phase | Week | Component | Effort | Status |
|-------|------|-----------|--------|--------|
| **Tier 0** | 1 | Simlish audio gen (procedural) | 8 h | Implementer |
| | 1 | SimlishVoiceProvider + FFI | 4 h | Implementer |
| | 1 | Settings UI (mode toggle) | 2 h | Implementer |
| | 1 | PrivacyInfo.xcprivacy | 0.5 h | Legal review |
| **Tier 1** | 3 | AvSpeechVoiceProvider | 3 h | Implementer |
| | 3 | Rive mouth-shape blending | 2 h | Implementer + designer (Rive asset) |
| | 3 | Viseme keyframe pipeline | 2 h | Implementer |
| **Tier 2** | 8 | ElevenLabsVoiceProvider + API | 6 h | Implementer |
| | 8 | Voice cloning onboarding flow | 3 h | Implementer + designer (UI) |
| | 8 | Privacy manifest update | 1 h | Legal review |
| **Tier 3** | 12+ | Adaptive voice synthesis | TBD | Deferred |

---

## 11. Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| **Simlish phoneme quality** (sounds robotic) | Use short, expressive phoneme samples; add subtle reverb/compression in post-production. |
| **Mouth-shape sync lag** (visemes behind audio) | Pre-load visemes before audio plays; use time-compensated animation curves. |
| **ElevenLabs API limits** (rate limiting, cost overruns) | Cache synthesized bubbles per emotion/text (memoization). Implement queue + backoff logic. |
| **Privacy disclosure complexity** | Involve legal review early; use Apple's template for "NSPrivacyAccessedAPIType" declarations. |
| **User trust (creepy voice)** | Offer "Silent mode" prominently; emphasize opt-in for cloud TTS. Default to Simlish (friendly, non-invasive). |

---

## References

- [AVSpeechSynthesizer Documentation — Apple Developer](https://developer.apple.com/documentation/avfoundation/speech-synthesis), accessed 2026-04-23.
- [Extend Speech Synthesis with personal and custom voices — WWDC 2023](https://developer.apple.com/videos/play/wwdc2023/10033/), accessed 2026-04-23.
- [Using your Personal Voice in iOS — Ben Dodson](https://bendodson.com/weblog/2024/04/03/using-your-personal-voice-in-an-ios-app/), accessed 2026-04-23.
- [ElevenLabs API Pricing](https://elevenlabs.io/pricing/api), accessed 2026-04-23.
- [ElevenLabs Pricing Breakdown 2026 — Flexprice](https://flexprice.io/blog/elevenlabs-pricing-breakdown), accessed 2026-04-23.
- [Privacy Manifest for iOS Apps — Capgo](https://capgo.app/blog/privacy-manifest-for-ios-apps/), accessed 2026-04-23.
- [Adding a privacy manifest to your app or third-party SDK — Apple Developer](https://developer.apple.com/documentation/bundleresources/adding-a-privacy-manifest-to-your-app-or-third-party-sdk), accessed 2026-04-23.
