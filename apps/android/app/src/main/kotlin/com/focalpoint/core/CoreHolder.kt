// CoreHolder — Singleton access to Rust FFI core
//
// Initializes FocalPointCore with device storage path.
// Provides coroutine-safe access to wallet balance, rules evaluation, audit logs.
//
// Real implementation details deferred; this is a stub that catches FFI errors gracefully.

package com.focalpoint.core

import android.content.Context
import androidx.lifecycle.ViewModel
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * Represents the current wallet state returned from the Rust core.
 * Placeholder; actual type comes from generated FocalPointCore.kt.
 */
data class WalletState(
    val creditsAvailable: Long = 0,
    val totalEarned: Long = 0,
    val totalSpent: Long = 0
)

/**
 * Singleton wrapper around the Rust FFI core.
 *
 * All Rust FFI calls are async (suspend functions) and run on IO dispatcher
 * to avoid blocking the main thread.
 */
class CoreHolder(private val context: Context) : ViewModel() {
    private val storagePath: String = context.filesDir.absolutePath
    private val userId: String = "default-user"

    // FFI core will be initialized here after uniffi-bindgen generates FocalPointCore.kt
    // For now, this is a placeholder that demonstrates the pattern.

    /**
     * Fetch current wallet balance from Rust core.
     * Returns credits available, total earned, and total spent.
     */
    suspend fun getWalletBalance(): WalletState = withContext(Dispatchers.IO) {
        return@withContext try {
            // TODO: call generated FFI once uniffi-bindgen runs:
            // val core = FocalPointCore(storagePath, userId)
            // val balance = core.getWalletBalance()
            // WalletState(balance.creditsAvailable, balance.totalEarned, balance.totalSpent)
            WalletState(5000, 8000, 3000)  // Placeholder
        } catch (e: Exception) {
            throw Exception("Failed to fetch wallet balance: ${e.message}", e)
        }
    }

    /**
     * Evaluate rules for a given app ID.
     * Returns true if any rule fired; false otherwise.
     */
    suspend fun evaluateRules(appId: String): Boolean = withContext(Dispatchers.IO) {
        return@withContext try {
            // TODO: call generated FFI once uniffi-bindgen runs:
            // val core = FocalPointCore(storagePath, userId)
            // val result = core.evaluateRules(appId)
            // result != null
            false  // Placeholder
        } catch (e: Exception) {
            throw Exception("Failed to evaluate rules for $appId: ${e.message}", e)
        }
    }

    /**
     * Fetch audit records (read-only view of all state changes).
     * Returns the latest [limit] records.
     */
    suspend fun getAuditRecords(limit: Int): List<String> = withContext(Dispatchers.IO) {
        return@withContext try {
            // TODO: call generated FFI once uniffi-bindgen runs:
            // val core = FocalPointCore(storagePath, userId)
            // core.getAuditRecords(limit).map { it.toString() }
            emptyList()  // Placeholder
        } catch (e: Exception) {
            throw Exception("Failed to fetch audit records: ${e.message}", e)
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
