//! Output DTO for a full finishing execution state.
//!
//! This module defines the application-level response returned to external
//! layers after plan creation or measurement registration.


use crate::{application::finishing::finishing_step_output::FinishingStepOutput};

/// Output DTO describing a finishing execution and all of its steps.
///
/// This is an application output contract that external interfaces can treat
/// as a snapshot of the current execution lifecycle state.
pub struct FinishingExecutionOutput {
    /// Stable execution identifier for subsequent update operations.
    pub execution_id: String,
    /// Ordered finishing steps with planned and optional measured values.
    pub steps: Vec<FinishingStepOutput>,
}
