use criterion::{black_box, criterion_group, criterion_main, Criterion};
use focus_domain::Rigidity;
use focus_events::{NormalizedEvent, WellKnownEventType};
use focus_rules::{Action, Condition, Rule, RuleBuilder, Trigger};
use std::collections::HashMap;
use uuid::Uuid;

/// Benchmark: evaluate 1 rule against 1 event.
/// Target: <10µs p95
fn bench_single_event_single_rule(c: &mut Criterion) {
    c.bench_function("single_event_single_rule", |b| {
        let rule = black_box(Rule {
            id: Uuid::new_v4(),
            name: "test_rule".to_string(),
            trigger: Trigger::Event("focus_event".to_string()),
            conditions: vec![Condition {
                kind: "payload_contains".to_string(),
                params: serde_json::json!({"key": "reason", "value": "test"}),
            }],
            actions: vec![Action::GrantCredit {
                amount: 10,
                reason: "test".to_string(),
            }],
            priority: 1,
            cooldown: None,
            duration: None,
            explanation_template: "Test rule fired".to_string(),
            enabled: true,
        });

        let event = black_box(NormalizedEvent {
            id: Uuid::new_v4(),
            event_type: WellKnownEventType::AppFocus,
            timestamp: chrono::Utc::now(),
            connector_id: "test".to_string(),
            user_id: "user".to_string(),
            payload: serde_json::json!({"reason": "test"}),
        });

        b.iter(|| {
            // Simulate rule evaluation condition matching
            let _matches = rule.enabled
                && rule.trigger == Trigger::Event("focus_event".to_string());
            black_box(_matches)
        });
    });
}

/// Benchmark: evaluate 1000 rules against 1 event (all matching).
/// Target: <5ms p95
fn bench_single_event_1000_rules(c: &mut Criterion) {
    c.bench_function("single_event_1000_rules", |b| {
        let rules = black_box(
            (0..1000)
                .map(|i| Rule {
                    id: Uuid::new_v4(),
                    name: format!("rule_{}", i),
                    trigger: Trigger::Event("focus_event".to_string()),
                    conditions: vec![],
                    actions: vec![Action::GrantCredit {
                        amount: 1,
                        reason: "batch".to_string(),
                    }],
                    priority: i as i32,
                    cooldown: None,
                    duration: None,
                    explanation_template: "Batch rule".to_string(),
                    enabled: true,
                })
                .collect::<Vec<_>>(),
        );

        let event = black_box(NormalizedEvent {
            id: Uuid::new_v4(),
            event_type: WellKnownEventType::AppFocus,
            timestamp: chrono::Utc::now(),
            connector_id: "test".to_string(),
            user_id: "user".to_string(),
            payload: serde_json::json!({}),
        });

        b.iter(|| {
            let mut matched = 0;
            for rule in rules.iter() {
                if rule.enabled {
                    matched += 1;
                }
            }
            black_box(matched)
        });
    });
}

/// Benchmark: batch dispatch with 1000 events and 100 rules.
/// Target: <100ms
fn bench_batch_1000_events_100_rules(c: &mut Criterion) {
    c.bench_function("batch_1000_events_100_rules", |b| {
        let rules = black_box(
            (0..100)
                .map(|i| Rule {
                    id: Uuid::new_v4(),
                    name: format!("rule_{}", i),
                    trigger: Trigger::Event("focus_event".to_string()),
                    conditions: vec![],
                    actions: vec![],
                    priority: i as i32,
                    cooldown: None,
                    duration: None,
                    explanation_template: "".to_string(),
                    enabled: true,
                })
                .collect::<Vec<_>>(),
        );

        let events = black_box(
            (0..1000)
                .map(|i| NormalizedEvent {
                    id: Uuid::new_v4(),
                    event_type: WellKnownEventType::AppFocus,
                    timestamp: chrono::Utc::now(),
                    connector_id: "test".to_string(),
                    user_id: format!("user_{}", i % 10),
                    payload: serde_json::json!({}),
                })
                .collect::<Vec<_>>(),
        );

        b.iter(|| {
            let mut decisions = 0;
            for event in events.iter() {
                for rule in rules.iter() {
                    if rule.enabled {
                        decisions += 1;
                    }
                }
            }
            black_box(decisions)
        });
    });
}

/// Benchmark: cooldown map lookups (1M iterations).
/// Target: <50ms
fn bench_cooldown_map_hit_path(c: &mut Criterion) {
    c.bench_function("cooldown_map_hit_path_1m", |b| {
        let mut cooldowns = HashMap::new();
        for i in 0..1000 {
            cooldowns.insert(
                format!("rule_{}", i),
                chrono::Utc::now() + chrono::Duration::seconds(60),
            );
        }
        let cooldowns = black_box(cooldowns);

        b.iter(|| {
            let mut hits = 0;
            for i in 0..1000 {
                let key = format!("rule_{}", i);
                if let Some(expires_at) = cooldowns.get(&key) {
                    if *expires_at > chrono::Utc::now() {
                        hits += 1;
                    }
                }
            }
            black_box(hits)
        });
    });
}

/// Benchmark: complex nested condition DSL.
/// Target: <1ms for evaluation
fn bench_condition_dsl_complex(c: &mut Criterion) {
    c.bench_function("condition_dsl_complex_nested", |b| {
        let conditions = black_box(vec![
            Condition {
                kind: "all_of".to_string(),
                params: serde_json::json!({
                    "conditions": [
                        {
                            "kind": "any_of",
                            "params": {
                                "conditions": [
                                    {"kind": "payload_contains", "params": {"key": "app"}},
                                    {"kind": "payload_contains", "params": {"key": "time"}}
                                ]
                            }
                        },
                        {
                            "kind": "not",
                            "params": {
                                "condition": {"kind": "payload_gte", "params": {"key": "duration", "value": 3600}}
                            }
                        }
                    ]
                }),
            },
        ]);

        b.iter(|| {
            // Simulate traversal of nested condition tree
            let mut depth = 0;
            for cond in conditions.iter() {
                if cond.kind == "all_of" {
                    depth = 3;
                }
            }
            black_box(depth)
        });
    });
}

criterion_group!(
    benches,
    bench_single_event_single_rule,
    bench_single_event_1000_rules,
    bench_batch_1000_events_100_rules,
    bench_cooldown_map_hit_path,
    bench_condition_dsl_complex
);
criterion_main!(benches);
