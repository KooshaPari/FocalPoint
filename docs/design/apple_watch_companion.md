# FocalPoint Apple Watch Companion App

## Overview

This document specifies the architecture, UX, and implementation strategy for a WatchOS companion app that extends FocalPoint to the Apple Watch. The companion prioritizes real-time focus timer control on the wrist, quick glances at credits balance, haptic Coachy nudges, and seamless sync with the iPhone host app.

## Strategic Rationale

Apple Watch is the natural extension of FocalPoint's ambient wellness model:
- **Wrist proximity**: Provides immediate timer feedback without reaching for the phone
- **Haptic channel**: Coachy can deliver non-intrusive nudges via taptic engine (perfect for focus sessions)
- **Complication real estate**: Balance/timer live on the watch face, requiring no app launch
- **Live Activity integration**: Sync with iPhone's Dynamic Island for unified session visibility
- **No standalone rust**: The watch is RAM/battery constrained; all heavy logic (rule engine, event sourcing) runs on iPhone and syncs via WatchConnectivity

---

## Architecture

### Connectivity Model

**WatchConnectivity WCSession** is the backbone:
1. iPhone (main app) is the source of truth for all persistent state
2. Watch maintains a local cache of essential state (current focus session, credits, today's streak)
3. On-demand sync and background sync keep the cache fresh
4. Complication timeline provider queries the cache for display updates

**No on-device data persistence** beyond what WatchKit provides (UserDefaults for cache, HealthKit for optional step integration). The watch never stores rule logic, task definitions, or full event logs.

### Data Flow

```
iPhone App
  ├─ Event Sourcing Core (Rust)
  ├─ Rule Engine (Rust)
  ├─ Wallet Mutations (Rust)
  └─ CloudKit Sync (primary) / Self-hosted (future)
       ↓ WatchConnectivity WCSession
  Watch Cache (UserDefaults)
       ├─ Current Focus State (session ID, elapsed, target duration)
       ├─ Credits Balance
       ├─ Today's Summary (sessions completed, streak)
       └─ Complication Data (minimal—just what face/complications need)
```

### WatchConnectivity Protocol

**Context payloads** exchanged via `sendMessage:replyHandler:` and `transferUserInfo:`:

1. **Focus Session Update** (when iPhone starts/stops a session):
   ```json
   {
     "type": "focus-session",
     "sessionId": "uuid",
     "state": "active|paused|completed",
     "elapsedSeconds": 120,
     "targetSeconds": 1800,
     "timestamp": 1713921600
   }
   ```

2. **Credits Balance Update** (on mutation or periodic sync):
   ```json
   {
     "type": "credits",
     "balance": 250.5,
     "lastUpdated": 1713921600
   }
   ```

3. **Today Summary** (daily snapshot):
   ```json
   {
     "type": "daily-summary",
     "sessionsCompleted": 5,
     "totalMinutes": 180,
     "streak": 12,
     "date": "2026-04-23"
   }
   ```

4. **Coachy Nudge** (haptic-driven):
   ```json
   {
     "type": "nudge",
     "message": "Time for a break!",
     "intensity": "light|medium|heavy",
     "hapticPattern": "peek|pop|nope"
   }
   ```

---

## WatchOS UX Design

### Complication Families (Watch Face Integration)

FocalPoint exposes **4 complication families** to maximize watch face real estate:

#### 1. Circular Complication (Small)
- **Center display**: Credits balance (e.g., "250 Cr")
- **Ring progress**: Today's session count (e.g., 5/8 sessions)
- **Tap action**: Launch app to start focus

#### 2. Corner Rectangular Complication (Variable)
- **Primary line**: Current session timer (HH:MM or "Not active")
- **Secondary line**: Credits balance
- **Gauge**: Focus session progress (0–100%)
- **Tap action**: Open Focus scene

#### 3. Extra Large Rectangular (watchOS 10+)
- **Large timer display**: Session time (minutes)
- **Credits sub-label**: Balance
- **Session status indicator**: Active/paused/idle
- **Supports tap + swipe gestures**

#### 4. Inline Complication
- **Text**: "Focus: 25m | Cr: 250" (auto-truncated for width)
- **Tap action**: Launch app

### Main App Scenes

#### Scene 1: Focus Control (Primary)
```
┌─────────────────────────────┐
│       FocalPoint            │  ← Header with complication sync status
├─────────────────────────────┤
│                             │
│         25:43               │  ← Large session timer (DynamicType-responsive)
│                             │
├─────────────────────────────┤
│   SESSION ACTIVE            │  ← State badge
├─────────────────────────────┤
│  [PAUSE]     [END SESSION]  │  ← Digital Crown scrolls to reveal more buttons
│  [SKIP BREAK]               │  ← Secondary actions
├─────────────────────────────┤
│ Credits: 250.5 Cr           │  ← Always visible
│ Today: 5/8 sessions         │  ← Progress toward daily goal
└─────────────────────────────┘
```

**Digital Crown Interactions:**
- Scroll up: Increase session duration (1 min per notch, up to 120 min)
- Scroll down: Decrease session duration (1 min per notch, minimum 5 min)
- Press: Start/resume focus session

#### Scene 2: Credits Glance
```
┌─────────────────────────────┐
│         CREDITS             │
├─────────────────────────────┤
│                             │
│      250.5 Cr               │  ← Large balance display
│                             │
├─────────────────────────────┤
│   +10.5 Cr (last session)   │  ← Most recent credit update
│   Redeemed: 2 perks         │  ← Summary of session usage
└─────────────────────────────┘
```

#### Scene 3: Today Summary
```
┌─────────────────────────────┐
│         TODAY               │
├─────────────────────────────┤
│  5 / 8 Sessions Completed   │  ← Progress ring
│  180 minutes focused        │
│  Streak: 12 days            │  ← Gamification element
├─────────────────────────────┤
│  Recent:                    │
│  • 3:15 PM — 30m focus      │  ← List (scrollable)
│  • 1:45 PM — 25m focus      │
│  • 10:00 AM — 45m focus     │
└─────────────────────────────┘
```

### Haptic Coachy Nudges

When the iPhone rules engine decides to send a nudge (e.g., "Time for a break!"), it delivers a haptic payload via WCSession:
- **Light nudge** (gentle reminder): `peek` pattern (short-short)
- **Medium nudge** (escalating prompt): `pop` pattern (thump)
- **Heavy nudge** (urgent break needed): `nope` pattern (rejection rhythm)

The watch displays the nudge text in a **transient notification banner** (2s) paired with the haptic feedback. No persistent alert.

---

## Implementation Phases

### Phase 1: MVP (6–8 tool-call batches)
- Minimal **Complication (circular)** showing credits balance
- **Focus scene** with start/stop/pause buttons
- **WatchConnectivity sync** for focus session state only
- **Digital Crown session-length adjustment** (5–60 min)
- No Coachy nudges yet; sync credits balance on app launch

**Effort**: ~6–8 tool-call batches
- SwiftUI scene layouts: 2 batches
- WCSession delegate + sync protocol: 2 batches
- Complication timeline provider: 1 batch
- Digital Crown gesture handling: 1 batch
- Integration testing: 1-2 batches

### Phase 2: Enhanced MVP (4–6 tool-call batches)
- **Extra Large + Corner Rectangular complications** with progress rings
- **Today Summary scene** with scrollable session list
- **Persistent sync** (background + on-demand)
- **Credits balance real-time updates**
- **Live Activity integration** (mirror iPhone session to watch)

**Effort**: ~4–6 tool-call batches

### Phase 3: Coachy Integration (2–4 tool-call batches)
- **Haptic nudge delivery** (iPhone → Watch via WCSession)
- **Nudge notification UI** (transient banner + haptic pattern)
- **Rule-driven nudge triggers** (e.g., "break time" from iPhone engine)

**Effort**: ~2–4 tool-call batches

### Phase 4: Deferred (Out of Scope v1.0)
- Full task list on watch (too cluttered; use iPhone instead)
- Voice-to-Coachy (requires on-device NLP; defer to watchOS 11)
- Standalone cellular mode (not required for WiFi-connected watches)
- HealthKit integration (step counting for gamification; future)
- Siri shortcuts (nice-to-have; non-blocking)

---

## Technical Decisions

### Why Not Rust on Watch?

The Rust event sourcing and rule engine are too heavy for watchOS:
- **Minimal RAM** (~512 MB available for app)
- **High CPU cost** of just-in-time compilation on older watch hardware
- **Battery drain** from spinning up async runtimes
- **App size bloat** (Rust binary is 30–50 MB; watch app budget is ~100 MB)

**Solution**: iPhone is the Rust runtime. Watch mirrors state via WatchConnectivity. This aligns with Apple's design philosophy (watch as glanceable extension, not independent compute).

### WCSession Over CloudKit on Watch

- **WCSession** is optimized for small, frequent updates between paired devices (10–100 KB payloads)
- **CloudKit** on watch is slower (requires network round-trip) and drains battery
- **Trade-off**: WCSession requires iPhone to be reachable (WiFi or BLE); if iPhone is offline, watch shows stale data (acceptable for focus sessions)

### No SQLite on Watch

Watch app uses only **UserDefaults** (cache) and **HealthKit** (optional):
- No event logs, no full task list, no mutation history
- Cache is garbage-collected after 30 days of inactivity (watchOS default)
- Event sourcing and audit trails remain on iPhone/backend

---

## Data Structures (Swift)

```swift
// Focus Session (cached on watch)
struct FocusSessionSnapshot: Codable {
    let sessionId: UUID
    let state: FocusState // active, paused, completed
    let elapsedSeconds: Int
    let targetSeconds: Int
    let startTime: Date
    let ruleId: UUID? // Originating rule
}

enum FocusState: String, Codable {
    case active, paused, completed, idle
}

// Credits Snapshot
struct CreditsSnapshot: Codable {
    let balance: Double
    let lastUpdated: Date
    let lastMutation: String? // "redeemed 2 perks", "+10.5 focus"
}

// Daily Summary
struct DailySummary: Codable {
    let date: Date
    let sessionsCompleted: Int
    let totalMinutes: Int
    let streak: Int // consecutive focus days
}

// Nudge Payload
struct CaochyNudge: Codable {
    let id: UUID
    let message: String
    let intensity: NudgeIntensity // light, medium, heavy
    let hapticPattern: HapticPattern
    let deliveredAt: Date
}

enum NudgeIntensity: String, Codable {
    case light, medium, heavy
}

enum HapticPattern: String, Codable {
    case peek, pop, nope // WKHapticType mapping
}
```

---

## Testing & QA

1. **Simulator testing**: Xcode Watch simulator with iPhone simulator (WCSession loopback works)
2. **Device pairing**: Pair real watch + iPhone; verify sync latency <1s
3. **Offline simulation**: Unpair iPhone; confirm watch displays stale data gracefully
4. **Complication timeline**: Use ClockKit simulator to verify updates every 15 min
5. **Haptic patterns**: Test all three patterns (peek/pop/nope) on device for UX feedback

---

## Success Metrics

- **Complication tap-through rate**: >30% of watch users tap complication daily
- **Focus session time**: Sessions started on watch account for >10% of total FocalPoint focus time
- **Session completion rate**: >85% of watch-started sessions complete (vs. 75% on iPhone)
- **Haptic nudge engagement**: >60% of delivered nudges trigger a user action (pause/extend/end)
- **Watch app crash rate**: <0.5% (watchOS 10+)

---

## Future Roadmap

1. **HealthKit integration** (watchOS 11): Correlate focus sessions with activity rings, sleep, heart rate
2. **Siri Intents**: "Hey Siri, start a 30-minute focus" (requires intent handling in host app)
3. **Cellular mode** (Series 7+): Sync via LTE if WiFi unavailable (low priority)
4. **Wrist-raise actions**: Quick-launch complication on wrist raise (low value; users have Lock Screen)
5. **Multi-watch support**: Sync across paired watches (edge case; skip for v1.0)

---

## Summary

The Apple Watch companion transforms FocalPoint into an always-accessible focus tool. By leveraging WatchConnectivity for state sync and dismissing heavy compute (Rust, event sourcing), we keep the watch app lean, responsive, and battery-efficient. The MVP (complication + focus control) ships in 6–8 batches; full Coachy integration follows in Phase 3.
