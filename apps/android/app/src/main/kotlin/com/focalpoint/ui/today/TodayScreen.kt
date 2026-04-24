// TodayScreen — Daily dashboard with greeting, focus button, stat cards
//
// Phase 1 implementation: greeting, quick-start CTA, 3 stats (streak, credits, sessions today).
// Uses LazyColumn for vertical scrolling. Placeholder Coachy image (drawable placeholder).
// Binding: calls coreHolder methods for stats (deferred to Phase 5 for live updates).

package com.focalpoint.ui.today

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.focalpoint.core.CoreHolder

@Composable
fun TodayScreen(coreHolder: CoreHolder) {
    var streak by remember { mutableStateOf("--") }
    var credits by remember { mutableStateOf("--") }
    var sessionsToday by remember { mutableStateOf("--") }

    LaunchedEffect(Unit) {
        try {
            // Phase 5: Replace with live coreHolder.getTodayStats()
            streak = "7"
            credits = coreHolder.getWalletBalance().toString()
            sessionsToday = "3"
        } catch (e: Exception) {
            // Graceful degradation: show -- on error
        }
    }

    LazyColumn(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.Top
    ) {
        // Greeting section with Coachy placeholder
        item {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(vertical = 24.dp)
            ) {
                // Coachy placeholder: scaled placeholder drawable (e.g., ic_coachy_wave)
                Card(
                    modifier = Modifier
                        .size(120.dp),
                    colors = CardDefaults.cardColors(
                        containerColor = Color(0xFFF3E5AB)
                    ),
                    shape = RoundedCornerShape(16.dp)
                ) {
                    Box(
                        contentAlignment = Alignment.Center,
                        modifier = Modifier.fillMaxSize()
                    ) {
                        Text(
                            text = "🏆",
                            fontSize = 64.sp,
                            textAlign = TextAlign.Center
                        )
                    }
                }

                Spacer(modifier = Modifier.height(16.dp))

                Text(
                    text = "Good morning!",
                    fontSize = 28.sp,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onBackground
                )

                Spacer(modifier = Modifier.height(8.dp))

                Text(
                    text = "Let's build focus today.",
                    fontSize = 16.sp,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }

        // Start focus button
        item {
            Button(
                onClick = { /* Phase 5: navigate to FocusTimerScreen */ },
                modifier = Modifier
                    .fillMaxWidth()
                    .height(56.dp),
                colors = ButtonDefaults.buttonColors(
                    containerColor = MaterialTheme.colorScheme.primary
                )
            ) {
                Icon(
                    imageVector = Icons.Default.PlayArrow,
                    contentDescription = null,
                    modifier = Modifier.size(24.dp)
                )
                Spacer(modifier = Modifier.width(8.dp))
                Text(
                    text = "Start focus session",
                    fontSize = 16.sp,
                    fontWeight = FontWeight.SemiBold
                )
            }

            Spacer(modifier = Modifier.height(32.dp))
        }

        // Stat cards
        item {
            Text(
                text = "Today's Progress",
                fontSize = 18.sp,
                fontWeight = FontWeight.Bold,
                modifier = Modifier.padding(bottom = 12.dp)
            )
        }

        item {
            StatCard(
                label = "Focus Streak",
                value = streak,
                unit = "days",
                backgroundColor = Color(0xFFE0E7FF)  // Light indigo
            )
        }

        item {
            Spacer(modifier = Modifier.height(12.dp))
        }

        item {
            StatCard(
                label = "Credits Available",
                value = credits,
                unit = "pts",
                backgroundColor = Color(0xFFEDE9FE)  // Light purple
            )
        }

        item {
            Spacer(modifier = Modifier.height(12.dp))
        }

        item {
            StatCard(
                label = "Sessions Today",
                value = sessionsToday,
                unit = "completed",
                backgroundColor = Color(0xFFFEF3C7)  // Light amber
            )
        }

        item {
            Spacer(modifier = Modifier.height(32.dp))
        }
    }
}

@Composable
private fun StatCard(
    label: String,
    value: String,
    unit: String,
    backgroundColor: Color
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 4.dp),
        colors = CardDefaults.cardColors(containerColor = backgroundColor),
        shape = RoundedCornerShape(12.dp)
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text(
                    text = label,
                    fontSize = 14.sp,
                    color = Color.Gray
                )
                Spacer(modifier = Modifier.height(4.dp))
                Row(verticalAlignment = Alignment.Baseline) {
                    Text(
                        text = value,
                        fontSize = 24.sp,
                        fontWeight = FontWeight.Bold,
                        color = MaterialTheme.colorScheme.onBackground
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = unit,
                        fontSize = 12.sp,
                        color = Color.Gray,
                        lineHeight = 32.sp
                    )
                }
            }
        }
    }
}
