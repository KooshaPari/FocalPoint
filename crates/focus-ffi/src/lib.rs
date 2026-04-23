#![allow(clippy::empty_line_after_doc_comments)]
//! UniFFI export surface for FocalPoint core.
//!
//! Exposes the mascot state machine plus rules/rewards/penalties/policy/
//! audit/sync sub-APIs to Swift (via UniFFI) and Kotlin (via UniFFI-Kotlin /
//! JNI) using a single UDL.
//!
//! The scaffolding file is generated at build time by `uniffi_build` and
//! included here via `include_scaffolding!`.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use focus_audit::{AuditStore, InMemoryAuditStore};
use focus_calendar::InMemoryCalendarPort;
use focus_coaching::{
    CoachingProvider, HttpCoachingProvider, NoopCoachingProvider, RateLimitedProvider,
};
use focus_mascot::{
    Emotion as CoreEmotion, MascotEvent as CoreMascotEvent, MascotMachine,
    MascotState as CoreMascotState, Pose as CorePose,
};
use focus_penalties::{
    EscalationTier, LockoutWindow as CoreLockoutWindow, PenaltyMutation as CorePenaltyMutation,
};
use focus_planning::{Task, TaskStore};
use focus_policy::{PolicyBuilder, ProfileState};
use focus_rewards::{Credit, MultiplierState, WalletMutation as CoreWalletMutation};
use focus_rituals::{
    EveningShutdown as CoreEveningShutdown, MorningBrief as CoreMorningBrief, RitualsEngine,
    SchedulePreview as CoreSchedulePreview, ScheduleWindowKind as CoreScheduleWindowKind,
    ScheduleWindowLine as CoreScheduleWindowLine, ShippedTask as CoreShippedTask,
    SlipReason as CoreSlipReason, SlippedTask as CoreSlippedTask, TaskActual as CoreTaskActual,
    TopPriorityLine as CoreTopPriorityLine,
};
use focus_rules::{
    Action as CoreAction, Condition as CoreCondition, PrioritizedDecision, Rule as CoreRule,
    Trigger as CoreTrigger,
};
use focus_scheduler::{Scheduler, WorkingHoursSpec};
use focus_storage::ports::{PenaltyStore, RuleStore, WalletStore};
use focus_storage::sqlite::rule_store::upsert_rule;
use focus_storage::sqlite::task_store::SqliteTaskStore;
use focus_storage::SqliteAdapter;
use focus_sync::SyncOrchestrator;
use secrecy::SecretString;
use thiserror::Error;
use tokio::runtime::Runtime;
use tokio::sync::Mutex as AsyncMutex;
use uuid::Uuid;

uniffi::include_scaffolding!("focus_ffi");

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum FfiError {
    #[error("not implemented")]
    NotImplemented,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("storage: {0}")]
    Storage(String),
    #[error("domain: {0}")]
    Domain(String),
    #[error("config: {0}")]
    Config(String),
    #[error("network: {0}")]
    Network(String),
}

impl From<anyhow::Error> for FfiError {
    fn from(e: anyhow::Error) -> Self {
        FfiError::Storage(e.to_string())
    }
}

// ---------------------------------------------------------------------------
// Mascot types (unchanged surface)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Rules: DTOs + conversions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RuleSummary {
    pub id: String,
    pub name: String,
    pub priority: i32,
    pub explanation_template: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum RuleActionDto {
    GrantCredit { amount: i32 },
    DeductCredit { amount: i32 },
    Block { profile: String, duration_seconds: i64 },
    Unblock { profile: String },
    StreakIncrement { name: String },
    StreakReset { name: String },
    Notify { message: String },
}

#[derive(Debug, Clone)]
pub struct RuleDraft {
    pub id: String,
    pub name: String,
    pub trigger_event: String,
    pub actions: Vec<RuleActionDto>,
    pub priority: i32,
    pub cooldown_seconds: Option<i64>,
    pub duration_seconds: Option<i64>,
    pub explanation_template: String,
    pub enabled: bool,
}

impl From<RuleActionDto> for CoreAction {
    fn from(a: RuleActionDto) -> Self {
        match a {
            RuleActionDto::GrantCredit { amount } => CoreAction::GrantCredit { amount },
            RuleActionDto::DeductCredit { amount } => CoreAction::DeductCredit { amount },
            RuleActionDto::Block { profile, duration_seconds } => CoreAction::Block {
                profile,
                duration: ChronoDuration::seconds(duration_seconds),
                rigidity: focus_domain::Rigidity::Hard,
            },
            RuleActionDto::Unblock { profile } => CoreAction::Unblock { profile },
            RuleActionDto::StreakIncrement { name } => CoreAction::StreakIncrement(name),
            RuleActionDto::StreakReset { name } => CoreAction::StreakReset(name),
            RuleActionDto::Notify { message } => CoreAction::Notify(message),
        }
    }
}

fn rule_to_summary(r: &CoreRule) -> RuleSummary {
    RuleSummary {
        id: r.id.to_string(),
        name: r.name.clone(),
        priority: r.priority,
        explanation_template: r.explanation_template.clone(),
        enabled: r.enabled,
    }
}

fn draft_to_core(d: RuleDraft) -> Result<CoreRule, FfiError> {
    let id = Uuid::parse_str(&d.id)
        .map_err(|e| FfiError::InvalidArgument(format!("rule id uuid: {e}")))?;
    Ok(CoreRule {
        id,
        name: d.name,
        trigger: CoreTrigger::Event(d.trigger_event),
        conditions: Vec::<CoreCondition>::new(),
        actions: d.actions.into_iter().map(CoreAction::from).collect(),
        priority: d.priority,
        cooldown: d.cooldown_seconds.map(ChronoDuration::seconds),
        duration: d.duration_seconds.map(ChronoDuration::seconds),
        explanation_template: d.explanation_template,
        enabled: d.enabled,
    })
}

// ---------------------------------------------------------------------------
// Rewards: DTOs + conversions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct StreakSummary {
    pub name: String,
    pub count: u32,
    pub last_incremented_iso: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WalletSummary {
    pub earned: i64,
    pub spent: i64,
    pub balance: i64,
    pub multiplier: f32,
    pub streaks: Vec<StreakSummary>,
}

#[derive(Debug, Clone)]
pub enum WalletMutationDto {
    GrantCredit { amount: i64 },
    SpendCredit { amount: i64, purpose: String },
    StreakIncrement { name: String },
    StreakReset { name: String },
    SetMultiplier { current: f32, expires_iso: Option<String> },
}

fn parse_iso(s: &str) -> Result<DateTime<Utc>, FfiError> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|e| FfiError::InvalidArgument(format!("parse rfc3339 '{s}': {e}")))
}

fn parse_iso_opt(s: Option<String>) -> Result<Option<DateTime<Utc>>, FfiError> {
    s.map(|v| parse_iso(&v)).transpose()
}

impl WalletMutationDto {
    fn into_core(self, now: DateTime<Utc>) -> Result<CoreWalletMutation, FfiError> {
        Ok(match self {
            WalletMutationDto::GrantCredit { amount } => CoreWalletMutation::GrantCredit(Credit {
                amount,
                source_rule_id: None,
                granted_at: now,
            }),
            WalletMutationDto::SpendCredit { amount, purpose } => {
                CoreWalletMutation::SpendCredit { amount, purpose }
            }
            WalletMutationDto::StreakIncrement { name } => {
                CoreWalletMutation::StreakIncrement(name)
            }
            WalletMutationDto::StreakReset { name } => CoreWalletMutation::StreakReset(name),
            WalletMutationDto::SetMultiplier { current, expires_iso } => {
                CoreWalletMutation::SetMultiplier(MultiplierState {
                    current,
                    expires_at: parse_iso_opt(expires_iso)?,
                })
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Penalties: DTOs + conversions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct LockoutWindowDto {
    pub starts_at_iso: String,
    pub ends_at_iso: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct PenaltyStateSummary {
    pub tier: String,
    pub bypass_budget: i64,
    pub debt_balance: i64,
    pub strict_mode_until_iso: Option<String>,
    pub lockouts: Vec<LockoutWindowDto>,
}

#[derive(Debug, Clone)]
pub struct BypassQuoteDto {
    pub cost: i64,
    pub remaining_after: i64,
    pub new_tier: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PenaltyMutationDto {
    Escalate { tier: String },
    SpendBypass { amount: i64 },
    GrantBypass { amount: i64 },
    AddLockout { window: LockoutWindowDto },
    ClearLockouts,
    SetStrictMode { until_iso: String },
    Clear,
}

fn tier_name(t: EscalationTier) -> &'static str {
    match t {
        EscalationTier::Clear => "Clear",
        EscalationTier::Warning => "Warning",
        EscalationTier::Restricted => "Restricted",
        EscalationTier::Strict => "Strict",
    }
}

fn tier_parse(s: &str) -> Result<EscalationTier, FfiError> {
    Ok(match s {
        "Clear" => EscalationTier::Clear,
        "Warning" => EscalationTier::Warning,
        "Restricted" => EscalationTier::Restricted,
        "Strict" => EscalationTier::Strict,
        other => {
            return Err(FfiError::InvalidArgument(format!("unknown escalation tier: {other}")))
        }
    })
}

impl PenaltyMutationDto {
    fn into_core(self) -> Result<CorePenaltyMutation, FfiError> {
        Ok(match self {
            PenaltyMutationDto::Escalate { tier } => {
                CorePenaltyMutation::Escalate(tier_parse(&tier)?)
            }
            PenaltyMutationDto::SpendBypass { amount } => CorePenaltyMutation::SpendBypass(amount),
            PenaltyMutationDto::GrantBypass { amount } => CorePenaltyMutation::GrantBypass(amount),
            PenaltyMutationDto::AddLockout { window } => {
                CorePenaltyMutation::AddLockout(CoreLockoutWindow {
                    starts_at: parse_iso(&window.starts_at_iso)?,
                    ends_at: parse_iso(&window.ends_at_iso)?,
                    reason: window.reason,
                    rigidity: focus_domain::Rigidity::Hard,
                })
            }
            PenaltyMutationDto::ClearLockouts => CorePenaltyMutation::ClearLockouts,
            PenaltyMutationDto::SetStrictMode { until_iso } => {
                CorePenaltyMutation::SetStrictMode { until: parse_iso(&until_iso)? }
            }
            PenaltyMutationDto::Clear => CorePenaltyMutation::Clear,
        })
    }
}

// ---------------------------------------------------------------------------
// Policy / Audit / Sync DTOs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct EnforcementPolicySummary {
    pub active: bool,
    pub profile_states: HashMap<String, String>,
    pub generated_at_iso: String,
}

#[derive(Debug, Clone)]
pub struct ConnectorHandleSummary {
    pub connector_id: String,
    pub health: String,
    pub next_sync_at_iso: String,
    pub last_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SyncReportDto {
    pub events_pulled: u32,
    pub connectors_synced: u32,
    pub errors: Vec<String>,
}

// ---------------------------------------------------------------------------
// Rituals DTOs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TopPriorityLineDto {
    pub task_id: String,
    pub title: String,
    pub deadline_label: String,
    pub rigidity_tag: String,
    pub estimated_duration_minutes: u32,
}

#[derive(Debug, Clone)]
pub struct ScheduleWindowLineDto {
    pub starts_at_iso: String,
    pub ends_at_iso: String,
    pub title: String,
    pub kind: String,
}

#[derive(Debug, Clone)]
pub struct SchedulePreviewDto {
    pub windows: Vec<ScheduleWindowLineDto>,
    pub soft_conflicts: u32,
    pub hard_conflicts: u32,
}

#[derive(Debug, Clone)]
pub struct MorningBriefDto {
    pub date: String,
    pub intention: Option<String>,
    pub top_priorities: Vec<TopPriorityLineDto>,
    pub schedule_preview: SchedulePreviewDto,
    pub coachy_opening: String,
    pub generated_at_iso: String,
}

#[derive(Debug, Clone)]
pub struct ShippedTaskDto {
    pub id: String,
    pub title: String,
    pub planned_minutes: u32,
    pub actual_minutes: u32,
}

#[derive(Debug, Clone)]
pub struct SlippedTaskDto {
    pub id: String,
    pub title: String,
    pub planned_minutes: u32,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct EveningShutdownDto {
    pub date: String,
    pub shipped: Vec<ShippedTaskDto>,
    pub slipped: Vec<SlippedTaskDto>,
    pub carryover: Vec<String>,
    pub wins_summary: String,
    pub coachy_closing: String,
    pub streak_deltas: HashMap<String, i32>,
    pub generated_at_iso: String,
}

#[derive(Debug, Clone)]
pub struct TaskActualDto {
    pub task_id: String,
    pub actual_minutes: u32,
    pub completed_at_iso: Option<String>,
    pub cancelled: bool,
}

fn kind_name(k: &CoreScheduleWindowKind) -> &'static str {
    match k {
        CoreScheduleWindowKind::FocusBlock => "FocusBlock",
        CoreScheduleWindowKind::Meeting => "Meeting",
        CoreScheduleWindowKind::Personal => "Personal",
        CoreScheduleWindowKind::Ritual => "Ritual",
    }
}

fn slip_name(r: &CoreSlipReason) -> &'static str {
    match r {
        CoreSlipReason::Skipped => "Skipped",
        CoreSlipReason::Deferred => "Deferred",
        CoreSlipReason::Overran => "Overran",
        CoreSlipReason::Cancelled => "Cancelled",
    }
}

impl From<&CoreTopPriorityLine> for TopPriorityLineDto {
    fn from(v: &CoreTopPriorityLine) -> Self {
        TopPriorityLineDto {
            task_id: v.task_id.to_string(),
            title: v.title.clone(),
            deadline_label: v.deadline_label.clone(),
            rigidity_tag: v.rigidity_tag.clone(),
            estimated_duration_minutes: v.estimated_duration_minutes,
        }
    }
}

impl From<&CoreScheduleWindowLine> for ScheduleWindowLineDto {
    fn from(v: &CoreScheduleWindowLine) -> Self {
        ScheduleWindowLineDto {
            starts_at_iso: v.starts_at.to_rfc3339(),
            ends_at_iso: v.ends_at.to_rfc3339(),
            title: v.title.clone(),
            kind: kind_name(&v.kind).to_string(),
        }
    }
}

impl From<&CoreSchedulePreview> for SchedulePreviewDto {
    fn from(v: &CoreSchedulePreview) -> Self {
        SchedulePreviewDto {
            windows: v.windows.iter().map(ScheduleWindowLineDto::from).collect(),
            soft_conflicts: v.soft_conflicts,
            hard_conflicts: v.hard_conflicts,
        }
    }
}

impl From<CoreMorningBrief> for MorningBriefDto {
    fn from(v: CoreMorningBrief) -> Self {
        MorningBriefDto {
            date: v.date.to_string(),
            intention: v.intention,
            top_priorities: v.top_priorities.iter().map(TopPriorityLineDto::from).collect(),
            schedule_preview: SchedulePreviewDto::from(&v.schedule_preview),
            coachy_opening: v.coachy_opening,
            generated_at_iso: v.generated_at.to_rfc3339(),
        }
    }
}

impl From<&CoreShippedTask> for ShippedTaskDto {
    fn from(v: &CoreShippedTask) -> Self {
        ShippedTaskDto {
            id: v.id.to_string(),
            title: v.title.clone(),
            planned_minutes: v.planned_minutes,
            actual_minutes: v.actual_minutes,
        }
    }
}

impl From<&CoreSlippedTask> for SlippedTaskDto {
    fn from(v: &CoreSlippedTask) -> Self {
        SlippedTaskDto {
            id: v.id.to_string(),
            title: v.title.clone(),
            planned_minutes: v.planned_minutes,
            reason: slip_name(&v.reason).to_string(),
        }
    }
}

impl From<CoreEveningShutdown> for EveningShutdownDto {
    fn from(v: CoreEveningShutdown) -> Self {
        EveningShutdownDto {
            date: v.date.to_string(),
            shipped: v.shipped.iter().map(ShippedTaskDto::from).collect(),
            slipped: v.slipped.iter().map(SlippedTaskDto::from).collect(),
            carryover: v.carryover.iter().map(|u| u.to_string()).collect(),
            wins_summary: v.wins_summary,
            coachy_closing: v.coachy_closing,
            streak_deltas: v.streak_deltas,
            generated_at_iso: v.generated_at.to_rfc3339(),
        }
    }
}

impl TaskActualDto {
    fn into_core(self) -> Result<CoreTaskActual, FfiError> {
        let task_id = Uuid::parse_str(&self.task_id)
            .map_err(|e| FfiError::InvalidArgument(format!("task_id uuid: {e}")))?;
        let completed_at = match self.completed_at_iso {
            Some(iso) => Some(parse_iso(&iso)?),
            None => None,
        };
        Ok(CoreTaskActual {
            task_id,
            actual_minutes: self.actual_minutes,
            completed_at,
            cancelled: self.cancelled,
        })
    }
}

// ---------------------------------------------------------------------------
// Shared context (stashed on FocalPointCore, cloned into sub-APIs)
// ---------------------------------------------------------------------------

struct CoreCtx {
    runtime: Arc<Runtime>,
    adapter: SqliteAdapter,
    audit: Arc<InMemoryAuditStore>,
    user_id: Uuid,
    recent_decisions: Arc<Mutex<Vec<PrioritizedDecision>>>,
    sync: Arc<tokio::sync::Mutex<SyncOrchestrator>>,
}

// ---------------------------------------------------------------------------
// Sub-API implementations
// ---------------------------------------------------------------------------

pub struct RuleQuery {
    ctx: Arc<CoreCtx>,
}

impl RuleQuery {
    pub fn list_enabled(&self) -> Result<Vec<RuleSummary>, FfiError> {
        let adapter = self.ctx.adapter.clone();
        let rules =
            self.ctx.runtime.block_on(async move { RuleStore::list_enabled(&adapter).await })?;
        Ok(rules.iter().map(rule_to_summary).collect())
    }
}

pub struct RuleMutation {
    ctx: Arc<CoreCtx>,
}

impl RuleMutation {
    pub fn set_enabled(&self, rule_id: String, enabled: bool) -> Result<(), FfiError> {
        let id = Uuid::parse_str(&rule_id)
            .map_err(|e| FfiError::InvalidArgument(format!("rule id uuid: {e}")))?;
        let adapter = self.ctx.adapter.clone();
        self.ctx.runtime.block_on(async move {
            let existing = RuleStore::get(&adapter, id)
                .await?
                .ok_or_else(|| FfiError::InvalidArgument(format!("rule not found: {id}")))?;
            let mut updated = existing;
            updated.enabled = enabled;
            upsert_rule(&adapter, updated).await?;
            Ok::<(), FfiError>(())
        })
    }

    pub fn upsert(&self, rule: RuleDraft) -> Result<(), FfiError> {
        let core = draft_to_core(rule)?;
        let adapter = self.ctx.adapter.clone();
        self.ctx.runtime.block_on(async move { upsert_rule(&adapter, core).await })?;
        Ok(())
    }
}

pub struct WalletApi {
    ctx: Arc<CoreCtx>,
}

impl WalletApi {
    pub fn load(&self) -> Result<WalletSummary, FfiError> {
        let adapter = self.ctx.adapter.clone();
        let user_id = self.ctx.user_id;
        let wallet =
            self.ctx.runtime.block_on(async move { WalletStore::load(&adapter, user_id).await })?;
        let multiplier = wallet.effective_multiplier(Utc::now());
        let streaks = wallet
            .streaks
            .values()
            .map(|s| StreakSummary {
                name: s.name.clone(),
                count: s.count,
                last_incremented_iso: s.last_incremented_at.map(|d| d.to_rfc3339()),
            })
            .collect();
        Ok(WalletSummary {
            earned: wallet.earned_credits,
            spent: wallet.spent_credits,
            balance: wallet.balance(),
            multiplier,
            streaks,
        })
    }

    pub fn apply_mutation(&self, m: WalletMutationDto) -> Result<(), FfiError> {
        let now = Utc::now();
        let core = m.into_core(now)?;
        let adapter = self.ctx.adapter.clone();
        let user_id = self.ctx.user_id;
        self.ctx
            .runtime
            .block_on(async move { WalletStore::apply(&adapter, user_id, core).await })?;
        // Audit append (best-effort, in-memory chain).
        let mut chain = self
            .ctx
            .audit
            .chain
            .lock()
            .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?;
        chain.append(
            "wallet.mutation",
            self.ctx.user_id.to_string(),
            serde_json::json!({"at": now.to_rfc3339()}),
            now,
        );
        Ok(())
    }
}

pub struct PenaltyApi {
    ctx: Arc<CoreCtx>,
}

impl PenaltyApi {
    pub fn load(&self) -> Result<PenaltyStateSummary, FfiError> {
        let adapter = self.ctx.adapter.clone();
        let user_id = self.ctx.user_id;
        let state = self
            .ctx
            .runtime
            .block_on(async move { PenaltyStore::load(&adapter, user_id).await })?;
        let lockouts = state
            .lockout_windows
            .iter()
            .map(|w| LockoutWindowDto {
                starts_at_iso: w.starts_at.to_rfc3339(),
                ends_at_iso: w.ends_at.to_rfc3339(),
                reason: w.reason.clone(),
            })
            .collect();
        Ok(PenaltyStateSummary {
            tier: tier_name(state.escalation_tier).to_string(),
            bypass_budget: state.bypass_budget,
            debt_balance: state.debt_balance,
            strict_mode_until_iso: state.strict_mode_until.map(|d| d.to_rfc3339()),
            lockouts,
        })
    }

    pub fn quote_bypass(&self, cost: i64) -> Result<BypassQuoteDto, FfiError> {
        let adapter = self.ctx.adapter.clone();
        let user_id = self.ctx.user_id;
        let state = self
            .ctx
            .runtime
            .block_on(async move { PenaltyStore::load(&adapter, user_id).await })?;
        let quote = state.quote_bypass(cost).map_err(|e| FfiError::Domain(e.to_string()))?;
        Ok(BypassQuoteDto {
            cost: quote.cost,
            remaining_after: quote.remaining_after,
            new_tier: quote.new_tier.map(|t| tier_name(t).to_string()),
        })
    }

    pub fn apply(&self, m: PenaltyMutationDto) -> Result<(), FfiError> {
        let now = Utc::now();
        let core = m.into_core()?;
        let adapter = self.ctx.adapter.clone();
        let user_id = self.ctx.user_id;
        self.ctx
            .runtime
            .block_on(async move { PenaltyStore::apply(&adapter, user_id, core).await })?;
        let mut chain = self
            .ctx
            .audit
            .chain
            .lock()
            .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?;
        chain.append(
            "penalty.mutation",
            self.ctx.user_id.to_string(),
            serde_json::json!({"at": now.to_rfc3339()}),
            now,
        );
        Ok(())
    }
}

pub struct PolicyApi {
    ctx: Arc<CoreCtx>,
}

impl PolicyApi {
    /// Builds an EnforcementPolicy from the most recent in-process rule
    /// decisions captured on this core handle. Persistent decision storage is
    /// a separate concern (FR-DATA-001 / rule_evaluations table) that is not
    /// yet wired; until then callers must feed decisions into the orchestrator
    /// for them to appear here. Returns an empty/inactive policy if none.
    pub fn build_from_recent_decisions(
        &self,
        limit: i32,
    ) -> Result<EnforcementPolicySummary, FfiError> {
        let recent = self
            .ctx
            .recent_decisions
            .lock()
            .map_err(|e| FfiError::Storage(format!("decisions mutex poisoned: {e}")))?;
        let n = if limit <= 0 { recent.len() } else { (limit as usize).min(recent.len()) };
        let slice: Vec<PrioritizedDecision> = recent.iter().rev().take(n).cloned().collect();
        let policy =
            PolicyBuilder::from_rule_decisions(&slice, Utc::now(), &focus_audit::NoopAuditSink);
        let profile_states = policy
            .profile_states
            .iter()
            .map(|(k, v)| {
                let repr = match v {
                    ProfileState::Blocked { ends_at, .. } => {
                        format!("blocked_until:{}", ends_at.to_rfc3339())
                    }
                    ProfileState::Unblocked => "unblocked".to_string(),
                };
                (k.clone(), repr)
            })
            .collect();
        Ok(EnforcementPolicySummary {
            active: policy.active,
            profile_states,
            generated_at_iso: policy.generated_at.to_rfc3339(),
        })
    }
}

pub struct AuditApi {
    ctx: Arc<CoreCtx>,
}

impl AuditApi {
    pub fn verify_chain(&self) -> Result<bool, FfiError> {
        self.ctx.audit.verify_chain().map_err(|e| FfiError::Storage(e.to_string()))
    }

    pub fn head_hash(&self) -> Result<Option<String>, FfiError> {
        self.ctx.audit.head_hash().map_err(|e| FfiError::Storage(e.to_string()))
    }
}

pub struct RitualsApi {
    ctx: Arc<CoreCtx>,
    tasks: Arc<dyn TaskStore>,
    engine: Arc<RitualsEngine>,
}

impl RitualsApi {
    pub fn generate_morning_brief(&self) -> Result<MorningBriefDto, FfiError> {
        let tasks: Vec<Task> = self
            .tasks
            .list(self.ctx.user_id)
            .map_err(|e| FfiError::Storage(format!("task store list: {e}")))?;
        let engine = self.engine.clone();
        let user_id = self.ctx.user_id;
        let brief = self.ctx.runtime.block_on(async move {
            engine.generate_morning_brief(&tasks, user_id, Utc::now()).await
        })?;
        Ok(MorningBriefDto::from(brief))
    }

    pub fn generate_evening_shutdown(
        &self,
        actuals: Vec<TaskActualDto>,
    ) -> Result<EveningShutdownDto, FfiError> {
        let tasks: Vec<Task> = self
            .tasks
            .list(self.ctx.user_id)
            .map_err(|e| FfiError::Storage(format!("task store list: {e}")))?;
        let engine = self.engine.clone();
        let converted: Vec<CoreTaskActual> =
            actuals.into_iter().map(|a| a.into_core()).collect::<Result<_, _>>()?;
        let now = Utc::now();
        let schedule = self.ctx.runtime.block_on(async move {
            engine.scheduler.plan(&tasks, &[], now, ChronoDuration::hours(24)).await
        })?;
        let engine2 = self.engine.clone();
        let shutdown = self.ctx.runtime.block_on(async move {
            engine2.generate_evening_shutdown(&schedule, &converted, now).await
        })?;
        Ok(EveningShutdownDto::from(shutdown))
    }
}

pub struct ConnectorApi {
    ctx: Arc<CoreCtx>,
}

impl ConnectorApi {
    /// Exchange a Canvas OAuth2 authorization `code` for an access token and
    /// persist it in the device keychain (service=`focalpoint`,
    /// account=`canvas:{instance_url}`). Appends an audit record on success.
    pub fn connect_canvas(&self, instance_url: String, code: String) -> Result<(), FfiError> {
        use connector_canvas::auth::{CanvasAuthConfig, CanvasOAuth2, KeychainStore, TokenStore};

        let cleaned = instance_url
            .trim()
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/')
            .to_string();
        if cleaned.is_empty() || !cleaned.contains('.') {
            return Err(FfiError::InvalidArgument(format!(
                "invalid canvas instance url: {instance_url}"
            )));
        }
        if code.trim().is_empty() {
            return Err(FfiError::InvalidArgument("empty authorization code".into()));
        }

        let client_id = std::env::var("FOCALPOINT_CANVAS_CLIENT_ID")
            .map_err(|_| FfiError::Config("canvas client id not configured".into()))?;
        let client_secret = std::env::var("FOCALPOINT_CANVAS_CLIENT_SECRET")
            .map_err(|_| FfiError::Config("canvas client id not configured".into()))?;

        let base_url = format!("https://{cleaned}");
        let cfg = CanvasAuthConfig {
            client_id,
            client_secret,
            base_url,
            redirect_uri: "focalpoint://auth/canvas/callback".to_string(),
        };
        let oauth = CanvasOAuth2::new(cfg)
            .map_err(|e| FfiError::Config(format!("canvas oauth init: {e}")))?;

        let inner: Arc<dyn focus_crypto::SecureSecretStore> =
            focus_crypto::default_secure_store("focalpoint").into();
        let account = format!("canvas:{cleaned}");
        let store = KeychainStore::new(account.clone(), inner);

        let now = Utc::now();
        self.ctx.runtime.block_on(async move {
            let http = reqwest::Client::new();
            let token = oauth
                .exchange_code(code, &http)
                .await
                .map_err(|e| FfiError::Network(format!("canvas code exchange: {e}")))?;
            store
                .save(&token)
                .await
                .map_err(|e| FfiError::Storage(format!("canvas keychain save: {e}")))?;
            Ok::<(), FfiError>(())
        })?;

        let mut chain = self
            .ctx
            .audit
            .chain
            .lock()
            .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?;
        chain.append(
            "connector.canvas.connected",
            account,
            serde_json::json!({
                "at": now.to_rfc3339(),
                "instance": cleaned,
            }),
            now,
        );
        Ok(())
    }
}

pub struct SyncApi {
    ctx: Arc<CoreCtx>,
}

impl SyncApi {
    pub fn connectors(&self) -> Vec<ConnectorHandleSummary> {
        let sync = self.ctx.sync.clone();
        self.ctx.runtime.block_on(async move {
            let guard = sync.lock().await;
            // SyncOrchestrator exposes `connector(id)` and `len` but not an
            // iterator. We don't have a public iter, so we return an empty
            // vec when no connectors are registered. (A richer iter can be
            // added to focus-sync later; keeping the FFI shim thin.)
            let _ = &*guard;
            Vec::<ConnectorHandleSummary>::new()
        })
    }

    pub fn tick(&self) -> SyncReportDto {
        let sync = self.ctx.sync.clone();
        self.ctx.runtime.block_on(async move {
            let mut guard = sync.lock().await;
            let report = guard.tick(Utc::now()).await;
            SyncReportDto {
                events_pulled: report.events_pulled as u32,
                connectors_synced: report.connectors_synced as u32,
                errors: report
                    .errors
                    .into_iter()
                    .map(|e| format!("{}: {}", e.connector_id, e.message))
                    .collect(),
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Top-level FocalPointCore
// ---------------------------------------------------------------------------

pub struct CoachingConfig {
    endpoint: String,
    api_key: SecretString,
    model: String,
}

impl CoachingConfig {
    pub fn new(endpoint: String, api_key: String, model: String) -> Self {
        Self { endpoint, api_key: SecretString::from(api_key), model }
    }
}

pub struct FocalPointCore {
    mascot: Mutex<MascotMachine>,
    ctx: Arc<CoreCtx>,
    coaching: Mutex<Option<Arc<dyn CoachingProvider>>>,
    /// Persistent task pool consumed by rituals + scheduler. Backed by
    /// `SqliteTaskStore` (table `tasks`, migration v4). Traces to FR-DATA-001
    /// / FR-PLAN-001; closed the "rituals hold tasks in memory" gap.
    tasks: Arc<dyn TaskStore>,
    /// Mascot handle used by the rituals engine; separate from `mascot` so we
    /// can take ownership across an async boundary without colliding with the
    /// sync mascot state machine used by `push_mascot_event`.
    rituals_mascot: Arc<AsyncMutex<MascotMachine>>,
    rituals_calendar: Arc<InMemoryCalendarPort>,
}

impl FocalPointCore {
    pub fn new(storage_path: String) -> Result<Self, FfiError> {
        let runtime =
            Arc::new(Runtime::new().map_err(|e| FfiError::Storage(format!("tokio runtime: {e}")))?);
        let path = PathBuf::from(&storage_path);
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| FfiError::Storage(format!("mkdir {}: {e}", parent.display())))?;
            }
        }
        let adapter = SqliteAdapter::open(&path).map_err(|e| FfiError::Storage(e.to_string()))?;
        let task_store: Arc<dyn TaskStore> = Arc::new(SqliteTaskStore::from_adapter(&adapter));
        let audit = Arc::new(InMemoryAuditStore::new());
        let ctx = Arc::new(CoreCtx {
            runtime,
            adapter,
            audit,
            user_id: Uuid::nil(),
            recent_decisions: Arc::new(Mutex::new(Vec::new())),
            sync: Arc::new(tokio::sync::Mutex::new(SyncOrchestrator::with_default_retry())),
        });
        Ok(Self {
            mascot: Mutex::new(MascotMachine::new()),
            ctx,
            coaching: Mutex::new(None),
            tasks: task_store,
            rituals_mascot: Arc::new(AsyncMutex::new(MascotMachine::new())),
            rituals_calendar: Arc::new(InMemoryCalendarPort::new()),
        })
    }

    /// Wire/unwire the LLM coaching provider. HTTP providers are wrapped in
    /// the default 10-call/60s [`RateLimitedProvider`] token bucket.
    pub fn set_coaching(&self, config: Option<Arc<CoachingConfig>>) {
        let provider: Option<Arc<dyn CoachingProvider>> = config.map(|c| {
            let http =
                HttpCoachingProvider::new(c.endpoint.clone(), c.api_key.clone(), c.model.clone());
            let inner: Arc<dyn CoachingProvider> = Arc::new(http);
            Arc::new(RateLimitedProvider::default_limits(inner)) as Arc<dyn CoachingProvider>
        });
        if let Ok(mut m) = self.mascot.lock() {
            m.set_coaching(provider.clone());
        }
        if let Ok(mut slot) = self.coaching.lock() {
            *slot = provider;
        }
    }

    /// Generate an LLM bubble line for `event` without mutating mascot state.
    /// Returns `None` if no provider is wired or the LLM call falls back.
    pub fn generate_bubble(&self, event: MascotEvent) -> Option<String> {
        let provider = {
            let g = self.coaching.lock().ok()?;
            g.clone()?
        };
        let core_event: CoreMascotEvent = event.into();
        let rt = self.ctx.runtime.clone();
        rt.block_on(async move {
            let mut tmp = MascotMachine::new().with_coaching(provider);
            let s = tmp.on_event_with_bubble(core_event).await;
            s.bubble_text.clone()
        })
    }

    /// Convert a natural-language rule spec into a Rule via the configured
    /// provider. Returns the rule's summary; the caller should review and
    /// then persist via `mutations().upsert(...)`.
    pub fn propose_rule_from_nl(&self, nl_spec: String) -> Result<RuleSummary, FfiError> {
        let provider = {
            let g = self
                .coaching
                .lock()
                .map_err(|e| FfiError::Storage(format!("coaching mutex poisoned: {e}")))?;
            g.clone()
                .ok_or_else(|| FfiError::InvalidArgument("no coaching provider wired".into()))?
        };
        let rt = self.ctx.runtime.clone();
        let rule = rt
            .block_on(async move {
                focus_rules::propose_rule_from_nl(&nl_spec, provider.as_ref()).await
            })
            .map_err(|e| FfiError::Domain(e.to_string()))?;
        Ok(rule_to_summary(&rule))
    }

    pub fn push_mascot_event(&self, event: MascotEvent) -> MascotState {
        let mut machine = self.mascot.lock().expect("mascot mutex poisoned");
        let core_event: CoreMascotEvent = event.into();
        let next = machine.on_event(core_event);
        MascotState::from(next)
    }

    pub fn mascot_state(&self) -> MascotState {
        let machine = self.mascot.lock().expect("mascot mutex poisoned");
        MascotState::from(&machine.state)
    }

    pub fn app_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    pub fn rules(&self) -> Arc<RuleQuery> {
        Arc::new(RuleQuery { ctx: self.ctx.clone() })
    }

    pub fn mutations(&self) -> Arc<RuleMutation> {
        Arc::new(RuleMutation { ctx: self.ctx.clone() })
    }

    pub fn wallet(&self) -> Arc<WalletApi> {
        Arc::new(WalletApi { ctx: self.ctx.clone() })
    }

    pub fn penalty(&self) -> Arc<PenaltyApi> {
        Arc::new(PenaltyApi { ctx: self.ctx.clone() })
    }

    pub fn policy(&self) -> Arc<PolicyApi> {
        Arc::new(PolicyApi { ctx: self.ctx.clone() })
    }

    pub fn audit(&self) -> Arc<AuditApi> {
        Arc::new(AuditApi { ctx: self.ctx.clone() })
    }

    pub fn sync(&self) -> Arc<SyncApi> {
        Arc::new(SyncApi { ctx: self.ctx.clone() })
    }

    pub fn connector(&self) -> Arc<ConnectorApi> {
        Arc::new(ConnectorApi { ctx: self.ctx.clone() })
    }

    /// Access the Planning Coach rituals surface (Morning Brief + Evening
    /// Shutdown). Uses the coaching provider wired via `set_coaching` when
    /// present, else falls back to the Noop provider (static copy).
    pub fn rituals(&self) -> Arc<RitualsApi> {
        let coaching: Arc<dyn CoachingProvider> = {
            let g = self.coaching.lock().ok().and_then(|g| g.clone());
            g.unwrap_or_else(|| Arc::new(NoopCoachingProvider))
        };
        let scheduler = Arc::new(Scheduler::new(WorkingHoursSpec::default()));
        let calendar: Arc<dyn focus_calendar::CalendarPort> = self.rituals_calendar.clone();
        let engine = Arc::new(RitualsEngine::new(
            scheduler,
            calendar,
            coaching,
            self.rituals_mascot.clone(),
        ));
        Arc::new(RitualsApi { ctx: self.ctx.clone(), tasks: self.tasks.clone(), engine })
    }

    // Test-only helper: replace the persistent task pool with `new`. Not
    // exposed over FFI. Clears any existing tasks for the core's user_id
    // then upserts each. Useful for ritual tests that need a known fixture.
    #[doc(hidden)]
    pub fn seed_tasks_for_test(&self, new: Vec<Task>) {
        let existing = self.tasks.list(self.ctx.user_id).expect("task list");
        for t in existing {
            self.tasks.delete(t.id).expect("task delete");
        }
        for t in &new {
            self.tasks.upsert(self.ctx.user_id, t).expect("task upsert");
        }
    }

    // ---- Test helpers -----------------------------------------------------

    /// Inject an arbitrary coaching provider for tests. Mirrors what
    /// `set_coaching` does but skips the HTTP/rate-limit wrapping so unit
    /// tests can exercise the FFI surface with a [`StubCoachingProvider`].
    #[doc(hidden)]
    pub fn set_coaching_provider_for_test(&self, provider: Arc<dyn CoachingProvider>) {
        if let Ok(mut m) = self.mascot.lock() {
            m.set_coaching(Some(provider.clone()));
        }
        if let Ok(mut slot) = self.coaching.lock() {
            *slot = Some(provider);
        }
    }

    /// Seed a prioritized decision into the in-process recent buffer. Used by
    /// tests and (eventually) by the rule engine runner. Not exposed over FFI.
    #[doc(hidden)]
    pub fn record_decision_for_test(&self, decision: PrioritizedDecision) {
        let mut recent = self.ctx.recent_decisions.lock().expect("decisions poisoned");
        recent.push(decision);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use focus_rules::RuleDecision;
    use tempfile::TempDir;

    fn mk_core() -> (TempDir, FocalPointCore) {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("focal.db");
        let core = FocalPointCore::new(path.to_string_lossy().into_owned()).expect("core");
        (dir, core)
    }

    #[test]
    fn mascot_surface_still_works() {
        let (_d, core) = mk_core();
        let s0 = core.mascot_state();
        assert!(matches!(s0.pose, Pose::Idle));
        let s1 = core
            .push_mascot_event(MascotEvent::StreakIncremented { name: "study".into(), count: 2 });
        assert!(matches!(s1.pose, Pose::Encouraging));
        assert_eq!(core.app_version(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn rule_upsert_then_list_enabled() {
        let (_d, core) = mk_core();
        let rules_api = core.rules();
        let muts = core.mutations();
        let id = Uuid::new_v4().to_string();
        muts.upsert(RuleDraft {
            id: id.clone(),
            name: "TestRule".into(),
            trigger_event: "TaskCompleted".into(),
            actions: vec![RuleActionDto::GrantCredit { amount: 5 }],
            priority: 10,
            cooldown_seconds: None,
            duration_seconds: None,
            explanation_template: "t".into(),
            enabled: true,
        })
        .expect("upsert");
        let listed = rules_api.list_enabled().expect("list");
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, id);
        muts.set_enabled(id.clone(), false).expect("disable");
        let listed2 = rules_api.list_enabled().expect("list2");
        assert!(listed2.is_empty());
    }

    #[test]
    fn wallet_grant_then_spend_through_ffi() {
        let (_d, core) = mk_core();
        let wallet = core.wallet();
        wallet.apply_mutation(WalletMutationDto::GrantCredit { amount: 100 }).expect("grant");
        let s = wallet.load().expect("load");
        assert_eq!(s.earned, 100);
        assert_eq!(s.balance, 100);
        wallet
            .apply_mutation(WalletMutationDto::SpendCredit { amount: 40, purpose: "unlock".into() })
            .expect("spend");
        let s2 = wallet.load().expect("load2");
        assert_eq!(s2.balance, 60);
    }

    #[test]
    fn penalty_escalate_quote_and_audit_chain_grows() {
        let (_d, core) = mk_core();
        let penalty = core.penalty();
        penalty.apply(PenaltyMutationDto::GrantBypass { amount: 10 }).expect("grant bypass");
        let q = penalty.quote_bypass(4).expect("quote");
        assert_eq!(q.cost, 4);
        assert_eq!(q.remaining_after, 6);
        penalty.apply(PenaltyMutationDto::Escalate { tier: "Warning".into() }).expect("escalate");
        let s = penalty.load().expect("load");
        assert_eq!(s.tier, "Warning");
        assert_eq!(s.bypass_budget, 10);
        // Audit chain should have records from the mutations above.
        let audit = core.audit();
        assert!(audit.verify_chain().expect("verify"));
        assert!(audit.head_hash().expect("head").is_some());
    }

    #[test]
    fn policy_empty_when_no_decisions_then_reflects_seeded_block() {
        let (_d, core) = mk_core();
        let policy = core.policy();
        let empty = policy.build_from_recent_decisions(10).expect("empty");
        assert!(!empty.active);
        assert!(empty.profile_states.is_empty());

        // Seed a Block decision through the test-only back door.
        let decision = PrioritizedDecision {
            rule_id: Uuid::new_v4(),
            priority: 50,
            decision: RuleDecision::Fired(vec![CoreAction::Block {
                profile: "games".into(),
                duration: ChronoDuration::minutes(30),
                rigidity: focus_domain::Rigidity::Hard,
            }]),
        };
        core.record_decision_for_test(decision);
        let active = policy.build_from_recent_decisions(10).expect("active");
        assert!(active.active);
        assert!(active.profile_states.contains_key("games"));
    }

    #[test]
    fn generate_bubble_none_when_no_provider() {
        let (_d, core) = mk_core();
        assert!(core.generate_bubble(MascotEvent::Idle).is_none());
    }

    #[test]
    fn generate_bubble_uses_injected_provider() {
        let (_d, core) = mk_core();
        let provider: Arc<dyn CoachingProvider> =
            Arc::new(focus_coaching::StubCoachingProvider::single("Nice work!"));
        core.set_coaching_provider_for_test(provider);
        let out =
            core.generate_bubble(MascotEvent::FocusSessionCompleted { minutes: 30 }).expect("some");
        assert_eq!(out, "Nice work!");
        // Main mascot state should NOT have mutated.
        assert!(matches!(core.mascot_state().pose, Pose::Idle));
    }

    #[test]
    fn propose_rule_from_nl_via_ffi_returns_summary() {
        let (_d, core) = mk_core();
        let id = Uuid::new_v4();
        let json_rule = serde_json::json!({
            "id": id.to_string(),
            "name": "FFI Rule",
            "trigger": {"Event": "TaskCompleted"},
            "conditions": [],
            "actions": [{"GrantCredit": {"amount": 3}}],
            "priority": 7,
            "cooldown": null,
            "duration": null,
            "explanation_template": "{rule_name}",
            "enabled": true
        })
        .to_string();
        let provider: Arc<dyn CoachingProvider> =
            Arc::new(focus_coaching::StubCoachingProvider::single(json_rule));
        core.set_coaching_provider_for_test(provider);
        let summary =
            core.propose_rule_from_nl("grant 3 credits on task complete".into()).expect("nl");
        assert_eq!(summary.name, "FFI Rule");
        assert_eq!(summary.priority, 7);
    }

    #[test]
    fn propose_rule_errors_when_no_provider() {
        let (_d, core) = mk_core();
        let err = core.propose_rule_from_nl("x".into()).unwrap_err();
        assert!(matches!(err, FfiError::InvalidArgument(_)));
    }

    #[test]
    fn connect_canvas_errors_without_env_client_id() {
        let (_d, core) = mk_core();
        // Ensure env vars are unset for deterministic failure. Safe: tests run
        // in a single process; this is a best-effort unset.
        std::env::remove_var("FOCALPOINT_CANVAS_CLIENT_ID");
        std::env::remove_var("FOCALPOINT_CANVAS_CLIENT_SECRET");
        let err = core
            .connector()
            .connect_canvas("canvas.example.com".into(), "the-code".into())
            .unwrap_err();
        match err {
            FfiError::Config(msg) => {
                assert!(msg.contains("canvas client id"), "got: {msg}");
            }
            other => panic!("expected Config error, got {other:?}"),
        }
    }

    #[test]
    fn connect_canvas_rejects_bogus_instance_url() {
        let (_d, core) = mk_core();
        let err =
            core.connector().connect_canvas("not-a-host".into(), "the-code".into()).unwrap_err();
        assert!(matches!(err, FfiError::InvalidArgument(_)));
    }

    #[test]
    fn sync_tick_with_no_connectors_is_noop() {
        let (_d, core) = mk_core();
        let sync = core.sync();
        let report = sync.tick();
        assert_eq!(report.connectors_synced, 0);
        assert_eq!(report.events_pulled, 0);
        assert!(report.errors.is_empty());
        assert!(sync.connectors().is_empty());
    }
}
