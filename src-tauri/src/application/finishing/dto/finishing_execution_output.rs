
use crate::{application::finishing::finishing_step_output::FinishingStepOutput};

pub struct FinishingExecutionOutput {
    pub execution_id: String,
    pub steps: Vec<FinishingStepOutput>,
}
