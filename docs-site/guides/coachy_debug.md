# Coachy Debug & Preview Guide

## Overview

**CoachyDebugView** is an interactive harness for designers and developers to preview and QA Coachy's visual states, audio modes, haptic feedback, and animation effects without running the full onboarding or focus flows.

Access it in-app:
1. Settings → tap version number 5 times to unlock Developer mode
2. Settings → Developer → "Coachy character sheet"

## Features

### 1. Live Preview
- See Coachy render in real-time as you adjust pose and emotion
- Pose + emotion + size all update the preview instantly
- Optional matched-geometry fly-in animation demo (tap to trigger)

### 2. Pose & Emotion Controls

#### Poses (7 variants)
- **Idle**: Default resting state
- **Confident**: Chest forward, assertive stance
- **Encouraging**: Open arms, welcoming posture
- **Curious**: Head tilt, inquisitive expression
- **Stern**: Serious, focused demeanor
- **Celebratory**: Victory pose, arms raised
- **Sleepy**: Drowsy, eyes closed, head drooping

#### Emotions (8 variants)
- **Neutral**: Flat affect
- **Happy**: Smile, upturned eyes
- **Excited**: Wide eyes, open-mouthed smile
- **Proud**: Confident expression, raised chin
- **Concerned**: Furrowed brow, worried eyes
- **Disappointed**: Downturned mouth, sad eyes
- **Tired**: Heavy eyelids, subtle frown
- **Focused**: Intense gaze, concentration wrinkles

### 3. Size Slider
Adjust Coachy's rendered size from **100pt to 360pt** to test layout integration and responsive scaling.

### 4. Animation Demo
- **Toggle**: "Matched-geometry fly-in demo"
- **Trigger**: Tap the live preview to animate Coachy flying in from the left with opacity fade
- Use this to verify entrance animations in context

### 5. Audio Section
- **Voice Mode Picker**: Switch between:
  - **Simlish**: Procedural phoneme sequencing (unique, memorable)
  - **AVSpeechSynthesizer**: Native iOS speech synthesis (accessible)
  - **Silent**: No voice output (for silent testing)
- **Volume Slider**: 0–100% control
- **Latency Readout**: Displays voice synthesis latency (in seconds)
- **Play Button**: Triggers Simlish greeting and measures latency

### 6. Haptics Section
**8 haptic cues** for immediate feedback testing:
- Light, Medium, Heavy (impact patterns)
- Celebrate (medium + light + light sequence)
- Warning (notification pattern)
- Triple tap (3× light impacts)
- Success, Error (notification patterns)

Each button triggers its corresponding haptic immediately.

### 7. Sound Effects Section
**6 sound cues** for audio feedback integration:
- Success, Encourage, Celebrate, Concerned, Ding, Whoosh

Each button plays the sound effect (gracefully skips if audio files are missing).

### 8. Snapshot Export
- **"Export current frame as PNG"**: Captures the current pose + emotion + size and saves to Photos
- Use for:
  - Designer review sessions (share via Slack/email)
  - Visual regression testing
  - Onboarding mockups

---

## Designer Workflow

### QA Checklist
1. **Unlock developer mode** (5× version tap in Settings)
2. **Open Coachy Preview** (Settings → Developer → "Coachy character sheet")
3. **Test each pose** with your preferred emotion:
   - Iterate through all 7 pose variants
   - For each, verify eyes/mouth/arms render correctly
   - Check alignment in different sizes (100pt, 200pt, 360pt)
4. **Test audio + haptics together**:
   - Switch voice modes and play greeting
   - Trigger a haptic cue, then a sound effect
   - Verify no conflicts or timing issues
5. **Test animation** (if using matched-geometry):
   - Enable fly-in demo
   - Tap preview several times
   - Verify smooth entrance and reset
6. **Export & review**:
   - Capture 3–5 representative frames (different poses/emotions)
   - Export to Photos
   - Share snapshots with team for visual alignment

### Tips
- **Isolated testing**: Use CoachyDebugView to test character in isolation, not in full app flow
- **Voice synthesis latency**: Watch the latency readout to catch unexpected delays
- **Haptics on device**: Haptics only work on physical iPhone/iPad; Simulator will log but not vibrate
- **Audio files**: Sound FX gracefully skip if audio files are missing (check Console for warnings)

---

## Developer Notes

### Implementation Details
- **Location**: `Sources/MascotUI/CoachyDebugView.swift`
- **Dependencies**:
  - `CoachyView` (pose + emotion rendering)
  - `CoachyState` (7 poses, 8 emotions)
  - `SimlishVoiceProvider` (procedural phoneme synthesis)
  - `HapticChoreographer` (8 named haptic patterns)
  - `SoundEffectPlayer` (graceful audio fallback)
- **No core logic changes**: This is a pure consumer of existing APIs

### Extension Points
- Add new pose/emotion variants to `CoachyState.swift` and they auto-appear in pickers
- Add haptic cues to `HapticChoreographer` and add buttons to the Haptics section
- Add sound effects to `SoundEffectPlayer` and list them in the Sound FX section
- Snapshot export: Currently a placeholder; integrate `UIGraphicsPDFRenderer` or SwiftUI snapshot API for full implementation

### Testing
```bash
# Run unit tests (iOS target)
xcodebuild test -scheme FocalPoint -destination 'platform=iOS Simulator,name=iPhone 15'

# Trace CoachyDebugView in-app
Settings → Developer → Coachy character sheet
```

### Known Limitations
- **Snapshot export**: Currently stub; full implementation requires PDF renderer integration
- **Haptics**: Simulator logs but does not vibrate; test on device
- **Audio**: Gracefully skips if .m4a files are missing; check Console for file paths

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Developer mode locked | Tap version in Settings 5 times |
| Haptics don't vibrate | Test on physical device (Simulator doesn't support haptics) |
| Voice is silent | Check voice mode is not "Silent"; ensure audio system not muted |
| Sound FX not playing | Verify .m4a files in `Resources/Audio/SFX/` and check Console logs |
| Snapshot not exported | Feature placeholder; requires PDF renderer integration |
| Preview stutters | Reduce size slider or disable matched-geometry fly-in |

---

## Related Files
- **Poses & Emotions**: `Sources/MascotUI/CoachyState.swift`
- **Character Rendering**: `Sources/MascotUI/CoachyView.swift`
- **Voice Synthesis**: `Sources/MascotUI/Voice/SimlishVoice.swift`
- **Haptics**: `Sources/MascotUI/Haptics/HapticChoreographer.swift`
- **Audio**: `Sources/MascotUI/Audio/SoundEffectPlayer.swift`
- **Settings Integration**: `Sources/FocalPointApp/Settings/SettingsView.swift` (line 292–294)
