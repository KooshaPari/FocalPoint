use focus_test::{MockTranspiler, TestHarness, TestContext, FocusError};

fn main() -> Result<(), FocusError> {
    let mut harness = TestHarness::new("basic_test");

    // Register a simple setup step
    harness.register_step("setup", |ctx: &mut TestContext| {
        ctx.set("app_name", "FocalPoint")?;
        ctx.assert_true(true, "setup complete")
    })?;

    // Register a step that uses the TestContext key-value store
    harness.register_step("check_data", |ctx: &mut TestContext| {
        let name = ctx.get("app_name").map(|s| s.to_string()).ok_or_else(|| {
            FocusError::from(focus_test::TestError::MissingData("app_name".into()))
        })?;
        ctx.assert_eq(name, "FocalPoint".to_string(), "app name matches")
    })?;

    // Register a step that uses assertions
    harness.register_step("assertions", |ctx: &mut TestContext| {
        ctx.assert_true(1 + 1 == 2, "math is correct")?;
        ctx.assert_eq("hello", "hello", "strings match")
    })?;

    // Register a step that mocks an external transpiler
    harness.register_step("mock_transpiler", |_ctx: &mut TestContext| {
        let mut mock = MockTranspiler::new("demo");
        mock.connect()?;
        mock.send("data")?;
        mock.disconnect()?;
        let calls = mock.recorded_calls();
        assert!(calls.contains(&"connect".to_string()));
        assert!(calls.contains(&"send(data)".to_string()));
        assert!(calls.contains(&"disconnect".to_string()));
        Ok(())
    })?;

    // Run sequentially and inspect the report
    let report = harness.run()?;
    println!("{}", report.summary());
    assert!(report.is_success(), "test suite failed");

    // Also run parallel
    let mut harness2 = TestHarness::new("basic_parallel_test");
    harness2.register_step("p1", |ctx| ctx.assert_eq(10, 10, "ten eq"))?;
    harness2.register_step("p2", |ctx| ctx.assert_true(true, "true"))?;
    let par_report = harness2.run_parallel()?;
    println!("{}", par_report.summary());
    assert!(par_report.is_success(), "parallel test suite failed");

    Ok(())
}
