// app/build.gradle.kts for FocalPoint Android
// See: docs/architecture/android_port_2026_04.md

plugins {
    id("com.android.application")
    kotlin("android")
    kotlin("serialization")
}

android {
    namespace = "com.focalpoint"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.focalpoint"
        minSdk = 29
        targetSdk = 34
        versionCode = 1
        versionName = "0.0.1-alpha"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        compose = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.8"
    }

    packagingOptions {
        resources {
            excludes.add("META-INF/proguard/androidx-*.pro")
            excludes.add("META-INF/LICENSE.md")
            excludes.add("META-INF/NOTICE.md")
        }
    }
}

dependencies {
    // Jetpack Compose (Material 3)
    implementation("androidx.compose.ui:ui:1.6.0")
    implementation("androidx.compose.ui:ui-graphics:1.6.0")
    implementation("androidx.compose.material3:material3:1.1.2")
    implementation("androidx.compose.material:material-icons-extended:1.6.0")
    implementation("androidx.compose.runtime:runtime:1.6.0")
    implementation("androidx.compose.foundation:foundation:1.6.0")
    implementation("androidx.compose.foundation:foundation-layout:1.6.0")

    // Compose Debugging
    debugImplementation("androidx.compose.ui:ui-tooling:1.6.0")
    implementation("androidx.compose.ui:ui-tooling-preview:1.6.0")

    // Lifecycle & State Management
    implementation("androidx.lifecycle:lifecycle-runtime-compose:2.7.0")
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.7.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.7.0")

    // Core Android
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("androidx.activity:activity-compose:1.8.1")
    implementation("androidx.core:core:1.13.0")
    implementation("androidx.core:core-splashscreen:1.0.1")

    // JSON & Serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.2")

    // Coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")

    // Lottie (Mascot animations — includes compose-lottie for animations)
    implementation("com.airbnb.android:lottie:6.1.0")
    implementation("com.airbnb.android:lottie-compose:6.5.2")

    // Custom Tabs (OAuth)
    implementation("androidx.browser:browser:1.7.0")

    // WorkManager (background sync)
    implementation("androidx.work:work-runtime-ktx:2.8.1")

    // Accessibility
    implementation("androidx.accessibility:accessibility:1.7.0")

    // Data Store (for app preferences)
    implementation("androidx.datastore:datastore-preferences:1.0.0")

    // Testing
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.compose.ui:ui-test-junit4:1.6.0")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
}
