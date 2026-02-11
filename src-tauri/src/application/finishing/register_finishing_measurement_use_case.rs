// application/finishing/register_finishing_measurement_use_case.rs

use crate::application::shared::AppResult;

use crate::application::finishing::dto::FinishingExecutionOutput;
use crate::application::finishing::generate_finishing_plan_use_case::GenerateFinishingPlanUseCase;

use crate::domain::{
    Diameter,
    FinishingExecution,
};

pub struct RegisterFinishingMeasurementUseCase;

impl RegisterFinishingMeasurementUseCase {

    pub fn execute(
        &self,
        execution: &mut FinishingExecution,
        step_number: u32,
        measurement_mm: f64,
    ) -> AppResult<FinishingExecutionOutput> {

        execution.register_measurement(
            step_number,
            Diameter::mm(measurement_mm)?,
        )?;

        Ok(GenerateFinishingPlanUseCase::to_output(execution))
    }
}
