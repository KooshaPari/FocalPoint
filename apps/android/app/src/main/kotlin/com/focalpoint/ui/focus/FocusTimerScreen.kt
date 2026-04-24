// FocusTimerScreen — Focus timer with circular progress, MM:SS countdown, controls
//
// Phase 1 implementation: circular progress indicator, MM:SS text via rememberUpdatedState,
// pause/cancel buttons, Coachy encouragement strip. Timer countdown uses remember + LaunchedEffect.
// Binding: Phase 5 will integrate with coreHolder.startFocusSession() and rule evaluation.

package com.focalpoint.ui.focus

import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Pause
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.focalpoint.core.CoreHolder
import kotlinx.coroutines.launch
import kotlin.math.max

@Composable
fun FocusTimerScreen(coreHolder: CoreHolder) {
    // Timer state: total seconds in session (default 25 min Pomodoro)
    var totalSeconds by remember { mutableStateOf(25 * 60) }
    var remainingSeconds by remember { mutableStateOf(totalSeconds) }
    var isRunning by remember { mutableStateOf(false) }
    var sessionStartedEmitted by remember { mutableStateOf(false) }
    val scope = rememberCoroutineScope()

    // Emit session_started event when timer begins
    LaunchedEffect(isRunning) {
        if (isRunning && !sessionStartedEmitted) {
            sessionStartedEmitted = true
            scope.launch {
                try {
                    coreHolder.emitHostEvent(
                        "session_started",
                        "{\"duration_minutes\": ${totalSeconds / 60}}"
                    )
                } catch (e: Exception) {
                    // Silently fail; don't block timer
                }
            }
        }
    }

    // Emit session_completed event when timer finishes
    LaunchedEffect(remainingSeconds) {
        if (remainingSeconds == 0 && sessionStartedEmitted) {
            sessionStartedEmitted = false
            scope.launch {
                try {
                    coreHolder.emitHostEvent(
                        "session_completed",
                        "{\"duration_minutes\": ${totalSeconds / 60}}"
                    )
                } catch (e: Exception) {
                    // Silently fail; don't block timer
                }
            }
        }
    }

    // Update on every tick
    LaunchedEffect(isRunning) {
        if (isRunning) {
            while (isRunning && remainingSeconds > 0) {
                kotlinx.coroutines.delay(1000L)
                remainingSeconds = max(0, remainingSeconds - 1)
                if (remainingSeconds == 0) {
                    isRunning = false
                }
            }
        }
    }

    // Progress for circular indicator (0f to 1f)
    val progress by rememberUpdatedState(
        1f - (remainingSeconds.toFloat() / totalSeconds.toFloat())
    )

    // Format time as MM:SS
    val minutes = remainingSeconds / 60
    val seconds = remainingSeconds % 60
    val timeString = String.format("%02d:%02d", minutes, seconds)

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(MaterialTheme.colorScheme.background)
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.SpaceBetween
    ) {
        // Header
        TopAppBar(
            title = { Text("Focus Session") },
            modifier = Modifier.fillMaxWidth(),
            colors = TopAppBarDefaults.topAppBarColors(
                containerColor = Color.Transparent
            )
        )

        // Circular progress indicator with countdown
        Box(
            contentAlignment = Alignment.Center,
            modifier = Modifier
                .size(280.dp)
                .padding(32.dp)
        ) {
            // Background circle
            androidx.compose.foundation.Canvas(
                modifier = Modifier.fillMaxSize()
            ) {
                drawCircle(
                    color = Color(0xFFE5E5E5),
                    radius = size.minDimension / 2,
                    style = Stroke(width = 8.dp.toPx())
                )
            }

            // Progress arc
            androidx.compose.foundation.Canvas(
                modifier = Modifier.fillMaxSize()
            ) {
                drawArc(
                    color = MaterialTheme.colorScheme.primary,
                    startAngle = -90f,
                    sweepAngle = progress.value * 360f,
                    useCenter = false,
                    style = Stroke(width = 8.dp.toPx())
                )
            }

            // Center text: MM:SS
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                modifier = Modifier.padding(16.dp)
            ) {
                Text(
                    text = timeString,
                    fontSize = 72.sp,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.primary,
                    fontFamily = androidx.compose.ui.text.font.FontFamily.Monospace
                )
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Focus Mode Active",
                    fontSize = 14.sp,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }

        // Controls
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 16.dp),
            horizontalArrangement = Arrangement.Center,
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Pause / Resume button
            FloatingActionButton(
                onClick = { isRunning = !isRunning },
                containerColor = MaterialTheme.colorScheme.secondary,
                modifier = Modifier.size(56.dp)
            ) {
                Icon(
                    imageVector = if (isRunning) Icons.Default.Pause else Icons.Default.PlayArrow,
                    contentDescription = if (isRunning) "Pause" else "Resume",
                    tint = Color.White,
                    modifier = Modifier.size(24.dp)
                )
            }

            Spacer(modifier = Modifier.width(24.dp))

            // Cancel button
            FloatingActionButton(
                onClick = {
                    isRunning = false
                    remainingSeconds = totalSeconds
                },
                containerColor = MaterialTheme.colorScheme.errorContainer,
                modifier = Modifier.size(56.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Close,
                    contentDescription = "Cancel",
                    tint = MaterialTheme.colorScheme.onErrorContainer,
                    modifier = Modifier.size(24.dp)
                )
            }
        }

        // Coachy encouragement strip
        Card(
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 16.dp),
            colors = CardDefaults.cardColors(
                containerColor = Color(0xFFFEF3C7)
            ),
            shape = RoundedCornerShape(12.dp)
        ) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = "💪",
                    fontSize = 32.sp
                )

                Column(
                    modifier = Modifier.weight(1f)
                ) {
                    Text(
                        text = "You're doing great!",
                        fontSize = 14.sp,
                        fontWeight = FontWeight.Bold,
                        color = Color.Black
                    )
                    Text(
                        text = if (remainingSeconds > 60) {
                            "${remainingSeconds / 60} min to focus glory"
                        } else {
                            "Almost there! Stay focused."
                        },
                        fontSize = 12.sp,
                        color = Color.Gray
                    )
                }
            }
        }

        Spacer(modifier = Modifier.height(16.dp))
    }
}
