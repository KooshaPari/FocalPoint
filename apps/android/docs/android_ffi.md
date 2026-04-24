# Android FFI Integration Guide

## Overview

FocalPoint's Rust core is exposed to Android via **UniFFI Kotlin bindings** and **JNI**. The build system automatically:

1. Compiles `crates/focus-ffi` for all Android ABIs (`arm64-v8a`, `armeabi-v7a`, `x86_64`, `x86`)
2. Generates Kotlin stubs into `app/src/main/kotlin/com/focalpoint/ffi/`
3. Packages `.so` libraries into `app/src/main/jniLibs/<abi>/libfocus_ffi.so`

No manual JNI code needed—UniFFI handles the C ABI and Kotlin marshalling.

## Setup

### Prerequisites

```bash
# Android SDK with NDK (r26+)
export ANDROID_NDK_HOME="$HOME/Library/Android/sdk/ndk/26.0.10810419"

# Rust toolchain with Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# Verify
cargo --version  # 1.81+
$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/clang --version
```

### Environment

Add to your shell profile (`~/.zshrc` or `~/.bash_profile`):

```bash
export ANDROID_NDK_HOME="$HOME/Library/Android/sdk/ndk/26.0.10810419"
export CARGO_NDK_ANDROID_API_LEVEL=29  # minSdk in build.gradle
```

Verify:
```bash
echo $ANDROID_NDK_HOME
```

## Build Flow

### Full Build (Gradle)

```bash
cd apps/android
./gradlew build
```

Steps:
1. Gradle calls `generateUniffiBindings` task (defined in `app/build.gradle.kts`)
2. Task executes `cargo run --release -p focus-ffi --bin android_bindings`
3. `android_bindings.rs` orchestrates:
   - Cargo build for 4 ABIs (release mode)
   - `cargo run uniffi-bindgen` to generate Kotlin stubs
   - Copy `.so` files to `jniLibs/<abi>/`
4. Gradle compiles Kotlin + links native libraries

### Incremental Build (Cargo Only)

To rebuild Rust without full Gradle:

```bash
cd /path/to/FocalPoint
cargo build --release -p focus-ffi --target aarch64-linux-android
cargo build --release -p focus-ffi --target armv7-linux-androideabi
cargo build --release -p focus-ffi --target x86_64-linux-android
cargo build --release -p focus-ffi --target i686-linux-android
```

Then regenerate bindings:

```bash
cargo run --release -p focus-ffi --bin android_bindings
```

## Generated Bindings

After a successful build, Kotlin stubs are at:

```
apps/android/app/src/main/kotlin/com/focalpoint/ffi/
  └── FocalPointCore.kt       # Main interface
  ├── *.kt                    # Auto-generated DTO + error types
```

### Example Usage in Kotlin

```kotlin
import com.focalpoint.ffi.FocalPointCore

val storagePath = context.filesDir.absolutePath
val core = FocalPointCore(storagePath)

// Wallet
val wallet = core.wallet().load()
println("Credits available: ${wallet.balance}")

// Tasks
val tasks = core.tasks().list()
tasks.forEach { task ->
    println("Task: ${task.title}")
}

// Audit
val records = core.audit().recent(10U)

// Host events (for timer start/stop, etc.)
val event = com.focalpoint.ffi.HostEventDto(
    event_type = "session_started",
    confidence = 1.0f,
    payload_json = "{\"duration_minutes\": 25}",
    dedupe_key = null
)
core.host_events().emit(event)
```

## Exported Functions

The UDL surface (`crates/focus-ffi/src/focus_ffi.udl`) exports:

| Class/Interface | Method | Notes |
|---|---|---|
| **FocalPointCore** | `constructor(storage_path)` | Init with SQLite path |
| | `wallet()` | Returns `WalletApi` |
| | `tasks()` | Returns `TaskApi` |
| | `audit()` | Returns `AuditApi` |
| | `rules()` | Returns `RuleQuery` |
| | `mutations()` | Returns `RuleMutation` |
| | `penalty()` | Returns `PenaltyApi` |
| | `policy()` | Returns `PolicyApi` |
| | `eval()` | Returns `EvalApi` |
| | `sync()` | Returns `SyncApi` |
| | `rituals()` | Returns `RitualsApi` |
| | `connector()` | Returns `ConnectorApi` |
| | `host_events()` | Returns `HostEventApi` |
| | `backup()` | Returns `BackupApi` |
| | `mascot_state()` | Returns mascot pose + emotion |
| **WalletApi** | `load()` | `→ WalletSummary` |
| | `apply_mutation(m)` | Apply reward/penalty |
| **TaskApi** | `list()` | `→ List<TaskSummaryDto>` |
| | `add(input)` | `→ String` (task ID) |
| | `remove(task_id)` | Delete task |
| | `mark_done(task_id)` | Mark complete |
| **AuditApi** | `verify_chain()` | Tamper verification |
| | `head_hash()` | Chain head |
| | `recent(limit)` | `→ List<AuditRecordDto>` |
| **HostEventApi** | `emit(dto)` | Inject synthetic events |

## Integration in CoreHolder

`app/src/main/kotlin/com/focalpoint/core/CoreHolder.kt` wraps the FFI:

```kotlin
class CoreHolder(context: Context) {
    private lazy val core = FocalPointCore(storagePath)

    suspend fun getWalletBalance(): WalletState {
        val wallet = core.wallet().load()
        return WalletState(
            creditsAvailable = wallet.balance,
            totalEarned = wallet.earned,
            totalSpent = wallet.spent
        )
    }

    suspend fun getTaskList(): List<TaskState> {
        return core.tasks().list().map { dto -> ... }
    }

    suspend fun emitHostEvent(eventType: String, payload: String) {
        val event = HostEventDto(event_type, 1.0f, payload, null)
        core.host_events().emit(event)
    }
}
```

All methods run on `Dispatchers.IO` to avoid main-thread blocking.

## Debugging

### Verify .so Presence

```bash
find apps/android/app/src/main/jniLibs -name "*.so"
```

Output should show:
```
apps/android/app/src/main/jniLibs/arm64-v8a/libfocus_ffi.so
apps/android/app/src/main/jniLibs/armeabi-v7a/libfocus_ffi.so
apps/android/app/src/main/jniLibs/x86_64/libfocus_ffi.so
```

### Trace Bindings Generation

```bash
cd apps/android
./gradlew generateUniffiBindings -i
```

Flag `-i` shows INFO-level logs including Cargo output.

### Check Kotlin Stubs

```bash
ls apps/android/app/src/main/kotlin/com/focalpoint/ffi/
```

Should contain `FocalPointCore.kt`, `WalletSummary.kt`, `TaskSummaryDto.kt`, etc.

### Runtime Error: libfocus_ffi.so Not Found

```
java.lang.UnsatisfiedLinkError: dalvik.system.PathClassLoader...libfocus_ffi.so
```

**Cause:** UniFFI bindings generation failed silently.

**Fix:**
1. Manually run: `cargo run -p focus-ffi --bin android_bindings`
2. Check for build errors (missing NDK, Rust target)
3. Verify `ANDROID_NDK_HOME` is set and points to valid NDK installation

### Runtime Error: FocalPointCore Constructor Fails

```
FfiException("storage: database open failed")
```

**Cause:** Invalid storage path or no write permission.

**Fix:**
- Verify `context.filesDir.absolutePath` is writable
- Check app has `android.permission.READ_WRITE_STORAGE` (not needed—context.filesDir is app-private)
- Ensure SQLite file can be created (e.g., no parent dir missing)

## Cross-Compile Targets

| Target | ABI | Device Type | Emulator |
|---|---|---|---|
| `aarch64-linux-android` | `arm64-v8a` | Modern phones (primary) | x |
| `armv7-linux-androideabi` | `armeabi-v7a` | Older devices | x |
| `x86_64-linux-android` | `x86_64` | x | ✓ Most emulators |
| `i686-linux-android` | `x86` | Legacy | ✓ Legacy emulators |

The build system compiles all 4 by default. To target only one:

```bash
cargo build --release -p focus-ffi --target aarch64-linux-android
```

Then update `ANDROID_ABIS` in `crates/focus-ffi/src/bin/android_bindings.rs`.

## Performance Notes

- **First build:** ~60–120 sec (compiles Rust for 4 ABIs)
- **Incremental:** ~5–10 sec (Gradle only rebuilds changed Kotlin)
- **Release build:** Rust compiled with `--release` (LTO enabled in Cargo.toml)
- **Debug APK:** Still uses release `.so` (acceptable for dev; `.so` size ~8–12 MB per ABI)

## References

- UniFFI docs: https://mozilla.github.io/uniffi-rs/
- Android NDK: https://developer.android.com/ndk
- FocalPoint scaffold: `../../../CLAUDE.md`
