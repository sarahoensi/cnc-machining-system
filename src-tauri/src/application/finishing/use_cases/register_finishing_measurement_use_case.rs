//! Use case for recording measured results in an active finishing execution.
//!
//! - Loads execution aggregate from repository
//! - Validates measurement input
//! - Delegates update logic to domain aggregate
//! - Maps domain errors into field-level ValidationErrors
//! - Persists updated state

use std::sync::Arc;

use crate::application::{ApplicationError, ValidationErrors};
use crate::application::shared::AppResult;

use crate::application::finishing::finishing_execution_output::FinishingExecutionOutput;
use crate::application::finishing::mapping::finishing_execution_mapper::to_execution_output;

use crate::domain::{
    units::Diameter,
    FinishingExecutionId,
    FinishingExecutionRepository,
    StrategyError,
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
        id: FinishingExecutionId,
        step_number: u32,
        measurement_mm: f64,
    ) -> AppResult<FinishingExecutionOutput> {

        // ---------------------------------------------------------
        // Validate measurement input
        // ---------------------------------------------------------

        let mut errors = ValidationErrors::new();

        let measurement = match Diameter::mm(measurement_mm) {
            Ok(v) => Some(v),
            Err(e) => {
                errors.push("measurement_mm", "invalid", e.to_string());
                None
            }
        };

        if step_number == 0 {
            errors.push("step_number", "invalid", "Step number must be ≥ 1");
        }

        if !errors.is_empty() {
            return Err(ApplicationError::Validation(errors));
        }

        // Safe unwrap (validated above)
        let measurement = measurement.unwrap();

        // ---------------------------------------------------------
        // Load execution
        // ---------------------------------------------------------

        let mut execution = self.repo
            .get(id)
            .map_err(|e| ApplicationError::Infrastructure(e.to_string()))?;

        // ---------------------------------------------------------
        // Delegate to domain
        // ---------------------------------------------------------

        execution
            .register_measurement(step_number, measurement)
            .map_err(map_strategy_error)?;

        // ---------------------------------------------------------
        // Persist updated state
        // ---------------------------------------------------------

        self.repo
            .save(execution.clone())
            .map_err(|e| ApplicationError::Infrastructure(e.to_string()))?;

        Ok(to_execution_output(&execution))
    }
}

fn map_strategy_error(err: StrategyError) -> ApplicationError {

    let mut v = ValidationErrors::new();

    match err {

        StrategyError::StepNumberMustBeOneBased => {
            v.push("step_number", "invalid", err.to_string());
        }

        StrategyError::StepNumberOutOfRange { .. } => {
            v.push("step_number", "out_of_range", err.to_string());
        }

        StrategyError::StepLocked { .. } => {
            v.push("step_number", "invalid_state", err.to_string());
        }

        StrategyError::MeasurementOutOfBounds { .. }
        | StrategyError::MeasurementBackwards { .. }
        | StrategyError::MeasurementExceedsTarget { .. } => {
            v.push("measurement_mm", "invalid_combination", err.to_string());
        }

        StrategyError::RecalculationDidNotReachTarget { .. }
        | StrategyError::ImpossiblePlan { .. }
        | StrategyError::DivisionByZero => {
            v.push("execution", "invalid_state", err.to_string());
        }

        other => {
            v.push("execution", "invalid", other.to_string());
        }
    }

    ApplicationError::Validation(v)
}