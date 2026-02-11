//infrastructure/finishing/in_memory_finishing_execution_repository.rs

use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::{
    FinishingExecution,
    FinishingExecutionId,
    FinishingExecutionRepository,
    StrategyError,
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
