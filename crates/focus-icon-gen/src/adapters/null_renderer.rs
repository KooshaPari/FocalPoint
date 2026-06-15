//! No-op [`RendererPort`] adapter for unit tests.
//!
//! `NullRenderer` satisfies the renderer port without performing any
//! pixel work, file I/O, or PNG encoding. It is the recommended stand-in
//! when a test needs a renderer-shaped dependency but does not care
//! about the produced pixels — for example, exercising the icon-build
//! pipeline's size negotiation, manifest generation, or wiring code.
//!
//! ## Behavior
//!
//! - [`render`](NullRenderer::render) returns an empty `Vec<u8>` and
//!   increments the internal call counter.
//! - [`icon_hash`](NullRenderer::icon_hash) returns the SHA-256 of the
//!   empty byte string (`e3b0c442…b855`) — the canonical hash of
//!   "nothing" — so that hash assertions in tests stay stable across
//!   runs.
//! - [`supported_sizes`](NullRenderer::supported_sizes) returns an
//!   empty slice by default; tests that need to exercise the
//!   "render-all-sizes" code path can populate it via
//!   [`NullRenderer::with_sizes`].
//!
//! ## Call tracking
//!
//! Every call to `render` and `icon_hash` is recorded on an atomic
//! counter so tests can assert that a code path actually invoked the
//! port. The counters are public (via [`NullRenderer::render_calls`]
//! and [`NullRenderer::hash_calls`]) to keep the adapter free of test
//! framework dependencies.
//!
//! ## Example
//!
//! ```rust,ignore
//! use focus_icon_gen::adapters::null_renderer::NullRenderer;
//! use focus_icon_gen::ports::RendererPort;
//!
//! let renderer = NullRenderer::new();
//! let png = renderer.render(1024).unwrap();
//! assert!(png.is_empty());
//! assert_eq!(renderer.render_calls(), 1);
//! ```

use std::sync::atomic::{AtomicU64, Ordering};

use crate::ports::RendererPort;

/// No-op renderer adapter. See the [module docs](self) for usage.
#[derive(Debug, Default)]
pub struct NullRenderer {
    render_calls: AtomicU64,
    hash_calls: AtomicU64,
    sizes: &'static [(u32, &'static str)],
}

impl NullRenderer {
    /// Build a `NullRenderer` with no declared sizes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a `NullRenderer` that advertises a specific size set.
    ///
    /// Tests exercising `render_all_sizes` (or any pipeline that
    /// negotiates sizes via the port) can pass a static slice so the
    /// adapter looks like a real one to downstream code.
    pub fn with_sizes(sizes: &'static [(u32, &'static str)]) -> Self {
        Self {
            render_calls: AtomicU64::new(0),
            hash_calls: AtomicU64::new(0),
            sizes,
        }
    }

    /// Total number of times [`render`](RendererPort::render) has been invoked.
    pub fn render_calls(&self) -> u64 {
        self.render_calls.load(Ordering::SeqCst)
    }

    /// Total number of times [`icon_hash`](RendererPort::icon_hash) has been invoked.
    pub fn hash_calls(&self) -> u64 {
        self.hash_calls.load(Ordering::SeqCst)
    }
}

impl RendererPort for NullRenderer {
    /// Returns an empty byte vector and bumps the render counter.
    ///
    /// The returned vector is intentionally not a valid PNG — no-op
    /// adapters are for tests, and any test that cares about pixel
    /// content should swap in a real adapter instead.
    fn render(&self, _size: u32) -> anyhow::Result<Vec<u8>> {
        self.render_calls.fetch_add(1, Ordering::SeqCst);
        Ok(Vec::new())
    }

    /// Returns the SHA-256 of the empty byte string and bumps the
    /// hash counter. Stable across runs so test assertions are
    /// deterministic.
    fn icon_hash(&self, size: u32) -> anyhow::Result<String> {
        self.hash_calls.fetch_add(1, Ordering::SeqCst);
        // Mirror the default trait implementation: hash the (empty)
        // bytes that `render` would have produced for this size. Going
        // through the trait default would double-count the render call,
        // so we inline the hash here.
        use sha2::{Digest, Sha256};
        let png = RendererPort::render(self, size)?;
        let mut hasher = Sha256::new();
        hasher.update(&png);
        Ok(hex::encode(hasher.finalize()))
    }

    /// Returns the size slice supplied at construction time (empty by default).
    fn supported_sizes(&self) -> &'static [(u32, &'static str)] {
        self.sizes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: NullRenderer contract — no-op render returns empty bytes.
    #[test]
    fn render_returns_empty_bytes() {
        let r = NullRenderer::new();
        let png = RendererPort::render(&r, 1024).expect("render must not fail");
        assert!(png.is_empty(), "NullRenderer must return an empty PNG buffer");
        assert_eq!(r.render_calls(), 1, "render call counter must increment");
    }

    // Traces to: NullRenderer contract — icon_hash is stable across calls.
    #[test]
    fn icon_hash_is_stable_empty_hash() {
        let r = NullRenderer::new();
        let h1 = r.icon_hash(512).expect("hash must not fail");
        let h2 = r.icon_hash(512).expect("hash must not fail");
        assert_eq!(h1, h2, "NullRenderer hash must be stable across calls");
        // SHA-256 of the empty byte string — the canonical "no pixels" hash.
        assert_eq!(
            h1,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "NullRenderer must hash the empty input deterministically"
        );
        assert_eq!(r.hash_calls(), 2, "hash call counter must increment");
    }

    // Traces to: NullRenderer contract — supported_sizes is empty by default.
    #[test]
    fn supported_sizes_is_empty_by_default() {
        let r = NullRenderer::new();
        assert!(r.supported_sizes().is_empty());
    }

    // Traces to: NullRenderer contract — with_sizes advertises the provided set.
    #[test]
    fn with_sizes_advertises_supplied_set() {
        static SIZES: &[(u32, &str)] = &[(1024, "1024x1024"), (180, "180x180")];
        let r = NullRenderer::with_sizes(SIZES);
        assert_eq!(r.supported_sizes(), SIZES);

        // render_all_sizes should still produce empty PNGs but respect
        // the advertised size metadata.
        let all = RendererPort::render_all_sizes(&r).expect("render_all_sizes");
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].size, 1024);
        assert_eq!(all[0].name, "1024x1024");
        assert!(all[0].png.is_empty());
        assert_eq!(all[1].size, 180);
        assert_eq!(all[1].name, "180x180");
        assert_eq!(r.render_calls(), 2, "render_all_sizes delegates per-size");
    }

    // Traces to: NullRenderer contract — render is idempotent under repeated calls.
    #[test]
    fn render_is_idempotent_and_counts_calls() {
        let r = NullRenderer::new();
        for _ in 0..5 {
            let png = RendererPort::render(&r, 256).expect("render");
            assert!(png.is_empty());
        }
        assert_eq!(r.render_calls(), 5);
    }
}
