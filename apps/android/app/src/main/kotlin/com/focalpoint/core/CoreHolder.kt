// CoreHolder — Singleton access to Rust FFI core
//
// Initializes FocalPointCore with device storage path via generated UniFFI bindings.
// Provides coroutine-safe access to wallet balance, task list, rules evaluation, audit logs.
// All FFI calls run on IO dispatcher to avoid main thread blocking.

package com.focalpoint.core

import android.content.Context
import androidx.lifecycle.ViewModel
import com.focalpoint.ffi.FocalPointCore
import com.focalpoint.ffi.TaskSummaryDto
import com.focalpoint.ffi.WalletSummary
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * Represents the current wallet state returned from the Rust core.
 * Maps WalletSummary from generated FFI types.
 */
data class WalletState(
    val creditsAvailable: Long = 0,
    val totalEarned: Long = 0,
    val totalSpent: Long = 0,
    val multiplier: Float = 1.0f
)

/**
 * Represents a task from the Rust core.
 * Maps TaskSummaryDto from generated FFI types.
 */
data class TaskState(
    val id: String,
    val title: String,
    val durationMinutes: UInt,
    val priorityWeight: Float,
    val deadlineIso: String? = null,
    val deadlineRigidity: String = "flexible",
    val status: String = "pending"
)

/**
 * Singleton wrapper around the Rust FFI core.
 *
 * Lazy-initializes FocalPointCore on first access.
 * All Rust FFI calls are suspend functions running on IO dispatcher.
 * Catches FFI errors and wraps them in Kotlin exceptions.
 */
class CoreHolder(private val context: Context) : ViewModel() {
    private val storagePath: String = context.filesDir.absolutePath

    private lazy val core by lazy {
        FocalPointCore(storagePath)
    }

    /**
     * Fetch current wallet state from Rust core.
     * Returns credits available, total earned, total spent, and multiplier.
     */
    suspend fun getWalletBalance(): WalletState = withContext(Dispatchers.IO) {
        return@withContext try {
            val wallet = core.wallet().load()
            WalletState(
                creditsAvailable = wallet.balance,
                totalEarned = wallet.earned,
                totalSpent = wallet.spent,
                multiplier = wallet.multiplier
            )
        } catch (e: Exception) {
            throw Exception("Failed to fetch wallet balance: ${e.message}", e)
        }
    }

    /**
     * Fetch task list from Rust core.
     * Returns all tasks with their status, priority, deadline, and duration.
     */
    suspend fun getTaskList(): List<TaskState> = withContext(Dispatchers.IO) {
        return@withContext try {
            core.tasks().list().map { dto ->
                TaskState(
                    id = dto.id,
                    title = dto.title,
                    durationMinutes = dto.duration_minutes,
                    priorityWeight = dto.priority_weight,
                    deadlineIso = dto.deadline_iso,
                    deadlineRigidity = dto.deadline_rigidity,
                    status = dto.status
                )
            }
        } catch (e: Exception) {
            throw Exception("Failed to fetch task list: ${e.message}", e)
        }
    }

    /**
     * Evaluate rules for a given app ID via policy builder.
     * Returns true if any rule fired; false otherwise.
     * Currently a stub pending rule evaluation API surface.
     */
    suspend fun evaluateRules(appId: String): Boolean = withContext(Dispatchers.IO) {
        return@withContext try {
            // Pending: rules() and eval() API full surface exposure
            false
        } catch (e: Exception) {
            throw Exception("Failed to evaluate rules for $appId: ${e.message}", e)
        }
    }

    /**
     * Fetch recent audit records (read-only view of all state changes).
     * Returns the latest [limit] records formatted as strings.
     */
    suspend fun getAuditRecords(limit: Int): List<String> = withContext(Dispatchers.IO) {
        return@withContext try {
            core.audit().recent(limit.toUInt()).map { record ->
                "${record.record_type}(${record.subject_ref}) @ ${record.occurred_at_iso}"
            }
        } catch (e: Exception) {
            throw Exception("Failed to fetch audit records: ${e.message}", e)
        }
    }

    /**
     * Emit a synthetic host event (e.g., focus session start/stop).
     * Allows the platform layer (Android sensors, timers, etc.) to feed events
     * directly into the rule evaluation pipeline.
     */
    suspend fun emitHostEvent(eventType: String, payload: String): Unit = withContext(Dispatchers.IO) {
        return@withContext try {
            val event = com.focalpoint.ffi.HostEventDto(
                event_type = eventType,
                confidence = 1.0f,
                payload_json = payload,
                dedupe_key = null
            )
            core.host_events().emit(event)
        } catch (e: Exception) {
            throw Exception("Failed to emit host event: ${e.message}", e)
        }
    }

    companion object {
        @Volatile
        private var instance: CoreHolder? = null

        fun getInstance(context: Context): CoreHolder {
            return instance ?: synchronized(this) {
                CoreHolder(context).also { instance = it }
            }
        }
    }
}
