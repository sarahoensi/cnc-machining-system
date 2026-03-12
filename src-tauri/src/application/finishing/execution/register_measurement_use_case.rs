// application/finishing/execution/register_measurement_use_case.rs

use crate::application::shared::{AppResult, InputParser};

use crate::application::finishing::dto::{
    FinishingExecutionOutput,
    RegisterFinishingMeasurementInput,
};

use crate::domain::{
    units::Diameter,
    machining::finishing::FinishingExecution,
};

pub struct RegisterFinishingMeasurementUseCase;

impl RegisterFinishingMeasurementUseCase {

    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        execution: &mut FinishingExecution,
        input: RegisterFinishingMeasurementInput,
    ) -> AppResult<FinishingExecutionOutput> {

        let mut p = InputParser::new();

        // -----------------------------------------------------
        // Parse input
        // -----------------------------------------------------

        let measurement =
            p.value("measurement_mm", Diameter::mm(input.measurement_mm));

        if input.step_number == 0 {
            p.push("step_number", "invalid", "Step number must be ≥ 1");
        }

        // -----------------------------------------------------
        // Domain operation
        // -----------------------------------------------------

        if let Some(measurement) = measurement {

            p.domain(
                "measurement_mm",
                execution.register_measurement(
                    input.step_number,
                    measurement,
                )
            );
        }

        p.finish()?;

        Ok((&*execution).into())
    }
}