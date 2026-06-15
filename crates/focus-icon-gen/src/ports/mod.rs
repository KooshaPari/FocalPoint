//! Port traits consumed by the focus-icon-gen adapters.
//!
//! The crate follows a hexagonal layout: this module owns the abstract
//! interfaces, the [`adapters`](crate::adapters) module owns the concrete
//! implementations, and [`IconGenerator`](crate::IconGenerator) is the
//! default production adapter kept at the crate root for backwards
//! compatibility with existing consumers.

pub mod renderer;

pub use renderer::{RenderedIcon, RendererPort};
