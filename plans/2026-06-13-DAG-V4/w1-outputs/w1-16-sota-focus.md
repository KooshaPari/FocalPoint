# W1-16 — SOTA Focus & Window Management on macOS (2026)

> Research date: 2026-06-13
> Scope: Apple Focus / macOS window management, Things 3, Reflect, Raycast Focus
> Purpose: Identify patterns and APIs relevant to FocalPoint development

---

## 1. Apple Focus & macOS Window Management

**Apple Focus** (system-wide, introduced in macOS 12 Monterey, continuously refined) is a first-party focus filter that lives at the OS level. It silences notifications, hides badges, and can filter apps/accounts based on context (Work, Personal, Sleep, etc.). Apps can declare themselves as "Focus-sensitive" via the `FocusFilter` API, and Shortcuts can trigger focus mode changes automatically.

**macOS Window Management (2026)** — As of June 2026, macOS 26 (2025) is current and macOS 27 *Golden Gate* is previewed for fall 2026. The built-in toolkit remains:
- **Mission Control** — spatial overview of all windows and spaces.
- **Split View** — native two-app tiling via the green zoom button.
- **App Exposé** — per-app window spread.
- **Stage Manager** (introduced in macOS 13) — sidebar of recent apps + single focused window.
- **Apple Intelligence / Siri AI** (macOS 27) — adds context-aware assistance and Visual Intelligence.

macOS still does not expose a public, low-level "focus window" API for arbitrary third-party apps to programmatically raise or push other apps' windows without Accessibility permissions. The primary surface for window manipulation is **Accessibility + AppleScript**, which is what tools like Raycast and Hammerspoon use.

**Relevance to FocalPoint:**
- Apple Focus is the incumbent; any third-party focus app must either integrate with it (e.g., trigger Focus modes) or differentiate by offering more granular window-level control.
- FocalPoint should consider reading the current Focus state via `NSWorkspace` or FocusFilter APIs to avoid fighting the system.
- The lack of a public window-focus API is the exact gap that FocalPoint can fill — but it will require Accessibility permissions and likely AX API usage.

---

## 2. Things 3 (Cultured Code)

**What it is:** An award-winning personal task manager (Apple Design Award winner) for Mac, iOS, iPad, Apple Watch, and Vision Pro.

**Focus-relevant features:**
- **Today / This Evening** — separates "right now" tasks from "later today" tasks, reducing cognitive load.
- **Slim Mode** — collapses the sidebar with a two-finger swipe to cut out distractions and focus on the current list.
- **Quick Find / Type Travel** — instant keyboard navigation to any project or tag without breaking flow.
- **Multiple Windows** — open projects in separate panes, use multiple displays, split view, or spaces.
- **Shortcuts integration** — allows automation of focus contexts (e.g., "start deep work" shortcut that opens a specific project in Things).

**Relevance to FocalPoint:**
- Things 3 proves that focus is a UI/UX problem as much as a technical one. Slim Mode is a direct "hide chrome" pattern FocalPoint could emulate.
- The integration of task management with focus context (Today / This Evening) suggests FocalPoint could benefit from a lightweight task or intent layer.
- Things is not a window manager; it stays within its own app. FocalPoint can differentiate by acting across the entire OS.

---

## 3. Reflect

**What it is:** A networked note-taking app with a built-in AI assistant (GPT-4, Whisper). Positioned as a "second brain" for thinking, writing, and meeting notes.

**Focus-relevant features:**
- **End-to-end encryption** — private by default, reducing the mental overhead of sensitive notes.
- **Instant capture + frictionless search** — fast input and retrieval so ideas don't linger in working memory.
- **Calendar integration** — links meeting agendas directly to notes, so context is always available.
- **AI assistant** — helps organize, summarize, and connect ideas, effectively reducing cognitive load.
- **Networked backlinks** — forms a graph of ideas so you can trace thoughts without losing context.

**Relevance to FocalPoint:**
- Reflect is not a window/focus tool, but it demonstrates that modern productivity apps are converging on "cognitive focus" — reducing context switching and cognitive load.
- FocalPoint could consider a similar philosophy: not just blocking apps, but surfacing the *right* context (e.g., notes, tasks, or references) for the current focus mode.
- The clean, distraction-free UI of Reflect is a benchmark for how FocalPoint's own interface should feel.

---

## 4. Raycast Focus

**What it is:** A free, built-in focus session manager within the Raycast launcher (macOS 13+, also available on Windows). It blocks distracting apps and websites during timed sessions.

**Key capabilities:**
- **Goal-based sessions** — define one goal per session to minimize context switching.
- **Flexible duration** — from 5 minutes to an entire workday.
- **Block categories** — pre-built lists of social, shopping, and other distracting apps/websites.
- **App & website blocking** — blocks native apps and browser tabs (Safari, Chrome, Firefox, and derivatives) without requiring a browser extension.
- **Floating focus bar** — a subtle HUD showing time remaining and the current goal.
- **Snooze / pause** — if you need to access a blocked app, you can snooze it for a few minutes or pause the session entirely.
- **Keyboard-first** — starts via Raycast command palette; remembers preferences for quick repeat sessions.

**Relevance to FocalPoint:**
- Raycast Focus is the most direct competitor in the macOS focus space. It is free, well-integrated, and already has a large user base.
- FocalPoint must differentiate. Potential angles:
  - **Deeper window management** — Raycast has a separate Window Manager extension; FocalPoint could combine focus + window management natively.
  - **Focus mode integration** — tie into Apple Focus or Things/Reflect contexts rather than just blocking apps.
  - **Visual / spatial focus** — use Stage Manager-like window grouping or virtual desktop automation.
  - **Native macOS feel** — Raycast is a launcher app; FocalPoint could be a menu-bar or background-first utility that feels more native.

---

## Summary Matrix

| Tool | Focus Layer | Window Mgmt | Task Context | AI | Free | macOS Native |
|------|-------------|-------------|--------------|-----|------|--------------|
| Apple Focus | System | No | No | No (Siri AI in macOS 27) | Yes | Yes |
| Things 3 | App (tasks) | No (multi-window) | Yes | No | Paid | Yes |
| Reflect | App (notes) | No | Yes (calendar) | Yes | Paid | Yes |
| Raycast Focus | Session (blocker) | Separate ext | No | No | Yes | Yes (launcher) |
| **FocalPoint** | **TBD** | **TBD** | **TBD** | **TBD** | **TBD** | **Yes** |

**Strategic gaps for FocalPoint:**
1. No major tool combines *system-level focus* + *window management* + *task context* in a single native macOS utility.
2. Apple Focus is powerful but coarse (notification-level); there is room for fine-grained window-level focus.
3. Raycast Focus is app-blocking only; it does not rearrange or manage the window layout.
4. The AI layer (Reflect model) is not yet present in any focus/window manager — a potential future differentiator.
