//! Append-only audit log, tamper-evident hash chain.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub id: uuid::Uuid,
    pub record_type: String,
    pub subject_ref: String,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub prev_hash: String,
    pub payload: serde_json::Value,
    pub hash: String,
}

impl AuditRecord {
    pub fn compute_hash(
        record_type: &str,
        subject_ref: &str,
        occurred_at: &chrono::DateTime<chrono::Utc>,
        prev_hash: &str,
        payload: &serde_json::Value,
    ) -> String {
        let mut h = Sha256::new();
        h.update(record_type.as_bytes());
        h.update(subject_ref.as_bytes());
        h.update(occurred_at.to_rfc3339().as_bytes());
        h.update(prev_hash.as_bytes());
        h.update(payload.to_string().as_bytes());
        hex::encode(h.finalize())
    }
}

pub trait AuditStore: Send + Sync {
    fn append(&self, record: AuditRecord) -> anyhow::Result<()>;
    fn verify_chain(&self) -> anyhow::Result<bool>;
}

pub struct AuditChain;
