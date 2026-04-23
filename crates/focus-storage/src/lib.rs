//! Storage ports + SQLite impls.

pub mod ports {
    use async_trait::async_trait;

    #[async_trait]
    pub trait EventStore: Send + Sync {
        async fn append(&self, _event: focus_events::NormalizedEvent) -> anyhow::Result<()>;
        async fn since_cursor(
            &self,
            _cursor: Option<&str>,
            _limit: usize,
        ) -> anyhow::Result<Vec<focus_events::NormalizedEvent>>;
    }

    #[async_trait]
    pub trait RuleStore: Send + Sync {
        async fn get(&self, _id: uuid::Uuid) -> anyhow::Result<Option<focus_rules::Rule>>;
        async fn list_enabled(&self) -> anyhow::Result<Vec<focus_rules::Rule>>;
    }

    #[async_trait]
    pub trait WalletStore: Send + Sync {
        async fn load(&self, _user_id: uuid::Uuid) -> anyhow::Result<focus_rewards::RewardWallet>;
        async fn apply(
            &self,
            _user_id: uuid::Uuid,
            _mutation: focus_rewards::WalletMutation,
        ) -> anyhow::Result<()>;
    }

    #[async_trait]
    pub trait PenaltyStore: Send + Sync {
        async fn load(&self, _user_id: uuid::Uuid) -> anyhow::Result<focus_penalties::PenaltyState>;
        async fn apply(
            &self,
            _user_id: uuid::Uuid,
            _mutation: focus_penalties::PenaltyMutation,
        ) -> anyhow::Result<()>;
    }
}

pub mod sqlite {
    //! SQLite adapter — schema migrations, prepared statements. Stub.
    pub struct SqliteAdapter;
}
