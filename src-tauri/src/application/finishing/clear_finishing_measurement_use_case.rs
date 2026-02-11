// application/finishing/clear_finishing_measurement_use_case.rs

// application/finishing/clear_finishing_measurement_use_case.rs

use crate::application::shared::AppResult;

use crate::application::finishing::dto::FinishingExecutionOutput;
use crate::application::finishing::finishing_output_mapper::to_output;

use crate::domain::FinishingExecution;

pub struct ClearFinishingMeasurementUseCase;

impl ClearFinishingMeasurementUseCase {

    pub fn execute(
        &self,
        execution: &mut FinishingExecution,
        step_number: u32,
    ) -> AppResult<FinishingExecutionOutput> {

        execution.clear_measurement(step_number)?;

        Ok(to_output(execution))
    }
}
