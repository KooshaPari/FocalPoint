use phenotype_error_core::PhenotypeError;
use sha2::{Digest, Sha256};

/// Compute a hash for an event given its aggregate ID, sequence, payload, and previous hash.
pub fn compute_hash(
    aggregate_id: &str,
    sequence: i64,
    payload: &str,
    previous_hash: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(aggregate_id.as_bytes());
    hasher.update(sequence.to_be_bytes());
    hasher.update(payload.as_bytes());
    hasher.update(previous_hash.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify the integrity of a hash chain.
pub fn verify_chain(events: &[(String, String)]) -> Result<(), PhenotypeError> {
    let mut previous_hash = String::new();
    for (i, (hash, payload)) in events.iter().enumerate() {
        let expected = compute_hash("aggregate", i as i64, payload, &previous_hash);
        if hash != &expected {
            return Err(PhenotypeError::event_sourcing(format!(
                "Chain broken at sequence {i}: expected {expected}, got {hash}"
            )));
        }
        previous_hash = hash.clone();
    }
    Ok(())
}

/// Detect gaps in a sequence of event numbers.
pub fn detect_gaps(sequences: &[i64]) -> Option<i64> {
    if sequences.is_empty() {
        return None;
    }
    let mut sorted = sequences.to_vec();
    sorted.sort_unstable();
    for window in sorted.windows(2) {
        if window[1] - window[0] > 1 {
            return Some(window[0] + 1);
        }
    }
    None
}
