// application/finishing/execution/register_measurement_use_case.rs

use std::sync::Arc;

use crate::application::{ApplicationError};
use crate::application::shared::{AppResult, InputParser};

use crate::application::finishing::dto::{
    FinishingExecutionOutput,
    RegisterFinishingMeasurementInput,
};

use crate::domain::{
    units::Diameter,
    FinishingExecutionRepository,
};

/// Registers one measured diameter value for a finishing step.
pub struct RegisterFinishingMeasurementUseCase {
    repo: Arc<dyn FinishingExecutionRepository>,
}

impl RegisterFinishingMeasurementUseCase {

    pub fn new(repo: Arc<dyn FinishingExecutionRepository>) -> Self {
        Self { repo }
    }

    pub fn execute(
        &self,
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
        // Load execution
        // -----------------------------------------------------

        let mut execution = self.repo
            .get(input.execution_id)
            .map_err(|e| ApplicationError::Infrastructure(e.to_string()))?;

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

        // -----------------------------------------------------
        // Persist updated state
        // -----------------------------------------------------

        self.repo
            .save(execution.clone())
            .map_err(|e| ApplicationError::Infrastructure(e.to_string()))?;

        Ok((&execution).into())
    }
}