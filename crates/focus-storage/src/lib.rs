//! Storage ports + SQLite impls.

pub mod ports {
    use async_trait::async_trait;

    #[async_trait]
    pub trait EventStore: Send + Sync {
        async fn append(&self, event: focus_events::NormalizedEvent) -> anyhow::Result<()>;
        async fn since_cursor(
            &self,
            cursor: Option<&str>,
            limit: usize,
        ) -> anyhow::Result<Vec<focus_events::NormalizedEvent>>;
    }

    #[async_trait]
    pub trait RuleStore: Send + Sync {
        async fn get(&self, id: uuid::Uuid) -> anyhow::Result<Option<focus_rules::Rule>>;
        async fn list_enabled(&self) -> anyhow::Result<Vec<focus_rules::Rule>>;
    }

    #[async_trait]
    pub trait WalletStore: Send + Sync {
        async fn load(&self, user_id: uuid::Uuid) -> anyhow::Result<focus_rewards::RewardWallet>;
        async fn apply(
            &self,
            user_id: uuid::Uuid,
            mutation: focus_rewards::WalletMutation,
        ) -> anyhow::Result<()>;
    }

    /// Re-export of the sync [`focus_planning::TaskStore`] port so callers
    /// routing through `focus_storage::ports` find it alongside the other
    /// store traits. Canonical definition lives in `focus-planning` to keep
    /// the domain type colocated with its persistence surface.
    pub use focus_planning::TaskStore;

    #[async_trait]
    pub trait PenaltyStore: Send + Sync {
        async fn load(&self, user_id: uuid::Uuid) -> anyhow::Result<focus_penalties::PenaltyState>;
        async fn apply(
            &self,
            user_id: uuid::Uuid,
            mutation: focus_penalties::PenaltyMutation,
        ) -> anyhow::Result<()>;
    }
}

pub mod sqlite;
pub mod wipe;

pub use sqlite::SqliteAdapter;
pub use wipe::{wipe_all, WipeReceipt};
