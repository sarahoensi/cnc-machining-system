// application/finishing/clear_finishing_measurement_use_case.rs

// application/finishing/clear_finishing_measurement_use_case.rs

use crate::application::shared::AppResult;

use crate::application::finishing::dto::FinishingExecutionOutput;
use crate::application::finishing::generate_finishing_plan_use_case::GenerateFinishingPlanUseCase;

use crate::domain::FinishingExecution;

pub struct ClearFinishingMeasurementUseCase;

impl ClearFinishingMeasurementUseCase {

    pub fn execute(
        &self,
        execution: &mut FinishingExecution,
        step_number: u32,
    ) -> AppResult<FinishingExecutionOutput> {

        execution.clear_measurement(step_number)?;

        Ok(GenerateFinishingPlanUseCase::to_output(execution))
    }
}
