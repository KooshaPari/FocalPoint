# iOS Coachy Renderer

iOS-side binding for `crates/focus-mascot` (Coachy). Renders `MascotState`
(pose × emotion × bubble_text) via Spline scenes.

**Character:** Coachy — "AI Focus Coach." Fiery flame-shaped character, red
cape, gold-star belt buckle. Warm-palette character against Phenotype cool-UI
chrome (see `docs/reference/design_tokens.md`).

**Status:** directory placeholder. Actual renderer + Spline assets pending.

## Design intent

- Rust core owns logical state (`Pose × Emotion × bubble_text`)
- Swift layer binds state changes to one Spline scene per pose;
  emotion drives per-scene parameter bindings (eye shape, head tilt, mouth
  curve, hand position)
- Bubble text comes from `MascotState::default_bubble_for()` at MVP;
  LLM-driven copy post-MVP

## Six canonical poses (from approved key art)

| Pose | Spline scene | Bubble copy (MVP) |
|---|---|---|
| `Confident` | hero-pointing.splineswift | "You can do harder things." |
| `Encouraging` | thumbs-up.splineswift | "You've got this!" |
| `CuriousThinking` | chin-hand.splineswift | "Let's figure it out." |
| `SternToughLove` | arms-crossed.splineswift | "Focus. No shortcuts." |
| `Celebratory` | arms-up-confetti.splineswift | "Task complete! Let's go!" |
| `SleepyDisappointed` | slumped-zzz.splineswift | "Rest up. Tomorrow's a win." |
| `Idle` | idle-breathe.splineswift | "Finish one task, earn a break." |

## Pending files

- `Package.swift` — SwiftPM target
- `Sources/Mascot/MascotView.swift` — SwiftUI wrapper
- `Sources/Mascot/MascotSplineRenderer.swift` — Spline scene driver
- `Sources/Mascot/MascotBindings.swift` — UniFFI glue from `focus-mascot`
- `Sources/Mascot/CoachyColors.swift` — warm palette asset tokens
- `Resources/scenes/` — 7 `.splineswift` exported scenes (one per pose)
- `Resources/key-art.png` — hero reference image from user

## Accessibility

- VoiceOver labels derived from `Pose` + `bubble_text`
- Reduced motion: static image per pose instead of Spline scene (user setting)
- Dynamic type: bubble text scales with system size
