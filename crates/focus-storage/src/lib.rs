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

pub use sqlite::SqliteAdapter;
