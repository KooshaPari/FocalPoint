//! Unlock session + proof validation (QR / NFC / Manual).
//!
//! Traces to: FR-ENF-006.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnlockProofType {
    Qr(String),
    Nfc(Vec<u8>),
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnlockOutcome {
    Pending,
    Validated,
    Rejected(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockSession {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub proof_type: UnlockProofType,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub outcome: UnlockOutcome,
}

pub struct UnlockValidator;

impl UnlockValidator {
    /// Validate a QR payload. Payload must start with `expected_prefix`.
    /// Traces to: FR-ENF-006.
    pub fn validate_qr(payload: &str, expected_prefix: &str, _now: DateTime<Utc>) -> UnlockOutcome {
        if expected_prefix.is_empty() {
            return UnlockOutcome::Rejected("empty expected prefix".into());
        }
        if payload.starts_with(expected_prefix) {
            UnlockOutcome::Validated
        } else {
            UnlockOutcome::Rejected("qr prefix mismatch".into())
        }
    }

    /// Validate an NFC payload. Payload must start with `expected_magic`.
    /// Traces to: FR-ENF-006.
    pub fn validate_nfc(payload: &[u8], expected_magic: &[u8]) -> UnlockOutcome {
        if expected_magic.is_empty() {
            return UnlockOutcome::Rejected("empty expected magic".into());
        }
        if payload.starts_with(expected_magic) {
            UnlockOutcome::Validated
        } else {
            UnlockOutcome::Rejected("nfc magic mismatch".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()
    }

    // Traces to: FR-ENF-006
    #[test]
    fn qr_valid() {
        let out = UnlockValidator::validate_qr("focalpoint:abc123", "focalpoint:", now());
        assert_eq!(out, UnlockOutcome::Validated);
    }

    // Traces to: FR-ENF-006
    #[test]
    fn qr_rejected() {
        let out = UnlockValidator::validate_qr("evil:abc123", "focalpoint:", now());
        assert!(matches!(out, UnlockOutcome::Rejected(_)));
    }

    // Traces to: FR-ENF-006
    #[test]
    fn nfc_valid() {
        let out = UnlockValidator::validate_nfc(b"FPtag-data", b"FP");
        assert_eq!(out, UnlockOutcome::Validated);
    }

    // Traces to: FR-ENF-006
    #[test]
    fn nfc_rejected() {
        let out = UnlockValidator::validate_nfc(b"junk", b"FP");
        assert!(matches!(out, UnlockOutcome::Rejected(_)));
    }
}
