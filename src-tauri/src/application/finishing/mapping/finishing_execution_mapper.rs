//! Mapper from domain finishing executions to application execution DTOs.

// application/finishing/mapping/finishing_execution_mapper.rs

use crate::{
    application::finishing::{
        finishing_execution_output::FinishingExecutionOutput,
        mapping::finishing_step_mapper::to_step_output,
    },
    domain::FinishingExecution,
};

/// Translates a domain [`FinishingExecution`] aggregate into
/// [`FinishingExecutionOutput`].
///
/// Transformation guarantee:
/// - Carries the execution identifier and mapped steps without mutating state.
pub fn to_execution_output(exec: &FinishingExecution) -> FinishingExecutionOutput {
    let steps = exec.steps().iter().map(to_step_output).collect();

    FinishingExecutionOutput {
        execution_id: exec.id().value().to_string(),
        steps,
    }
}
