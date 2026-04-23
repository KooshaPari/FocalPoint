# FocalPoint Android

Stub. Kotlin + Jetpack Compose + UsageStats + AccessibilityService.

**Status:** directory placeholder. Gradle project pending Phase 2.

## Pending scaffold

- `build.gradle.kts` (root + `:app`)
- `app/src/main/kotlin/com/focalpoint/FocalPointApp.kt` — Compose root
- `app/src/main/kotlin/com/focalpoint/enforcement/AndroidEnforcementDriver.kt`
- `app/src/main/kotlin/com/focalpoint/accessibility/FocalPointAccessibilityService.kt`
- `app/src/main/kotlin/com/focalpoint/secure/AndroidSecureStoreAdapter.kt`
- `app/src/main/kotlin/com/focalpoint/ffi/FocalPointCore.kt` — JNI bindings (from `crates/focus-ffi`)

## Permissions required

- `PACKAGE_USAGE_STATS` — user-grant via settings deep link
- `BIND_ACCESSIBILITY_SERVICE` — user-grant; explain flow in onboarding
- Foreground service for enforcement daemon

## Donor candidates

- **Reef** — Android blocker/routines reference; license + URL TBD
