#![allow(async_fn_in_trait)]

use std::fmt::Debug;
use std::hash::Hash;

/// Outbound port for generic persistence.
pub trait Repository<E, I>: Send + Sync
where
    E: Debug + Clone + Send + Sync,
    I: Clone + Send + Sync + Eq + Hash,
{
    async fn find(&self, id: &I) -> Result<Option<E>, crate::Error>;
    async fn save(&self, entity: &E) -> Result<(), crate::Error>;
    async fn delete(&self, id: &I) -> Result<(), crate::Error>;
}

/// Outbound port for unit-of-work transactions.
pub trait UnitOfWork: Send + Sync {
    async fn commit(&self) -> Result<(), crate::Error>;
    async fn rollback(&self) -> Result<(), crate::Error>;
}

/// Outbound port for caching operations.
pub trait CachePort: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, crate::Error>;
    async fn set(
        &self,
        key: &str,
        value: Vec<u8>,
        ttl: Option<std::time::Duration>,
    ) -> Result<(), crate::Error>;
    async fn delete(&self, key: &str) -> Result<(), crate::Error>;
    async fn clear(&self) -> Result<(), crate::Error>;
}

/// Outbound port for publishing domain events.
pub trait EventPublisher: Send + Sync {
    async fn publish<E: serde::Serialize>(&self, event: &E) -> Result<(), crate::Error>;
    async fn publish_batch<E: serde::Serialize>(&self, events: &[E]) -> Result<(), crate::Error>;
}

/// Outbound port for subscribing to domain events.
pub trait EventSubscriber: Send + Sync {
    async fn subscribe<E: serde::de::DeserializeOwned>(
        &self,
        topic: &str,
    ) -> Result<(), crate::Error>;
}

/// Outbound port for secret management.
pub trait SecretPort: Send + Sync {
    async fn get_secret(&self, key: &str) -> Result<Option<String>, crate::Error>;
    async fn set_secret(&self, key: &str, value: &str) -> Result<(), crate::Error>;
    async fn delete_secret(&self, key: &str) -> Result<(), crate::Error>;
}

/// Outbound port for versioned secret management.
pub trait VersionedSecretPort: SecretPort + Send + Sync {
    async fn get_secret_version(
        &self,
        key: &str,
        version: &str,
    ) -> Result<Option<String>, crate::Error>;
    async fn list_secret_versions(&self, key: &str) -> Result<Vec<String>, crate::Error>;
    async fn rotate_secret(&self, key: &str, value: &str) -> Result<String, crate::Error>;
}
