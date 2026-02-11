// application/finishing/mapping/finishing_execution_mapper.rs

use crate::{
    application::finishing::{
        finishing_execution_output::FinishingExecutionOutput,
        mapping::finishing_step_mapper::to_step_output,
    },
    domain::FinishingExecution,
};

pub fn to_execution_output(exec: &FinishingExecution) -> FinishingExecutionOutput {
    let steps = exec.steps().iter().map(to_step_output).collect();

    FinishingExecutionOutput {
        execution_id: exec.id().value().to_string(),
        steps,
    }
}
