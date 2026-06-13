use std::collections::HashMap;
use std::sync::Arc;

pub use focus_errors::FocusError;
pub use focus_result::Result;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during test execution.
#[derive(Debug, Error, Serialize, Deserialize, Clone, PartialEq)]
pub enum TestError {
    #[error("test step failed: {0}")]
    StepFailed(String),
    #[error("assertion failed: {0}")]
    AssertionFailed(String),
    #[error("missing data for key: {0}")]
    MissingData(String),
    #[error("test timed out")]
    Timeout,
    #[error("parallel execution failed: {0:?}")]
    ParallelExecutionFailed(Vec<String>),
}

impl From<TestError> for FocusError {
    fn from(err: TestError) -> Self {
        FocusError::Internal {
            message: err.to_string(),
        }
    }
}

/// Result of a single assertion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssertionResult {
    Passed { msg: String },
    Failed {
        msg: String,
        expected: String,
        actual: String,
    },
}

/// Context passed to each test step.
#[derive(Debug, Clone, Default)]
pub struct TestContext {
    pub data: HashMap<String, String>,
    assertions: Vec<AssertionResult>,
}

impl TestContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Store a key/value pair in the test context.
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Retrieve a value from the test context.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    /// Assert that a condition is true.
    pub fn assert_true(&mut self, condition: bool, msg: &str) -> Result<()> {
        if condition {
            self.assertions.push(AssertionResult::Passed { msg: msg.to_string() });
            Ok(())
        } else {
            let result = AssertionResult::Failed {
                msg: msg.to_string(),
                expected: "true".to_string(),
                actual: "false".to_string(),
            };
            self.assertions.push(result.clone());
            Err(FocusError::from(TestError::AssertionFailed(msg.to_string())))
        }
    }

    /// Assert that two values are equal.
    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(
        &mut self,
        a: T,
        b: T,
        msg: &str,
    ) -> Result<()> {
        if a == b {
            self.assertions.push(AssertionResult::Passed { msg: msg.to_string() });
            Ok(())
        } else {
            let result = AssertionResult::Failed {
                msg: msg.to_string(),
                expected: format!("{:?}", a),
                actual: format!("{:?}", b),
            };
            self.assertions.push(result.clone());
            Err(FocusError::from(TestError::AssertionFailed(msg.to_string())))
        }
    }

    /// Return all recorded assertion results.
    pub fn assertions(&self) -> Vec<AssertionResult> {
        self.assertions.clone()
    }
}

/// Report generated after running a test harness.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestReport {
    pub name: String,
    pub total_steps: usize,
    pub passed: usize,
    pub failed: usize,
    pub duration_ms: u64,
    pub errors: Vec<FocusError>,
}

impl TestReport {
    pub fn is_success(&self) -> bool {
        self.failed == 0 && self.errors.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "Test '{}' - {} steps, {} passed, {} failed, {} ms, success: {}",
            self.name,
            self.total_steps,
            self.passed,
            self.failed,
            self.duration_ms,
            self.is_success()
        )
    }
}

/// A named test step.
struct TestStep {
    name: String,
    func: Box<dyn FnMut(&mut TestContext) -> Result<()> + Send>,
}

/// Harness for registering and running test steps.
pub struct TestHarness {
    name: String,
    steps: Vec<TestStep>,
    context: TestContext,
    assertions: Vec<AssertionResult>,
}

impl TestHarness {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            steps: Vec::new(),
            context: TestContext::new(),
            assertions: Vec::new(),
        }
    }

    /// Register a named test step.
    pub fn register_step<F>(&mut self, name: &str, step: F) -> Result<()>
    where
        F: FnMut(&mut TestContext) -> Result<()> + Send + 'static,
    {
        self.steps.push(TestStep {
            name: name.to_string(),
            func: Box::new(step),
        });
        Ok(())
    }

    /// Run all steps sequentially.
    pub fn run(&mut self) -> Result<TestReport> {
        let start = Utc::now();
        let mut passed = 0usize;
        let mut failed = 0usize;
        let mut errors = Vec::new();

        for step in &mut self.steps {
            match (step.func)(&mut self.context) {
                Ok(()) => {
                    passed += 1;
                }
                Err(e) => {
                    failed += 1;
                    errors.push(e);
                }
            }
        }

        self.assertions.extend(self.context.assertions());
        let duration_ms = (Utc::now() - start).num_milliseconds() as u64;

        Ok(TestReport {
            name: self.name.clone(),
            total_steps: self.steps.len(),
            passed,
            failed,
            duration_ms,
            errors,
        })
    }

    /// Run all steps in parallel using a thread pool.
    pub fn run_parallel(&mut self) -> Result<TestReport> {
        let start = Utc::now();
        let steps = std::mem::take(&mut self.steps);
        let total_steps = steps.len();

        let mut handles = Vec::with_capacity(steps.len());
        for mut step in steps {
            let ctx = Arc::new(std::sync::Mutex::new(self.context.clone()));
            handles.push(std::thread::spawn(move || {
                let mut guard = ctx.lock().unwrap();
                let result = (step.func)(&mut *guard);
                let assertions = guard.assertions();
                (result, assertions, step.name)
            }));
        }

        let mut passed = 0usize;
        let mut failed = 0usize;
        let mut errors = Vec::new();
        let mut parallel_assertions = Vec::new();

        for handle in handles {
            match handle.join() {
                Ok((result, assertions, _name)) => {
                    parallel_assertions.extend(assertions);
                    match result {
                        Ok(()) => passed += 1,
                        Err(e) => {
                            failed += 1;
                            errors.push(e);
                        }
                    }
                }
                Err(_) => {
                    failed += 1;
                    errors.push(FocusError::from(TestError::StepFailed(
                        "thread panicked".to_string(),
                    )));
                }
            }
        }

        self.assertions.extend(parallel_assertions);
        self.context.data.clear(); // parallel contexts were clones
        let duration_ms = (Utc::now() - start).num_milliseconds() as u64;

        Ok(TestReport {
            name: self.name.clone(),
            total_steps,
            passed,
            failed,
            duration_ms,
            errors,
        })
    }

    /// Return all assertions recorded so far.
    pub fn assertions(&self) -> Vec<AssertionResult> {
        self.assertions.clone()
    }
}

/// A mock transpiler for testing external dependencies.
#[derive(Debug, Clone, Default)]
pub struct MockTranspiler {
    name: String,
    connected: bool,
    calls: Vec<String>,
    inbox: Vec<String>,
}

impl MockTranspiler {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            connected: false,
            calls: Vec::new(),
            inbox: Vec::new(),
        }
    }

    /// Simulate connecting.
    pub fn connect(&mut self) -> Result<()> {
        self.calls.push("connect".to_string());
        self.connected = true;
        Ok(())
    }

    /// Simulate sending data.
    pub fn send(&mut self, data: &str) -> Result<()> {
        self.calls.push(format!("send({})", data));
        if !self.connected {
            return Err(FocusError::from(TestError::StepFailed(
                "not connected".to_string(),
            )));
        }
        Ok(())
    }

    /// Simulate receiving data.
    pub fn receive(&mut self) -> Result<String> {
        self.calls.push("receive".to_string());
        if !self.connected {
            return Err(FocusError::from(TestError::StepFailed(
                "not connected".to_string(),
            )));
        }
        self.inbox.pop().ok_or_else(|| {
            FocusError::from(TestError::MissingData("inbox empty".to_string()))
        })
    }

    /// Queue a message for `receive` to return.
    pub fn queue_receive(&mut self, data: &str) {
        self.inbox.push(data.to_string());
    }

    /// Simulate disconnecting.
    pub fn disconnect(&mut self) -> Result<()> {
        self.calls.push("disconnect".to_string());
        self.connected = false;
        Ok(())
    }

    /// Return all recorded call strings.
    pub fn recorded_calls(&self) -> Vec<String> {
        self.calls.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harness_sequential() {
        let mut harness = TestHarness::new("seq_test");
        harness
            .register_step("step1", |ctx| {
                ctx.set("key", "value")?;
                ctx.assert_true(true, "always true")
            })
            .unwrap();
        let report = harness.run().unwrap();
        assert!(report.is_success());
        assert_eq!(report.total_steps, 1);
    }

    #[test]
    fn test_harness_parallel() {
        let mut harness = TestHarness::new("par_test");
        harness
            .register_step("step_a", |ctx| ctx.assert_eq(1, 1, "one eq"))
            .unwrap();
        harness
            .register_step("step_b", |ctx| ctx.assert_true(2 == 2, "two eq"))
            .unwrap();
        let report = harness.run_parallel().unwrap();
        assert!(report.is_success());
        assert_eq!(report.total_steps, 2);
    }

    #[test]
    fn test_mock_transpiler() {
        let mut mock = MockTranspiler::new("mock");
        mock.connect().unwrap();
        mock.send("hello").unwrap();
        let calls = mock.recorded_calls();
        assert!(calls.contains(&"connect".to_string()));
        assert!(calls.contains(&"send(hello)".to_string()));
        mock.disconnect().unwrap();
    }

    #[test]
    fn test_context_assertions() {
        let mut ctx = TestContext::new();
        ctx.set("foo", "bar").unwrap();
        assert_eq!(ctx.get("foo"), Some("bar"));
        ctx.assert_true(true, "pass").unwrap();
        assert_eq!(ctx.assertions().len(), 1);
    }
}
