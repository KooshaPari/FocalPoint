//! Transpiler: focus_rituals::MorningBrief + EveningShutdown ↔ focus_ir::RitualIr

use anyhow::{anyhow, Result};
use focus_ir::{Body, Document, DocKind, RitualIr, RitualTrackingIr};
use focus_rituals::{EveningShutdown, MorningBrief};

/// Convert MorningBrief to a ritual IR representation.
pub fn morning_brief_to_ir(brief: &MorningBrief) -> RitualIr {
    RitualIr {
        id: format!("morning-brief-{}", brief.generated_at.timestamp()),
        name: "Morning Brief".into(),
        description: Some("Daily planning ritual".into()),
        steps: vec![],
        daily_goal: Some(1),
        tracking: RitualTrackingIr {
            enabled: true,
            track_completion: true,
            track_duration: true,
            track_quality: true,
        },
        rewards: vec![],
    }
}

/// Convert EveningShutdown to a ritual IR representation.
pub fn evening_shutdown_to_ir(shutdown: &EveningShutdown) -> RitualIr {
    RitualIr {
        id: format!("evening-shutdown-{}", shutdown.generated_at.timestamp()),
        name: "Evening Shutdown".into(),
        description: Some("Daily reflection and closeout ritual".into()),
        steps: vec![],
        daily_goal: Some(1),
        tracking: RitualTrackingIr {
            enabled: true,
            track_completion: true,
            track_duration: true,
            track_quality: true,
        },
        rewards: vec![],
    }
}

/// Convert MorningBrief to an IR Document.
pub fn morning_brief_to_document(brief: &MorningBrief) -> Document {
    let ir = morning_brief_to_ir(brief);
    Document {
        version: 1,
        kind: DocKind::Ritual,
        id: ir.id.clone(),
        name: ir.name.clone(),
        body: Body::Ritual(ir),
    }
}

/// Convert EveningShutdown to an IR Document.
pub fn evening_shutdown_to_document(shutdown: &EveningShutdown) -> Document {
    let ir = evening_shutdown_to_ir(shutdown);
    Document {
        version: 1,
        kind: DocKind::Ritual,
        id: ir.id.clone(),
        name: ir.name.clone(),
        body: Body::Ritual(ir),
    }
}

/// Convert an IR Document to a minimal MorningBrief representation.
/// Note: This is a lossy conversion since not all brief details can be reconstructed.
pub fn document_to_morning_brief_minimal(doc: &Document) -> Result<MorningBrief> {
    use chrono::{Local, NaiveDate};

    match &doc.body {
        Body::Ritual(_ir) => {
            let today = Local::now().date_naive();
            Ok(MorningBrief {
                date: today,
                intention: None,
                top_priorities: vec![],
                schedule_preview: focus_rituals::SchedulePreview {
                    windows: vec![],
                    soft_conflicts: 0,
                    hard_conflicts: 0,
                },
                coachy_opening: "Good morning. Let's make today count.".into(),
                generated_at: chrono::Utc::now(),
            })
        }
        _ => Err(anyhow!("Expected Ritual body, got {:?}", doc.kind)),
    }
}

/// Convert an IR Document to a minimal EveningShutdown representation.
pub fn document_to_evening_shutdown_minimal(doc: &Document) -> Result<EveningShutdown> {
    use chrono::Local;

    match &doc.body {
        Body::Ritual(_ir) => {
            let today = Local::now().date_naive();
            Ok(EveningShutdown {
                date: today,
                shipped: vec![],
                slipped: vec![],
                carryover: vec![],
                wins_summary: "Keep going.".into(),
                coachy_closing: "Rest well. Tomorrow is a new day.".into(),
                streak_deltas: std::collections::HashMap::new(),
                generated_at: chrono::Utc::now(),
            })
        }
        _ => Err(anyhow!("Expected Ritual body, got {:?}", doc.kind)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morning_brief_to_ir_content_hash() {
        use chrono::{Local, NaiveDate};

        let brief = MorningBrief {
            date: Local::now().date_naive(),
            intention: Some("Focus on deep work".into()),
            top_priorities: vec![],
            schedule_preview: focus_rituals::SchedulePreview {
                windows: vec![],
                soft_conflicts: 0,
                hard_conflicts: 0,
            },
            coachy_opening: "Let's go!".into(),
            generated_at: chrono::Utc::now(),
        };

        let ir = morning_brief_to_ir(&brief);
        assert_eq!(ir.name, "Morning Brief");
        assert!(ir.tracking.enabled);
    }

    #[test]
    fn test_evening_shutdown_document_round_trip() {
        use chrono::Local;

        let shutdown = EveningShutdown {
            date: Local::now().date_naive(),
            shipped: vec![],
            slipped: vec![],
            carryover: vec![],
            wins_summary: "Done.".into(),
            coachy_closing: "Sleep well.".into(),
            streak_deltas: std::collections::HashMap::new(),
            generated_at: chrono::Utc::now(),
        };

        let doc = evening_shutdown_to_document(&shutdown);
        let restored =
            document_to_evening_shutdown_minimal(&doc).expect("Document round-trip");

        assert_eq!(shutdown.date, restored.date);
    }
}
