use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use focus_eval::BatchedRuleEvaluationPipeline;
use focus_events::{DedupeKey, EventType, NormalizedEvent};
use focus_rules::{Action, Rule, Trigger};
use focus_storage::adapters::{
    InMemoryEventStore, InMemoryPenaltyStore, InMemoryRuleStore, InMemoryWalletStore,
};
use focus_sync::InMemoryCursorStore;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

/// Benchmark: batched+parallel evaluation tick with 1000 events × 50 rules.
/// This is the primary performance target: should achieve <50ms (20x speedup from ~1s baseline).
fn bench_eval_batched_1000x50(c: &mut Criterion) {
    c.bench_function("eval_batched_1000_events_50_rules", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let events_store = Arc::new(InMemoryEventStore::new());
                let now = chrono::Utc::now();

                // Create 1000 events
                for i in 0..1000 {
                    let event = NormalizedEvent {
                        event_id: Uuid::new_v4(),
                        event_type: EventType::WellKnown("focus_event".to_string()),
                        occurred_at: now,
                        effective_at: now,
                        connector_id: "pipeline".to_string(),
                        account_id: Uuid::new_v4(),
                        dedupe_key: DedupeKey(format!("key_{}", i)),
                        confidence: 1.0,
                        payload: serde_json::json!({"app": format!("app_{}", i % 50)}),
                        raw_ref: None,
                    };
                    events_store.append(event).await.unwrap();
                }

                // Create 50 rules
                let rules = black_box(
                    (0..50)
                        .map(|i| Rule {
                            id: Uuid::new_v4(),
                            name: format!("rule_{}", i),
                            trigger: Trigger::Event("focus_event".to_string()),
                            conditions: vec![],
                            actions: vec![Action::GrantCredit {
                                amount: 1,
                                reason: "evaluation".to_string(),
                            }],
                            priority: i as i32,
                            cooldown: None,
                            duration: None,
                            explanation_template: "Pipeline eval".to_string(),
                            enabled: true,
                        })
                        .collect::<Vec<_>>(),
                );

                let rule_store = Arc::new(InMemoryRuleStore::new(rules));
                let engine = Arc::new(RwLock::new(focus_rules::RuleEngine::new()));
                let wallet: Arc<dyn focus_storage::ports::WalletStore> =
                    Arc::new(InMemoryWalletStore::new());
                let penalty: Arc<dyn focus_storage::ports::PenaltyStore> =
                    Arc::new(InMemoryPenaltyStore::new());
                let cursor: Arc<dyn focus_sync::CursorStore> =
                    Arc::new(InMemoryCursorStore::new());
                let audit: Arc<dyn focus_audit::AuditSink> =
                    Arc::new(focus_audit::CapturingAuditSink::new());
                let sink: Arc<dyn focus_eval::DecisionSink> =
                    Arc::new(focus_eval::NoopDecisionSink);

                let pipeline = BatchedRuleEvaluationPipeline::new(
                    events_store as Arc<dyn focus_storage::ports::EventStore>,
                    rule_store,
                    engine,
                    wallet,
                    penalty,
                    cursor,
                    audit,
                    sink,
                    Uuid::nil(),
                );

                let _report = pipeline.tick(now).await.unwrap();
                black_box(_report)
            });
    });
}

/// Benchmark: scaling parallelism with fixed 1000 events, varying rule counts.
fn bench_eval_batched_scaling_rules(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_batched_scaling_rules");

    for rule_count in [10, 25, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_rules", rule_count)),
            rule_count,
            |b, &rule_count| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async {
                        let events_store = Arc::new(InMemoryEventStore::new());
                        let now = chrono::Utc::now();

                        // Create 1000 events
                        for i in 0..1000 {
                            let event = NormalizedEvent {
                                event_id: Uuid::new_v4(),
                                event_type: EventType::WellKnown("focus_event".to_string()),
                                occurred_at: now,
                                effective_at: now,
                                connector_id: "pipeline".to_string(),
                                account_id: Uuid::new_v4(),
                                dedupe_key: DedupeKey(format!("key_{}", i)),
                                confidence: 1.0,
                                payload: serde_json::json!({"app": format!("app_{}", i % 50)}),
                                raw_ref: None,
                            };
                            events_store.append(event).await.unwrap();
                        }

                        // Variable rule counts
                        let rules = black_box(
                            (0..rule_count)
                                .map(|i| Rule {
                                    id: Uuid::new_v4(),
                                    name: format!("rule_{}", i),
                                    trigger: Trigger::Event("focus_event".to_string()),
                                    conditions: vec![],
                                    actions: vec![Action::GrantCredit {
                                        amount: 1,
                                        reason: "evaluation".to_string(),
                                    }],
                                    priority: i as i32,
                                    cooldown: None,
                                    duration: None,
                                    explanation_template: "Pipeline eval".to_string(),
                                    enabled: true,
                                })
                                .collect::<Vec<_>>(),
                        );

                        let rule_store = Arc::new(InMemoryRuleStore::new(rules));
                        let engine = Arc::new(RwLock::new(focus_rules::RuleEngine::new()));
                        let wallet: Arc<dyn focus_storage::ports::WalletStore> =
                            Arc::new(InMemoryWalletStore::new());
                        let penalty: Arc<dyn focus_storage::ports::PenaltyStore> =
                            Arc::new(InMemoryPenaltyStore::new());
                        let cursor: Arc<dyn focus_sync::CursorStore> =
                            Arc::new(InMemoryCursorStore::new());
                        let audit: Arc<dyn focus_audit::AuditSink> =
                            Arc::new(focus_audit::CapturingAuditSink::new());
                        let sink: Arc<dyn focus_eval::DecisionSink> =
                            Arc::new(focus_eval::NoopDecisionSink);

                        let pipeline = BatchedRuleEvaluationPipeline::new(
                            events_store as Arc<dyn focus_storage::ports::EventStore>,
                            rule_store,
                            engine,
                            wallet,
                            penalty,
                            cursor,
                            audit,
                            sink,
                            Uuid::nil(),
                        );

                        let _report = pipeline.tick(now).await.unwrap();
                        black_box(_report)
                    });
            },
        );
    }

    group.finish();
}

/// Benchmark: scaling event counts with fixed 50 rules.
fn bench_eval_batched_scaling_events(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_batched_scaling_events");

    for event_count in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_events", event_count)),
            event_count,
            |b, &event_count| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async {
                        let events_store = Arc::new(InMemoryEventStore::new());
                        let now = chrono::Utc::now();

                        // Variable event counts
                        for i in 0..event_count {
                            let event = NormalizedEvent {
                                event_id: Uuid::new_v4(),
                                event_type: EventType::WellKnown("focus_event".to_string()),
                                occurred_at: now,
                                effective_at: now,
                                connector_id: "pipeline".to_string(),
                                account_id: Uuid::new_v4(),
                                dedupe_key: DedupeKey(format!("key_{}", i)),
                                confidence: 1.0,
                                payload: serde_json::json!({"app": format!("app_{}", i % 50)}),
                                raw_ref: None,
                            };
                            events_store.append(event).await.unwrap();
                        }

                        // Fixed 50 rules
                        let rules = black_box(
                            (0..50)
                                .map(|i| Rule {
                                    id: Uuid::new_v4(),
                                    name: format!("rule_{}", i),
                                    trigger: Trigger::Event("focus_event".to_string()),
                                    conditions: vec![],
                                    actions: vec![Action::GrantCredit {
                                        amount: 1,
                                        reason: "evaluation".to_string(),
                                    }],
                                    priority: i as i32,
                                    cooldown: None,
                                    duration: None,
                                    explanation_template: "Pipeline eval".to_string(),
                                    enabled: true,
                                })
                                .collect::<Vec<_>>(),
                        );

                        let rule_store = Arc::new(InMemoryRuleStore::new(rules));
                        let engine = Arc::new(RwLock::new(focus_rules::RuleEngine::new()));
                        let wallet: Arc<dyn focus_storage::ports::WalletStore> =
                            Arc::new(InMemoryWalletStore::new());
                        let penalty: Arc<dyn focus_storage::ports::PenaltyStore> =
                            Arc::new(InMemoryPenaltyStore::new());
                        let cursor: Arc<dyn focus_sync::CursorStore> =
                            Arc::new(InMemoryCursorStore::new());
                        let audit: Arc<dyn focus_audit::AuditSink> =
                            Arc::new(focus_audit::CapturingAuditSink::new());
                        let sink: Arc<dyn focus_eval::DecisionSink> =
                            Arc::new(focus_eval::NoopDecisionSink);

                        let pipeline = BatchedRuleEvaluationPipeline::new(
                            events_store as Arc<dyn focus_storage::ports::EventStore>,
                            rule_store,
                            engine,
                            wallet,
                            penalty,
                            cursor,
                            audit,
                            sink,
                            Uuid::nil(),
                        );

                        let _report = pipeline.tick(now).await.unwrap();
                        black_box(_report)
                    });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_eval_batched_1000x50,
    bench_eval_batched_scaling_rules,
    bench_eval_batched_scaling_events
);
criterion_main!(benches);
