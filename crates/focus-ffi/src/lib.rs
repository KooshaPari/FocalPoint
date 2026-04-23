//! UniFFI/JNI export surface.
//!
//! Stub. Real UDL and bindings come in Phase 1 when iOS/Android shells wire up.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FfiError {
    #[error("not implemented")]
    NotImplemented,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("domain: {0}")]
    Domain(String),
}

pub struct FocalPointCore {
    // Stub: will hold router, stores, and configured clock.
}

impl FocalPointCore {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FocalPointCore {
    fn default() -> Self {
        Self::new()
    }
}
