#![forbid(unsafe_code)]

//! Phenotype Policy Engine — TOML-configured policy and rule evaluation engine.

pub mod context;
pub mod engine;
pub mod loader;
pub mod policy;
pub mod result;
pub mod rule;

pub use context::*;
pub use engine::*;
pub use loader::*;
pub use policy::*;
pub use result::*;
pub use rule::*;
