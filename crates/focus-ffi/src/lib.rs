//! UniFFI export surface for FocalPoint core.
//!
//! Exposes the mascot state machine (and, over time, rules/rewards/audit) to
//! Swift (via UniFFI) and Kotlin (via UniFFI-Kotlin / JNI) using a single UDL.
//!
//! The scaffolding file is generated at build time by `uniffi_build` and
//! included here via `include_scaffolding!`.

use std::sync::Mutex;

use focus_mascot::{
    Emotion as CoreEmotion, MascotEvent as CoreMascotEvent, MascotMachine,
    MascotState as CoreMascotState, Pose as CorePose,
};
use thiserror::Error;

uniffi::include_scaffolding!("focus_ffi");

// ---- Errors ---------------------------------------------------------------

#[derive(Debug, Error)]
pub enum FfiError {
    #[error("not implemented")]
    NotImplemented,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("domain: {0}")]
    Domain(String),
}

// ---- FFI-facing types -----------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pose {
    Confident,
    Encouraging,
    CuriousThinking,
    SternToughLove,
    Celebratory,
    SleepyDisappointed,
    Idle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emotion {
    Neutral,
    Happy,
    Proud,
    Concerned,
    Stern,
    Excited,
    Tired,
    Warm,
}

#[derive(Debug, Clone)]
pub struct MascotState {
    pub pose: Pose,
    pub emotion: Emotion,
    pub since_iso: String,
    pub bubble_text: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MascotEvent {
    RuleFired { rule_name: String },
    StreakIncremented { name: String, count: u32 },
    StreakReset { name: String },
    CreditEarned { amount: i64 },
    BypassSpent { remaining: i64 },
    PenaltyEscalated { tier: String },
    AppLaunchedWhileBlocked { bundle_id: String },
    FocusSessionStarted { minutes: u32 },
    FocusSessionCompleted { minutes: u32 },
    DailyCheckIn,
    SleepDebtReported { hours: f32 },
    Idle,
}

// ---- Conversions ----------------------------------------------------------

impl From<CorePose> for Pose {
    fn from(value: CorePose) -> Self {
        match value {
            CorePose::Confident => Pose::Confident,
            CorePose::Encouraging => Pose::Encouraging,
            CorePose::CuriousThinking => Pose::CuriousThinking,
            CorePose::SternToughLove => Pose::SternToughLove,
            CorePose::Celebratory => Pose::Celebratory,
            CorePose::SleepyDisappointed => Pose::SleepyDisappointed,
            CorePose::Idle => Pose::Idle,
        }
    }
}

impl From<CoreEmotion> for Emotion {
    fn from(value: CoreEmotion) -> Self {
        match value {
            CoreEmotion::Neutral => Emotion::Neutral,
            CoreEmotion::Happy => Emotion::Happy,
            CoreEmotion::Proud => Emotion::Proud,
            CoreEmotion::Concerned => Emotion::Concerned,
            CoreEmotion::Stern => Emotion::Stern,
            CoreEmotion::Excited => Emotion::Excited,
            CoreEmotion::Tired => Emotion::Tired,
            CoreEmotion::Warm => Emotion::Warm,
        }
    }
}

impl From<&CoreMascotState> for MascotState {
    fn from(s: &CoreMascotState) -> Self {
        MascotState {
            pose: s.pose.into(),
            emotion: s.emotion.into(),
            since_iso: s.since.to_rfc3339(),
            bubble_text: s.bubble_text.clone(),
        }
    }
}

impl From<MascotEvent> for CoreMascotEvent {
    fn from(e: MascotEvent) -> Self {
        match e {
            MascotEvent::RuleFired { rule_name } => CoreMascotEvent::RuleFired { rule_name },
            MascotEvent::StreakIncremented { name, count } => {
                CoreMascotEvent::StreakIncremented { name, count }
            }
            MascotEvent::StreakReset { name } => CoreMascotEvent::StreakReset(name),
            MascotEvent::CreditEarned { amount } => CoreMascotEvent::CreditEarned { amount },
            MascotEvent::BypassSpent { remaining } => CoreMascotEvent::BypassSpent { remaining },
            MascotEvent::PenaltyEscalated { tier } => CoreMascotEvent::PenaltyEscalated { tier },
            MascotEvent::AppLaunchedWhileBlocked { bundle_id } => {
                CoreMascotEvent::AppLaunchedWhileBlocked { bundle_id }
            }
            MascotEvent::FocusSessionStarted { minutes } => {
                CoreMascotEvent::FocusSessionStarted { minutes }
            }
            MascotEvent::FocusSessionCompleted { minutes } => {
                CoreMascotEvent::FocusSessionCompleted { minutes }
            }
            MascotEvent::DailyCheckIn => CoreMascotEvent::DailyCheckIn,
            MascotEvent::SleepDebtReported { hours } => {
                CoreMascotEvent::SleepDebtReported { hours }
            }
            MascotEvent::Idle => CoreMascotEvent::Idle,
        }
    }
}

// ---- Core object ----------------------------------------------------------

pub struct FocalPointCore {
    mascot: Mutex<MascotMachine>,
}

impl FocalPointCore {
    pub fn new() -> Self {
        Self {
            mascot: Mutex::new(MascotMachine::new()),
        }
    }

    pub fn push_mascot_event(&self, event: MascotEvent) -> MascotState {
        let mut machine = self.mascot.lock().expect("mascot mutex poisoned");
        let core_event: CoreMascotEvent = event.into();
        let next = machine.on_event(core_event);
        MascotState::from(&*next)
    }

    pub fn mascot_state(&self) -> MascotState {
        let machine = self.mascot.lock().expect("mascot mutex poisoned");
        MascotState::from(&machine.state)
    }

    pub fn app_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

impl Default for FocalPointCore {
    fn default() -> Self {
        Self::new()
    }
}

// ---- Tests ----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_core_starts_idle() {
        let core = FocalPointCore::new();
        let s = core.mascot_state();
        assert!(matches!(s.pose, Pose::Idle));
        assert!(matches!(s.emotion, Emotion::Neutral));
    }

    #[test]
    fn push_idle_event_stays_idle() {
        let core = FocalPointCore::new();
        let s = core.push_mascot_event(MascotEvent::Idle);
        assert!(matches!(s.pose, Pose::Idle));
    }

    #[test]
    fn streak_routes_to_encouraging_proud() {
        let core = FocalPointCore::new();
        let s = core.push_mascot_event(MascotEvent::StreakIncremented {
            name: "study".into(),
            count: 3,
        });
        assert!(matches!(s.pose, Pose::Encouraging));
        assert!(matches!(s.emotion, Emotion::Proud));
    }

    #[test]
    fn app_version_matches_cargo() {
        let core = FocalPointCore::new();
        assert_eq!(core.app_version(), env!("CARGO_PKG_VERSION"));
    }
}
