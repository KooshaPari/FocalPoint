# OnboardingViewV2: Mascot-First Duolingo-Grade Rework

## Overview

OnboardingViewV2 transforms the onboarding experience from utilitarian form-based flows into a character-led, emotionally resonant journey. Every page centers on Coachy as the dominant hero (280–320 pt), with matched-geometry transitions preserving his identity across steps. Each beat triggers micro-animations, sound effects, and haptics to make onboarding feel like a conversation with a friend, not a configuration wizard.

## Design Philosophy

**Mascot-First**: Coachy is the primary visual focus, not a sidebar decoration. He occupies 60–70% of screen real estate and actively "reacts" to user choices (poses shift, sparkles appear, scale animates).

**Coherent Motion**: `matchedGeometryEffect(id: "coachy.onboarding", in: namespace)` ensures Coachy maintains visual continuity as pages transition — he doesn't cross-fade; he "walks" across the screen.

**Micro-Interactions**: Every action (selection, confirmation, skip) triggers:
- Sound effect (page turn, select tick, success chime)
- Haptic pulse (light tap, medium tap, celebrate)
- Voice cue (Simlich greeting or encouragement)
- Particle effect (sparkles on goal selection, confetti on final step)

## Page-by-Page Breakdown

### 1. Welcome Page (`OnboardingWelcomePageV2`)
- **Pose**: `happy`
- **Emotion**: `warm`
- **Bubble**: "Hi! I'm Coachy, your focus coach."
- **Entry Animation**: Scale in from 0.8 → 1.0 (600ms easeOut)
- **SFX**: None (entry silence; voice only)
- **Haptics**: None (silent welcome)
- **Copy**: Headline "Focus Coach" + body text "Let's set up your focus rules and goals together."
- **CTA**: Primary "Next" button

**Intent**: Establish Coachy as a friendly, non-threatening guide. No overwhelming info; just introduction and simple call to action.

### 2. Goals Page (`OnboardingGoalsPageV2`)
- **Pose**: `curious`
- **Emotion**: `engaged`
- **Bubble**: "What are your focus goals?"
- **Entry Animation**: Matched geometry (smooth transition from Welcome)
- **SFX**: `.select` on each goal tap; voice "Goal name? Great choice!"
- **Haptics**: `.lightTap` on selection
- **Particles**: Sparkles (count: 12) overlay on selected goal card for 600ms
- **Copy**: 4 goal cards — Sleep, Fitness, Study, Work-Life Balance
- **CTA**: "Next" enabled only if ≥1 goal selected

**Intent**: Gamify the selection process. Multiple sparkles + voice confirm user's choice, making each selection feel like a mini-win.

### 3. Connect Page (`OnboardingConnectPageV2`)
- **Pose**: `encouraging`
- **Emotion**: `warm`
- **Bubble**: "Let's connect a calendar."
- **Entry Animation**: Matched geometry + scale animation when button is tapped
- **SFX**: `.success` on connection; voice "Calendar connected!"
- **Haptics**: `.celebrate` on successful connection
- **Particles**: None (connection is textual confirmation)
- **Copy**: Headline "Connect Your Calendar" + description "I'll sync your events to help manage your time."
- **CTA**: "Connect Google Calendar" button (disables after success, shows checkmark)

**Intent**: Build trust by showing integration in action. Coachy "leans forward" (scale 1.0 → 1.05) to celebrate the connection.

### 4. Template Page (`OnboardingTemplatePageV2`)
- **Pose**: `curiousThinking`
- **Emotion**: `neutral`
- **Bubble**: "Pick your focus style."
- **Entry Animation**: Matched geometry
- **SFX**: `.select` on template tap
- **Haptics**: `.lightTap`
- **Particles**: None
- **Copy**: 3 template cards — Pomodoro (25/5), Deep Study (90min), Balanced
- **CTA**: "Next" enabled once a template is selected

**Intent**: Present choices in a non-judgmental way. Neutral pose signals "no wrong choice."

### 5. Permissions Page (`OnboardingPermissionsPageV2`)
- **Pose**: `sternToughLove`
- **Emotion**: `serious`
- **Bubble**: "One more thing — I need your trust."
- **Entry Animation**: Matched geometry
- **SFX**: None (serious moment)
- **Haptics**: None
- **Particles**: None
- **Copy**: 3 permission items (Family Controls, Screen Time, Keychain)
- **CTA**: "Next" (always enabled; user can skip and grant later)

**Intent**: Shift tone to serious without being scary. Coachy's stern-but-caring expression conveys the importance of permissions without guilt-tripping.

### 6. Final Page (`OnboardingFinalPageV2`)
- **Pose**: `confident`
- **Emotion**: `ecstatic`
- **Bubble**: "You're ready! Let's go!"
- **Entry Animation**: Matched geometry + celebratory scale (1.0 → 1.1)
- **SFX**: `.fanfare`
- **Haptics**: `.celebrate`
- **Particles**: Confetti (80 pieces, rainbow colors, full-screen overlay)
- **Copy**: "Welcome to FocalPoint" + "You're all set. Let's build great focus habits together!"
- **CTA**: "Finish" (advances to main app)

**Intent**: Climactic celebration. Confetti + sound + haptics + voice create a shared "high-five" moment.

## Pose & Emotion Matrix

| Page | Pose | Emotion | Intent |
|------|------|---------|--------|
| Welcome | `happy` | `warm` | Approachable, friendly |
| Goals | `curious` | `engaged` | Interested in your choices |
| Connect | `encouraging` | `warm` | Supportive, celebratory |
| Template | `curiousThinking` | `neutral` | Non-judgmental |
| Permissions | `sternToughLove` | `serious` | Trustworthy, firm |
| Final | `confident` | `ecstatic` | Celebratory, ready |

## Transition & Motion

**Matched-Geometry**:
```swift
CoachyView(...)
    .matchedGeometryEffect(id: "coachy.onboarding", in: namespace)
```
All pages share the same `id` and `namespace`, creating a seamless "Coachy walks across pages" effect rather than cross-fades.

**Page-Level Asymmetric Transitions**:
```swift
.transition(.asymmetric(
    insertion: .move(edge: .trailing).combined(with: .opacity),
    removal: .move(edge: .leading).combined(with: .opacity)
))
```
New pages slide in from the right; old pages slide out to the left (forward motion cue).

**Bubble Text Transitions**:
Each bubble text update uses `.transition(.opacity.combined(with: .scale(scale: 0.95)))` so bubbles fade and slightly shrink on exit, then fade and grow on entry.

## Sound Design

**SoundEffectPlayer Integration**:
- `.pageTurn` — advancing to next page (whoosh, 300ms)
- `.select` — selecting a goal or template (tick, 100ms)
- `.success` — calendar connection success (chime, 500ms)
- `.fanfare` — final page completion (orchestral swell, 2s)

**SimlishVoice Integration**:
- "Welcome!" on Welcome page entry
- "[Goal Name]? Great choice!" on each goal selection
- "Calendar connected!" on successful connection
- "You're ready!" on Final page entry

Voice cues use the existing `SimlishVoice.shared.speak(_:)` helper and respect the voiceMode setting (`.silent` skips all speech).

## Haptic Design

**HapticChoreographer Integration**:
- `.lightTap` — goal/template selection (85ms light pulse)
- `.mediumTap` — page advance (120ms medium pulse)
- `.softTap` — back button (50ms soft pulse)
- `.celebrate` — final step + connection success (3-beat pattern: light → medium → heavy, 150ms each)

## Particle Effects

**ParticleOverlay Component**:
Custom Canvas-based particle renderer:
- **Confetti** — 80 random-colored squares falling across full screen
- **Sparkles** — 12 accent-colored dots at random positions

Particles appear:
- **Goal selection**: Sparkles overlay on the selected goal card for 600ms
- **Final page**: Confetti fills entire screen for the duration of the Final page

## Feature-Flag Switch

**AppStorage Toggle**:
```swift
@AppStorage("app.onboardingV2") private var onboardingV2: Bool = true
```

**Behavior**:
- New installs default to V2 (`true`)
- Users can opt back to V1 via Settings → Onboarding (if added)
- Switch is evaluated at app launch in `FocalPointApp.body`

**Path**:
```
if !hasSeenWake { LaunchCoachyView... }
else if hasOnboarded { RootTabView() }
else if onboardingV2 { OnboardingViewV2() }
else { OnboardingView() }
```

## Page Sequencing

Default order (can be re-sequenced by reordering tags in TabView):
1. Welcome
2. Goals
3. Connect
4. Template
5. Permissions
6. Final

**Handling Consent Page (if merged from consent-gate agent)**:
If `OnboardingConsentPage` exists and should be first, insert before Welcome:
```swift
OnboardingConsentPage(coord: coord)
    .tag(OnboardingCoordinator.Step.consent)
// ... then Welcome, Goals, etc.
```

Current scaffold assumes consent is handled post-completion; adjust if spec changes.

## Tier-0 Primitives Reused

- **CoachyView** — render Coachy with pose/emotion/bubble
- **CoachyScene** — animation config (not directly used; for future enhancement)
- **SoundEffectPlayer** — SFX playback (`.play(_:variation:)`)
- **SimlishVoice** — voice cues (`.speak(_:)`)
- **HapticChoreographer** — haptic patterns (`.perform(_:)`)
- **ParticleOverlay** — custom canvas renderer for sparkles/confetti

**No Extensions Needed**: All primitives used as-is. Custom `ParticleOverlay` is scoped to OnboardingViewV2; can be extracted to MascotUI if reused elsewhere.

## Testing Strategy

**Snapshot Tests** (one per page):
```swift
func testWelcomePageSnapshot() {
    let view = OnboardingWelcomePageV2(namespace: Namespace().wrappedValue)
    assertSnapshot(matching: view, as: .image(size: .iPhoneXsMax))
}
// ... repeat for Goals, Connect, Template, Permissions, Final
```

**Interaction Test** (goal selection sparkles):
```swift
func testGoalSelectionTriggersSparkles() {
    @State var sparkleId: String?
    let view = OnboardingGoalsPageV2(namespace: ..., sparkleId: $sparkleId)
    // Simulate tap on goal card
    // Assert sparkleId != nil and ParticleOverlay renders
}
```

## Build & Verification

**Build**:
```bash
xcodebuild -scheme FocalPointApp \
  -destination 'platform=macOS,arch=arm64,variant=Designed for iPad' \
  -configuration Debug build
```

**xcodegen**:
```bash
xcodegen generate
```

**Manual Testing**:
1. Delete app from simulator
2. Rebuild and launch
3. Tap through each onboarding page
4. Verify:
   - Coachy maintains visual continuity (matched geometry)
   - Each page's pose/emotion displays correctly
   - Bubble text updates on entry
   - Goal/template selection triggers sparkles + haptics + SFX
   - Final page shows confetti + fanfare
   - "Finish" → main app (hasOnboarded = true)

## Future Enhancements

- **Animated Coachy Poses**: Replace pose-change opacity transition with 3D rotation or morphing animation
- **Gesture Integration**: Swipe gestures to advance pages (currently TabView handles this)
- **Localization**: Bubble text, copy, and voice cues in multiple languages
- **A/B Testing**: Record which goals/templates are selected; use data to personalize follow-up nudges
- **Replay**: Add "Replay Onboarding" button in Settings to re-watch the sequence
- **Analytics**: Track time spent on each page, drop-off points, repeat completions

