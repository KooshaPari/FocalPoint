//! Append-only audit log with a tamper-evident SHA-256 hash chain.
//!
//! Each [`AuditRecord`] commits to the hash of its predecessor, the first
//! record's `prev_hash` being the literal string `"genesis"`. Hashes are
//! computed over a canonicalized representation of the payload (see
//! [`canonical::canonicalize`]) so that semantically-equal JSON always
//! produces the same digest across runs and platforms.

pub mod canonical;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Mutex;

/// Sentinel `prev_hash` for the first record in a chain.
pub const GENESIS_PREV_HASH: &str = "genesis";

#[derive(Debug, thiserror::Error)]
pub enum ChainError {
    #[error("chain is empty")]
    Empty,
    #[error("hash mismatch at index {index}: expected {expected}, got {actual}")]
    HashMismatch { index: usize, expected: String, actual: String },
    #[error("prev_hash link broken at index {index}")]
    PrevHashBroken { index: usize },
}

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
    /// Compute the SHA-256 hash for a record.
    ///
    /// The payload is canonicalized (recursively sorted object keys) to make
    /// the hash stable regardless of serialization order.
    pub fn compute_hash(
        record_type: &str,
        subject_ref: &str,
        occurred_at: &chrono::DateTime<chrono::Utc>,
        prev_hash: &str,
        payload: &serde_json::Value,
    ) -> String {
        let mut h = Sha256::new();
        h.update(record_type.as_bytes());
        h.update(b"\x00");
        h.update(subject_ref.as_bytes());
        h.update(b"\x00");
        h.update(occurred_at.to_rfc3339().as_bytes());
        h.update(b"\x00");
        h.update(prev_hash.as_bytes());
        h.update(b"\x00");
        h.update(canonical::canonicalize(payload).as_bytes());
        hex::encode(h.finalize())
    }

    /// Recompute this record's hash (for verification).
    pub fn recompute_hash(&self) -> String {
        Self::compute_hash(
            &self.record_type,
            &self.subject_ref,
            &self.occurred_at,
            &self.prev_hash,
            &self.payload,
        )
    }
}

/// In-memory append-only hash chain.
#[derive(Debug, Default, Clone)]
pub struct AuditChain {
    pub records: Vec<AuditRecord>,
}

impl AuditChain {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Return the hash of the tip record, or [`GENESIS_PREV_HASH`] if empty.
    pub fn head_hash(&self) -> &str {
        self.records.last().map(|r| r.hash.as_str()).unwrap_or(GENESIS_PREV_HASH)
    }

    /// Append a new record, computing its `prev_hash` and `hash`.
    pub fn append(
        &mut self,
        record_type: impl Into<String>,
        subject_ref: impl Into<String>,
        payload: serde_json::Value,
        now: chrono::DateTime<chrono::Utc>,
    ) -> AuditRecord {
        let record_type = record_type.into();
        let subject_ref = subject_ref.into();
        let prev_hash = self.head_hash().to_string();
        let hash =
            AuditRecord::compute_hash(&record_type, &subject_ref, &now, &prev_hash, &payload);
        let record = AuditRecord {
            id: uuid::Uuid::new_v4(),
            record_type,
            subject_ref,
            occurred_at: now,
            prev_hash,
            payload,
            hash,
        };
        self.records.push(record.clone());
        record
    }

    /// Walk the chain and verify hash/prev-hash integrity.
    ///
    /// Returns [`ChainError::Empty`] if the chain has no records,
    /// [`ChainError::PrevHashBroken`] if a record's `prev_hash` doesn't match
    /// its predecessor's `hash`, and [`ChainError::HashMismatch`] if a record's
    /// `hash` doesn't match a recomputation from its own fields.
    pub fn verify(&self) -> Result<(), ChainError> {
        if self.records.is_empty() {
            return Err(ChainError::Empty);
        }
        let mut expected_prev = GENESIS_PREV_HASH.to_string();
        for (index, rec) in self.records.iter().enumerate() {
            if rec.prev_hash != expected_prev {
                return Err(ChainError::PrevHashBroken { index });
            }
            let recomputed = rec.recompute_hash();
            if recomputed != rec.hash {
                return Err(ChainError::HashMismatch {
                    index,
                    expected: recomputed,
                    actual: rec.hash.clone(),
                });
            }
            expected_prev = rec.hash.clone();
        }
        Ok(())
    }
}

/// Storage abstraction for audit records. SQLite-backed implementations live
/// in `focus-storage`; this crate provides an in-memory reference impl.
pub trait AuditStore: Send + Sync {
    fn append(&self, record: AuditRecord) -> anyhow::Result<()>;
    fn verify_chain(&self) -> anyhow::Result<bool>;
    /// Return the tip hash (or `None` if the chain is empty).
    fn head_hash(&self) -> anyhow::Result<Option<String>>;
}

/// In-memory [`AuditStore`] backed by an [`AuditChain`].
#[derive(Debug, Default)]
pub struct InMemoryAuditStore {
    pub chain: Mutex<AuditChain>,
}

impl InMemoryAuditStore {
    pub fn new() -> Self {
        Self { chain: Mutex::new(AuditChain::new()) }
    }
}

impl AuditStore for InMemoryAuditStore {
    fn append(&self, record: AuditRecord) -> anyhow::Result<()> {
        let mut chain =
            self.chain.lock().map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
        // Caller-constructed record; trust-but-verify its prev_hash link.
        let expected_prev = chain.head_hash().to_string();
        if record.prev_hash != expected_prev {
            anyhow::bail!(
                "prev_hash mismatch on append: expected {expected_prev}, got {}",
                record.prev_hash
            );
        }
        chain.records.push(record);
        Ok(())
    }

    fn verify_chain(&self) -> anyhow::Result<bool> {
        let chain =
            self.chain.lock().map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
        match chain.verify() {
            Ok(()) => Ok(true),
            Err(ChainError::Empty) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn head_hash(&self) -> anyhow::Result<Option<String>> {
        let chain =
            self.chain.lock().map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
        Ok(if chain.is_empty() { None } else { Some(chain.head_hash().to_string()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ts(n: i64) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(1_700_000_000 + n, 0).unwrap()
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn empty_chain_verify_returns_empty() {
        let chain = AuditChain::new();
        assert!(chain.is_empty());
        assert!(matches!(chain.verify(), Err(ChainError::Empty)));
        assert_eq!(chain.head_hash(), GENESIS_PREV_HASH);
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn single_record_chain_verifies() {
        let mut chain = AuditChain::new();
        let rec = chain.append("test", "subject-1", json!({"v": 1}), ts(0));
        assert_eq!(rec.prev_hash, GENESIS_PREV_HASH);
        chain.verify().expect("single-record chain should verify");
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn hundred_record_chain_builds_and_verifies() {
        let mut chain = AuditChain::new();
        for i in 0..100 {
            chain.append("evt", format!("s-{i}"), json!({"i": i}), ts(i as i64));
        }
        assert_eq!(chain.len(), 100);
        chain.verify().expect("100-record chain should verify");
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn tamper_detection_via_payload_mutation() {
        let mut chain = AuditChain::new();
        for i in 0..5 {
            chain.append("evt", "s", json!({"i": i}), ts(i));
        }
        // Mutate record at index 2's payload after the hash was sealed.
        chain.records[2].payload = json!({"i": 999});
        match chain.verify() {
            Err(ChainError::HashMismatch { index, .. }) => assert_eq!(index, 2),
            other => panic!("expected HashMismatch at 2, got {other:?}"),
        }
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn prev_hash_break_detected() {
        let mut chain = AuditChain::new();
        for i in 0..3 {
            chain.append("evt", "s", json!({"i": i}), ts(i));
        }
        // Splice a bogus prev_hash at index 1.
        chain.records[1].prev_hash = "not-the-real-prev".to_string();
        match chain.verify() {
            Err(ChainError::PrevHashBroken { index }) => assert_eq!(index, 1),
            other => panic!("expected PrevHashBroken at 1, got {other:?}"),
        }
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn canonicalization_makes_hash_key_order_independent() {
        let a = json!({"a": 1, "b": 2});
        let b = json!({"b": 2, "a": 1});
        let t = ts(0);
        let ha = AuditRecord::compute_hash("t", "s", &t, GENESIS_PREV_HASH, &a);
        let hb = AuditRecord::compute_hash("t", "s", &t, GENESIS_PREV_HASH, &b);
        assert_eq!(ha, hb);
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn compute_hash_is_deterministic_across_calls() {
        let payload = json!({"nested": {"z": 1, "a": [1, 2, {"y": 9, "x": 8}]}, "k": "v"});
        let t = ts(42);
        let h1 = AuditRecord::compute_hash("type", "subj", &t, "prev", &payload);
        let h2 = AuditRecord::compute_hash("type", "subj", &t, "prev", &payload);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64); // hex-encoded SHA-256
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn in_memory_store_append_and_head_hash() {
        let store = InMemoryAuditStore::new();
        assert_eq!(store.head_hash().unwrap(), None);

        let mut chain = AuditChain::new();
        let rec = chain.append("t", "s", json!({"x": 1}), ts(0));
        store.append(rec.clone()).expect("append should succeed");

        assert_eq!(store.head_hash().unwrap(), Some(rec.hash.clone()));
        assert!(store.verify_chain().unwrap());

        // Bad prev_hash on a subsequent append is rejected.
        let bogus = AuditRecord {
            id: uuid::Uuid::new_v4(),
            record_type: "t".into(),
            subject_ref: "s".into(),
            occurred_at: ts(1),
            prev_hash: "wrong".into(),
            payload: json!({}),
            hash: "irrelevant".into(),
        };
        assert!(store.append(bogus).is_err());
    }

    // Traces to: FR-DATA-002, FR-DATA-003
    #[test]
    fn head_hash_advances_with_each_append() {
        let mut chain = AuditChain::new();
        let r1 = chain.append("t", "s", json!({"i": 1}), ts(1));
        assert_eq!(chain.head_hash(), r1.hash);
        let r2 = chain.append("t", "s", json!({"i": 2}), ts(2));
        assert_eq!(chain.head_hash(), r2.hash);
        assert_eq!(r2.prev_hash, r1.hash);
    }
}
