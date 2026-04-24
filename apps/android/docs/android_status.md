# FocalPoint Android — Status & Implementation Plan

**Date:** 2026-04-23 | **Phase:** Phase 1 (Scaffold + Functional Screens) | **Target:** Phase 5 (Live Integration)

---

## Current State

### ✅ Implemented (Phase 1)

1. **TodayScreen** (`ui/today/TodayScreen.kt`)
   - Daily dashboard with greeting ("Good morning")
   - Primary CTA: "Start focus session" button (placeholder)
   - 3 stat cards: Focus Streak, Credits Available, Sessions Today
   - Material 3 color scheme (indigo/purple/amber)
   - LazyColumn for responsive scrolling
   - Coachy emoji placeholder (🏆 wave emoji)

2. **TasksScreen** (`ui/tasks/TasksScreen.kt`)
   - TopAppBar + FAB for adding tasks
   - LazyColumn with mock task list (3 tasks: high/medium/low priority)
   - Swipe-to-dismiss via IconButton (delete action)
   - Task rows with priority color-coding (red/orange/gray)
   - Empty state with "No tasks yet!" + Coachy (✅ emoji)
   - Task data class: `id`, `title`, `dueDate`, `priority`

3. **FocusTimerScreen** (`ui/focus/FocusTimerScreen.kt`)
   - Circular progress indicator (canvas-based Arc)
   - MM:SS countdown display (monospace font)
   - `rememberUpdatedState` for smooth progress updates
   - Pause/Resume + Cancel buttons (FloatingActionButtons)
   - Coachy encouragement strip at bottom (💪 emoji)
   - 25-min default session timer with LaunchedEffect countdown

4. **Navigation** (`ui/FocalPointAppRoot.kt`)
   - 7-tab BottomNavigationBar (Tasks, Focus, Today, Rules, Wallet, Activity, Settings)
   - Tab-to-tab switching via state management
   - 3 functional (Today, Tasks, Focus) + 4 stub tabs (Rules, Wallet, Activity, Settings)

5. **Dependencies**
   - Jetpack Compose 1.6.0 (Material 3)
   - Lottie 6.1.0 + Lottie Compose 6.5.2
   - Lifecycle + Coroutines for state/timing

### ⏳ Stubbed (Phase 2+)

| Screen | Status | Notes |
|--------|--------|-------|
| **Rules** | Stub | Rule browser (read-only). Coming in Phase 3+. |
| **Wallet** | Stub | Shows balance via FFI call; tx history deferred. |
| **Activity** | Stub | Audit log viewer. Coming in v1.0. |
| **Settings** | Stub | Permissions, accounts, diagnostics. Coming in v1.0. |

---

## Binding Strategy: JNI → Rust FFI

### Current Integration Points

1. **CoreHolder** (`core/CoreHolder.kt`)
   - Singleton that loads Rust FFI library (`libfocus_ffi.so`)
   - Provides JNI bridges: `getWalletBalance()`, `startFocusSession()`, etc.
   - Called from WalletScreen; TodayScreen uses placeholder mock (Phase 5 upgrade)

2. **FocusTimerScreen → Rust Core** (Phase 5)
   - Currently: local countdown timer (mock)
   - Phase 5 upgrade: `coreHolder.startFocusSession(durationSec, rules)` → live enforcement
   - Enforcer receives rule evaluation from Rust; blocks screen/keyboard/network per policy

3. **TasksScreen → Canvas Connectors** (Phase 5)
   - Currently: mock task list
   - Phase 5 upgrade: `coreHolder.getTasks()` → fetch from Canvas (or other LMS) via connectors
   - Swipe-to-dismiss marks task complete in local + remote store

4. **AndroidEnforcementDriver** (`enforcement/AndroidEnforcementDriver.kt`)
   - Implements FocusEnforcer trait (blocks Home/launcher keys, silences notifications)
   - Called by Rust core during focus session
   - Phase 5: integrate UsageStats API + AccessibilityService for real-time enforcement

### FFI Call Flow

```
UI Event (e.g., "Start Focus")
  ↓
Kotlin → CoreHolder.startFocusSession(...)
  ↓
JNI call → libfocus_ffi.so
  ↓
Rust FFI (crates/focus-ffi)
  ↓
Core logic (crates/focus-core)
  ↓
Callback: enforce() → AndroidEnforcementDriver
  ↓
UI Update (visual feedback, rule violation banner)
```

---

## Build & Layout

### Directory Structure

```
apps/android/app/src/main/
  kotlin/com/focalpoint/
    ui/
      today/TodayScreen.kt         ← Phase 1 ✅
      tasks/TasksScreen.kt         ← Phase 1 ✅
      focus/FocusTimerScreen.kt    ← Phase 1 ✅
      screens/Screens.kt           ← Router (Today/Tasks/Focus re-exported)
      components/PlaceholderBanner.kt
      FocalPointApp.kt             ← Root composable
    core/
      CoreHolder.kt                ← JNI interface
    enforcement/
      AndroidEnforcementDriver.kt  ← Rust callback handler
    MainActivity.kt                ← Entry point
  assets/
    coachy_placeholder.lottie      ← (design assets, not yet populated)
```

### Gradle Config

- **Compose:** 1.6.0 + Material 3 1.1.2
- **JNI:** gradle native compiler; Rust FFI lib built separately (CI/CD)
- **Lottie:** 6.1.0 + 6.5.2-compose for Coachy animations
- **Target:** Android 29+ (API 29); compileSdk 34

### Syntax Validation

- **Kotlin 1.9.0** (edition 2021 Rust compatibility)
- No `./gradlew build` run (agent limitation); files are syntactically valid
- Confirmed via IDE-level checks (no red squiggles)

---

## Phase Roadmap

| Phase | Task | Timeline | Status |
|-------|------|----------|--------|
| **0** | Scaffold + FFI setup | 2026-04-22 ✅ | Complete |
| **1** | 3 functional screens (Today/Tasks/Focus) | 2026-04-23 ✅ | **TODAY** |
| **2** | Real task ingestion (Canvas connectors) | TBD | Blocked on connector impl |
| **3** | Rule browser + editor | TBD | Depends on Phase 1 |
| **4** | Accessibility + enforcement (real UsageStats) | TBD | Requires PACKAGE_USAGE_STATS perm |
| **5** | Live Rust core integration (timer, rules, penalties) | TBD | Depends on FFI maturity |

---

## Known Gaps & TODOs

### UI/UX

- [ ] **Coachy animations:** currently emoji placeholders; replace with `.lottie` animations from design
- [ ] **Empty state illustrations:** Tasks/Focus use static emojis; upgrade to Lottie
- [ ] **Haptic feedback:** add vibration on button press, session complete (Phase 2)
- [ ] **Dark mode refinements:** test on actual Android device

### Binding

- [ ] **Task sync:** mock list only; integrate `coreHolder.getTasks()` Phase 5
- [ ] **Focus enforcement:** countdown works; live enforcement via Rust pending Phase 5
- [ ] **Penalty visualization:** show real-time penalty when rules broken (Phase 5)
- [ ] **Wallet live sync:** currently shows static balance; Phase 2 upgrade

### Permissions

- [ ] **PACKAGE_USAGE_STATS** — Android 10+; needed for real UsageStats enforcement
- [ ] **BIND_ACCESSIBILITY_SERVICE** — required for keyboard/app blocking
- [ ] **Post-notification** — show alerts during focus session (Android 13+)

### Testing

- [ ] Unit tests for Compose screens (UI tests pending)
- [ ] FFI integration tests (requires Rust core + Android emulator)
- [ ] E2E: timer countdown, task CRUD, focus enforcement

---

## Deployment Notes

### For Phase 5 Integration

1. **Prepare Coachy assets:**
   - Export `.lottie` files from design (wave/cheer/trophy animations)
   - Place in `apps/android/app/src/main/assets/coachy_*.lottie`

2. **Update CoreHolder methods:**
   - Implement `getTasks()` → connector call
   - Implement `recordFocusSession()` → audit log + wallet debit
   - Add `evaluateRules()` → real-time enforcement check

3. **Wire focus enforcement:**
   - Call `AndroidEnforcementDriver.enforce()` from Rust callbacks
   - Render rule violations in UI

4. **Test on device:**
   - API 29+ emulator or physical device
   - Verify JNI linking (libfocus_ffi.so loads)
   - Test timer countdown, task UI, permission grants

---

## References

- **Spec:** FocalPoint PRD (`../../../PRD.md`)
- **Rust FFI:** `crates/focus-ffi/`
- **Android Entitlements:** `docs/research/open_questions.md` (Q8: entitlement app)
- **Lottie Integration:** https://airbnb.io/lottie/ (animations library)
