# FocalPoint iOS

Stub. SwiftUI + FamilyControls/DeviceActivity/ManagedSettings/CoreNFC.

**Status:** directory placeholder. Xcode project + SwiftPM manifest
pending Phase 1 (see `../../PLAN.md`).

## Pending scaffold

- `FocalPoint.xcodeproj` or `Package.swift`
- `Sources/FocalPointApp/` — SwiftUI root
- `Sources/IosEnforcementDriver/` — FamilyControls + ManagedSettings adapter
- `Sources/IosNfcUnlockAdapter/` — CoreNFC reader (optional per ADR-009)
- `Sources/IosSecureStoreAdapter/` — Keychain wrapper
- `Sources/IosNotificationAdapter/` — UNUserNotificationCenter wrapper
- `Sources/FocalPointCore/` — UniFFI-generated Swift bindings (from `crates/focus-ffi`)

## Entitlements required (blocker)

- `com.apple.developer.family-controls` — submit application before Phase 1
  iOS work begins. Apple review historically 1–4 weeks.

## Donor candidates

- **Foqos** — screen-time/NFC reference; license + URL TBD (see `docs/research/open_questions.md` Q5)
