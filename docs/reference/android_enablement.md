# Android Enablement Checklist

**Status:** v0.0.1 scaffold (Phase 2 deferred).

This guide gets you to a buildable Android project in ~30 minutes.

---

## Prerequisites

### 1. Android NDK

The Rust FFI bindings compile for Android via the NDK cross-compiler toolchain.

#### Installation (macOS)

1. **Open Android Studio:**
   ```bash
   open -a "Android Studio"
   ```

2. **Navigate:** Top menu → Android Studio → Settings → Appearance & Behavior → System Settings → Android SDK.

3. **Install NDK:**
   - Click **SDK Tools** tab.
   - Check **NDK (Side by side)** → click Install.
   - Wait ~2 min for download + extraction.

4. **Find NDK path:**
   ```bash
   ls ~/Library/Android/sdk/ndk/
   # Shows: 27.0.12077973 (or newer)
   ```

5. **Set environment variable** (add to `~/.zshrc` or `~/.bashrc`):
   ```bash
   export ANDROID_NDK_HOME="$HOME/Library/Android/sdk/ndk/27.0.12077973"
   ```

6. **Reload shell:**
   ```bash
   source ~/.zshrc  # or ~/.bashrc
   echo $ANDROID_NDK_HOME  # verify
   ```

#### Installation (Linux)

```bash
# Ubuntu/Debian
sudo apt-get install android-sdk-ndk

# Arch
yay -S android-ndk

# Or manual: download from https://developer.android.com/ndk/downloads
# Extract and export ANDROID_NDK_HOME=/path/to/ndk/<version>
```

### 2. Rust Android Targets

Install cross-compilation targets:

```bash
rustup target add aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```

Verify:
```bash
rustup target list | grep android
# Should show all 4 targets with "(installed)"
```

### 3. Gradle & Android Studio

Gradle comes bundled with the project (`./gradlew` wrapper). No separate install needed.

**Recommended:** Open the project in Android Studio for best IDE support:

```bash
cd /Users/you/CodeProjects/Phenotype/repos/FocalPoint
open -a "Android Studio" apps/android
```

---

## First Build: Step by Step

### Step 1: Generate Kotlin FFI Bindings

From the FocalPoint workspace root:

```bash
cd /Users/you/CodeProjects/Phenotype/repos/FocalPoint

# Validate NDK is set
echo $ANDROID_NDK_HOME
# Should print: /Users/you/Library/Android/sdk/ndk/27.0.12077973 (or newer)

# Generate bindings (compiles Rust for 4 ABIs + generates Kotlin code)
cargo run --release -p focus-ffi --bin android_bindings

# Expected output:
# 🔨 FocalPoint Android Bindings Generator
# ✓ Android NDK found at: ...
# ✓ Workspace root: ...
# 🔨 Building focus-ffi for Android targets...
#   → aarch64-linux-android ... ✓
#   → armv7-linux-androideabi ... ✓
#   → x86_64-linux-android ... ✓
#   → i686-linux-android ... ✓
# 🔨 Generating Kotlin bindings...
# ✓ Bindings generated at: ...
# 🔨 Packing .so libraries into jniLibs...
#   → aarch64-linux-android (arm64-v8a) ... ✓
#   → armv7-linux-androideabi (armeabi-v7a) ... ✓
#   → x86_64-linux-android (x86_64) ... ✓
#   → i686-linux-android (x86) ... ✓
# ✅ Android bindings generated successfully!
```

**If it fails:**

```
error: ANDROID_NDK_HOME not set
```

→ Go back to **Prerequisites** § 1, step 5. Make sure the export is in your shell config and you've reloaded.

```
error: libfocus_ffi.so not found at ...
```

→ `cargo build` failed for one of the ABIs. Check the error message above. Likely: missing Rust target or NDK toolchain. Run `rustup target add ...` from **Prerequisites** § 2.

### Step 2: Verify Bindings Were Generated

```bash
ls apps/android/app/src/main/kotlin/com/focalpoint/ffi/
# Should show: FocalPointCore.kt (+ uniffi generated .kt files)

ls apps/android/app/src/main/jniLibs/*/
# Should show: arm64-v8a/libfocus_ffi.so, armeabi-v7a/libfocus_ffi.so, etc.
```

### Step 3: Build the Android App

Navigate to the Android app directory:

```bash
cd apps/android
```

Build via Gradle:

```bash
# First build (downloads Gradle + dependencies, ~3 min)
./gradlew build

# Or if you prefer to use Android Studio:
open -a "Android Studio" .
# Gradle syncs automatically; run → Build → Build Bundles/APKs → Build APK
```

**Expected:** No build errors. Output:

```
BUILD SUCCESSFUL in 3m 24s
```

### Step 4: Run on Emulator or Device

#### Option A: Android Studio UI (easiest)

1. Open `apps/android` in Android Studio (see Step 3).
2. Top menu → **Run** → **Run 'app'**.
3. Select target:
   - **Create new emulator** if none exist (Device Manager → + icon).
   - Or connect physical Android 14+ device via USB (enable Developer Mode first).
4. Click **Run**.

#### Option B: Command Line

```bash
# List available devices/emulators
adb devices

# If empty: launch emulator first
emulator -avd Pixel_7_API_34

# Then:
./gradlew installDebug
adb shell am start -n com.focalpoint/.MainActivity
```

#### Option C: Build APK

```bash
./gradlew assembleDebug

# APK at: app/build/outputs/apk/debug/app-debug.apk
adb install app/build/outputs/apk/debug/app-debug.apk
```

### Step 5: Verify App Runs

When the app launches, you should see:

- **7 tabs** at the bottom: Tasks, Focus, Today, Rules, Wallet, Activity, Settings.
- **Placeholder banner** on all screens: "🚧 Developer Placeholder — This is a scaffold."
- **Wallet tab shows credits balance** (fetched from Rust core via FFI).
- **No crashes** on startup or navigation.

---

## Troubleshooting

### "ANDROID_NDK_HOME not set"

```bash
# 1. Check if it's really set
echo $ANDROID_NDK_HOME

# 2. If empty, add to ~/.zshrc:
echo 'export ANDROID_NDK_HOME="$HOME/Library/Android/sdk/ndk/27.0.12077973"' >> ~/.zshrc

# 3. Reload
source ~/.zshrc

# 4. Verify
echo $ANDROID_NDK_HOME
# Should print a path, not be empty
```

### Build fails with "target not found"

```bash
# Install all Rust targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# Verify
rustup target list | grep android
```

### Gradle build fails: "Cannot find Android SDK"

```bash
# Check ANDROID_HOME (different from ANDROID_NDK_HOME)
echo $ANDROID_HOME

# If unset, add to ~/.zshrc:
echo 'export ANDROID_HOME="$HOME/Library/Android/sdk"' >> ~/.zshrc

source ~/.zshrc
```

### App crashes on startup

If you see:

```
E/AndroidRuntime: java.lang.UnsatisfiedLinkError: dlopen failed: library "libfocus_ffi.so" not found
```

→ Bindings weren't generated. Run `cargo run --release -p focus-ffi --bin android_bindings` from the workspace root.

### Emulator won't launch

```bash
# List available emulators
emulator -list-avds

# If empty, create one:
# Android Studio → Device Manager → + → Download image → Create device

# Launch:
emulator -avd Pixel_7_API_34
```

---

## Architecture Notes

### What Was Generated

- **`FocalPointCore.kt`** — UniFFI Kotlin bindings (auto-generated). This is the interface to the Rust core.
- **`.so libraries`** — Native shared objects for each ABI (ARM64, ARM32, x86_64, x86). Located in `jniLibs/`.
- **`MainActivity.kt`** — Entrypoint; initializes `FocalPointCore` and displays the tab UI.

### What You're NOT Doing Yet

- ❌ Connectors (Canvas OAuth, etc.) — Phase 2+.
- ❌ App enforcement (blocking) — Phase 2+.
- ❌ Background sync — Phase 2+.
- ❌ Rules configuration UI — Phase 3+.

This is a **read-only showcase** of the Rust core via Kotlin FFI.

---

## Next Steps (Phase 2+)

1. **Implement tab screens** (currently stubs with placeholder banner).
2. **Add Canvas connector OAuth** (Custom Tabs flow).
3. **Implement enforcement driver** (AccessibilityService + overlay).
4. **Add background sync** (WorkManager).
5. **Rule creation UI** (interactive rule builder in Compose).

---

## References

- **Android Bindings Generator:** `crates/focus-ffi/src/bin/android_bindings.rs`
- **Architecture Design:** `docs/architecture/android_port_2026_04.md`
- **Rust Core:** `crates/focus-ffi/` + all `crates/focus-*` packages
- **Official Android Docs:** https://developer.android.com/docs
- **Gradle Documentation:** https://docs.gradle.org/current/userguide/kotlin_dsl.html
- **UniFFI Guide:** https://mozilla.github.io/uniffi-rs/

