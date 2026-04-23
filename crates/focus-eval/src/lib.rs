//! Event → Rule → Action evaluation pipeline.
//!
//! [`RuleEvaluationPipeline::tick`] pulls new events from an [`EventStore`]
//! via a persisted cursor, evaluates every enabled rule against each event
//! using a shared [`RuleEngine`] (so cooldown state survives across ticks),
//! dispatches fired actions into the wallet / penalty / policy layers, and
//! appends a `rule.fired` audit record per decision.
//!
//! This crate closes the event→rule→action loop: connectors persist events
//! via `EventSink`, the SQLite event store holds them, and the pipeline
//! turns those rows into wallet/penalty/policy mutations.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use focus_audit::AuditSink;
use focus_events::NormalizedEvent;
use focus_penalties::PenaltyMutation;
use focus_rewards::{Credit, WalletMutation};
use focus_rules::{Action, PrioritizedDecision, Rule, RuleDecision, RuleEngine};
use focus_storage::ports::{EventStore, PenaltyStore, RuleStore, WalletStore};
use focus_sync::CursorStore;
use serde_json::json;
use tokio::sync::RwLock;
use tracing::{debug, warn};
use uuid::Uuid;

/// Canonical `(connector_id, entity_type)` pair used to persist the
/// rule-evaluation cursor. Stored here so callers and tests agree.
pub const RULE_EVAL_CONNECTOR_ID: &str = "rule_eval";
pub const RULE_EVAL_ENTITY_TYPE: &str = "events";

/// Sink for decisions that mutate enforcement policy (Block/Unblock). The
/// FFI layer stashes these in a ring buffer that `PolicyApi::build_from_
/// recent_decisions` reads; tests can use a no-op.
pub trait DecisionSink: Send + Sync {
    fn record(&self, decision: PrioritizedDecision);
}

/// Drops everything — tests, or callers that don't care about policy.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopDecisionSink;

impl DecisionSink for NoopDecisionSink {
    fn record(&self, _decision: PrioritizedDecision) {}
}

/// Captures into a shared `Vec<PrioritizedDecision>`. Used by the FFI core
/// to feed the `recent_decisions` buffer that `PolicyApi` reads.
#[derive(Debug, Clone)]
pub struct VecDecisionSink {
    inner: Arc<Mutex<Vec<PrioritizedDecision>>>,
    cap: usize,
}

impl VecDecisionSink {
    pub fn new(inner: Arc<Mutex<Vec<PrioritizedDecision>>>, cap: usize) -> Self {
        Self { inner, cap }
    }
}

impl DecisionSink for VecDecisionSink {
    fn record(&self, decision: PrioritizedDecision) {
        if let Ok(mut g) = self.inner.lock() {
            g.push(decision);
            let len = g.len();
            if len > self.cap {
                let drop = len - self.cap;
                g.drain(0..drop);
            }
        }
    }
}

/// Summary of a single [`RuleEvaluationPipeline::tick`] pass. Surfaced over
/// FFI via `EvalApi::tick` so the UI can show "evaluated N, fired M".
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationReport {
    pub events_evaluated: u32,
    pub decisions_fired: u32,
    pub decisions_suppressed: u32,
    pub decisions_skipped: u32,
}

/// Default batch size for each tick; keeps the mutex-held section bounded.
pub const DEFAULT_BATCH_SIZE: usize = 256;

/// Event → Rule → Action pipeline.
///
/// Runs **alongside** the [`focus_sync::SyncOrchestrator`], not inside it:
/// the orchestrator populates the event store, then this pipeline drains it.
pub struct RuleEvaluationPipeline {
    event_store: Arc<dyn EventStore>,
    rule_store: Arc<dyn RuleStore>,
    engine: Arc<RwLock<RuleEngine>>,
    wallet_store: Arc<dyn WalletStore>,
    #[allow(dead_code)]
    penalty_store: Arc<dyn PenaltyStore>,
    cursor_store: Arc<dyn CursorStore>,
    audit: Arc<dyn AuditSink>,
    decision_sink: Arc<dyn DecisionSink>,
    user_id: Uuid,
    batch_size: usize,
}

impl RuleEvaluationPipeline {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_store: Arc<dyn EventStore>,
        rule_store: Arc<dyn RuleStore>,
        engine: Arc<RwLock<RuleEngine>>,
        wallet_store: Arc<dyn WalletStore>,
        penalty_store: Arc<dyn PenaltyStore>,
        cursor_store: Arc<dyn CursorStore>,
        audit: Arc<dyn AuditSink>,
        decision_sink: Arc<dyn DecisionSink>,
        user_id: Uuid,
    ) -> Self {
        Self {
            event_store,
            rule_store,
            engine,
            wallet_store,
            penalty_store,
            cursor_store,
            audit,
            decision_sink,
            user_id,
            batch_size: DEFAULT_BATCH_SIZE,
        }
    }

    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size.max(1);
        self
    }

    /// Drive one evaluation pass. Idempotent at the cursor level: a second
    /// call on the same pipeline returns an empty report once all events
    /// have been consumed.
    pub async fn tick(&self, now: DateTime<Utc>) -> anyhow::Result<EvaluationReport> {
        let mut report = EvaluationReport::default();

        let cursor = self
            .cursor_store
            .load(RULE_EVAL_CONNECTOR_ID, RULE_EVAL_ENTITY_TYPE)
            .await
            .unwrap_or(None);
        // NOTE: The underlying `EventStore::since_cursor` uses its own
        // ordering-dependent cursor (event_id lex compare for
        // `SqliteAdapter`). We do our own `occurred_at` gating here so the
        // pipeline's cursor semantics are independent of storage: the
        // highest `occurred_at` ISO string we've seen is the resume point.
        let raw = self.event_store.since_cursor(None, self.batch_size).await?;
        let events: Vec<NormalizedEvent> = match cursor.as_deref() {
            Some(c) => {
                raw.into_iter().filter(|e| e.occurred_at.to_rfc3339().as_str() > c).collect()
            }
            None => raw,
        };

        if events.is_empty() {
            return Ok(report);
        }

        let rules = self.rule_store.list_enabled().await.unwrap_or_default();

        let mut last_cursor: Option<String> = None;
        for event in &events {
            last_cursor = Some(event.occurred_at.to_rfc3339());
            let decisions = {
                let mut engine = self.engine.write().await;
                engine.evaluate_all(&rules, event, now)
            };
            report.events_evaluated = report.events_evaluated.saturating_add(1);

            for decision in decisions {
                match &decision.decision {
                    RuleDecision::Fired(actions) => {
                        report.decisions_fired = report.decisions_fired.saturating_add(1);
                        self.dispatch_actions(actions, &decision, event, now).await;
                        self.audit_fired(&decision, event, actions, now);
                        self.decision_sink.record(decision);
                    }
                    RuleDecision::Suppressed { .. } => {
                        report.decisions_suppressed = report.decisions_suppressed.saturating_add(1);
                    }
                    RuleDecision::Skipped { .. } => {
                        report.decisions_skipped = report.decisions_skipped.saturating_add(1);
                    }
                }
            }
        }

        if let Some(next_cursor) = last_cursor {
            if let Err(e) = self
                .cursor_store
                .save(RULE_EVAL_CONNECTOR_ID, RULE_EVAL_ENTITY_TYPE, &next_cursor)
                .await
            {
                warn!(error = %e, "rule_eval cursor persist failed");
            }
        }

        Ok(report)
    }

    async fn dispatch_actions(
        &self,
        actions: &[Action],
        decision: &PrioritizedDecision,
        event: &NormalizedEvent,
        now: DateTime<Utc>,
    ) {
        for action in actions {
            match action {
                Action::GrantCredit { amount } => {
                    let credit = Credit {
                        amount: *amount as i64,
                        source_rule_id: Some(decision.rule_id),
                        granted_at: now,
                    };
                    let mutation = WalletMutation::GrantCredit(credit);
                    if let Err(e) = self.wallet_store.apply(self.user_id, mutation).await {
                        warn!(error = %e, "wallet grant failed");
                    }
                }
                Action::DeductCredit { amount } => {
                    let mutation = WalletMutation::SpendCredit {
                        amount: *amount as i64,
                        purpose: "rule:deduct".into(),
                    };
                    if let Err(e) = self.wallet_store.apply(self.user_id, mutation).await {
                        warn!(error = %e, "wallet deduct failed");
                    }
                }
                Action::StreakIncrement(name) => {
                    if let Err(e) = self
                        .wallet_store
                        .apply(self.user_id, WalletMutation::StreakIncrement(name.clone()))
                        .await
                    {
                        warn!(error = %e, "streak increment failed");
                    }
                }
                Action::StreakReset(name) => {
                    if let Err(e) = self
                        .wallet_store
                        .apply(self.user_id, WalletMutation::StreakReset(name.clone()))
                        .await
                    {
                        warn!(error = %e, "streak reset failed");
                    }
                }
                Action::Block { .. } | Action::Unblock { .. } => {
                    // Policy-side: the decision itself is recorded by the
                    // caller into `recent_decisions`; `PolicyApi::build_
                    // from_recent_decisions` reads that buffer.
                    debug!(?action, "policy-affecting action — stashed in decision sink");
                }
                Action::Notify(message) => {
                    // Emit a dedicated `notify.dispatched` audit line so
                    // iOS can tail the chain and present a real
                    // UNNotificationContent per Notify action. Deduped on
                    // AuditRecord.id host-side.
                    let payload = json!({
                        "rule_id": decision.rule_id.to_string(),
                        "message": message,
                    });
                    if let Err(e) = self.audit.record_mutation(
                        "notify.dispatched",
                        &self.user_id.to_string(),
                        payload,
                        now,
                    ) {
                        warn!(error = %e, "notify.dispatched audit append failed");
                    }
                }
                Action::EmergencyExit { .. }
                | Action::Intervention { .. }
                | Action::ScheduledUnlockWindow { .. } => {
                    // Audit-only for now; UI surfaces these out of the audit
                    // chain.
                    debug!(?action, "UI-facing action recorded via audit only");
                }
            }
        }
        let _ = event; // reserved for future per-event context on mutations
    }

    fn audit_fired(
        &self,
        decision: &PrioritizedDecision,
        event: &NormalizedEvent,
        actions: &[Action],
        now: DateTime<Utc>,
    ) {
        let action_variants: Vec<&'static str> = actions.iter().map(action_variant_name).collect();
        let payload = json!({
            "rule_id": decision.rule_id.to_string(),
            "event_id": event.event_id.to_string(),
            "decision": "fired",
            "priority": decision.priority,
            "actions": action_variants,
            "explanation": format!("rule {} fired on event {}", decision.rule_id, event.event_id),
        });
        if let Err(e) =
            self.audit.record_mutation("rule.fired", &self.user_id.to_string(), payload, now)
        {
            warn!(error = %e, "rule.fired audit append failed");
        }
    }
}

fn action_variant_name(action: &Action) -> &'static str {
    match action {
        Action::GrantCredit { .. } => "GrantCredit",
        Action::DeductCredit { .. } => "DeductCredit",
        Action::Block { .. } => "Block",
        Action::Unblock { .. } => "Unblock",
        Action::StreakIncrement(_) => "StreakIncrement",
        Action::StreakReset(_) => "StreakReset",
        Action::Notify(_) => "Notify",
        Action::EmergencyExit { .. } => "EmergencyExit",
        Action::Intervention { .. } => "Intervention",
        Action::ScheduledUnlockWindow { .. } => "ScheduledUnlockWindow",
    }
}

// ---------------------------------------------------------------------------
// In-memory test doubles used by the integration tests. Exposed publicly so
// FFI-side unit tests can reuse them too.
// ---------------------------------------------------------------------------

/// In-memory [`EventStore`] used by pipeline tests. Orders events by
/// insertion and implements cursoring by "event_id" string comparison to
/// mirror [`focus_storage::SqliteAdapter`]'s semantics.
#[derive(Debug, Default, Clone)]
pub struct InMemoryEventStore {
    inner: Arc<Mutex<Vec<NormalizedEvent>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append(&self, event: NormalizedEvent) -> anyhow::Result<()> {
        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
        if g.iter().any(|e| e.dedupe_key == event.dedupe_key) {
            return Ok(());
        }
        g.push(event);
        Ok(())
    }

    async fn since_cursor(
        &self,
        cursor: Option<&str>,
        limit: usize,
    ) -> anyhow::Result<Vec<NormalizedEvent>> {
        let g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
        let mut out: Vec<NormalizedEvent> = g
            .iter()
            .filter(|e| match cursor {
                Some(c) => e.occurred_at.to_rfc3339().as_str() > c,
                None => true,
            })
            .cloned()
            .collect();
        out.sort_by(|a, b| a.occurred_at.cmp(&b.occurred_at).then(a.event_id.cmp(&b.event_id)));
        out.truncate(limit);
        Ok(out)
    }
}

/// In-memory [`RuleStore`] seeded at construction.
#[derive(Debug, Default, Clone)]
pub struct InMemoryRuleStore {
    inner: Arc<Mutex<Vec<Rule>>>,
}

impl InMemoryRuleStore {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { inner: Arc::new(Mutex::new(rules)) }
    }
}

#[async_trait]
impl RuleStore for InMemoryRuleStore {
    async fn get(&self, id: Uuid) -> anyhow::Result<Option<Rule>> {
        let g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
        Ok(g.iter().find(|r| r.id == id).cloned())
    }
    async fn list_enabled(&self) -> anyhow::Result<Vec<Rule>> {
        let g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
        Ok(g.iter().filter(|r| r.enabled).cloned().collect())
    }
}

/// In-memory [`WalletStore`] that applies mutations against a single
/// [`focus_rewards::RewardWallet`].
#[derive(Debug, Default, Clone)]
pub struct InMemoryWalletStore {
    inner: Arc<Mutex<focus_rewards::RewardWallet>>,
    audit: Arc<focus_audit::NoopAuditSink>,
}

impl InMemoryWalletStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn snapshot(&self) -> focus_rewards::RewardWallet {
        self.inner.lock().expect("wallet poisoned").clone()
    }
}

#[async_trait]
impl WalletStore for InMemoryWalletStore {
    async fn load(&self, user_id: Uuid) -> anyhow::Result<focus_rewards::RewardWallet> {
        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
        g.user_id = user_id;
        Ok(g.clone())
    }
    async fn apply(&self, user_id: Uuid, mutation: WalletMutation) -> anyhow::Result<()> {
        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
        g.user_id = user_id;
        g.apply(mutation, Utc::now(), self.audit.as_ref())
            .map_err(|e| anyhow::anyhow!("wallet apply: {e}"))?;
        Ok(())
    }
}

/// In-memory [`PenaltyStore`] no-op for pipeline tests that don't exercise
/// penalty mutations directly.
#[derive(Debug, Default, Clone)]
pub struct InMemoryPenaltyStore {
    inner: Arc<Mutex<focus_penalties::PenaltyState>>,
    audit: Arc<focus_audit::NoopAuditSink>,
}

impl InMemoryPenaltyStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl PenaltyStore for InMemoryPenaltyStore {
    async fn load(&self, user_id: Uuid) -> anyhow::Result<focus_penalties::PenaltyState> {
        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
        g.user_id = user_id;
        Ok(g.clone())
    }
    async fn apply(&self, user_id: Uuid, mutation: PenaltyMutation) -> anyhow::Result<()> {
        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
        g.user_id = user_id;
        g.apply(mutation, Utc::now(), self.audit.as_ref())
            .map_err(|e| anyhow::anyhow!("penalty apply: {e}"))?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};
    use focus_events::{DedupeKey, EventType, WellKnownEventType};
    use focus_rules::{Action, Trigger};
    use focus_sync::InMemoryCursorStore;
    use serde_json::json;

    fn mk_event(i: u64) -> NormalizedEvent {
        let ts = Utc.with_ymd_and_hms(2026, 4, 22, 12, 0, 0).unwrap() + Duration::seconds(i as i64);
        NormalizedEvent {
            event_id: Uuid::new_v4(),
            connector_id: "test".into(),
            account_id: Uuid::nil(),
            event_type: EventType::WellKnown(WellKnownEventType::TaskCompleted),
            occurred_at: ts,
            effective_at: ts,
            dedupe_key: DedupeKey(format!("test:{i}")),
            confidence: 1.0,
            payload: json!({}),
            raw_ref: None,
        }
    }

    fn mk_rule_grant(amount: i32, cooldown: Option<Duration>) -> Rule {
        Rule {
            id: Uuid::new_v4(),
            name: "grant".into(),
            trigger: Trigger::Event("TaskCompleted".into()),
            conditions: vec![],
            actions: vec![Action::GrantCredit { amount }],
            priority: 0,
            cooldown,
            duration: None,
            explanation_template: "fired".into(),
            enabled: true,
        }
    }

    fn mk_pipeline(
        events: Arc<InMemoryEventStore>,
        rules: Arc<InMemoryRuleStore>,
        engine: Arc<RwLock<RuleEngine>>,
        wallet: Arc<InMemoryWalletStore>,
        cursor: Arc<dyn CursorStore>,
    ) -> RuleEvaluationPipeline {
        let penalty: Arc<dyn PenaltyStore> = Arc::new(InMemoryPenaltyStore::new());
        let audit: Arc<dyn AuditSink> = Arc::new(focus_audit::CapturingAuditSink::new());
        let decisions: Arc<dyn DecisionSink> = Arc::new(NoopDecisionSink);
        RuleEvaluationPipeline::new(
            events as Arc<dyn EventStore>,
            rules as Arc<dyn RuleStore>,
            engine,
            wallet as Arc<dyn WalletStore>,
            penalty,
            cursor,
            audit,
            decisions,
            Uuid::nil(),
        )
    }

    #[tokio::test]
    async fn event_matching_rule_produces_wallet_grant() {
        let events = Arc::new(InMemoryEventStore::new());
        events.append(mk_event(0)).await.unwrap();
        let rules = Arc::new(InMemoryRuleStore::new(vec![mk_rule_grant(5, None)]));
        let engine = Arc::new(RwLock::new(RuleEngine::new()));
        let wallet = Arc::new(InMemoryWalletStore::new());
        let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());
        let pipeline = mk_pipeline(events.clone(), rules, engine, wallet.clone(), cursor);

        let report = pipeline.tick(Utc::now()).await.expect("tick");
        assert_eq!(report.events_evaluated, 1);
        assert_eq!(report.decisions_fired, 1);
        assert_eq!(wallet.snapshot().earned_credits, 5);
    }

    #[tokio::test]
    async fn dedupe_prevents_double_grant_for_same_event() {
        let events = Arc::new(InMemoryEventStore::new());
        let ev = mk_event(0);
        events.append(ev.clone()).await.unwrap();
        // Appending the same event twice is a no-op (same dedupe_key).
        events.append(ev.clone()).await.unwrap();
        let rules = Arc::new(InMemoryRuleStore::new(vec![mk_rule_grant(5, None)]));
        let engine = Arc::new(RwLock::new(RuleEngine::new()));
        let wallet = Arc::new(InMemoryWalletStore::new());
        let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());
        let pipeline = mk_pipeline(events, rules, engine, wallet.clone(), cursor);

        let report = pipeline.tick(Utc::now()).await.expect("tick");
        assert_eq!(report.events_evaluated, 1);
        assert_eq!(wallet.snapshot().earned_credits, 5);
    }

    #[tokio::test]
    async fn cooldown_suppresses_second_fire_within_window() {
        let events = Arc::new(InMemoryEventStore::new());
        events.append(mk_event(0)).await.unwrap();
        events.append(mk_event(1)).await.unwrap();
        let rules =
            Arc::new(InMemoryRuleStore::new(vec![mk_rule_grant(5, Some(Duration::hours(1)))]));
        let engine = Arc::new(RwLock::new(RuleEngine::new()));
        let wallet = Arc::new(InMemoryWalletStore::new());
        let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());
        let pipeline = mk_pipeline(events, rules, engine, wallet.clone(), cursor);

        let now = Utc.with_ymd_and_hms(2026, 4, 22, 12, 30, 0).unwrap();
        let report = pipeline.tick(now).await.expect("tick");
        assert_eq!(report.events_evaluated, 2);
        assert_eq!(report.decisions_fired, 1);
        assert_eq!(report.decisions_suppressed, 1);
        assert_eq!(wallet.snapshot().earned_credits, 5);
    }

    #[tokio::test]
    async fn cursor_persists_across_pipeline_instances() {
        let events = Arc::new(InMemoryEventStore::new());
        events.append(mk_event(0)).await.unwrap();
        let rules = Arc::new(InMemoryRuleStore::new(vec![mk_rule_grant(3, None)]));
        let engine = Arc::new(RwLock::new(RuleEngine::new()));
        let wallet = Arc::new(InMemoryWalletStore::new());
        let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());

        // Session 1: consume the first event.
        {
            let pipeline = mk_pipeline(
                events.clone(),
                rules.clone(),
                engine.clone(),
                wallet.clone(),
                cursor.clone(),
            );
            let r = pipeline.tick(Utc::now()).await.expect("tick1");
            assert_eq!(r.events_evaluated, 1);
            assert_eq!(r.decisions_fired, 1);
        }

        // Append a second event and run a fresh pipeline over the same
        // cursor store: only the new event must be evaluated.
        events.append(mk_event(1)).await.unwrap();
        {
            let pipeline = mk_pipeline(
                events.clone(),
                rules.clone(),
                engine.clone(),
                wallet.clone(),
                cursor.clone(),
            );
            let r = pipeline.tick(Utc::now()).await.expect("tick2");
            assert_eq!(
                r.events_evaluated, 1,
                "cursor hydration must skip the previously-seen event"
            );
            assert_eq!(r.decisions_fired, 1);
        }

        assert_eq!(wallet.snapshot().earned_credits, 6);
    }

    #[tokio::test]
    async fn decision_sink_receives_fired_decisions() {
        let events = Arc::new(InMemoryEventStore::new());
        events.append(mk_event(0)).await.unwrap();
        let rules = Arc::new(InMemoryRuleStore::new(vec![mk_rule_grant(1, None)]));
        let engine = Arc::new(RwLock::new(RuleEngine::new()));
        let wallet = Arc::new(InMemoryWalletStore::new());
        let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());
        let penalty: Arc<dyn PenaltyStore> = Arc::new(InMemoryPenaltyStore::new());
        let audit: Arc<dyn AuditSink> = Arc::new(focus_audit::CapturingAuditSink::new());
        let captured: Arc<Mutex<Vec<PrioritizedDecision>>> = Arc::new(Mutex::new(Vec::new()));
        let sink: Arc<dyn DecisionSink> = Arc::new(VecDecisionSink::new(captured.clone(), 100));

        let pipeline = RuleEvaluationPipeline::new(
            events as Arc<dyn EventStore>,
            rules as Arc<dyn RuleStore>,
            engine,
            wallet as Arc<dyn WalletStore>,
            penalty,
            cursor,
            audit,
            sink,
            Uuid::nil(),
        );
        pipeline.tick(Utc::now()).await.unwrap();
        assert_eq!(captured.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn audit_records_rule_fired_entries() {
        let events = Arc::new(InMemoryEventStore::new());
        events.append(mk_event(0)).await.unwrap();
        let rules = Arc::new(InMemoryRuleStore::new(vec![mk_rule_grant(2, None)]));
        let engine = Arc::new(RwLock::new(RuleEngine::new()));
        let wallet = Arc::new(InMemoryWalletStore::new());
        let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());
        let penalty: Arc<dyn PenaltyStore> = Arc::new(InMemoryPenaltyStore::new());
        let capturing = Arc::new(focus_audit::CapturingAuditSink::new());
        let audit: Arc<dyn AuditSink> = capturing.clone();
        let sink: Arc<dyn DecisionSink> = Arc::new(NoopDecisionSink);

        let pipeline = RuleEvaluationPipeline::new(
            events as Arc<dyn EventStore>,
            rules as Arc<dyn RuleStore>,
            engine,
            wallet as Arc<dyn WalletStore>,
            penalty,
            cursor,
            audit,
            sink,
            Uuid::nil(),
        );
        pipeline.tick(Utc::now()).await.unwrap();
        let snap = capturing.snapshot();
        assert_eq!(snap.len(), 1);
        assert_eq!(snap[0].0, "rule.fired");
    }

    /// Traces to: Notify → notify.dispatched audit-line bridge for iOS
    /// NotificationDispatcher. Proves the new audit payload surface
    /// (rule_id, message) so a regression breaking the Swift side
    /// fails loud in Rust CI.
    #[tokio::test]
    async fn notify_action_emits_notify_dispatched_audit_line() {
        let events = Arc::new(InMemoryEventStore::new());
        events.append(mk_event(0)).await.unwrap();
        let rule = Rule {
            id: Uuid::new_v4(),
            name: "nudge".into(),
            trigger: Trigger::Event("TaskCompleted".into()),
            conditions: vec![],
            actions: vec![Action::Notify("Take a break".into())],
            priority: 0,
            cooldown: None,
            duration: None,
            explanation_template: "fired".into(),
            enabled: true,
        };
        let rules = Arc::new(InMemoryRuleStore::new(vec![rule]));
        let engine = Arc::new(RwLock::new(RuleEngine::new()));
        let wallet: Arc<dyn WalletStore> = Arc::new(InMemoryWalletStore::new());
        let penalty: Arc<dyn PenaltyStore> = Arc::new(InMemoryPenaltyStore::new());
        let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());
        let capturing = Arc::new(focus_audit::CapturingAuditSink::new());
        let audit: Arc<dyn AuditSink> = capturing.clone();
        let sink: Arc<dyn DecisionSink> = Arc::new(NoopDecisionSink);

        let pipeline = RuleEvaluationPipeline::new(
            events as Arc<dyn EventStore>,
            rules as Arc<dyn RuleStore>,
            engine,
            wallet,
            penalty,
            cursor,
            audit,
            sink,
            Uuid::nil(),
        );
        pipeline.tick(Utc::now()).await.unwrap();
        let snap = capturing.snapshot();
        // Expect both the rule.fired meta line AND the notify.dispatched line.
        let kinds: Vec<&str> = snap.iter().map(|r| r.0.as_str()).collect();
        assert!(
            kinds.contains(&"rule.fired") && kinds.contains(&"notify.dispatched"),
            "expected rule.fired + notify.dispatched, got {kinds:?}"
        );
        let notify = snap.iter().find(|r| r.0 == "notify.dispatched").unwrap();
        assert_eq!(
            notify.2.get("message").and_then(|v| v.as_str()),
            Some("Take a break"),
        );
    }
}
