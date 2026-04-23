# iOS Mascot Binding

iOS-side binding for `crates/focus-mascot`. Renders `MascotState` transitions
via Spline (or comparable 3D runtime).

**Status:** directory placeholder. Actual `MascotSplineRenderer.swift` +
`MascotView.swift` + Spline scene assets pending Phase 1 UX work.

## Design intent

- Rust core owns logical state (`Pose` × `Emotion` × `bubble_text`)
- Swift side binds state changes to Spline scenes — one scene per major pose,
  emotion drives per-scene parameters (eye shape, head tilt, mouth curve)
- LLM-driven `bubble_text` generation comes later; MVP uses templates

## Pending files

- `Package.swift` — SwiftPM target
- `Sources/Mascot/MascotView.swift` — SwiftUI view wrapping Spline
- `Sources/Mascot/MascotSplineRenderer.swift` — Spline scene driver
- `Sources/Mascot/MascotBindings.swift` — UniFFI-generated glue from `focus-mascot`
- `Resources/scenes/` — .splineswift exported scenes per pose
