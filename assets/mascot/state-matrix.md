# Coachy State Matrix — 20 Expressions

5 expressions × 4 intensities = 20 states. Each rendered as deterministic parametric SVG.

## Expressions

| # | Name | Meaning | Mouth | Brows | Eyes | Use Case |
|---|------|---------|-------|-------|------|----------|
| 1 | idle | Default state | Neutral line | Relaxed | Open | Standing by, waiting for interaction |
| 2 | happy | Positive reinforcement | Smile arc | Relaxed | Squinting | Reward, streak, bypass earned |
| 3 | focused | Concentrating hard | Thin line (serious) | Inward angled | Narrowed | Active rule enforcement, locked |
| 4 | concerned | Warning / low energy | Wavy mouth | Worried (V-shape) | Open | Token expiry, low battery, auth fail |
| 5 | sleeping | Resting / dormant | None (zzz line) | Closed | Closed | Rule disabled, quiet hours active |

## Intensities

| # | Name | Meaning | Color | Badge | Use Case |
|---|------|---------|-------|-------|----------|
| 1 | calm | Baseline, relaxed | Primary coral | None | Normal operation |
| 2 | active | Engaged, aware | Primary (slightly lighter) | Blue dot | Timer running, rule monitoring |
| 3 | intense | High stakes, urgent | Dark coral | Red triangle | Block active, penalty pending |
| 4 | post-rule | Just rewarded, pleased | Green (#2bb673) | Green + | Bypass granted, rule paused |

## State Grid (20 Total)

```
              calm          active        intense       post-rule
idle    →   1. relaxed    2. alert       3. warned      4. pleased
happy   →   5. content    6. encouraged  7. proud       8. triumphant
focused →   9. concentr.  10. locked     11. strict     12. completed
concerned→  13. worried   14. checking   15. alarmed    16. recovered
sleeping→   17. dormant   18. light      19. paused     20. waking
```

## Rendering Notes

- **Breathing:** All states except sleeping show subtle vertical sway (2-4px depending on intensity).
- **Blinking:** Automatic in interactive contexts; parameterized for frame export.
- **Color modulation:** Badge and accent colors change by intensity; body defaults to primary coral unless post-rule (green).
- **SVG size:** All frames 256×320 px (4:5 ratio), optimized with svgo for minimal file size.
- **Deterministic:** No randomness; all states reproducible via props `(expression, intensity, breathPhase, blinkOpen)`.

## Frame Export Process

1. Generate 20 SVG files via parametric React component
2. Run `svgo` optimization on each frame
3. Output to `frames/coachy-<expr>-<intensity>.svg`
4. Total: ~20 KB (gzipped) for all 20 frames

## Animation Context

- **Component-level:** `<Mascot animated={true} />` enables CSS `@keyframes breathe`
- **Frame-by-frame (Rive integration):** Each SVG frame can be imported as a static asset; Rive driver supplies frame sequencing
- **Motion guidelines:** Respect `prefers-reduced-motion`; collapses to instant state changes in that context
