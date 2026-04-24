// MainActivity — FocalPoint Android entry point
//
// This is a scaffold: demonstrates FFI loading + placeholder UI.
// Real tab implementations (connectors, enforcement, etc.) are deferred to Phase 2+.

package com.focalpoint

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import com.focalpoint.core.CoreHolder
import com.focalpoint.ui.FocalPointAppRoot

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        // Initialize Rust FFI core. This reads/writes to device storage.
        val coreHolder = CoreHolder.getInstance(this)

        setContent {
            val selectedTab = remember { mutableStateOf(0) }

            MaterialTheme(
                colorScheme = darkColorScheme()
            ) {
                FocalPointAppRoot(
                    selectedTab = selectedTab.value,
                    onTabSelected = { selectedTab.value = it },
                    coreHolder = coreHolder
                )
            }
        }
    }
}

// Material 3 dark color scheme
private fun darkColorScheme() = androidx.compose.material3.darkColorScheme(
    primary = androidx.compose.ui.graphics.Color(0xFF6366F1),      // Indigo
    secondary = androidx.compose.ui.graphics.Color(0xFF8B5CF6),    // Purple
    tertiary = androidx.compose.ui.graphics.Color(0xFFF59E0B)      // Amber
)
