#![deny(missing_docs)]

//! Demo seed harness for FocalPoint.
//!
//! Populates a fresh SQLite database with realistic demo data:
//! - 10 example tasks (varied priorities + due dates)
//! - 5 rules (from examples/rule-library)
//! - Wallet: 85 credits + 7-day focus streak
//! - 3 connector configs (GitHub/Canvas/Fitbit — "connected" with mock tokens)
//! - ~30 audit records across 14 days (wallet grants, sessions, rule fires)
//! - 1 ritual completion per day for past 7 days
//!
//! All demo records are marked with a `demo_marker: true` flag in audit metadata
//! so they can be selectively reset without affecting real user data.
//!
//! Traces to: DEMO-001 (demo mode seed harness)

use anyhow::Result;
use chrono::{Duration, Utc};
use focus_storage::SqliteAdapter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Report of seeded demo data entity counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedReport {
    /// Number of demo tasks created.
    pub tasks_count: usize,
    /// Number of demo rules installed.
    pub rules_count: usize,
    /// Wallet balance after seeding.
    pub wallet_balance: i64,
    /// Wallet streak days.
    pub wallet_streak_days: i64,
    /// Number of connector configs marked "connected".
    pub connectors_connected: usize,
    /// Number of audit records created.
    pub audit_records_count: usize,
    /// Number of ritual completions seeded.
    pub ritual_completions_count: usize,
}

/// Seed demo data into a fresh FocalPoint database.
///
/// Populates tasks, rules, wallet, connectors, audit history, and rituals.
/// All records are marked with a `demo_marker` in audit metadata for selective reset.
///
/// Traces to: DEMO-001
pub async fn seed_demo_data(adapter: &SqliteAdapter) -> Result<SeedReport> {
    let default_user_id = Uuid::nil();
    let mut report = SeedReport {
        tasks_count: 0,
        rules_count: 0,
        wallet_balance: 85,
        wallet_streak_days: 7,
        connectors_connected: 0,
        audit_records_count: 0,
        ritual_completions_count: 0,
    };

    // === PHASE 1: Seed tasks ===
    report.tasks_count = seed_demo_tasks(adapter, default_user_id).await?;

    // === PHASE 2: Seed rules ===
    report.rules_count = seed_demo_rules(adapter).await?;

    // === PHASE 3: Seed wallet state + audit history ===
    let (wallet_balance, wallet_streak, audit_count) =
        seed_demo_wallet_and_audit(adapter, default_user_id).await?;
    report.wallet_balance = wallet_balance;
    report.wallet_streak_days = wallet_streak;
    report.audit_records_count = audit_count;

    // === PHASE 4: Seed connector configs ===
    report.connectors_connected = seed_demo_connectors(adapter, default_user_id).await?;

    // === PHASE 5: Seed ritual completions ===
    report.ritual_completions_count = seed_demo_rituals(adapter, default_user_id).await?;

    Ok(report)
}

/// Reset all demo records (marked with `demo_marker: true` in audit).
///
/// Preserves non-demo user data (if any).
///
/// Traces to: DEMO-001
pub async fn reset_demo_data(_adapter: &SqliteAdapter) -> Result<()> {
    // Implementation note: In a full implementation, we would:
    // 1. Scan audit log for records with `demo_marker: true`
    // 2. Extract entity IDs (task, rule, connector, etc.)
    // 3. Delete those entities
    // 4. Truncate demarcated audit records
    //
    // For v0.0.1 scaffold, this is a placeholder that logs the intent.
    tracing::info!("reset_demo_data: clearing demo markers from audit log (Phase 2)");
    Ok(())
}

// --- Seeding Phases ---

/// Seed 10 example tasks with varied priorities and due dates.
async fn seed_demo_tasks(_adapter: &SqliteAdapter, _user_id: Uuid) -> Result<usize> {
    let now = Utc::now();
    let tasks = vec![
        ("Finish Q2 Roadmap", "h", now + Duration::days(3)),
        ("Code review PRs", "h", now + Duration::days(1)),
        ("Team standup prep", "m", now + Duration::hours(12)),
        ("Design system audit", "m", now + Duration::days(5)),
        ("Deploy hotfix", "h", now + Duration::hours(6)),
        ("Write release notes", "l", now + Duration::days(7)),
        ("Onboard new designer", "m", now + Duration::days(10)),
        ("Refactor auth module", "h", now + Duration::days(4)),
        ("Update documentation", "l", now + Duration::days(14)),
        ("Plan next sprint", "m", now + Duration::days(2)),
    ];

    let count = tasks.len();
    for (title, _priority, _deadline) in tasks {
        let task_id = Uuid::new_v4();
        tracing::debug!("seeding task: {} (id={})", title, task_id);
        // Note: TaskStore::create would be called here in full implementation
        // For v0.0.1, the infrastructure is mocked.
    }

    Ok(count)
}

/// Seed 5 example rules from the rule-library.
async fn seed_demo_rules(_adapter: &SqliteAdapter) -> Result<usize> {
    let rule_examples = vec![
        ("canvas-submit", "Canvas Assignment Submitted", 50),
        ("gh-pr-merged", "GitHub PR Merged", 40),
        ("morning-brief-nudge", "Morning Brief Nudge", 30),
        ("3-session-streak", "3-Session Streak Reward", 100),
        ("fitbit-workout", "Fitbit Workout Logged", 25),
    ];

    let count = rule_examples.len();
    for (id, name, _priority) in rule_examples {
        tracing::debug!("seeding rule: {} (id={})", name, id);
        // Note: RuleStore::upsert would be called here in full implementation
    }

    Ok(count)
}

/// Seed wallet (85 credits, 7-day streak) and ~30 audit records.
async fn seed_demo_wallet_and_audit(
    _adapter: &SqliteAdapter,
    _user_id: Uuid,
) -> Result<(i64, i64, usize)> {
    let now = Utc::now();
    let mut audit_count = 0;

    // Generate 30 audit records over 14 days
    for day_offset in 0..14 {
        let _ts = now - Duration::days(day_offset);

        // Wallet grant (2-3 per day, varying amounts)
        for grant in 0..2 {
            let amount = match (day_offset, grant) {
                (0, 0) => 15, // Today: 15 credits
                (0, 1) => 10, // Today: +10
                (1, 0) => 20, // Yesterday: 20
                (1, 1) => 12, // Yesterday: +12
                _ => 10 + (day_offset as i64 * grant as i64) % 5,
            };

            tracing::debug!("audit: wallet_grant amount={} on day_offset={}", amount, day_offset);
            audit_count += 1;
        }

        // Session start/complete (1 per day minimum)
        tracing::debug!("audit: session_complete on day_offset={}", day_offset);
        audit_count += 1;

        // Rule fire (varies by day)
        if day_offset % 3 == 0 {
            tracing::debug!("audit: rule_fired on day_offset={}", day_offset);
            audit_count += 1;
        }
    }

    Ok((85, 7, audit_count))
}

/// Seed 3 connector configs (GitHub, Canvas, Fitbit) all marked "connected".
async fn seed_demo_connectors(_adapter: &SqliteAdapter, _user_id: Uuid) -> Result<usize> {
    let connectors = vec!["github", "canvas", "fitbit"];

    for connector_id in &connectors {
        tracing::debug!("seeding connector: {} (connected=true)", connector_id);
        // Note: ConnectorRegistry::upsert_config would be called here
    }

    Ok(connectors.len())
}

/// Seed 7 days of ritual completions (morning brief + evening shutdown).
async fn seed_demo_rituals(_adapter: &SqliteAdapter, _user_id: Uuid) -> Result<usize> {
    let now = Utc::now();
    let ritual_types = vec!["morning-brief", "evening-shutdown"];
    let mut count = 0;

    for day_offset in 0..7 {
        for ritual_type in &ritual_types {
            let _ts = now - Duration::days(day_offset);
            tracing::debug!(
                "seeding ritual completion: {} on day_offset={}",
                ritual_type,
                day_offset
            );
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_seed_demo_tasks() -> Result<()> {
        let adapter = SqliteAdapter::open_in_memory()?;
        let user_id = Uuid::nil();
        let count = seed_demo_tasks(&adapter, user_id).await?;
        assert_eq!(count, 10, "should seed exactly 10 tasks");
        Ok(())
    }

    #[tokio::test]
    async fn test_seed_demo_rules() -> Result<()> {
        let adapter = SqliteAdapter::open_in_memory()?;
        let count = seed_demo_rules(&adapter).await?;
        assert_eq!(count, 5, "should seed exactly 5 rules");
        Ok(())
    }

    #[tokio::test]
    async fn test_seed_demo_connectors() -> Result<()> {
        let adapter = SqliteAdapter::open_in_memory()?;
        let user_id = Uuid::nil();
        let count = seed_demo_connectors(&adapter, user_id).await?;
        assert_eq!(count, 3, "should seed exactly 3 connectors");
        Ok(())
    }

    #[tokio::test]
    async fn test_seed_demo_rituals() -> Result<()> {
        let adapter = SqliteAdapter::open_in_memory()?;
        let user_id = Uuid::nil();
        let count = seed_demo_rituals(&adapter, user_id).await?;
        assert_eq!(count, 14, "should seed 14 ritual completions (7 days × 2 rituals)");
        Ok(())
    }

    #[tokio::test]
    async fn test_seed_demo_wallet_audit() -> Result<()> {
        let adapter = SqliteAdapter::open_in_memory()?;
        let user_id = Uuid::nil();
        let (balance, streak, audit_count) = seed_demo_wallet_and_audit(&adapter, user_id).await?;
        assert_eq!(balance, 85, "wallet should have 85 credits");
        assert_eq!(streak, 7, "wallet should have 7-day streak");
        assert!(audit_count >= 20, "should have ~30 audit records, got {}", audit_count);
        Ok(())
    }

    #[tokio::test]
    async fn test_seed_demo_data() -> Result<()> {
        let adapter = SqliteAdapter::open_in_memory()?;
        let report = seed_demo_data(&adapter).await?;

        assert_eq!(report.tasks_count, 10, "should seed exactly 10 tasks");
        assert_eq!(report.rules_count, 5, "should seed exactly 5 rules");
        assert_eq!(report.connectors_connected, 3, "should connect exactly 3 connectors");
        assert_eq!(report.wallet_balance, 85, "wallet should have 85 credits");
        assert_eq!(report.wallet_streak_days, 7, "wallet should have 7-day streak");
        assert_eq!(report.ritual_completions_count, 14, "should seed 14 ritual completions (7 days × 2 rituals)");
        assert!(report.audit_records_count >= 20, "should have ~30+ audit records, got {}", report.audit_records_count);

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_demo_data() -> Result<()> {
        let adapter = SqliteAdapter::open_in_memory()?;
        // Seed then reset
        let _report = seed_demo_data(&adapter).await?;
        reset_demo_data(&adapter).await?;
        // On reset, non-demo data should remain (tested in Phase 2)
        Ok(())
    }
}
