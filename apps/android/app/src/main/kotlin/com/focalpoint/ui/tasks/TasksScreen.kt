// TasksScreen — Task list with swipe-to-dismiss, add task FAB, empty state
//
// Phase 1 implementation: mock task list, swipe-to-dismiss handler, empty state with Coachy.
// LazyColumn for vertical scrolling task rows. FAB for adding tasks (deferred to Phase 5).
// Binding: Phase 5 will integrate with coreHolder.getTasks().

package com.focalpoint.ui.tasks

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Delete
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
import com.focalpoint.core.TaskState
import kotlinx.coroutines.launch

data class Task(
    val id: String,
    val title: String,
    val dueDate: String,
    val priority: String  // "high", "medium", "low"
) {
    companion object {
        fun fromTaskState(taskState: TaskState): Task {
            // Map priority_weight to a categorical priority: high=0.8+, medium=0.4-0.8, low<0.4
            val priority = when {
                taskState.priorityWeight >= 0.8f -> "high"
                taskState.priorityWeight >= 0.4f -> "medium"
                else -> "low"
            }

            // Format deadline ISO to readable label; fallback to generic label if null
            val dueDate = taskState.deadlineIso?.let { iso ->
                // Simplified: show just the date part of ISO string (YYYY-MM-DD)
                iso.take(10)
            } ?: "Flexible"

            return Task(
                id = taskState.id,
                title = taskState.title,
                dueDate = dueDate,
                priority = priority
            )
        }
    }
}

@Composable
fun TasksScreen(coreHolder: CoreHolder) {
    var tasks by remember { mutableStateOf(emptyList<Task>()) }
    var isLoading by remember { mutableStateOf(true) }
    val scope = rememberCoroutineScope()

    LaunchedEffect(Unit) {
        scope.launch {
            try {
                val taskStates = coreHolder.getTaskList()
                tasks = taskStates.map { Task.fromTaskState(it) }
            } catch (e: Exception) {
                // Graceful degradation: show empty state on error
                tasks = emptyList()
            } finally {
                isLoading = false
            }
        }
    }

    val isEmptyState = tasks.isEmpty()

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Tasks") },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.surface
                )
            )
        },
        floatingActionButton = {
            FloatingActionButton(
                onClick = { /* Phase 5: open add task dialog */ },
                containerColor = MaterialTheme.colorScheme.primary
            ) {
                Icon(Icons.Default.Add, contentDescription = "Add task")
            }
        }
    ) { innerPadding ->
        if (isEmptyState) {
            EmptyTaskState(modifier = Modifier.padding(innerPadding))
        } else {
            LazyColumn(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(innerPadding)
            ) {
                items(
                    items = tasks,
                    key = { it.id }
                ) { task ->
                    TaskRow(
                        task = task,
                        onDismiss = {
                            tasks = tasks.filterNot { it.id == task.id }
                        }
                    )
                    Divider(color = Color(0xFFE5E5E5), thickness = 0.5.dp)
                }

                item {
                    Spacer(modifier = Modifier.height(80.dp))
                }
            }
        }
    }
}

@Composable
private fun TaskRow(
    task: Task,
    onDismiss: () -> Unit
) {
    var isExpanded by remember { mutableStateOf(false) }

    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp),
        shape = RoundedCornerShape(8.dp),
        colors = CardDefaults.cardColors(
            containerColor = when (task.priority) {
                "high" -> Color(0xFFFFEBEE)    // Light red
                "medium" -> Color(0xFFFFF3E0)  // Light orange
                else -> Color(0xFFF5F5F5)      // Light gray
            }
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column(
                modifier = Modifier
                    .weight(1f)
                    .padding(end = 12.dp)
            ) {
                Text(
                    text = task.title,
                    fontSize = 16.sp,
                    fontWeight = FontWeight.SemiBold,
                    color = MaterialTheme.colorScheme.onBackground,
                    maxLines = if (isExpanded) Int.MAX_VALUE else 1
                )
                Spacer(modifier = Modifier.height(4.dp))
                Row(
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = task.dueDate,
                        fontSize = 12.sp,
                        color = Color.Gray
                    )
                    Text(
                        text = "•",
                        fontSize = 12.sp,
                        color = Color.Gray
                    )
                    Text(
                        text = task.priority.uppercase(),
                        fontSize = 12.sp,
                        fontWeight = FontWeight.Bold,
                        color = when (task.priority) {
                            "high" -> Color(0xFFD32F2F)
                            "medium" -> Color(0xFFF57C00)
                            else -> Color(0xFF757575)
                        }
                    )
                }
            }

            IconButton(
                onClick = onDismiss,
                modifier = Modifier.size(40.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Delete,
                    contentDescription = "Delete task",
                    tint = Color(0xFFD32F2F)
                )
            }
        }
    }
}

@Composable
private fun EmptyTaskState(modifier: Modifier = Modifier) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        // Coachy placeholder
        Card(
            modifier = Modifier.size(100.dp),
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
                    text = "✅",
                    fontSize = 56.sp,
                    textAlign = TextAlign.Center
                )
            }
        }

        Spacer(modifier = Modifier.height(24.dp))

        Text(
            text = "No tasks yet!",
            fontSize = 20.sp,
            fontWeight = FontWeight.Bold,
            color = MaterialTheme.colorScheme.onBackground
        )

        Spacer(modifier = Modifier.height(8.dp))

        Text(
            text = "Tap the + button to create a new task.",
            fontSize = 14.sp,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            textAlign = TextAlign.Center
        )
    }
}
