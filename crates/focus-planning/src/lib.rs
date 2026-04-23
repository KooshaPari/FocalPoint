//! focus-planning — Task model for the Motion-layer scheduler.
//!
//! Traces to: FR-PLAN-001 (Task model).
//!
//! Provides the pure data types the scheduler consumes: `Task`, `DurationSpec`,
//! `Priority`, `Deadline`, `ChunkingPolicy`, `Constraint`, `TaskStatus`,
//! `TimeBlock`. All types are `Clone + Serialize + Deserialize` so they can
//! cross the FFI / storage boundary later.

#![deny(clippy::all)]

use chrono::{DateTime, Duration, NaiveTime, Utc, Weekday};
use focus_domain::Rigidity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Duration
// ---------------------------------------------------------------------------

/// A p50/p90 estimate for how long a task will take. Useful when the user
/// genuinely does not know.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Estimate {
    pub p50: Duration,
    pub p90: Duration,
}

/// Describes how long a task takes. Either a fixed commitment (an exam slot, a
/// calendar-pinned block) or a probabilistic estimate (most writing/coding).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DurationSpec {
    pub fixed: Option<Duration>,
    pub estimate: Option<Estimate>,
}

impl DurationSpec {
    pub fn fixed(d: Duration) -> Self {
        Self { fixed: Some(d), estimate: None }
    }

    pub fn estimated(p50: Duration, p90: Duration) -> Self {
        Self { fixed: None, estimate: Some(Estimate { p50, p90 }) }
    }

    /// Best-guess duration to feed the scheduler. Fixed wins; else p90 (so we
    /// plan for the pessimistic case and free slack if we finish early).
    pub fn planning_duration(&self) -> Duration {
        if let Some(f) = self.fixed {
            return f;
        }
        if let Some(e) = &self.estimate {
            return e.p90;
        }
        Duration::zero()
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed.is_some()
    }
}

// ---------------------------------------------------------------------------
// Priority
// ---------------------------------------------------------------------------

/// Continuous priority in [0.0, 1.0]. Aging nudges weight toward 1.0 on each
/// bump (older unsatisfied tasks climb the stack).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Priority {
    pub weight: f32,
}

impl Priority {
    pub const fn new(weight: f32) -> Self {
        Self { weight }
    }

    pub fn clamped(weight: f32) -> Self {
        Self { weight: weight.clamp(0.0, 1.0) }
    }

    /// Returns a new Priority whose weight is nudged toward 1.0 by `bumps`
    /// steps. Each bump closes ~10% of the remaining gap to 1.0.
    pub fn aged(&self, bumps: u32) -> Priority {
        let mut w = self.weight.clamp(0.0, 1.0);
        for _ in 0..bumps {
            w += (1.0 - w) * 0.10;
        }
        Priority { weight: w.clamp(0.0, 1.0) }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self { weight: 0.5 }
    }
}

// ---------------------------------------------------------------------------
// Deadline
// ---------------------------------------------------------------------------

/// An optional deadline. When `when` is `None` the task has no deadline.
/// `rigidity` mirrors `focus_domain::Rigidity`: Hard = exam/flight, Soft =
/// preference, Semi(cost) = miss-with-consequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deadline {
    pub when: Option<DateTime<Utc>>,
    pub rigidity: Rigidity,
}

impl Deadline {
    pub fn none() -> Self {
        Self { when: None, rigidity: Rigidity::Soft }
    }

    pub fn hard(when: DateTime<Utc>) -> Self {
        Self { when: Some(when), rigidity: Rigidity::Hard }
    }

    pub fn soft(when: DateTime<Utc>) -> Self {
        Self { when: Some(when), rigidity: Rigidity::Soft }
    }

    pub fn is_hard(&self) -> bool {
        self.when.is_some() && self.rigidity.is_hard()
    }
}

// ---------------------------------------------------------------------------
// Chunking
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkingPolicy {
    pub allow_split: bool,
    pub min_chunk: Duration,
    pub max_chunk: Duration,
    pub ideal_chunk: Duration,
}

impl Default for ChunkingPolicy {
    fn default() -> Self {
        Self {
            allow_split: true,
            min_chunk: Duration::minutes(25),
            max_chunk: Duration::hours(2),
            ideal_chunk: Duration::minutes(50),
        }
    }
}

impl ChunkingPolicy {
    pub fn atomic() -> Self {
        Self {
            allow_split: false,
            min_chunk: Duration::zero(),
            max_chunk: Duration::hours(24),
            ideal_chunk: Duration::hours(1),
        }
    }
}

// ---------------------------------------------------------------------------
// Constraints
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyTier {
    DeepFocus,
    Light,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Constraint {
    /// Allowed scheduling window by time-of-day and weekday.
    WorkingHours {
        start: NaiveTime,
        end: NaiveTime,
        days: Vec<Weekday>,
    },
    NoEarlierThan(DateTime<Utc>),
    NoLaterThan(DateTime<Utc>),
    /// Minimum padding around any placement for context-switching.
    Buffer(Duration),
    EnergyTier(EnergyTier),
}

// ---------------------------------------------------------------------------
// TimeBlock
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeBlock {
    pub task_id: Uuid,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub rigidity: Rigidity,
}

impl TimeBlock {
    pub fn duration(&self) -> Duration {
        self.ends_at - self.starts_at
    }

    pub fn overlaps(&self, other_start: DateTime<Utc>, other_end: DateTime<Utc>) -> bool {
        self.starts_at < other_end && other_start < self.ends_at
    }
}

// ---------------------------------------------------------------------------
// Task + status
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    #[default]
    Pending,
    Scheduled {
        chunks: Vec<TimeBlock>,
    },
    InProgress,
    Completed,
    Cancelled,
}

impl TaskStatus {
    /// Returns `true` if the requested transition is legal.
    pub fn can_transition_to(&self, next: &TaskStatus) -> bool {
        use TaskStatus::*;
        match (self, next) {
            (Pending, Scheduled { .. }) => true,
            (Pending, Cancelled) => true,
            (Scheduled { .. }, InProgress) => true,
            (Scheduled { .. }, Pending) => true,
            (Scheduled { .. }, Cancelled) => true,
            (InProgress, Completed) => true,
            (InProgress, Cancelled) => true,
            (InProgress, Scheduled { .. }) => true, // reflow
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub duration: DurationSpec,
    pub priority: Priority,
    pub deadline: Deadline,
    pub chunking: ChunkingPolicy,
    pub constraints: Vec<Constraint>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Priority has non-Eq f32; Task needs PartialEq (not Eq) to compile. Override:
impl Eq for Priority {}

impl Task {
    pub fn new(title: impl Into<String>, duration: DurationSpec, now: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            duration,
            priority: Priority::default(),
            deadline: Deadline::none(),
            chunking: ChunkingPolicy::default(),
            constraints: Vec::new(),
            status: TaskStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use focus_domain::RigidityCost;

    fn t0() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 5, 1, 9, 0, 0).unwrap()
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn duration_spec_fixed_wins() {
        let ds = DurationSpec::fixed(Duration::minutes(45));
        assert!(ds.is_fixed());
        assert_eq!(ds.planning_duration(), Duration::minutes(45));
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn duration_spec_uses_p90_when_no_fixed() {
        let ds = DurationSpec::estimated(Duration::minutes(30), Duration::minutes(75));
        assert!(!ds.is_fixed());
        assert_eq!(ds.planning_duration(), Duration::minutes(75));
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn duration_spec_empty_is_zero() {
        let ds = DurationSpec::default();
        assert_eq!(ds.planning_duration(), Duration::zero());
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn priority_aged_monotonic_toward_one() {
        let p = Priority::new(0.2);
        let a1 = p.aged(1);
        let a5 = p.aged(5);
        assert!(a1.weight > p.weight);
        assert!(a5.weight > a1.weight);
        assert!(a5.weight < 1.0 + f32::EPSILON);
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn priority_aged_clamps_at_one() {
        let p = Priority::new(0.99);
        let bumped = p.aged(1000);
        assert!(bumped.weight <= 1.0);
        assert!(bumped.weight > 0.99);
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn deadline_hardness_reflects_rigidity() {
        let hard = Deadline::hard(t0());
        let soft = Deadline::soft(t0());
        let semi =
            Deadline { when: Some(t0()), rigidity: Rigidity::Semi(RigidityCost::CreditCost(10)) };
        let none = Deadline::none();
        assert!(hard.is_hard());
        assert!(!soft.is_hard());
        assert!(!semi.is_hard());
        assert!(!none.is_hard());
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn constraint_composition_holds_multiple() {
        let task = Task {
            constraints: vec![
                Constraint::WorkingHours {
                    start: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                    end: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                    days: vec![Weekday::Mon, Weekday::Tue, Weekday::Wed],
                },
                Constraint::Buffer(Duration::minutes(10)),
                Constraint::EnergyTier(EnergyTier::DeepFocus),
            ],
            ..Task::new("compose", DurationSpec::fixed(Duration::hours(1)), t0())
        };
        assert_eq!(task.constraints.len(), 3);
        assert!(matches!(task.constraints[2], Constraint::EnergyTier(EnergyTier::DeepFocus)));
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn task_status_transitions_legal_paths() {
        let pending = TaskStatus::Pending;
        let scheduled = TaskStatus::Scheduled { chunks: vec![] };
        let in_progress = TaskStatus::InProgress;
        let completed = TaskStatus::Completed;

        assert!(pending.can_transition_to(&scheduled));
        assert!(scheduled.can_transition_to(&in_progress));
        assert!(in_progress.can_transition_to(&completed));
        // illegal: Pending -> Completed
        assert!(!pending.can_transition_to(&completed));
        // illegal: Completed -> anything
        assert!(!completed.can_transition_to(&pending));
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn time_block_overlap_detection() {
        let id = Uuid::new_v4();
        let tb = TimeBlock {
            task_id: id,
            starts_at: t0(),
            ends_at: t0() + Duration::hours(1),
            rigidity: Rigidity::Soft,
        };
        // overlapping
        assert!(tb.overlaps(t0() + Duration::minutes(30), t0() + Duration::minutes(90)));
        // adjacent non-overlap
        assert!(!tb.overlaps(t0() + Duration::hours(1), t0() + Duration::hours(2)));
        // before
        assert!(!tb.overlaps(t0() - Duration::hours(1), t0()));
    }

    // Traces to: FR-PLAN-001
    #[test]
    fn task_serde_round_trip() {
        let task = Task {
            priority: Priority::new(0.7),
            deadline: Deadline::hard(t0() + Duration::days(2)),
            ..Task::new(
                "ship scheduler",
                DurationSpec::estimated(Duration::hours(2), Duration::hours(4)),
                t0(),
            )
        };
        let json = serde_json::to_string(&task).expect("serialize");
        let back: Task = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(task, back);
    }
}
