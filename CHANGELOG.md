# Changelog

## 0.0.2 — 2026-04-22 (Q-resolutions)

### Decided (see `docs/research/open_questions.md`)
- Q1: project name final = **FocalPoint**
- Q2: **iOS-only MVP**, single-device. Android deferred beyond Phase 5.
- Q3: QR-only unlock MVP; NFC in Phase 1.5
- Q4: Rust default for services; Zig considered case-by-case; services deferred
- Q5: **Foqos** (MIT, active, 465★) approved for `apps/ios/` donor. Reef deferred with Android.
- Q6: **mascot scaffolded** — new crate `focus-mascot` + `apps/ios/Mascot/` placeholder
- Q7: template marketplace deferred to Phase 3
- Q8: entitlement app non-urgent (no publish till year-end)

### Added
- `crates/focus-mascot/` — `Pose` / `Emotion` / `MascotEvent` / `MascotState` / `MascotDriver` trait / `MascotMachine` stub
- `apps/ios/Mascot/README.md` — iOS Spline renderer placeholder
- Workspace now has 17 crates.

### Changed
- PLAN.md phase graph: iOS-focused; Android marked deferred
- README stack section: SwiftUI-only; cross-native frameworks explicitly rejected per ADR-001

## 0.0.1 — 2026-04-22 (scaffold)

### Added
- Rust workspace with 16 crate stubs (no business logic).
- iOS + Android app directory placeholders.
- Spec docs carved from `ChatGPT-App Architecture for Screen Time.md`:
  PRD, ADR index + 9 ADR files, FUNCTIONAL_REQUIREMENTS, PLAN, USER_JOURNEYS.
- Connector SDK spec stub + ecosystem strategy stub.
- `.gitignore`, `LICENSE` (MIT OR Apache-2.0).

### Status
- No impls — only type names, trait signatures, module boundaries.
- Stack decision locked (ADR-001..ADR-009).
- Open questions tracked in `docs/research/open_questions.md`.
