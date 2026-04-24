# FocalPoint Android

Kotlin + Jetpack Compose + Material 3 + UniFFI FFI bindings.

**Status:** v0.0.1 scaffold (2026-04-23). See `docs/architecture/android_port_2026_04.md` + `docs/reference/android_enablement.md`.

---

## Quick Start

1. **Install Android NDK** (see `docs/reference/android_enablement.md`):
   ```bash
   export ANDROID_NDK_HOME="$HOME/Library/Android/sdk/ndk/27.0.12077973"
   ```

2. **Generate Kotlin bindings** (from workspace root):
   ```bash
   cargo run --release -p focus-ffi --bin android_bindings
   ```

3. **Build & run:**
   ```bash
   cd apps/android
   ./gradlew build
   ./gradlew installDebug  # or open in Android Studio
   ```

4. **Expected:** App launches with 7 tabs (Tasks, Focus, Today, Rules, Wallet, Activity, Settings).
   - Placeholder banner on all screens.
   - Wallet tab shows credits balance (from Rust core).
   - No enforcement, connectors, or background sync yet (Phase 2+).

---

## Project Structure

```
apps/android/
├── build.gradle.kts              # Root gradle config
├── settings.gradle.kts
├── gradle.properties
├── app/
│   ├── build.gradle.kts          # App module config
│   ├── src/main/
│   │   ├── kotlin/com/focalpoint/
│   │   │   ├── MainActivity.kt
│   │   │   ├── ui/
│   │   │   │   ├── FocalPointApp.kt
│   │   │   │   ├── screens/Screens.kt     # 7 stub screens
│   │   │   │   └── components/PlaceholderBanner.kt
│   │   │   ├── core/CoreHolder.kt         # FFI access
│   │   │   ├── enforcement/               # Phase 2+
│   │   │   └── analytics/                 # Phase 2+
│   │   ├── AndroidManifest.xml
│   │   └── res/
│   │       └── values/
│   │           ├── strings.xml
│   │           ├── themes.xml
│   │           └── colors.xml
│   └── src/jniLibs/                       # Generated .so libs
└── README.md (this file)
```

---

## Architecture Decisions

- **FFI:** UniFFI Kotlin (same toolchain as iOS Swift bindings).
- **Storage:** Reuse Rust SQLite path (no Room in v1).
- **Enforcement:** Soft blocking (overlay + notification) in v1; hard blocking (DevicePolicyManager) deferred to v2+.
- **Build:** Gradle 8.2.0 Kotlin DSL; API 34 (Android 14+) target.
- **UI:** Jetpack Compose + Material 3.

---

## Phase 2+ Todos

- [ ] Implement tab screens (currently stubs).
- [ ] Canvas connector OAuth flow (Custom Tabs).
- [ ] AccessibilityService enforcement driver.
- [ ] WorkManager background sync.
- [ ] Rule creation UI.
- [ ] Audit log viewer.
- [ ] Settings: permissions, accounts, diagnostics.

---

## Documentation

- **Design Doc:** [`docs/architecture/android_port_2026_04.md`](../../docs/architecture/android_port_2026_04.md)
  - FFI strategy, architecture, unknowns, risks, gaps.
  
- **Enablement Guide:** [`docs/reference/android_enablement.md`](../../docs/reference/android_enablement.md)
  - NDK setup, first build, troubleshooting.

- **Rust Bindings Generator:** [`crates/focus-ffi/src/bin/android_bindings.rs`](../../crates/focus-ffi/src/bin/android_bindings.rs)
  - Validates NDK, compiles for 4 ABIs, runs uniffi-bindgen, packs .so libs.

---

## Permissions Required (Phase 2+)

- `PACKAGE_USAGE_STATS` — user-grant via settings deep link; query app foreground time.
- `BIND_ACCESSIBILITY_SERVICE` — user-grant; monitor app lifecycle.
- `SCHEDULE_EXACT_ALARM` — schedule enforcement checks.
- `INTERNET` — connector OAuth flows.

---

## Known Unknowns

See `docs/architecture/android_port_2026_04.md` § 8 for:
- App blocking UX (soft vs hard)
- AccessibilityService lifetime
- Wear OS Ongoing Activity support
- Material You dynamic color

---

## Versioning

- **Gradle:** 8.2.0
- **Kotlin:** 1.9.23
- **Compose:** 1.6.0
- **Material3:** 1.1.2
- **Android SDK:** compileSdk 34, minSdk 29, targetSdk 34
- **Rust:** MSRV 1.82 (same as workspace)
