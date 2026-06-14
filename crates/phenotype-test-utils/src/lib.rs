//! Phenotype Test Utils — Test utilities, fixtures, and mock implementations.

use phenotype_contracts::{CachePort, Entity, Repository, SecretPort};
use phenotype_error_core::{PhenotypeError, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Generic in-memory store for testing.
#[derive(Debug, Clone)]
pub struct InMemoryStore<K, V> {
    data: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> Default for InMemoryStore<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<K, V> InMemoryStore<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, key: K, value: V) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        data.remove(key)
    }

    pub fn clear(&self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
    }

    pub fn len(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// In-memory cache implementation for testing.
#[derive(Debug, Clone, Default)]
pub struct InMemoryCache {
    store: InMemoryStore<String, (Vec<u8>, Option<std::time::Instant>)>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CachePort for InMemoryCache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        if let Some((value, expiry)) = self.store.get(&key.to_string()) {
            if let Some(exp) = expiry {
                if std::time::Instant::now() > exp {
                    return Ok(None);
                }
            }
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    async fn set(&self, key: &str, value: Vec<u8>, ttl: Option<std::time::Duration>) -> Result<()> {
        let expiry = ttl.map(|d| std::time::Instant::now() + d);
        self.store.insert(key.to_string(), (value, expiry));
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        self.store.remove(&key.to_string());
        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        self.store.clear();
        Ok(())
    }
}

/// In-memory repository implementation for testing.
#[derive(Debug, Clone)]
pub struct InMemoryRepository<E: Entity> {
    store: InMemoryStore<E::Id, E>,
}

impl<E: Entity> Default for InMemoryRepository<E> {
    fn default() -> Self {
        Self {
            store: InMemoryStore::new(),
        }
    }
}

impl<E: Entity> InMemoryRepository<E> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E: Entity + Send + Sync> Repository<E, E::Id> for InMemoryRepository<E> {
    async fn find(&self, id: &E::Id) -> Result<Option<E>> {
        Ok(self.store.get(id))
    }

    async fn save(&self, entity: &E) -> Result<()> {
        self.store.insert(entity.id().clone(), entity.clone());
        Ok(())
    }

    async fn delete(&self, id: &E::Id) -> Result<()> {
        self.store.remove(id);
        Ok(())
    }
}

/// In-memory secret store for testing.
#[derive(Debug, Clone, Default)]
pub struct InMemorySecretStore {
    store: InMemoryStore<String, String>,
}

impl InMemorySecretStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SecretPort for InMemorySecretStore {
    async fn get_secret(&self, key: &str) -> Result<Option<String>> {
        Ok(self.store.get(&key.to_string()))
    }

    async fn set_secret(&self, key: &str, value: &str) -> Result<()> {
        self.store.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn delete_secret(&self, key: &str) -> Result<()> {
        self.store.remove(&key.to_string());
        Ok(())
    }
}

/// In-memory event publisher for testing.
#[derive(Debug, Clone, Default)]
pub struct InMemoryEventPublisher {
    events: Arc<Mutex<Vec<serde_json::Value>>>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event_count(&self) -> usize {
        let events = self.events.lock().unwrap();
        events.len()
    }

    pub fn clear(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }
}

impl phenotype_contracts::EventPublisher for InMemoryEventPublisher {
    async fn publish<E: serde::Serialize>(&self, event: &E) -> Result<()> {
        let value = serde_json::to_value(event)
            .map_err(|e| PhenotypeError::serialization(e.to_string()))?;
        let mut events = self.events.lock().unwrap();
        events.push(value);
        Ok(())
    }

    async fn publish_batch<E: serde::Serialize>(&self, events: &[E]) -> Result<()> {
        let mut store = self.events.lock().unwrap();
        for event in events {
            let value = serde_json::to_value(event)
                .map_err(|e| PhenotypeError::serialization(e.to_string()))?;
            store.push(value);
        }
        Ok(())
    }
}
