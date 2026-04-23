//! Integration test: drives FocalPointCore through a sequence of events and
//! verifies the FFI-facing state reflects the core mascot machine.

use focus_ffi::{Emotion, FocalPointCore, MascotEvent, Pose};

#[test]
fn focus_session_completion_celebrates() {
    let core = FocalPointCore::new();
    let s = core.push_mascot_event(MascotEvent::FocusSessionCompleted { minutes: 50 });
    assert!(matches!(s.pose, Pose::Celebratory));
    assert!(matches!(s.emotion, Emotion::Excited));
    assert!(s.bubble_text.is_some());
    assert!(!s.since_iso.is_empty());
}

#[test]
fn event_sequence_updates_state_in_place() {
    let core = FocalPointCore::new();

    let _ = core.push_mascot_event(MascotEvent::DailyCheckIn);
    let cur = core.mascot_state();
    assert!(matches!(cur.pose, Pose::Confident));

    let s = core.push_mascot_event(MascotEvent::SleepDebtReported { hours: 4.0 });
    assert!(matches!(s.pose, Pose::SleepyDisappointed));
    assert!(matches!(s.emotion, Emotion::Tired));

    let latest = core.mascot_state();
    assert!(matches!(latest.pose, Pose::SleepyDisappointed));
}
