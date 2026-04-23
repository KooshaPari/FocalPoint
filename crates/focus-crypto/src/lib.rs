//! Token wrapping, secure-storage helpers.

pub mod unlock;
pub use unlock::*;

use secrecy::{ExposeSecret, SecretString};

pub trait SecureSecretStore: Send + Sync {
    fn store(&self, key: &str, value: SecretString) -> anyhow::Result<()>;
    fn load(&self, key: &str) -> anyhow::Result<Option<SecretString>>;
    fn delete(&self, key: &str) -> anyhow::Result<()>;
}

pub struct TokenWrap {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl TokenWrap {
    pub fn new(_key: &SecretString, _plaintext: &[u8]) -> anyhow::Result<Self> {
        // Stub: AEAD via ring::aead when implemented.
        let _ = _key.expose_secret();
        anyhow::bail!("TokenWrap::new not implemented")
    }
}

pub struct IntegrityDigest(pub [u8; 32]);
