# Coachy — FocalPoint Mascot Design System

Coachy is FocalPoint's parametric SVG mascot. 5 expressions × 4 intensities = 20 deterministic states, all version-controlled and generated from a single React component.

## Architecture

### Component (mascot.svg.tsx)

React component emitting parametric SVG with four control surfaces:

```typescript
interface MascotProps {
  expression?: 'idle' | 'happy' | 'focused' | 'concerned' | 'sleeping';
  intensity?: 'calm' | 'active' | 'intense' | 'post-rule';
  breathPhase?: number;  // [0-1] continuous oscillation
  blinkOpen?: boolean;
  size?: number;
  animated?: boolean;    // CSS @keyframes breathing
}
```

### Design Tokens (mascot-tokens.json)

Centralized color, dimension, and animation curve library:

- **Colors:** Brand primary (coral), semantics (ok/warn/block/info), neutrals
- **Dimensions:** Head radius, eye sizes, body proportions, cape geometry
- **Easing curves:** iOS ease-out-expo default, ease-in-out for breathing
- **Durations:** 150ms micro, 220ms standard, 400ms enter/exit, 3000ms breathing

### State Matrix (state-matrix.md)

Full grid of all 20 expressions:

| Expression | Meaning | Mouth | Brows | Use Case |
|------------|---------|-------|-------|----------|
| idle | Baseline | Neutral | Relaxed | Waiting, standby |
| happy | Positive | Smile | Relaxed | Reward, bypass |
| focused | Concentrating | Thin | Inward | Active lock |
| concerned | Warning | Wavy | Worried | Auth fail, low battery |
| sleeping | Dormant | None | Closed | Rule disabled, quiet hours |

Intensities modulate color, badge, and mouth size:
- **calm:** Primary coral, no badge
- **active:** Blue dot badge, engaged
- **intense:** Dark coral + red triangle badge, urgent
- **post-rule:** Green body + green + badge, rewarded

### Frame Export

20 optimized SVG frames, one per state:

```
frames/
  coachy-idle-calm.svg       (1.0 KB)
  coachy-idle-active.svg     (1.3 KB)
  ... [20 total]
```

**Total size:** 25 KB raw | ~9 KB gzipped

**Generation:** `node gen-frames.js` (deterministic, no randomness)

## Rendering Contract

### Component-Level (Interactive UI)

```tsx
import Mascot from '@assets/mascot/mascot.svg';

// Default: calm idle mascot
<Mascot />

// Breathing animation + state
<Mascot expression="happy" intensity="active" breathPhase={0.5} animated={true} />

// Static frame (preview, export)
<Mascot expression="focused" intensity="intense" breathPhase={0.5} animated={false} />
```

### Frame-Level (Static Export)

Import static SVG frame directly:

```tsx
import coachy from '@assets/mascot/frames/coachy-happy-post-rule.svg';
<img src={coachy} alt="Coachy: happy (post-rule)" />
```

Supports Rive integration for frame-by-frame animation sequencing.

### Motion Behavior

#### Breathing (parametric)
All states except sleeping show subtle continuous sway:
- **Amplitude:** 2 px (calm), 4 px (active/intense)
- **Frequency:** 3000 ms cycle
- **Easing:** ease-in-out (smooth, human-like)
- **Respects:** `prefers-reduced-motion` → instant state changes

#### Blinking (frame control)
Blink state parameterized for interactive contexts:
- **On:** Eyes fully open, pupils visible
- **Off:** Eyelid line (thin stroke across eye)
- **Sleeping:** Always closed

#### Color Modulation
By intensity:
- **calm:** Primary coral (#ff6b3d)
- **active:** Lighter coral (#ff8b65) + blue dot
- **intense:** Dark coral (#c94a21) + red triangle
- **post-rule:** Green (#2bb673) + green +

## Integration Patterns

### Pattern 1: Contextual Inline Display

Display Coachy's current emotional state during rule enforcement:

```tsx
function RuleCard({ rule, isActive }) {
  const intensity = isActive ? 'intense' : 'calm';
  const expression = rule.type === 'focus' ? 'focused' : 'idle';

  return (
    <div>
      <Mascot expression={expression} intensity={intensity} size={128} />
      <h2>{rule.name}</h2>
    </div>
  );
}
```

### Pattern 2: Animated Reward State

Trigger happy post-rule reward:

```tsx
function RewardBadge() {
  const [breathPhase, setBreathPhase] = useState(0);

  useEffect(() => {
    const id = setInterval(() => {
      setBreathPhase(p => (p + 0.05) % 1);
    }, 50);
    return () => clearInterval(id);
  }, []);

  return (
    <Mascot
      expression="happy"
      intensity="post-rule"
      breathPhase={breathPhase}
      animated={true}
    />
  );
}
```

### Pattern 3: Static Frame Export

Preview or export a specific state:

```tsx
export function ExportFrame(expr, intensity) {
  const svg = renderToString(
    <Mascot expression={expr} intensity={intensity} breathPhase={0.5} />
  );
  return fs.writeFileSync(`coachy-${expr}-${intensity}.svg`, svg);
}
```

## Design Principles

### Parametric & Deterministic
- No randomness. Same props → identical SVG.
- Version-controlled component. No design debt.
- Breathing and blink parameterized; animation logic is optional client-side.

### Proportional & Responsive
- Head, body, cape scale with single `size` prop.
- All spacing & strokes scale proportionally.
- SVG primitive (no bitmaps). Renders crisply at any size.

### Semantic & Warm
- Expression conveys intent: happy for reward, focused for lock, concerned for fail.
- Never cartoonish. Minimal linework; solid shapes. Matches FocalPoint brand tone.
- Acknowledges failure gracefully (concerned expression, warm voice).

### Accessible & Reduced-Motion Aware
- All animated states respect `prefers-reduced-motion`.
- Breathing collapses to instant state transition if motion is disabled.
- Color choices meet WCAG AA contrast ratios (dark mode primary).

## File Locations

```
assets/mascot/
  mascot.svg.tsx           # React component (291 LOC)
  mascot-tokens.json       # Design tokens
  state-matrix.md          # Expression grid + specs
  gen-frames.js            # Frame generator (no React DOM needed)
  frames/
    coachy-*.svg           # 20 optimized static exports
  preview.html             # Interactive state gallery
```

## Future Extensions

### Rive Integration
Import frames as Rive mesh deformers for smooth transition animation between states. Rive supplies frame sequencing; FocalPoint supplies state transitions.

### Accessibility & Alt Text
- Frames emit semantic SVG with `<title>` and `<desc>` for screen readers.
- Alt text convention: `"Coachy: {expression} ({intensity})"`.

### Internationalization
Colors and badge glyphs are culture-neutral. Motion timing respects device locale for reduced-motion preference.

## Version History

- **v0.1.0 (2026-04-25):** Initial parametric SVG primitive. 20 states, 25 KB total, ~9 KB gzipped. Component-level and frame-level rendering supported.
