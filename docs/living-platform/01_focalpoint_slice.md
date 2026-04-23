# FocalPoint Slice — Morning Brief Experiment

> Companion to `00_design.md`. Planner-agent deliverable, no code.
> Scope: the smallest living-platform experiment that is shippable in a single focused agent session.

## 1. The Experiment in One Sentence

An agent may autonomously restyle the Morning Brief's chatbar and hero during an idle window; the change appears as a 300–500 ms visual morph on both web (`apps/web-admin`) and iOS (`apps/ios`), without disturbing the user's intention draft, caret, mascot, or focus timer.

## 2. Entities Involved (stable `entity_id` required)

| Entity ID | Owner crate | Web representation | iOS representation |
|---|---|---|---|
| `brief.chatbar` | `focus-rituals` | `<ChatBar data-entity-id="brief.chatbar">` | `ChatBar().matchedGeometryEffect(id: "brief.chatbar", in: ns)` |
| `brief.hero` | `focus-rituals` | `<Hero data-entity-id="brief.hero">` | `Hero().matchedGeometryEffect(id: "brief.hero", in: ns)` |
| `brief.intention_draft` | `focus-rituals` | `<textarea data-entity-id="brief.intention_draft">` | `TextEditor().focused($focus, equals: .draft)` |
| `brief.mascot` | `focus-mascot` | canvas + state machine | `MascotView` with `@StateObject` pose store |
| `brief.timer` | `focus-rituals` | read-only subscription | read-only subscription |

The first three are the restyle targets. The last two are **preservation-only** — any transition that mutates them fails the planner contract and is rejected at governance time.

## 3. Participating Surfaces

**Rust core surfaces:**
- `crates/focus-domain` — add `WorldModel::stage(patch)` + `WorldModel::commit(patch_id)`; extend journal with `{before, after, dwell_ms}`.
- `crates/focus-rituals` — emit `IdleWindow::Entered / Exited` events; these gate autonomous-patch eligibility.
- `crates/focus-governance` *(new)* — patch schema (serde), classifier (cosmetic vs destructive), approval bus (stub for slice; no destructive ops in slice scope).
- `crates/focus-events` — add `PerceptualEvent` variant + contract tests.
- `crates/focus-ffi` — expose patch apply + subscribe for Swift.

**Web surfaces (`apps/web-admin`):**
- Transition planner module (TS) consumes world-model patch stream.
- Motion's `<motion.div layout>` wraps `brief.chatbar` and `brief.hero`.
- `document.startViewTransition` invoked on structural patches; shared `view-transition-name: "brief.chatbar"` on both states.
- Draft persistence hook on `brief.intention_draft` writes to IDB on every keystroke, rehydrates post-transition.

**iOS surfaces (`apps/ios`):**
- Since View Transitions API is web-only, iOS uses SwiftUI-native primitives:
  - `matchedGeometryEffect(id:in:)` on `brief.chatbar` and `brief.hero` to morph frames ([SwiftUI Lab](https://swiftui-lab.com/matchedgeometryeffect-part1/), accessed 2026-04-23).
  - `withAnimation(.spring(response: 0.4, dampingFraction: 0.8)) { ... }` wrapping the patch-apply closure.
  - `.animation(.smooth, value: chatbarShape)` for the shape property itself.
  - `@FocusState` and `@AppStorage` preserve caret and draft across view rebuilds ([Hacking with Swift](https://www.hackingwithswift.com/quick-start/swiftui/how-to-synchronize-animations-from-one-view-to-another-with-matchedgeometryeffect), accessed 2026-04-23).
- A `PatchRenderer` actor translates Rust-core patches into SwiftUI state mutations inside a single `withAnimation` block so every property interpolates in lockstep.

## 4. What "Autonomous Styling Change" Actually Means Here

Concretely, an agent (running in the `cheap-llm` lane, not a Claude call) emits:

```
{
  op: "restyle",
  target: "brief.chatbar",
  props: { shape: "pill", radius: 999, px: 24, py: 12 },
  rationale: "dwell-time on block variant lower than pill variant in journal",
  window: "idle"
}
```

The classifier routes it to cosmetic-autonomous. The planner picks:

- **Web:** wrap state swap in `document.startViewTransition`; Motion handles the radius/padding tween because those props are on a `layout`-enabled component; CSS custom properties handle color. Single frame, 400 ms.
- **iOS:** `withAnimation(.smooth(duration: 0.4)) { chatbarShape = .pill }`; `matchedGeometryEffect` on the `.background(Capsule())` vs `.background(RoundedRectangle)` morphs the stroke path. `@FocusState` is untouched — caret survives.

The user sees the chatbar smoothly inflate from a block into a pill while they are mid-sentence. The sentence does not vanish. The timer keeps ticking.

## 5. Acceptance Criteria

1. Agent-emitted cosmetic patch on `brief.chatbar` renders as a single continuous animation on web (Chrome, Safari) — verified by Playwright recording showing no remount (entity's `data-entity-id` element retains same JS object identity via `WeakRef` check).
2. Same patch renders as a continuous animation on iOS — verified by UI test asserting `matchedGeometryEffect` animation completes and `@FocusState` unchanged.
3. `brief.intention_draft` content and caret index are byte-identical before and after transition.
4. `brief.timer` tick count is monotonic across the transition (no pause).
5. `brief.mascot` pose index at `t_before` and `t_after + 0` is identical.
6. Journal entry exists with `{before, after, dwell_ms, agent_id}`.
7. If idle window is exited mid-transition, transition completes but subsequent patches from the same agent are deferred.

## 6. Agent-Time Budget

See §9 of `00_design.md`. Total: **~60 min wall-clock, ~90 tool calls, 3–4 parallel subagents.** iOS parity adds ~30 min if attempted in the same session; recommend deferring to a follow-up session.

## 7. Out of Scope for Slice

- Destructive patches, governance approval UI, worktree preview surfacing, OTel dashboard embedding, MF 2.0 scaffold. Those are follow-on slices and gated on this one's success.

## Sources

- [SwiftUI Lab — matchedGeometryEffect Part 1](https://swiftui-lab.com/matchedgeometryeffect-part1/) (accessed 2026-04-23)
- [Hacking with Swift — matchedGeometryEffect](https://www.hackingwithswift.com/quick-start/swiftui/how-to-synchronize-animations-from-one-view-to-another-with-matchedgeometryeffect) (accessed 2026-04-23)
- [Create with Swift — Animated transitions with matchedGeometryEffect](https://www.createwithswift.com/create-an-animated-transition-with-matched-geometry-effect-in-swiftui/) (accessed 2026-04-23)
- [Motion — React Layout Animations](https://motion.dev/docs/react-layout-animations) (accessed 2026-04-23)
- [MDN — View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API) (accessed 2026-04-23)
