// application/finishing/register_finishing_measurement_use_case.rs

use std::sync::Arc;

use crate::application::finishing::finishing_execution_output::FinishingExecutionOutput;
use crate::application::shared::AppResult;

use crate::application::finishing::mapping::finishing_execution_mapper::to_execution_output;

use crate::domain::{
    Diameter,
    FinishingExecutionId,
    FinishingExecutionRepository,
};


pub struct RegisterFinishingMeasurementUseCase {
    repo: Arc<dyn FinishingExecutionRepository>,
}


impl RegisterFinishingMeasurementUseCase {

    pub fn new(repo: Arc<dyn FinishingExecutionRepository>) -> Self {
        Self { repo }
    }


    pub fn execute(
        &self,
        id: FinishingExecutionId,
        step_number: u32,
        measurement_mm: f64,
    ) -> AppResult<FinishingExecutionOutput> {

        let mut execution = self.repo.get(id)?;

        execution.register_measurement(
            step_number,
            Diameter::mm(measurement_mm)?,
        )?;

        self.repo.save(execution.clone())?;

        Ok(to_execution_output(&execution))
    }
}
