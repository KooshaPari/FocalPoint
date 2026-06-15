//! Renderer port — abstract interface for icon-rendering adapters.
//!
//! Adapters implementing [`RendererPort`] produce PNG-encoded icons at a
//! requested pixel size. The port is intentionally minimal so that
//! production adapters (procedural pixel generation, image override
//! loading, FFI delegations, etc.) and test doubles can all satisfy the
//! same contract.
//!
//! ## Hexagonal layout
//!
//! - This file defines the **port**.
//! - `crate::adapters::null_renderer::NullRenderer` is a no-op adapter
//!   intended for unit tests that need a renderer implementation
//!   without the cost (or side effects) of actually drawing pixels.
//! - `crate::IconGenerator` is the default production adapter that
//!   renders the procedural Coachy flame silhouette.

use anyhow::Result;

/// One rendered icon: pixel size, logical name, and PNG-encoded bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedIcon {
    /// Pixel size of the icon (e.g. `1024` for a 1024×1024 asset).
    pub size: u32,
    /// Human-readable name (e.g. `"1024x1024"`, `"180x180"`).
    pub name: &'static str,
    /// PNG-encoded image data.
    pub png: Vec<u8>,
}

/// Port trait for icon-rendering adapters.
///
/// Implementors produce deterministic PNG bytes for a requested pixel
/// size. The trait is `Send + Sync` so adapters can be stored behind
/// `Arc<dyn RendererPort>` in concurrent code paths (build scripts,
/// MCP servers, the FFI bridge, etc.).
///
/// ## No-op semantics
///
/// A no-op implementation (see `NullRenderer`) must return an empty
/// `Vec<u8>` from [`render`](Self::render) and the SHA-256 of the empty
/// byte string from [`icon_hash`](Self::icon_hash) so that callers can
/// rely on the value being stable across runs.
pub trait RendererPort: Send + Sync {
    /// Render an icon at the given pixel size.
    ///
    /// Returns PNG-encoded image bytes on success, or an error if the
    /// adapter cannot produce a valid image (e.g. invalid size, I/O
    /// failure on an override asset, etc.).
    fn render(&self, size: u32) -> Result<Vec<u8>>;

    /// Compute a stable identifier for the icon at the given size.
    ///
    /// The default implementation derives a SHA-256 hash of the PNG
    /// bytes returned by [`render`](Self::render). Adapters may
    /// override this to short-circuit the encode-then-hash round trip
    /// when they already have a deterministic source (e.g. a procedural
    /// pixel buffer).
    fn icon_hash(&self, size: u32) -> Result<String> {
        use sha2::{Digest, Sha256};
        let png = self.render(size)?;
        let mut hasher = Sha256::new();
        hasher.update(&png);
        Ok(hex::encode(hasher.finalize()))
    }

    /// The set of `(size, name)` pairs this adapter is able to produce.
    ///
    /// Adapters that cannot pre-declare their size set (e.g. ones that
    /// read override assets at runtime) may return an empty slice; the
    /// default is an empty list.
    fn supported_sizes(&self) -> &'static [(u32, &'static str)] {
        &[]
    }

    /// Render every size listed by [`supported_sizes`](Self::supported_sizes).
    ///
    /// The default implementation iterates the declared sizes and
    /// delegates to [`render`](Self::render) for each one. Adapters that
    /// need custom batching (e.g. writing a single sprite atlas) may
    /// override this method.
    fn render_all_sizes(&self) -> Result<Vec<RenderedIcon>> {
        let sizes = self.supported_sizes();
        let mut out = Vec::with_capacity(sizes.len());
        for &(size, name) in sizes {
            out.push(RenderedIcon {
                size,
                name,
                png: self.render(size)?,
            });
        }
        Ok(out)
    }
}
