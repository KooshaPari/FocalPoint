// Root build.gradle.kts for FocalPoint Android project
// See: docs/architecture/android_port_2026_04.md

plugins {
    id("com.android.application") version "8.2.0" apply false
    kotlin("android") version "1.9.23" apply false
    kotlin("serialization") version "1.9.23" apply false
}

tasks.register("clean", Delete::class) {
    delete(rootProject.buildDir)
}

// Helper task to run the Android bindings generator
tasks.register("generateAndroidBindings") {
    description = "Generate UniFFI Kotlin bindings for Android. Requires ANDROID_NDK_HOME set."
    doFirst {
        val ndkHome = System.getenv("ANDROID_NDK_HOME")
        if (ndkHome.isNullOrEmpty()) {
            throw GradleException("""
                ANDROID_NDK_HOME not set. Install Android NDK:
                1. Open Android Studio → SDK Manager → SDK Tools
                2. Check "NDK (Side by side)" → install latest
                3. Set: export ANDROID_NDK_HOME=/Users/you/Library/Android/sdk/ndk/<version>

                See docs/reference/android_enablement.md for details.
            """.trimIndent())
        }
    }

    // Invoke the Rust binary
    exec {
        workingDir(rootProject.rootDir.parent) // workspace root
        commandLine("cargo", "run", "--release", "-p", "focus-ffi", "--bin", "android_bindings")
    }
}
