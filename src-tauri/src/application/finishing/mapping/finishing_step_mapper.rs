//! Mapper from domain finishing steps to application step DTOs.

// application/finishing/mapping/finishing_step_mapper.rs

use crate::{
    application::finishing::finishing_step_output::FinishingStepOutput,
    domain::FinishingStep,
};

/// Translates a domain [`FinishingStep`] into [`FinishingStepOutput`].
///
/// Transformation guarantee:
/// - Preserves step order/identity and exposes domain values in millimeters.
pub fn to_step_output(step: &FinishingStep) -> FinishingStepOutput {
    FinishingStepOutput {
        index: step.index(),
        start_mm: step.start().mm_value(),
        planned_delta_mm: step.planned_delta().mm_value(),
        planned_end_mm: step.planned_end().mm_value(),
        measurement_mm: step.measurement().map(|m| m.mm_value()),
    }
}
