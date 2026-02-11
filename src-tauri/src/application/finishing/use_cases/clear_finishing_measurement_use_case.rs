// application/finishing/clear_finishing_measurement_use_case.rs

use std::sync::Arc;

use crate::application::finishing::finishing_output_mapper::to_output;
use crate::application::shared::AppResult;

use crate::application::finishing::dto::finishing_execution_output::FinishingExecutionOutput;

use crate::domain::{
    FinishingExecutionRepository,
    FinishingExecutionId,
};


pub struct ClearFinishingMeasurementUseCase {
    repo: Arc<dyn FinishingExecutionRepository>,
}


impl ClearFinishingMeasurementUseCase {

    pub fn new(repo: Arc<dyn FinishingExecutionRepository>) -> Self {
        Self { repo }
    }


    pub fn execute(
        &self,
        id: FinishingExecutionId,
        step_number: u32,
    ) -> AppResult<FinishingExecutionOutput> {

        let mut execution = self.repo.get(id)?;

        execution.clear_measurement(step_number)?;

        self.repo.save(execution.clone())?;

        Ok(to_output(&execution))
    }
}
