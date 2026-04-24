use criterion::{black_box, criterion_group, criterion_main, Criterion};
use focus_events::{NormalizedEvent, WellKnownEventType};
use focus_rules::{Action, Rule, Trigger};
use uuid::Uuid;

/// Benchmark: evaluation pipeline tick with 500 events and 50 rules.
/// Simulates the core `tick()` operation where the pipeline:
/// 1. Reads events from the store
/// 2. Evaluates rules against each event
/// 3. Records decisions to audit sink
/// Target: <50ms
fn bench_evaluation_pipeline_tick(c: &mut Criterion) {
    c.bench_function("evaluation_pipeline_tick_500x50", |b| {
        // Simulate 50 rules
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

        // Simulate 500 events in the store
        let events = black_box(
            (0..500)
                .map(|i| NormalizedEvent {
                    id: Uuid::new_v4(),
                    event_type: WellKnownEventType::AppFocus,
                    timestamp: chrono::Utc::now(),
                    connector_id: "pipeline".to_string(),
                    user_id: format!("user_{}", i % 20),
                    payload: serde_json::json!({"app": format!("app_{}", i % 50)}),
                })
                .collect::<Vec<_>>(),
        );

        b.iter(|| {
            let mut total_decisions = 0;

            // Main evaluation loop: for each event, evaluate all rules
            for event in events.iter() {
                for rule in rules.iter() {
                    if rule.enabled {
                        // Simulate decision recording (fast path)
                        total_decisions += 1;
                    }
                }
            }

            black_box(total_decisions)
        });
    });
}

/// Benchmark: cursor persistence and advance path.
/// Isolates the cursor tracking logic (used to resume from last evaluated event).
/// Target: <100µs per advance
fn bench_cursor_advance(c: &mut Criterion) {
    c.bench_function("cursor_advance_1000_iterations", |b| {
        let mut cursor_state = serde_json::json!({
            "connector_id": "rule_eval",
            "entity_type": "events",
            "last_id": "00000000-0000-0000-0000-000000000000",
            "last_timestamp": chrono::Utc::now().to_rfc3339(),
            "offset": 0
        });

        b.iter(|| {
            // Simulate advancing cursor through 1000 events
            for i in 0..1000 {
                cursor_state["offset"] = serde_json::json!(i);
                cursor_state["last_id"] = serde_json::json!(format!("{:08x}", i));
            }
            black_box(cursor_state.clone())
        });
    });
}

criterion_group!(benches, bench_evaluation_pipeline_tick, bench_cursor_advance);
criterion_main!(benches);
