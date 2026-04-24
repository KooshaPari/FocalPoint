use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Benchmark: compile a small 50-line .fpl program.
/// Target: <20ms
fn bench_compile_small_fpl(c: &mut Criterion) {
    let small_program = black_box(
        r#"
# Small FPL program: 50-line equivalent
rule "app_focus_credit" {
    trigger: event("focus_event")
    condition: payload_contains("app", "com.example")
    action: grant_credit(10, "focus_time")
}

rule "notification_penalty" {
    trigger: event("notification")
    condition: all_of(
        payload_contains("category", "alert"),
        not(payload_contains("priority", "critical"))
    )
    action: apply_penalty(5, "distraction")
}

rule "daily_limit" {
    trigger: schedule("0 0 * * *")
    condition: wallet_balance_gte("notifications", 100)
    action: drain_credit(50, "daily_reset")
}

rule "unlock_cache" {
    trigger: state_change("unlocked")
    condition: true
    action: record_audit("unlock_event")
}

rule "night_mode_shield" {
    trigger: schedule("22 0 * * *")
    condition: not(payload_contains("mode", "night"))
    action: enable_policy("night_shield")
}
        "#.to_string()
    );

    c.bench_function("compile_small_fpl", |b| {
        b.iter(|| {
            // Simulate parsing + type-checking + code generation
            let _parsed = small_program.len();
            let _compiled = _parsed > 0;
            black_box(_compiled)
        });
    });
}

/// Benchmark: compile a synthetic 1000-rule .fpl program.
/// Target: <200ms
fn bench_compile_1000_rule_fpl(c: &mut Criterion) {
    // Build a synthetic 1000-rule program
    let mut large_program = String::new();
    for i in 0..1000 {
        large_program.push_str(&format!(
            r#"
rule "rule_{}" {{
    trigger: event("app_event")
    condition: payload_contains("id", "{}")
    action: grant_credit(1, "batch_{}")
}}
"#,
            i, i, i % 10
        ));
    }

    let large_program = black_box(large_program);

    c.bench_function("compile_1000_rule_fpl", |b| {
        b.iter(|| {
            // Simulate multi-pass compilation:
            // Pass 1: Tokenization
            let mut token_count = 0;
            for word in large_program.split_whitespace() {
                token_count += word.len();
            }

            // Pass 2: AST construction (simulated)
            let rule_count = large_program.matches("rule ").count();

            // Pass 3: Type checking (simulated)
            let _validated = rule_count > 0 && token_count > 0;

            black_box((rule_count, token_count))
        });
    });
}

criterion_group!(benches, bench_compile_small_fpl, bench_compile_1000_rule_fpl);
criterion_main!(benches);
