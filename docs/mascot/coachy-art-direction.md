# Coachy — Art Direction Brief

Audience: character designer + motion designer contracted to deliver the
production Coachy asset set.
Scope: this brief locks identity, pose/accessory taxonomy, deliverables, color
tokens, and the launch-screen wake sequence. It does **not** re-open character
redesign — Coachy is locked.

## 1. Character identity (LOCKED — do not redesign)

- **Name:** Coachy. No renames, no "Meevo", no alternate mascots.
- **Silhouette:** a fiery flame. Teardrop/flame body is the primary read; eyes
  and mouth sit on the upper third of the flame, arms emerge mid-body.
- **Palette:** orange-red gradient from base (deep red-orange) through core
  (orange) to edge (warm gold). Exact tokens in §5.
- **Personality band:** *stern → tough-love → encouraging → celebratory*. Coachy
  is a drill coach with a warm core, not a cheerleader and not a scold.
- **Cape:** retained. Red cape reads behind the flame body at shoulder-to-ankle.
- **Belt:** allowed as a simple band if the designer wants to anchor the waist.
  **No star buckle** — the earlier gold-star belt buckle was explicitly removed
  and must not return.
- **Reference art:** image #3 in the original mascot thread (the fiery-flame
  rendering with stern-to-encouraging range). That is canon.

## 2. Pose catalog (expand from the current 7 to 14)

The in-app enum (`CoachyPose` in `Sources/MascotUI/CoachyState.swift`) will
track this list 1:1. Currently 7 cases ship; the designer expands to 14.

| # | Pose ID (Swift case) | Description |
|---|----------------------|-------------|
| 1 | `idle` | Neutral stance, arms relaxed, looking forward. Breathing-loop ready. |
| 2 | `confident` | Weight forward, pointing finger, slight smirk. |
| 3 | `encouraging` | Open-hand gesture toward viewer, warm smile, slight lean. |
| 4 | `curiousThinking` | Chin-on-hand, one eye slightly narrowed, mouth a small "o". |
| 5 | `sternToughLove` | Crossed arms, level brow, neutral mouth. *Not* angry. |
| 6 | `celebratory` | Both arms up, mouth open wide, flame wisps flaring. |
| 7 | `sleepyDisappointed` | Half-lidded eyes, slumped posture, mouth flat; small Z's optional. |
| 8 | `thumbsUp` | Single arm forward with thumb, closed-smile approval. |
| 9 | `workMode` | Sleeves-rolled feel; lean-in, focused brow, arms working unseen below frame. |
| 10 | `cheering` | Arms punching air alternately, flame flaring on the up-beat. |
| 11 | `focusMode` | Wears **Headphones** accessory; eyes half-closed in flow; slight head-bob loop. |
| 12 | `studyMode` | Wears **Glasses + Book** accessory; reading downward gaze. |
| 13 | `achievement` | Holds **Trophy** accessory overhead; celebratory face. |
| 14 | `lockdown` | **Padlock** accessory hovering in front; stern face; arms wide blocking. |

Note: poses 11–14 carry a *default accessory* but the accessory itself is an
orthogonal layer (§3) so it can also be composed onto other poses.

## 3. Accessory catalog (orthogonal layer)

Accessories ride on top of any compatible pose via a separate Rive layer /
SVG group. Designer delivers each as a standalone asset rigged to Coachy's
head/hand/front-of-body anchor points.

| Accessory | Anchor | Notes |
|-----------|--------|-------|
| Headphones | Head (top + sides) | Gold metal band, dark cups; subtle cable flick on movement. |
| Glasses + Book | Face + both hands | Round frames; book held open, pages render as simple spread. |
| Trophy | Both hands, raised | Gold cup with red accent ribbon; sparkle loop. |
| Shield | One arm, forward | Red shield with flame emblem; blocks-forward pose hint. |
| Padlock | Mid-body, floating | Simple padlock, optionally with a subtle pulse. |

## 4. State × Accessory × Emotion matrix

`CoachyEmotion` already exposes 8 values (`neutral`, `happy`, `excited`, `proud`,
`concerned`, `disappointed`, `tired`, `focused`). Emotion modulates
eye-shape / mouth-shape *within* a pose. The Rive state machine should accept
three independent inputs:

```
input pose       : enum(14)   // pose from §2
input accessory  : enum(6)    // none + 5 from §3
input emotion    : enum(8)    // matches CoachyEmotion
```

The valid matrix is the full cross-product (14 × 6 × 8 = 672 combinations) but
the shipping catalog only needs *pose × emotion* keyframes (112) plus the 5
accessory overlays rendered independently. Emotion is a blend parameter, not a
separate animation.

Default emotion per pose (used when caller does not override):

| Pose | Default emotion |
|------|-----------------|
| idle | neutral |
| confident | proud |
| encouraging | happy |
| curiousThinking | focused |
| sternToughLove | concerned |
| celebratory | excited |
| sleepyDisappointed | disappointed |
| thumbsUp | happy |
| workMode | focused |
| cheering | excited |
| focusMode | focused |
| studyMode | focused |
| achievement | proud |
| lockdown | concerned |

## 5. Color tokens (canonical source)

All color references MUST resolve through
`apps/ios/FocalPoint/Sources/DesignSystem/Palette.swift` (`CoachyColors`,
`AppColors`). Designer delivers assets with layer names matching these tokens
so we can retint at runtime.

| Token | Hex | Use |
|-------|-----|-----|
| `CoachyColors.flameEdge` | `#F8B26A` | Top/outer flame gradient |
| `CoachyColors.flameCore` | `#F07B3F` | Mid flame body |
| `CoachyColors.flameBase` | `#E05A26` | Base of flame / arms |
| `CoachyColors.cape` | `#D4462E` | Cape |
| `CoachyColors.buckleGold` | `#F9C86A` | Trophy / headphone accents |
| `CoachyColors.eyes` | `#121212` | Eyes, mouth stroke |
| `AppColors.background` dark | `#0F1012` | Preview backdrop (dark mode default) |
| `AppColors.surface` dark | `#353A40` | Bubble / card backdrop |
| `AppColors.accent` | `#7EBAB5` | Chat-bubble stroke |

If the designer proposes a palette tweak, update `Palette.swift` in the same PR
— tokens are the source of truth, not the asset.

## 6. Deliverables per pose

For each of the 14 poses:

1. **Rive `.riv` state-machine** (primary). Single `Coachy.riv` bundle containing
   the full state machine named `CoachyStateMachine` with the three inputs from
   §4. This is what the iOS runtime consumes.
2. **Lottie `.json` per pose** (fallback). One file per pose ID, filename
   `coachy-<poseId>.json`. Used on platforms where Rive runtime is unavailable
   or a single pose animation is cheaper than booting the state machine.
3. **Static `.svg` per pose** (previews). Used by Mac/web preview surfaces,
   docs, marketing. Filename `coachy-<poseId>.svg`. Must use the color tokens
   from §5 as named CSS variables on the SVG root.

Accessories ship as:

- 1 layer/group in the main `Coachy.riv` (toggled by the `accessory` input).
- 1 standalone `accessory-<name>.svg` so we can composite onto the static SVGs.

Drop location (once delivered):

```
apps/ios/FocalPoint/Resources/Mascot/
  Coachy.riv
  Lottie/coachy-<poseId>.json
  SVG/coachy-<poseId>.svg
  SVG/accessory-<name>.svg
```

The `CoachyAnimationEngine` in `Sources/MascotUI/` auto-detects
`Coachy.riv` and switches from the SwiftUI fallback renderer to the Rive
pipeline — no code change required at asset delivery time.

## 7. Launch-screen sequence (Coachy sleep → wake)

Three phases, total duration ~3.5 s. All motion uses `easeInOut` with the
durations below. `matchedGeometryEffect(id: "coachy.identity", in: ns)` is
required on the Coachy view in **every** phase so identity is preserved into
the first `HomeView`.

| Phase | Duration | Pose | Emotion | Visual beat |
|-------|----------|------|---------|-------------|
| 1. Sleeping | 1.0 s | `sleepyDisappointed` | `tired` | Z's drifting up, gentle breathing scale (0.98 ↔ 1.02, 1 Hz). No bubble. |
| 2. Eyes open + stretch | 1.5 s | `encouraging` | `neutral` | Eyes animate open; flame wisp flares; small rotational stretch. Still no bubble. |
| 3. Full wake + first bubble | 1.0 s | `confident` | `proud` | Bubble fades in with text "Ready?"; Coachy settles into the hero pose. |

On completion, invoke the `onFinish` callback. The calling view should run the
`matchedGeometryEffect` transition directly into `HomeView`'s Coachy frame so
Coachy appears to walk straight out of the launch animation into the app.

The launch sequence is gated by `AppStorage("app.hasSeenWake")` — plays once on
first cold launch. This is independent of onboarding (`app.hasOnboarded`) so
returning-but-pre-onboarding users still get the wake sequence once.

## 8. Acceptance checklist (designer → engineering)

- [ ] All 14 poses exist as Rive states in `Coachy.riv` with correct input wiring.
- [ ] All 14 Lottie fallbacks export cleanly.
- [ ] All 14 static SVGs use tokenized fills (CSS vars).
- [ ] All 5 accessories render as orthogonal layers (Rive) and standalone SVGs.
- [ ] Launch-sequence phases 1/2/3 render at the durations in §7.
- [ ] No star-buckle anywhere. No Meevo anywhere.
- [ ] Palette tokens in `Palette.swift` match the delivered assets 1:1.
