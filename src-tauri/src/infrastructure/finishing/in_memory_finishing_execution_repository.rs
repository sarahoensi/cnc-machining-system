//! In-memory repository implementation for finishing executions.
//!
//! Infrastructure adapter implementing the domain repository port.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::{
    FinishingExecution,
    FinishingExecutionId,
    FinishingExecutionRepository,
    FinishingRepositoryError,
};

pub struct InMemoryFinishingExecutionRepository {
    store: Mutex<HashMap<FinishingExecutionId, FinishingExecution>>,
}

impl InMemoryFinishingExecutionRepository {

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
    ) -> Result<FinishingExecution, FinishingRepositoryError> {

        let store = self
            .store
            .lock()
            .map_err(|_| FinishingRepositoryError::PersistenceFailure)?;

        store
            .get(&id)
            .cloned()
            .ok_or(FinishingRepositoryError::NotFound)
    }

    fn save(
        &self,
        execution: FinishingExecution,
    ) -> Result<(), FinishingRepositoryError> {

        let mut store = self
            .store
            .lock()
            .map_err(|_| FinishingRepositoryError::PersistenceFailure)?;

        store.insert(execution.id(), execution);

        Ok(())
    }
}