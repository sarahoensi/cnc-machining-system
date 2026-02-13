//! Use case for recording measured results in an active finishing execution.
//!
//! The workflow loads a finishing execution aggregate, delegates measurement
//! registration to domain rules, persists updated state, and returns a new
//! execution snapshot for external consumers.

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


/// Registers one measured diameter value for a finishing step.
///
/// This use case orchestrates the update phase of the finishing lifecycle.
pub struct RegisterFinishingMeasurementUseCase {
    repo: Arc<dyn FinishingExecutionRepository>,
}


impl RegisterFinishingMeasurementUseCase {

    /// Creates the use case with a finishing execution repository dependency.
    pub fn new(repo: Arc<dyn FinishingExecutionRepository>) -> Self {
        Self { repo }
    }


    /// Records a step measurement and persists updated execution state.
    ///
    /// Purpose:
    /// - Applies measured shop-floor feedback to a specific finishing step.
    ///
    /// Required inputs:
    /// - Execution identifier for an existing finishing workflow.
    /// - `step_number` identifying the step to update.
    /// - `measurement_mm` as measured diameter in millimeters.
    ///
    /// Output meaning:
    /// - Returns updated [`FinishingExecutionOutput`] reflecting the newly
    ///   registered measurement.
    ///
    /// Domain invariants enforced:
    /// - Measurement value validity and step update rules are enforced by the
    ///   finishing execution aggregate.
    ///
    /// Side effects:
    /// - Reads and writes execution state through the repository.
    ///
    /// Error scenarios:
    /// - Unknown execution ID from repository lookup.
    /// - Invalid measurement value or invalid step transition in domain logic.
    /// - Repository persistence failures.
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
