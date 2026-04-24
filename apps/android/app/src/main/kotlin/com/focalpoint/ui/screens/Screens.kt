// Tab screen implementations — Phase 1 concrete screens + stubs
//
// Today, Tasks, Focus: concrete implementations with Material 3 + Jetpack Compose.
// Rules, Wallet, Activity, Settings: Phase 2+ stubs awaiting implementation.

package com.focalpoint.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.focalpoint.core.CoreHolder
import com.focalpoint.ui.components.PlaceholderBanner
import com.focalpoint.ui.tasks.TasksScreen
import com.focalpoint.ui.focus.FocusTimerScreen
import com.focalpoint.ui.today.TodayScreen

// Re-export concrete screens from their modules
// TasksScreen -> com.focalpoint.ui.tasks.TasksScreen (imported above)
// FocusScreen -> com.focalpoint.ui.focus.FocusTimerScreen
// TodayScreen -> com.focalpoint.ui.today.TodayScreen (imported above)

@Composable
fun FocusScreen(coreHolder: CoreHolder) = FocusTimerScreen(coreHolder)

// Rules — Rule browser (read-only in v1) — Phase 2+
@Composable
fun RulesScreen(coreHolder: CoreHolder) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        PlaceholderBanner()
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Rule browser and editor coming in Phase 3+.",
            fontSize = 14.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

// Wallet — Credits balance + transaction log — Phase 2+
@Composable
fun WalletScreen(coreHolder: CoreHolder) {
    val walletBalance = remember { mutableStateOf("Loading...") }

    LaunchedEffect(Unit) {
        try {
            val balance = coreHolder.getWalletBalance()
            walletBalance.value = balance.toString()
        } catch (e: Exception) {
            walletBalance.value = "Error: ${e.message}"
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        PlaceholderBanner()
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Credits Balance",
            fontSize = 18.sp,
            fontWeight = androidx.compose.ui.text.font.FontWeight.Bold
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = walletBalance.value,
            fontSize = 16.sp,
            modifier = Modifier.fillMaxWidth()
        )
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Transaction history coming in v1.0.",
            fontSize = 12.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

// Activity — Audit log viewer (read-only) — Phase 2+
@Composable
fun ActivityScreen(coreHolder: CoreHolder) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        PlaceholderBanner()
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Audit log viewer coming in v1.0.",
            fontSize = 14.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

// Settings — Permissions, accounts, diagnostics — Phase 2+
@Composable
fun SettingsScreen(coreHolder: CoreHolder) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        PlaceholderBanner()
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Permissions, accounts, and diagnostics coming in v1.0.",
            fontSize = 14.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}
