# Foqos FamilyControls White-Box Graft

## Overview

This directory contains Swift code grafted from the **Foqos** iOS screen-time app (https://github.com/awaseem/foqos) under the MIT license. FocalPoint adopts Foqos's battle-tested FamilyControls integration patterns while replacing Foqos's storage layer (SwiftData) with FocalPoint's Rust core + SQLite audit chain.

## Source Attribution

| Source | Commit | License | Permission |
|--------|--------|---------|-----------|
| **awaseem/foqos** | `5cb17ffcf4463c722d96a103737655f7ae07d01f` | MIT | ✓ White-box borrow + attribution |

**Source URL:** https://github.com/awaseem/foqos  
**Commit Date:** April 20, 2026 (v1.32.1)  
**Access Date:** April 23, 2026

## Files Lifted (Direct Copy + Attribution)

| File | Source Path | Adapted For | SPDX |
|------|------------|------------|------|
| `DeviceActivityMonitorExtension.swift` | `FoqosDeviceMonitor/DeviceActivityMonitorExtension.swift` | FocalPoint extension scaffold | MIT |
| `FoqosDeviceMonitor.entitlements` | `FoqosDeviceMonitor/FoqosDeviceMonitor.entitlements` | Reference (not directly used; for signature) | MIT |

**Lines of Code Lifted:** ~30 LOC (DeviceActivityMonitorExtension.swift)

**Attribution Block:** Every lifted file includes:
```swift
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 awaseem and Foqos contributors
// Adapted for FocalPoint by the FocalPoint team
// Source: https://github.com/awaseem/foqos (commit 5cb17ffcf...)
```

## Files Inspired-Only (Not Direct Copies)

| Pattern | Source | Inspiration Use | Status |
|---------|--------|-----------------|--------|
| **FamilyActivitySelection** integration | `Foqos/Models/BlockedProfiles.swift` | How to route selectedActivity → FamilyControls tokens | Code-reviewed, reimplemented |
| **ManagedSettingsStore** + **DeviceActivityName** setup | Foqos enforcement patterns | Namespace isolation (`app.focalpoint.shield` vs. Foqos's `group.dev.ambitionsoftware.foqos`) | Reimplemented in `Enforcement.swift` |
| **DeviceActivityCenter monitor lifecycle** | Foqos extension patterns | When to start/stop monitoring; intervalDidStart/intervalDidEnd hooks | Documented in code comments |

**Lines of Code Inspired-Only:** ~0 (patterns learned, not copied)

## Integration Path

### 1. **DeviceActivityMonitor Extension**
   - **File:** `FoqosDeviceMonitorExtension.swift`
   - **Role:** Monitors DeviceActivity.Monitor lifecycle events (intervalDidStart/intervalDidEnd)
   - **Routes To:** FocalPoint's Rust core via FFI (`FocalPointCore` module)
   - **Dependencies Removed:** `AppBlockerUtil` + `TimerActivityUtil` (Foqos-specific; replaced with FocalPoint Rust calls)

### 2. **FamilyControls Entitlements**
   - **File:** Reference only (actual entitlements in `.entitlements` files managed by Xcode)
   - **Role:** Shows required `com.apple.developer.family-controls` key for app signing
   - **Note:** FocalPoint's main app + extension bundles must include this key

### 3. **Rust Core Integration**
   - Device activity events flow from extension → Rust FFI → enforcement policy evaluation
   - Shield lift/engage determined by Rust rule engine (not Swift logic)
   - Audit trail appended on every state change (immutable SQLite chain)

## Build Behavior

### With `FOCALPOINT_HAS_FAMILYCONTROLS = 0` (Default)
- Extension code compiles but enforcement is no-op (log only)
- Safe for simulator + CI (no entitlement needed)
- Allows feature-development without waiting for Apple approval

### With `FOCALPOINT_HAS_FAMILYCONTROLS = 1` (After Entitlement)
- Full enforcement enabled
- Requires signed app + Apple-approved entitlement on real device
- DeviceActivityMonitorExtension hooks are live

## Test Plan

See `docs/reference/family_controls_enablement.md` for full procedure:

1. **Submit entitlement app** to Apple (Phase 0 blocker)
2. **Flip `FOCALPOINT_HAS_FAMILYCONTROLS = 1`** in Xcode project config
3. **Run on real device** with signed certificate
4. **User flow:** Settings → FamilyControls → Authorize → Pick apps via FamilyActivityPicker
5. **Expect:** Shield engages when rule-scheduled interval starts; lifts when interval ends

## Full License Text

See `docs/legal/third_party_attributions.md` for complete MIT license text and attribution.

## TODOs (Future Phases)

- [ ] Phase 2: Lock Screen widget integration (Foqos's WidgetKit patterns)
- [ ] Phase 3: Strategy enum orchestration (Foqos's NFC/QR/timer unlock patterns)
- [ ] Phase 4: Deep-link profile activation (Foqos's URL scheme pattern)

---

**Last Updated:** 2026-04-23  
**FocalPoint Commit:** `feat(enforcement): white-box graft Foqos FamilyControls scaffolds (MIT)`
