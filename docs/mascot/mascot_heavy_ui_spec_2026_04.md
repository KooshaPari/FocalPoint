# Mascot-Heavy UI Specification — FocalPoint iOS

**User Brief (Verbatim):** "very very Duolingo-like in usage, face as screen loading screens, sudden fly-ins and lots of emotion, + ability to talk whether in sims like lang or real lang, using AI gen and other means."

**Current State:** Coachy appears at 80–220pt sizes in every tab with static bubble text. No face-as-screen loading states. No sound, talking, fly-ins, or haptic choreography. ~2/10 vs Duolingo target.

**Target:** Coachy is the primary narrative and emotional carrier for FocalPoint. Every async boundary, state change, and achievement becomes a face-moment. Duolingo's owl (Duo) sets the UX north star.

---

## 1. Face-as-Loading-Screen Pattern

### Duolingo Reference
Duolingo replaces generic spinners with full-screen character states ([DEV Community: Bringing Mascots to Life](https://dev.to/uianimation/bringing-mascots-to-life-duolingo-style-character-animation-in-rive-5d19), accessed 2026-04-23). The owl *becomes* the loading boundary, reacting emotionally to app state. This signals app aliveness and prevents abandonment during wait times.

### FocalPoint Async Boundaries & Coachy Responses

| Boundary | Trigger | Current UX | Coachy Replacement |
|----------|---------|-----------|-------------------|
| **App cold launch** | App enters `willFinishLaunching` | Black/splash screen (1–3s) | Full-screen Coachy `.sleepy` → `.encouraging` wake sequence (3.5s). See §7 of `coachy-art-direction.md`. Bubble: "Morning! Ready?" |
| **Onboarding step advance** | User taps "Next" between signup/auth screens | ProgressView spinner (0.5–1.5s) | Coachy `.workMode` with focused brow, subtle scale-pulse. Bubble: "One sec…" Eyes briefly close at step 3/5 (excited) and open at step 5/5 (confident). |
| **OAuth callback spinner** | Exchange code for token; FocalPoint core blocks on `AuthApi::exchange_code` | Network spinner (2–5s) | Coachy `.confident` with a checkmark icon that appears over the chest, scales 1.2× then settles. Mouth animates mouth-open to closed (thinking pose). No bubble; sound-cue `auth-sparkle`. |
| **Rule eval tick** | `RuleStore` evaluates active rules on intent capture or ritual completion | None currently (silent, ~100ms) | **Micro-reaction only**: Coachy `.idle` → brief `.celebratory` blink (0.2s) if rule fires, or `.sternToughLove` micro-frown (0.1s) if rule blocks. No bubble; haptic: light tap. |
| **Connector sync** | `Connector` pulls new events from external service (OAuth'd calendar, tracker, etc.) | None currently; runs in background | If user navigates to sync point, show full-screen Coachy `.workMode` with a loading-bar animation under the character (0–100% fill). Bubble: "Pulling your [source]…" Sound: `sync-whoosh`. |
| **Rituals generation** | `RitualsApi` creates new ritual from template or user input (1–3s LLM call) | ProgressView | Full-screen Coachy `.curiousThinking` with animated thought-bubble (3 dots pulsing). Bubble: "Dreaming up rituals…" or "Crafting your flow…" Sound: `think-hum`. |
| **Task creation** | User submits new task; `TaskApi` validates and stores (0.3–1s) | Sheet dismissal is instant, but store is async | Brief full-screen Coachy `.celebratory` pose, 0.5s hold, scale 1.1 then shrink back. Bubble: "Task logged!" Sound: `task-clink`. Haptic: medium success. |
| **Template install** | User selects ritual template; system clones it into personal rituals (0.5–1.5s) | None; happens silently on sheet submit | Full-screen Coachy `.encouraging` with arms-open gesture, scale breathing (1.0 ↔ 1.05). Bubble: "Adding to your rituals…" Sound: `install-pop`. |
| **Audit chain load** | On app start, `AuditStore` verifies tamper-evident SHA-256 chain (10–50ms) | None | Skip visual if <100ms; if >200ms, show Coachy `.sternToughLove` (security check pose) with a shield icon overlay. Bubble: "Verifying integrity…" |
| **Stats reload** | User pulls-to-refresh StatsView or swaps date range (0.5–1.5s) | SwiftUI ProgressView | Coachy `.workMode` appears from top (fly-in from above), 0.3s ease-out. Bubble: "Crunching numbers…" Sound: `stats-whoosh`. Haptic: light tap on dismiss. |

### Implementation Pattern

1. **Wrapper layer:** Create `CoachyLoadingView<Content>` — a generic container that:
   - Takes a `LoadingState` enum: `idle`, `loading(reason: String)`, `loaded`, `error(String)`.
   - When `.loading`, shows full-screen Coachy with specified pose, emotion, bubble, and sound/haptic.
   - When `.loaded` or `.error`, transitions Content in via `.transition(.asymmetric(...))`.
   - Holds Coachy for minimum 0.5s to prevent flash on fast networks.

2. **Rust-side directive:** `CoachyDirector` (in `crates/focus-mascot/src/director.rs`) maps event types to loading states:
   ```rust
   pub enum CoachyLoadingDirective {
       AppColdLaunch,
       OnboardingStep(step: u8),
       OAuthCallback,
       RuleEvalTick { fired: bool },
       ConnectorSync { source: String },
       RitualsGeneration,
       TaskCreation,
       TemplateInstall,
       AuditChainLoad { duration_ms: u64 },
       StatsReload,
   }
   ```
   Each variant maps to a pose, emotion, bubble, sound-cue name, and haptic pattern (via FFI).

3. **Exposure:** Wrap every async boundary:
   - `TasksView`: `.task { reload() }` → `CoachyLoadingView` with `.loading(.tasksReload)`
   - `OnboardingView`: Each step advance → `.loading(.onboardingStep(2))`
   - `AuthView`: OAuth callback → `.loading(.oauthCallback)`

---

## 2. Sudden Fly-Ins & Choreography

### Trigger Events & Poses

| Event | Trigger | Entry | Easing | Hold | Exit | Sound Cue | Haptic |
|-------|---------|-------|--------|------|------|-----------|--------|
| **Rule fires** | `RuleStore` fires; penalty or reward issued | Swing in from **right** (270° → 0°), 0.4s | `easeOut` | 0.8s | Bounce/settle to center, 0.2s | `rule-fire-sparkle` | medium tap × 2 |
| **Credit lands** | Reward added to wallet; `WalletStore` incremented | Bouncing gold-coin trail ×5 from top, each 0.1s offset; Coachy `.celebratory` center | `easeOut` (coins), `spring` (Coachy) | 1.0s | Coins fade down, Coachy shrink to HUD size, 0.3s | `credit-clink` | success pulse |
| **Streak extends** | Daily ritual completed; streak counter increments | Coachy `.thumbsUp` flies in from **left**, 0.3s; flame-colored confetti bursts | `easeOut` | 1.2s | Coachy shrink and recenter to HUD; confetti fade | `streak-success` | heavy success ×3 |
| **Penalty escalates** | Penalty tier increases (e.g., lockdown imminent) | Coachy `.sternToughLove` drops in from **top**, 0.4s; red flash behind | `easeIn` (drop only), then hold | 1.5s | Shrink down, fade red behind | `penalty-warning` | warning ×2 + haptic decay |
| **Lockdown engages** | Penalty reaches MAX or user opts-in; screen lock active | Coachy `.lockdown` (padlock front) plunges from **top**, 0.5s; padlock spins 180° mid-fall | `easeIn` | 2.0s (hold longer) | Settle center; button only shows "I understand" | `lockdown-thunk` + low rumble | heavy + decay pattern |
| **Session complete** | Focus session ends cleanly (timer → 0); `RitualsApi` marks done | **Full-screen takeover**: Coachy `.achievement` with trophy, 1.0s grow from center (0.5 → 1.5 scale). Confetti burst. | `easeOut` | 2.0s (full-screen) | Scale back to 220pt HUD, reposition to corner of HomeView, 0.5s | `session-complete-fanfare` | triple success (heavy, light, heavy) |

### Choreography Specs (Implementer API)

Each fly-in becomes a call to `CoachyScene::fly_in()`:

```swift
// Pseudocode for iOS view
CoachyFlyIn(
    pose: .celebratory,
    emotion: .excited,
    entry: .fromRight(duration: 0.4, easing: .easeOut),
    hold: 0.8,
    exit: .shrinkToHUD(duration: 0.3),
    soundCue: "rule-fire-sparkle",
    hapticPattern: .tapTwice(.medium),
    accessories: [.none], // or .trophy, .padlock, etc.
    particles: .none // or .confetti, .coinTrail, etc.
)
```

**Particle systems** (delivered by designer as Lottie `.json` or Rive layers):
- **Confetti burst:** 12–15 pieces, 0.3s spawn, 2.0s fade-out, randomized rotation/drift.
- **Coin trail:** 5 coins, 0.1s offset, 0.4s fall, 0.6s scale-bounce at bottom.
- **Red flash background:** semi-transparent red (0.3 alpha), 0.2s fade-in, 1.0s hold, 0.5s fade-out.

---

## 3. Emotion Through UI State

### Tab-Level Emotional Rendering

Each tab renders Coachy as a reactive status indicator, not frozen. Coachy lives in a small HUD position (120–160pt) and morphs based on current state.

#### Today Tab (TasksView / RitualsView)

| State | Pose | Emotion | Bubble | Rationale |
|-------|------|---------|--------|-----------|
| **Intention captured** (≥1 ritual selected for today) | `.encouraging` | `.happy` | Optional: "You've got this" or none | Warm glow behind Coachy; breathing scale 1.0 ↔ 1.05. |
| **No rituals planned** | `.curiousThinking` | `.concerned` | "What's today about?" or none | Coachy leans left, eyebrow raised; no glow. |
| **All rituals completed** | `.celebratory` | `.excited` | "Amazing day!" or none | Warm glow + slow confetti fall, 1–2 pieces every 0.3s. |
| **One ritual in progress** (timer running) | `.focusMode` | `.focused` | None | Pulse scale 0.98 ↔ 1.02 in sync with ambient timer. Headphones accessory. |

#### Focus Mode (FocusModeView)

| State | Pose | Emotion | Bubble | Accessory | Animation |
|-------|------|---------|--------|-----------|-----------|
| **Session starting** | `.workMode` | `.focused` | "Let's go" (2s fade) | Headphones | Scale 1.0, opacity fade-in from 0.8. |
| **Session active (breathing)** | `.focusMode` | `.focused` | None | Headphones | Continuous scale-breathe: 0.98 ↔ 1.02 (0.5 Hz), ~4s cycle. |
| **User tries to exit early** | `.sternToughLove` | `.concerned` | "Stick with it" (pulse) | Headphones + frown | Eyebrows lower; face tilt left/right ±5° at 1 Hz (disapproval shimmy). |
| **Final 10s countdown** | `.confident` | `.proud` | "Almost there!" | Headphones | Scale pulse: 1.0 ↔ 1.08 every 1s; Coachy nods. |
| **Session complete** | `.achievement` | `.excited` | "You crushed it!" | Trophy | Fly-in: grow 1.5×, confetti burst. |

#### Rewards / Wallet (WalletView)

| State | Pose | Emotion | Bubble | Rationale |
|-------|------|---------|--------|-----------|
| **Wallet empty** (0 credits) | `.sleepyDisappointed` | `.disappointed` | "Earn some credits?" or none | Head tilt down; eyes half-closed. |
| **Credits available (unused)** | `.idle` | `.neutral` | None | Neutral stance; slight breathing. |
| **Credits earned (animated bump)** | `.celebratory` → `.confident` (0.5s transition) | `.excited` → `.proud` | "Nice!" (fade 2s) | Bounce animation: +0.1 scale on earn, settle 0.3s. |
| **Redeeming reward** | `.workMode` | `.focused` | "Processing…" | Spinning coin/star icon overlay. |
| **Premium rewards locked** | `.curiousThinking` | `.concerned` | "Unlock premium" or none | Gaze fixed on locked icon; slight head tilt. |

#### Audit / Activity (ActivityView)

| State | Pose | Emotion | Bubble | Rationale |
|-------|------|---------|--------|-----------|
| **Recent activity (last 7d)** | `.confident` or `.celebratory` | `.proud` or `.excited` | "Look at you go!" or none | Warm glow; Coachy appears engaged. Glow intensity ∝ activity streak. |
| **No recent activity** | `.sleepyDisappointed` | `.tired` | "Miss you…" or none | Slumped posture; soft glow fades. |
| **Suspicious audit flag** (tamper detected) | `.sternToughLove` | `.concerned` | "Heads up" (pulse red) | Red tint to background; Coachy blocks with arms. |
| **Audit chain verified** | `.confident` | `.proud` | None (appears at load) | Green checkmark overlay; calm breathing. |

#### Settings (SettingsView)

| State | Pose | Emotion | Context |
|-------|------|---------|---------|
| **Coachy voice enabled** | `.encouraging` | `.happy` | Subtitle: "Coachy will talk to you" with sound icon. |
| **Coachy voice disabled** | `.idle` | `.neutral` | Subtitle: "Silent mode" with mute icon. |
| **Theme: Dark** | `.idle` (flame colors shifted) | `.neutral` | Coachy rendered in dark palette. |
| **Theme: Light** | `.idle` (flame colors brightened) | `.neutral` | Coachy rendered in light palette. |

### Implementation: State-Driven Emotion

Create a `CoachyReactionProvider` protocol:

```swift
protocol CoachyReactionProvider {
    func reactionFor(state: AppState) -> CoachyState
}

// Implement per-view:
class TodayCoachyReactionProvider: CoachyReactionProvider {
    func reactionFor(state: AppState) -> CoachyState {
        if state.rituals.allCompleted {
            return CoachyState(pose: .celebratory, emotion: .excited, bubbleText: "Amazing day!")
        } else if state.rituals.isEmpty {
            return CoachyState(pose: .curiousThinking, emotion: .concerned, bubbleText: "What's today about?")
        } else {
            return CoachyState(pose: .encouraging, emotion: .happy, bubbleText: nil)
        }
    }
}
```

Inject into each view via `@EnvironmentObject` or `@State` tied to observed changes. On any state mutation (ritual added, ritual completed, credits earned), the Coachy view automatically re-renders with the new pose/emotion/bubble.

---

## 4. Duolingo-Grade Reference Teardown

### Duolingo's UX Patterns (Applicable to FocalPoint)

**Source:** [How Duolingo Uses Rive for Their Character Animation — DEV Community](https://dev.to/uianimation/how-duolingo-uses-rive-for-their-character-animation-and-how-you-can-build-a-similar-rive-mascot-5d19), accessed 2026-04-23.

1. **Character as Page Hero**
   - Duo (owl) is the primary narrative, not a sidebar decoration. Every lesson screen centers on Duo's emotional reaction to the task.
   - **FocalPoint adaptation:** Coachy is the hero of every key screen (Tasks, Focus, Rituals, Wallet, Activity). Remove small corner placements; expand Coachy to 200–240pt on empty states and 150–180pt on active states.

2. **Character-Driven Progress Indicators**
   - Duolingo shows progress through Duo's expressions (proud, encouraging, playful) rather than bars.
   - **FocalPoint adaptation:** Coachy's pose/emotion *is* the progress state. Ritual completion → Coachy `.confident`; rule pending → Coachy `.curiousThinking`.

3. **Heart System (Lives Metaphor)**
   - Duolingo's "hearts" (lives) are the resource constraint, and losing a heart triggers Duo's disappointed reaction.
   - **FocalPoint adaptation:** Credits act as the "heart" equivalent. Spend a credit → Coachy `.sternToughLove` (tough love on penalty). Earn credit → Coachy `.celebratory` (celebration on reward).

4. **Streak Flame on Everything**
   - Streak visuals appear on every screen, reinforcing continuity.
   - **FocalPoint adaptation:** Daily ritual streak should appear as a flame icon next to Coachy HUD, growing brighter with consecutive days. Coachy's pose shifts to `.celebratory` if streak extends.

5. **Celebration Moments After Every Win**
   - Duolingo celebrates *every* lesson completion with confetti, fanfare sound, and Duo's celebratory pose.
   - **FocalPoint adaptation:** Every ritual completion, streak extension, rule fire, and reward earn trigger a fly-in celebration (see §2).

**Additional Duolingo References:**
- [Duolingo Onboarding Screen Mascot Animation using Figma](https://www.figma.com/community/file/1242450193697067214/duolingo-onboarding-screen-mascot-animation-using-figma), accessed 2026-04-23.
- [The evolution of the Duolingo owl — Apple Developer](https://developer.apple.com/news/?id=e2e1faj4), accessed 2026-04-23.

### Headspace & Finch Pet-App Reference

**Source:** [Finch Character Animation — Bella Alfonsi Portfolio](https://www.bellaalfonsi.com/work/finch-character-animation), accessed 2026-04-23; [The Magic of Finch: Where Self-Care Meets Enchanted Design](https://www.sophiepilley.com/post/the-magic-of-finch-where-self-care-meets-enchanted-design), accessed 2026-04-23.

Finch (a self-care pet app) uses:
- **Constant emotional reaction:** The bird tilts its head, blinks, and leans based on mood check-ins and activity streaks.
- **Haptic + Sound reinforcement:** Petting the bird triggers gentle haptics and soft coos.
- **Micro-animations on every state change:** No static screens.

**FocalPoint adaptation:** Adopt Finch's pattern of *always* showing Coachy reacting, not just on async boundaries. Add haptics to tab switches, button presses, and state transitions.

---

## 5. Implementation Deltas: File-by-File UI Updates

### Current Structure

```
Sources/FocalPointApp/Views/
  TasksView.swift              (line 26: ProgressView spinner)
  FocusModeView.swift          (line TBD: timer ProgressView)
  RitualsView.swift            (empty state only)
  WalletView.swift             (static Coachy HUD)
  StatsView.swift              (pull-to-refresh ProgressView)
  ActivityView.swift           (static history list)
  SettingsView.swift           (static Coachy toggle)
  OnboardingView.swift         (auth screens, no Coachy)
  LaunchCoachyView.swift       (current wake sequence)
  CoachyTabView.swift          (HUD placement)

Sources/MascotUI/
  CoachyView.swift             (static render)
  CoachyState.swift            (pose/emotion/bubble model)
  CoachyAnimationEngine.swift  (Rive/Lottie bridge)
```

### Delta 1: TasksView.swift

**Line 26 (current):**
```swift
ProgressView().controlSize(.large).frame(maxWidth: .infinity, maxHeight: .infinity)
```

**Replace with:**
```swift
CoachyLoadingView(loadingState: .loading(.tasksReload)) {
    taskList
}
```

**New dependency:** Import `MascotUI.CoachyLoadingView`.

---

### Delta 2: FocusModeView.swift

**Replace timer-active state rendering with:**
```swift
ZStack {
    // Background gradient
    
    // Coachy HUD (emotion-driven, see §3)
    CoachyView(
        state: coachyStateFor(sessionState),
        size: 160
    )
    .offset(y: -200) // Positioned above timer
    .matchedGeometryEffect(id: "coachy.focus", in: ns)
    
    // Timer + rings
    TimerRings(...) // existing
}
```

**State function (add to model):**
```swift
private func coachyStateFor(_ sessionState: SessionState) -> CoachyState {
    switch sessionState {
    case .starting:
        return CoachyState(pose: .workMode, emotion: .focused, bubbleText: "Let's go")
    case .active:
        return CoachyState(pose: .focusMode, emotion: .focused, bubbleText: nil)
    case .userExitAttempt:
        return CoachyState(pose: .sternToughLove, emotion: .concerned, bubbleText: "Stick with it")
    case .final10s:
        return CoachyState(pose: .confident, emotion: .proud, bubbleText: "Almost there!")
    case .complete:
        return CoachyState(pose: .achievement, emotion: .excited, bubbleText: "You crushed it!")
    }
}
```

---

### Delta 3: RitualsView.swift

**Add emotional Coachy HUD to the top:**
```swift
VStack {
    // Coachy reacts to ritual state
    CoachyView(
        state: ritualCoachyState,
        size: 140
    )
    .padding(.vertical, 12)
    
    // Ritual list / completion toggles
    ritualList
}
```

**Compute `ritualCoachyState` based on completion count:**
```swift
private var ritualCoachyState: CoachyState {
    let completed = rituals.filter { $0.completedToday }.count
    let total = rituals.count
    
    if total == 0 {
        return CoachyState(pose: .curiousThinking, emotion: .concerned, bubbleText: "What's today about?")
    } else if completed == total {
        return CoachyState(pose: .celebratory, emotion: .excited, bubbleText: "Amazing day!")
    } else {
        return CoachyState(pose: .encouraging, emotion: .happy, bubbleText: nil)
    }
}
```

---

### Delta 4: WalletView.swift

**Replace static HUD with reactive display:**
```swift
VStack {
    CoachyWalletDisplay(
        credits: wallet.balance,
        recentEarn: wallet.lastEarnedAmount,
        isPremium: user.isPremiumUser
    )
    
    // Reward list / redeem buttons
    rewardList
}
```

**New component `CoachyWalletDisplay`:**
```swift
struct CoachyWalletDisplay: View {
    @State private var showEarnAnimation = false
    
    var body: some View {
        VStack {
            CoachyView(
                state: walletCoachyState,
                size: 180
            )
            .scaleEffect(showEarnAnimation ? 1.1 : 1.0)
            .onChange(of: recentEarn) { old, new in
                if new > (old ?? 0) {
                    withAnimation(.easeOut(duration: 0.3)) {
                        showEarnAnimation = true
                    }
                    DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                        showEarnAnimation = false
                    }
                }
            }
            
            HStack {
                Label("\(credits)", systemImage: "bolt.fill")
                    .font(.headline)
                if isPremium {
                    Badge("Premium")
                }
            }
        }
    }
    
    private var walletCoachyState: CoachyState {
        if credits == 0 {
            return CoachyState(pose: .sleepyDisappointed, emotion: .disappointed)
        } else if recentEarn != nil {
            return CoachyState(pose: .celebratory, emotion: .excited, bubbleText: "Nice!")
        } else {
            return CoachyState(pose: .idle, emotion: .neutral)
        }
    }
}
```

---

### Delta 5: StatsView.swift

**Replace pull-to-refresh ProgressView:**
```swift
ZStack {
    if isLoadingStats {
        CoachyLoadingView(loadingState: .loading(.statsReload)) {
            statsDisplay
        }
    } else {
        statsDisplay
    }
}
.refreshable {
    try? await reloadStats()
}
```

---

### Delta 6: ActivityView.swift

**Add Coachy reaction based on activity:**
```swift
VStack {
    CoachyView(
        state: activityCoachyState,
        size: 150
    )
    .padding(.vertical, 16)
    
    // Activity timeline
    activityTimeline
}

private var activityCoachyState: CoachyState {
    let streakDays = activity.streak
    if streakDays == 0 {
        return CoachyState(pose: .sleepyDisappointed, emotion: .tired, bubbleText: "Miss you…")
    } else if streakDays >= 7 {
        return CoachyState(pose: .celebratory, emotion: .excited, bubbleText: "Look at you go!")
    } else {
        return CoachyState(pose: .confident, emotion: .proud)
    }
}
```

---

### Delta 7: SettingsView.swift

**Update Coachy voice toggle display:**
```swift
Toggle(isOn: $coachyVoiceEnabled) {
    VStack(alignment: .leading) {
        Label("Coachy's voice", systemImage: coachyVoiceEnabled ? "speaker.wave.2" : "speaker.slash")
        Text(coachyVoiceEnabled ? "Coachy will talk to you" : "Silent mode")
            .font(.caption)
            .foregroundStyle(.secondary)
    }
}
```

Add Coachy preview (show `.encouraging` if enabled, `.idle` if disabled).

---

### Delta 8: OnboardingView.swift

**Add Coachy progression through signup steps:**
```swift
VStack {
    // Step 1-2: Email / password auth
    // Step 3: OAuth chooser (GitHub, Google Calendar, Canvas)
    // Step 4: Ritual selection
    // Step 5: Daily time selection
    
    CoachyLoadingView(loadingState: currentLoadingState) {
        stepContent[currentStep]
    }
    
    HStack {
        if currentStep > 0 {
            Button("Back") { advance(by: -1) }
        }
        Spacer()
        Button(currentStep < 4 ? "Next" : "Done") {
            if currentStep < 4 {
                advance(by: 1)
            } else {
                finishOnboarding()
            }
        }
    }
}

private var currentLoadingState: CoachyLoadingView.LoadingState {
    switch onboardingPhase {
    case .authenticating:
        return .loading(.onboardingStep(currentStep))
    case .idle:
        return .idle
    }
}
```

---

### Delta 9: LaunchCoachyView.swift

**Already implements the wake sequence (§7 of `coachy-art-direction.md`). No changes required.**

---

### Delta 10: CoachyTabView.swift

**Update HUD placement and sizing:**
```swift
// Current HUD shows Coachy at fixed size in TabView toolbar
// New: Place Coachy in upper-right corner at 120pt
// HUD stays visible during tab switches
// On tab change, fade + scale transition (matched geometry effect)

ZStack(alignment: .topTrailing) {
    TabView(selection: $selectedTab) {
        TasksView()
            .tag(Tab.tasks)
        FocusModeView()
            .tag(Tab.focus)
        // ...
    }
    
    // Floating HUD
    CoachyView(state: hudCoachyState, size: 120)
        .matchedGeometryEffect(id: "coachy.hud", in: ns)
        .padding(16)
}
```

---

## 6. Motion Primitives: CoachyScene API Sketch

### Swift Caller Interface

```swift
/// Encapsulates a complete Coachy scene:
/// pose + emotion + accessories + bubbles + sound + haptic + particle effects.
struct CoachyScene {
    // Static properties
    let pose: CoachyPose
    let emotion: CoachyEmotion
    let accessories: [CoachyAccessory] // e.g., [.none, .headphones, .trophy]
    let bubbleText: String?
    
    // Dynamics
    let soundCueId: String? // e.g., "rule-fire-sparkle", "streak-success"
    let hapticPattern: CoachyHapticPattern?
    let particleSystems: [CoachyParticle]? // e.g., [.confetti, .coinTrail]
    
    // Animations
    let entry: CoachyEntryAnimation? // fly-in, fade-in, etc.
    let hold: TimeInterval // 0.0 for transient, >0 for persistent
    let exit: CoachyExitAnimation?
    
    // Lifecycle
    let onStart: (() -> Void)?
    let onComplete: (() -> Void)?
}

enum CoachyEntryAnimation {
    case fade(duration: TimeInterval, easing: UICurve)
    case flyIn(from: CoachyDirection, duration: TimeInterval, easing: UICurve)
    case grow(from: CGFloat, to: CGFloat, duration: TimeInterval, easing: UICurve)
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

enum CoachyAccessory: String, Codable {
    case none, headphones, glassesAndBook, trophy, shield, padlock
}

enum CoachyParticle {
    case confetti(count: Int)
    case coinTrail(count: Int)
    case redFlash(duration: TimeInterval)
}

enum CoachyHapticPattern {
    case lightTap
    case mediumTap
    case heavyTap
    case success // triple success pulse
    case warning // warning ×2
    case decayPattern(startIntensity: UIImpactFeedbackGenerator.FeedbackStyle, steps: Int)
}
```

### Swift View Container

```swift
struct CoachySceneView: View {
    let scene: CoachyScene
    @State private var isAnimating = false
    
    var body: some View {
        ZStack {
            // Background particle systems
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
            .onAppear {
                isAnimating = true
                scene.onStart?()
                // Play haptic
                if let hapticPattern = scene.hapticPattern {
                    playHaptic(hapticPattern)
                }
                // Play sound
                if let soundCueId = scene.soundCueId {
                    SoundEffectPlayer.play(soundCueId)
                }
                // Schedule exit
                if scene.hold > 0 {
                    DispatchQueue.main.asyncAfter(deadline: .now() + scene.hold) {
                        isAnimating = false
                        scene.onComplete?()
                    }
                }
            }
        }
        .transition(computeTransition())
    }
    
    private func computeTransition() -> AnyTransition {
        if let entry = scene.entry {
            // Build transition from entry animation
            switch entry {
            case .fade(let duration, let easing):
                return AnyTransition.opacity.animation(.timingCurve(...))
            case .flyIn(let from, let duration, let easing):
                let offset = offsetForDirection(from)
                return AnyTransition.offset(CGSize(...)).animation(...)
            default:
                return AnyTransition.opacity
            }
        }
        return AnyTransition.opacity
    }
}
```

### Rust-Side Director

```rust
// crates/focus-mascot/src/director.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoachyLoadingDirective {
    // Async boundary events
    AppColdLaunch,
    OnboardingStep { step: u8 },
    OAuthCallback,
    RuleEvalTick { fired: bool },
    ConnectorSync { source: String },
    RitualsGeneration,
    TaskCreation,
    TemplateInstall,
    AuditChainLoad { duration_ms: u64 },
    StatsReload,
    
    // Fly-in celebration events
    RuleFires,
    CreditLands { amount: u32 },
    StreakExtends { days: u32 },
    PenaltyEscalates { level: u8 },
    LockdownEngages,
    SessionComplete,
}

impl CoachyLoadingDirective {
    /// Maps directive to a complete CoachyScene description.
    pub fn to_scene(&self) -> CoachySceneDto {
        match self {
            Self::AppColdLaunch => CoachySceneDto {
                pose: CoachyPose::Sleepy,
                emotion: CoachyEmotion::Tired,
                bubble_text: Some("Morning! Ready?".to_string()),
                accessories: vec![],
                sound_cue: None,
                haptic_pattern: None,
                particles: vec![],
                entry_animation: None,
                hold_ms: 3500,
                exit_animation: None,
            },
            Self::RuleFires => CoachySceneDto {
                pose: CoachyPose::Celebratory,
                emotion: CoachyEmotion::Excited,
                bubble_text: None,
                accessories: vec![],
                sound_cue: Some("rule-fire-sparkle".to_string()),
                haptic_pattern: Some(HapticPatternDto::TapTwice { intensity: 1 }),
                particles: vec![ParticleDto::Confetti { count: 12 }],
                entry_animation: Some(EntryAnimationDto::FlyIn {
                    direction: "right",
                    duration_ms: 400,
                }),
                hold_ms: 800,
                exit_animation: Some(ExitAnimationDto::Shrink { duration_ms: 300 }),
            },
            // ... other cases ...
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoachySceneDto {
    pub pose: CoachyPose,
    pub emotion: CoachyEmotion,
    pub bubble_text: Option<String>,
    pub accessories: Vec<String>,
    pub sound_cue: Option<String>,
    pub haptic_pattern: Option<HapticPatternDto>,
    pub particles: Vec<ParticleDto>,
    pub entry_animation: Option<EntryAnimationDto>,
    pub hold_ms: u64,
    pub exit_animation: Option<ExitAnimationDto>,
}
```

### FFI Exposure (UniFFI)

```rust
#[uniffi::export]
pub fn coachy_scene_from_directive(directive_str: String) -> CoachySceneDto {
    // Parse JSON string → CoachyLoadingDirective enum
    // Call to_scene()
    // Return DTO for Swift
}
```

Swift call:
```swift
let directive = try JSONDecoder().decode(CoachyLoadingDirective.self, from: data)
let sceneDto = FocusCore.coachySceneFromDirective(directive.jsonString)
let scene = CoachyScene(from: sceneDto)
CoachySceneView(scene: scene).present()
```

---

## 7. Summary of Deltas

| File | Current | New | Effort |
|------|---------|-----|--------|
| `TasksView.swift` | ProgressView spinner line 26 | CoachyLoadingView | 1 h |
| `FocusModeView.swift` | Static timer + no Coachy | Emotion-driven Coachy, state function | 2 h |
| `RitualsView.swift` | Static list | Coachy HUD + state logic | 1.5 h |
| `WalletView.swift` | Static HUD | Reactive Coachy + earn animation | 2 h |
| `StatsView.swift` | ProgressView | CoachyLoadingView | 1 h |
| `ActivityView.swift` | Static history | Coachy reaction to streak | 1 h |
| `SettingsView.swift` | Toggle only | Coachy preview + voice toggle label | 0.5 h |
| `OnboardingView.swift` | No Coachy | Coachy progression + loading states | 2 h |
| `LaunchCoachyView.swift` | Current wake sequence | No change | 0 h |
| `CoachyTabView.swift` | Fixed-position HUD | Updated sizing + matched geometry | 1 h |
| **New:** `CoachyLoadingView.swift` | — | Generic async boundary container | 1.5 h |
| **New:** `CoachyWalletDisplay.swift` | — | Earn animation component | 1 h |
| **Rust:** `crates/focus-mascot/src/director.rs` | — | CoachyLoadingDirective + to_scene() | 2 h |
| **Rust:** UniFFI exposure | — | Export coachy_scene_from_directive | 0.5 h |

**Total estimate: ~16 hours** (implementer + designer for asset delivery).

---

## References

- [How Duolingo Uses Rive for Their Character Animation — DEV Community](https://dev.to/uianimation/how-duolingo-uses-rive-for-their-character-animation-and-how-you-can-build-a-similar-rive-mascot-5d19), accessed 2026-04-23.
- [Bringing Mascots to Life: Duolingo-Style Character Animation in Rive — Medium](https://uianimation.medium.com/bringing-mascots-to-life-duolingo-style-character-animation-in-rive-a075d648cf19), accessed 2026-04-23.
- [Duolingo Onboarding Screen Mascot Animation using Figma](https://www.figma.com/community/file/1242450193697067214/duolingo-onboarding-screen-mascot-animation-using-figma), accessed 2026-04-23.
- [The evolution of the Duolingo owl — Apple Developer](https://developer.apple.com/news/?id=e2e1faj4), accessed 2026-04-23.
- [Finch Character Animation — Bella Alfonsi Portfolio](https://www.bellaalfonsi.com/work/finch-character-animation), accessed 2026-04-23.
- [The Magic of Finch: Where Self-Care Meets Enchanted Design](https://www.sophiepilley.com/post/the-magic-of-finch-where-self-care-meets-enchanted-design), accessed 2026-04-23.
- [Mobile App Animation: 12 Patterns Every Designer Should Study](https://www.svgator.com/blog/mobile-apps-animation-examples/), accessed 2026-04-23.
- [What's Changing in Mobile App Design? UI Patterns That Matter in 2026](https://muz.li/blog/whats-changing-in-mobile-app-design-ui-patterns-that-matter-in-2026/), accessed 2026-04-23.
