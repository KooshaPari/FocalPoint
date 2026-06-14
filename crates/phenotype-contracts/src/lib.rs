#![forbid(unsafe_code)]

//! Phenotype Contracts — Domain primitives, ports, and DDD building blocks.

pub mod models;
pub mod ports;

pub use models::*;
pub use ports::inbound::*;
pub use ports::outbound::*;

use phenotype_error_core::PhenotypeError;

/// Re-exported unified error type.
pub type Error = PhenotypeError;

/// Re-exported unified result type.
pub type Result<T> = phenotype_error_core::Result<T>;
