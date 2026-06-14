#![allow(async_fn_in_trait)]

// std::fmt::Debug removed — unused in this module.

/// Inbound port for use cases — the primary entry point for domain operations.
pub trait UseCase<I, O>: Send + Sync {
    async fn execute(&self, input: I) -> Result<O, crate::Error>;
}

/// Inbound port for command handlers.
pub trait CommandHandler<C>: Send + Sync {
    async fn handle(&self, command: C) -> Result<(), crate::Error>;
}

/// Inbound port for query handlers.
pub trait QueryHandler<Q, R>: Send + Sync {
    async fn handle(&self, query: Q) -> Result<R, crate::Error>;
}

/// Inbound port for event handlers.
pub trait EventHandler<E>: Send + Sync {
    async fn handle(&self, event: E) -> Result<(), crate::Error>;
}
