//! Mascot pose/emotion state machine.
//!
//! The Rust side owns the *logical* mascot state. The Swift/iOS layer binds
//! this to Spline (or a comparable 3D/animation runtime) to render pose +
//! emotion transitions. LLM-driven personality generation is deferred.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Coarse-grained pose category driven by app/rule state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pose {
    Idle,
    Cheering,
    Disappointed,
    Focused,
    Sleeping,
    Celebrating,
    Warning,
    Locked,
}

/// Fine-grained emotional tint layered over a pose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Emotion {
    Neutral,
    Happy,
    Proud,
    Concerned,
    Stern,
    Excited,
    Tired,
}

/// Event that may trigger a mascot transition. Platform-agnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MascotEvent {
    RuleFired { rule_name: String },
    StreakIncremented { name: String, count: u32 },
    PenaltyEscalated { tier: String },
    BypassSpent { remaining: i64 },
    AppLaunchedWhileBlocked { bundle_id: String },
    DailyCheckIn,
    Idle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MascotState {
    pub pose: Pose,
    pub emotion: Emotion,
    pub since: DateTime<Utc>,
    pub bubble_text: Option<String>,
}

impl Default for MascotState {
    fn default() -> Self {
        Self {
            pose: Pose::Idle,
            emotion: Emotion::Neutral,
            since: Utc::now(),
            bubble_text: None,
        }
    }
}

/// Platform-binding trait: Swift/Kotlin adapter implements this to route
/// `MascotState` transitions into a 3D/animation runtime (Spline on iOS).
pub trait MascotDriver: Send + Sync {
    fn apply(&self, state: &MascotState);
}

/// Stub state machine: maps events → state. Real transition matrix TBD.
pub struct MascotMachine {
    pub state: MascotState,
}

impl MascotMachine {
    pub fn new() -> Self {
        Self { state: MascotState::default() }
    }

    pub fn on_event(&mut self, event: MascotEvent) -> &MascotState {
        // Stub: real transition table lands with Phase 1 UX work.
        let _ = event;
        &self.state
    }
}

impl Default for MascotMachine {
    fn default() -> Self {
        Self::new()
    }
}
