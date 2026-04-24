//! Transpiler: focus_rewards::WalletMutation ↔ focus_ir::WalletMutationIr

use anyhow::{anyhow, Result};
use focus_ir::{Body, Document, DocKind, MutationOpIr, WalletMutationIr};
use focus_rewards::WalletMutation;
use uuid::Uuid;

/// Convert focus_rewards::WalletMutation to focus_ir::WalletMutationIr.
pub fn mutation_to_ir(mutation: &WalletMutation) -> WalletMutationIr {
    let (operation, wallet_type) = match mutation {
        WalletMutation::GrantCredit(_) => (MutationOpIr::Add, "points"),
        WalletMutation::SpendCredit { .. } => (MutationOpIr::Subtract, "points"),
        WalletMutation::StreakIncrement(_) => (MutationOpIr::Add, "streak"),
        WalletMutation::StreakReset(_) => (MutationOpIr::Set, "streak"),
        WalletMutation::SetMultiplier(_) => (MutationOpIr::Multiply, "multiplier"),
    };

    let (name, amount, reason) = match mutation {
        WalletMutation::GrantCredit(c) => {
            ("grant_credit".into(), c.amount, "Credit grant".into())
        }
        WalletMutation::SpendCredit { amount, purpose } => {
            ("spend_credit".into(), -*amount, purpose.clone())
        }
        WalletMutation::StreakIncrement(name) => {
            ("streak_increment".into(), 1, format!("Streak: {}", name))
        }
        WalletMutation::StreakReset(name) => {
            ("streak_reset".into(), 0, format!("Reset: {}", name))
        }
        WalletMutation::SetMultiplier(m) => {
            ("set_multiplier".into(), m.current as i64, "Multiplier".into())
        }
    };

    WalletMutationIr {
        id: Uuid::new_v4().to_string(),
        name,
        wallet_type: wallet_type.into(),
        operation,
        amount,
        reason,
        conditional: None,
    }
}

/// Convert focus_ir::WalletMutationIr back to focus_rewards::WalletMutation.
/// Note: This is a lossy conversion since WalletMutation enum doesn't capture all IR details.
pub fn ir_to_mutation(ir: &WalletMutationIr) -> Result<WalletMutation> {
    use focus_rewards::Credit;

    let mutation = match (ir.operation, ir.wallet_type.as_str()) {
        (MutationOpIr::Add, "points") => WalletMutation::GrantCredit(Credit {
            amount: ir.amount,
            source_rule_id: None,
            granted_at: chrono::Utc::now(),
        }),
        (MutationOpIr::Subtract, "points") => WalletMutation::SpendCredit {
            amount: ir.amount.abs(),
            purpose: ir.reason.clone(),
        },
        (MutationOpIr::Add, "streak") => {
            WalletMutation::StreakIncrement(extract_streak_name(&ir.reason))
        }
        (MutationOpIr::Set, "streak") => {
            WalletMutation::StreakReset(extract_streak_name(&ir.reason))
        }
        (MutationOpIr::Multiply, "multiplier") => WalletMutation::SetMultiplier(
            focus_rewards::MultiplierState {
                current: ir.amount as f32 / 100.0,
                expires_at: None,
            },
        ),
        _ => {
            return Err(anyhow!(
                "Cannot reconstruct WalletMutation from operation {:?} on type {}",
                ir.operation,
                ir.wallet_type
            ))
        }
    };

    Ok(mutation)
}

fn extract_streak_name(reason: &str) -> String {
    reason
        .strip_prefix("Streak: ")
        .or_else(|| reason.strip_prefix("Reset: "))
        .unwrap_or(reason)
        .into()
}

/// Convert WalletMutation to an IR Document.
pub fn mutation_to_document(mutation: &WalletMutation) -> Document {
    let ir = mutation_to_ir(mutation);
    Document {
        version: 1,
        kind: DocKind::WalletMutation,
        id: ir.id.clone(),
        name: ir.name.clone(),
        body: Body::WalletMutation(ir),
    }
}

/// Convert an IR Document back to a WalletMutation.
pub fn document_to_mutation(doc: &Document) -> Result<WalletMutation> {
    match &doc.body {
        Body::WalletMutation(ir) => ir_to_mutation(ir),
        _ => Err(anyhow!("Expected WalletMutation body, got {:?}", doc.kind)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_credit_round_trip() {
        let mutation = WalletMutation::GrantCredit(focus_rewards::Credit {
            amount: 100,
            source_rule_id: None,
            granted_at: chrono::Utc::now(),
        });

        let ir = mutation_to_ir(&mutation);
        let restored = ir_to_mutation(&ir).expect("Round-trip");

        match (mutation, restored) {
            (
                WalletMutation::GrantCredit(orig),
                WalletMutation::GrantCredit(rest),
            ) => {
                assert_eq!(orig.amount, rest.amount);
            }
            _ => panic!("Type mismatch"),
        }
    }

    #[test]
    fn test_spend_credit_round_trip() {
        let mutation = WalletMutation::SpendCredit {
            amount: 50,
            purpose: "unlock".into(),
        };

        let ir = mutation_to_ir(&mutation);
        let restored = ir_to_mutation(&ir).expect("Round-trip");

        match (mutation, restored) {
            (
                WalletMutation::SpendCredit {
                    amount: orig_amt,
                    purpose: orig_purp,
                },
                WalletMutation::SpendCredit {
                    amount: rest_amt,
                    purpose: rest_purp,
                },
            ) => {
                assert_eq!(orig_amt, rest_amt);
                assert_eq!(orig_purp, rest_purp);
            }
            _ => panic!("Type mismatch"),
        }
    }

    #[test]
    fn test_document_round_trip() {
        let mutation = WalletMutation::GrantCredit(focus_rewards::Credit {
            amount: 200,
            source_rule_id: None,
            granted_at: chrono::Utc::now(),
        });

        let doc = mutation_to_document(&mutation);
        let restored = document_to_mutation(&doc).expect("Document round-trip");

        match (mutation, restored) {
            (
                WalletMutation::GrantCredit(orig),
                WalletMutation::GrantCredit(rest),
            ) => {
                assert_eq!(orig.amount, rest.amount);
            }
            _ => panic!("Type mismatch"),
        }
    }
}
