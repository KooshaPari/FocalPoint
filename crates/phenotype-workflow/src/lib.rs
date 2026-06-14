#![forbid(unsafe_code)]

//! Phenotype Workflow — Workflow orchestration engine.

use async_trait::async_trait;
use phenotype_error_core::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Alias for workflow results.
pub type WorkflowResult<T> = Result<T>;

/// Trait for workflow states.
pub trait State: Debug + Clone + Send + Sync + 'static {}

impl<T: Debug + Clone + Send + Sync + 'static> State for T {}

/// Trait for workflow steps.
#[async_trait]
pub trait WorkflowStep: Send + Sync {
    type State: State;
    fn name(&self) -> &str;
    async fn execute(&self, state: &Self::State) -> WorkflowResult<Self::State>;
    async fn compensate(&self, _state: &Self::State) -> WorkflowResult<()> {
        Ok(())
    }
}

/// A workflow that executes steps sequentially.
pub struct SequentialWorkflow<S: State> {
    name: String,
    steps: Vec<Box<dyn WorkflowStep<State = S>>>,
}

impl<S: State> std::fmt::Debug for SequentialWorkflow<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SequentialWorkflow")
            .field("name", &self.name)
            .field("step_count", &self.steps.len())
            .finish()
    }
}

impl<S: State> SequentialWorkflow<S> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: Vec::new(),
        }
    }

    pub fn add_step(mut self, step: Box<dyn WorkflowStep<State = S>>) -> Self {
        self.steps.push(step);
        self
    }

    pub async fn execute(&self, initial_state: S) -> WorkflowResult<S> {
        let mut state = initial_state;
        for step in &self.steps {
            state = step.execute(&state).await?;
        }
        Ok(state)
    }
}

/// A workflow that executes branches in parallel.
pub struct ParallelWorkflow<S: State> {
    name: String,
    branches: Vec<SequentialWorkflow<S>>,
}

impl<S: State> std::fmt::Debug for ParallelWorkflow<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParallelWorkflow")
            .field("name", &self.name)
            .field("branch_count", &self.branches.len())
            .finish()
    }
}

impl<S: State> ParallelWorkflow<S> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            branches: Vec::new(),
        }
    }

    pub fn add_branch(mut self, branch: SequentialWorkflow<S>) -> Self {
        self.branches.push(branch);
        self
    }

    pub async fn execute(&self, initial_state: S) -> WorkflowResult<Vec<S>> {
        let mut results = Vec::new();
        for branch in &self.branches {
            let state = branch.execute(initial_state.clone()).await?;
            results.push(state);
        }
        Ok(results)
    }
}

/// A workflow definition that can be loaded from configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub name: String,
    pub steps: Vec<StepDefinition>,
}

/// Definition of a single workflow step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepDefinition {
    pub name: String,
    pub step_type: String,
    pub config: serde_json::Value,
}
