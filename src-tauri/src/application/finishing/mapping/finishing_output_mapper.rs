// application/finishing/mapping/finishing_output_mapper.rs

use crate::{application::finishing::{finishing_execution_output::FinishingExecutionOutput, finishing_step_output::FinishingStepOutput}, domain::FinishingExecution};

pub fn to_output(exec: &FinishingExecution) -> FinishingExecutionOutput {

    let steps = exec
        .steps()
        .iter()
        .map(|s| FinishingStepOutput {
            index: s.index(),
            start_mm: s.start().mm_value(),
            planned_delta_mm: s.planned_delta().mm_value(),
            planned_end_mm: s.planned_end().mm_value(),
            measurement_mm: s.measurement().map(|m| m.mm_value()),
        })
        .collect();

    FinishingExecutionOutput {
        execution_id: exec.id().value().to_string(),
        steps,
    }
}
