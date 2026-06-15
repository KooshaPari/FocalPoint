//! Concrete [`RendererPort`] implementations.
//!
//! Each adapter lives in its own submodule. The default production
//! renderer ([`IconGenerator`](crate::IconGenerator)) is kept at the
//! crate root for backwards compatibility; this module collects the
//! other adapters — currently the [`null_renderer::NullRenderer`] test
//! double, with future slots for FFI, image-override, and
//! sprite-atlas renderers.

pub mod null_renderer;
