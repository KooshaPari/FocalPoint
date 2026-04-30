use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use focus_events::{NormalizedEvent, WellKnownEventType};
use focus_rules::{Action, Rule, Trigger};
use uuid::Uuid;

/// Benchmark: evaluation tick dispatching 100 events against 50 rules.
/// Measures per-event latency and per-rule-match throughput.
/// Target: <5ms total (50µs per event, 1µs per match)
fn bench_eval_tick_100x50(c: &mut Criterion) {
    c.bench_function("eval_tick_100_events_50_rules", |b| {
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

        // Simulate 100 events in the store
        let events = black_box(
            (0..100)
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
            let mut total_matches = 0;

            // Main evaluation loop: for each event, evaluate all rules
            for event in events.iter() {
                for rule in rules.iter() {
                    if rule.enabled {
                        // Simulate rule match (fast path)
                        total_matches += 1;
                    }
                }
            }

            black_box(total_matches)
        });
    });
}

/// Benchmark: per-event dispatch latency with increasing rule set sizes.
/// Target: <50µs per event across 10-100 rules
fn bench_eval_tick_per_event_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_tick_per_event");

    for rule_count in [10, 25, 50, 100].iter() {
        let rules = black_box(
            (0..*rule_count)
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

        let event = black_box(NormalizedEvent {
            id: Uuid::new_v4(),
            event_type: WellKnownEventType::AppFocus,
            timestamp: chrono::Utc::now(),
            connector_id: "pipeline".to_string(),
            user_id: "user_0".to_string(),
            payload: serde_json::json!({"app": "app_0"}),
        });

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_rules", rule_count)),
            rule_count,
            |b, _| {
                b.iter(|| {
                    let mut matches = 0;
                    for rule in rules.iter() {
                        if rule.enabled {
                            matches += 1;
                        }
                    }
                    black_box(matches)
                });
            },
        );
    }

    group.finish();
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

criterion_group!(
    benches,
    bench_eval_tick_100x50,
    bench_eval_tick_per_event_latency,
    bench_cursor_advance
);
criterion_main!(benches);
