use std::fmt::Debug;
use std::hash::Hash;

/// Core trait for all domain entities with typed identity.
pub trait Entity: Debug + Clone + Send + Sync {
    type Id: Clone + Send + Sync + Eq + Hash;
    fn id(&self) -> &Self::Id;
}

/// Extension trait providing helper methods for Entity types.
pub trait EntityExt: Entity {
    fn same_identity(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<T: Entity> EntityExt for T {}

/// Trait for value objects — immutable, equality-by-value objects.
pub trait ValueObject: Debug + Clone + PartialEq + Eq + Send + Sync {}

/// Trait for aggregate roots — consistency boundaries in the domain.
pub trait AggregateRoot: Entity {
    type Event: DomainEvent;
    fn pull_events(&mut self) -> Vec<Self::Event>;
    fn has_pending_events(&self) -> bool;
}

/// Extension trait for aggregate roots.
pub trait AggregateRootExt: AggregateRoot {
    fn flush_events(&mut self) -> Vec<Self::Event> {
        self.pull_events()
    }
}

impl<T: AggregateRoot> AggregateRootExt for T {}

/// Trait for domain events.
pub trait DomainEvent: Debug + Clone + Send + Sync + 'static {
    fn event_type(&self) -> &'static str;
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;
}

/// Wrapper for optional entity existence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityResult<E> {
    Found(E),
    NotFound,
}
