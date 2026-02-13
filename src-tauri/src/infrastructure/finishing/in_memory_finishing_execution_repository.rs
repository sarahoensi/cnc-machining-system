//! In-memory repository implementation for finishing executions.
//!
//! This adapter implements the domain repository port for finishing workflows
//! and is intended for local runtime, integration wiring, and tests where
//! process-local persistence is sufficient.

//infrastructure/finishing/in_memory_finishing_execution_repository.rs

use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::{
    FinishingExecution,
    FinishingExecutionId,
    FinishingExecutionRepository,
    StrategyError,
};

/// Process-local in-memory storage for finishing execution aggregates.
///
/// Infrastructure role:
/// - Implements [`FinishingExecutionRepository`] as a technical adapter.
/// - Decouples application use cases from concrete persistence technology.
///
/// Storage behavior:
/// - Stores executions in a `Mutex<HashMap<...>>` for shared mutable access.
/// - Data lifetime is limited to the running process.
///
/// Error behavior:
/// - Returns domain-compatible `StrategyError` values for lookup misses.
pub struct InMemoryFinishingExecutionRepository {
    store: Mutex<HashMap<FinishingExecutionId, FinishingExecution>>,
}

impl InMemoryFinishingExecutionRepository {
    /// Creates an empty in-memory finishing execution repository.
    ///
    /// This constructor is typically used by interface/integration wiring when
    /// no external database adapter is configured.
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryFinishingExecutionRepository {
    fn default() -> Self {
        Self::new()
    }
}


impl FinishingExecutionRepository for InMemoryFinishingExecutionRepository {

    fn get(
        &self,
        id: FinishingExecutionId,
    ) -> Result<FinishingExecution, StrategyError> {

        let store = self.store.lock().unwrap();

        store
            .get(&id)
            .cloned()
            .ok_or(StrategyError::InvalidInputs("execution not found"))
    }

    fn save(
        &self,
        execution: FinishingExecution,
    ) -> Result<(), StrategyError> {

        let mut store = self.store.lock().unwrap();

        store.insert(execution.id(), execution);

        Ok(())
    }
}
