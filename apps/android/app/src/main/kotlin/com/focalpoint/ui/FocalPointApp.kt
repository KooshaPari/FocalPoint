// FocalPointApp — Root composable for all tabs
//
// This is v0.0.1: all screens are stubs with placeholder banners.
// Real implementations deferred to Phase 2+.

package com.focalpoint.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import com.focalpoint.core.CoreHolder
import com.focalpoint.ui.screens.*

data class Tab(val label: String, val icon: ImageVector)

private val TABS = listOf(
    Tab("Tasks", Icons.Default.Task),
    Tab("Focus", Icons.Default.FavoriteBorder),
    Tab("Today", Icons.Default.Home),
    Tab("Rules", Icons.Default.Settings),
    Tab("Wallet", Icons.Default.AttachMoney),
    Tab("Activity", Icons.Default.History),
    Tab("Settings", Icons.Default.MoreVert)
)

@Composable
fun FocalPointAppRoot(
    selectedTab: Int,
    onTabSelected: (Int) -> Unit,
    coreHolder: CoreHolder
) {
    Scaffold(
        bottomBar = {
            NavigationBar(
                modifier = Modifier.fillMaxWidth(),
                containerColor = MaterialTheme.colorScheme.surface,
                contentColor = MaterialTheme.colorScheme.onSurface
            ) {
                TABS.forEachIndexed { index, tab ->
                    NavigationBarItem(
                        icon = { Icon(tab.icon, contentDescription = tab.label) },
                        label = { Text(tab.label) },
                        selected = selectedTab == index,
                        onClick = { onTabSelected(index) },
                        alwaysShowLabel = true
                    )
                }
            }
        }
    ) { innerPadding ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPadding)
        ) {
            when (selectedTab) {
                0 -> TasksScreen(coreHolder)
                1 -> FocusScreen(coreHolder)
                2 -> TodayScreen(coreHolder)
                3 -> RulesScreen(coreHolder)
                4 -> WalletScreen(coreHolder)
                5 -> ActivityScreen(coreHolder)
                6 -> SettingsScreen(coreHolder)
            }
        }
    }
}
