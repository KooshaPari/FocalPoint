# Motion Pipeline — Rive + Lottie

FocalPoint motion system: SVG primitives → Rive state machine + 12 Lottie micro-animations.

## Overview

- **Rive State Machine**: 20-state mascot controller (5 expressions × 4 intensities)
- **Lottie Animations**: 12 micro-interactions for UI feedback (rule-fire, error-shake, etc.)
- **Output**: JSON intermediate formats (Rive editor import + direct Lottie rendering)
- **Size Gates**: Rive ≤50KB; Lottie ≤30KB per file
- **iOS Integration**: Swift bindings in `apps/ios/FocalPoint/Animations/`

## Directory Structure

```
assets/motion/
├── src/
│   ├── rive-converter.rs       # Generate Rive state machine JSON
│   └── lottie-converter.rs     # Generate 12 Lottie animations
├── rive/                       # Output: Rive state machine
│   └── coachy-state-machine.json (18KB)
├── lottie/                     # Output: 12 Lottie animations
│   ├── rule-create.json
│   ├── rule-fire.json
│   ├── intervention-warn.json
│   ├── emergency-exit.json
│   ├── achievement-unlock.json
│   ├── mascot-blink.json
│   ├── mascot-yawn.json
│   ├── focus-start.json
│   ├── focus-end.json
│   ├── sync-pulse.json
│   ├── error-shake.json
│   └── success-checkmark.json
├── Cargo.toml                  # Rust project (serde, serde_json)
├── Cargo.lock
├── build.sh                    # Main build orchestrator
├── size-gate.sh                # Size enforcement (CI gate)
└── tests/
    └── motion_tests.rs         # Unit tests (parsing, frame counts, size gates)
```

## Build

### Full Pipeline
```bash
cd assets/motion
./build.sh
```

### Individual Steps
```bash
# Generate Rive state machine (JSON intermediate)
cargo run --release --bin rive-converter

# Generate 12 Lottie animations
cargo run --release --bin lottie-converter

# Size gate verification (CI gate)
bash size-gate.sh
```

### Tests
```bash
cd assets/motion
cargo test --test motion_tests -- --nocapture
```

## Outputs

### Rive: coachy-state-machine.json (18KB)
**Format**: Rive Runtime JSON intermediate format

**States**: 20 total (5 expressions × 4 intensities)
- **Expressions**: idle, happy, focused, concerned, sleeping
- **Intensities**: calm, active, intense, post-rule

**Transitions**: 92 (optimized; same-expression intensity changes + focus toggle)

**Next Step**: Import into [Rive Editor](https://rive.app) for binary (.riv) export and refinement.

### Lottie: 12 Micro-Animations (3KB total)
**Format**: Lottie JSON (LottieJS 5.7.0)

| Name | Frames | Purpose |
|------|--------|---------|
| rule-create | 30 | UI appear on rule creation |
| rule-fire | 15 | Visual pulse when rule triggered |
| intervention-warn | 40 | Warning shake before intervention |
| emergency-exit | 50 | Rapid red flash on emergency exit |
| achievement-unlock | 45 | Burst animation on achievement |
| mascot-blink | 5 | Quick eye close/open |
| mascot-yawn | 30 | Mascot yawn animation |
| focus-start | 20 | Focus session starting |
| focus-end | 20 | Focus session ending |
| sync-pulse | 20 | Data sync shimmer (looping) |
| error-shake | 12 | Error state rapid shake |
| success-checkmark | 25 | Checkmark stroke animation |

**Size**: ~278 bytes per file; 3.3KB total (well under 30KB per-file limit)

## iOS Integration

Swift components in `apps/ios/FocalPoint/Animations/`:

### RiveAnimationView.swift
UIViewRepresentable wrapper for Rive Runtime (pod 'Rive')

```swift
RiveAnimationView(stateMachine: "coachy-state-machine", autoplay: true)
    .frame(height: 320)
```

**Requirements**:
- Rive iOS library installed via CocoaPods
- `coachy-state-machine.riv` binary bundled in app (export from editor)

### LottieAnimationView.swift
UIViewRepresentable wrapper for Lottie iOS (pod 'lottie-ios')

```swift
LottieAnimationView(name: "rule-fire", loopMode: .loop, speed: 1.0)
    .frame(height: 120)
```

**Requirements**:
- Lottie iOS library installed via CocoaPods
- Lottie JSON files bundled in app bundle

### AnimationDemoScreen.swift
Complete demo with 3-tab gallery: Rive, Lottie index, and animation reference.

## Rive Editor Handoff

**Status**: JSON intermediate ready; binary export awaits editor.

**Workflow**:
1. Open [Rive Editor](https://rive.app)
2. Import `coachy-state-machine.json` (File → Import)
3. Refine state machine (add guards, adjust transitions)
4. Export as `.riv` binary (File → Export)
5. Copy to `apps/ios/FocalPoint/` and bundle in app target

**Why JSON intermediate**: Rive binary format requires the editor for creation. We emit JSON to avoid vendor lock-in and support scripted generation.

## Size Gates (CI)

**Lottie**: ≤30KB per file (3.3KB actual)
**Rive**: ≤50KB per file (18KB actual)

Gate enforced via `bash size-gate.sh` in CI pipelines. Violations cause non-zero exit.

## Testing

Unit tests verify:
- **FR-MOTION-001**: Rive state machine structure (20 states, properties) + Lottie frame counts
- **FR-MOTION-002**: All 12 animations exist, parse, have expected dimensions (256×320)
- **Size gates**: Lottie ≤30KB; Rive ≤50KB (CI enforcement)

```bash
cargo test --test motion_tests -- --nocapture
```

## Development

**Converters** are idempotent Rust binaries (no side effects, deterministic JSON).

To modify animation details:
1. Edit `src/rive-converter.rs` or `src/lottie-converter.rs`
2. Run build: `./build.sh`
3. Verify size gates: `bash size-gate.sh`
4. Commit output JSON files + source changes

**Tracing**:
- All tests reference FR-MOTION-001/002
- All output files are deterministic from source (no binary blobs)

## Dependencies

- **serde**: JSON serialization
- **serde_json**: JSON parsing/generation
- **anyhow**: Error handling (minimal; panics on write failures)

No external animation tools required (no Adobe Animate, no bodymovin, no Rive Runtime compile).

## References

- Coachy mascot: `assets/mascot/state-matrix.md`
- iOS app: `apps/ios/FocalPoint/Animations/`
- Rive Documentation: https://rive.app/docs
- Lottie Documentation: https://airbnb.io/lottie/
