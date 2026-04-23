//! Coachy — the FocalPoint mascot state machine.
//!
//! Coachy is a fiery flame-shaped coach with a red cape + gold-star buckle. The
//! Rust side owns the *logical* state (pose × emotion × bubble copy). The Swift
//! layer binds this to Spline scenes to render transitions.
//!
//! The character image is fixed; the palette tokens live in
//! `docs/reference/design_tokens.md` under "Mascot asset tokens (Coachy)."
//!
//! LLM-driven bubble text generation is deferred; MVP uses the static copy
//! returned by `MascotState::default_bubble_for()`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Brand name of the mascot. Used in UI copy + accessibility labels.
pub const MASCOT_NAME: &str = "Coachy";

/// Coarse-grained pose category. Each corresponds to one Spline scene.
///
/// Matches the six emotions in the approved Coachy key art:
/// Confident (hero pose) · Encouraging · Curious/Thinking · Stern/Tough-Love ·
/// Celebratory · Sleepy/Disappointed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pose {
    /// Default hero pose — arms crossed confidently or finger raised ("You can do harder things")
    Confident,
    /// Thumbs up, sparkles, supportive ("You've got this!")
    Encouraging,
    /// Finger on chin, question mark, contemplative
    CuriousThinking,
    /// Arms crossed, eyebrows furrowed ("Focus. No shortcuts.")
    SternToughLove,
    /// Arms up, confetti, celebrating ("Task complete! Let's go!")
    Celebratory,
    /// Slumped, zzz's, low battery ("Rest up. Tomorrow's a win.")
    SleepyDisappointed,
    /// Soft idle state between events
    Idle,
}

/// Fine-grained emotional tint layered over a pose. Drives eye shape, mouth
/// curve, head tilt in the Spline scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Event that may trigger a mascot transition. Platform-agnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MascotEvent {
    RuleFired { rule_name: String },
    StreakIncremented { name: String, count: u32 },
    StreakReset(String),
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

/// Full mascot state pushed across FFI to the Swift renderer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MascotState {
    pub pose: Pose,
    pub emotion: Emotion,
    pub since: DateTime<Utc>,
    pub bubble_text: Option<String>,
}

impl MascotState {
    pub fn new(pose: Pose, emotion: Emotion, bubble: Option<String>) -> Self {
        Self {
            pose,
            emotion,
            since: Utc::now(),
            bubble_text: bubble,
        }
    }

    /// MVP copy bank — deterministic strings per pose. Swap for LLM later.
    pub fn default_bubble_for(pose: Pose) -> &'static str {
        match pose {
            Pose::Confident => "You can do harder things.",
            Pose::Encouraging => "You've got this!",
            Pose::CuriousThinking => "Let's figure it out.",
            Pose::SternToughLove => "Focus. No shortcuts.",
            Pose::Celebratory => "Task complete! Let's go!",
            Pose::SleepyDisappointed => "Rest up. Tomorrow's a win.",
            Pose::Idle => "Finish one task, earn a break.",
        }
    }
}

impl Default for MascotState {
    fn default() -> Self {
        Self::new(Pose::Idle, Emotion::Neutral, Some(Self::default_bubble_for(Pose::Idle).into()))
    }
}

/// Platform-binding trait: the Swift Spline renderer implements this so every
/// Rust-side state transition reaches the animation runtime.
pub trait MascotDriver: Send + Sync {
    fn apply(&self, state: &MascotState);
}

/// Minimal transition table. Designed for extension, not completeness.
pub struct MascotMachine {
    pub state: MascotState,
}

impl MascotMachine {
    pub fn new() -> Self {
        Self {
            state: MascotState::default(),
        }
    }

    /// Apply an event → next state. Pure function of (current state, event).
    pub fn on_event(&mut self, event: MascotEvent) -> &MascotState {
        let (pose, emotion) = match event {
            MascotEvent::RuleFired { .. } => (Pose::SternToughLove, Emotion::Stern),
            MascotEvent::StreakIncremented { .. } => (Pose::Encouraging, Emotion::Proud),
            MascotEvent::StreakReset(_) => (Pose::SleepyDisappointed, Emotion::Concerned),
            MascotEvent::CreditEarned { amount } if amount >= 10 => (Pose::Celebratory, Emotion::Excited),
            MascotEvent::CreditEarned { .. } => (Pose::Encouraging, Emotion::Happy),
            MascotEvent::BypassSpent { remaining } if remaining <= 0 => (Pose::SternToughLove, Emotion::Stern),
            MascotEvent::BypassSpent { .. } => (Pose::CuriousThinking, Emotion::Concerned),
            MascotEvent::PenaltyEscalated { .. } => (Pose::SternToughLove, Emotion::Concerned),
            MascotEvent::AppLaunchedWhileBlocked { .. } => (Pose::SternToughLove, Emotion::Stern),
            MascotEvent::FocusSessionStarted { .. } => (Pose::Confident, Emotion::Neutral),
            MascotEvent::FocusSessionCompleted { minutes } if minutes >= 25 => (Pose::Celebratory, Emotion::Excited),
            MascotEvent::FocusSessionCompleted { .. } => (Pose::Encouraging, Emotion::Happy),
            MascotEvent::DailyCheckIn => (Pose::Confident, Emotion::Warm),
            MascotEvent::SleepDebtReported { hours } if hours < 5.0 => (Pose::SleepyDisappointed, Emotion::Tired),
            MascotEvent::SleepDebtReported { .. } => (Pose::CuriousThinking, Emotion::Concerned),
            MascotEvent::Idle => (Pose::Idle, Emotion::Neutral),
        };
        self.state = MascotState::new(
            pose,
            emotion,
            Some(MascotState::default_bubble_for(pose).to_string()),
        );
        &self.state
    }
}

impl Default for MascotMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streak_shows_proud_encouraging() {
        let mut m = MascotMachine::new();
        let s = m.on_event(MascotEvent::StreakIncremented { name: "study".into(), count: 3 });
        assert_eq!(s.pose, Pose::Encouraging);
        assert_eq!(s.emotion, Emotion::Proud);
    }

    #[test]
    fn low_sleep_triggers_sleepy_tired() {
        let mut m = MascotMachine::new();
        let s = m.on_event(MascotEvent::SleepDebtReported { hours: 4.2 });
        assert_eq!(s.pose, Pose::SleepyDisappointed);
        assert_eq!(s.emotion, Emotion::Tired);
    }

    #[test]
    fn big_focus_session_celebrates() {
        let mut m = MascotMachine::new();
        let s = m.on_event(MascotEvent::FocusSessionCompleted { minutes: 50 });
        assert_eq!(s.pose, Pose::Celebratory);
    }

    #[test]
    fn bubble_defaults_are_set() {
        assert!(!MascotState::default_bubble_for(Pose::Confident).is_empty());
        assert!(!MascotState::default_bubble_for(Pose::SleepyDisappointed).is_empty());
    }
}
