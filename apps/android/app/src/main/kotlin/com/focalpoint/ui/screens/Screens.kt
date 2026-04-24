// Tab screen stubs — all v0.0.1 placeholders
//
// Only WalletScreen demonstrates FFI integration (fetches credits balance).
// Others are pure placeholders awaiting Phase 2+ implementations.

package com.focalpoint.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.focalpoint.core.CoreHolder
import com.focalpoint.ui.components.PlaceholderBanner

// Tasks — Canvas assignment ingestion
@Composable
fun TasksScreen(coreHolder: CoreHolder) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        PlaceholderBanner()
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Tasks from Canvas connectors will appear here.",
            fontSize = 14.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

// Focus — Active session timer + rule evaluation
@Composable
fun FocusScreen(coreHolder: CoreHolder) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        PlaceholderBanner()
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Focus timer and real-time rule evaluation will be here.",
            fontSize = 14.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

// Today — Daily dashboard + rituals
@Composable
fun TodayScreen(coreHolder: CoreHolder) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        PlaceholderBanner()
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Today's rituals and habit dashboard coming soon.",
            fontSize = 14.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

// Rules — Rule browser (read-only in v1)
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

// Wallet — Credits balance + transaction log
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
            text = "Transaction history coming soon.",
            fontSize = 12.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

// Activity — Audit log viewer (read-only)
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
            text = "Audit log viewer coming soon.",
            fontSize = 14.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

// Settings — Permissions, accounts, diagnostics
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
            text = "Permissions, accounts, and diagnostics coming soon.",
            fontSize = 14.sp,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )
    }
}
